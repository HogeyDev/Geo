#[derive(Clone, Debug)]
pub struct Node {
    pub connections: Vec<f64>, // list of weights; length of vector should be equal to size of previous layer
    pub value: f64,
}

impl Node {
    pub fn new(num_inputs: usize) -> Self {
        let mut connections = Vec::new();
        if num_inputs > 0 {
            connections.resize(num_inputs, 0.0);
        }
        Self {
            connections,
            value: 0.0,
        }
    }
}
