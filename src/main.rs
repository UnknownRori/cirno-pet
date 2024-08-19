use raylib::prelude::*;

pub const IMG_WIDTH: f64 = 220.;
pub const IMG_HEIGHT: f64 = 256.;
pub const DANCE_ROW: u32 = 8;
pub const MOVE_ROW: u32 = 9;
pub const ANIMATION_FRAME: u32 = 7;
pub const ANIMATION_FPS: u32 = 5;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(220, 256)
        .title("Hello, World")
        .vsync()
        .undecorated()
        .transparent()
        .build();
    rl.set_target_fps(60);

    let fumo = rl
        .load_texture(&thread, "assets/cirno.png")
        .expect("Cirno is not here:(");

    let mut render_texture = rl
        .load_render_texture(&thread, IMG_WIDTH as u32, IMG_HEIGHT as u32)
        .expect("Render Texture fail:(");

    let mut animate_fps = 0.;
    let mut animate_frame = 0;
    while !rl.window_should_close() {
        // rl.set_window_position(window_pos.x as i32, window_pos.y as i32);
        let mut d = rl.begin_drawing(&thread);
        animate_fps += 1.;

        d.clear_background(Color::new(50, 0, 0, 128));

        // TODO : Need refactor
        let mut holding = false;
        match update(&d) {
            AppState::ShouldExit => break,
            AppState::Holding => {
                holding = true;
            }
            _ => {}
        };

        {
            // TODO : Refactor this
            if animate_fps > ANIMATION_FPS as f32 {
                if animate_frame > ANIMATION_FRAME {
                    animate_frame = 0;
                }

                let mut mode = d.begin_texture_mode(&thread, &mut render_texture);
                mode.clear_background(Color::BLANK);
                // 220x256
                let pos = if holding { MOVE_ROW } else { DANCE_ROW };
                let source = Rectangle::new(
                    IMG_WIDTH as f32 * animate_frame as f32,
                    IMG_HEIGHT as f32 * pos as f32,
                    IMG_WIDTH as f32,
                    IMG_HEIGHT as f32,
                );
                mode.draw_texture_pro(
                    &fumo,
                    source,
                    Rectangle::new(0., 0., IMG_WIDTH as f32, IMG_HEIGHT as f32),
                    Vector2::new(220., 256.),
                    180.,
                    Color::WHITE,
                );
                animate_fps = 0.;
                animate_frame += 1;
            }
        }

        d.draw_texture_ex(&render_texture, Vector2::new(0., 0.), 0., 1., Color::WHITE);
    }
}

pub enum AppState {
    ShouldExit,
    Normal,
    Holding,
}

pub fn update(d: &RaylibDrawHandle) -> AppState {
    if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT) {
        return AppState::ShouldExit;
    }

    if d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
        return AppState::Holding;
    }

    return AppState::Normal;
}
