pub mod fsm;
use std::collections::VecDeque;

pub use fsm::*;

pub mod select;
pub use select::*;

use tuix::*;

pub enum Tool {
    Select,
}

pub trait Store {
    type A: Action<Data = Self>;

    fn process_actions(&mut self, actions: &mut VecDeque<Self::A>) {
        while let Some(mut action) = actions.pop_front() {
            action.apply(self);
        }
    }
}

#[derive(Default)]
pub struct EditorData {
    selected_nodes: Vec<Entity>,
}

// TODO - Move to derive macro
impl Store for EditorData {
    type A = EditorAction;
}

pub enum EditorAction {
    // Add a selected node to the selected nodes list
    AddSelection {
        added_node: Entity,
    },
    // Remove a selected node from the selected nodes list
    RemoveSelection {
        removed_node: Entity,
    },

    // Cleares all selected nodes
    ClearSelection {
        removed_nodes: Vec<Entity>,
    }

}

pub trait Action {

    type Data;

    fn apply(&mut self, data: &mut Self::Data);

    fn undo(&mut self, data: &mut Self::Data);
}

impl Action for EditorAction {
    type Data = EditorData;

    fn apply(&mut self, data: &mut Self::Data) {
        use EditorAction::*;
        match self {
            AddSelection { added_node } => {
                data.selected_nodes.push(*added_node);
            }

            RemoveSelection { removed_node } => {
                if let Some(index) = data.selected_nodes.iter().position(|&x| x == *removed_node) {
                    data.selected_nodes.remove(index);
                }
            }

            ClearSelection { removed_nodes } => {
                *removed_nodes = data.selected_nodes.clone();
                data.selected_nodes.clear();
            }
        }
    }

    fn undo(&mut self, data: &mut Self::Data) {
        use EditorAction::*;
        match self {
            AddSelection { added_node } => {
                if let Some(index) = data.selected_nodes.iter().position(|&x| x == *added_node) {
                    data.selected_nodes.remove(index);
                }
            }

            RemoveSelection { removed_node } => {
                data.selected_nodes.push(*removed_node);
            }

            ClearSelection { removed_nodes } => {
                data.selected_nodes = removed_nodes.clone();
            }

        }
    }
}