use std::{
    collections::{ HashMap, BTreeMap },
    rc::Rc,
    cell::RefCell
};

use ordered_float::OrderedFloat;

use bionet_common::{
    data::DataTypeValue,
    neuron::{ Neuron, NeuronID }
};
use polars::export::num::ToPrimitive;

use crate::simple::magds::MAGDS;

pub fn predict(
    magds: &mut MAGDS, 
    features: &HashMap<Rc<str>, DataTypeValue>,
    target: Rc<str>,
    fuzzy: bool
) -> Option<(DataTypeValue, f32)> {
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

    let max_activation = features.len() as f32;
    let proba = winner_activation.to_f32()? / max_activation;

    let predicted_value = winner.borrow().explain_one(target)?;

    Some((predicted_value, proba))
}

pub fn predict_weighted(
    magds: &mut MAGDS, 
    features: HashMap<Rc<str>, (DataTypeValue, f32)>,
    target: Rc<str>,
    fuzzy: bool
) -> Option<(DataTypeValue, f32)> {
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

    let max_activation = features.len() as f32;
    let proba = winner_activation.to_f32()? / max_activation;

    let predicted_value = winner.borrow().explain_one(target)?;

    Some((predicted_value, proba))
}