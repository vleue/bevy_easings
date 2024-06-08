use bevy::prelude::*;

use bevy_easings::*;
use rand::Rng;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::default()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_easings::EasingsPlugin)
        .add_systems(Startup, setup)
        .add_systems(PostUpdate, add_new_easing)
        .run();

    Ok(())
}

fn setup(mut commands: Commands, windows: Query<&Window>) {
    commands.spawn(Camera2dBundle::default());

    let window = windows.single();

    let width = window.width() / 2.0;
    let height = window.height() / 2.0;
    let x = rand::thread_rng().gen_range(-width..width);
    let y = rand::thread_rng().gen_range(-height..height);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(100., 100.)),
                color: Color::RED,
                ..Default::default()
            },
            ..Default::default()
        },
        Transform::IDENTITY.ease_to(
            Transform::from_xyz(x, y, 0.0),
            bevy_easings::EaseFunction::QuadraticInOut,
            bevy_easings::EasingType::Once {
                duration: std::time::Duration::from_millis(2500),
            },
        ),
    ));
}

fn add_new_easing(
    mut commands: Commands,
    mut removed: RemovedComponents<EasingComponent<Transform>>,
    transform: Query<&Transform>,
    windows: Query<&Window>,
) {
    let window = windows.single();

    for entity in removed.read() {
        let width = window.width() / 2.0;
        let height = window.height() / 2.0;
        let x = rand::thread_rng().gen_range(-width..width);
        let y = rand::thread_rng().gen_range(-height..height);

        commands
            .entity(entity)
            .insert(transform.get(entity).unwrap().ease_to(
                Transform::from_xyz(x, y, 0.0),
                bevy_easings::EaseFunction::QuadraticInOut,
                bevy_easings::EasingType::Once {
                    duration: std::time::Duration::from_millis(2500),
                },
            ));
    }
}
