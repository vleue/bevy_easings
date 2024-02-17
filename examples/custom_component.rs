use bevy::prelude::*;

use bevy_easings::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::default()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_easings::EasingsPlugin)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, check_value)
        .insert_resource(Time::<Fixed>::from_seconds(0.2))
        .add_systems(Update, bevy_easings::custom_ease_system::<CustomComponent>)
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
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        ImageBundle {
            style: Style {
                width: Val::Percent(3.),
                height: Val::Percent(3.),

                margin: UiRect {
                    bottom: Val::Percent(0.),
                    left: Val::Px(3.),
                    ..Default::default()
                },
                ..Default::default()
            },
            background_color: BackgroundColor(Color::RED),
            ..Default::default()
        },
        // as `CustomComponent` is not already part of the components of the entity,
        // insert the component with a basic value, it will be replaced immediately
        CustomComponent(-1.),
        CustomComponent(0.).ease_to(
            CustomComponent(100.),
            bevy_easings::EaseFunction::QuadraticInOut,
            bevy_easings::EasingType::PingPong {
                duration: std::time::Duration::from_secs(1),
                pause: Some(std::time::Duration::from_millis(500)),
            },
        ),
    ));
}

fn check_value(mut query: Query<&CustomComponent, Changed<CustomComponent>>) {
    for custom in query.iter_mut() {
        println!("Change detected: {:?}", custom.0);
    }
}
