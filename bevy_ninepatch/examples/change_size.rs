use bevy::prelude::*;

use bevy_ninepatch::{NinePatchBuilder, NinePatchComponents, NinePatchData, NinePatchPlugin};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::build()
        .add_default_plugins()
        // Add the `NinePatchPlugin` plugin
        .add_plugin(NinePatchPlugin::<()>::default())
        .add_startup_system(setup.system())
        // this system will change the size depending on time elapsed since startup
        .add_system(update_size.system())
        .run();

    Ok(())
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut nine_patches: ResMut<Assets<NinePatchBuilder<()>>>,
) {
    let panel_texture_handle = asset_server
        .load_sync(&mut textures, "assets/glassPanel_corners.png")
        .unwrap();

    // load the 9-Patch as an assets and keep an `Handle<NinePatchBuilder<()>>`
    let nine_patch_handle = nine_patches.add(NinePatchBuilder::by_margins(20, 20, 20, 20, ()));

    commands.spawn(
        // this component bundle will be detected by the plugin, and the 9-Patch UI element will be added as a child
        // of this entity
        NinePatchComponents {
            style: Style {
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                size: Size::new(Val::Px(50.), Val::Px(50.)),
                ..Default::default()
            },
            nine_patch_data: NinePatchData {
                nine_patch: nine_patch_handle,
                texture: panel_texture_handle,
                ..Default::default()
            },
            ..Default::default()
        },
    );

    commands.spawn(UiCameraComponents::default());
}

// by changing the component `Style.size`, the 9-Patch UI element will be resized
fn update_size(time: Res<Time>, mut style: Mut<Style>, _np_component: &NinePatchData<()>) {
    let (x, y) = time.seconds_since_startup.sin_cos();

    style.size.width = Val::Px((250. + 200. * x as f32).ceil());
    style.size.height = Val::Px((250. + 200. * y as f32).ceil());
}
