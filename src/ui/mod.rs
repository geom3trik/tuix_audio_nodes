pub mod node_view;
pub use node_view::*;

pub mod node_widget;
pub use node_widget::*;

pub mod socket_widget;
pub use socket_widget::*;

use tuix::*;

#[derive(Debug, Clone, PartialEq)]
pub enum NodeEvent {
    TrySnap(Entity, Entity),
    ConnectSockets(Entity),
    ConnectInput,
    ConnectOutput,
    //Disconnect(Entity),
    Snap(Entity, Entity),
    Connecting,
    Disconnect,
}