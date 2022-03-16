//! Poor attempt to write the [ssloy's tinyraycaster](https://github.com/ssloy/tinyraycaster/wiki) in rust
//! to teach mysef both rust and raycasting

use std::fs::OpenOptions;
use std::io::prelude::*;

/// Windows width
const WIDTH: usize = 512;

/// Window height
const HEIGHT: usize = 512;

const MAP_WIDTH: usize = 16;
const MAP_HEIGHT: usize = 16;

const RECT_W: usize = WIDTH / MAP_WIDTH;
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

fn draw_rectangle(framebuffer: &mut [u32; WIDTH * HEIGHT], x: usize, y: usize, w: usize, h: usize, color: u32) {
    for i in 0..w {
        for j in 0..h {
            let cx = x + i;
            let cy = y + j;

            assert!(cx < WIDTH && cy < HEIGHT);
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

    file.write_all(format!("P6\n{WIDTH} {HEIGHT}\n255\n").as_bytes())?;
    for i in framebuffer {
        let (r, g, b, _a) = unpack_color(i);

        file.write_all(&[r, g, b])?;
    }

    file.sync_all()?;

    Ok(())
}

fn main() {
    let mut framebuffer: [u32; WIDTH * HEIGHT] = [255; WIDTH * HEIGHT];

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

    let (player_x, player_y, player_a): (f64, f64, f64) = (3.456, 2.345, 1.523);

    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let r: u8 = (255 * i / HEIGHT) as u8;
            let g: u8 = (255 * j / WIDTH) as u8;
            let b: u8 = 0;

            framebuffer[i + j * HEIGHT] = pack_color(r, g, b, None);
        }
    }

    // Draw the rectangles
    for (i, c) in map.chars().enumerate() {
        let x = i % MAP_WIDTH * RECT_H;
        let y = i / MAP_WIDTH * RECT_W;
        match c {
            ' ' => (), 
            _ => draw_rectangle(&mut framebuffer, x, y, RECT_W, RECT_H, pack_color(0, 255, 255, None)),
        }
    }

    // Draw the player
    draw_rectangle(&mut framebuffer, (player_x * RECT_W as f64) as usize, (player_y * RECT_H as f64) as usize, 5, 5, pack_color(255, 255, 255, None));

    // Draw the line
    for t in 0u32..200000 {
      let t = f64::from(t) * 0.005;
      let cx = player_x as f64 + t*player_a.cos();
      let cy = player_y as f64 + t*player_a.sin();

      if let Some(c) = map.chars().nth(cx as usize + cy as usize * MAP_WIDTH) {
        if c != ' ' {
          break;
        }
      }

      let (pix_x, pix_y) = ((cx * RECT_W as f64) as usize, (cy * RECT_H as f64) as usize);
      framebuffer[pix_x + pix_y * WIDTH] = pack_color(255, 255, 255, None);
    }

    // Drop that PPM
    drop_ppm_image("./out.ppm", &framebuffer).expect("Could not write data on disk");
}
