use crate::utils::pack_color;
use image::GenericImageView;

pub struct Texture {
    image_width: usize,
    count: usize,
    pub size: usize,
    image: Vec<u32>,
}

impl Texture {
    pub fn new(path: &str) -> std::io::Result<Self> {
        let img = image::open(path).expect("Could not load image");

        let (image_width, image_height) =
            (img.dimensions().0 as usize, img.dimensions().1 as usize);
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

        Ok(Texture {
            image_width,
            count: texture_count,
            size: texture_size,
            image: texture,
        })
    }

    pub fn get_pixel(&self, i: usize, j: usize, index: usize) -> u32 {
        assert!(index < self.count);
        self.image[i + index * self.size + j * self.image_width]
    }

    pub fn get_scaled_column(
        &self, texture_id: usize, texture_coordonate: usize, column_height: usize,
    ) -> Vec<u32> {
        let mut column: Vec<u32> = Vec::with_capacity(column_height);
        for i in 0..column_height {
            column.push(self.get_pixel(
                texture_coordonate,
                (i * self.size) / column_height,
                texture_id,
            ))
        }

        column
    }
}
