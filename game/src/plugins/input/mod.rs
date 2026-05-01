use bevy::prelude::*;

mod input_update;

pub struct InputPlugin;

#[derive(Resource, Default)]
pub struct InputActionState {
    pub move_axis: Vec2,
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup)
            .add_systems(Update, input_update::update);
    }
}

fn startup(mut commands: Commands) {
    commands.init_resource::<InputActionState>();
}
