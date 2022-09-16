use std::{
    rc::Rc,
    cell::RefCell,
    collections::HashMap
};

use asa_graphs::neural::graph::ASAGraph;
use bionet_common::{
    neuron::{ Neuron, NeuronID },
    data::{ DataType, DataTypeValue, DataCategory},
    sensor::Sensor
};

use crate::neuron::simple_neuron::SimpleNeuron;

use super::sensor::SensorConatiner;

pub struct MAGDS {
    pub(crate) sensors: HashMap<Rc<str>, SensorConatiner>,
    pub(crate) neurons: HashMap<NeuronID, Rc<RefCell<dyn Neuron>>>
}

impl MAGDS {
    pub fn new() -> MAGDS {
        MAGDS { sensors: HashMap::new(), neurons: HashMap::new() }
    }
    
    pub fn new_rc() -> Rc<RefCell<MAGDS>> {
        Rc::new(RefCell::new(MAGDS { sensors: HashMap::new(), neurons: HashMap::new() }))
    }

    pub fn create_sensor(&mut self, id: Rc<str>, data_type: DataType) {
        let sensor = match data_type {
            DataType::Bool => SensorConatiner::Bool(ASAGraph::<bool, 25>::new_rc(&id)),
            DataType::U8 => SensorConatiner::U8(ASAGraph::<u8, 25>::new_rc(&id)),
            DataType::U16 => SensorConatiner::U16(ASAGraph::<u16, 25>::new_rc(&id)),
            DataType::U32 => SensorConatiner::U32(ASAGraph::<u32, 25>::new_rc(&id)),
            DataType::U64 => SensorConatiner::U64(ASAGraph::<u64, 25>::new_rc(&id)),
            DataType::U128 => SensorConatiner::U128(ASAGraph::<u128, 25>::new_rc(&id)),
            DataType::USize => SensorConatiner::USize(ASAGraph::<usize, 25>::new_rc(&id)),
            DataType::I8 => SensorConatiner::I8(ASAGraph::<i8, 25>::new_rc(&id)),
            DataType::I16 => SensorConatiner::I16(ASAGraph::<i16, 25>::new_rc(&id)),
            DataType::I32 => SensorConatiner::I32(ASAGraph::<i32, 25>::new_rc(&id)),
            DataType::I64 => SensorConatiner::I64(ASAGraph::<i64, 25>::new_rc(&id)),
            DataType::I128 => SensorConatiner::I128(ASAGraph::<i128, 25>::new_rc(&id)),
            DataType::ISize => SensorConatiner::ISize(ASAGraph::<isize, 25>::new_rc(&id)),
            DataType::F32 => SensorConatiner::F32(ASAGraph::<f32, 25>::new_rc(&id)),
            DataType::F64 => SensorConatiner::F64(ASAGraph::<f64, 25>::new_rc(&id)),
            DataType::RcStr => SensorConatiner::RcStr(ASAGraph::<Rc<str>, 25>::new_rc(&id)),
            DataType::String => SensorConatiner::String(ASAGraph::<String, 25>::new_rc(&id)),
            DataType::Unknown => panic!("unknown data type sensor is not allowed")
        };
        self.sensors.insert(id, sensor);
    }

    pub fn sensor_id(&self, id: Rc<str>) -> Option<Rc<str>> { 
        Some(self.sensors.get(&id)?.id())
    }

    pub fn sensor_data_type(&self, id: Rc<str>) -> Option<DataType> { 
        Some(self.sensors.get(&id)?.data_type())
    }

    pub fn sensor_data_category(&self, id: Rc<str>) -> Option<DataCategory> { 
        Some(self.sensors.get(&id)?.data_category())
    }

    pub fn sensor_insert(
        &mut self, id: Rc<str>, item: &DataTypeValue
    ) -> Option<Rc<RefCell<dyn Neuron>>> {
        Some(self.sensors.get_mut(&id)?.insert(item))
    }
    
    pub fn sensor_search(
        &self, id: Rc<str>, item: &DataTypeValue
    ) -> Option<Rc<RefCell<dyn Neuron>>> { 
        self.sensors.get(&id)?.search(item) 
    }

    pub fn sensor_activate(
        &mut self, 
        id: Rc<str>, 
        item: &DataTypeValue,
        signal: f32,
        propagate_horizontal: bool, 
        propagate_vertical: bool
    ) -> Result<HashMap<NeuronID, Rc<RefCell<dyn Neuron>>>, String> {
        self.sensors
            .get_mut(&id)
            .unwrap_or(Err(format!("sensor {} doesn't exists", id))?)
            .activate(item, signal, propagate_horizontal, propagate_vertical)
    }

    pub fn sensor_deactivate(
        &mut self, 
        id: Rc<str>, 
        item: &DataTypeValue,
        propagate_horizontal: bool, 
        propagate_vertical: bool
    ) -> Result<(), String> {
        self.sensors
            .get_mut(&id)
            .unwrap_or(Err(format!("sensor {} doesn't exists", id))?)
            .deactivate(item, propagate_horizontal, propagate_vertical)
    }

    pub fn deactivate_whole_sensor(&mut self, id: Rc<str>) -> Result<(), String> {
        self.sensors
            .get_mut(&id)
            .unwrap_or(Err(format!("sensor {} doesn't exists", id))?)
            .deactivate_sensor();
        Ok(())
    }
    
    pub fn add_neuron(&mut self, neuron: Rc<RefCell<SimpleNeuron>>) {
        self.neurons.insert(neuron.borrow().id(), neuron.clone());
    }

    pub fn neuron_from_id(&self, id: &NeuronID) -> Option<Rc<RefCell<dyn Neuron>>> {
        Some(self.neurons.get(id)?.clone())
    }

    pub fn neuron(&self, id: &str, parent_id: &str) -> Option<Rc<RefCell<dyn Neuron>>> {
        Some(self.neurons.get(&NeuronID::new(id, parent_id))?.clone())
    }
}

#[cfg(test)]
mod tests {
    use std::{
        rc::Rc,
        cell::RefCell
    };

    use asa_graphs::neural::graph::ASAGraph;
    
    use bionet_common::{
        neuron::NeuronID,
        sensor::Sensor,
        data::DataType
    };
    
    use crate::neuron::simple_neuron::SimpleNeuron;

    use super::MAGDS;

    #[test]
    fn create_magds() {
        let mut magds = MAGDS::new();

        magds.create_sensor(Rc::from("test"), DataType::I32);
        let sensor_element = magds.sensor_insert(Rc::from("test"), &32.into());
    }
}