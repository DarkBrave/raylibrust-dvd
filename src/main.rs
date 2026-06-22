use raylib::prelude::*;

struct Dvd {
    position: Vector2,
    velocity: Vector2,
    size: Vector2,
    hue: f32
}

impl Dvd {
    fn new(position: Vector2, velocity: Vector2, size: Vector2) -> Self {
        Self {
            position,
            velocity,
            size,
            hue: 0.0
        }
    }
    fn update(&mut self, screen: &Vector2, dt: &f32, bounce_sound: &Sound) {
        let mut bounce_events = || {
            self.hue = (self.hue + 30.0) % 360.0;
            bounce_sound.play();
        };

        self.position.x += self.velocity.x * dt;
        self.position.y += self.velocity.y * dt;

        if self.position.x <= 0.0 || self.position.x >= screen.x - self.size.x {
            self.velocity.x = -self.velocity.x;
            bounce_events();
        }
        if self.position.y <= 0.0 || self.position.y >= screen.y - self.size.y {
            self.velocity.y = -self.velocity.y;
            bounce_events();
        }
        self.position.x = self.position.x.clamp(0.0, screen.x - self.size.x);
        self.position.y = self.position.y.clamp(0.0, screen.y - self.size.y);
    }
    fn draw(&self, d: &mut RaylibDrawHandle, texture: &Texture2D, source: &Rectangle) {
        let dvd_logo: Rectangle = Rectangle::new(self.position.x, self.position.y, self.size.x, self.size.y);
        let current_color = Color::color_from_hsv(self.hue, 1.0, 1.0);
        d.draw_texture_pro(&texture, *source, dvd_logo, Vector2::new(0.0, 0.0), 0.0, current_color);
    }
}

fn main() {
    println!("Test App");
    let (mut rl, thread) = raylib::init()
        .size(900, 600)
        .title("DVD")
        .resizable()
        .build();

    let dvd_texture: Texture2D = rl.load_texture(&thread, "assets/dvd.png").unwrap();
    let dvd_source: Rectangle = Rectangle::new(0.0, 0.0, dvd_texture.width as f32, dvd_texture.height as f32);

    let mut dvd = Dvd::new(
        Vector2::new(rl.get_screen_width() as f32/2.0, rl.get_screen_height() as f32/2.0),
        Vector2::new(600.0, 300.0),
        Vector2::new(150.0, 80.0)
    );

    let audio: RaylibAudio = RaylibAudio::init_audio_device().unwrap();
    let music: Music = audio.new_music("assets/music.ogg").unwrap();
    let bounce_sound: Sound = audio.new_sound("assets/bounce.ogg").unwrap();
    bounce_sound.set_volume(0.1);

    music.play_stream();

    while !rl.window_should_close() {
        music.update_stream();
        let dt: f32 = rl.get_frame_time();
        let screen: Vector2 = {Vector2::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32)};

        dvd.update(&screen, &dt, &bounce_sound);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        dvd.draw(&mut d, &dvd_texture, &dvd_source);
    }
}