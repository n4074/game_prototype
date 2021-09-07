use std::f32::consts::PI;

use bevy::{
    input::keyboard::KeyboardInput,
    input::mouse::{MouseButton, MouseMotion, MouseWheel},
    prelude::*,
    render::camera::PerspectiveProjection,
};

use bevy_rapier3d::prelude::*;

use bevy_mod_picking::{PickingEvent, PickingEvent::{Selection}, SelectionEvent::{JustDeselected,JustSelected}};
use log::debug;

pub struct SelectionPlugin;

#[derive(Default)]
pub struct DragStart(Vec3);

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(
            selection
                .system(),
        )
        .insert_resource(DragStart::default());
    }
}

fn selection(mut commands: Commands, mut events: EventReader<PickingEvent>) {
    for event in events.iter() {
        match event {
            Selection(JustSelected(entity)) => {
                commands.entity(*entity).insert(crate::ship::Selected);
            }
            Selection(JustDeselected(entity)) => {
                commands.entity(*entity).remove::<crate::ship::Selected>();
            }
            _ => {()}
        }
    }
}

fn drag_selection(mut input_mouse: Res<Input<MouseButton>>, mut drag_start: ResMut<DragStart>) {
    if input_mouse.just_pressed(MouseButton::Left) {
        // store world space starting ray
    }

    if input_mouse.just_released(MouseButton::Left) {
        // store world space ending ray
    }
}

/* Test intersections inside of a system. */
fn test_intersections(query_pipeline: Res<QueryPipeline>, collider_query: QueryPipelineColliderComponentsQuery) {
    // Wrap the bevy query so it can be used by the query pipeline.
    let collider_set = QueryPipelineColliderComponentsSet(&collider_query);

    let shape = Cuboid::new(Vec3::new(1.0, 2.0, 3.0).into());
    let shape_pos = Isometry::new(Vec3::new(0.0, 1.0, 0.0).into(), Vec3::new(0.2, 0.7, 0.1).into());
    let groups = InteractionGroups::all();
    let filter = None;

    query_pipeline.intersections_with_shape(
        &collider_set, &shape_pos, &shape, groups, filter, |handle| {
        println!("The entity {:?} intersects our shape.", handle.entity());
        true // Return `false` instead if we want to stop searching for other colliders that contain this point.
    });

    let aabb = AABB::new(Vec3::new(-1.0, -2.0, -3.0).into(), Vec3::new(1.0, 2.0, 3.0).into());
    query_pipeline.colliders_with_aabb_intersecting_aabb(&aabb, |handle| {
        println!("The entity {:?} has an AABB intersecting our test AABB", handle.entity());
        true // Return `false` instead if we want to stop searching for other colliders that contain this point.
    });
}