use super::node::Node;

#[derive(Clone, Debug)]
pub enum LayerType {
    Input,
    Hidden,
    Output,
}

#[derive(Clone, Debug)]
pub struct Layer {
    pub nodes: Vec<Node>,
    pub node_type: LayerType,
}

impl Layer {
    pub fn from(size: usize, previous_layer_size: usize, layer_type: LayerType) -> Self {
        let mut nodes = Vec::new();
        nodes.resize(size, Node::new(previous_layer_size));
        Self {
            nodes,
            node_type: layer_type,
        }
    }
    pub fn update_from(&mut self, previous: Layer) {
        for node in &mut self.nodes {
            node.value = 0.0;
            for (i, conn) in node.connections.iter().enumerate() {
                node.value += previous.nodes[i].value * conn;
            }
        }
    }
}
