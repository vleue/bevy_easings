# Bevy Easings

![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)
[![Doc](https://docs.rs/bevy_easings/badge.svg)](https://docs.rs/bevy_easings)
[![Crate](https://img.shields.io/crates/v/bevy_easings.svg)](https://crates.io/crates/bevy_easings)
[![Bevy Tracking](https://img.shields.io/badge/Bevy%20tracking-main-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)
[![CI](https://github.com/vleue/bevy_easings/actions/workflows/ci.yml/badge.svg)](https://github.com/vleue/bevy_easings/actions/workflows/ci.yml)

Easings on Bevy components using [interpolation](https://crates.io/crates/interpolation).

![menu example](https://raw.githubusercontent.com/vleue/bevy_easings/main/examples/menu.webp)

## Usage

### System setup

Add the plugin to your app:

```rust
use bevy::prelude::*;
use bevy_easings::EasingsPlugin;

fn main() {
    App::new()
        .add_plugins(EasingsPlugin);
}
```

### Easing a component to a new value

And then just ease your components to their new state!

```rust
use bevy::prelude::*;
use bevy_easings::Ease;

fn my_system(mut commands: Commands){
    commands
        .spawn((
            Sprite {
                ..Default::default()
            },
            Sprite {
                custom_size: Some(Vec2::new(10., 10.)),
                ..Default::default()
            }
            .ease_to(
                Sprite {
                    custom_size: Some(Vec2::new(100., 100.)),
                    ..Default::default()
                },
                bevy_easings::EaseFunction::QuadraticIn,
                bevy_easings::EasingType::PingPong {
                    duration: std::time::Duration::from_secs(1),
                    pause: Some(std::time::Duration::from_millis(500)),
                },
            ),
        ));
}
```

If the component being eased is not already a component of the entity, the component should first be inserted for the target entity.

### Easing using EaseMethod

The EaseMethod enum can be used to provide easing methods that are not avaliable in EaseFunction.

```rust,ignore
pub enum EaseMethod {
    /// Follow `EaseFunction`
    EaseFunction(EaseFunction),
    /// Linear interpolation, with no function
    Linear,
    /// Discrete interpolation, eased value will jump from start to end
    Discrete,
    /// Use a custom function to interpolate the value
    CustomFunction(fn(f32) -> f32),
}
```

This is shown below

```rust
use bevy::prelude::*;
use bevy_easings::Ease;

fn my_system(mut commands: Commands){
    commands
        .spawn((
            Sprite {
                ..Default::default()
            },
            Sprite {
                custom_size: Some(Vec2::new(10., 10.)),
                ..Default::default()
            }
            .ease_to(
                Sprite {
                    custom_size: Some(Vec2::new(100., 100.)),
                    ..Default::default()
                },
                bevy_easings::EaseMethod::Linear,
                bevy_easings::EasingType::PingPong {
                    duration: std::time::Duration::from_secs(1),
                    pause: Some(std::time::Duration::from_millis(500)),
                },
            ),
        ));
}
```

### Chaining easing

You can chain easings, if they are not set to repeat they will happen in sequence.

```rust
use bevy::prelude::*;
use bevy_easings::Ease;

fn my_system(mut commands: Commands){
    commands
        .spawn((
            Sprite {
                ..Default::default()
            },
            Sprite {
                custom_size: Some(Vec2::new(10., 10.)),
                ..Default::default()
            }
            .ease_to(
                Sprite {
                    custom_size: Some(Vec2::new(300., 300.)),
                    ..Default::default()
                },
                bevy_easings::EaseFunction::QuadraticIn,
                bevy_easings::EasingType::Once {
                    duration: std::time::Duration::from_secs(1),
                },
            )
            .ease_to(
                Sprite {
                    custom_size: Some(Vec2::new(350., 350.)),
                    ..Default::default()
                },
                bevy_easings::EaseFunction::QuadraticIn,
                bevy_easings::EasingType::PingPong {
                    duration: std::time::Duration::from_millis(500),
                    pause: Some(std::time::Duration::from_millis(200)),
                },
            ),
        ));
}
```

## Custom component support

To be able to ease a component, it needs to implement the traits `Default` and [`Lerp`](https://docs.rs/interpolation/0.2.0/interpolation/trait.Lerp.html). This trait is re-exported by `beavy_easings`.

```rust
use bevy::prelude::*;
use bevy_easings::*;

#[derive(Default, Component)]
struct CustomComponent(f32);
impl Lerp for CustomComponent {
    type Scalar = f32;

    fn lerp(&self, other: &Self, scalar: &Self::Scalar) -> Self {
        CustomComponent(interpolation::lerp(&self.0, &other.0, scalar))
    }
}
```

The basic formula for lerp (linear interpolation) is `self + (other - self) * scalar`.

Then, the system `custom_ease_system::<CustomComponent>` needs to be added to the application.

## Examples

See [examples](https://github.com/vleue/bevy_easings/tree/main/examples)

### Choosing the ease function

![easing on size](https://raw.githubusercontent.com/vleue/bevy_easings/main/examples/lerping-sizes.webp)

![easing on color](https://raw.githubusercontent.com/vleue/bevy_easings/main/examples/lerping-color.webp)

When easing colors, pay attention on the color space you are using

![color spaces](https://raw.githubusercontent.com/vleue/bevy_easings/main/examples/lerping-color-spaces.webp)

## Ease Functions

Many [ease functions](https://docs.rs/interpolation/0.2.0/interpolation/enum.EaseFunction.html) are available:

- QuadraticIn
- QuadraticOut
- QuadraticInOut
- CubicIn
- CubicOut
- CubicInOut
- QuarticIn
- QuarticOut
- QuarticInOut
- QuinticIn
- QuinticOut
- QuinticInOut
- SineIn
- SineOut
- SineInOut
- CircularIn
- CircularOut
- CircularInOut
- ExponentialIn
- ExponentialOut
- ExponentialInOut
- ElasticIn
- ElasticOut
- ElasticInOut
- BackIn
- BackOut
- BackInOut
- BounceIn
- BounceOut
- BounceInOut

## Bevy supported version

|Bevy|bevy_easings|
|---|---|
|main|main|
|0.15|0.15|
|0.14|0.14|
|0.13|0.14|
|0.12|0.12|
|0.11|0.11|
|0.10|0.10|
|0.9|0.9|
|0.8|0.8|
|0.7|0.7|
|0.6|0.6|
|0.5|0.4|
