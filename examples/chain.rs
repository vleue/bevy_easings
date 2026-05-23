use bevy::{color::palettes, prelude::*};

use bevy_easings::{Ease, *};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::default()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_easings::EasingsPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(PostUpdate, add_easing)
        .run();

    Ok(())
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_color(palettes::basic::RED, Vec2::new(100., 100.)),
        Transform::from_scale(Vec3::ZERO)
            .ease_to(
                Transform::from_scale(Vec3::splat(2.0)),
                bevy_easings::EaseFunction::QuadraticInOut,
                bevy_easings::EasingType::Once {
                    duration: std::time::Duration::from_millis(500),
                },
            )
            .ease_to(
                Transform::from_scale(Vec3::ONE),
                bevy_easings::EaseFunction::QuadraticInOut,
                bevy_easings::EasingType::Once {
                    duration: std::time::Duration::from_millis(500),
                },
            ),
    ));
}

fn add_easing(
    mut commands: Commands,
    mut removed: RemovedComponents<EasingChainComponent<Transform>>,
) {
    for entity in removed.read() {
        let waypoints = [
            Transform::from_translation(Vec3::new(500., 0., 0.)),
            Transform::from_translation(Vec3::new(500., 300., 0.)),
            Transform::from_translation(Vec3::new(-500., 300., 0.)),
            Transform::from_translation(Vec3::new(-500., -300., 0.)),
            Transform::from_translation(Vec3::new(500., -300., 0.)),
            Transform::from_translation(Vec3::new(500., 0., 0.)),
            Transform::default(),
        ];

        let ease = bevy_easings::EaseFunction::QuadraticInOut;
        let easing_type = bevy_easings::EasingType::Once {
            duration: std::time::Duration::from_millis(500),
        };

        let mut iter = waypoints.into_iter();
        let first = iter.next().unwrap();
        let chain = iter.fold(
            Transform::default().ease_to(first, ease, easing_type).chain(),
            |acc, wp| acc.ease_to(wp, ease, easing_type),
        );

        commands.entity(entity).insert(chain);
    }
}
