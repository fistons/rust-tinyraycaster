pub struct Framebuffer {
    image: Vec<u32>,
    width: usize,
    height: usize,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            image: vec![0u32; width * height],
            width,
            height,
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: u32) {
        self.image[x + y * self.width] = pixel;
    }

    pub fn clear(&mut self) {
        self.image = vec![0u32; self.width * self.height];
    }

    pub fn draw_rectangle(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        for i in 0..width {
            for j in 0..height {
                let cx = x + i;
                let cy = y + j;

                if cx > self.width || cy > self.height {
                    continue;
                }

                self.image[cx + cy * self.width] = color;
            }
        }
    }

    pub fn get_image(&self) -> &Vec<u32> {
        &self.image
    }
}
