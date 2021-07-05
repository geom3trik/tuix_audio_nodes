use tuix::*;
use femtovg::{
    Canvas, renderer::OpenGl, Align, Baseline, FillRule, FontId, ImageFlags, ImageId, LineCap, LineJoin,
    Paint, Path, Renderer, Solidity,
};

use super::NodeEvent;


// Widget for the connecting wire between an output and input socket
pub struct ConnectionWidget {
    output_socket: Entity,
    input_socket: Entity,
}

impl ConnectionWidget {
    pub fn new(input_socket: Entity) -> Self {
        Self {
            input_socket,
            output_socket: Entity::null(),
        }
    }
}

impl Widget for ConnectionWidget {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_z_order(state, -1)
    }

    fn on_draw(&mut self, state: &mut State, entity: Entity, canvas: &mut Canvas<OpenGl>) {
        if self.input_socket != Entity::null() && self.output_socket != Entity::null() {

            let transform = state.data.get_transform(entity);
            
            canvas.save();
            canvas.set_transform(transform[0], transform[1], transform[2], transform[3], transform[4], transform[5]);

            let input_bounds = state.data.get_bounds(self.input_socket);
            let output_bounds = state.data.get_bounds(self.output_socket);

            let mut path = Path::new();
            path.move_to(output_bounds.x + output_bounds.w / 2.0, output_bounds.y + output_bounds.h / 2.0);
            let mid_x = ((input_bounds.x + input_bounds.w / 2.0) - (output_bounds.x + output_bounds.w / 2.0)) / 2.0;
            path.bezier_to((input_bounds.x + input_bounds.w / 2.0) - mid_x, output_bounds.y + output_bounds.h / 2.0, (output_bounds.x + output_bounds.w / 2.0) + mid_x, input_bounds.y + input_bounds.h / 2.0, input_bounds.x + input_bounds.w / 2.0, input_bounds.y + input_bounds.h / 2.0);
            let mut paint = Paint::color(femtovg::Color::rgb(200, 200, 200));
            paint.set_line_width(2.0);
            canvas.stroke_path(&mut path, paint);
            canvas.restore();
        }
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(node_event) = event.message.downcast() {
            match node_event {
                
                NodeEvent::ConnectSockets(output) => {
                    self.output_socket = *output;
                }

                NodeEvent::Disconnect => {
                    self.output_socket = Entity::null();
                }

                _=> {}
            }
        }
    }
}

pub struct InputSocket {
    // Flag to determine if the socket is currently being connected
    connecting: bool,
    // The wire between this socket and an output socket
    connection: Entity,
    // Id of any connected output socket
    connected_output: Entity,
    // Id socket hovered, causing the connection to snap to the centre
    snapped_socket: Entity,
    // Flag to determine if the connection is snapping to the hovered socket
    snapping: bool,
}

impl InputSocket {
    pub fn new() -> Self {
        Self {
            connecting: false,
            connection: Entity::null(),
            connected_output: Entity::null(),
            snapping: false,
            snapped_socket: Entity::null(),
        }
    }
}

impl Widget for InputSocket {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        Element::new().build(state, entity, |builder| 
            builder
                .set_width(Pixels(10.0))
                .set_height(Pixels(10.0))
                .set_border_radius(Pixels(5.0))
                //.set_background_color(Color::rgb(200,40,40))
                .set_space(Pixels(5.0))
                .set_hoverability(false)
                .class("socket")
        );

        self.connection = ConnectionWidget::new(entity).build(state, entity, |builder| 
            builder
                .set_hoverability(false)
                
        );
        
        entity
            .set_width(state, Pixels(20.0))
            .set_height(state, Pixels(20.0))
            .class(state, "snap")
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseDown(button) => {
                    if *button == MouseButton::Left {
                        state.capture(entity);
                        self.connecting = true;
                        //state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));
                        entity.emit_to(state, Entity::root(), WindowEvent::Redraw);
                        entity.set_z_order(state, 1);
                    }
                }

