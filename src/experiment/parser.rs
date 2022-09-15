use std::{
    rc::Rc,
    cell::RefCell,
    marker::PhantomData
};

use polars::prelude::*;

use asa_graphs::neural::graph::ASAGraph;

use bionet_common::{
    sensor::{ Sensor, SensorData },
    data::{ DataVec, DataCategory, DataDeductor },
};

use crate::dynamic::magds::MAGDS;

pub(crate) fn asagraph_from_datavec(
    magds: &mut MAGDS, id: &str, data: &DataVec
) -> Option<Rc<RefCell<dyn Sensor<dyn SensorData>>>> {
    match data {
        DataVec::Unknown => {
            log::error!("can't parse vec data type for sensor {id}");
            return None
        }
        DataVec::BoolVec(vec) => {
            let data_category = DataCategory::from(&vec[..]);
            asagraph_from_datavec_inner(magds, id, data_category, vec);
        }
        DataVec::UInt8Vec(vec) => {
            let data_category = DataCategory::from(&vec[..]);
            asagraph_from_datavec_inner(magds, id, data_category, vec);
        }
        DataVec::UInt16Vec(vec) => {
            let data_category = DataCategory::from(&vec[..]);
            asagraph_from_datavec_inner(magds, id, data_category, vec);
        }
        DataVec::UInt32Vec(vec) => {
            let data_category = DataCategory::from(&vec[..]);
            asagraph_from_datavec_inner(magds, id, data_category, vec);
        }
        DataVec::UInt64Vec(vec) => {
            let data_category = DataCategory::from(&vec[..]);
            asagraph_from_datavec_inner(magds, id, data_category, vec);
        }
        DataVec::Int8Vec(vec) => {
            let data_category = DataCategory::from(&vec[..]);
            asagraph_from_datavec_inner(magds, id, data_category, vec);
        }
        DataVec::Int16Vec(vec) => {
            let data_category = DataCategory::from(&vec[..]);
            asagraph_from_datavec_inner(magds, id, data_category, vec);
        }
        DataVec::Int32Vec(vec) => {
            let data_category = DataCategory::from(&vec[..]);
            asagraph_from_datavec_inner(magds, id, data_category, vec);
        }
        DataVec::Int64Vec(vec) => {
            let data_category = DataCategory::from(&vec[..]);
            asagraph_from_datavec_inner(magds, id, data_category, vec);
        }
        DataVec::Float32Vec(vec) => {
            let data_category = DataCategory::from(&vec[..]);
            asagraph_from_datavec_inner(magds, id, data_category, vec);
        }
        DataVec::Float64Vec(vec) => {
            let data_category = DataCategory::from(&vec[..]);
            asagraph_from_datavec_inner(magds, id, data_category, vec);
        }
        DataVec::Utf8Vec(vec) => {
            let data_category = DataCategory::from(&vec[..]);
            asagraph_from_datavec_inner(magds, id, data_category, vec);
        }
    }
    magds.sensor_dynamic(id)
}

fn asagraph_from_datavec_inner<T: SensorData>(
    magds: &mut MAGDS, id: &str, data_category: DataCategory, data: &Vec<T>
) -> Option<Rc<RefCell<dyn Sensor<T>>>> where PhantomData<T>: DataDeductor {
    let graph = ASAGraph::<_, 25>::new_rc_from_vec(id, &data[..]);
    magds.add_sensor(graph);
    Some(magds.sensor(id).unwrap())
}

#[cfg(test)]
mod tests {
    use polars::datatypes::DataType;

    use bionet_common::{
        polars as polars_common
    };

    use asa_graphs::neural::graph::ASAGraph;

    use crate::dynamic::magds::MAGDS;

    #[test]
    fn csv_to_dataframe() {
        let mut magds = MAGDS::new();

        let filename = "data/iris.csv";
        let df = polars_common::csv_to_dataframe(filename);
        assert!(df.is_ok());
        let df = df.unwrap();
        println!("{}", df);

        let variety_df = df.column("variety").unwrap();
        assert_eq!(*variety_df.dtype(), DataType::Utf8);
        let variety_df_datavec = polars_common::series_to_datavec(variety_df).unwrap();
        let _graph = super::asagraph_from_datavec(&mut magds, "variety", &variety_df_datavec);
        unsafe {
            let vv_from_magds: &mut ASAGraph<String, 25> =
                magds.sensor_base::<ASAGraph<String, 25>, String>("variety").unwrap();
            vv_from_magds.print_graph();
        }
        // assert!(magds.sensor("variety").is_some());
        let variety_from_magds = magds.sensor("variety").unwrap();
        let versicolor_result = variety_from_magds.borrow().search(&"Versicolor".to_string());
        assert!(versicolor_result.is_some());
        assert_eq!(versicolor_result.unwrap().borrow().counter(), 50);

        
        let sepal_length_df = df.column("sepal.length").unwrap();
        assert_eq!(*sepal_length_df.dtype(), DataType::Float64);
        let sepal_length_df_datavec = polars_common::series_to_datavec(sepal_length_df).unwrap();
        let _graph = super::asagraph_from_datavec(&mut magds, "sepal.length", &sepal_length_df_datavec);
        assert!(magds.sensor::<f64>("sepal.length").is_some());
        let sepal_length_from_magds = magds.sensor::<f64>("sepal.length").unwrap();
        let versicolor_result = sepal_length_from_magds.borrow().search(&5.8_f64);
        assert!(versicolor_result.is_some());
        assert_eq!(versicolor_result.unwrap().borrow().counter(), 7);
    }
}