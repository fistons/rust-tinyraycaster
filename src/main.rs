//! Poor attempt to write the [ssloy's tinyraycaster](https://github.com/ssloy/tinyraycaster/wiki) in rust
//! to teach mysef both rust and raycasting

use framebuffer::Framebuffer;
use map::Map;
use player::Player;
use sprite::Sprite;
use std::f64::consts::PI;
use texture::Texture;
use utils::{drop_ppm_image, pack_color};

mod framebuffer;
mod map;
mod player;
mod sprite;
mod texture;
mod utils;
mod gui;

/// Windows width
const WIDTH: usize = 1024;

/// Window height
const HEIGHT: usize = 512;

fn wall_x_texture_coordonate(x: f64, y: f64, texture: &Texture) -> usize {
    let hit_x = x - (x + 0.5).floor();
    let hit_y = y - (y + 0.5).floor();

    let mut x_texture_coordinate = if hit_y.abs() > hit_x.abs() {
        hit_y * texture.get_size() as f64
    } else {
        hit_x * texture.get_size() as f64
    };

    if x_texture_coordinate < 0f64 {
        x_texture_coordinate += texture.get_size() as f64;
    }

    x_texture_coordinate as usize
}

fn map_show_sprite(sprite: &Sprite, framebuffer: &mut Framebuffer, map: &Map) {
    let rect_w = (framebuffer.get_width() / (map.get_width() * 2)) as f64;
    let rect_h = (framebuffer.get_height() / map.get_height()) as f64;

    framebuffer.draw_rectangle(
        (sprite.get_x() * rect_w - 3f64) as usize,
        (sprite.get_y() * rect_h - 3f64) as usize,
        6,
        6,
        pack_color(255, 0, 0, None),
    )
}

fn draw_sprite(
    sprite: &Sprite, framebuffer: &mut Framebuffer, player: &Player, texture_monster: &Texture,
    depth_buffer: &mut [f64],
) {
    let mut sprite_direction =
        (sprite.get_y() - player.get_pos().1).atan2(sprite.get_x() - player.get_pos().0);
    while sprite_direction - player.get_angle() > PI {
        sprite_direction -= 2f64 * PI;
    }
    while sprite_direction - player.get_angle() < -PI {
        sprite_direction += 2f64 * PI;
    }

    let sprite_size = std::cmp::min(
        1000isize,
        (framebuffer.get_height() as f64 / sprite.get_player_dist()) as isize,
    );

    let offset_screen: isize = (framebuffer.get_width() as isize / 2) / 2 - sprite_size / 2; // offset on the view screen
    let h_offset: isize = ((sprite_direction - player.get_angle())
        * (framebuffer.get_width() / 2) as f64 // as f64 to keep the precision
        / player.get_fov()) as isize; // as isize because we can have a negative offset
    let h_offset = h_offset + offset_screen;
    let v_offset = framebuffer.get_height() as isize / 2 - sprite_size / 2;

    for i in 0..sprite_size {
        if h_offset + i >= framebuffer.get_width() as isize / 2 || h_offset + i < 0 {
            continue;
        }
        if depth_buffer[(h_offset + i) as usize] < sprite.get_player_dist() {
            continue; // Occulted
        }

        for j in 0..sprite_size {
            if v_offset + j >= framebuffer.get_height() as isize || v_offset + j < 0 {
                continue;
            }

            let color = texture_monster.get_pixel(
                (i * texture_monster.get_size() as isize / sprite_size) as usize,
                (j * texture_monster.get_size() as isize / sprite_size) as usize,
                sprite.get_id(),
            );
            if utils::unpack_color(&color).3 < 128 {
                // If the alpha of the color > 128 ("transparent" pixel) we skip
                continue;
            }
            framebuffer.set_pixel(
                (framebuffer.get_width() as isize / 2 + h_offset + i) as usize,
                (v_offset + j) as usize,
                color,
            )
        }
    }
}

fn render(
    framebuffer: &mut Framebuffer, map: &Map, player: &Player, sprites: &mut [Sprite],
    texture: &Texture, texture_monster: &Texture,
) {
    framebuffer.clear(pack_color(255, 255, 255, None));

    let rect_width = framebuffer.get_width() / (map.get_width() * 2);
    let rect_height = framebuffer.get_height() / map.get_height();

    // Draw the map
    for j in 0..map.get_height() {
        for i in 0..map.get_width() {
            if map.is_empty(i, j) {
                continue;
            }

            let rect_x = i * rect_width;
            let rect_y = j * rect_height;
            let texture_id = map
                .get(i, j)
                .unwrap_or_else(|| panic!("We should have a texture id at {i}:{j}"));

            framebuffer.draw_rectangle(
                rect_x,
                rect_y,
                rect_width,
                rect_height,
                texture.get_pixel(0, 0, texture_id),
            );
        }
    }

    let mut depth_buffer = vec![1f64; framebuffer.get_width() / 2];
    let (player_x, player_y) = player.get_pos();
    for i in 0..WIDTH / 2 {
        let angle = player.get_angle() - player.get_fov() / 2f64
            + player.get_fov() * i as f64 / (WIDTH / 2) as f64;

        // Draw the line
        for t in 0u32..20000 {
            let t = f64::from(t) * 0.002;
            let cx = player_x + t * angle.cos();
            let cy = player_y + t * angle.sin();

            let (pix_x, pix_y) = (
                (cx * rect_width as f64) as usize,
                (cy * rect_height as f64) as usize,
            );
            framebuffer.set_pixel(pix_x, pix_y, pack_color(160, 160, 160, None)); // Write the 'dot' of the ray trajectory on the framebuffer

            if let Some(texture_id) = map.get(cx as usize, cy as usize) {
                let dist = t * (angle - player.get_angle()).cos();
                depth_buffer[i] = dist;
                let column_height = (HEIGHT as f64 / dist) as usize;

                let texture_x_coordinate = wall_x_texture_coordonate(cx, cy, texture);
                let column =
                    texture.get_scaled_column(texture_id, texture_x_coordinate, column_height);
                let pix_x = WIDTH / 2 + i;
                for (j, pixel) in column.iter().enumerate().take(column_height) {
                    let pix_y = j + HEIGHT / 2 - column_height / 2;
                    if pix_y > HEIGHT {
                        continue;
                    }
                    framebuffer.set_pixel(pix_x, pix_y, *pixel);
                }
                break;
            }
        }
    }

    /* Compute player distance and sort the sprite by distance */
    for sprite in sprites.iter_mut() {
        let distance = ((player.get_pos().0 - sprite.get_x()).powi(2)
            + (player.get_pos().1 - sprite.get_y()).powi(2))
        .sqrt();
        sprite.set_player_dist(distance);
    }
    sprites.sort_by(|a, b| {
        b.get_player_dist()
            .to_bits()
            .cmp(&a.get_player_dist().to_bits())
    });

    for sprite in sprites {
        map_show_sprite(sprite, framebuffer, map);
        draw_sprite(
            sprite,
            framebuffer,
            player,
            texture_monster,
            &mut depth_buffer,
        );
    }
}

fn main() {
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    let mut player = Player::new(3.456, 2.345, 1.523, PI / 3f64);
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

    for i in 0..720 {
        player.add_angle(2f64 * PI / 360f64);
        render(
            &mut framebuffer,
            &map,
            &player,
            &mut sprites,
            &texture,
            &texture_monster,
        );
        drop_ppm_image(
            &format!("./out_{i:0width$}.ppm", width = 3),
            framebuffer.get_image(),
        )
        .expect("Could not write data on disk");
    }
}
