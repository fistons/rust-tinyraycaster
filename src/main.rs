//! Poor attempt to write the [ssloy's tinyraycaster](https://github.com/ssloy/tinyraycaster/wiki) in rust
//! to teach mysef both rust and raycasting

use std::fs::OpenOptions;
use std::io::prelude::*;

/// Windows width
const WIDTH: usize = 1024;

/// Window height
const HEIGHT: usize = 512;

const MAP_WIDTH: usize = 16;
const MAP_HEIGHT: usize = 16;

const RECT_W: usize = WIDTH / (MAP_WIDTH * 2);
const RECT_H: usize = HEIGHT / MAP_HEIGHT;

/// Convert Red/Green/Blue/Alpha color component in a 32 bits integer.
fn pack_color(r: u8, g: u8, b: u8, alpha: Option<u8>) -> u32 {
    let a = alpha.unwrap_or(0);

    ((a as u32) << 24) + ((b as u32) << 16) + ((g as u32) << 8) + (r as u32)
}

/// Convert a 32 bits integer into its four color RGBA component.
fn unpack_color(color: &u32) -> (u8, u8, u8, u8) {
    let r: u8 = (color & 255) as u8;
    let g: u8 = ((color >> 8) & 255) as u8;
    let b: u8 = ((color >> 16) & 255) as u8;
    let a: u8 = ((color >> 24) & 255) as u8;

    (r, g, b, a)
}

/// Draw a rectangle on the framebuffer.
fn draw_rectangle(framebuffer: &mut [u32; WIDTH * HEIGHT], x: usize, y: usize, w: usize, h: usize, color: u32) {
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

/// Write the framebuffer to the disk as a [PPM](http://netpbm.sourceforge.net/doc/ppm.html) image.
fn drop_ppm_image(file_name: &str, framebuffer: &[u32; WIDTH * HEIGHT]) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .append(false)
        .open(file_name)?;

    let mut buffer = format!("P6\n{WIDTH} {HEIGHT}\n255\n").as_bytes().to_vec(); // Header in the write buffer
    framebuffer.iter()
      .map(|pix| unpack_color(pix))
      //.map(|(r, g, b, _a)| vec![r, g, b])
      //.for_each(|pix| buffer.extend(pix)); // Slower but why?
      .for_each(|(r, g, b, _a)| {
        buffer.push(r);
        buffer.push(g);
        buffer.push(b);
      }); // Frame in the write buffer

    file.write_all(&buffer)?; // Write all the things

    Ok(())
}

fn main() {
    let map = "0000222222220000\
               1              0\
               1      11111   0\
               1     0        0\
               0     0  1110000\
               0     3        0\
               0   10000      0\
               0   0   11100  0\
               0   0   0      0\
               0   0   1  00000\
               0       1      0\
               2       1      0\
               0       0      0\
               0 0000000      0\
               0              0\
               0002222222200000";
    assert!(map.len() == MAP_WIDTH * MAP_HEIGHT);

    // Init colors
    let mut colors = [pack_color(0, 0, 0, None); 10];
    for i in 0..colors.len() {
      colors[i] = pack_color(rand::random::<u8>() %255, rand::random::<u8>() % 255, rand::random::<u8>() % 255, None);
    }

    let (player_x, player_y, mut player_a): (f64, f64, f64) = (3.456, 2.345, 1.523);
    let player_fov = std::f64::consts::PI / 3f64;
    for a in 0..360 {

      player_a += 2f64 * std::f64::consts::PI / 360f64;

      let mut framebuffer = [pack_color(255, 255, 255, None); WIDTH * HEIGHT];

      // Draw the map
      for (i, c) in map.chars().enumerate() {
        let x = i % MAP_WIDTH * RECT_H;
        let y = i / MAP_WIDTH * RECT_W;
        let color = colors[c as usize % 10];
        match c {
            ' ' => (), // Blank char, so nothing to write on the map
            _ => draw_rectangle(&mut framebuffer, x, y, RECT_W, RECT_H, color)
        }
      }

      // Draw the player
      draw_rectangle(&mut framebuffer, (player_x * RECT_W as f64) as usize, (player_y * RECT_H as f64) as usize, 5, 5, pack_color(255, 255, 255, None));

      for i in 0..WIDTH / 2 {
          let angle = player_a - player_fov / 2f64 + player_fov * i as f64 / (WIDTH / 2) as f64;

          // Draw the line
          for t in 0u32..20000 {
              let t = f64::from(t) * 0.05;
              let cx = player_x as f64 + t * angle.cos();
              let cy = player_y as f64 + t * angle.sin();
            
              let (pix_x, pix_y) = ((cx * RECT_W as f64) as usize, (cy * RECT_H as f64) as usize);
              framebuffer[pix_x + pix_y * WIDTH] = pack_color(160, 160, 160, None); // Write the 'dot' of the ray trajectory on the framebuffer

              match map.chars().nth(cx as usize + cy as usize * MAP_WIDTH) {
                  Some(c) if c != ' ' => {
                    let column_height = (HEIGHT as f64/ (t* (angle - player_a).cos())) as usize;
                    let color = colors[c as usize % 10];
                    draw_rectangle(&mut framebuffer, WIDTH / 2 + i, HEIGHT/2 - column_height/2, 1, column_height, color);
                    break;
                  },
                  _ => (),
              }
          }
      }

      // Drop that PPM
      drop_ppm_image(&format!("./out_{a:0width$}.ppm", width = 3), &framebuffer).expect("Could not write data on disk");
    }
}
