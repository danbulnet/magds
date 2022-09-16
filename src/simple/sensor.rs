use std::{
    rc::Rc,
    cell::RefCell,
    collections::HashMap
};

use enum_as_inner::EnumAsInner;

use bionet_common::{
    neuron::{ Neuron, NeuronID },
    sensor::Sensor,
    data::{ DataType, DataTypeValue, DataCategory }
};

use asa_graphs::neural::graph::ASAGraph;

#[derive(EnumAsInner)]
pub enum SensorConatiner {
    Bool(ASAGraph<bool>),
    U8(ASAGraph<u8>),
    U16(ASAGraph<u16>),
    U32(ASAGraph<u32>),
    U64(ASAGraph<u64>),
    U128(ASAGraph<u128>),
    USize(ASAGraph<usize>),
    I8(ASAGraph<i8>),
    I16(ASAGraph<i16>),
    I32(ASAGraph<i32>),
    I64(ASAGraph<i64>),
    I128(ASAGraph<i128>),
    ISize(ASAGraph<isize>),
    F32(ASAGraph<f32>),
    F64(ASAGraph<f64>),
    RcStr(ASAGraph<Rc<str>>),
    String(ASAGraph<String>)
}

impl Sensor<DataTypeValue> for SensorConatiner {
    fn id(&self) -> Rc<str> {
        match self {
            SensorConatiner::Bool(v) => v.id(),
            SensorConatiner::U8(v) => v.id(),
            SensorConatiner::U16(v) => v.id(),
            SensorConatiner::U32(v) => v.id(),
            SensorConatiner::U64(v) => v.id(),
            SensorConatiner::U128(v) => v.id(),
            SensorConatiner::USize(v) => v.id(),
            SensorConatiner::I8(v) => v.id(),
            SensorConatiner::I16(v) => v.id(),
            SensorConatiner::I32(v) => v.id(),
            SensorConatiner::I64(v) => v.id(),
            SensorConatiner::I128(v) => v.id(),
            SensorConatiner::ISize(v) => v.id(),
            SensorConatiner::F32(v) => v.id(),
            SensorConatiner::F64(v) => v.id(),
            SensorConatiner::RcStr(v) => v.id(),
            SensorConatiner::String(v) => v.id()
        }
    }

    fn data_type(&self) -> DataType {
        match self {
            SensorConatiner::Bool(v) => v.data_type(),
            SensorConatiner::U8(v) => v.data_type(),
            SensorConatiner::U16(v) => v.data_type(),
            SensorConatiner::U32(v) => v.data_type(),
            SensorConatiner::U64(v) => v.data_type(),
            SensorConatiner::U128(v) => v.data_type(),
            SensorConatiner::USize(v) => v.data_type(),
            SensorConatiner::I8(v) => v.data_type(),
            SensorConatiner::I16(v) => v.data_type(),
            SensorConatiner::I32(v) => v.data_type(),
            SensorConatiner::I64(v) => v.data_type(),
            SensorConatiner::I128(v) => v.data_type(),
            SensorConatiner::ISize(v) => v.data_type(),
            SensorConatiner::F32(v) => v.data_type(),
            SensorConatiner::F64(v) => v.data_type(),
            SensorConatiner::RcStr(v) => v.data_type(),
            SensorConatiner::String(v) => v.data_type()
        }
    }

    fn data_category(&self) -> DataCategory {
        match self {
            SensorConatiner::Bool(v) => v.data_category(),
            SensorConatiner::U8(v) => v.data_category(),
            SensorConatiner::U16(v) => v.data_category(),
            SensorConatiner::U32(v) => v.data_category(),
            SensorConatiner::U64(v) => v.data_category(),
            SensorConatiner::U128(v) => v.data_category(),
            SensorConatiner::USize(v) => v.data_category(),
            SensorConatiner::I8(v) => v.data_category(),
            SensorConatiner::I16(v) => v.data_category(),
            SensorConatiner::I32(v) => v.data_category(),
            SensorConatiner::I64(v) => v.data_category(),
            SensorConatiner::I128(v) => v.data_category(),
            SensorConatiner::ISize(v) => v.data_category(),
            SensorConatiner::F32(v) => v.data_category(),
            SensorConatiner::F64(v) => v.data_category(),
            SensorConatiner::RcStr(v) => v.data_category(),
            SensorConatiner::String(v) => v.data_category()
        }
    }

