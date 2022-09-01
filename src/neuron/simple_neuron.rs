use std::{
    rc::{ Rc, Weak },
    cell::RefCell,
    collections::HashMap
};

use bionet_common::{
    neuron::{ Neuron, NeuronID, NeuronConnect }, 
    connection::{
        Connection,
        ConnectionKind,
        ConnectionID,
        defining_connection::DefiningConnection
    }
};

pub struct SimpleNeuron {
    pub id: NeuronID,
    pub activation: f32,
    pub(crate) self_ptr: Weak<RefCell<SimpleNeuron>>,
    pub(crate) definitions_from_self: 
        HashMap<ConnectionID, Rc<RefCell<dyn Connection<From = Self, To = dyn Neuron>>>>,
    pub(crate) definitions_to_self: 
        HashMap<ConnectionID, Rc<RefCell<dyn Connection<From = dyn Neuron, To = Self>>>>
}

impl SimpleNeuron {
    pub fn new(id: &Rc<str>, parent_id: &Rc<str>) -> Rc<RefCell<SimpleNeuron>> {
        let neuron_ptr = Rc::new(
            RefCell::new(
                SimpleNeuron {
                    id: NeuronID { id: id.clone(), parent_id: parent_id.clone() },
                    activation: 0.0f32,
                    self_ptr: Weak::new(), 
                    definitions_from_self: HashMap::new(),
                    definitions_to_self: HashMap::new()
                }
            )
        );

        neuron_ptr.borrow_mut().self_ptr = Rc::downgrade(&neuron_ptr);
        neuron_ptr
    }

    pub(crate) fn defined_neurons(&self) -> HashMap<NeuronID, Rc<RefCell<dyn Neuron>>> {
        let mut neurons = HashMap::new();
        for (_id, definition) in &self.definitions_from_self {
            let neuron = definition.borrow().to();
            if !neuron.borrow().is_sensor() {
                neurons.insert(neuron.borrow().id(), neuron.clone());
            }
        }
        neurons
    }

    pub(crate) fn defining_sensors(&self) -> HashMap<NeuronID, Rc<RefCell<dyn Neuron>>> {
        let mut sensors = HashMap::new();
        for (_id, definition) in &self.definitions_to_self {
            let neuron = definition.borrow().from();
            if neuron.borrow().is_sensor() {
                sensors.insert(neuron.borrow().id(), neuron.clone());
            }
        }
        sensors
    }

    pub(crate) fn defining_neurons(&self) -> HashMap<NeuronID, Rc<RefCell<dyn Neuron>>> {
        let mut neurons = HashMap::new();
        for (_id, definition) in &self.definitions_to_self {
            let neuron = definition.borrow().from();
            if !neuron.borrow().is_sensor() {
                neurons.insert(neuron.borrow().id(), neuron.clone());
            }
        }
        neurons
    }
}

impl Neuron for SimpleNeuron {
    fn id(&self) -> NeuronID { self.id.clone() }

    fn activation(&self) -> f32 { self.activation }

    fn is_sensor(&self) -> bool { false }

    fn counter(&self) -> usize { 1usize }

    fn activate(
        &mut self, signal: f32, propagate_horizontal: bool, propagate_vertical: bool
    ) -> HashMap<NeuronID, Rc<RefCell<dyn Neuron>>> {
        self.activation += signal;

        let mut neurons = self.defined_neurons();
        if propagate_vertical {
            for (_id, neuron) in &neurons.clone() {
                if !neuron.borrow().is_sensor() {
                    neurons.extend(
                        neuron.borrow_mut().activate(
                            self.activation, propagate_horizontal, propagate_vertical
                        )
                    );
                }
            }
        }

        neurons
    }

    fn explain(&mut self) -> HashMap<NeuronID, Rc<RefCell<dyn Neuron>>> {
        self.defining_sensors()
    }

    fn deactivate(&mut self, propagate_horizontal: bool, propagate_vertical: bool) {
        self.activation = 0.0f32;

        if propagate_vertical {
            for (_id, neuron) in &self.defined_neurons() {
                neuron.borrow_mut().deactivate(propagate_horizontal, propagate_vertical);
            }
        }
    }
}

impl NeuronConnect for SimpleNeuron {
    fn connect_to(
        &mut self, to: Rc<RefCell<dyn Neuron>>, kind: ConnectionKind
    ) -> Result<Rc<RefCell<dyn Connection<From = Self, To = dyn Neuron>>>, String> {
        match kind {
            ConnectionKind::Defining => {
                let connection = Rc::new(RefCell::new(DefiningConnection::new(
                    self.self_ptr.upgrade().unwrap(), 
                    to.clone()
                )));
                let connection_id = ConnectionID { from: self.id(), to: to.borrow().id() };
                self.definitions_from_self.insert(connection_id, connection.clone());
                Ok(connection)
            },
            _ => {
                let msg = "only defining connection to SimpleNeuron can be created";
                log::error!("{}", msg);
                Err(msg.to_string())
            }
        }
    }

    fn connect_to_connection(
        &mut self, to_connection: Rc<RefCell<dyn Connection<From = Self, To = dyn Neuron>>>
    ) -> Result<Rc<RefCell<dyn Connection<From = Self, To = dyn Neuron>>>, String> {
        match to_connection.borrow().kind() {
            ConnectionKind::Defining => {
                let to_neuron = to_connection.borrow().to().clone();
                let connection_id = ConnectionID { from: self.id(), to: to_neuron.borrow().id() };
                self.definitions_from_self.insert(connection_id, to_connection.clone());
                Ok(to_connection.clone())
            },
            _ => {
                let msg = "only defining connection to SimpleNeuron can be created";
                log::error!("{}", msg);
                Err(msg.to_string())
            }
        }
    }

    fn connect_from(
        &mut self, from: Rc<RefCell<dyn Neuron>>, kind: ConnectionKind
    ) -> Result<Rc<RefCell<dyn Connection<From = dyn Neuron, To = Self>>>, String> {
        match kind {
            ConnectionKind::Defining => {
                let connection = Rc::new(RefCell::new(DefiningConnection::new(
                    from.clone(),
                    self.self_ptr.upgrade().unwrap()
                )));
                let connection_id = ConnectionID { from: from.borrow().id(), to: self.id() };
                self.definitions_to_self.insert(connection_id, connection.clone());
                Ok(connection)
            },
            _ => {
                let msg = "only defining connection to SimpleNeuron can be created";
                log::error!("{}", msg);
                Err(msg.to_string())
            }
        }
    }

    fn connect_from_connection(
        &mut self, from_connection: Rc<RefCell<dyn Connection<From = dyn Neuron, To = Self>>>
    ) -> Result<Rc<RefCell<dyn Connection<From = dyn Neuron, To = Self>>>, String> {
        match from_connection.borrow().kind() {
            ConnectionKind::Defining => {
                let from_neuron = from_connection.borrow().from().clone();
                let connection_id = ConnectionID { from: from_neuron.borrow().id(), to: self.id() };
                self.definitions_to_self.insert(connection_id, from_connection.clone());
                Ok(from_connection.clone())
            },
            _ => {
                let msg = "only defining connection to SimpleNeuron can be created";
                log::error!("{}", msg);
                Err(msg.to_string())
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use std::{
        rc::Rc,
        cell::RefCell
    };

    use bionet_common::{
        neuron::{ Neuron, NeuronConnect },
        connection::ConnectionKind,
        data::DataCategory
    };

    use asa_graphs::neural::{
        element::Element,
        graph::ASAGraph
    };

    use super::SimpleNeuron;
}