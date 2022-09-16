use std::{
    rc::Rc,
    cell::RefCell,
    collections::HashMap
};

use enum_as_inner::EnumAsInner;

use bionet_common::{
    neuron::{ Neuron, NeuronID },
    sensor::Sensor,
    data::{ DataType, DataTypeValue, DataCategory}
};

#[derive(EnumAsInner)]
pub enum SensorConatiner {
    Bool(Rc<RefCell<dyn Sensor<bool>>>),
    U8(Rc<RefCell<dyn Sensor<u8>>>),
    U16(Rc<RefCell<dyn Sensor<u16>>>),
    U32(Rc<RefCell<dyn Sensor<u32>>>),
    U64(Rc<RefCell<dyn Sensor<u64>>>),
    U128(Rc<RefCell<dyn Sensor<u128>>>),
    USize(Rc<RefCell<dyn Sensor<usize>>>),
    I8(Rc<RefCell<dyn Sensor<i8>>>),
    I16(Rc<RefCell<dyn Sensor<i16>>>),
    I32(Rc<RefCell<dyn Sensor<i32>>>),
    I64(Rc<RefCell<dyn Sensor<i64>>>),
    I128(Rc<RefCell<dyn Sensor<i128>>>),
    ISize(Rc<RefCell<dyn Sensor<isize>>>),
    F32(Rc<RefCell<dyn Sensor<f32>>>),
    F64(Rc<RefCell<dyn Sensor<f64>>>),
    RcStr(Rc<RefCell<dyn Sensor<Rc<str>>>>),
    String(Rc<RefCell<dyn Sensor<String>>>)
}

impl Sensor<DataTypeValue> for SensorConatiner {
    fn id(&self) -> Rc<str> {
        match self {
            SensorConatiner::Bool(v) => v.borrow().id(),
            SensorConatiner::U8(v) => v.borrow().id(),
            SensorConatiner::U16(v) => v.borrow().id(),
            SensorConatiner::U32(v) => v.borrow().id(),
            SensorConatiner::U64(v) => v.borrow().id(),
            SensorConatiner::U128(v) => v.borrow().id(),
            SensorConatiner::USize(v) => v.borrow().id(),
            SensorConatiner::I8(v) => v.borrow().id(),
            SensorConatiner::I16(v) => v.borrow().id(),
            SensorConatiner::I32(v) => v.borrow().id(),
            SensorConatiner::I64(v) => v.borrow().id(),
            SensorConatiner::I128(v) => v.borrow().id(),
            SensorConatiner::ISize(v) => v.borrow().id(),
            SensorConatiner::F32(v) => v.borrow().id(),
            SensorConatiner::F64(v) => v.borrow().id(),
            SensorConatiner::RcStr(v) => v.borrow().id(),
            SensorConatiner::String(v) => v.borrow().id()
        }
    }

    fn data_type(&self) -> DataType {
        match self {
            SensorConatiner::Bool(v) => v.borrow().data_type(),
            SensorConatiner::U8(v) => v.borrow().data_type(),
            SensorConatiner::U16(v) => v.borrow().data_type(),
            SensorConatiner::U32(v) => v.borrow().data_type(),
            SensorConatiner::U64(v) => v.borrow().data_type(),
            SensorConatiner::U128(v) => v.borrow().data_type(),
            SensorConatiner::USize(v) => v.borrow().data_type(),
            SensorConatiner::I8(v) => v.borrow().data_type(),
            SensorConatiner::I16(v) => v.borrow().data_type(),
            SensorConatiner::I32(v) => v.borrow().data_type(),
            SensorConatiner::I64(v) => v.borrow().data_type(),
            SensorConatiner::I128(v) => v.borrow().data_type(),
            SensorConatiner::ISize(v) => v.borrow().data_type(),
            SensorConatiner::F32(v) => v.borrow().data_type(),
            SensorConatiner::F64(v) => v.borrow().data_type(),
            SensorConatiner::RcStr(v) => v.borrow().data_type(),
            SensorConatiner::String(v) => v.borrow().data_type()
        }
    }

