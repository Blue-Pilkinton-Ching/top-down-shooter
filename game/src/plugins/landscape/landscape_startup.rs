use bevy::prelude::*;

pub fn startup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    // circular base
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(4.0))),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
}
