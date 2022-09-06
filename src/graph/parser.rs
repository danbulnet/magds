use std::{
    rc::Rc,
    cell::RefCell
};

use polars::prelude::*;

use asa_graphs::neural::graph::ASAGraph;

use bionet_common::{
    sensor::{ SensorDynamic, SensorDataDynamic, SensorDynamicDowncast },
    data::{ DataVec, DataCategory },
};

use crate::graph::magds::MAGDS;

pub(crate) fn asagraph_from_vec(
    magds: &mut MAGDS, id: &str, data: &DataVec
) -> Option<Rc<RefCell<dyn SensorDynamic<Data = dyn SensorDataDynamic>>>> {
    match data {
        DataVec::Unknown => {
            log::error!("can't parse vec data type for sensor {id}");
            return None
        }
        DataVec::BoolVec(vec) => {
            let data_category = DataCategory::from(&vec[..]);
            asagraph_from_vec_inner(magds, id, data_category, vec);
        }
        DataVec::UInt8Vec(vec) => {
            let data_category = DataCategory::from(&vec[..]);
            asagraph_from_vec_inner(magds, id, data_category, vec);
        }
        DataVec::UInt16Vec(vec) => {
            let data_category = DataCategory::from(&vec[..]);
            asagraph_from_vec_inner(magds, id, data_category, vec);
        }
        DataVec::UInt32Vec(vec) => {
            let data_category = DataCategory::from(&vec[..]);
            asagraph_from_vec_inner(magds, id, data_category, vec);
        }
        DataVec::UInt64Vec(vec) => {
            let data_category = DataCategory::from(&vec[..]);
            asagraph_from_vec_inner(magds, id, data_category, vec);
        }
        DataVec::Int8Vec(vec) => {
            let data_category = DataCategory::from(&vec[..]);
            asagraph_from_vec_inner(magds, id, data_category, vec);
        }
        DataVec::Int16Vec(vec) => {
            let data_category = DataCategory::from(&vec[..]);
            asagraph_from_vec_inner(magds, id, data_category, vec);
        }
        DataVec::Int32Vec(vec) => {
            let data_category = DataCategory::from(&vec[..]);
            asagraph_from_vec_inner(magds, id, data_category, vec);
        }
        DataVec::Int64Vec(vec) => {
            let data_category = DataCategory::from(&vec[..]);
            asagraph_from_vec_inner(magds, id, data_category, vec);
        }
        DataVec::Float32Vec(vec) => {
            let data_category = DataCategory::from(&vec[..]);
            asagraph_from_vec_inner(magds, id, data_category, vec);
        }
        DataVec::Float64Vec(vec) => {
            let data_category = DataCategory::from(&vec[..]);
            asagraph_from_vec_inner(magds, id, data_category, vec);
        }
        DataVec::Utf8Vec(vec) => {
            let data_category = DataCategory::from(&vec[..]);
            asagraph_from_vec_inner(magds, id, data_category, vec);
        }
    }
    magds.sensor_dynamic(id)
}

fn asagraph_from_vec_inner<T: SensorDataDynamic>(
    magds: &mut MAGDS, id: &str, data_category: DataCategory, data: &Vec<T>
) -> Option<Rc<RefCell<dyn SensorDynamic<Data = T>>>> {
    let graph = ASAGraph::<_, 25>::new_rc_from_vec(id, data_category, &data[..]);
    magds.add_sensor(graph);
    Some(magds.sensor(id).unwrap())
}

#[cfg(test)]
mod tests {
    use polars::datatypes::DataType;

    use bionet_common::{
        polars as polars_common,
        data::{DataVec, DataCategory}
    };

    use asa_graphs::neural::graph::ASAGraph;

    use super::MAGDS;

    #[test]
    fn csv_to_dataframe() {
        let working_dir = std::env::current_dir().unwrap();
        println!("{}", working_dir.display());

        let filename = "data/iris.csv";
        let df = polars_common::csv_to_dataframe(filename);
        assert!(df.is_ok());
        let df = df.unwrap();
        println!("{}", df);

        let cd = df.column("sepal.length").unwrap();
        let cdvv = polars_common::series_to_vec(cd).unwrap();
        println!("{}", cd);
        let cddt = cd.dtype();

        assert_eq!(*cddt, DataType::Float64);

        let s: Vec<Option<f64>> = cd.f64().unwrap().into_iter().collect();
        println!("{}", cd);

        let v = df.column("variety").unwrap();
        println!("{}", v);
        println!("v_dtype: {}", v.dtype());
        assert_eq!(*v.dtype(), DataType::Utf8);
        let vv = polars_common::series_to_vec(v).unwrap();
        let mut magds = MAGDS::new();
        let graph = super::asagraph_from_vec(&mut magds, "test", &vv);
        unsafe {
            let vv_from_magds: &mut ASAGraph<String, 25> =
                magds.sensor_base::<String, ASAGraph<String, 25>>("test").unwrap();
            vv_from_magds.print_graph();
        }
    }
}