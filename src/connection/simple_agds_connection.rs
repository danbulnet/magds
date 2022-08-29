use std::{
    rc::Rc,
    cell::RefCell
};

use bionet_common::{
    neuron::Neuron,
    connection::{ Connection, ConnectionKind, ConnectionID }
};

/// Allows to use flat object structure, without relationships between objects.
/// This connection is enough for AGDS but not for MAGDS.
pub struct SimpleAGDSConnection<From: Neuron, To: Neuron> {
    from: Rc<RefCell<From>>,
    to: Rc<RefCell<To>>
}

impl<From: Neuron, To: Neuron> SimpleAGDSConnection<From, To> {
    pub fn new(from: Rc<RefCell<From>>, to: Rc<RefCell<To>>) -> SimpleAGDSConnection<From, To> {
        SimpleAGDSConnection { from, to }
    }
}

impl<From: Neuron, To: Neuron> Connection for SimpleAGDSConnection<From, To> {
    type From = From;
    type To = To;

    fn id(&self) -> ConnectionID { 
        ConnectionID { from: self.from.borrow().id(), to: self.to.borrow().id() }
    }

    fn from(&self) -> Rc<RefCell<From>> { self.from.clone() }
    
    fn to(&self) -> Rc<RefCell<To>> { self.to.clone() }

    fn kind(&self) -> ConnectionKind { 
        let is_from_sensor = self.from.borrow().is_sensor();
        let is_to_sensor = self.from.borrow().is_sensor();
        match is_from_sensor {
            true => match is_to_sensor {
                true => ConnectionKind::Dummy,
                false => ConnectionKind::Definition
            },
            false => {
                match is_to_sensor {
                    true => ConnectionKind::Explanation,
                    false => ConnectionKind::Dummy
                }
            }
        }
    }
    
    fn weight(&self) -> f32 {
        match self.kind() {
            ConnectionKind::Definition => 1.0f32 / self.from.borrow().counter() as f32,
            ConnectionKind::Explanation => 1.0f32,
            _ => 0.0f32
        }
    }
}