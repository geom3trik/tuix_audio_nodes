

pub enum NodeType {

}

pub enum SocketType {
    Input,
    Output,
    Both,
}

pub struct SocketData {
    pub name: String,
    pub socket_type: SocketType,
}

pub struct Program {
    
    sockets: Vec<f32>,
    data: Vec<f32>,

}