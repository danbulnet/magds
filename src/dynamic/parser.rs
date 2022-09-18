use std::{
    rc::Rc,
    cell::RefCell,
    marker::PhantomData
};

use polars::prelude::*;

use asa_graphs::neural::graph::ASAGraph;

use bionet_common::{
    polars::{ self as polars_common, DataVec, DataVecOption },
    neuron::{ Neuron, NeuronID, NeuronConnectBilateral },
    connection::ConnectionKind,
    sensor::{ Sensor, SensorData },
    data::{ DataDeductor, DataTypeValue }
};

use crate::{
    neuron::simple_neuron::SimpleNeuron,
    dynamic::{
        magds::MAGDS,
        sensor::SensorConatiner
    }
};

#[allow(dead_code)]
pub(crate) fn sensor_from_datavec(
    magds: &mut MAGDS, id: &str, data: &DataVec
) -> Option<Rc<RefCell<SensorConatiner>>> {
    match data {
        DataVec::Unknown => {
            log::error!("can't parse vec data type for sensor {id}");
            return None
        }
        DataVec::BoolVec(vec) => {
            let graph = ASAGraph::<_>::new_box_from_vec(id, vec) as Box<dyn Sensor<bool>>;
            magds.add_sensor(Rc::new(RefCell::new(graph.into())))
        }
        DataVec::UInt8Vec(vec) => {
            let graph = ASAGraph::<_>::new_box_from_vec(id, vec) as Box<dyn Sensor<u8>>;
            magds.add_sensor(Rc::new(RefCell::new(graph.into())))
        }
        DataVec::UInt16Vec(vec) => {
            let graph = ASAGraph::<_>::new_box_from_vec(id, vec) as Box<dyn Sensor<u16>>;
            magds.add_sensor(Rc::new(RefCell::new(graph.into())))
        }
        DataVec::UInt32Vec(vec) => {
            let graph = ASAGraph::<_>::new_box_from_vec(id, vec) as Box<dyn Sensor<u32>>;
            magds.add_sensor(Rc::new(RefCell::new(graph.into())))
        }
        DataVec::UInt64Vec(vec) => {
            let graph = ASAGraph::<_>::new_box_from_vec(id, vec) as Box<dyn Sensor<u64>>;
            magds.add_sensor(Rc::new(RefCell::new(graph.into())))
        }
        DataVec::Int8Vec(vec) => {
            let graph = ASAGraph::<_>::new_box_from_vec(id, vec) as Box<dyn Sensor<i8>>;
            magds.add_sensor(Rc::new(RefCell::new(graph.into())))
        }
        DataVec::Int16Vec(vec) => {
            let graph = ASAGraph::<_>::new_box_from_vec(id, vec) as Box<dyn Sensor<i16>>;
            magds.add_sensor(Rc::new(RefCell::new(graph.into())))
        }
        DataVec::Int32Vec(vec) => {
            let graph = ASAGraph::<_>::new_box_from_vec(id, vec) as Box<dyn Sensor<i32>>;
            magds.add_sensor(Rc::new(RefCell::new(graph.into())))
        }
        DataVec::Int64Vec(vec) => {
            let graph = ASAGraph::<_>::new_box_from_vec(id, vec) as Box<dyn Sensor<i64>>;
            magds.add_sensor(Rc::new(RefCell::new(graph.into())))
        }
        DataVec::Float32Vec(vec) => {
            let graph = ASAGraph::<_>::new_box_from_vec(id, vec) as Box<dyn Sensor<f32>>;
            magds.add_sensor(Rc::new(RefCell::new(graph.into())))
        }
        DataVec::Float64Vec(vec) => {
            let graph = ASAGraph::<_>::new_box_from_vec(id, vec) as Box<dyn Sensor<f64>>;
            magds.add_sensor(Rc::new(RefCell::new(graph.into())))
        }
        DataVec::Utf8Vec(vec) => {
            let graph = ASAGraph::<String>::new_box_from_vec(id, vec) as Box<dyn Sensor<String>>;
            magds.add_sensor(Rc::new(RefCell::new(graph.into())))
        }
    }
}

