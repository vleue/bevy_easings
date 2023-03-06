use bevy::prelude::*;

use bevy_easings::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::default()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_easings::EasingsPlugin)
        .add_startup_system(setup)
        .add_system(pause.in_schedule(CoreSchedule::FixedUpdate))
        .insert_resource(FixedTime::new_from_secs(0.25))
        .run();

    Ok(())
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(100., 100.)),
                color: Color::RED,
                ..Default::default()
            },
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(-500., 0., 0.)).ease_to(
            Transform::from_translation(Vec3::new(500., 0., 0.)),
            bevy_easings::EaseFunction::QuadraticInOut,
            bevy_easings::EasingType::PingPong {
                duration: std::time::Duration::from_millis(500),
                pause: Some(std::time::Duration::from_millis(100)),
            },
        ),
    ));
}

fn pause(mut query: Query<&mut bevy_easings::EasingComponent<Transform>>) {
    for mut easing in query.iter_mut() {
        easing.state = !easing.state;
    }
}
