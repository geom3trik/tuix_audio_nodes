

use tuix::*;

use rand::Rng;

use femtovg::{
    Canvas, renderer::OpenGl, Align, Baseline, FillRule, FontId, ImageFlags, ImageId, LineCap, LineJoin,
    Paint, Path, Renderer, Solidity,
};

use super::node_widget::*;
use super::socket_widget::*;

pub struct NodeView {
    translate_x: f32,
    translate_y: f32,
    scale: f64,

    prev_translate_x: f32,
    prev_translate_y: f32,
    panning: bool,

    canvas: Entity,
}

impl NodeView {
    pub fn new() -> Self {
        Self {
            translate_x: 0.0,
            translate_y: 0.0,
            scale: 1.0,

            prev_translate_x: 0.0,
            prev_translate_y: 0.0,
            panning: false,

            canvas: Entity::null(),
        }
    }
}

impl Widget for NodeView {
    type Ret = Entity;

    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {

        self.canvas = Element::new().build(state, entity, |builder| 
            builder
            .set_clip_widget(entity)
            //.set_background_color(Color::rgb(50,50,200))
        );

        let sine = NodeWidget::new("Sine").build(state, self.canvas, |builder| 
            builder
        );

        let row = Row::new().build(state, sine, |builder| 
            builder
                .set_height(Pixels(30.0))
                .set_child_space(Stretch(1.0))
        );

        Label::new("Out").build(state, row, |builder| 
            builder
                .set_child_space(Stretch(1.0))
                .set_child_right(Pixels(5.0))
                .set_space(Pixels(0.0))
                .set_hoverability(false)
        );

        OutputSocket::new().build(state, row, |builder| 
            builder
                .set_left(Stretch(0.0))
                .set_right(Pixels(-10.0))
        );

        let row = Row::new().build(state, sine, |builder| 
            builder
                .set_height(Pixels(30.0))
                .set_child_space(Stretch(1.0))
        );
    
        InputSocket::new().build(state, row, |builder| 
            builder
                .set_left(Pixels(-10.0))
                .set_right(Stretch(0.0))
        );
    
        Label::new("Freq").build(state, row, |builder| 
            builder
                .set_child_space(Stretch(1.0))
                .set_child_left(Pixels(5.0))
                .set_space(Pixels(0.0))
                .set_hoverability(false)
        );

        Textbox::new("440").build(state, row, |builder| 
            builder
                .set_child_space(Stretch(1.0))
                .set_child_left(Pixels(5.0))
                .set_space(Pixels(0.0))
                .set_background_color(Color::rgb(15, 15, 15))
                .set_right(Pixels(5.0))
                .set_color(Color::white())
                .set_opacity(1.0)
        );

        let amplify = NodeWidget::new("Amplify").build(state, self.canvas, |builder| 
            builder
                .set_left(Pixels(200.0))
                .set_top(Pixels(200.0))
        );

        let row = Row::new().build(state, amplify, |builder| 
            builder
                .set_height(Pixels(30.0))
                .set_child_space(Stretch(1.0))
        );
    
        InputSocket::new().build(state, row, |builder| 
            builder
                .set_left(Pixels(-10.0))
                .set_right(Stretch(0.0))
        );
    
        Label::new("In").build(state, row, |builder| 
            builder
                .set_child_space(Stretch(1.0))
                .set_child_left(Pixels(5.0))
                .set_space(Pixels(0.0))
                .set_hoverability(false)
        );

        let row = Row::new().build(state, amplify, |builder| 
            builder
                .set_height(Pixels(30.0))
                .set_child_space(Stretch(1.0))
        );

        Label::new("Out").build(state, row, |builder| 
            builder
                .set_child_space(Stretch(1.0))
                .set_child_right(Pixels(5.0))
                .set_space(Pixels(0.0))
                .set_hoverability(false)
        );

        OutputSocket::new().build(state, row, |builder| 
            builder
                .set_left(Stretch(0.0))
                .set_right(Pixels(-10.0))
        );

        let output = NodeWidget::new("Output").build(state, self.canvas, |builder| 
            builder
                .set_left(Pixels(300.0))
                .set_top(Pixels(300.0))
        );

        let row = Row::new().build(state, output, |builder| 
            builder
                .set_height(Pixels(30.0))
                .set_child_space(Stretch(1.0))
        );
    
        InputSocket::new().build(state, row, |builder| 
            builder
                .set_left(Pixels(-10.0))
                .set_right(Stretch(0.0))
        );
    
        Label::new("In").build(state, row, |builder| 
            builder
                .set_child_space(Stretch(1.0))
                .set_child_left(Pixels(5.0))
                .set_space(Pixels(0.0))
                .set_hoverability(false)
        );

        // for i in 1..800 {
        //     let rand_x = rand::thread_rng().gen_range(0, 800);
        //     let rand_y = rand::thread_rng().gen_range(0,600);
        //     NodeWidget::new().build(state, entity, |builder| 
        //         builder
        //             .set_left(Pixels(rand_x as f32))
        //             .set_top(Pixels(rand_y as f32))
        //     );
        // }

        state.set_focus(entity);

        entity
        //.set_background_color(state, Color::rgb(50,100,50))
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        
        if let Some(window_event) = event.message.downcast() {
            match window_event {

                
                WindowEvent::MouseDown(button) => {
                    //if event.target == entity {
                        if *button == MouseButton::Middle {
                            self.panning = true;
                            state.capture(entity);
                            self.prev_translate_x = self.translate_x;
                            self.prev_translate_y = self.translate_y;
                        }
                    //}
                }

                WindowEvent::MouseUp(button) => {
                    if event.target == entity {
                        if *button == MouseButton::Middle {
                            self.panning = false;
                            state.release(entity);
                        }
                    }
                }

                WindowEvent::MouseMove(x, y) => {
                    // When middle mouse button is pressed, pan the canvas when mouse is moved
                    if self.panning {
                        let dx = *x - state.mouse.middle.pos_down.0;
                        let dy = *y - state.mouse.middle.pos_down.1;

                        self.translate_x = self.prev_translate_x + dx;
                        self.translate_y = self.prev_translate_y + dy;
                        //println!("x: {}, y: {}", self.translate_x, self.translate_y);
                        self.canvas.set_translate(state, (self.translate_x, self.translate_y));
                        state.insert_event(Event::new(WindowEvent::Redraw).target(Entity::root()));
                    }

                }

                WindowEvent::MouseScroll(x,y) => {
                    self.scale += 0.1 * *y as f64;
                    if self.scale >= 2.0 {
                        self.scale = 2.0;
                    }

                    if self.scale <= 0.5 {
                        self.scale = 0.5;
                    }



                    self.canvas.set_scale(state, self.scale as f32);
                    //println!("scale: {}", self.scale);
                }

                WindowEvent::KeyDown(code, key) => {
                    println!("Key: {:?} {:?}", code, key);
                    match *code {


                        _=> {}
                    }
                }

                _=> {}
            }
        }
    }
}