pub(crate) fn connected_sensor_from_datavec(
    mut magds: &mut MAGDS, id: &str, data: &DataVecOption, neurons: &[Rc<RefCell<SimpleNeuron>>]
) -> Option<Rc<RefCell<SensorConatiner>>> {
    fn connector<T: SensorData>(
        magds: &mut MAGDS, id: &str, vec: &[Option<T>], neurons: &[Rc<RefCell<SimpleNeuron>>]
    ) -> Option<Rc<RefCell<SensorConatiner>>> 
    where 
        PhantomData<T>: DataDeductor, 
        SensorConatiner: From<Box<dyn Sensor<T>>>,
        DataTypeValue: From<T>
    {
        assert_eq!(neurons.len(), vec.len());
        let mut sensor = ASAGraph::<T>::new_box(id);
        for (i, key) in vec.into_iter().enumerate() {
            if let Some(key) = key {
                let element = sensor.insert(key);
                let neuron_ptr = neurons[i].clone();
                let mut neuron = neuron_ptr.borrow_mut();
                if let Err(e) = neuron.connect_bilateral_from(
                    element.clone(), ConnectionKind::Defining
                ) {
                    log::error!(
                        "error connecting neuron {} with sensor {}, error: {e}", 
                        neuron, 
                        element.borrow()
                    );
                }
            } else {
                continue
            }
        }
        magds.add_sensor(Rc::new(RefCell::new((sensor as Box<dyn Sensor<T>>).into())))
    }
    
    match data {
        DataVecOption::Unknown => {
            log::error!("can't parse vec data type for sensor {id}");
            return None
        }
        DataVecOption::BoolVec(vec) => { connector(&mut magds, id, vec, neurons) }
        DataVecOption::UInt8Vec(vec) => { connector(&mut magds, id, vec, neurons) }
        DataVecOption::UInt16Vec(vec) => { connector(&mut magds, id, vec, neurons) }
        DataVecOption::UInt32Vec(vec) => { connector(&mut magds, id, vec, neurons) }
        DataVecOption::UInt64Vec(vec) => { connector(&mut magds, id, vec, neurons) }
        DataVecOption::Int8Vec(vec) => { connector(&mut magds, id, vec, neurons) }
        DataVecOption::Int16Vec(vec) => { connector(&mut magds, id, vec, neurons) }
        DataVecOption::Int32Vec(vec) => { connector(&mut magds, id, vec, neurons) }
        DataVecOption::Int64Vec(vec) => { connector(&mut magds, id, vec, neurons) }
        DataVecOption::Float32Vec(vec) => { connector(&mut magds, id, vec, neurons) }
        DataVecOption::Float64Vec(vec) => { connector(&mut magds, id, vec, neurons) }
        DataVecOption::Utf8Vec(vec) => { connector(&mut magds, id, vec, neurons) }
    }
}

pub fn magds_from_df(df_name: Rc<str>, df: &DataFrame) -> MAGDS {
    let mut magds = MAGDS::new();
    
    log::info!("magds_from_df: df size: {} (cols) x {} (rows)", df.width(), df.height());
    log::info!("magds_from_df: df columns: {:?}", df.get_column_names());
    

    let mut neurons: Vec<Rc<RefCell<SimpleNeuron>>> = Vec::new();
    for i in 0..df.height() {
        let neuron = SimpleNeuron::new(
            NeuronID{ id: (i + 1).to_string().into(), parent_id: df_name.clone() }
        );
        neurons.push(neuron.clone());
        magds.add_neuron(neuron as Rc<RefCell<dyn Neuron>>).unwrap();
    }

    for column in df.get_columns() {
        let column_name = column.name();
        let datavec = match polars_common::series_to_datavec(column) {
            Ok(v) => v,
            Err(e) => { 
                log::error!("error convering {column_name} to datavec, error: {e}");
                continue
            }
        };
        if let None = connected_sensor_from_datavec(
            &mut magds, column_name, &datavec, &neurons
        ) {
            log::error!("error convering {column_name} datavec to sensor");
            continue
        };
    }

    magds
}

#[cfg(test)]
mod tests {
    use polars::datatypes::DataType;

    use bionet_common::{
        polars as polars_common,
        sensor::Sensor
    };

    use crate::dynamic::magds::MAGDS;

