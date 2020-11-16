use bevy::prelude::*;

use bevy_easings::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_easings::EasingsPlugin)
        .add_startup_system(setup.system())
        .add_system(pause.system())
        .run();

    Ok(())
}

fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(SpriteBundle {
            material: materials.add(Color::RED.into()),
            sprite: Sprite {
                size: Vec2::new(100., 100.),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(
            Transform::from_translation(Vec3::new(-500., 0., 0.)).ease_to(
                Transform::from_translation(Vec3::new(500., 0., 0.)),
                bevy_easings::EaseFunction::QuadraticInOut,
                bevy_easings::EasingType::PingPong {
                    duration: std::time::Duration::from_millis(500),
                    pause: Some(std::time::Duration::from_millis(100)),
                },
            ),
        )
        .with(Timer::from_seconds(0.25, true));
}

fn pause(mut query: Query<(&Timer, &mut bevy_easings::EasingComponent<Transform>)>) {
    for (timer, mut easing) in query.iter_mut() {
        if timer.just_finished {
            easing.state = !easing.state;
        }
    }
}
