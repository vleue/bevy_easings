use bevy::prelude::*;

use crate::ninepatch::*;

#[derive(Debug, Clone, Copy)]
pub struct NinePatchLoaded(bool);
impl Default for NinePatchLoaded {
    fn default() -> Self {
        NinePatchLoaded(false)
    }
}

#[derive(Bundle)]
pub struct NinePatchComponents {
    pub nine_patch: Handle<NinePatchBuilder<()>>,
    pub texture: Handle<Texture>,
    pub loaded: NinePatchLoaded,
    pub node: Node,
    pub transform: Transform,
    pub global_transform: GlobalTransform,

    pub style: Style,
    // pub mesh: Handle<Mesh>, // TODO: maybe abstract this out
    // pub material: Handle<ColorMaterial>,
    // pub draw: Draw,
    // pub render_pipelines: RenderPipelines,
}

impl Default for NinePatchComponents {
    fn default() -> Self {
        NinePatchComponents {
            nine_patch: Default::default(),
            texture: Default::default(),
            loaded: Default::default(),
            node: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            style: Default::default(),
            // draw: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NinePatchPlugin;
impl Plugin for NinePatchPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Assets<NinePatchBuilder<()>>>()
            .add_system(setup.system());
    }
}

fn setup(
    mut commands: Commands,
    nine_patches: Res<Assets<NinePatchBuilder<()>>>,
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut patches_query: Query<(
        Entity,
        &mut NinePatchLoaded,
        &Handle<NinePatchBuilder<()>>,
        &Handle<Texture>,
    )>,
) {
    for (entity, mut loaded, nine_patch, texture_handle) in &mut patches_query.iter() {
        if !loaded.0 {
            if let Some(nine_patch) = nine_patches.get(&nine_patch) {
                commands
                    .spawn(NodeComponents {
                        draw: Draw {
                            is_transparent: true,
                            ..Default::default()
                        },
                        material: materials.add(Color::NONE.into()),
                        ..Default::default()
                    })
                    .with(bevy::ui::FocusPolicy::Pass);
                let parent = commands.current_entity().unwrap();
                commands.with_children(|p| {
                    nine_patch
                        .apply(*texture_handle, &mut textures, &mut materials)
                        .add(p, 300., 300., |_, _| {})
                });
                commands.push_children(entity, &[parent]);
                *loaded = NinePatchLoaded(true);
            }
        }
    }
}
