use crate::plugins::{player::Player, util::PreviousTranslation};

use bevy::prelude::*;

pub fn startup(mut commands: Commands) {
    // cube
    commands.spawn((Transform::default(), Player, PreviousTranslation::default()));
    commands.spawn(Camera2d);
}
