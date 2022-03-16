//! Poor attempt to write the [ssloy's tinyraycaster](https://github.com/ssloy/tinyraycaster/wiki) in rust
//! to teach mysef both rust and raycasting


use std::fs::OpenOptions;
use std::io::prelude::*;


/// Windows width
const WIDTH: usize = 512;

/// Window height
const HEIGHT: usize = 512;

/// Convert Red/Green/Blue/Alpha color component in a 32 bit integer. 
fn pack_color(r: u8, g: u8, b: u8, alpha: Option<u8>) -> u32 {
    let a = alpha.unwrap_or(0);

    ((a as u32) << 24) + ((b as u32) << 16) + ((g as u32) << 8) + (r as u32)
}

/// Convert a 32 bit integer into its four color RGBA component.
fn unpack_color(color: &u32) -> (u8, u8, u8, u8) {
    let r: u8 = (color & 255) as u8;
    let g: u8 = ((color >> 8) & 255) as u8;
    let b: u8 = ((color >> 16) & 255) as u8;
    let a: u8 = ((color >> 24) & 255) as u8;

    (r, g, b, a)
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

    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let r: u8 = (255 * i / HEIGHT) as u8;
            let g: u8 = (255 * j / WIDTH) as u8;
            let b: u8 = 0;

            framebuffer[i + j * HEIGHT] = pack_color(r, g, b, None);
        }
    }

    drop_ppm_image("./out.ppm", &framebuffer).expect("Could not write data on disk");
}
