use crate::activation::Activation;
use crate::neuron::Neuron;

#[derive(Debug, Clone)]
pub struct Layer {
    pub id: u32,
    pub neurons: Vec<Neuron>,
    pub activation: Activation,
}

impl Layer {
    pub fn new(id: u32, activation: Activation) -> Self {
        Layer {
            id,
            neurons: Vec::new(),
            activation,
        }
    }

    pub fn forward(&self, input: &[f32]) -> Vec<f32> {
        self.neurons.iter().map(|n| n.activate(input)).collect()
    }

    pub fn neuron_count(&self) -> usize {
        self.neurons.len()
    }
}
