use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use std::f32::consts::FRAC_PI_2;
use bevy::core_pipeline::bloom::Bloom;

#[derive(Component)]
pub struct FlyCamera {
    pub pitch: f32,
    pub yaw: f32,
    pub sensitivity: f32,
    pub speed: f32,
    pub velocity: Vec3,
    pub grounded: bool,
}

impl Default for FlyCamera {
    fn default() -> Self {
        Self {
            pitch: 0.0,
            yaw: 0.0,
            sensitivity: 0.002,
            speed: 10.0,
            velocity: Vec3::ZERO,
            grounded: true,
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera, grab_cursor))
            .add_systems(Update, (camera_movement, camera_look));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        FlyCamera::default(),
        Bloom::NATURAL, 
        Transform::from_xyz(0.0, 1.5, 5.0),
    ));
}

fn grab_cursor(mut window: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = window.single_mut() {
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
        window.cursor_options.visible = false;
    }
}

fn camera_look(
    mut motion: EventReader<bevy::input::mouse::MouseMotion>,
    mut query: Query<(&mut Transform, &mut FlyCamera)>,
) {
    let Ok((mut transform, mut camera)) = query.single_mut() else {
        return;
    };
    
    for ev in motion.read() {
        camera.yaw -= ev.delta.x * camera.sensitivity;
        camera.pitch -= ev.delta.y * camera.sensitivity;

        camera.pitch = camera.pitch.clamp(-FRAC_PI_2 + 0.01, FRAC_PI_2 - 0.01);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, camera.yaw, camera.pitch, 0.0);
    }
}

fn camera_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut FlyCamera)>,
) {
    let Ok((mut transform, mut camera)) = query.single_mut() else {
        return;
    };

    let dt = time.delta_secs();
    
    let mut direction = Vec3::ZERO;

    let forward = *transform.forward();
    let right = *transform.right();

    let forward_flat = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();
    let right_flat = Vec3::new(right.x, 0.0, right.z).normalize_or_zero();

    if keyboard.pressed(KeyCode::KeyW) {
        direction += forward_flat;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction -= forward_flat;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        direction -= right_flat;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction += right_flat;
    }

    if direction.length() > 0.0 {
        direction = direction.normalize();
    }

    transform.translation += direction * camera.speed * dt;

    let gravity = -25.0;

    if camera.grounded && keyboard.just_pressed(KeyCode::Space) {
        camera.velocity.y = 8.0;
        camera.grounded = false;
    }

    camera.velocity.y += gravity * dt;
    transform.translation.y += camera.velocity.y * dt;

    if transform.translation.y <= 1.5 {
        transform.translation.y = 1.5;
        camera.velocity.y = 0.0;
        camera.grounded = true;
    }
}
