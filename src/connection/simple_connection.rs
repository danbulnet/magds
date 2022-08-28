use std::{
    rc::Rc,
    cell::RefCell
};

use bionet_common::{
    neuron::Neuron,
    connection::Connection
};

pub struct SimpleConnection<From: Neuron, To: Neuron> {
    from: Rc<RefCell<From>>,
    to: Rc<RefCell<To>>
}

impl<From: Neuron, To: Neuron> SimpleConnection<From, To> {
    pub fn new(from: Rc<RefCell<From>>, to: Rc<RefCell<To>>) -> SimpleConnection<From, To> {
        SimpleConnection { from, to }
    }
}

impl<From: Neuron, To: Neuron> Connection for SimpleConnection<From, To> {
    type From = From;
    type To = To;

    fn from(&self) -> Rc<RefCell<From>> { self.from.clone() }
    
    fn to(&self) -> Rc<RefCell<To>> { self.to.clone() }
}