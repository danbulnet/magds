use std::{
    rc::Rc,
    cell::RefCell,
    collections::HashMap
};

use bionet_common::{
    neuron::{ Neuron, NeuronID },
    sensor::{ SensorDynamic, SensorDataDynamic, SensorDynamicDowncast, SensorStaticDowncast }
};

pub struct MAGDS {
    pub(crate) sensors: HashMap<Rc<str>, Rc<RefCell<dyn SensorDynamic<Data = dyn SensorDataDynamic>>>>,
    pub(crate) neurons: HashMap<NeuronID, Rc<RefCell<dyn Neuron>>>,
}

impl MAGDS {
    pub fn new() -> MAGDS {
        MAGDS { sensors: HashMap::new(), neurons: HashMap::new() }
    }

    pub fn add_sensor<T: SensorDataDynamic>(&mut self, sensor: Rc<RefCell<dyn SensorDynamic<Data = T>>>) {
        let sensor_name = Rc::from(sensor.borrow().name());
        let sensor_dyn = unsafe {
            std::mem::transmute::<
                Rc<RefCell<dyn SensorDynamic<Data = T>>>, 
                Rc<RefCell<dyn SensorDynamic<Data = dyn SensorDataDynamic>>>
            >(sensor)
        };
        self.sensors.insert(
            sensor_name,
            sensor_dyn
        );
    }

    pub fn add_neuron(&mut self, neuron: Rc<RefCell<dyn Neuron>>) {
        self.neurons.insert(neuron.borrow().id(), neuron.clone());
    }

    pub fn sensor<T: SensorDataDynamic>(
        &self, name: &str
    ) -> Rc<RefCell<dyn SensorDynamic<Data = T>>> {
        let sensor = self.sensors[&Rc::from(name)].clone();
        <dyn SensorDynamic<Data = T> as SensorDynamicDowncast::<T>>::sensor_dynamic_downcast(
            sensor.clone()
        )
    }

    // experimental
    pub unsafe fn sensor_base<D: SensorDataDynamic, T: SensorDynamic>(
        &self, name: &str
    ) -> &mut T {
        let sensor = self.sensors[&Rc::from(name)].clone();
        let ptr = <
            dyn SensorDynamic<Data = D> as SensorStaticDowncast::<T>
        >::sensor_static_downcast( sensor.clone() );
        &mut *ptr
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

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
        for i in 1..=9 {
            sensor_1.borrow_mut().insert(&i);
        }

        let sensor_2 = <ASAGraph::<String, 3> as SensorDynamicBuilder::<String>>::new(
            "test_string", DataCategory::Numerical
        );
        for i in 1..=9 {
            sensor_2.borrow_mut().insert(&i.to_string());
        }

        let parent_name = Rc::from("test");
        let neuron_1 = SimpleNeuron::new(&Rc::from("neuron_1"), &parent_name);
        let neuron_2 = SimpleNeuron::new(&Rc::from("neuron_2"), &parent_name);

        magds.add_sensor(sensor_1.clone());
        magds.add_sensor(sensor_2.clone());
        magds.add_neuron(neuron_1);
        magds.add_neuron(neuron_2);

        let sensor_1_from_magds = magds.sensor("test");
        sensor_1_from_magds.borrow_mut().insert(&10);
        sensor_1_from_magds.borrow_mut().insert(&11);
        sensor_1_from_magds.borrow_mut().insert(&12);

        unsafe {
            let sensor_1_base_from_magds: &mut ASAGraph<i32, 25> = 
                magds.sensor_base::<i32, ASAGraph<i32, 25>>("test");
            sensor_1_base_from_magds.insert(&13);
            sensor_1_base_from_magds.insert(&14);
            sensor_1_base_from_magds.insert(&15);
            assert_eq!(sensor_1_base_from_magds.count_elements_unique(), 15);
            for (i, el) in sensor_1_base_from_magds.into_iter().enumerate() {
                assert_eq!(el.borrow().key, i as i32 + 1);
            }
        }

        let sensor_2_from_magds = magds.sensor("test_string");
        sensor_2_from_magds.borrow_mut().insert(&10.to_string());
        sensor_2_from_magds.borrow_mut().insert(&11.to_string());
        sensor_2_from_magds.borrow_mut().insert(&12.to_string());

        unsafe {
            let sensor_2_base_from_magds: &mut ASAGraph<String, 3> =
                magds.sensor_base::<String, ASAGraph<String, 3>>("test_string");
            sensor_2_base_from_magds.insert(&13.to_string());
            sensor_2_base_from_magds.insert(&14.to_string());
            sensor_2_base_from_magds.insert(&15.to_string());
            assert_eq!(sensor_2_base_from_magds.count_elements_unique(), 15);
        }
    }
}