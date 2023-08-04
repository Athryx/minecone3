use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::math::Vec4Swizzles;
use bevy::window::CursorGrabMode;

use crate::GameSet;
use crate::world::ChunkLoader;
use crate::types::ChunkPos;

const SPEED: f32 = 1.0;
const FAST_SPEED: f32 = 20.0;
const ROTATION_SPEED: f32 = 2.0;
const MOUSE_ROTATION_SPEED: f32 = 0.001;

#[derive(Component, Default)]
pub struct Controller {
    forward_pressed: bool,
    backward_pressed: bool,
    left_pressed: bool,
    right_pressed: bool,
    up_pressed: bool,
    down_pressed: bool,
    horizantal_rotation: f32,
    verticle_rotation: f32,
    sprint_pressed: bool,
}

fn handle_keyboard_input(
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

        controller.sprint_pressed = keys.pressed(KeyCode::ShiftLeft)
            || keys.pressed(KeyCode::ShiftRight);
    }
}

fn handle_mouse_input(
    mut mouse_motion: EventReader<MouseMotion>,
    mut controllers: Query<&mut Controller>,
) {
    let mut mouse_delta = Vec2::ZERO;

    for motion_event in mouse_motion.iter() {
        mouse_delta += motion_event.delta;
    }

    mouse_delta *= MOUSE_ROTATION_SPEED;

    for mut controller in controllers.iter_mut() {
        controller.horizantal_rotation = mouse_delta.x;
        controller.verticle_rotation = mouse_delta.y;
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

        let mut forward4 = Vec4::new(
            camera_forward.x,
            camera_forward.y,
            camera_forward.z,
            0.0,
        );

        let verticle_rotation = Mat4::from_axis_angle(camera_right_norm, -controller.verticle_rotation);
        let forward_temp = verticle_rotation * forward4;

        // stop camera from rotating all the way around top or bottom
        if forward_temp.xyz().normalize().dot(up).abs() < 0.98 {
            forward4 = forward_temp;
        }

        let horizantal_rotation = Mat4::from_axis_angle(up, -controller.horizantal_rotation);
        forward4 = horizantal_rotation * forward4;

        let forward = forward4.xyz();

        *camera_transform = camera_transform.looking_at(camera_transform.translation + forward, up);
    }
}

fn grab_mouse(mut query: Query<&mut Window>) {
    let mut window = query.single_mut();

    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;
}

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, grab_mouse)
            .add_systems(
                Update,
                (
                    (handle_keyboard_input, handle_mouse_input),
                    move_camera,
                ).chain().in_set(GameSet::Main)
            );
    }
}