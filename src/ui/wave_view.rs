

use tuix::*;

pub struct WaveView {

}

impl WaveView {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl Widget for WaveView {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity
    }
}