    fn data_category(&self) -> DataCategory {
        match self {
            SensorConatiner::Bool(v) => v.borrow().data_category(),
            SensorConatiner::U8(v) => v.borrow().data_category(),
            SensorConatiner::U16(v) => v.borrow().data_category(),
            SensorConatiner::U32(v) => v.borrow().data_category(),
            SensorConatiner::U64(v) => v.borrow().data_category(),
            SensorConatiner::U128(v) => v.borrow().data_category(),
            SensorConatiner::USize(v) => v.borrow().data_category(),
            SensorConatiner::I8(v) => v.borrow().data_category(),
            SensorConatiner::I16(v) => v.borrow().data_category(),
            SensorConatiner::I32(v) => v.borrow().data_category(),
            SensorConatiner::I64(v) => v.borrow().data_category(),
            SensorConatiner::I128(v) => v.borrow().data_category(),
            SensorConatiner::ISize(v) => v.borrow().data_category(),
            SensorConatiner::F32(v) => v.borrow().data_category(),
            SensorConatiner::F64(v) => v.borrow().data_category(),
            SensorConatiner::RcStr(v) => v.borrow().data_category(),
            SensorConatiner::String(v) => v.borrow().data_category()
        }
    }

    fn insert(&mut self, item: &DataTypeValue) -> Rc<RefCell<dyn Neuron>> {
        match self {
            SensorConatiner::Bool(v) => {
                v.borrow_mut().insert(item.as_bool().unwrap())
            },
            SensorConatiner::U8(v) => {
                v.borrow_mut().insert(item.as_u8().unwrap())
            },
            SensorConatiner::U16(v) => {
                v.borrow_mut().insert(item.as_u16().unwrap())
            },
            SensorConatiner::U32(v) => {
                v.borrow_mut().insert(item.as_u32().unwrap())
            },
            SensorConatiner::U64(v) => {
                v.borrow_mut().insert(item.as_u64().unwrap())
            },
            SensorConatiner::U128(v) => {
                v.borrow_mut().insert(item.as_u128().unwrap())
            },
            SensorConatiner::USize(v) => {
                v.borrow_mut().insert(item.as_u_size().unwrap())
            },
            SensorConatiner::I8(v) => {
                v.borrow_mut().insert(item.as_i8().unwrap())
            },
            SensorConatiner::I16(v) => {
                v.borrow_mut().insert(item.as_i16().unwrap())
            },
            SensorConatiner::I32(v) => {
                v.borrow_mut().insert(item.as_i32().unwrap())
            },
            SensorConatiner::I64(v) => {
                v.borrow_mut().insert(item.as_i64().unwrap())
            },
            SensorConatiner::I128(v) => {
                v.borrow_mut().insert(item.as_i128().unwrap())
            },
            SensorConatiner::ISize(v) => {
                v.borrow_mut().insert(item.as_i_size().unwrap())
            },
            SensorConatiner::F32(v) => {
                v.borrow_mut().insert(item.as_f32().unwrap())
            },
            SensorConatiner::F64(v) => {
                v.borrow_mut().insert(item.as_f64().unwrap())
            },
            SensorConatiner::RcStr(v) => {
                v.borrow_mut().insert(item.as_rc_str().unwrap())
            },
            SensorConatiner::String(v) => {
                v.borrow_mut().insert(item.as_string().unwrap())
            }
        }
    }

