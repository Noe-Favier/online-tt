use ruscii::app::{App, State};
use ruscii::drawing::Pencil;
use ruscii::drawing::RectCharset;
use ruscii::gui::FPSCounter;
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::Window;


fn event(){

//check keyboard event

}


fn action(){}

fn display(){
    
}



fn main() {
    let mut fps_counter = FPSCounter::new();
    let mut app = App::new();

    //0 = empty space; 1 = cross; -1 = circle; 
    /*
    let morpionGrid = 
    [ 
    mut [mut 0, 0, 0], 
    mut [mut 0, 0, 0],
    mut [mut 0, 0, 0]
    ];
     */

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

        let screen_side = if pencil.dimension().x > pencil.dimension().y {
            pencil.dimension().y
        } else {
            pencil.dimension().x
        };
        let rect_size = Vec2::xy(screen_side / 4 * 2, screen_side / 4);
        let origin_x = (pencil.dimension().x/2 - (rect_size.x as f64 * 1.5).floor() as i32) as i32;
        let origin_y = (pencil.dimension().y/2 - (rect_size.y as f64 * 1.5).floor() as i32) as i32;

        for line in (0..3) {
            for row in (0..3) {
                pencil.draw_rect(
                    &RectCharset::simple_round_lines(),
                    Vec2::xy(
                        origin_x + rect_size.x * row,
                        origin_y + rect_size.y * line,
                    ),
                    rect_size,
                );
            }
        }

        pencil.draw_text(&format!("FPS: {}", fps_counter.count()), Vec2::xy(1, 1));
    });
}