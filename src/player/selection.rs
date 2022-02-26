//! # Selection
//! The selection module handles both click and drag selection events
use bevy::{
    input::mouse::MouseButton,
    math::*,
    prelude::*,
    render::camera::{Camera, PerspectiveProjection},
};

use bevy_rapier3d::prelude::*;

use bevy_mod_picking::{
    PickingEvent,
    PickingEvent::Selection,
    SelectionEvent::{JustDeselected, JustSelected},
};

//use crate::units;

pub struct SelectionPlugin;

#[derive(Default, Debug, Copy, Clone)]
pub struct DragCoords {
    start: Option<Vec2>,
    end: Option<Vec2>,
}
#[derive(Default, Debug, Copy, Clone)]
pub struct DragRays {
    start: Option<crate::player::camera::MouseRay>,
    end: Option<crate::player::camera::MouseRay>,
}
impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(selection.system())
            .add_system(drag_selection.system())
            .add_system(debug_draw_frustum.system())
            .insert_resource(DragCoords::default())
            .insert_resource(DragRays::default())
            .insert_resource(Option::<ConvexPolyhedron>::default());
    }
}

fn selection(mut commands: Commands, mut events: EventReader<PickingEvent>) {
    for event in events.iter() {
        match event {
            Selection(JustSelected(entity)) => {
                commands.entity(*entity).insert(super::Selected);
            }
            Selection(JustDeselected(entity)) => {
                commands.entity(*entity).remove::<super::Selected>();
            }
            _ => (),
        }
    }
}

/// System handles dra
fn drag_selection(
    mut commands: Commands,
    windows: Res<Windows>,
    input_mouse: Res<Input<MouseButton>>,
    query_pipeline: Res<QueryPipeline>,
    collider_query: QueryPipelineColliderComponentsQuery,
    mut drag: Local<DragCoords>,
    mut dragrays: Local<DragRays>,
    q: Query<(
        &Camera,
        &GlobalTransform,
        &PerspectiveProjection,
        &crate::player::camera::MouseRayComponent,
    )>,
    mut deselect: Query<(Entity, With<super::Selected>)>,
) {
    let cursor_position = windows.get_primary().and_then(|w| w.cursor_position());

    let (camera, camera_transform, projection, mouseray) = q.single();

    if input_mouse.just_pressed(MouseButton::Left) {
        drag.start = cursor_position;
        dragrays.start = mouseray.0;
    }

    if input_mouse.pressed(MouseButton::Left) {
        drag.end = cursor_position;
        dragrays.end = mouseray.0;
    }

    if input_mouse.just_released(MouseButton::Left) {
        if let DragCoords {
            start: Some(start),
            end: Some(end),
        } = *drag
        {
            if start.cmpeq(end).any() {
                // start and end coords should not share x or y coords
                return;
            }

            for (entity, _) in deselect.iter_mut() {
                commands.entity(entity).remove::<super::Selected>();
            }

            //let (camera, camera_transform, projection, mouseray) = q.single().unwrap();

            let max = Vec2::max(start, end);
            let min = Vec2::min(start, end);

            let corners = [
                vec2(min.x, min.y),
                vec2(max.x, min.y),
                vec2(max.x, max.y),
                vec2(min.x, max.y),
            ];

            let mut points: Vec<Point<Real>> = Vec::with_capacity(8);

            for corner in corners.iter() {
                let ray = bevy_mod_raycast::Ray3d::from_screenspace(
                    *corner,
                    &windows,
                    camera,
                    camera_transform,
                )
                .expect("Failed to cast screen ray");

                points.push((ray.origin() + ray.direction() * projection.near).into());
                points.push((ray.origin() + ray.direction() * projection.far).into());
            }

            let frustum =
                bevy_rapier3d::prelude::ConvexPolyhedron::from_convex_mesh(points, FRUSTUM_INDICES)
                    .unwrap();

            commands.insert_resource(Some(frustum.clone()));

            let collider_set = QueryPipelineColliderComponentsSet(&collider_query);
            let groups = InteractionGroups::all();
            let filter = None;

            query_pipeline.intersections_with_shape(
                &collider_set,
                &[0.0, 0.0, 0.0].into(), // we constructed the frustum with worldspace coordinates
                &frustum,
                groups,
                filter,
                |handle| {
                    println!("The entity {:?} intersects our shape.", handle.entity());
                    commands.entity(handle.entity()).insert(super::Selected);
                    true
                },
            );
        }
    }
}

// These indices are used to form a convex polyhedron from
// a trimesh. Note that the
const FRUSTUM_INDICES: &[[u32; 3]] = &[
    // front
    [0, 2, 4],
    [0, 4, 6],
    // bottom
    [0, 3, 2],
    [0, 1, 3],
    // left
    [0, 7, 1],
    [0, 6, 7],
    // right
    [5, 2, 3],
    [5, 4, 2],
    // top
    [5, 6, 4],
    [5, 7, 6],
    // back
    [5, 3, 1],
    [5, 1, 7],
];

const FRUSTUM_WIREFRAME_INDICES: &[(usize, usize)] = &[
    // near rectangle
    (0, 2),
    (2, 4),
    (4, 6),
    (6, 0),
    // frustum edges
    (0, 1),
    (2, 3),
    (4, 5),
    (6, 7),
    // far rectangle
    (1, 3),
    (3, 5),
    (5, 7),
    (7, 1),
];

// Draws debug lines to show the selection frustum
fn debug_draw_frustum(
    mut frustum: ResMut<Option<ConvexPolyhedron>>,
    mut lines: ResMut<bevy_prototype_debug_lines::DebugLines>,
) {
    if let Some(frustum) = frustum.take() {
        let (points, _indices) = frustum.to_trimesh();

        for (start, end) in FRUSTUM_WIREFRAME_INDICES {
            lines.line(points[*start].into(), points[*end].into(), 5.0)
        }
    }
}
