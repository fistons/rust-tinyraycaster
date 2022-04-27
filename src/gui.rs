use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use std::f64::consts::PI;
use std::time::Duration;

use crate::framebuffer::Framebuffer;
use crate::map::Map;
use crate::player::Player;
use crate::render;
use crate::sprite::Sprite;
use crate::texture::Texture;

/// Windows width
const WIDTH: usize = 1024;

/// Window height
const HEIGHT: usize = 512;

pub fn main() {
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    let player = Player::new(3.456, 2.345, 1.523, PI / 3f64);
    let map = Map::default();
    let texture = Texture::new("resources/walltext.png")
        .unwrap_or_else(|_| panic!("Could not load walls texture"));
    let texture_monster = Texture::new("resources/monsters.png")
        .unwrap_or_else(|_| panic!("Could not load monsters texture"));
    let mut sprites = vec![
        Sprite::new(2.823, 3.812, 2, 0.0),
        Sprite::new(1.834, 8.765, 0, 0.0),
        Sprite::new(5.323, 5.365, 1, 0.0),
        Sprite::new(4.123, 10.265, 0, 0.0),
    ];
    render(
        &mut framebuffer,
        &map,
        &player,
        &mut sprites,
        &texture,
        &texture_monster,
    );

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Tiny Raycaster", 1024, 512)
        .position_centered()
        .vulkan()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();

    let mut buffer: Vec<u8> = vec![];
    framebuffer
        .get_image()
        .iter()
        .for_each(|val| buffer.extend_from_slice(&val.to_le_bytes()));
    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator
        .create_texture_streaming(Some(PixelFormatEnum::ABGR8888), 1024, 512)
        .unwrap();

    texture
        .update(None, &buffer, framebuffer.get_width() * 4)
        .unwrap();

    canvas.copy(&texture, None, None).unwrap();

    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'main_loop: loop {
        canvas.clear();
        canvas.copy(&texture, None, None).unwrap();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main_loop,
                _ => {}
            }
        }

        canvas.present();
        std::thread::sleep(Duration::from_millis(7)); // 1 frame every 7 ms => 144 hz more or less
    }
}
