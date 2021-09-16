use crate::physics;
use crate::ship;
use bevy::prelude::*;

pub struct PlayerControllerPlugin;

#[derive(Clone, PartialEq, Eq, Hash, Debug, SystemLabel)]
pub enum PlayerControlSystem {
    PlayerMovement,
}

pub struct Keys {
    forward: KeyCode,
    backward: KeyCode,
}

impl Default for Keys {
    fn default() -> Keys {
        Keys {
            forward: KeyCode::Up,
            backward: KeyCode::Down,
        }
    }
}

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(movement.system().label(PlayerControlSystem::PlayerMovement))
            .insert_resource(Keys::default());
    }
}

fn movement(
    keyboard: Res<Input<KeyCode>>,
    keys: Res<Keys>,
    mut q: Query<
        (
            &mut physics::RigidBodyVelocity,
            &physics::RigidBodyMassProps,
        ),
        With<ship::Selected>,
    >,
) {
    for (mut rb_vel, rb_mprops) in q.iter_mut() {
        let mut direction = Vec3::new(0.0, 0.0, 0.0);

        if keyboard.pressed(keys.forward) {
            //rb_forces.force = Vec3::new(1.0, 2.0, 3.0).into();
            //rb_forces.torque = Vec3::new(0.2, 0.4, 0.8).into();
            direction += Vec3::new(1.0, 0.0, 0.0);

            // Apply impulses.
            //rb_vel.apply_torque_impulse(rb_mprops, Vec3::new(140.0, 80.0, 20.0).into());
        } else if keyboard.pressed(keys.backward) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }

        rb_vel.apply_impulse(rb_mprops, direction.into());
    }
}
