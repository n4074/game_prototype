use bevy::prelude::*;
pub use bevy_rapier3d::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_startup_system(setup_physics.system())
            .add_plugin(RapierRenderPlugin);
        //.add_plugin(DebugUiPlugin);
    }
}

pub fn setup_physics(mut commands: Commands, mut config: ResMut<RapierConfiguration>) {
    /*
     * Ground
     */

    // low gravity
    config.gravity = vector!(0.0, 0.0, 0.0);

    // false ground
    let ground_size = 200.1;
    let ground_height = 0.1;
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(ground_size, ground_height, ground_size),
        position: [0.0, -ground_height - 10.0, 0.0].into(),
        ..ColliderBundle::default()
    };

    //commands
    //    .spawn_bundle(collider)
    //    .insert(ColliderDebugRender::default())
    //    .insert(ColliderPositionSync::Discrete);
}
