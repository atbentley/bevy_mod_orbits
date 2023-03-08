use bevy::prelude::*;
use bevy_mod_orbits::prelude::*;

#[bevy_main]
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(OrbitPlugin)
        .add_startup_system(startup)
        .run();
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::ZERO, Vec3::NEG_Z),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(
            shape::Icosphere {
                radius: 0.45,
                subdivisions: 32,
            }
            .into(),
        ),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.7, 0.3, 0.3),
            unlit: true,
            ..default()
        }),
        ..default()
    });

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(
                shape::Icosphere {
                    radius: 0.45,
                    subdivisions: 32,
                }
                .into(),
            ),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.7, 0.3, 0.3),
                unlit: true,
                ..default()
            }),
            ..default()
        },
        Orbit {
            semi_major_axis: 4.0,
            eccentricity: 0.0,
            argument_of_periapsis: 0.0,
            initial_mean_anomaly: 0.0,
        },
    ));
}
