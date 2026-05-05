use crate::player::PLAYER_SPEED;
use crate::plugins::{input::InputActionState, player::Player};
use bevy::prelude::*;

pub fn fixed_update(
    input_state: Res<InputActionState>,
    mut transform: Single<&mut Transform, With<Player>>,
) {
    transform.translation.x += input_state.move_axis.x * PLAYER_SPEED;
    transform.translation.y += input_state.move_axis.y * PLAYER_SPEED;
}