    fn search(&self, item: &DataTypeValue) -> Option<Rc<RefCell<dyn Neuron>>> {
        match self {
            SensorConatiner::Bool(v) => {
                v.borrow_mut().search(item.as_bool()?)
            },
            SensorConatiner::U8(v) => {
                v.borrow_mut().search(item.as_u8()?)
            },
            SensorConatiner::U16(v) => {
                v.borrow_mut().search(item.as_u16()?)
            },
            SensorConatiner::U32(v) => {
                v.borrow_mut().search(item.as_u32()?)
            },
            SensorConatiner::U64(v) => {
                v.borrow_mut().search(item.as_u64()?)
            },
            SensorConatiner::U128(v) => {
                v.borrow_mut().search(item.as_u128()?)
            },
            SensorConatiner::USize(v) => {
                v.borrow_mut().search(item.as_u_size()?)
            },
            SensorConatiner::I8(v) => {
                v.borrow_mut().search(item.as_i8()?)
            },
            SensorConatiner::I16(v) => {
                v.borrow_mut().search(item.as_i16()?)
            },
            SensorConatiner::I32(v) => {
                v.borrow_mut().search(item.as_i32()?)
            },
            SensorConatiner::I64(v) => {
                v.borrow_mut().search(item.as_i64()?)
            },
            SensorConatiner::I128(v) => {
                v.borrow_mut().search(item.as_i128()?)
            },
            SensorConatiner::ISize(v) => {
                v.borrow_mut().search(item.as_i_size()?)
            },
            SensorConatiner::F32(v) => {
                v.borrow_mut().search(item.as_f32()?)
            },
            SensorConatiner::F64(v) => {
                v.borrow_mut().search(item.as_f64()?)
            },
            SensorConatiner::RcStr(v) => {
                v.borrow_mut().search(item.as_rc_str()?)
            },
            SensorConatiner::String(v) => {
                v.borrow_mut().search(item.as_string()?)
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
                v.borrow_mut().activate(
                    item.as_bool().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::U8(v) => {
                v.borrow_mut().activate(
                    item.as_u8().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::U16(v) => {
                v.borrow_mut().activate(
                    item.as_u16().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::U32(v) => {
                v.borrow_mut().activate(
                    item.as_u32().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::U64(v) => {
                v.borrow_mut().activate(
                    item.as_u64().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::U128(v) => {
                v.borrow_mut().activate(
                    item.as_u128().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::USize(v) => {
                v.borrow_mut().activate(
                    item.as_u_size().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::I8(v) => {
                v.borrow_mut().activate(
                    item.as_i8().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::I16(v) => {
                v.borrow_mut().activate(
                    item.as_i16().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::I32(v) => {
                v.borrow_mut().activate(
                    item.as_i32().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::I64(v) => {
                v.borrow_mut().activate(
                    item.as_i64().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::I128(v) => {
                v.borrow_mut().activate(
                    item.as_i128().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::ISize(v) => {
                v.borrow_mut().activate(
                    item.as_i_size().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::F32(v) => {
                v.borrow_mut().activate(
                    item.as_f32().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::F64(v) => {
                v.borrow_mut().activate(
                    item.as_f64().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::RcStr(v) => {
                v.borrow_mut().activate(
                    item.as_rc_str().unwrap(), signal, propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::String(v) => {
                v.borrow_mut().activate(
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
                v.borrow_mut().deactivate(
                    item.as_bool().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::U8(v) => {
                v.borrow_mut().deactivate(
                    item.as_u8().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::U16(v) => {
                v.borrow_mut().deactivate(
                    item.as_u16().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::U32(v) => {
                v.borrow_mut().deactivate(
                    item.as_u32().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::U64(v) => {
                v.borrow_mut().deactivate(
                    item.as_u64().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::U128(v) => {
                v.borrow_mut().deactivate(
                    item.as_u128().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::USize(v) => {
                v.borrow_mut().deactivate(
                    item.as_u_size().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::I8(v) => {
                v.borrow_mut().deactivate(
                    item.as_i8().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::I16(v) => {
                v.borrow_mut().deactivate(
                    item.as_i16().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::I32(v) => {
                v.borrow_mut().deactivate(
                    item.as_i32().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::I64(v) => {
                v.borrow_mut().deactivate(
                    item.as_i64().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::I128(v) => {
                v.borrow_mut().deactivate(
                    item.as_i128().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::ISize(v) => {
                v.borrow_mut().deactivate(
                    item.as_i_size().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::F32(v) => {
                v.borrow_mut().deactivate(
                    item.as_f32().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::F64(v) => {
                v.borrow_mut().deactivate(
                    item.as_f64().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::RcStr(v) => {
                v.borrow_mut().deactivate(
                    item.as_rc_str().unwrap(), propagate_horizontal, propagate_vertical
                )
            },
            SensorConatiner::String(v) => {
                v.borrow_mut().deactivate(
                    item.as_string().unwrap(), propagate_horizontal, propagate_vertical
                )
            }
        }
    }

    fn deactivate_sensor(&mut self) {
        match self {
            SensorConatiner::Bool(v) => v.borrow_mut().deactivate_sensor(),
            SensorConatiner::U8(v) => v.borrow_mut().deactivate_sensor(),
            SensorConatiner::U16(v) => v.borrow_mut().deactivate_sensor(),
            SensorConatiner::U32(v) => v.borrow_mut().deactivate_sensor(),
            SensorConatiner::U64(v) => v.borrow_mut().deactivate_sensor(),
            SensorConatiner::U128(v) => v.borrow_mut().deactivate_sensor(),
            SensorConatiner::USize(v) => v.borrow_mut().deactivate_sensor(),
            SensorConatiner::I8(v) => v.borrow_mut().deactivate_sensor(),
            SensorConatiner::I16(v) => v.borrow_mut().deactivate_sensor(),
            SensorConatiner::I32(v) => v.borrow_mut().deactivate_sensor(),
            SensorConatiner::I64(v) => v.borrow_mut().deactivate_sensor(),
            SensorConatiner::I128(v) => v.borrow_mut().deactivate_sensor(),
            SensorConatiner::ISize(v) => v.borrow_mut().deactivate_sensor(),
            SensorConatiner::F32(v) => v.borrow_mut().deactivate_sensor(),
            SensorConatiner::F64(v) => v.borrow_mut().deactivate_sensor(),
            SensorConatiner::RcStr(v) => v.borrow_mut().deactivate_sensor(),
            SensorConatiner::String(v) => v.borrow_mut().deactivate_sensor()
        }
    }
}