use bevy::{color::palettes, prelude::*};

use bevy_easings::{Ease, *};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::default()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_easings::EasingsPlugin::new())
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
        let transform0 = Transform::default();
        let transform1 = Transform::from_translation(Vec3::new(500., 0., 0.));
        let transform2 = Transform::from_translation(Vec3::new(500., 300., 0.));
        let transform3 = Transform::from_translation(Vec3::new(-500., 300., 0.));
        let transform4 = Transform::from_translation(Vec3::new(-500., -300., 0.));
        let transform5 = Transform::from_translation(Vec3::new(500., -300., 0.));
        let transform6 = Transform::from_translation(Vec3::new(500., 0., 0.));
        let transform7 = Transform::default();

        let duration = std::time::Duration::from_millis(500);
        commands.entity(entity).insert(
            transform0
                .ease_to(
                    transform1,
                    bevy_easings::EaseFunction::QuadraticInOut,
                    bevy_easings::EasingType::Once { duration },
                )
                .ease_to(
                    transform2,
                    bevy_easings::EaseFunction::QuadraticInOut,
                    bevy_easings::EasingType::Once { duration },
                )
                .ease_to(
                    transform3,
                    bevy_easings::EaseFunction::QuadraticInOut,
                    bevy_easings::EasingType::Once { duration },
                )
                .ease_to(
                    transform4,
                    bevy_easings::EaseFunction::QuadraticInOut,
                    bevy_easings::EasingType::Once { duration },
                )
                .ease_to(
                    transform5,
                    bevy_easings::EaseFunction::QuadraticInOut,
                    bevy_easings::EasingType::Once { duration },
                )
                .ease_to(
                    transform6,
                    bevy_easings::EaseFunction::QuadraticInOut,
                    bevy_easings::EasingType::Once { duration },
                )
                .ease_to(
                    transform7,
                    bevy_easings::EaseFunction::QuadraticInOut,
                    bevy_easings::EasingType::Once { duration },
                ),
        );
    }
}
