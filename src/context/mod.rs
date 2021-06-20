pub mod node;
pub use node::*;

pub mod buffer;
pub use buffer::*;

pub mod graph;
pub use graph::*;

pub struct Context {
    buffers: Vec<Vec<f32>>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            buffers: Vec::new(),
        }
    }
}
