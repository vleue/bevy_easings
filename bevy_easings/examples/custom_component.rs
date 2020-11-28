use bevy::prelude::*;

use bevy_easings::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_easings::EasingsPlugin)
        .add_startup_system(setup.system())
        .add_system(check_value.system())
        .add_system(bevy_easings::custom_ease_system::<CustomComponent>.system())
        .run();

    Ok(())
}

#[derive(Default)]
struct CustomComponent(f32);
impl bevy_easings::Lerp for CustomComponent {
    type Scalar = f32;

    fn lerp(&self, other: &Self, scalar: &Self::Scalar) -> Self {
        CustomComponent(self.0.lerp(&other.0, scalar))
    }
}

fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(UiCameraBundle::default());

    commands
        .spawn(ImageBundle {
            material: materials.add(Color::RED.into()),
            style: Style {
                size: Size {
                    width: Val::Percent(3.),
                    height: Val::Percent(3.),
                },

                margin: Rect {
                    bottom: Val::Percent(0.),
                    left: Val::Px(3.),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        // as `CustomComponent` is not already part of the components of the entity,
        // insert the component with a basic value, it will be replaced immediately
        .with(CustomComponent(-1.))
        .with(CustomComponent(0.).ease_to(
            CustomComponent(100.),
            bevy_easings::EaseFunction::QuadraticInOut,
            bevy_easings::EasingType::PingPong {
                duration: std::time::Duration::from_secs(1),
                pause: Some(std::time::Duration::from_millis(500)),
            },
        ))
        .with(Timer::from_seconds(0.2, true));
}

fn check_value(query: Query<(&Timer, &CustomComponent)>) {
    for (timer, custom) in query.iter() {
        if timer.just_finished() {
            println!("got {:?}", custom.0);
        }
    }
}
