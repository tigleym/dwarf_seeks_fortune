use crate::components::{Steering, SteeringMode};
use crate::resources::TileMap;
use amethyst::core::ecs::{Join, Read, System, WriteStorage};
use amethyst::core::Transform;

/// Implements wrapping behaviour for levels.
///
/// IE: if character falls out the bottom, they appear at top. If character goes off to the left,
/// they wrap around to the right.
pub struct LevelWrappingSystem;

impl<'s> System<'s> for LevelWrappingSystem {
    type SystemData = (
        WriteStorage<'s, Steering>,
        WriteStorage<'s, Transform>,
        Read<'s, TileMap>,
    );

    fn run(&mut self, (mut steerings, mut transforms, tile_map): Self::SystemData) {
        for (transform, steering) in (&mut transforms, &mut steerings).join() {
            if transform.translation().x < tile_map.pos.x as f32 {
                transform.set_translation_x(transform.translation().x + tile_map.dimens.x as f32);
                steering.pos.x += tile_map.dimens.x;
                steering.destination.x += tile_map.dimens.x;
            } else if transform.translation().x > (tile_map.pos.x + tile_map.dimens.x) as f32 {
                transform.set_translation_x(transform.translation().x - tile_map.dimens.x as f32);
                steering.pos.x -= tile_map.dimens.x;
                steering.destination.x -= tile_map.dimens.x;
            }

            if transform.translation().y < tile_map.pos.y as f32 {
                transform.set_translation_y(transform.translation().y + tile_map.dimens.y as f32);
                steering.pos.y += tile_map.dimens.y;
                steering.destination.y += tile_map.dimens.y;
                // Ignore warning, we'll want to add more patterns in the future.
                #[allow(clippy::single_match)]
                match steering.mode {
                    SteeringMode::Falling {
                        x_movement,
                        starting_y_pos,
                        duration,
                    } => {
                        steering.mode = SteeringMode::Falling {
                            x_movement,
                            starting_y_pos: starting_y_pos + tile_map.dimens.y as f32,
                            duration,
                        }
                    }
                    _ => (),
                };
            } else if transform.translation().y > (tile_map.pos.y + tile_map.dimens.y) as f32 {
                steering.pos.y -= tile_map.dimens.y;
                steering.destination.y -= tile_map.dimens.y;
            }
        }
    }
}
