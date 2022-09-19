use std::{
    collections::{ HashMap, BTreeMap },
    rc::Rc,
    cell::RefCell
};

use ordered_float::OrderedFloat;

use bionet_common::{
    data::{ DataTypeValue, DataTypeValueStr },
    neuron::{ Neuron, NeuronID }, distances::Distance
};
use polars::export::num::{ToPrimitive, traits::Pow};

use crate::simple::magds::MAGDS;

pub fn predict(
    magds: &mut MAGDS, 
    features: &HashMap<Rc<str>, DataTypeValue>,
    target: Rc<str>,
    fuzzy: bool
) -> Option<(DataTypeValue, f64)> {
    let mut neurons: HashMap<NeuronID, Rc<RefCell<dyn Neuron>>> = HashMap::new();

    for (id, value) in features {
        let sensor = match magds.sensor_search(id.clone(), value) {
            Some(s) => s,
            None => {
                log::warn!("cannot find sensor {id}, skipping");
                continue
            }
        };
        neurons.extend(sensor.borrow_mut().activate(1.0_f32, fuzzy, true));
    }

    if neurons.is_empty() { return None }

    let neurons_activations: Vec<OrderedFloat<f32>> = neurons.values()
        .cloned()
        .map(|neuron| OrderedFloat(neuron.borrow().activation()))
        .collect();
    let neurons: Vec<Rc<RefCell<dyn Neuron>>> = neurons.values().cloned().collect();

    let neurons_sorted: BTreeMap<OrderedFloat<f32>, Rc<RefCell<dyn Neuron>>> 
        = BTreeMap::from_iter(neurons_activations.into_iter().zip(neurons));

    let (winner_activation, winner) = neurons_sorted.into_iter().next()?;

    let max_activation = features.len() as f64;
    let proba = winner_activation.to_f32()? as f64 / max_activation;

    let predicted_value = winner.borrow().explain_one(target)?;

    Some((predicted_value, proba))
}

pub fn predict_weighted(
    magds: &mut MAGDS, 
    features: HashMap<Rc<str>, (DataTypeValue, f32)>,
    target: Rc<str>,
    fuzzy: bool
) -> Option<(DataTypeValue, f64)> {
    let mut neurons: HashMap<NeuronID, Rc<RefCell<dyn Neuron>>> = HashMap::new();

    for (id, (value, weight)) in &features {
        let sensor = match magds.sensor_search(id.clone(), &value) {
            Some(s) => s,
            None => {
                log::warn!("cannot find sensor {id}, skipping");
                continue
            }
        };
        neurons.extend(sensor.borrow_mut().activate(*weight, fuzzy, true));
    }

    if neurons.is_empty() { return None }

    let neurons_activations: Vec<OrderedFloat<f32>> = neurons.values()
        .cloned()
        .map(|neuron| OrderedFloat(neuron.borrow().activation()))
        .collect();
    let neurons: Vec<Rc<RefCell<dyn Neuron>>> = neurons.values().cloned().collect();

    let neurons_sorted: BTreeMap<OrderedFloat<f32>, Rc<RefCell<dyn Neuron>>> 
        = BTreeMap::from_iter(neurons_activations.into_iter().zip(neurons));

    let (winner_activation, winner) = neurons_sorted.into_iter().next()?;

    let max_activation = features.len() as f64;
    let proba = winner_activation.to_f32()? as f64 / max_activation;

    let predicted_value = winner.borrow().explain_one(target)?;

    Some((predicted_value, proba))
}

pub fn prediction_score(
    train: &mut MAGDS, test: &mut MAGDS, target: Rc<str>, fuzzy: bool
) -> Option<(f64, f64)> {
    let mut total_proba = 0.0;
    let mut total_error = 0.0;

    for (neuron_id, neuron) in &mut test.neurons {
        let mut features: HashMap<Rc<str>, DataTypeValue> = HashMap::new();
        let sensors = neuron.borrow().defining_sensors();
        let mut test_reference_value = DataTypeValue::Unknown;
        let mut should_skip = false;

        for (sensor_id, sensor) in sensors {
            let feature_name: Rc<str> = sensor_id.parent_id.into();
            let feaure_value_str = DataTypeValueStr(&feature_name);
            let feature_data_type = sensor.borrow().data_type();
            let feature_value = feaure_value_str.data_type_value(feature_data_type);
            if *feature_name == *target {
                match feature_value {
                    Some(v) => test_reference_value = v,
                    None => {
                        log::warn!("target feature {target} is None for {neuron_id}, skipping");
                        should_skip = true;
                        break
                    }
                };
            } else {
                match feature_value {
                    Some(v) => features.insert(feature_name, v),
                    None => continue,
                };
            }
        }

        if should_skip { continue }
        if test_reference_value.is_unknown() { 
            panic!("test_reference_value shouldn't be unknown");
        }

        let (winner_value, winner_proba) = predict(train, &features, target.clone(), fuzzy)?;
        total_proba += winner_proba;
        total_error += winner_value.distance(&test_reference_value).pow(2);
        train.deactivate();
    }

    let final_proba = total_proba / test.neurons.len() as f64;
    let rmse = (total_error as f64).sqrt();
    
    Some((rmse, final_proba))
}