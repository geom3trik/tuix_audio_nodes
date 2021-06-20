// Node Select Tool

use std::collections::VecDeque;

use crate::editor::{EditorMessage, tool::fsm::*};
use tuix::*;

use super::EditorAction;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectToolMessage {
    // User clicked on a node
    PressedNode(Entity),

    ReleasedNode(Entity),
    // User clicked on the canvas
    PressedCanvas,
    // User dragged the mouse cursor
    MouseMoved(f32,f32),

    EnterMultiSelect,
    LeaveMultiSelect,
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectToolState {
    // No nodes have been selected
    SingleSelect,

    MultiSelect,
    // At least one node has been selected
    Selected,
    // Box selection
    Dragging,
    // Selected nodes are being moved
    Moving,
}

pub struct SelectToolData {

}

impl Fsm for SelectToolState {
    type Message = EditorMessage;
    type Data = SelectToolData;
    type Action = EditorAction;

    fn transition(self, message: Self::Message, data: Self::Data, actions: &mut VecDeque<Self::Action>) -> Self {

        use SelectToolState::*;
        use EditorMessage::*;

        match (self, message) {
            // Selecting a node in single select mode will clear the list of selected nodes and add the new one
            (SingleSelect, PressedNode(entity)) => {
                actions.push_back(EditorAction::ClearSelection { removed_nodes: Vec::new() });
                actions.push_back(EditorAction::AddSelection { added_node: entity });      
                
                Selected
            }

            // Selecting the canvas in single select mode or selected mode will clear the list of selected nodes
            (SingleSelect | Selected, PressedCanvas) => {
                actions.push_back(EditorAction::ClearSelection { removed_nodes: Vec::new() });

                SingleSelect
            }



            _=> self,
        }
    }
}
