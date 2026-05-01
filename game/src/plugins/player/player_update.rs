use bevy::prelude::*;

use crate::plugins::{input::InputActionState, player::Player};

pub fn update(input_state: Res<InputActionState>, mut player: Single<(&Player, &mut Transform)>) {
    player.1.translation.x += input_state.move_axis.x;
    player.1.translation.y += input_state.move_axis.y;
}
