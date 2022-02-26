use crate::physics;
use bevy::prelude::*;

#[derive(Component)]
pub struct MoveTarget(pub Vec3);

pub fn movement_system(
    mut query: Query<(
        &MoveTarget,
        &GlobalTransform,
        &mut physics::RigidBodyVelocityComponent,
        &mut physics::RigidBodyForcesComponent,
        &physics::RigidBodyMassPropsComponent,
    )>,
) {
    for (movetarget, transform, rb_vel, mut rb_forces, _rb_mprops) in query.iter_mut() {
        // this is not great but roughly approximates something that basically sort of works
        let disp: Vec3 = movetarget.0 - transform.translation;
        let acc = 10.0;

        let direction = disp.normalize();
        let distance = disp.length();
        let optimal_speed = (2.0 * acc * distance).sqrt();
        let optimal_velocity = direction * optimal_speed;

        let imp = (optimal_velocity - rb_vel.linvel.into()).normalize() * acc;

        rb_forces.force = imp.into();
    }
}
