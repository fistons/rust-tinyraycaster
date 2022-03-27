//! Poor attempt to write the [ssloy's tinyraycaster](https://github.com/ssloy/tinyraycaster/wiki) in rust
//! to teach mysef both rust and raycasting

use framebuffer::Framebuffer;
use map::Map;
use player::Player;
use std::f64::consts::PI;
use texture::Texture;
use utils::{drop_ppm_image, pack_color};

mod framebuffer;
mod map;
mod player;
mod texture;
mod utils;

/// Windows width
const WIDTH: usize = 1024;

/// Window height
const HEIGHT: usize = 512;

const MAP_WIDTH: usize = 16;
const MAP_HEIGHT: usize = 16;

const RECT_W: usize = WIDTH / (MAP_WIDTH * 2);
const RECT_H: usize = HEIGHT / MAP_HEIGHT;

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

fn main() {
    let map = Map::default();
    // let map = "0000222222220000\
    // 1              0\
    // 1      11111   0\
    // 1     0        0\
    // 0     0  1110000\
    // 0     3        0\
    // 0   10000      0\
    // 0   3   11100  0\
    // 5   4   0      0\
    // 5   4   1  00000\
    // 0       1      0\
    // 2       1      0\
    // 0       0      0\
    // 0 0000000      0\
    // 0              0\
    // 0002222222200000";
    // assert!(map.len() == MAP_WIDTH * MAP_HEIGHT);
    //
    // Load texture
    let texture = Texture::new("resources/walltext.png").expect("Can't load texture file");

    let mut player = Player::new(3.456, 2.345, 1.523, PI / 3f64);
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    for a in 0..360 {
        framebuffer.clear();
        player.add_angle(2f64 * PI / 360f64);

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

        // Draw the player
        let (player_x, player_y) = player.get_pos();
        framebuffer.draw_rectangle(
            (player_x * RECT_W as f64) as usize,
            (player_y * RECT_H as f64) as usize,
            5,
            5,
            pack_color(255, 255, 255, None),
        );

        for i in 0..WIDTH / 2 {
            let angle = player.get_angle() - player.get_fov() / 2f64
                + player.get_fov() * i as f64 / (WIDTH / 2) as f64;

            // Draw the line
            for t in 0u32..20000 {
                let t = f64::from(t) * 0.05;
                let cx = player_x + t * angle.cos();
                let cy = player_y + t * angle.sin();

                let (pix_x, pix_y) = ((cx * RECT_W as f64) as usize, (cy * RECT_H as f64) as usize);
                framebuffer.set_pixel(pix_x, pix_y, pack_color(160, 160, 160, None)); // Write the 'dot' of the ray trajectory on the framebuffer

                if let Some(texture_id) = map.get(cx as usize, cy as usize) {
                    let column_height =
                        (HEIGHT as f64 / (t * (angle - player.get_angle()).cos())) as usize;

                    let texture_x_coordinate = wall_x_texture_coordonate(cx, cy, &texture);
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

        // Drop that PPM
        drop_ppm_image(
            &format!("./out_{a:0width$}.ppm", width = 3),
            framebuffer.get_image(),
        )
        .expect("Could not write data on disk");
    }
}
