use crate::game::GamePlayState;
use bevy::{prelude::*, sprite::Anchor};

#[derive(Component)]
pub struct Terrain;

#[derive(Bundle)]
pub struct TerrainBundle {
    terrain: Terrain,
    sprite: SpriteBundle,
}

pub fn spawn_terrain(commands: &mut Commands, texture: Handle<Image>) {
    commands.spawn((
        Name::new("Terrain"),
        TerrainBundle {
            sprite: SpriteBundle {
                texture,
                sprite: Sprite {
                    anchor: Anchor::BottomCenter,
                    ..default()
                },
                transform: Transform::from_translation(Vec2::new(0.0, 0.0).extend(0.0)),
                ..default()
            },
            terrain: Terrain,
        },
        StateScoped(GamePlayState::InGame),
    ));
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CollidedSide {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Clone, Copy, Debug)]
pub struct TerrainCollisionData {
    pub position: Vec2,
    pub side: CollidedSide,
}

// Temporary implementation (requires https://github.com/bevyengine/bevy/pull/10392 to be merged)
pub trait ImagePixelOps {
    fn linearize_position(&self, position: IVec2) -> usize;
    fn get_pixel(&self, position: IVec2) -> Color;
    fn is_pixel_transparent(&self, position: IVec2) -> bool;
    fn set_pixel(&mut self, position: IVec2, value: Color);
}

impl ImagePixelOps for Image {
    fn linearize_position(&self, position: IVec2) -> usize {
        let size = self.size().as_ivec2();
        let bpp = size.x * 4;
        (position.y * bpp + (position.x * 4)) as usize
    }

    fn is_pixel_transparent(&self, position: IVec2) -> bool {
        let index = self.linearize_position(position) + 3;
        self.data[index as usize] == 0
    }

    fn get_pixel(&self, position: IVec2) -> Color {
        let index = self.linearize_position(position);
        let color = &self.data[index..index + 4];
        Color::srgba_u8(color[0], color[1], color[2], color[3])
    }

    fn set_pixel(&mut self, position: IVec2, value: Color) {
        let index = self.linearize_position(position);
        let rgba = value.to_srgba().to_u8_array();
        self.data[index..index + 4].clone_from_slice(&rgba);
    }
}

impl Terrain {
    pub fn to_terrain_space(
        position: Vec2,
        terrain_transform: &Transform,
        terrain_half_size: Vec2,
    ) -> Vec2 {
        let terrain_position = terrain_transform.translation.xy();
        Vec2::new(position.x, -position.y) - terrain_position
            + Vec2::new(terrain_half_size.x, terrain_half_size.y * 2.0)
    }

    pub fn from_terrain_space(
        position: Vec2,
        terrain_transform: &Transform,
        terrain_half_size: Vec2,
    ) -> Vec2 {
        let terrain_position = terrain_transform.translation.xy();
        (position + terrain_position - Vec2::new(terrain_half_size.x, terrain_half_size.y * 2.0))
            * Vec2::new(1.0, -1.0)
    }

    pub fn test_collision(
        position: Vec2,
        extent: Vec2,
        terrain_transform: &Transform,
        terrain_image: &Image,
    ) -> Option<TerrainCollisionData> {
        let terrain_scale = terrain_transform.scale.xy();
        let extent = extent * terrain_scale;
        let half_extent = (extent * 0.5).as_ivec2();
        let terrain_half_size = terrain_image.size().as_vec2() * 0.5 * terrain_scale;

        let position = Terrain::to_terrain_space(position, terrain_transform, terrain_half_size)
            * terrain_scale;
        let p1 = position.as_ivec2() - half_extent;
        let p2 = p1 + extent.as_ivec2();

        if p1.cmplt(IVec2::ZERO).any() || p2.cmpge(terrain_image.size().as_ivec2()).any() {
            return None;
        }

        // TODO: refactor and decompose
        let mut test_position = IVec2::new(p1.x, p2.y);
        while test_position.x <= p2.x {
            if !terrain_image.is_pixel_transparent(test_position) {
                return Some(TerrainCollisionData {
                    position: Terrain::from_terrain_space(
                        test_position.as_vec2(),
                        terrain_transform,
                        terrain_half_size,
                    ),
                    side: CollidedSide::Bottom,
                });
            }
            test_position.x += 1;
        }

        let mut test_position = IVec2::new(p2.x, p2.y);
        while test_position.y > p1.y {
            if !terrain_image.is_pixel_transparent(test_position) {
                return Some(TerrainCollisionData {
                    position: Terrain::from_terrain_space(
                        test_position.as_vec2(),
                        terrain_transform,
                        terrain_half_size,
                    ),
                    side: CollidedSide::Right,
                });
            }
            test_position.y -= 1;
        }

        let mut test_position = IVec2::new(p1.x, p2.y);
        while test_position.y > p1.y {
            if !terrain_image.is_pixel_transparent(test_position) {
                return Some(TerrainCollisionData {
                    position: Terrain::from_terrain_space(
                        test_position.as_vec2(),
                        terrain_transform,
                        terrain_half_size,
                    ),
                    side: CollidedSide::Left,
                });
            }
            test_position.y -= 1;
        }

        let mut test_position = IVec2::new(p1.x, p1.y);
        while test_position.x < p2.x {
            if !terrain_image.is_pixel_transparent(test_position) {
                return Some(TerrainCollisionData {
                    position: Terrain::from_terrain_space(
                        test_position.as_vec2(),
                        terrain_transform,
                        terrain_half_size,
                    ),
                    side: CollidedSide::Top,
                });
            }
            test_position.x += 1;
        }

        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn it_converts_from_terrain_space() {
        let position = Vec2::new(5.0, 10.0);
        let terrain_transform = Transform::from_translation(Vec3::ZERO);
        let terrain_half_size = Vec2::splat(5.0);

        let position_terrain_space =
            Terrain::to_terrain_space(position, &terrain_transform, terrain_half_size);
        let position_world_space = Terrain::from_terrain_space(
            position_terrain_space,
            &terrain_transform,
            terrain_half_size,
        );

        assert_eq!(position, position_world_space);
    }
}
