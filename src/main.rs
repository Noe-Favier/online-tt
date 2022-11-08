use ruscii::app::{App, State};
use ruscii::terminal::{Window};
use ruscii::drawing::{Pencil};
use ruscii::keyboard::{KeyEvent, Key};
use ruscii::spatial::{Vec2};
use ruscii::gui::{FPSCounter};
use ruscii::drawing::RectCharset;

fn main() {
    let mut fps_counter = FPSCounter::new();
    let mut app = App::new();

    app.run(|app_state: &mut State, window: &mut Window| {
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                KeyEvent::Pressed(Key::Q) => app_state.stop(),
                _ => (),
            }
        }

        fps_counter.update();
        let mut pencil = Pencil::new(window.canvas_mut());

        pencil.draw_rect(&RectCharset::simple_round_lines(), Vec2::xy(pencil.dimension().x/3, 1), Vec2::xy(90, 90));

        pencil.draw_text(&format!("FPS: {}", fps_counter.count()), Vec2::xy(1, 1));
    });
}