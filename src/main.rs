use rand::RngExt;
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
    fn update(&mut self, screen: &Vector2, dt: f32, bounce_sound: &Sound) {
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
    let mut rng = rand::rng();

    let dvd_texture: Texture2D = rl.load_texture(&thread, "assets/dvd.png").unwrap();
    let dvd_source: Rectangle = Rectangle::new(0.0, 0.0, dvd_texture.width as f32, dvd_texture.height as f32);

    let mut dvds: Vec<Dvd> = Vec::new();
    dvds.push(Dvd::new(
        Vector2::new(rl.get_screen_width() as f32/2.0, rl.get_screen_height() as f32/2.0),
        Vector2::new(600.0, 300.0),
        Vector2::new(150.0, 80.0)
    ));
    let add_dvd_random = |dvds: &mut Vec<Dvd>, screen: &Vector2, rng: &mut rand::rngs::ThreadRng| {
        dvds.push(Dvd::new(
            Vector2::new(rng.random_range(0.0..screen.x), rng.random_range(0.0..screen.y)),
            Vector2::new(rng.random_range(-1000.0..1000.0), rng.random_range(-800.0..800.0)),
            Vector2::new(150.0, 80.0)
        ));
    };

    let mut debug_overlay: bool = false;
    let mut spam_dvds: bool = false;

    let audio: RaylibAudio = RaylibAudio::init_audio_device().unwrap();
    let music: Music = audio.new_music("assets/music.ogg").unwrap();
    let bounce_sound: Sound = audio.new_sound("assets/bounce.ogg").unwrap();
    bounce_sound.set_volume(0.1);

    music.play_stream();

    while !rl.window_should_close() {
        music.update_stream();

        if rl.is_key_pressed(KeyboardKey::KEY_A) { add_dvd_random(&mut dvds, &Vector2::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32), & mut rng); }
        if rl.is_key_pressed(KeyboardKey::KEY_S) { spam_dvds = !spam_dvds; }
        if rl.is_key_pressed(KeyboardKey::KEY_C) { dvds.clear(); }
        if rl.is_key_pressed(KeyboardKey::KEY_D) { debug_overlay = !debug_overlay; }

        if(spam_dvds) {add_dvd_random(&mut dvds, &Vector2::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32), &mut rng);}

        let dt: f32 = rl.get_frame_time();
        let screen: Vector2 = {Vector2::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32)};

        for dvd in &mut dvds {
            dvd.update(&screen, dt, &bounce_sound);
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        for dvd in &mut dvds {
            dvd.draw(&mut d, &dvd_texture, &dvd_source);
        }
        if debug_overlay {
            let debug_text = format!("DVDs: {}, FPS: {}", dvds.len(), d.get_fps());
            d.draw_text(&*debug_text, 20, 10, 20, Color::WHITE);
        }
    }
}