                WindowEvent::MouseUp(button) => {
                    if *button == MouseButton::Left {
                        if self.connecting {
                            self.connecting = false;
                            state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));
                            //state.insert_event(Event::new(NodeEvent::ConnectSockets(entity, state.hovered)).direct(state.hovered).origin(entity));
                            //state.insert_event(Event::new(NodeEvent::ConnectInput).direct(state.hovered).origin(entity));
                            entity.emit_to(state, state.hovered, NodeEvent::ConnectInput);
                            entity.set_z_order(state, 0);                            
                        }
                        state.release(entity);

                    }
                }

                WindowEvent::MouseMove(x,y) => {
                    if event.target == entity {
                        state.insert_event(Event::new(NodeEvent::TrySnap(entity, state.hovered)).direct(state.hovered).origin(entity));
                        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));
                        if state.hovered == self.snapped_socket {
                            self.snapping = true;
                        } else {
                            self.snapping = false;
                        }
                    }
                    
                }

                WindowEvent::MouseOut => {
                    
                    if self.connected_output != Entity::null() && self.connecting {
                        state.insert_event(Event::new(NodeEvent::Disconnect).direct(entity).origin(entity));
                        state.insert_event(Event::new(NodeEvent::Disconnect).direct(self.connected_output).origin(entity));
                        self.connecting = false;
                    }
                }

                _=> {}
            }
        }

        if let Some(node_event) = event.message.downcast() {
            match node_event {

                NodeEvent::ConnectOutput => {
                    if event.target == entity && event.origin != entity {
                        self.connected_output = event.origin;
                        state.insert_event(Event::new(NodeEvent::ConnectSockets(event.origin)).direct(self.connection).origin(entity));
                    }
                }

                NodeEvent::Snap(input, output) => {
                    self.snapped_socket = *output;
                }

                NodeEvent::TrySnap(input, output) => {
                    state.insert_event(Event::new(NodeEvent::Snap(*input, *output)).direct(event.origin).origin(entity));
                }

                NodeEvent::Disconnect => {
                    if event.target == entity {
                        state.insert_event(Event::new(NodeEvent::Disconnect).direct(self.connection).origin(entity));
                        self.connected_output = Entity::null();
                    }
                }

                _=> {}
            }
        }
    }

    fn on_draw(&mut self, state: &mut State, entity: Entity, canvas: &mut Canvas<OpenGl>) {
        let bounds = state.data.get_bounds(entity);

        let background_color = state
            .style
            .background_color
            .get(entity)
            .cloned()
            .unwrap_or_default();

        let opacity = state.data.get_opacity(entity);

        let parent = state
            .hierarchy
            .get_parent(entity)
            .expect("Failed to find parent somehow");

        let parent_width = state.data.get_width(parent);
        let parent_height = state.data.get_height(parent);

        let border_radius_top_left = match state
            .style
            .border_radius_top_left
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Units::Pixels(val) => val,
            Units::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let border_radius_top_right = match state
            .style
            .border_radius_top_right
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Units::Pixels(val) => val,
            Units::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let border_radius_bottom_left = match state
            .style
            .border_radius_bottom_left
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Units::Pixels(val) => val,
            Units::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let border_radius_bottom_right = match state
            .style
            .border_radius_bottom_right
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Units::Pixels(val) => val,
            Units::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let border_width = match state
            .style
            .border_width
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Units::Pixels(val) => val,
            Units::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let mut background_color: femtovg::Color = background_color.into();
        background_color.set_alphaf(background_color.a * opacity);

        canvas.save();

        let origin = state.data.get_origin(entity);
        let transform = state.data.get_transform(entity);
        
        canvas.set_transform(transform[0], transform[1], transform[2], transform[3], transform[4], transform[5]);

        canvas.translate(bounds.x, bounds.y);

        let mut path = Path::new();

        if border_radius_bottom_left == (bounds.w - 2.0 * border_width) / 2.0
            && border_radius_bottom_right == (bounds.w - 2.0 * border_width) / 2.0
            && border_radius_top_left == (bounds.w - 2.0 * border_width) / 2.0
            && border_radius_top_right == (bounds.w - 2.0 * border_width) / 2.0
        {
            path.circle(
                0.0 + (border_width / 2.0) + (bounds.w - border_width) / 2.0,
                0.0 + (border_width / 2.0) + (bounds.h - border_width) / 2.0,
                bounds.w / 2.0,
            );
        } else {
            // Draw rounded rect
            path.rounded_rect_varying(
                (border_width / 2.0),
                (border_width / 2.0),
                bounds.w - border_width,
                bounds.h - border_width,
                border_radius_top_left,
                border_radius_top_right,
                border_radius_bottom_right,
                border_radius_bottom_left,
            );
        }

        // Fill with background color
        let mut paint = Paint::color(background_color);

        canvas.fill_path(&mut path, paint);

        canvas.restore();

        // canvas.save();

        // let origin = state.data.get_origin(entity);
        let mut transform = state.data.get_transform(entity);
        
        // canvas.translate(origin.0, origin.1);
        // canvas.set_transform(transform[0], transform[1], transform[2], transform[3], transform[4], transform[5]);
        // canvas.translate(-origin.0, -origin.1);

        // transform start point into local frame
        //transform.inverse();
        let (px, py) = transform.transform_point(bounds.x + bounds.w / 2.0, bounds.y + bounds.h / 2.0);

        if self.connecting {
            let mut path = Path::new();
            path.move_to(px, py);
            if self.snapping {
                let snapped_socket = state.data.get_bounds(state.hovered);
                let (sx, sy) = transform.transform_point(snapped_socket.x + snapped_socket.w / 2.0, snapped_socket.y + snapped_socket.h / 2.0);
                let mid_x = (sx - px) / 2.0;
                path.bezier_to(sx - mid_x, py, px + mid_x, sy, sx, sy);
                //path.line_to(sx, sy);
            } else {
                
                //transform.inverse();
                //let (mx, my) = transform.transform_point(state.mouse.cursorx, state.mouse.cursory);
                let mid_x = (state.mouse.cursorx - px) / 2.0;
                path.bezier_to(state.mouse.cursorx - mid_x, py, px + mid_x, state.mouse.cursory, state.mouse.cursorx, state.mouse.cursory);
                //path.line_to(state.mouse.cursorx, state.mouse.cursory);
            }
            let mut paint = Paint::color(femtovg::Color::rgb(200, 200, 200));
            paint.set_line_width(2.0);
            canvas.stroke_path(&mut path, paint);
        }

        //canvas.restore();
    }
}

