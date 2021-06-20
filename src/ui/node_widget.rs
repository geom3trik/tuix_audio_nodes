use femtovg::{
    renderer::OpenGl, Align, Baseline, Canvas, FillRule, FontId, ImageFlags, ImageId, LineCap,
    LineJoin, Paint, Path, Renderer, Solidity,
};
use tuix::*;

use super::{NodeEvent, socket_widget::*};

pub struct NodeWidget {
    selected: bool,
    moving: bool,

    mouse_down_x: f32,
    mouse_down_y: f32,

    translate_x: f32,
    translate_y: f32,

    prev_translate_x: f32,
    prev_translate_y: f32,

    name: String,
}

impl NodeWidget {
    pub fn new(name: &str) -> Self {
        Self {
            selected: false,
            moving: false,

            mouse_down_x: 0.0,
            mouse_down_y: 0.0,

            prev_translate_x: 0.0,
            prev_translate_y: 0.0,
            translate_x: 0.0,
            translate_y: 0.0,

            name: name.to_string(),
        }
    }
}

impl Widget for NodeWidget {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        Label::new(&self.name).build(state, entity, |builder| {
            builder
                .set_height(Pixels(30.0))
                .set_child_space(Stretch(1.0))
                //.set_background_color(Color::rgb(50, 50, 200))
                //.set_border_radius(Pixels(3.0))
                //.set_border_radius_top_left(Pixels(3.0))
                //.set_border_radius_top_right(Pixels(3.0))
                .set_hoverability(false)
                .class("node_label")
        });

        Element::new().build(state, entity, |builder| {
            builder.set_height(Pixels(10.0)).set_hoverability(false)
        });

        let conatiner = Element::new().build(state, entity, |builder| {
            builder.set_height(Auto).set_hoverability(false)
        });

        // self.add_input_socket(state, entity);
        // self.add_output_socket(state, entity);

        Element::new().build(state, entity, |builder| {
            builder.set_height(Pixels(10.0)).set_hoverability(false)
        });

        entity
            .set_width(state, Pixels(200.0))
            .set_height(state, Auto)
            .set_left(state, Pixels(100.0))
            .set_top(state, Pixels(100.0))
            //.set_background_color(state, Color::rgb(50,50,50))
            //.set_border_radius(state, Pixels(3.0))
            //.set_border_width(state, Pixels(1.0))
            //.set_border_color(state, Color::rgb(100, 100, 100))
            .set_position_type(state, PositionType::SelfDirected)
            .class(state, "node");

        conatiner
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseDown(button) => {
                    if event.target == entity {
                        if *button == MouseButton::Left {

                            entity.emit(state, entity, Event::new(NodeEvent::SelectNode(entity)));

                            self.moving = true;
                            state.capture(entity);
                            self.prev_translate_x = self.translate_x;
                            self.prev_translate_y = self.translate_y;
                            let mut transform = state.data.get_transform(entity);
                            transform.inverse();
                            let (mx, my) = transform.transform_point(
                                state.mouse.left.pos_down.0,
                                state.mouse.left.pos_down.1,
                            );

                            let parent = entity.get_parent(state).unwrap();
                            self.mouse_down_x =
                                mx - state.data.get_posx(entity) + state.data.get_posx(parent);
                            self.mouse_down_y =
                                my - state.data.get_posy(entity) + state.data.get_posy(parent);
                        }
                    }
                }

                WindowEvent::MouseUp(button) => {
                    if event.target == entity {
                        if *button == MouseButton::Left {
                            self.moving = false;
                            state.release(entity);
                        }
                    }
                }

                WindowEvent::MouseMove(x, y) => {
                    if event.target == entity {
                        if self.moving {
                            let parent = entity.get_parent(state).unwrap();
                            self.translate_x =
                                self.prev_translate_x + (*x - state.mouse.left.pos_down.0);
                            self.translate_y =
                                self.prev_translate_y + (*y - state.mouse.left.pos_down.1);

                            let mut transform = state.data.get_transform(entity);
                            transform.inverse();

                            let (tx, ty) = transform.transform_point(*x, *y);

                            entity
                                //.set_translate(state, (tx, ty));
                                .set_left(state, Pixels(tx - self.mouse_down_x))
                                .set_top(state, Pixels(ty - self.mouse_down_y));
                            state.insert_event(
                                Event::new(WindowEvent::Redraw).target(Entity::root()),
                            );
                        }
                    }
                }

                _ => {}
            }
        }
    }
}
