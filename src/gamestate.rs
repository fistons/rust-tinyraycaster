use crate::framebuffer::Framebuffer;
use crate::map::Map;
use crate::player::Player;
use crate::sprite::Sprite;
use crate::texture::Texture;
use crate::utils::pack_color;
use crate::{draw_sprite, map_show_sprite, wall_x_texture_coordonate, HEIGHT, WIDTH};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Gamestate {
    framebuffer: Framebuffer,
    map: Map,
    player: Player,
    texture_wall: Texture,
    texture_monster: Texture,
    sprites: Vec<Sprite>,
}

impl Gamestate {
    pub fn new(
        framebuffer: Framebuffer, map: Map, player: Player, texture_wall: Texture,
        texture_monster: Texture, sprites: Vec<Sprite>,
    ) -> Self {
        Self {
            framebuffer,
            map,
            player,
            texture_wall,
            texture_monster,
            sprites,
        }
    }

    pub fn update_player(&mut self) {
        self.player.angle += self.player.turn as f64 * 0.075; // TODO measure elapsed time and modify the speed accordingly

        let nx = self.player.x + self.player.walk as f64 * self.player.angle.cos() * 0.05;
        let ny = self.player.y + self.player.walk as f64 * self.player.angle.sin() * 0.05;

        if (nx as usize) < self.map.width && (ny as usize) < self.map.height {
            if self.map.is_empty(nx as usize, self.player.y as usize) {
                self.player.x = nx;
            }
            if self.map.is_empty(self.player.x as usize, ny as usize) {
                self.player.y = ny;
            }
        }
    }

    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::KeyUp {
                keycode: Some(k), ..
            } => match k {
                Keycode::Z | Keycode::S => self.player.walk = 0,
                Keycode::Q | Keycode::D => self.player.turn = 0,
                _ => (),
            },
            Event::KeyDown {
                keycode: Some(k), ..
            } => match k {
                Keycode::Z => self.player.walk = 1,
                Keycode::S => self.player.walk = -1,
                Keycode::Q => self.player.turn = -1,
                Keycode::D => self.player.turn = 1,
                _ => (),
            },
            _ => (),
        }
    }

    pub fn render(&mut self) {
        self.framebuffer.clear(pack_color(255, 255, 255, None));

        let rect_width = crate::WIDTH / (self.map.width * 2);
        let rect_height = crate::HEIGHT / self.map.height;

        // Draw the map
        for j in 0..self.map.height {
            for i in 0..self.map.width {
                if self.map.is_empty(i, j) {
                    continue;
                }

                let rect_x = i * rect_width;
                let rect_y = j * rect_height;
                let texture_id = self
                    .map
                    .get(i, j)
                    .unwrap_or_else(|| panic!("We should have a texture id at {i}:{j}"));

                self.framebuffer.draw_rectangle(
                    rect_x,
                    rect_y,
                    rect_width,
                    rect_height,
                    self.texture_wall.get_pixel(0, 0, texture_id),
                );
            }
        }

        let mut depth_buffer = vec![1f64; crate::WIDTH / 2];
        let (player_x, player_y) = (self.player.x, self.player.y);
        for i in 0..WIDTH / 2 {
            let angle = self.player.angle - self.player.fov / 2f64
                + self.player.fov * i as f64 / (WIDTH / 2) as f64;

            // Draw the line
            for t in 0u32..20000 {
                let t = f64::from(t) * 0.002;
                let cx = player_x + t * angle.cos();
                let cy = player_y + t * angle.sin();

                let (pix_x, pix_y) = (
                    (cx * rect_width as f64) as usize,
                    (cy * rect_height as f64) as usize,
                );
                self.framebuffer
                    .set_pixel(pix_x, pix_y, pack_color(160, 160, 160, None)); // Write the 'dot' of the ray trajectory on the framebuffer

                if let Some(texture_id) = self.map.get(cx as usize, cy as usize) {
                    let dist = t * (angle - self.player.angle).cos();
                    depth_buffer[i] = dist;
                    let column_height = (HEIGHT as f64 / dist) as usize;

                    let texture_x_coordinate =
                        wall_x_texture_coordonate(cx, cy, &self.texture_wall);
                    let column = self.texture_wall.get_scaled_column(
                        texture_id,
                        texture_x_coordinate,
                        column_height,
                    );
                    let pix_x = WIDTH / 2 + i;
                    for (j, pixel) in column.iter().enumerate().take(column_height) {
                        let pix_y = j + HEIGHT / 2 - column_height / 2;
                        if pix_y > HEIGHT {
                            continue;
                        }
                        self.framebuffer.set_pixel(pix_x, pix_y, *pixel);
                    }
                    break;
                }
            }
        }

        /* Compute player distance and sort the sprite by distance */
        for sprite in self.sprites.iter_mut() {
            let distance =
                ((self.player.x - sprite.x).powi(2) + (self.player.y - sprite.y).powi(2)).sqrt();
            sprite.player_dist = distance;
        }
        self.sprites
            .sort_by(|a, b| b.player_dist.to_bits().cmp(&a.player_dist.to_bits()));

        for sprite in &self.sprites {
            map_show_sprite(sprite, &mut self.framebuffer, &self.map);
            draw_sprite(
                sprite,
                &mut self.framebuffer,
                &self.player,
                &self.texture_monster,
                &mut depth_buffer,
            );
        }
    }

    pub fn convert_buffer(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = vec![];
        let frame = &self.framebuffer.image;
        frame
            .iter()
            .for_each(|val| buffer.extend_from_slice(&val.to_le_bytes()));

        buffer
    }
}
