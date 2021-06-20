use std::collections::VecDeque;



// Trait describing a finite state machine used for tools
pub trait Fsm {
    type Message;
    type Data;
    type Action;

    fn transition(self, message: Self::Message, data: Self::Data, actions: &mut VecDeque<Self::Action>) -> Self;
}