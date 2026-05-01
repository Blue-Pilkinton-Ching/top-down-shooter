use crate::plugins::{player::Player, util::PreviousTranslation};

use bevy::prelude::*;

pub fn startup(mut commands: Commands) {
<<<<<<< HEAD
    // cube
    commands.spawn(Transform::default());
=======
    commands.spawn((Transform::default(), Player, PreviousTranslation::default()));
    commands.spawn((Transform::default(), Camera3d::default()));
>>>>>>> 137afdf (wip)
}

// TODO: Lerp camera position between the players previous and current position using
