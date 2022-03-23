//! Poor attempt to write the [ssloy's tinyraycaster](https://github.com/ssloy/tinyraycaster/wiki) in rust
//! to teach mysef both rust and raycasting

use image::GenericImageView;
use utils::{pack_color, drop_ppm_image};

mod utils;


/// Windows width
const WIDTH: usize = 1024;

/// Window height
const HEIGHT: usize = 512;

const MAP_WIDTH: usize = 16;
const MAP_HEIGHT: usize = 16;

const RECT_W: usize = WIDTH / (MAP_WIDTH * 2);
const RECT_H: usize = HEIGHT / MAP_HEIGHT;

fn texture_column(
    image: &[u32], texture_size: usize, texture_number: usize, texture_id: usize,
    texture_coordonate: usize, column_height: usize,
) -> Vec<u32> {
    let image_width = texture_size * texture_number;

    let mut column: Vec<u32> = Vec::with_capacity(column_height);
    for i in 0..column_height {
        // For each point of the texture_column
        let pix_x = texture_id * texture_size + texture_coordonate;
        let pix_y = (i * texture_size) / column_height;
        column.push(
            *image
                .get(pix_x + pix_y * image_width)
                .expect("Could not create the texture column"),
        );
    }

    column
}

/// Load a texture from an image file using the `image` crate.
/// We use it instead of the Rust port of stb (as adviced in ssloy's
/// tutorial) because it's a bit more Rusty.
fn load_image(path: &str) -> std::io::Result<(Vec<u32>, usize, usize)> {
    let img = image::open(path).expect("Could not load image");

    let (image_width, image_height) = (img.dimensions().0, img.dimensions().1);
    let texture_count = image_width / image_height;
    let texture_size = image_width / texture_count;
    if image_width != image_height * texture_count {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Error: the texture file must contain N square textures packed horizontally",
        ));
    }

    let texture: Vec<u32> = img
        .pixels()
        .into_iter()
        .map(|(_, _, p)| p)
        .map(|p| pack_color(p[0], p[1], p[2], Some(p[3])))
        .collect();

    Ok((texture, texture_size as usize, texture_count as usize))
}


/// Draw a rectangle on the framebuffer.
fn draw_rectangle(
    framebuffer: &mut [u32; WIDTH * HEIGHT], x: usize, y: usize, w: usize, h: usize, color: u32,
) {
    for i in 0..w {
        for j in 0..h {
            let cx = x + i;
            let cy = y + j;
            if cx >= WIDTH || cy >= HEIGHT {
                continue;
            }
            framebuffer[cx + cy * WIDTH] = color;
        }
    }
}


fn main() {
    let map = "0000222222220000\
               1              0\
               1      11111   0\
               1     0        0\
               0     0  1110000\
               0     3        0\
               0   10000      0\
               0   3   11100  0\
               5   4   0      0\
               5   4   1  00000\
               0       1      0\
               2       1      0\
               0       0      0\
               0 0000000      0\
               0              0\
               0002222222200000";
    assert!(map.len() == MAP_WIDTH * MAP_HEIGHT);

    // Load texture
    let (texture, texture_size, texture_count) =
        load_image("resources/walltext.png").expect("Can't load texture file");

    let (player_x, player_y, mut player_a): (f64, f64, f64) = (3.456, 2.345, 1.523);
    let player_fov = std::f64::consts::PI / 3f64;
    for a in 0..360 {
        player_a += 2f64 * std::f64::consts::PI / 360f64;

        let mut framebuffer = [pack_color(255, 255, 255, None); WIDTH * HEIGHT];

        // Draw the map
        for (i, c) in map.chars().enumerate() {
            let x = i % MAP_WIDTH * RECT_H;
            let y = i / MAP_WIDTH * RECT_W;
            match c {
                ' ' => (), // Blank char, so nothing to write on the map
                _ => {
                    let texture_id = c.to_digit(10).unwrap() as usize;
                    assert!(texture_id < texture_count);
                    draw_rectangle(
                        &mut framebuffer,
                        x,
                        y,
                        RECT_W,
                        RECT_H,
                        *texture.get(texture_id * texture_size).unwrap(),
                    );
                }
            }
        }

        // Draw the player
        draw_rectangle(
            &mut framebuffer,
            (player_x * RECT_W as f64) as usize,
            (player_y * RECT_H as f64) as usize,
            5,
            5,
            pack_color(255, 255, 255, None),
        );

        for i in 0..WIDTH / 2 {
            let angle = player_a - player_fov / 2f64 + player_fov * i as f64 / (WIDTH / 2) as f64;

            // Draw the line
            for t in 0u32..20000 {
                let t = f64::from(t) * 0.05;
                let cx = player_x + t * angle.cos();
                let cy = player_y + t * angle.sin();

                let (pix_x, pix_y) = ((cx * RECT_W as f64) as usize, (cy * RECT_H as f64) as usize);
                framebuffer[pix_x + pix_y * WIDTH] = pack_color(160, 160, 160, None); // Write the 'dot' of the ray trajectory on the framebuffer

                match map.chars().nth(cx as usize + cy as usize * MAP_WIDTH) {
                    Some(c) if c != ' ' => {
                        let column_height =
                            (HEIGHT as f64 / (t * (angle - player_a).cos())) as usize;
                        let texture_id = c.to_digit(10).unwrap() as usize;

                        let hit_x = cx - (cx + 0.5).floor();
                        let hit_y = cy - (cy + 0.5).floor();

                        let mut x_texture_coordinate = if hit_y.abs() > hit_x.abs() {
                            hit_y * texture_size as f64
                        } else {
                            hit_x * texture_size as f64
                        };

                        if x_texture_coordinate < 0f64 {
                            x_texture_coordinate += texture_size as f64;
                        }

                        let column = texture_column(
                            &texture,
                            texture_size,
                            texture_count,
                            texture_id,
                            x_texture_coordinate as usize,
                            column_height,
                        );
                        let pix_x = WIDTH / 2 + i;
                        for j in 0..column_height {
                            let pix_y = j + HEIGHT / 2 - column_height / 2;
                            if pix_y > HEIGHT {
                                continue;
                            }
                            framebuffer[pix_x + pix_y * WIDTH] = *column.get(j).unwrap();
                        }
                        break;
                    }
                    _ => (),
                }
            }
        }

        // Drop that PPM
        drop_ppm_image(&format!("./out_{a:0width$}.ppm", width = 3), &framebuffer)
            .expect("Could not write data on disk");
    }
}
