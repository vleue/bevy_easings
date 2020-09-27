# Bevy Easings

Easings on Bevy components using [interpolation](https://crates.io/crates/interpolation).

Add the plugin to your app:
```rust
    App::build()
        .add_default_plugins()
        .add_plugin(bevy_easings::EasingsPlugin)
        ...
```

And then just ease your components to their new state!
```rust
commands
    .spawn(SpriteComponents {
        material: materials.add(Color::RED.into()),
        ..Default::default()
    })
    .with(
        Sprite {
            size: Vec2::new(10., 10.),
            ..Default::default()
        }
        .ease_to(
            Sprite {
                size: Vec2::new(100., 100.),
                ..Default::default()
            },
            EaseFunction::QuadraticIn,
            bevy_easings::AnimationType::PingPong {
                duration: std::time::Duration::from_secs(1),
                pause: std::time::Duration::from_millis(500),
            },
        ),
    );
```


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


## Examples

See [examples](https://github.com/mockersf/bevy_extra/tree/master/bevy_easings/examples)

![colormaterial_color](https://raw.githubusercontent.com/mockersf/bevy_extra/master/bevy_easings/examples/colormaterial_color.gif)

![sprite_size](https://raw.githubusercontent.com/mockersf/bevy_extra/master/bevy_easings/examples/sprite_size.gif)

![transform_rotation](https://raw.githubusercontent.com/mockersf/bevy_extra/master/bevy_easings/examples/transform_rotation.gif)

![transform_translation](https://raw.githubusercontent.com/mockersf/bevy_extra/master/bevy_easings/examples/transform_translation.gif)
