use tuix::*;
use ui::*;

pub mod ui;
pub mod graph;
pub mod document;

const STYLE: &str = r#"
    .node {
        background-color: #303030;
    }

    .socket {
        background-color: green;
    }
    
    .node_label {
        background-color: #303099;
    }


"#;

fn main() {

    let window_description = WindowDescription::new().with_title("Audio Nodes");

    let app = Application::new(window_description, |state, window| {
        
        state.add_theme(STYLE);

        window.set_background_color(state, Color::rgb(30,30,30));

        let column = Column::new().build(state, window, |builder| builder);
        

        WaveView::new().build(state, column, |builder| 
            builder
                .set_background_color(Color::rgb(35,35,35))
                .set_text("Waveform editor will go here")
        );

        NodeView::new().build(state, column, |builder| {
            builder
        });
    });

    app.run();
}
