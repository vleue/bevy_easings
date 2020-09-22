# Bevy 9-Patch plugin

Implementation of 9-patch images in Bevy. Let you have a UI that scale only the right parts of your images.

![9 patch example](https://raw.githubusercontent.com/mockersf/bevy_extra/master/bevy_ninepatch/result.png)

See [the examples](https://github.com/mockersf/bevy_extra/tree/master/bevy_ninepatch/examples) for examples of what can be done.

## Usage

### Simple case

A simple builder based on Godot's [NinePatchRect](https://docs.godotengine.org/en/3.2/classes/class_ninepatchrect.html) is available.

```rust
    let panel_texture_handle = asset_server
        .load_sync(&mut textures, "assets/glassPanel_corners.png")
        .unwrap();

    commands
        .spawn(NodeComponents {
            style: Style {
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),

            ..Default::default()
        })
        .with_children(|global_parent| {
            bevy_ninepatch::NinePatchBuilder::by_margins(20., 20., 20., 20.)
                .apply(panel_texture_handle, &mut textures, &mut materials)
                .add(global_parent, 500., 300., |_, _| {});
        });
```

See [simple.rs example](https://github.com/mockersf/bevy_extra/blob/master/bevy_ninepatch/examples/simple.rs) for a complete example.

### Usage as a plugin

A plugin and a component bundle are exposed, and can be used to add 9-Patch UI elements.

See [plugin.rs example](https://github.com/mockersf/bevy_extra/blob/master/bevy_ninepatch/examples/plugin.rs) for a complete example.

### Specify content to use

You can specify the content to be used inside the 9-Patch UI element. When creating a 9-Patch by specifying the margins, a content zone will be available by default for the center of the 9-Patch UI element.

See [content.rs example](https://github.com/mockersf/bevy_extra/blob/master/bevy_ninepatch/examples/content.rs) for a complete example.

### More flexible definition

It is possible to set any number of patches for an image, the only constraints is that all patches in a line must have the same height. Using this methods, different parts of the image can grow at different rates, and several content zones can be created.

See [full.rs example](https://github.com/mockersf/bevy_extra/blob/master/bevy_ninepatch/examples/full.rs) for a complete example.