    fn insert(&mut self, item: &DataTypeValue) -> Rc<RefCell<dyn Neuron>> {
        match self {
            SensorConatiner::Bool(v) => {
                v.insert(item.as_bool().unwrap())
            },
            SensorConatiner::U8(v) => {
                v.insert(item.as_u8().unwrap())
            },
            SensorConatiner::U16(v) => {
                v.insert(item.as_u16().unwrap())
            },
            SensorConatiner::U32(v) => {
                v.insert(item.as_u32().unwrap())
            },
            SensorConatiner::U64(v) => {
                v.insert(item.as_u64().unwrap())
            },
            SensorConatiner::U128(v) => {
                v.insert(item.as_u128().unwrap())
            },
            SensorConatiner::USize(v) => {
                v.insert(item.as_u_size().unwrap())
            },
            SensorConatiner::I8(v) => {
                v.insert(item.as_i8().unwrap())
            },
            SensorConatiner::I16(v) => {
                v.insert(item.as_i16().unwrap())
            },
            SensorConatiner::I32(v) => {
                v.insert(item.as_i32().unwrap())
            },
            SensorConatiner::I64(v) => {
                v.insert(item.as_i64().unwrap())
            },
            SensorConatiner::I128(v) => {
                v.insert(item.as_i128().unwrap())
            },
            SensorConatiner::ISize(v) => {
                v.insert(item.as_i_size().unwrap())
            },
            SensorConatiner::F32(v) => {
                v.insert(item.as_f32().unwrap())
            },
            SensorConatiner::F64(v) => {
                v.insert(item.as_f64().unwrap())
            },
            SensorConatiner::RcStr(v) => {
                v.insert(item.as_rc_str().unwrap())
            },
            SensorConatiner::String(v) => {
                v.insert(item.as_string().unwrap())
            }
        }
    }

    fn search(&self, item: &DataTypeValue) -> Option<Rc<RefCell<dyn Neuron>>> {
        match self {
            SensorConatiner::Bool(v) => {
                Some(v.search(item.as_bool()?)? as Rc<RefCell<dyn Neuron>>)
            }
            SensorConatiner::U8(v) => {
                Some(v.search(item.as_u8()?)? as Rc<RefCell<dyn Neuron>>)
            }
            SensorConatiner::U16(v) => {
                Some(v.search(item.as_u16()?)? as Rc<RefCell<dyn Neuron>>)
            }
            SensorConatiner::U32(v) => {
                Some(v.search(item.as_u32()?)? as Rc<RefCell<dyn Neuron>>)
            }
            SensorConatiner::U64(v) => {
                Some(v.search(item.as_u64()?)? as Rc<RefCell<dyn Neuron>>)
            }
            SensorConatiner::U128(v) => {
                Some(v.search(item.as_u128()?)? as Rc<RefCell<dyn Neuron>>)
            }
            SensorConatiner::USize(v) => {
                Some(v.search(item.as_u_size()?)? as Rc<RefCell<dyn Neuron>>)
            }
            SensorConatiner::I8(v) => {
                Some(v.search(item.as_i8()?)? as Rc<RefCell<dyn Neuron>>)
            }
            SensorConatiner::I16(v) => {
                Some(v.search(item.as_i16()?)? as Rc<RefCell<dyn Neuron>>)
            }
            SensorConatiner::I32(v) => {
                Some(v.search(item.as_i32()?)? as Rc<RefCell<dyn Neuron>>)
            }
            SensorConatiner::I64(v) => {
                Some(v.search(item.as_i64()?)? as Rc<RefCell<dyn Neuron>>)
            }
            SensorConatiner::I128(v) => {
                Some(v.search(item.as_i128()?)? as Rc<RefCell<dyn Neuron>>)
            }
            SensorConatiner::ISize(v) => {
                Some(v.search(item.as_i_size()?)? as Rc<RefCell<dyn Neuron>>)
            }
            SensorConatiner::F32(v) => {
                Some(v.search(item.as_f32()?)? as Rc<RefCell<dyn Neuron>>)
            }
            SensorConatiner::F64(v) => {
                Some(v.search(item.as_f64()?)? as Rc<RefCell<dyn Neuron>>)
            }
            SensorConatiner::RcStr(v) => {
                Some(v.search(item.as_rc_str()?)? as Rc<RefCell<dyn Neuron>>)
            }
            SensorConatiner::String(v) => {
                Some(v.search(item.as_string()?)? as Rc<RefCell<dyn Neuron>>)
            }
        }
    }

