// use std::{
//     rc::Rc,
//     cell::RefCell,
//     collections::HashMap
// };

// use bionet_common::{
//     sensor::{ Sensor, SensorData },
//     data::DataCategory,
//     neuron::{ Neuron, NeuronID }
// };

// use asa_graphs::neural::graph::ASAGraph;

// pub enum SimpleSensor {
//     Bool(ASAGraph<bool, 25>),
//     U8(ASAGraph<u8, 25>),
//     U16(ASAGraph<u16, 25>),
//     U32(ASAGraph<u32, 25>),
//     U64(ASAGraph<u64, 25>),
//     I8(ASAGraph<i8, 25>),
//     I16(ASAGraph<i16, 25>),
//     I32(ASAGraph<i32, 25>),
//     I64(ASAGraph<i64, 25>),
//     F32(ASAGraph<f32, 25>),
//     F64(ASAGraph<f64, 25>),
//     String(ASAGraph<Rc<str>, 25>),
// }

// impl<D: SensorData> Sensor<D> for SimpleSensor {
//     fn id(&self) -> &str { self.id() }

//     fn data_category(&self) -> DataCategory { self.data_category() }

//     fn insert(&mut self, key: &D) -> Rc<RefCell<dyn Neuron>> {
//         self.insert(key)
//     }

//     fn search(&self, key: &D) -> Option<Rc<RefCell<dyn Neuron>>> { 
//         Some(
//             self.search(key).unwrap() as Rc<RefCell<dyn Neuron>>
//         )
//     }

//     fn activate(
//         &mut self, item: &D, signal: f32, propagate_horizontal: bool, propagate_vertical: bool
//     ) -> Result<HashMap<NeuronID, Rc<RefCell<dyn Neuron>>>, String> {
//         self.activate(item, signal, propagate_horizontal, propagate_vertical)
//     }

//     fn deactivate(
//         &mut self, item: &D, propagate_horizontal: bool, propagate_vertical: bool
//     ) -> Result<(), String> {
//         self.deactivate(item, propagate_horizontal, propagate_vertical)
//     }

//     fn deactivate_sensor(&mut self) { self.deactivate_sensor() }
// }