use crate::physics;
use bevy::prelude::*;
use bevy_rapier3d::na::Normed;

pub struct MoveTarget(pub Vec3);

pub fn movement_system(
    mut query: Query<
        (
            &MoveTarget,
            &GlobalTransform,
            &mut physics::RigidBodyVelocity,
            &mut physics::RigidBodyForces,
            &physics::RigidBodyMassProps,
        ),
    >,
) {
    for (movetarget, transform, mut rb_vel, mut rb_forces, rb_mprops) in query.iter_mut() {
        // this is not great but roughly approximates something that basically sort of works
        let disp: Vec3 = movetarget.0 - transform.translation;
        let acc = 10.0;

        let direction = disp.normalize();
        let distance = disp.length();
        let optimal_speed = (2.0 * acc * distance).sqrt();
        let optimal_velocity = (direction * optimal_speed);

        let imp = (optimal_velocity - rb_vel.linvel.into()).normalize() * acc;

        rb_forces.force = imp.into();
    }
}
