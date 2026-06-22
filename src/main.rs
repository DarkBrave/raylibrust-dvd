use raylib::prelude::*;

fn main() {
    println!("Test App");
    let (mut rl, thread) = raylib::init()
        .size(900, 600)
        .title("DVD")
        .resizable()
        .build();

    let mut current_hue: i32 = 0_i32;

    let texture: Texture2D = rl.load_texture(&thread, "dvd.png").unwrap();
    let dvd_source: Rectangle = Rectangle::new(0.0, 0.0, texture.width as f32, texture.height as f32);

    let mut x_pos: f32 = 450.0;
    let mut y_pos: f32 = 300.0;
    let x_size: f32 = 150.0;
    let y_size: f32 = 80.0;
    let mut velocity_x: f32 = 600.0;
    let mut velocity_y: f32 = 300.0;

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        let mut update_hue = || {
            current_hue = (current_hue + 30) % 360;
        };
        let screen_width: f32 = rl.get_screen_width() as f32;
        let screen_height: f32 = rl.get_screen_height() as f32;

        x_pos += velocity_x * dt;
        y_pos += velocity_y * dt;

        if x_pos <= 0f32 {
            x_pos = 0f32;
            velocity_x = -velocity_x;
            update_hue();
        }
        if x_pos >= screen_width - x_size {
            x_pos = screen_width - x_size;
            velocity_x = -velocity_x;
            update_hue();
        }
        if y_pos <= 0f32 {
            y_pos = 0f32;
            velocity_y = -velocity_y;
            update_hue();
        }
        if y_pos >= screen_height - y_size {
            y_pos = screen_height - y_size;
            velocity_y = -velocity_y;
            update_hue();
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        let dvd_logo: Rectangle = Rectangle::new(x_pos, y_pos, x_size, y_size);
        let current_color = Color::color_from_hsv(current_hue as f32, 1.0, 1.0);
        d.draw_texture_pro(&texture, dvd_source, dvd_logo, Vector2::new(0.0, 0.0), 0.0, current_color);
    }
}