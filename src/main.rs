use macroquad::prelude::*;
use macroquad::audio::{load_sound, play_sound, PlaySoundParams};
use macroquad::color;

struct Dvd {
    position: Vec2,
    velocity: Vec2,
    size: Vec2,
    hue: f32
}

impl Dvd {
    fn new(position: Vec2, velocity: Vec2, size: Vec2) -> Self {
        Self {
            position,
            velocity,
            size,
            hue: 0.0
        }
    }
    fn update(&mut self, bounce_sound: &macroquad::audio::Sound) {
        let mut bounce_events = || {
            play_sound(&bounce_sound, PlaySoundParams{looped: false, volume: 0.1});
            self.hue = (self.hue + 0.05) % 1.0;
        };

        self.position.x += self.velocity.x * get_frame_time();
        self.position.y += self.velocity.y * get_frame_time();

        if self.position.x <= 0.0 || self.position.x >= screen_width() - self.size.x {
            self.velocity.x = -self.velocity.x;
            bounce_events();
        }
        if self.position.y <= 0.0 || self.position.y >= screen_height() - self.size.y {
            self.velocity.y = -self.velocity.y;
            bounce_events();
        }
        self.position.x = self.position.x.clamp(0.0, screen_width() - self.size.x);
        self.position.y = self.position.y.clamp(0.0, screen_height() - self.size.y);
    }
    fn draw(&self, texture: &Texture2D) {
        let rgb = color::hsl_to_rgb(self.hue,1.0,0.5);
        draw_texture_ex(
            &texture,
            self.position.x, self.position.y,
            rgb,
            DrawTextureParams {
                dest_size: Some(vec2(self.size.x, self.size.y)),
                ..Default::default()
            },
        );
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "DVD".to_string(),
        window_width: 900,
        window_height: 600,
        window_resizable: true,
        fullscreen: false,
        high_dpi: true,
        sample_count: 4,
        icon: None,
        platform: Default::default(),
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    println!("DVD");

    let dvd_texture: Texture2D = load_texture("assets/dvd.png").await.unwrap();

    let mut dvds: Vec<Dvd> = Vec::new();
    dvds.push(Dvd::new(
        Vec2::new(screen_width()/2.0, screen_height()/2.0),
        Vec2::new(600.0, 300.0),
        Vec2::new(150.0, 80.0)
    ));

    let music: macroquad::audio::Sound = load_sound("assets/music.ogg").await.unwrap();
    let bounce_sound: macroquad::audio::Sound = load_sound("assets/bounce.ogg").await.unwrap();

    play_sound(&music, PlaySoundParams {looped: true, volume:1.0});

    loop {
        for dvd in &mut dvds {
            dvd.update(&bounce_sound);
        }
        clear_background(BLACK);

        for dvd in &mut dvds {
            dvd.draw(&dvd_texture);
        }
        next_frame().await;
    }
}