    fn activate(
        &mut self, 
        item: &DataTypeValue, 
        signal: f32, 
        propagate_horizontal: bool, 
        propagate_vertical: bool
    ) -> Result<HashMap<NeuronID, Rc<RefCell<dyn Neuron>>>, String> {
        match self {
            SensorConatiner::Bool(v) => {
                v.activate(
                    item.as_bool().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::U8(v) => {
                v.activate(
                    item.as_u8().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::U16(v) => {
                v.activate(
                    item.as_u16().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::U32(v) => {
                v.activate(
                    item.as_u32().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::U64(v) => {
                v.activate(
                    item.as_u64().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::U128(v) => {
                v.activate(
                    item.as_u128().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::USize(v) => {
                v.activate(
                    item.as_u_size().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::I8(v) => {
                v.activate(
                    item.as_i8().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::I16(v) => {
                v.activate(
                    item.as_i16().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::I32(v) => {
                v.activate(
                    item.as_i32().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::I64(v) => {
                v.activate(
                    item.as_i64().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::I128(v) => {
                v.activate(
                    item.as_i128().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::ISize(v) => {
                v.activate(
                    item.as_i_size().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::F32(v) => {
                v.activate(
                    item.as_f32().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::F64(v) => {
                v.activate(
                    item.as_f64().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::RcStr(v) => {
                v.activate(
                    item.as_rc_str().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::String(v) => {
                v.activate(
                    item.as_string().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            }
        }
    }

    fn deactivate(
        &mut self, 
        item: &DataTypeValue, 
        propagate_horizontal: bool, 
        propagate_vertical: bool
    ) -> Result<(), String> {
        match self {
            SensorConatiner::Bool(v) => {
                v.deactivate(
                    item.as_bool().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::U8(v) => {
                v.deactivate(
                    item.as_u8().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::U16(v) => {
                v.deactivate(
                    item.as_u16().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::U32(v) => {
                v.deactivate(
                    item.as_u32().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::U64(v) => {
                v.deactivate(
                    item.as_u64().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::U128(v) => {
                v.deactivate(
                    item.as_u128().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::USize(v) => {
                v.deactivate(
                    item.as_u_size().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::I8(v) => {
                v.deactivate(
                    item.as_i8().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::I16(v) => {
                v.deactivate(
                    item.as_i16().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::I32(v) => {
                v.deactivate(
                    item.as_i32().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::I64(v) => {
                v.deactivate(
                    item.as_i64().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::I128(v) => {
                v.deactivate(
                    item.as_i128().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::ISize(v) => {
                v.deactivate(
                    item.as_i_size().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::F32(v) => {
                v.deactivate(
                    item.as_f32().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::F64(v) => {
                v.deactivate(
                    item.as_f64().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::RcStr(v) => {
                v.deactivate(
                    item.as_rc_str().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::String(v) => {
                v.deactivate(
                    item.as_string().unwrap(), propagate_horizontal, propagate_vertical
                )
            }
        }
    }

    fn deactivate_sensor(&mut self) {
        match self {
            SensorConatiner::Bool(v) => v.deactivate_sensor(),
            SensorConatiner::U8(v) => v.deactivate_sensor(),
            SensorConatiner::U16(v) => v.deactivate_sensor(),
            SensorConatiner::U32(v) => v.deactivate_sensor(),
            SensorConatiner::U64(v) => v.deactivate_sensor(),
            SensorConatiner::U128(v) => v.deactivate_sensor(),
            SensorConatiner::USize(v) => v.deactivate_sensor(),
            SensorConatiner::I8(v) => v.deactivate_sensor(),
            SensorConatiner::I16(v) => v.deactivate_sensor(),
            SensorConatiner::I32(v) => v.deactivate_sensor(),
            SensorConatiner::I64(v) => v.deactivate_sensor(),
            SensorConatiner::I128(v) => v.deactivate_sensor(),
            SensorConatiner::ISize(v) => v.deactivate_sensor(),
            SensorConatiner::F32(v) => v.deactivate_sensor(),
            SensorConatiner::F64(v) => v.deactivate_sensor(),
            SensorConatiner::RcStr(v) => v.deactivate_sensor(),
            SensorConatiner::String(v) => v.deactivate_sensor()
        }
    }
}