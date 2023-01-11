use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::input::mouse::MouseMotion;
use std::f32::consts::{PI, TAU};
use bevy::window::CursorGrabMode;

pub const HEIGHT: f32 = 1080.0;
pub const WIDTH: f32 = 2560.0;

#[derive(Component,Reflect)]
pub struct Tower {
    shooting_timer: Timer,
}

#[derive(Component,Reflect)]
pub struct Lifetime {
    timer: Timer,
}


fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.3,0.1,0.2)))

    .add_startup_system(spawn_basic_scene)
    .add_startup_system(spawn_camera)
    .add_startup_system(grab_mouse)
    .add_system(tower_shooting)
    .add_system(bullet_despawn)
    .add_system(camera_movement)
    //.add_system(mouse_motion)
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            width: WIDTH,
            height: HEIGHT,
            title: "My Bevy Game".to_string(),
             resizable: false,
            ..default()
        },
        ..default()
    }))
    .add_plugin(WorldInspectorPlugin)
    .register_type::<Tower>()
    .register_type::<Lifetime>()
    .run();
}


fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,


) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {size: 5.0})),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    })
    .insert(Name::new("Ground"));
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube {size: 1.0})),
        material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
        transform: Transform::from_xyz(0.0,0.5,0.0),
        ..default()
    })
    .insert(Tower {
        shooting_timer: Timer::from_seconds(1.0, TimerMode::Repeating)
    })
    .insert(Name::new("Tower"));
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    })
    .insert(Name::new("Tower"));
}

fn tower_shooting(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut towers: Query<&mut Tower>,
    time: Res<Time>,
) {
    for mut tower in &mut towers {
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            let spawn_transform = 
                Transform::from_xyz(0.0,0.7,0.6).with_rotation(Quat::from_rotation_y(-PI / 2.0));
        
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube {size: 0.1})),
            material: materials.add(Color::rgb(0.87, 0.44, 0.42).into()),
            transform: spawn_transform,
            ..default()
        })
        .insert(Lifetime {
            timer: Timer::from_seconds(0.5, TimerMode::Repeating)
        })
        .insert(Name::new("Bullet"));
    }
    }
}

fn bullet_despawn(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime )in &mut bullets {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn camera_movement(
    windows: Res<Windows>,
    mut commands: Commands,
    mut keys: Res<Input<KeyCode>>,
    mut motion_evr: EventReader<MouseMotion>,
    mut cameras: Query<&mut Transform, With<Camera>>,
) {
    let mut rotation_move = Vec2::ZERO;
    for mut camera in &mut cameras {
        for ev in motion_evr.iter() {
            rotation_move += ev.delta;
            let window = get_primary_window_size(&windows);
            let delta_x = {
            let up = camera.rotation * Vec3::Y;
            let delta = rotation_move.x / window.x * std::f32::consts::PI * 2.0 * 0.1;
            if up.y <= 0.0 { -delta } else { delta }
            };
            let delta_y = rotation_move.y / window.y * std::f32::consts::PI * 0.1;
            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            camera.rotation = yaw * camera.rotation; // rotate around global y axis
            camera.rotation = camera.rotation * pitch; // rotate around local x axis
            println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);
        }
        if keys.pressed(KeyCode::D) {
            let right = camera.rotation * Vec3::X * 0.1;
            camera.translation += right;
            println!("Rotation: {:?}", camera.rotation.y);
        }
        if keys.pressed(KeyCode::A) {
            let right = camera.rotation * Vec3::X * -0.1;
            camera.translation += right;
        }
        if keys.pressed(KeyCode::W) {
            let forward = camera.rotation * Vec3::Z * -0.1;
            camera.translation += forward;
        }
        if keys.pressed(KeyCode::S) {
            let forward = camera.rotation * Vec3::Z * 0.1;
            camera.translation += forward;
        }
        if keys.pressed(KeyCode::Space) {
            let up = camera.rotation * Vec3::Y * 0.1;
            camera.translation += up;
        }
        if keys.pressed(KeyCode::LControl) {
            let up = camera.rotation * Vec3::Y * -0.1;
            camera.translation += up;
        }
    }
}

fn mouse_motion(
    mut motion_evr: EventReader<MouseMotion>,
) {
    for ev in motion_evr.iter() {
        println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);
    } 
}

fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();
    let window = Vec2::new(window.width() as f32, window.height() as f32);
    window
}

fn grab_mouse(mut windows: ResMut<Windows>,) {
    let window = windows.get_primary_mut().unwrap();
    window.set_cursor_grab_mode(CursorGrabMode::Locked);
    window.set_cursor_visibility(false);
}