use super::layer::{Layer, LayerType};

#[derive(Debug)]
pub struct NeuralNetwork {
    pub layers: Vec<Layer>,
}

impl NeuralNetwork {
    pub fn new(sizes: Vec<usize>) -> Self {
        let mut layers = Vec::new();
        for (i, layer) in sizes.iter().enumerate() {
            layers.push(Layer::from(layer.to_owned(), if i == 0 { 0 } else { sizes[i - 1] }, match i {
                0 => LayerType::Input,
                _ => if i >= sizes.len() - 1 { LayerType::Output } else { LayerType::Hidden },
            }));
        }
        Self {
            layers
        }
    }
    pub fn calculate(&mut self) {
        let mut previous_layer = self.layers[0].clone();
        for layer in &mut self.layers.iter_mut().skip(0) {
            layer.update_from(previous_layer);
            previous_layer = layer.clone();
            println!("{layer:#?}");
        }
    }
}