pub struct OutputSocket {
    connecting: bool,

    snapped_socket: Entity,

    snapping: bool,
}

impl OutputSocket {
    pub fn new() -> Self {
        Self {
            connecting: false,
            snapping: false,
            snapped_socket: Entity::null(),
        }
    }
}

impl Widget for OutputSocket {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        Element::new().build(state, entity, |builder| 
            builder
                .set_width(Pixels(10.0))
                .set_height(Pixels(10.0))
                .set_border_radius(Pixels(5.0))
                //.set_background_color(Color::rgb(200,40,40))
                .set_space(Pixels(5.0))
                .set_hoverability(false)
                .class("socket")
        );
        
        entity
            .set_width(state, Pixels(20.0))
            .set_height(state, Pixels(20.0))
            .class(state, "snap")
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseDown(button) => {
                    if *button == MouseButton::Left {
                        state.capture(entity);
                        self.connecting = true;
                        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));
                        entity.set_z_order(state, 1);
                    }
                }

                WindowEvent::MouseUp(button) => {
                    if *button == MouseButton::Left {
                        state.release(entity);
                        self.connecting = false;
                        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));
                        state.insert_event(Event::new(NodeEvent::ConnectOutput).direct(state.hovered).origin(entity));
                        
                        entity.set_z_order(state, 0);
                    }
                }

                WindowEvent::MouseMove(x,y) => {
                    if event.target == entity {
                        state.insert_event(Event::new(NodeEvent::TrySnap(entity, state.hovered)).direct(state.hovered).origin(entity));
                        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));
                        if state.hovered == self.snapped_socket {
                            self.snapping = true;
                        } else {
                            self.snapping = false;
                        }
                    }
                    
                } 

                _=> {}
            }
        }

        if let Some(node_event) = event.message.downcast() {
            match node_event {

                NodeEvent::ConnectInput => {
                    if event.target == entity {
                        state.insert_event(Event::new(NodeEvent::ConnectOutput).direct(event.origin).origin(entity));
                    }
                }

                NodeEvent::Snap(input, output) => {
                    self.snapped_socket = *output;
                }

                NodeEvent::TrySnap(input, output) => {
                    state.insert_event(Event::new(NodeEvent::Snap(*input, *output)).direct(event.origin).origin(entity));
                }

                NodeEvent::Disconnect => {
                    if event.target == entity {
                        self.connecting = true;
                        self.snapping = false;
                        state.capture(entity);
                        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));
                    }
                }

                _=> {}
            }
        }
    }

    fn on_draw(&mut self, state: &mut State, entity: Entity, canvas: &mut Canvas<OpenGl>) {
        let bounds = state.data.get_bounds(entity);

        let background_color = state
            .style
            .background_color
            .get(entity)
            .cloned()
            .unwrap_or_default();

        let opacity = state.data.get_opacity(entity);

        let parent = state
            .hierarchy
            .get_parent(entity)
            .expect("Failed to find parent somehow");

        let parent_width = state.data.get_width(parent);
        let parent_height = state.data.get_height(parent);

        let border_radius_top_left = match state
            .style
            .border_radius_top_left
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Units::Pixels(val) => val,
            Units::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let border_radius_top_right = match state
            .style
            .border_radius_top_right
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Units::Pixels(val) => val,
            Units::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let border_radius_bottom_left = match state
            .style
            .border_radius_bottom_left
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Units::Pixels(val) => val,
            Units::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let border_radius_bottom_right = match state
            .style
            .border_radius_bottom_right
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Units::Pixels(val) => val,
            Units::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let border_width = match state
            .style
            .border_width
            .get(entity)
            .cloned()
            .unwrap_or_default()
        {
            Units::Pixels(val) => val,
            Units::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let mut background_color: femtovg::Color = background_color.into();
        background_color.set_alphaf(background_color.a * opacity);

        canvas.save();

        let origin = state.data.get_origin(entity);
        let transform = state.data.get_transform(entity);
        
        canvas.set_transform(transform[0], transform[1], transform[2], transform[3], transform[4], transform[5]);

        canvas.translate(bounds.x, bounds.y);

        let mut path = Path::new();

        if border_radius_bottom_left == (bounds.w - 2.0 * border_width) / 2.0
            && border_radius_bottom_right == (bounds.w - 2.0 * border_width) / 2.0
            && border_radius_top_left == (bounds.w - 2.0 * border_width) / 2.0
            && border_radius_top_right == (bounds.w - 2.0 * border_width) / 2.0
        {
            path.circle(
                0.0 + (border_width / 2.0) + (bounds.w - border_width) / 2.0,
                0.0 + (border_width / 2.0) + (bounds.h - border_width) / 2.0,
                bounds.w / 2.0,
            );
        } else {
            // Draw rounded rect
            path.rounded_rect_varying(
                (border_width / 2.0),
                (border_width / 2.0),
                bounds.w - border_width,
                bounds.h - border_width,
                border_radius_top_left,
                border_radius_top_right,
                border_radius_bottom_right,
                border_radius_bottom_left,
            );
        }

        let paint = Paint::color(background_color);

        canvas.fill_path(&mut path, paint);

        canvas.restore();

        

        if self.connecting {

            let transform = state.data.get_transform(entity);
            let (px, py) = transform.transform_point(bounds.x + bounds.w / 2.0, bounds.y + bounds.h / 2.0);

            let mut path = Path::new();
            path.move_to(px, py);
            if self.snapping {
                let snapped_socket = state.data.get_bounds(state.hovered);
                let (sx, sy) = transform.transform_point(snapped_socket.x + snapped_socket.w / 2.0, snapped_socket.y + snapped_socket.h / 2.0);
                let mid_x = (sx - px) / 2.0;
                path.bezier_to(sx - mid_x, py, px + mid_x, sy, sx, sy);
                //path.line_to(sx, sy);
                let mut paint = Paint::color(femtovg::Color::rgb(200, 200, 200));
                paint.set_line_width(2.0);
                canvas.stroke_path(&mut path, paint);

            } else {
                
                //transform.inverse();
                //let (mx, my) = transform.transform_point(state.mouse.cursorx, state.mouse.cursory);
                let mid_x = (state.mouse.cursorx - px) / 2.0;
                path.bezier_to(state.mouse.cursorx - mid_x, py, px + mid_x, state.mouse.cursory, state.mouse.cursorx, state.mouse.cursory);
                //path.line_to(state.mouse.cursorx, state.mouse.cursory);
                let mut paint = Paint::color(femtovg::Color::rgb(200, 200, 200));
                paint.set_line_width(2.0);
                canvas.stroke_path(&mut path, paint);

            }
            
        }

        //canvas.restore();
    }
}

