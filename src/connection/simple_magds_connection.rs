use std::{
    rc::Rc,
    cell::RefCell
};

use bionet_common::{
    neuron::Neuron,
    connection::{ Connection, ConnectionKind, ConnectionID }
};

/// Allows to use hierarchical object structure.
/// This connection is enough for MAGDS.
/// It shouldn't be used for complex relations like inhibition, similarity or consequence.
pub struct SimpleMAGDSConnection<From: Neuron, To: Neuron> {
    from: Rc<RefCell<From>>,
    to: Rc<RefCell<To>>,
    kind: ConnectionKind
}

impl<From: Neuron, To: Neuron> SimpleMAGDSConnection<From, To> {
    pub fn new(
        from: Rc<RefCell<From>>, to: Rc<RefCell<To>>, kind: ConnectionKind
    ) -> SimpleMAGDSConnection<From, To> { 
        match kind {
            ConnectionKind::Definition | ConnectionKind::Explanation => SimpleMAGDSConnection { 
                from, to, kind 
            },
            _ => panic!("SimpleMAGDSConnection allows to use Definition and Explanation kinds only") 
        }
    }
}

impl<From: Neuron, To: Neuron> Connection for SimpleMAGDSConnection<From, To> {
    type From = From;
    type To = To;

    fn id(&self) -> ConnectionID { 
        ConnectionID { from: self.from.borrow().id(), to: self.to.borrow().id() }
    }

    fn from(&self) -> Rc<RefCell<From>> { self.from.clone() }
    
    fn to(&self) -> Rc<RefCell<To>> { self.to.clone() }

    fn kind(&self) -> ConnectionKind { self.kind }
    
    fn weight(&self) -> f32 {
        match self.kind() {
            ConnectionKind::Definition => 1.0f32 / self.from.borrow().counter() as f32,
            ConnectionKind::Explanation => 1.0f32,
            _ => 0.0f32
        }
    }
}