use std::f32::consts::PI;

use bevy::{
    input::keyboard::KeyboardInput,
    input::mouse::{MouseButton, MouseMotion, MouseWheel},
    prelude::*,
    render::camera::PerspectiveProjection,
};

use bevy_mod_picking::{PickingEvent, PickingEvent::{Selection}, SelectionEvent::{JustDeselected,JustSelected}};
use log::debug;

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(
            selection
                .system(),
        );
    }
}

fn selection(mut commands: Commands, mut events: EventReader<PickingEvent>) {
    for event in events.iter() {
        println!("This event happened! {:?}", event);
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