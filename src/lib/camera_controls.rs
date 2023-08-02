use bevy::prelude::*;
use bevy::math::Vec4Swizzles;

use crate::GameSet;
use crate::world::ChunkLoader;
use crate::types::ChunkPos;

const SPEED: f32 = 1.0;
const FAST_SPEED: f32 = 20.0;
const ROTATION_SPEED: f32 = 2.0;

const RENDER_DISTANCE: UVec3 = UVec3::new(10, 5, 10);

#[derive(Component, Default)]
pub struct Controller {
    forward_pressed: bool,
    backward_pressed: bool,
    left_pressed: bool,
    right_pressed: bool,
    up_pressed: bool,
    down_pressed: bool,
    rotate_up_pressed: bool,
    rotate_down_pressed: bool,
    rotate_left_pressed: bool,
    rotate_right_pressed: bool,
    sprint_pressed: bool,
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle::default(),
        Controller::default(),
        ChunkLoader::new(ChunkPos::new(0, 0, 0), RENDER_DISTANCE),
    ));
}

fn handle_input(
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut Controller>,
) {
    for mut controller in query.iter_mut() {
        controller.forward_pressed = keys.pressed(KeyCode::W);
        controller.backward_pressed = keys.pressed(KeyCode::S);

        controller.left_pressed = keys.pressed(KeyCode::A);
        controller.right_pressed = keys.pressed(KeyCode::D);

        controller.up_pressed = keys.pressed(KeyCode::Space);
        controller.down_pressed = keys.pressed(KeyCode::AltLeft)
            || keys.pressed(KeyCode::AltRight);
        

        controller.rotate_up_pressed = keys.pressed(KeyCode::Up);
        controller.rotate_down_pressed = keys.pressed(KeyCode::Down);

        controller.rotate_left_pressed = keys.pressed(KeyCode::Left);
        controller.rotate_right_pressed = keys.pressed(KeyCode::Right);

        controller.sprint_pressed = keys.pressed(KeyCode::ShiftLeft)
            || keys.pressed(KeyCode::ShiftRight);
    }
}

fn move_camera(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Controller)>,
) {
    let delta = time.delta_seconds();
    let up = Vec3::Y;

    for (mut camera_transform, controller) in query.iter_mut() {
        // all these or from camera pov
        let camera_forward = camera_transform.forward();
        let camera_right = camera_transform.right();
        let camera_up = camera_transform.up();

        let camera_forward_norm = camera_forward.normalize();
        let camera_right_norm = camera_right.normalize();
        let camera_up_norm = camera_up.normalize();

        let distance_moved = if controller.sprint_pressed {
            FAST_SPEED
        } else {
            SPEED
        };

        // TODO: change so total speed is not increesed when going diagonal
        if controller.forward_pressed {
            camera_transform.translation += camera_forward_norm * distance_moved;
        }
        if controller.backward_pressed {
            camera_transform.translation -= camera_forward_norm * distance_moved;
        }

        if controller.left_pressed {
            camera_transform.translation -= camera_right_norm * distance_moved;
        }
        if controller.right_pressed {
            camera_transform.translation += camera_right_norm * distance_moved;
        }

        if controller.up_pressed {
            camera_transform.translation += camera_up_norm * distance_moved;
        }
        if controller.down_pressed {
            camera_transform.translation -= camera_up_norm * distance_moved;
        }

        let angle_rotated = delta * ROTATION_SPEED;

        let mut forward4 = Vec4::new(
            camera_forward.x,
            camera_forward.y,
            camera_forward.z,
            0.0,
        );

        if controller.rotate_up_pressed {
            let verticle_rotation = Mat4::from_axis_angle(camera_right_norm, angle_rotated);
            let forward_temp = verticle_rotation * forward4;
            if forward_temp.xyz().normalize().dot(up) < 0.98 {
                forward4 = forward_temp;
            }
        }
        if controller.rotate_down_pressed {
            let verticle_rotation = Mat4::from_axis_angle(camera_right_norm, -angle_rotated);
            let forward_temp = verticle_rotation * forward4;
            if forward_temp.xyz().normalize().dot(up) > -0.98 {
                forward4 = forward_temp;
            }
        }

        if controller.rotate_left_pressed {
            let horizantal_rotation = Mat4::from_axis_angle(up, angle_rotated);
            forward4 = horizantal_rotation * forward4;
        }
        if controller.rotate_right_pressed {
            let horizantal_rotation = Mat4::from_axis_angle(up, -angle_rotated);
            forward4 = horizantal_rotation * forward4;
        }

        let forward = forward4.xyz();

        *camera_transform = camera_transform.looking_at(camera_transform.translation + forward, up);
    }
}

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    handle_input,
                    move_camera,
                ).chain().in_set(GameSet::Main)
            );
    }
}