use super::terrain::{ImagePixelOps, Terrain};
use bevy::prelude::*;

#[derive(Clone, Debug)]
pub enum TerrainModificationShape {
    Circle { radius: f32 },
}

#[derive(Clone, Debug)]
pub enum TerrainModificationOp {
    Or,
    And,
}

#[derive(Clone, Debug)]
pub struct TerrainModification {
    pub position: Vec2,
    pub shape: TerrainModificationShape,
    pub op: TerrainModificationOp,
    pub color: Color,
}

pub trait TerrainModificationExtensions {
    fn apply_modification(
        modification: &TerrainModification,
        terrain_transform: &Transform,
        terrain_image: &mut Image,
    );
}

impl TerrainModificationExtensions for Terrain {
    fn apply_modification(
        modification: &TerrainModification,
        terrain_transform: &Transform,
        mut terrain_image: &mut Image,
    ) {
        let terrain_half_size = terrain_image.size_f32() / 2.0;
        let position =
            Self::to_terrain_space(modification.position, &terrain_transform, terrain_half_size);
        match modification.shape {
            TerrainModificationShape::Circle { radius } => {
                draw_circle(
                    &mut terrain_image,
                    position.as_ivec2(),
                    radius as _,
                    modification.color,
                );
            }
        }
    }
}

fn draw_circle(image: &mut Image, position: IVec2, radius: u32, color: Color) {
    // TODO: make the code less spooky
    let mut x = 0i32;
    let mut y = radius as i32;
    let mut p = 3i32 - 2i32 * radius as i32;

    while y >= x {
        draw_line(image, position.x - x, position.x + x, position.y - y, color);
        draw_line(image, position.x - y, position.x + y, position.y - x, color);
        draw_line(image, position.x - x, position.x + x, position.y + y, color);
        draw_line(image, position.x - y, position.x + y, position.y + x, color);
        if p < 0 {
            p += 4 * x + 6;
            x += 1
        } else {
            p += 4 * (x - y) + 10;
            x += 1;
            y -= 1;
        }
    }
}

fn draw_line(image: &mut Image, sx: i32, ex: i32, ny: i32, color: Color) {
    // TODO: make the code less spooky
    let image_size = image.size().as_ivec2();
    for i in sx..ex {
        if ny >= 0
            && ny < image_size.y
            && i >= 0
            && i < image_size.x
            && !image
                .get_pixel(IVec2::new(i as _, ny as _))
                .is_fully_transparent()
        {
            image.set_pixel(IVec2::new(i as _, ny as _), color);
        }
    }
}
