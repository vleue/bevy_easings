use bevy::prelude::*;

use bevy_easings::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::default()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_easings::EasingsPlugin)
        .add_startup_system(setup)
        .add_system(add_easing)
        .run();

    Ok(())
}

#[derive(Component)]
struct ChainTimer(Timer);

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(100., 100.)),
                color: Color::RED,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ChainTimer(Timer::from_seconds(1., false)));
}

fn add_easing(
    mut commands: Commands,
    mut query: Query<(&mut ChainTimer, Entity)>,
    time: Res<Time>,
) {
    for (mut timer, entity) in query.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            let transform0 = Transform::default();
            let transform1 = Transform::from_translation(Vec3::new(500., 0., 0.));
            let transform2 = Transform::from_translation(Vec3::new(500., 300., 0.));
            let transform3 = Transform::from_translation(Vec3::new(-500., 300., 0.));
            let transform4 = Transform::from_translation(Vec3::new(-500., -300., 0.));
            let transform5 = Transform::from_translation(Vec3::new(500., -300., 0.));
            let transform6 = Transform::from_translation(Vec3::new(500., 0., 0.));
            let transform7 = Transform::default();

            let duration = std::time::Duration::from_millis(500);
            commands
                .entity(entity)
                .insert(ChainTimer(Timer::from_seconds(
                    7. * duration.as_secs_f32() as f32 + 1.,
                    false,
                )));
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
}
