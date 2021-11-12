use crate::{
    physics, SystemLabels};
use bevy::prelude::*;

pub mod ship;

mod movement;
pub use movement::MoveTarget;
pub struct Selected;
pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(
                movement::movement_system
                    .system()
                    .after(SystemLabels::Input),
            );
    }
}
#[derive(Bundle, Default)]
struct ShipBundle {
    #[bundle]
    pbr_bundle: PbrBundle,
    collider_position_sync: physics::ColliderPositionSync,
    #[bundle]
    collider: physics::ColliderBundle,
    #[bundle]
    rigid_body: physics::RigidBodyBundle,
}