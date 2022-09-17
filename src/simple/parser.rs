use std::{
    rc::Rc,
    cell::RefCell
};

use asa_graphs::neural::graph::ASAGraph;

use bionet_common::{
    polars::DataVec
};

use crate::simple::{
    magds::MAGDS,
    sensor::SensorConatiner
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
            let graph = ASAGraph::<_>::new_from_vec(id, vec);
            magds.add_sensor(Rc::new(RefCell::new(graph.into())))
        }
        DataVec::UInt8Vec(vec) => {
            let graph = ASAGraph::<_>::new_from_vec(id, vec);
            magds.add_sensor(Rc::new(RefCell::new(graph.into())))
        }
        DataVec::UInt16Vec(vec) => {
            let graph = ASAGraph::<_>::new_from_vec(id, vec);
            magds.add_sensor(Rc::new(RefCell::new(graph.into())))
        }
        DataVec::UInt32Vec(vec) => {
            let graph = ASAGraph::<_>::new_from_vec(id, vec);
            magds.add_sensor(Rc::new(RefCell::new(graph.into())))
        }
        DataVec::UInt64Vec(vec) => {
            let graph = ASAGraph::<_>::new_from_vec(id, vec);
            magds.add_sensor(Rc::new(RefCell::new(graph.into())))
        }
        DataVec::Int8Vec(vec) => {
            let graph = ASAGraph::<_>::new_from_vec(id, vec);
            magds.add_sensor(Rc::new(RefCell::new(graph.into())))
        }
        DataVec::Int16Vec(vec) => {
            let graph = ASAGraph::<_>::new_from_vec(id, vec);
            magds.add_sensor(Rc::new(RefCell::new(graph.into())))
        }
        DataVec::Int32Vec(vec) => {
            let graph = ASAGraph::<_>::new_from_vec(id, vec);
            magds.add_sensor(Rc::new(RefCell::new(graph.into())))
        }
        DataVec::Int64Vec(vec) => {
            let graph = ASAGraph::<_>::new_from_vec(id, vec);
            magds.add_sensor(Rc::new(RefCell::new(graph.into())))
        }
        DataVec::Float32Vec(vec) => {
            let graph = ASAGraph::<_>::new_from_vec(id, vec);
            magds.add_sensor(Rc::new(RefCell::new(graph.into())))
        }
        DataVec::Float64Vec(vec) => {
            let graph = ASAGraph::<_>::new_from_vec(id, vec);
            magds.add_sensor(Rc::new(RefCell::new(graph.into())))
        }
        DataVec::Utf8Vec(vec) => {
            let graph = ASAGraph::<String>::new_from_vec(id, vec);
            magds.add_sensor(Rc::new(RefCell::new(graph.into())))
        }
    }
}

#[cfg(test)]
mod tests {
    use polars::datatypes::DataType;

    use bionet_common::{
        polars as polars_common,
        sensor::Sensor
    };

    use crate::simple::magds::MAGDS;

    #[test]
    fn csv_to_dataframe() {
        let mut magds = MAGDS::new();

        let df = polars_common::csv_to_dataframe("data/iris.csv");
        assert!(df.is_ok());
        let df = df.unwrap();
        println!("{}", df);

        let variety_df = df.column("variety").unwrap();
        assert_eq!(*variety_df.dtype(), DataType::Utf8);
        let variety_df_datavec = polars_common::series_to_datavec(variety_df).unwrap();
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
        let sepal_length_df_datavec = polars_common::series_to_datavec(sepal_length_df).unwrap();
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