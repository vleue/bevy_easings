# Examples

## Working on `SpriteComponents`

### [colormaterial_color.rs](https://github.com/mockersf/bevy_extra/blob/master/bevy_easings/examples/colormaterial_color.rs)

```rust
materials.add(Color::RED.into()).ease_to(
    materials.add(Color::BLUE.into()),
    *ease_function,
    bevy_easings::AnimationType::PingPong {
        duration: std::time::Duration::from_secs(1),
        pause: std::time::Duration::from_millis(500),
    },
)
```

![colormaterial_color](https://raw.githubusercontent.com/mockersf/bevy_extra/master/bevy_easings/examples/colormaterial_color.gif)

### [sprite_size.rs](https://github.com/mockersf/bevy_extra/blob/master/bevy_easings/examples/sprite_size.rs)

```rust
Sprite {
    size: Vec2::new(initial_size, initial_size),
    ..Default::default()
}
.ease_to(
    Sprite {
        size: Vec2::new(final_size, final_size),
        ..Default::default()
    },
    *ease_function,
    bevy_easings::AnimationType::PingPong {
        duration: std::time::Duration::from_secs(1),
        pause: std::time::Duration::from_millis(500),
    },
)
```

![sprite_size](https://raw.githubusercontent.com/mockersf/bevy_extra/master/bevy_easings/examples/sprite_size.gif)

### [transform_rotation.rs](https://github.com/mockersf/bevy_extra/blob/master/bevy_easings/examples/transform_rotation.rs)

```rust
Transform::default().ease_to(
    Transform::default().with_rotation(Quat::from_rotation_ypr(0., 0., 3.1415 / 2.)),
    *ease_function,
    bevy_easings::AnimationType::PingPong {
        duration: std::time::Duration::from_secs(1),
        pause: std::time::Duration::from_millis(500),
    },
)
```

![transform_rotation](https://raw.githubusercontent.com/mockersf/bevy_extra/master/bevy_easings/examples/transform_rotation.gif)

### [transform_translation.rs](https://github.com/mockersf/bevy_extra/blob/master/bevy_easings/examples/transform_translation.rs)

```rust
Transform::default()
    .with_translate(Vec3::new(x, screen_y, 0.))
    .ease_to(
        Transform::default().with_translate(Vec3::new(x, -screen_y, 0.)),
        *ease_function,
        bevy_easings::AnimationType::PingPong {
            duration: std::time::Duration::from_secs(1),
            pause: std::time::Duration::from_millis(500),
        },
    ),
```

![transform_translation](https://raw.githubusercontent.com/mockersf/bevy_extra/master/bevy_easings/examples/transform_translation.gif)
