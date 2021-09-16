use bevy::prelude::*;
struct OrdersPlugin;

impl Plugin for OrdersPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(movement_system.system());
    }
}

fn movement_system(_commands: Commands) {}
