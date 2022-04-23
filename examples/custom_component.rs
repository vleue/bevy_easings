use bevy::{core::FixedTimestep, prelude::*};

use bevy_easings::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::default()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_easings::EasingsPlugin)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.2))
                .with_system(check_value),
        )
        .add_system(bevy_easings::custom_ease_system::<CustomComponent>)
        .run();

    Ok(())
}

#[derive(Default, Component)]
struct CustomComponent(f32);
impl bevy_easings::Lerp for CustomComponent {
    type Scalar = f32;

    fn lerp(&self, other: &Self, scalar: &Self::Scalar) -> Self {
        CustomComponent(interpolation::lerp(&self.0, &other.0, scalar))
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(ImageBundle {
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
            color: UiColor(Color::RED),
            ..Default::default()
        })
        // as `CustomComponent` is not already part of the components of the entity,
        // insert the component with a basic value, it will be replaced immediately
        .insert(CustomComponent(-1.))
        .insert(CustomComponent(0.).ease_to(
            CustomComponent(100.),
            bevy_easings::EaseFunction::QuadraticInOut,
            bevy_easings::EasingType::PingPong {
                duration: std::time::Duration::from_secs(1),
                pause: Some(std::time::Duration::from_millis(500)),
            },
        ));
}

fn check_value(mut query: Query<&CustomComponent>) {
    for custom in query.iter_mut() {
        println!("got {:?}", custom.0);
    }
}
