use std::{
    rc::Rc,
    cell::RefCell,
    collections::HashMap
};

use bionet_common::{
    neuron::{ Neuron, NeuronID },
    sensor::{ SensorDynamic, SensorDataDynamic }
};

pub struct MAGDS {
    pub sensors: HashMap<Rc<str>, Rc<RefCell<dyn SensorDynamic<Data = dyn SensorDataDynamic>>>>,
    pub neurons: HashMap<NeuronID, Rc<RefCell<dyn Neuron>>>,
}

impl MAGDS {
    pub fn new() -> MAGDS {
        MAGDS { sensors: HashMap::new(), neurons: HashMap::new() }
    }

    pub fn add_sensor<T: SensorDataDynamic>(&mut self, sensor: Rc<RefCell<dyn SensorDynamic<Data = T>>>) {
        let sensor_dyn = unsafe { 
            std::mem::transmute_copy::<
                Rc<RefCell<dyn SensorDynamic<Data = T>>>, 
                Rc<RefCell<dyn SensorDynamic<Data = dyn SensorDataDynamic>>>
            >(&sensor) 
        };
        self.sensors.insert(
            Rc::from(sensor.borrow().name()),
            sensor_dyn
        );
    }

    pub fn add_neuron(&mut self, neuron: Rc<RefCell<dyn Neuron>>) {
        self.neurons.insert(neuron.borrow().id(), neuron.clone());
    }
}

#[cfg(test)]
mod tests {
    use std::{
        rc::Rc
    };

    use asa_graphs::neural::graph::ASAGraph;
    
    use bionet_common::{
        data::DataCategory,
        sensor::SensorDynamicBuilder
    };
    
    use crate::neuron::simple_neuron::SimpleNeuron;

    use super::MAGDS;

    #[test]
    fn create_magds() {
        let mut magds = MAGDS::new();

        let sensor_1 = <ASAGraph::<i32, 25> as SensorDynamicBuilder::<i32>>::new(
            "test", DataCategory::Numerical
        );

        let parent_name = Rc::from("test");
        let neuron_1 = SimpleNeuron::new(&Rc::from("neuron_1"), &parent_name);
        let neuron_2 = SimpleNeuron::new(&Rc::from("neuron_2"), &parent_name);

        magds.add_sensor(sensor_1);
        magds.add_neuron(neuron_1);
        magds.add_neuron(neuron_2);
    }
}