use bevy::prelude::*;

use bevy_easings::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::build()
        .add_default_plugins()
        .add_plugin(bevy_easings::EasingsPlugin)
        .add_startup_system(setup.system())
        .add_system(pause.system())
        .run();

    Ok(())
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dComponents::default());

    commands
        .spawn(SpriteComponents {
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
                    pause: std::time::Duration::from_millis(100),
                },
            ),
        )
        .with(Timer::from_seconds(0.25, true));
}

fn pause(timer: &Timer, mut easing: Mut<bevy_easings::EasingComponent<Transform>>) {
    if timer.just_finished {
        easing.state = !easing.state;
    }
}
