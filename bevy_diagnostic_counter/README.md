# bevy_diagnostic_counter

> :warning: **Now integrated into Bevy**, won't be updated here

Simple diagnostics that can count entities or assets.

```rust
App::default()
    .add_plugin(::bevy_diagnostic_counter::EntityCountDiagnosticsPlugin)
    .add_plugin(::bevy_diagnostic_counter::AssetCountDiagnosticsPlugin::<ColorMaterial>::default())
    .add_plugin(::bevy_diagnostic_counter::AssetCountDiagnosticsPlugin::<Texture>::default())
```

will log:
```
DEBUG bevy_log_diagnostic: diagnostic: entity_count                                            : 74.0000     (avg 74.0000)
DEBUG bevy_log_diagnostic: diagnostic: asset_count bevy_sprite::color_material::ColorMaterial  : 75.0000     (avg 75.0000)
DEBUG bevy_log_diagnostic: diagnostic: asset_count bevy_render::texture::texture::Texture      : 65.0000     (avg 65.0000)
```
