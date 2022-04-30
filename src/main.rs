//! Poor attempt to write the [ssloy's tinyraycaster](https://github.com/ssloy/tinyraycaster/wiki) in rust
//! to teach mysef both rust and raycasting
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use std::f64::consts::PI;
use std::time::Duration;

use framebuffer::Framebuffer;
use gamestate::Gamestate;
use map::Map;
use player::Player;
use sprite::Sprite;
use texture::Texture;
use utils::pack_color;

mod framebuffer;
mod gamestate;
mod map;
mod player;
mod sprite;
mod texture;
mod utils;

/// Windows width
const WIDTH: usize = 1024;

/// Window height
const HEIGHT: usize = 512;

fn wall_x_texture_coordonate(x: f64, y: f64, texture: &Texture) -> usize {
    let hit_x = x - (x + 0.5).floor();
    let hit_y = y - (y + 0.5).floor();

    let mut x_texture_coordinate = if hit_y.abs() > hit_x.abs() {
        hit_y * texture.size as f64
    } else {
        hit_x * texture.size as f64
    };

    if x_texture_coordinate < 0f64 {
        x_texture_coordinate += texture.size as f64;
    }

    x_texture_coordinate as usize
}

fn map_show_sprite(sprite: &Sprite, framebuffer: &mut Framebuffer, map: &Map) {
    let rect_w = (WIDTH / (map.width * 2)) as f64;
    let rect_h = (HEIGHT / map.height) as f64;

    framebuffer.draw_rectangle(
        (sprite.x * rect_w - 3f64) as usize,
        (sprite.y * rect_h - 3f64) as usize,
        6,
        6,
        pack_color(255, 0, 0, None),
    )
}

fn draw_sprite(
    sprite: &Sprite, framebuffer: &mut Framebuffer, player: &Player, texture_monster: &Texture,
    depth_buffer: &mut [f64],
) {
    let mut sprite_direction = (sprite.y - player.y).atan2(sprite.x - player.x);
    while sprite_direction - player.angle > PI {
        sprite_direction -= 2f64 * PI;
    }
    while sprite_direction - player.angle < -PI {
        sprite_direction += 2f64 * PI;
    }

    let sprite_size = std::cmp::min(1000isize, (HEIGHT as f64 / sprite.player_dist) as isize);

    let offset_screen: isize = (WIDTH as isize / 2) / 2 - sprite_size / 2; // offset on the view screen
    let h_offset: isize = ((sprite_direction - player.angle)
        * (WIDTH / 2) as f64 // as f64 to keep the precision
        / player.fov) as isize; // as isize because we can have a negative offset
    let h_offset = h_offset + offset_screen;
    let v_offset = HEIGHT as isize / 2 - sprite_size / 2;

    for i in 0..sprite_size {
        if h_offset + i >= WIDTH as isize / 2 || h_offset + i < 0 {
            continue;
        }
        if depth_buffer[(h_offset + i) as usize] < sprite.player_dist {
            continue; // Occulted
        }

        for j in 0..sprite_size {
            if v_offset + j >= HEIGHT as isize || v_offset + j < 0 {
                continue;
            }

            let color = texture_monster.get_pixel(
                (i * texture_monster.size as isize / sprite_size) as usize,
                (j * texture_monster.size as isize / sprite_size) as usize,
                sprite.texture_id,
            );
            if utils::unpack_color(&color).3 < 128 {
                // If the alpha of the color > 128 ("transparent" pixel) we skip
                continue;
            }
            framebuffer.set_pixel(
                (WIDTH as isize / 2 + h_offset + i) as usize,
                (v_offset + j) as usize,
                color,
            )
        }
    }
}

fn main() {
    let framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    let player = Player::new(3.456, 2.345, 1.523, PI / 3f64);
    let map = Map::default();
    let texture_wall = Texture::new("resources/walltext.png")
        .unwrap_or_else(|_| panic!("Could not load walls texture"));
    let texture_monster = Texture::new("resources/monsters.png")
        .unwrap_or_else(|_| panic!("Could not load monsters texture"));
    let sprites = vec![
        Sprite::new(2.823, 3.812, 2, 0.0),
        Sprite::new(1.834, 8.765, 0, 0.0),
        Sprite::new(5.323, 5.365, 1, 0.0),
        Sprite::new(4.123, 10.265, 0, 0.0),
    ];

    let mut gamestate = Gamestate::new(
        framebuffer,
        map,
        player,
        texture_wall,
        texture_monster,
        sprites,
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

    let texture_creator = canvas.texture_creator();
    let mut sdl_texture = texture_creator
        .create_texture_streaming(Some(PixelFormatEnum::ABGR8888), 1024, 512)
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main_loop,
                _ => gamestate.handle_event(event),
            }
        }
        gamestate.update_player();

        gamestate.render();
        // &mut framebuffer,
        // &map,
        // &player,
        // &mut sprites,
        // &texture_wall,
        // &texture_monster,
        // );
        let buffer = gamestate.convert_buffer();
        sdl_texture.update(None, &buffer, WIDTH * 4).unwrap();

        canvas.clear();
        canvas.copy(&sdl_texture, None, None).unwrap();
        canvas.present();
        std::thread::sleep(Duration::from_millis(7)); // 1 frame evey 7 ms => 144 hz more or less
    }
}
