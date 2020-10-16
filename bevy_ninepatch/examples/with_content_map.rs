use bevy::prelude::*;

use bevy_ninepatch::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::build()
        .add_default_plugins()
        // Add the `NinePatchPlugin` plugin
        .add_plugin(NinePatchPlugin::<()>::default())
        .add_startup_system(setup.system())
        .run();

    Ok(())
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut nine_patches: ResMut<Assets<NinePatchBuilder<()>>>,
) {
    commands.spawn(UiCameraComponents::default());

    // prepare the button
    let button_texture_handle = asset_server.load("assets/blue_button02.png").unwrap();
    let button_nine_patch_handle = nine_patches.add(NinePatchBuilder::by_margins(5, 10, 6, 6, ()));

    let font = asset_server
        .load("assets/Kenney Future Narrow.ttf")
        .expect("was able to load font");

    let text_cancel = commands
        .spawn(TextComponents {
            style: Style {
                margin: Rect {
                    left: Val::Px(50.),
                    right: Val::Auto,
                    top: Val::Auto,
                    bottom: Val::Px(10.),
                },
                ..Default::default()
            },
            text: Text {
                value: "Cancel".to_string(),
                font,
                style: TextStyle {
                    font_size: 50.,
                    color: Color::RED,
                },
            },
            ..Default::default()
        })
        .current_entity()
        .unwrap();
    let mut cancel_content = std::collections::HashMap::new();
    cancel_content.insert((), text_cancel);

    commands.spawn(
        // this component bundle will be detected by the plugin, and the 9-Patch UI element will be added as a child
        // of this entity
        NinePatchComponents {
            style: Style {
                margin: Rect {
                    left: Val::Px(0.),
                    right: Val::Auto,
                    top: Val::Auto,
                    bottom: Val::Px(0.),
                },
                size: Size::new(Val::Px(300.), Val::Px(80.)),

                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            nine_patch_data: NinePatchData {
                nine_patch: button_nine_patch_handle,
                texture: button_texture_handle,
                content: cancel_content,
                ..Default::default()
            },
            ..Default::default()
        },
    );

    let text_ok = commands
        .spawn(TextComponents {
            style: Style {
                margin: Rect {
                    left: Val::Px(50.),
                    right: Val::Auto,
                    top: Val::Auto,
                    bottom: Val::Px(10.),
                },
                ..Default::default()
            },
            text: Text {
                value: "OK".to_string(),
                font,
                style: TextStyle {
                    font_size: 50.,
                    color: Color::GREEN,
                },
            },
            ..Default::default()
        })
        .current_entity()
        .unwrap();
    let mut ok_content = std::collections::HashMap::new();
    ok_content.insert((), text_ok);

    commands.spawn(
        // this component bundle will be detected by the plugin, and the 9-Patch UI element will be added as a child
        // of this entity
        NinePatchComponents {
            style: Style {
                margin: Rect {
                    left: Val::Px(0.),
                    right: Val::Auto,
                    top: Val::Auto,
                    bottom: Val::Px(0.),
                },
                size: Size::new(Val::Px(300.), Val::Px(80.)),

                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            nine_patch_data: NinePatchData {
                nine_patch: button_nine_patch_handle,
                texture: button_texture_handle,
                content: ok_content,
                ..Default::default()
            },
            ..Default::default()
        },
    );
}