fn draw_socket(state: &mut State, entity: Entity, canvas: &mut Canvas<OpenGl>) {
    let bounds = state.data.get_bounds(entity);

    let background_color = entity.get_background_color(state);

    let opacity = state.data.get_opacity(entity);

    let parent = state
        .hierarchy
        .get_parent(entity)
        .expect("Failed to find parent somehow");

    let parent_width = state.data.get_width(parent);
    let parent_height = state.data.get_height(parent);

    let border_radius_top_left = match state
        .style
        .border_radius_top_left
        .get(entity)
        .cloned()
        .unwrap_or_default()
    {
        Units::Pixels(val) => val,
        Units::Percentage(val) => parent_width * val,
        _ => 0.0,
    };

    let border_radius_top_right = match state
        .style
        .border_radius_top_right
        .get(entity)
        .cloned()
        .unwrap_or_default()
    {
        Units::Pixels(val) => val,
        Units::Percentage(val) => parent_width * val,
        _ => 0.0,
    };

    let border_radius_bottom_left = match state
        .style
        .border_radius_bottom_left
        .get(entity)
        .cloned()
        .unwrap_or_default()
    {
        Units::Pixels(val) => val,
        Units::Percentage(val) => parent_width * val,
        _ => 0.0,
    };

    let border_radius_bottom_right = match state
        .style
        .border_radius_bottom_right
        .get(entity)
        .cloned()
        .unwrap_or_default()
    {
        Units::Pixels(val) => val,
        Units::Percentage(val) => parent_width * val,
        _ => 0.0,
    };

    let border_width = match state
        .style
        .border_width
        .get(entity)
        .cloned()
        .unwrap_or_default()
    {
        Units::Pixels(val) => val,
        Units::Percentage(val) => parent_width * val,
        _ => 0.0,
    };

    let mut background_color: femtovg::Color = background_color.into();
    background_color.set_alphaf(background_color.a * opacity);

    canvas.save();

    let origin = state.data.get_origin(entity);
    let transform = state.data.get_transform(entity);
    
    canvas.set_transform(transform[0], transform[1], transform[2], transform[3], transform[4], transform[5]);

    canvas.translate(bounds.x, bounds.y);

    let mut path = Path::new();

    if border_radius_bottom_left == (bounds.w - 2.0 * border_width) / 2.0
        && border_radius_bottom_right == (bounds.w - 2.0 * border_width) / 2.0
        && border_radius_top_left == (bounds.w - 2.0 * border_width) / 2.0
        && border_radius_top_right == (bounds.w - 2.0 * border_width) / 2.0
    {
        path.circle(
            0.0 + (border_width / 2.0) + (bounds.w - border_width) / 2.0,
            0.0 + (border_width / 2.0) + (bounds.h - border_width) / 2.0,
            bounds.w / 2.0,
        );
    } else {
        // Draw rounded rect
        path.rounded_rect_varying(
            (border_width / 2.0),
            (border_width / 2.0),
            bounds.w - border_width,
            bounds.h - border_width,
            border_radius_top_left,
            border_radius_top_right,
            border_radius_bottom_right,
            border_radius_bottom_left,
        );
    }

    let paint = Paint::color(background_color);

    canvas.fill_path(&mut path, paint);

    canvas.restore();
}