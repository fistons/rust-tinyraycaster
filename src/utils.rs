use std::fs::OpenOptions;
use std::io::prelude::*;
use crate::{HEIGHT, WIDTH};

/// Convert Red/Green/Blue/Alpha color component in a 32 bits integer.
pub fn pack_color(r: u8, g: u8, b: u8, alpha: Option<u8>) -> u32 {
    let a = alpha.unwrap_or(0);

    ((a as u32) << 24) + ((b as u32) << 16) + ((g as u32) << 8) + (r as u32)
}

/// Convert a 32 bits integer into its four color RGBA component.
pub fn unpack_color(color: &u32) -> (u8, u8, u8, u8) {
    let r: u8 = (color & 255) as u8;
    let g: u8 = ((color >> 8) & 255) as u8;
    let b: u8 = ((color >> 16) & 255) as u8;
    let a: u8 = ((color >> 24) & 255) as u8;

    (r, g, b, a)
}


/// Write the framebuffer to the disk as a [PPM](http://netpbm.sourceforge.net/doc/ppm.html) image.
pub fn drop_ppm_image(file_name: &str, framebuffer: &[u32; WIDTH * HEIGHT]) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .append(false)
        .open(file_name)?;

    let mut buffer = format!("P6\n{WIDTH} {HEIGHT}\n255\n").as_bytes().to_vec(); // Header in the write buffer
    framebuffer
        .iter()
        .map(unpack_color)
        .for_each(|(r, g, b, _a)| buffer.extend([r, g, b])); // Frame in the write buffer

    file.write_all(&buffer)?; // Write all the things

    Ok(())
}
