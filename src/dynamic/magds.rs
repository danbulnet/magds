use std::{
    rc::Rc,
    cell::RefCell,
    collections::HashMap,
};

use asa_graphs::neural::graph::ASAGraph;
use bionet_common::{
    neuron::{ Neuron, NeuronID },
    data::{ DataType, DataTypeValue, DataCategory },
    sensor::Sensor
    };

use crate::neuron::simple_neuron::SimpleNeuron;

use super::sensor::SensorConatiner;

pub struct MAGDS {
    pub(crate) sensors: HashMap<Rc<str>, Rc<RefCell<SensorConatiner>>>,
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
            DataType::Bool => SensorConatiner::Bool(ASAGraph::<bool>::new_box(&id)),
            DataType::U8 => SensorConatiner::U8(ASAGraph::<u8>::new_box(&id)),
            DataType::U16 => SensorConatiner::U16(ASAGraph::<u16>::new_box(&id)),
            DataType::U32 => SensorConatiner::U32(ASAGraph::<u32>::new_box(&id)),
            DataType::U64 => SensorConatiner::U64(ASAGraph::<u64>::new_box(&id)),
            DataType::U128 => SensorConatiner::U128(ASAGraph::<u128>::new_box(&id)),
            DataType::USize => SensorConatiner::USize(ASAGraph::<usize>::new_box(&id)),
            DataType::I8 => SensorConatiner::I8(ASAGraph::<i8>::new_box(&id)),
            DataType::I16 => SensorConatiner::I16(ASAGraph::<i16>::new_box(&id)),
            DataType::I32 => SensorConatiner::I32(ASAGraph::<i32>::new_box(&id)),
            DataType::I64 => SensorConatiner::I64(ASAGraph::<i64>::new_box(&id)),
            DataType::I128 => SensorConatiner::I128(ASAGraph::<i128>::new_box(&id)),
            DataType::ISize => SensorConatiner::ISize(ASAGraph::<isize>::new_box(&id)),
            DataType::F32 => SensorConatiner::F32(ASAGraph::<f32>::new_box(&id)),
            DataType::F64 => SensorConatiner::F64(ASAGraph::<f64>::new_box(&id)),
            DataType::RcStr => SensorConatiner::RcStr(ASAGraph::<Rc<str>>::new_box(&id)),
            DataType::String => SensorConatiner::String(ASAGraph::<String>::new_box(&id)),
            DataType::Unknown => panic!("unknown data type sensor is not allowed")
        };
        self.sensors.insert(id, Rc::new(RefCell::new(sensor)));
    }

    pub fn add_sensor(
        &mut self, sensor: Rc<RefCell<SensorConatiner>>
    ) -> Option<Rc<RefCell<SensorConatiner>>> {
        let sensor_id = sensor.borrow().id().clone();
        self.sensors.insert(sensor_id, sensor)
    }

    pub fn sensor(&self, id: Rc<str>) -> Option<&Rc<RefCell<SensorConatiner>>> {
        self.sensors.get(&id)
    }

    pub fn sensor_id(&self, id: Rc<str>) -> Option<Rc<str>> { 
        Some(self.sensors.get(&id)?.borrow().id().clone())
    }

    pub fn sensor_data_type(&self, id: Rc<str>) -> Option<DataType> { 
        Some(self.sensors.get(&id)?.borrow().data_type())
    }

    pub fn sensor_data_category(&self, id: Rc<str>) -> Option<DataCategory> { 
        Some(self.sensors.get(&id)?.borrow().data_category())
    }

    pub fn sensor_insert(
        &mut self, id: Rc<str>, item: &DataTypeValue
    ) -> Option<Rc<RefCell<dyn Neuron>>> {
        Some(self.sensors.get_mut(&id)?.borrow_mut().insert(item))
    }
    
    pub fn sensor_search(
        &self, id: Rc<str>, item: &DataTypeValue
    ) -> Option<Rc<RefCell<dyn Neuron>>> { 
        self.sensors.get(&id)?.borrow().search(item) 
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
            .borrow_mut()
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
            .borrow_mut()
            .deactivate(item, propagate_horizontal, propagate_vertical)
    }

    pub fn deactivate_whole_sensor(&mut self, id: Rc<str>) -> Result<(), String> {
        self.sensors
            .get_mut(&id)
            .unwrap_or(Err(format!("sensor {} doesn't exists", id))?)
            .borrow_mut()
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

        let mut sensor_1 = ASAGraph::<i32>::new_box("test") as Box<dyn Sensor<i32>>;
        for i in 1..=9 { sensor_1.insert(&i); }

        let mut sensor_2 = ASAGraph::<String, 3>::new_box("test_string") as Box<dyn Sensor<String>>;
        for i in 1..=9 { sensor_2.insert(&i.to_string()); }

        let parent_name = Rc::from("test");
        let neuron_1 = SimpleNeuron::new(&Rc::from("neuron_1"), &parent_name);
        let neuron_2 = SimpleNeuron::new(&Rc::from("neuron_2"), &parent_name);

        magds.add_sensor(Rc::new(RefCell::new(sensor_1.into())));
        magds.add_sensor(Rc::new(RefCell::new(sensor_2.into())));
        magds.add_neuron(neuron_1);
        magds.add_neuron(neuron_2);

        let sensor_1_from_magds = magds.sensor("test".into()).unwrap();
        sensor_1_from_magds.borrow_mut().insert(&10.into());
        sensor_1_from_magds.borrow_mut().insert(&11.into());
        sensor_1_from_magds.borrow_mut().insert(&12.into());
        assert!(sensor_1_from_magds.borrow().as_i32().is_some());

        let mut sum = 0;
        for i in 1..=12 {
            let el = sensor_1_from_magds.borrow().search(&(i as i32).into());
            assert!(el.is_some());
            let neuron_id = NeuronID { 
                id: Rc::from(i.to_string()), parent_id: "test".into() 
            };
            let el = el.unwrap();
            assert_eq!(el.borrow().id(), neuron_id);
            let counter = el.borrow().counter();
            sum += counter;
            assert_eq!(counter, 1);
        }
        assert_eq!(sum, 12);

        let sensor_2_from_magds = magds.sensor("test_string".into()).unwrap();
        sensor_2_from_magds.borrow_mut().insert(&10.to_string().into());
        sensor_2_from_magds.borrow_mut().insert(&11.to_string().into());
        sensor_2_from_magds.borrow_mut().insert(&12.to_string().into());

        let mut sum = 0;
        for i in 1..=12 {
            let el = sensor_2_from_magds.borrow().search(&(i.to_string()).into());
            assert!(el.is_some());
            let neuron_id = NeuronID { 
                id: Rc::from(i.to_string()), parent_id: "test_string".into() 
            };
            let el = el.unwrap();
            assert_eq!(el.borrow().id(), neuron_id);
            let counter = el.borrow().counter();
            sum += counter;
            assert_eq!(counter, 1);
        }
        assert_eq!(sum, 12);

        let neuron_1_id = NeuronID::new("neuron_1", "test");
        let neuron_1_from_magds = magds.neuron_from_id(&neuron_1_id).unwrap();
        assert_eq!(neuron_1_from_magds.borrow().id(), neuron_1_id);
        let neuron_1_from_magds = magds.neuron("neuron_1", "test").unwrap();
        assert_eq!(neuron_1_from_magds.borrow().id(), neuron_1_id);

        magds.create_sensor(Rc::from("rcstr_test"), DataType::RcStr);
        let text: Rc<str> = Rc::from("test");
        let sensor_element = magds.sensor_insert(Rc::from("rcstr_test"), &text.into());
        assert!(sensor_element.is_some())
    }
}