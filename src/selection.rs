use std::f32::consts::PI;

use bevy::{
    input::keyboard::KeyboardInput,
    input::mouse::{MouseButton, MouseMotion, MouseWheel},
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
use log::debug;

pub struct SelectionPlugin;
pub struct FrustumDebug;

#[derive(Default, Debug)]
pub struct DragStart(Option<Vec2>);

//impl Default for DragStart {
//    fn default() -> Self {
//        DragStart(None)
//    }
//}

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(selection.system())
            .add_system(drag_selection.system())
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
            _ => (),
        }
    }
}

fn screenspace_ray(
    screen_coords: Vec2,
    windows: &Res<Windows>,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<bevy_mod_raycast::Ray3d> {
    //let cursor_position = windows.get_primary().unwrap().cursor_position().unwrap();
    // store world space starting ray
    let ray = bevy_mod_raycast::Ray3d::from_screenspace(
        screen_coords,
        &windows,
        camera,
        camera_transform,
    );
    ray
}

fn create_frustum() -> Mesh {
    let mut mesh = Mesh::new(bevy::render::pipeline::PrimitiveTopology::TriangleList);
    mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]],
    );
    mesh.set_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]],
    );
    mesh.set_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]],
    );
    mesh.set_indices(Some(bevy::render::mesh::Indices::U32(vec![0, 1, 2])));
    mesh
}

fn drag_selection(
    mut commands: Commands,
    windows: Res<Windows>,
    input_mouse: Res<Input<MouseButton>>,
    query_pipeline: Res<QueryPipeline>,
    collider_query: QueryPipelineColliderComponentsQuery,
    mut drag_start: ResMut<DragStart>,
    q: Query<(&Camera, &GlobalTransform, &PerspectiveProjection)>,
    mut deselect: Query<(Entity, With<crate::ship::Selected>)>,
    mut frustum_debug: Query<(Entity, With<FrustumDebug>)>,
) {
    let cursor_position = windows.get_primary().unwrap().cursor_position();

    if input_mouse.just_pressed(MouseButton::Left) {
        drag_start.0 = cursor_position;
    } else if input_mouse.just_released(MouseButton::Left) {

        let drag_start = drag_start.0.unwrap();
        let drag_end = cursor_position.unwrap();

        if drag_start.x == drag_end.x || drag_start.y == drag_end.y {
            return;
        }


        for (entity,_) in deselect.iter_mut() {
            commands.entity(entity).remove::<crate::ship::Selected>();
        }

        let screen_points = [
            drag_start,
            drag_end,
            vec2(drag_start.x, drag_end.y),
            vec2(drag_end.x, drag_start.y)
        ];

        let mut points: Vec<Point<Real>> = vec![];

        let (camera, camera_transform, projection) = q.single().unwrap();

        for screen_point in screen_points.iter() {
            let ray =
                screenspace_ray(*screen_point, &windows, camera, camera_transform).unwrap();
            let near_point: Vec3 = ray.origin() + ray.direction() * projection.near;
            let far_point: Vec3 = ray.origin() + ray.direction() * projection.far;
            points.push(near_point.into());
            points.push(far_point.into());
        }

        let frustum = bevy_rapier3d::prelude::ConvexPolyhedron::from_convex_hull(&points.clone()).unwrap();

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
                commands
                    .entity(handle.entity())
                    .insert(crate::ship::Selected);
                true // Return `false` instead if we want to stop searching for other colliders that contain this point.
            },
        );

        let (points, indices) = frustum.to_trimesh();

        let collider = ColliderBundle {
            shape: SharedShape::new(crate::physics::TriMesh::new(points, indices)),
            collider_type: ColliderType::Sensor,
            ..ColliderBundle::default()
        };

        for (entity, debug) in frustum_debug.iter_mut() {
            commands.entity(entity).despawn();
        }

        commands
            .spawn_bundle(collider)
            .insert(FrustumDebug)
            .insert(ColliderDebugRender::default())
            .insert(ColliderPositionSync::Discrete);
    }
}

/* Test intersections inside of a system. */
fn test_intersections(
    query_pipeline: Res<QueryPipeline>,
    collider_query: QueryPipelineColliderComponentsQuery,
) {
    // Wrap the bevy query so it can be used by the query pipeline.
    let collider_set = QueryPipelineColliderComponentsSet(&collider_query);

    let shape = Cuboid::new(Vec3::new(1.0, 2.0, 3.0).into());
    let shape_pos = Isometry::new(
        Vec3::new(0.0, 1.0, 0.0).into(),
        Vec3::new(0.2, 0.7, 0.1).into(),
    );
    let groups = InteractionGroups::all();
    let filter = None;

    query_pipeline.intersections_with_shape(
        &collider_set,
        &shape_pos,
        &shape,
        groups,
        filter,
        |handle| {
            println!("The entity {:?} intersects our shape.", handle.entity());
            true // Return `false` instead if we want to stop searching for other colliders that contain this point.
        },
    );

    let aabb = AABB::new(
        Vec3::new(-1.0, -2.0, -3.0).into(),
        Vec3::new(1.0, 2.0, 3.0).into(),
    );
    query_pipeline.colliders_with_aabb_intersecting_aabb(&aabb, |handle| {
        println!(
            "The entity {:?} has an AABB intersecting our test AABB",
            handle.entity()
        );
        true // Return `false` instead if we want to stop searching for other colliders that contain this point.
    });
}
