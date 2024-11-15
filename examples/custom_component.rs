use bevy::prelude::*;

use bevy_easings::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::default()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_easings::EasingsPlugin::new())
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, check_value)
        .insert_resource(Time::<Fixed>::from_seconds(0.2))
        .add_systems(Update, bevy_easings::custom_ease_system::<CustomComponent>)
        .run();

    Ok(())
}

#[derive(Default, Component, Clone)]
struct CustomComponent(f32);
impl bevy_easings::Lerp for CustomComponent {
    type Scalar = f32;

    fn lerp(&self, other: &Self, scalar: &Self::Scalar) -> Self {
        CustomComponent(interpolation::lerp(&self.0, &other.0, scalar))
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(
        CustomComponent(0.)
            .ease_to(
                CustomComponent(100.),
                bevy_easings::EaseFunction::QuadraticInOut,
                bevy_easings::EasingType::PingPong {
                    duration: std::time::Duration::from_secs(1),
                    pause: Some(std::time::Duration::from_millis(500)),
                },
            )
            // as `CustomComponent` is not already part of the components of the entity,
            // we can either insert the component with a basic value, it will be replaced immediately,
            // or call `with_original_value` if the `CustomComponent` implements `Clone`
            .with_original_value(),
    );
}

fn check_value(mut query: Query<&CustomComponent, Changed<CustomComponent>>) {
    for custom in query.iter_mut() {
        println!("Change detected: {:?}", custom.0);
    }
}