    #[test]
    fn df_to_magds() {
        let df = polars_common::csv_to_dataframe("data/iris.csv").unwrap();
        let magds = super::magds_from_df("iris".into(), &df);
        println!("{magds}");

        let versicolor = 
            magds.sensor_search("variety".into(), &"Versicolor".to_string().into()).unwrap();
        let setosa = 
            magds.sensor_search("variety".into(), &"Setosa".to_string().into()).unwrap();
        let virginica = 
            magds.sensor_search("variety".into(), &"Virginica".to_string().into()).unwrap();
        assert_eq!(setosa.borrow().counter(), 49);
        assert_eq!(versicolor.borrow().counter(), 50);
        assert_eq!(virginica.borrow().counter(), 50);

        let sl58 = magds.sensor_search("sepal.length".into(), &5.8_f64.into()).unwrap();
        assert_eq!(sl58.borrow().counter(), 7);

        let neuron_1 = magds.neuron("1".into(), "iris".into()).unwrap();
        println!("neuron_1 {}", neuron_1.borrow());
        for (id, sensor) in &neuron_1.borrow().explain() {
            println!("{id}");
            if id.parent_id == "petal.length".into() {
                assert_eq!(sensor.borrow().id().id, "1.4".into());
            } else if id.parent_id == "petal.width".into() {
                assert_eq!(sensor.borrow().id().id, "0.2".into());
            } else if id.parent_id == "sepal.width".into() {
                assert_eq!(sensor.borrow().id().id, "3.5".into());
            } else if id.parent_id == "variety".into() {
                assert_eq!(sensor.borrow().id().id, "Setosa".into());
            } else if id.parent_id == "sepal.length".into() {
                panic!()
            } 
        }

        let neuron_2 = magds.neuron("2".into(), "iris".into()).unwrap();
        println!("neuron_2 {}", neuron_2.borrow());
        for (id, sensor) in &neuron_2.borrow().explain() {
            println!("{id}");
            if id.parent_id == "petal.length".into() {
                assert_eq!(sensor.borrow().id().id, "1.4".into());
            } else if id.parent_id == "petal.width".into() {
                assert_eq!(sensor.borrow().id().id, "0.2".into());
            } else if id.parent_id == "sepal.width".into() {
                assert_eq!(sensor.borrow().id().id, "3".into());
            } else if id.parent_id == "variety".into() {
                assert_eq!(sensor.borrow().id().id, "Setosa".into());
            } else if id.parent_id == "sepal.length".into() {
                assert_eq!(sensor.borrow().id().id, "4.9".into());
            } 
        }
    }

    #[test]
    fn csv_to_sensors() {
        let mut magds = MAGDS::new();

        let df = polars_common::csv_to_dataframe("data/iris.csv");
        assert!(df.is_ok());
        let df = df.unwrap();
        println!("{}", df);

        let variety_df = df.column("variety").unwrap();
        assert_eq!(*variety_df.dtype(), DataType::Utf8);
        let variety_df_datavec = polars_common::series_to_datavec_skipna(variety_df).unwrap();
        let variety_graph = super::sensor_from_datavec(
            &mut magds, "variety", &variety_df_datavec
        );
        assert!(variety_graph.is_some());
        let variety_graph = variety_graph.unwrap();
        println!("{}", variety_graph.borrow());
        let variety_from_magds = magds.sensor("variety".into()).unwrap();
        let versicolor_result = variety_from_magds.borrow().search(
            &"Versicolor".to_string().into()
        );
        assert!(versicolor_result.is_some());
        assert_eq!(versicolor_result.unwrap().borrow().counter(), 50);
        
        let sepal_length_df = df.column("sepal.length").unwrap();
        assert_eq!(*sepal_length_df.dtype(), DataType::Float64);
        let sepal_length_df_datavec = 
            polars_common::series_to_datavec_skipna(sepal_length_df).unwrap();
        let sepal_length_graph = super::sensor_from_datavec(
            &mut magds, "sepal.length", &sepal_length_df_datavec
        );

        assert!(sepal_length_graph.is_some());
        let sepal_length_graph = sepal_length_graph.unwrap();
        println!("{}", sepal_length_graph.borrow());
        let sepal_length_graph_from_magds = magds.sensor("sepal.length".into()).unwrap();
        let sepal_length_result = sepal_length_graph_from_magds.borrow().search(&5.8_f64.into());
        assert!(sepal_length_result.is_some());
        assert_eq!(sepal_length_result.unwrap().borrow().counter(), 7);
    }
}
// use std::{
//     rc::Rc,
//     cell::RefCell
// };

// use asa_graphs::neural::graph::ASAGraph;

// use bionet_common::{
//     polars::DataVec,
//     sensor::Sensor
// };

// use crate::dynamic::{
//     magds::MAGDS,
//     sensor::SensorConatiner
// };

