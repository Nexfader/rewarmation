use crate::terrain::{
    Terrain, TerrainModification, TerrainModificationExtensions, TerrainModificationOp,
    TerrainModificationShape,
};
use bevy::prelude::*;

#[derive(Clone, Debug, Event)]
pub struct ExplosionEvent {
    pub center: Vec2,
    pub radius: f32,
}

pub fn handle_explosions(
    mut ev_explosion: EventReader<ExplosionEvent>,
    mut images: ResMut<Assets<Image>>,
    terrains: Query<(&Transform, &Handle<Image>), With<Terrain>>,
) {
    const BORDER_THICKNESS: f32 = 4.0;
    const BORDER_COLOR: Color = Color::srgb(0.71, 0.45, 0.32);

    for event in ev_explosion.read() {
        for (transform, image) in &terrains {
            let Some(image) = images.get_mut(image) else {
                continue;
            };
            let modifications = [
                TerrainModification {
                    shape: TerrainModificationShape::Circle {
                        radius: event.radius,
                    },
                    position: event.center,
                    op: TerrainModificationOp::And,
                    color: BORDER_COLOR,
                },
                TerrainModification {
                    shape: TerrainModificationShape::Circle {
                        radius: event.radius - BORDER_THICKNESS,
                    },
                    position: event.center,
                    op: TerrainModificationOp::And,
                    color: Color::NONE,
                },
            ];

            for modification in &modifications {
                Terrain::apply_modification(modification, transform, image);
            }

            // TODO: apply force to nearby objects

            // TODO: apply damage
        }
    }
}
