# Bevy 9-Patch plugin

Implementation of 9-patch images in Bevy. Let you have a UI that scale only the right parts of your images.

![9 patch example](https://raw.githubusercontent.com/mockersf/bevy_extra/master/bevy_ninepatch/result.png)

See [the examples](https://github.com/mockersf/bevy_extra/tree/master/bevy_ninepatch/examples) for what can be done.

## Simple usage

After adding the `NinePatchPlugin` plugin, spawning an `Entity` with the `NinePatchComponents` component bundle will add a 9-patch UI element.

A simple builder based on Godot's [NinePatchRect](https://docs.godotengine.org/en/3.2/classes/class_ninepatchrect.html) is available.

```rust
let panel_texture_handle = asset_server
    .load_sync(&mut textures, "assets/glassPanel_corners.png")
    .unwrap();

let nine_patch_handle = nine_patches.add(NinePatchBuilder::by_margins(20., 20., 20., 20., ()));


commands.spawn(
    NinePatchComponents {
        style: Style {
            margin: Rect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        nine_patch_data: NinePatchData {
            nine_patch: nine_patch_handle,
            texture: panel_texture_handle,
            ..Default::default()
        },
        nine_patch_size: NinePatchSize(Vec2::new(500., 300.)),
        ..Default::default()
    },
);
```

See [plugin.rs example](https://github.com/mockersf/bevy_extra/blob/master/bevy_ninepatch/examples/plugin.rs) for a complete example.

## Changing element size

The component `NinePatchSize` can be changed to update the size of the 9-Patch UI element. Changing this component must happen during the `UPDATE` stage and may not work otherwise.

See [change_size.rs example](https://github.com/mockersf/bevy_extra/blob/master/bevy_ninepatch/examples/change_size.rs) for a complete example.

## Specify content to use

You can specify the content to be used inside the 9-Patch UI element. When creating a 9-Patch by specifying the margins, a content zone will be available by default for the center of the 9-Patch UI element. This can be set with the `NinePatchContent` component.

See [multi_content.rs example](https://github.com/mockersf/bevy_extra/blob/master/bevy_ninepatch/examples/content.rs) for a complete example.

## More flexible definition

It is possible to set any number of patches for an image, the only constraints is that all patches in a line must have the same height. Using this methods, different parts of the image can grow at different rates, and several content zones can be created.

See [full.rs example](https://github.com/mockersf/bevy_extra/blob/master/bevy_ninepatch/examples/full.rs) for a complete example.

## Usage without a plugin

9-Patch UI elements can be added without using a plugin if needed. Without the plugin, 9-Path UI elements can still be created, but changing size won't work.

See [no_plugin_simple.rs example](https://github.com/mockersf/bevy_extra/blob/master/bevy_ninepatch/examples/no_plugin_simple.rs) for a simple example, or [the other examples without plugin](https://github.com/mockersf/bevy_extra/tree/master/bevy_ninepatch/examples#without-plugin).

