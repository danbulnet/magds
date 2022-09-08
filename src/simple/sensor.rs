use std::{
    rc::Rc,
    cell::RefCell,
    mem
};

use asa_graphs::neural::graph::ASAGraph;

pub enum ASAGraphSensor {
    Bool(ASAGraph<bool, 25>),
    U8(ASAGraph<u8, 25>),
    U16(ASAGraph<u16, 25>),
    U32(ASAGraph<u32, 25>),
    U64(ASAGraph<u64, 25>),
    I8(ASAGraph<i8, 25>),
    I16(ASAGraph<i16, 25>),
    I32(ASAGraph<i32, 25>),
    I64(ASAGraph<i64, 25>),
    F32(ASAGraph<f32, 25>),
    F64(ASAGraph<f64, 25>),
    RcStr(ASAGraph<Rc<str>, 25>),
    String(ASAGraph<String, 25>),
}

impl From<ASAGraph<bool, 25>> for ASAGraphSensor {
    fn from(sensor: ASAGraph<bool, 25>) -> ASAGraphSensor {
        ASAGraphSensor::Bool(sensor)
    }
}

impl From<ASAGraph<u8, 25>> for ASAGraphSensor {
    fn from(sensor: ASAGraph<u8, 25>) -> ASAGraphSensor {
        ASAGraphSensor::U8(sensor)
    }
}

impl From<ASAGraph<u16, 25>> for ASAGraphSensor {
    fn from(sensor: ASAGraph<u16, 25>) -> ASAGraphSensor {
        ASAGraphSensor::U16(sensor)
    }
}

impl From<ASAGraph<u32, 25>> for ASAGraphSensor {
    fn from(sensor: ASAGraph<u32, 25>) -> ASAGraphSensor {
        ASAGraphSensor::U32(sensor)
    }
}

impl From<ASAGraph<u64, 25>> for ASAGraphSensor {
    fn from(sensor: ASAGraph<u64, 25>) -> ASAGraphSensor {
        ASAGraphSensor::U64(sensor)
    }
}

impl From<ASAGraph<i8, 25>> for ASAGraphSensor {
    fn from(sensor: ASAGraph<i8, 25>) -> ASAGraphSensor {
        ASAGraphSensor::I8(sensor)
    }
}

impl From<ASAGraph<i16, 25>> for ASAGraphSensor {
    fn from(sensor: ASAGraph<i16, 25>) -> ASAGraphSensor {
        ASAGraphSensor::I16(sensor)
    }
}

impl From<ASAGraph<i32, 25>> for ASAGraphSensor {
    fn from(sensor: ASAGraph<i32, 25>) -> ASAGraphSensor {
        ASAGraphSensor::I32(sensor)
    }
}

impl From<ASAGraph<i64, 25>> for ASAGraphSensor {
    fn from(sensor: ASAGraph<i64, 25>) -> ASAGraphSensor {
        ASAGraphSensor::I64(sensor)
    }
}

impl From<ASAGraph<f32, 25>> for ASAGraphSensor {
    fn from(sensor: ASAGraph<f32, 25>) -> ASAGraphSensor {
        ASAGraphSensor::F32(sensor)
    }
}

impl From<ASAGraph<f64, 25>> for ASAGraphSensor {
    fn from(sensor: ASAGraph<f64, 25>) -> ASAGraphSensor {
        ASAGraphSensor::F64(sensor)
    }
}

impl From<ASAGraph<Rc<str>, 25>> for ASAGraphSensor {
    fn from(sensor: ASAGraph<Rc<str>, 25>) -> ASAGraphSensor {
        ASAGraphSensor::RcStr(sensor)
    }
}

impl From<ASAGraph<String, 25>> for ASAGraphSensor {
    fn from(sensor: ASAGraph<String, 25>) -> ASAGraphSensor {
        ASAGraphSensor::String(sensor)
    }
}
