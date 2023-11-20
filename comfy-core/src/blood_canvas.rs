use crate::*;

use image::{GenericImage, GenericImageView, RgbaImage};
use std::fmt::Debug;

#[derive(Debug)]
pub struct CanvasBlock {
    pub image: DynamicImage,
    pub handle: TextureHandle,
    pub modified: bool,
}

pub const CANVAS_BLOCK_SIZE: i32 = 1024;
const BLOCK_SIZE: i32 = CANVAS_BLOCK_SIZE;
const PIXELS_PER_WORLD_UNIT: i32 = 16;

pub const fn blood_block_world_size() -> i32 {
    BLOCK_SIZE / PIXELS_PER_WORLD_UNIT
}

#[derive(Debug)]
pub struct BloodCanvas {
    pub creator: Arc<AtomicRefCell<dyn TextureCreator + Send + Sync + 'static>>,
    pub blocks: HashMap<IVec2, CanvasBlock>,
}

impl BloodCanvas {
    pub fn new(
        creator: Arc<AtomicRefCell<dyn TextureCreator + Send + Sync + 'static>>,
    ) -> Self {
        Self { creator, blocks: HashMap::default() }
    }

    pub fn set_pixel(&mut self, position: Vec2, color: Color) {
        let position = position * PIXELS_PER_WORLD_UNIT as f32;

        self.set_pixel_internal(position.x as i32, position.y as i32, color)
    }

    fn get_block(&mut self, x: i32, y: i32) -> &mut CanvasBlock {
        let key = ivec2(x, y);

        self.blocks.entry(key).or_insert_with(|| {
            let image = DynamicImage::ImageRgba8(RgbaImage::new(
                BLOCK_SIZE as u32,
                BLOCK_SIZE as u32,
            ));

            let name = format!("blood-canvas-{}-{}", x, y);

            let handle =
                self.creator.borrow_mut().handle_from_image(&name, &image);

            CanvasBlock { handle, image, modified: false }
        })
    }

    pub fn circle_at_internal(
        &mut self,
        position: Vec2,
        radius: i32,
        pixel_prob: f32,
        color: fn() -> Color,
    ) {
        let position = position * PIXELS_PER_WORLD_UNIT as f32;

        let x = position.x as i32;
        let y = position.y as i32;

        for dx in -radius..radius {
            for dy in -radius..radius {
                if dx * dx + dy * dy < radius * radius && flip_coin(pixel_prob)
                {
                    self.set_pixel_internal(x + dx, y + dy, color());
                }
            }
        }
    }

    fn set_pixel_internal(&mut self, x: i32, y: i32, color: Color) {
        let bx = (x as f32 / BLOCK_SIZE as f32).floor() as i32;
        let by = (y as f32 / BLOCK_SIZE as f32).floor() as i32;

        let block = self.get_block(bx, by);

        block.modified = true;
        block.image.put_pixel(
            (x - bx * BLOCK_SIZE) as u32,
            (y - by * BLOCK_SIZE) as u32,
            image::Rgba([
                (color.r * 255.0) as u8,
                (color.g * 255.0) as u8,
                (color.b * 255.0) as u8,
                (color.a * 255.0) as u8,
            ]),
        );
    }

    pub fn blit_at(
        &mut self,
        texture: TextureHandle,
        position: Vec2,
        source_rect: Option<IRect>,
        tint: Color,
    ) {
        let assets = ASSETS.borrow_mut();
        let image_map = assets.texture_image_map.lock();

        if let Some(image) = image_map.get(&texture) {
            let rect = source_rect.unwrap_or(IRect::new(
                ivec2(0, 0),
                ivec2(image.width() as i32, image.height() as i32),
            ));

            let size_offset = rect.size.as_vec2() / 2.0;

            for x in 0..rect.size.x {
                for y in 0..rect.size.y {
                    let px = image.get_pixel(
                        (x + rect.offset.x) as u32,
                        (y + rect.offset.y) as u32,
                    );

                    if px.0[3] > 0 {
                        self.set_pixel(
                            position + vec2(x as f32, y as f32) / 16.0 -
                                size_offset / 16.0,
                            // position,
                            Into::<Color>::into(px) * tint,
                            // RED,
                        );
                    }
                }
            }
        }
    }
}

pub trait TextureCreator: Debug {
    fn handle_from_image(
        &self,
        name: &str,
        image: &DynamicImage,
    ) -> TextureHandle;

    fn update_texture(&self, image: &DynamicImage, texture: TextureHandle);
}
