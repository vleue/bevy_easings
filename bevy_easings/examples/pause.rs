use bevy::prelude::*;

use bevy_easings::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::default()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_easings::EasingsPlugin)
        .add_startup_system(setup)
        .add_system(pause)
        .run();

    Ok(())
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(100., 100.)),
                color: Color::RED,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(
            Transform::from_translation(Vec3::new(-500., 0., 0.)).ease_to(
                Transform::from_translation(Vec3::new(500., 0., 0.)),
                bevy_easings::EaseFunction::QuadraticInOut,
                bevy_easings::EasingType::PingPong {
                    duration: std::time::Duration::from_millis(500),
                    pause: Some(std::time::Duration::from_millis(100)),
                },
            ),
        )
        .insert(Timer::from_seconds(0.25, true));
}

fn pause(
    mut query: Query<(&mut Timer, &mut bevy_easings::EasingComponent<Transform>)>,
    time: Res<Time>,
) {
    for (mut timer, mut easing) in query.iter_mut() {
        if timer.tick(time.delta()).just_finished() {
            easing.state = !easing.state;
        }
    }
}
