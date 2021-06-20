pub mod tool;
pub use tool::*;

pub mod node_editor;
pub use node_editor::*;


use tuix::*;


// Stores front-end state of the editor such as selected nodes
pub struct NodeEditorState {
    selected_tool: Tool,
    selected_nodes: Vec<Entity>,
}