use bevy::prelude::*;

use bevy_easings::*;
use rand::Rng;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::default()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_easings::EasingsPlugin)
        .add_startup_system(setup)
        .add_system_to_stage(CoreStage::PostUpdate, add_new_easing)
        .run();

    Ok(())
}

fn setup(mut commands: Commands, windows: Res<Windows>) {
    commands.spawn_bundle(Camera2dBundle::default());

    let width = windows.primary().width() / 2.0;
    let height = windows.primary().height() / 2.0;
    let x = rand::thread_rng().gen_range(-width..width);
    let y = rand::thread_rng().gen_range(-height..height);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(100., 100.)),
                color: Color::RED,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Transform::identity().ease_to(
            Transform::from_xyz(x, y, 0.0),
            bevy_easings::EaseFunction::QuadraticInOut,
            bevy_easings::EasingType::Once {
                duration: std::time::Duration::from_millis(2500),
            },
        ));
}

fn add_new_easing(
    mut commands: Commands,
    removed: RemovedComponents<EasingComponent<Transform>>,
    transform: Query<&Transform>,
    windows: Res<Windows>,
) {
    for entity in removed.iter() {
        let width = windows.primary().width() / 2.0;
        let height = windows.primary().height() / 2.0;
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
