use std::{
    rc::{ Rc, Weak },
    cell::RefCell
};

use bionet_common::{
    neuron::{ Neuron, NeuronID, NeuronConnect }, 
    connection::{
        Connection,
        ConnectionKind,
        defining_connection::DefiningConnection
    }
};

pub struct SimpleNeuron<Element: Neuron> {
    pub id: NeuronID,
    pub activation: f32,
    pub(crate) self_ptr: Weak<RefCell<SimpleNeuron<Element>>>,
    pub(crate) definitions: Vec<Rc<RefCell<DefiningConnection<Self, dyn Neuron>>>>,
}

impl<Element: Neuron> SimpleNeuron<Element> {
    pub fn new(id: &Rc<str>, parent_id: &Rc<str>) -> Rc<RefCell<SimpleNeuron<Element>>> {
        let neuron_ptr = Rc::new(
            RefCell::new(
                SimpleNeuron {
                    id: NeuronID { id: id.clone(), parent_id: parent_id.clone() },
                    activation: 0.0f32,
                    self_ptr: Weak::new(), 
                    definitions: Vec::new()
                }
            )
        );

        neuron_ptr.borrow_mut().self_ptr = Rc::downgrade(&neuron_ptr);
        neuron_ptr
    }
}

impl<Element: Neuron> Neuron for SimpleNeuron<Element> {
    fn id(&self) -> NeuronID { self.id.clone() }

    fn activation(&self) -> f32 { self.activation }

    fn is_sensor(&self) -> bool { false }

    fn counter(&self) -> usize { 1usize }

    fn activate(
        &mut self, signal: f32, propagate_horizontal: bool, propagate_vertical: bool
    ) -> Vec<Rc<RefCell<dyn Neuron>>> {
        self.activation += signal;

        let mut activated_neurons = Vec::new();

        if propagate_vertical {
            for e in &self.definitions {
                let to = e.borrow().to().clone();
                let is_sensor = to.borrow().is_sensor();
                to.borrow_mut().activate(self.activation, propagate_horizontal, !is_sensor);
                activated_neurons.push(to);
            }
        }

        activated_neurons
    }

    fn explain(&mut self) -> Vec<Rc<RefCell<dyn Neuron>>> { Vec::new() }

    fn deactivate(&mut self, propagate_horizontal: bool, propagate_vertical: bool) {
        self.activation = 0.0f32;

        if propagate_vertical {
            for e in &self.definitions {
                let to = e.borrow().to().clone();
                let is_sensor = to.borrow().is_sensor();
                to.borrow_mut().deactivate(propagate_horizontal, !is_sensor);
            }
        }
    }
}

impl<Element: Neuron + 'static> NeuronConnect for SimpleNeuron<Element> {
    fn connect(
        &mut self, to: Rc<RefCell<dyn Neuron>>, kind: ConnectionKind
    ) -> Result<Rc<RefCell<dyn Connection<From = Self, To = dyn Neuron>>>, String> {
        match kind {
            ConnectionKind::Defining => {
                let connection = Rc::new(RefCell::new(DefiningConnection::new(
                    self.self_ptr.upgrade().unwrap(), 
                    to
                )));

                self.definitions.push(connection.clone());

                Ok(connection)
            },
            _ => Err("only explanatory connection can be created fo asa-graphs element".to_string())
        }
    }
}