// #[allow(dead_code)]
// pub(crate) fn sensor_from_datavec(
//     magds: &mut MAGDS, id: &str, data: &DataVec
// ) -> Option<Rc<RefCell<SensorConatiner>>> {
//     match data {
//         DataVec::Unknown => {
//             log::error!("can't parse vec data type for sensor {id}");
//             return None
//         }
//         DataVec::BoolVec(vec) => {
//             let graph = ASAGraph::<_>::new_box_from_vec(id, vec) as Box<dyn Sensor<bool>>;
//             magds.add_sensor(Rc::new(RefCell::new(graph.into())))
//         }
//         DataVec::UInt8Vec(vec) => {
//             let graph = ASAGraph::<_>::new_box_from_vec(id, vec) as Box<dyn Sensor<u8>>;
//             magds.add_sensor(Rc::new(RefCell::new(graph.into())))
//         }
//         DataVec::UInt16Vec(vec) => {
//             let graph = ASAGraph::<_>::new_box_from_vec(id, vec) as Box<dyn Sensor<u16>>;
//             magds.add_sensor(Rc::new(RefCell::new(graph.into())))
//         }
//         DataVec::UInt32Vec(vec) => {
//             let graph = ASAGraph::<_>::new_box_from_vec(id, vec) as Box<dyn Sensor<u32>>;
//             magds.add_sensor(Rc::new(RefCell::new(graph.into())))
//         }
//         DataVec::UInt64Vec(vec) => {
//             let graph = ASAGraph::<_>::new_box_from_vec(id, vec) as Box<dyn Sensor<u64>>;
//             magds.add_sensor(Rc::new(RefCell::new(graph.into())))
//         }
//         DataVec::Int8Vec(vec) => {
//             let graph = ASAGraph::<_>::new_box_from_vec(id, vec) as Box<dyn Sensor<i8>>;
//             magds.add_sensor(Rc::new(RefCell::new(graph.into())))
//         }
//         DataVec::Int16Vec(vec) => {
//             let graph = ASAGraph::<_>::new_box_from_vec(id, vec) as Box<dyn Sensor<i16>>;
//             magds.add_sensor(Rc::new(RefCell::new(graph.into())))
//         }
//         DataVec::Int32Vec(vec) => {
//             let graph = ASAGraph::<_>::new_box_from_vec(id, vec) as Box<dyn Sensor<i32>>;
//             magds.add_sensor(Rc::new(RefCell::new(graph.into())))
//         }
//         DataVec::Int64Vec(vec) => {
//             let graph = ASAGraph::<_>::new_box_from_vec(id, vec) as Box<dyn Sensor<i64>>;
//             magds.add_sensor(Rc::new(RefCell::new(graph.into())))
//         }
//         DataVec::Float32Vec(vec) => {
//             let graph = ASAGraph::<_>::new_box_from_vec(id, vec) as Box<dyn Sensor<f32>>;
//             magds.add_sensor(Rc::new(RefCell::new(graph.into())))
//         }
//         DataVec::Float64Vec(vec) => {
//             let graph = ASAGraph::<_>::new_box_from_vec(id, vec) as Box<dyn Sensor<f64>>;
//             magds.add_sensor(Rc::new(RefCell::new(graph.into())))
//         }
//         DataVec::Utf8Vec(vec) => {
//             let graph = ASAGraph::<String>::new_box_from_vec(id, vec) as Box<dyn Sensor<String>>;
//             magds.add_sensor(Rc::new(RefCell::new(graph.into())))
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use polars::datatypes::DataType;

//     use bionet_common::{
//         polars as polars_common,
//         sensor::Sensor
//     };

//     use crate::dynamic::magds::MAGDS;

//     #[test]
//     fn csv_to_dataframe() {
//         let mut magds = MAGDS::new();

//         let df = polars_common::csv_to_dataframe("data/iris.csv");
//         assert!(df.is_ok());
//         let df = df.unwrap();
//         println!("{}", df);

//         let variety_df = df.column("variety").unwrap();
//         assert_eq!(*variety_df.dtype(), DataType::Utf8);
//         let variety_df_datavec = polars_common::series_to_datavec(variety_df).unwrap();
//         let variety_graph = super::sensor_from_datavec(
//             &mut magds, "variety", &variety_df_datavec
//         );
//         assert!(variety_graph.is_some());
//         let variety_graph = variety_graph.unwrap();
//         println!("{}", variety_graph.borrow());
//         let variety_from_magds = magds.sensor("variety".into()).unwrap();
//         let versicolor_result = variety_from_magds.borrow().search(
//             &"Versicolor".to_string().into()
//         );
//         assert!(versicolor_result.is_some());
//         assert_eq!(versicolor_result.unwrap().borrow().counter(), 50);
        
//         let sepal_length_df = df.column("sepal.length").unwrap();
//         assert_eq!(*sepal_length_df.dtype(), DataType::Float64);
//         let sepal_length_df_datavec = polars_common::series_to_datavec(sepal_length_df).unwrap();
//         let sepal_length_graph = super::sensor_from_datavec(
//             &mut magds, "sepal.length", &sepal_length_df_datavec
//         );

//         assert!(sepal_length_graph.is_some());
//         let sepal_length_graph = sepal_length_graph.unwrap();
//         println!("{}", sepal_length_graph.borrow());
//         let sepal_length_graph_from_magds = magds.sensor("sepal.length".into()).unwrap();
//         let sepal_length_result = sepal_length_graph_from_magds.borrow().search(&5.8_f64.into());
//         assert!(sepal_length_result.is_some());
//         assert_eq!(sepal_length_result.unwrap().borrow().counter(), 7);
//     }
// }