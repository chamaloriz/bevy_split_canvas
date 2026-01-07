use bevy::{camera::Viewport, prelude::*, window::WindowResolution};

use bevy_split_canvas::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn run() {
    // visibility: hidden; position: absolute; can be added to hide the main canvas
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.5, 0.5, 0.9)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("canvas".to_string()),
                resolution: WindowResolution::new(500, 500),
                ..default()
            }),
            ..default()
        }))
        // the plugin
        .add_plugins(MultiCanvasPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, cube_rotator_system)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let camera_configs = [
        ("First Camera", UVec2::new(0, 0), Vec3::new(-2.0, 2.5, 5.0)),
        (
            "Second Camera",
            UVec2::new(250, 0),
            Vec3::new(2.0, 2.5, 5.0),
        ),
        ("Third Camera", UVec2::new(0, 250), Vec3::new(0.0, 5.0, 0.1)),
        (
            "Fourth Camera",
            UVec2::new(250, 250),
            Vec3::new(5.0, 2.5, -2.0),
        ),
    ];

    for (index, (name, position, camera_pos)) in camera_configs.iter().enumerate() {
        let camera = commands
            .spawn((
                Camera3d::default(),
                // canvas
                RenderToCanvas {
                    canvas_id: format!("canvas{}", index + 1),
                },
                Camera {
                    viewport: Some(Viewport {
                        physical_position: *position,
                        physical_size: UVec2::new(250, 250),
                        ..default()
                    }),
                    order: -(4 - index as isize),
                    ..default()
                },
                Transform::from_translation(*camera_pos).looking_at(Vec3::ZERO, Vec3::Y),
            ))
            .id();

        commands
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(12.0),
                    left: Val::Px(12.0),
                    ..default()
                },
                UiTargetCamera(camera),
            ))
            .with_child((Text::new(*name), TextShadow::default()));
    }

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

fn cube_rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Mesh3d>>) {
    for mut transform in &mut query {
        transform.rotate_x(1.0 * time.delta_secs());
        transform.rotate_y(0.7 * time.delta_secs());
    }
}
