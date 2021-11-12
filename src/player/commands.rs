use crate::{
    input::{MappedInput, Switch},
    player::camera::ControlCursor,
    units::MoveTarget,
    SystemLabels,
};
use bevy::prelude::*;

pub struct CommandPlugin;

impl Plugin for CommandPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(commands.system().after(SystemLabels::Input));
    }
}

#[derive(Debug, num_derive::ToPrimitive)]
pub enum Orders {
    Move,
}

fn setup(mut inputs: ResMut<MappedInput>) {
    inputs.bind(
        [Switch::Key(KeyCode::M), MouseButton::Left.into()],
        Orders::Move,
    );
}

fn commands(
    mut commands: Commands,
    inputs: Res<MappedInput>,
    cursor: Query<&Option<ControlCursor>>,
    selected_units: Query<Entity, With<crate::units::Selected>>,
) {
    if inputs.just_deactivated(Orders::Move) {
        if let Ok(Some(ControlCursor { pos })) = cursor.single() {
            for entity in selected_units.iter() {
                log::debug!("commanding unit {:?} to position {:?}", entity, pos);
                commands.entity(entity).insert(MoveTarget(*pos));
            }
        }
    }
}
