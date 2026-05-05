use bevy::prelude::*;

const LEFT_KEYS: [KeyCode; 3] = [KeyCode::KeyA, KeyCode::ArrowLeft, KeyCode::KeyJ];
const RIGHT_KEYS: [KeyCode; 3] = [KeyCode::KeyD, KeyCode::ArrowRight, KeyCode::KeyL];

const UP_KEYS: [KeyCode; 3] = [KeyCode::KeyW, KeyCode::ArrowUp, KeyCode::KeyI];
const DOWN_KEYS: [KeyCode; 3] = [KeyCode::KeyS, KeyCode::ArrowDown, KeyCode::KeyK];

pub fn update(keys: Res<ButtonInput<KeyCode>>, mut input_state: ResMut<super::InputActionState>) {
    // TODO: Handle other forms of input (gamepads?, mouse?, mobile touch? )

    let left_pressed = LEFT_KEYS.iter().any(|key| keys.pressed(*key));
    let right_pressed = RIGHT_KEYS.iter().any(|key| keys.pressed(*key));
    let up_pressed = UP_KEYS.iter().any(|key| keys.pressed(*key));
    let down_pressed = DOWN_KEYS.iter().any(|key| keys.pressed(*key));

    if left_pressed == right_pressed {
        input_state.move_axis.x = 0.;
    } else if left_pressed {
        input_state.move_axis.x = -1.0;
    } else {
        input_state.move_axis.x = 1.0;
    }

    if up_pressed == down_pressed {
        input_state.move_axis.y = 0.;
    } else if up_pressed {
        input_state.move_axis.y = 1.0;
    } else {
        input_state.move_axis.y = -1.0;
    }

    let _ = input_state.move_axis.normalize();
}
