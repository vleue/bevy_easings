use bevy::prelude::*;

use crate::ninepatch::*;

/// State of the current `NinePatch`
#[derive(Debug, Clone)]
pub struct NinePatchData<T: Clone + Send + Sync + Eq + std::hash::Hash + 'static> {
    /// Handle of the texture
    pub texture: Handle<Texture>,
    /// Handle to the `NinePatchBuilder`
    pub nine_patch: Handle<NinePatchBuilder<T>>,
    /// Is the element already loaded and displayed
    pub loaded: bool,
    /// Entity that should be used for the content
    pub content: Option<std::collections::HashMap<T, Entity>>,
}

impl<T: Clone + Send + Sync + Eq + std::hash::Hash + 'static> Default for NinePatchData<T> {
    fn default() -> Self {
        NinePatchData {
            texture: Default::default(),
            nine_patch: Default::default(),
            loaded: false,
            content: Default::default(),
        }
    }
}

impl<T: Clone + Send + Sync + Default + Eq + std::hash::Hash + 'static> NinePatchData<T> {
    /// Create a NinePathData with content when there is only one content
    pub fn with_single_content(
        texture: Handle<Texture>,
        nine_patch: Handle<NinePatchBuilder<T>>,
        content: Entity,
    ) -> NinePatchData<T> {
        let mut content_map = std::collections::HashMap::with_capacity(1);
        content_map.insert(T::default(), content);
        NinePatchData {
            texture,
            nine_patch,
            loaded: false,
            content: Some(content_map),
        }
    }
}

#[derive(Bundle)]
/// Component Bundle to place the NinePatch
pub struct NinePatchBundle<T: Clone + Send + Sync + Eq + std::hash::Hash + 'static> {
    /// Style of this UI node
    pub style: Style,
    /// State of the `NinePatch`
    pub nine_patch_data: NinePatchData<T>,
    /// UI node
    pub node: Node,
    /// Transform
    pub transform: Transform,
    /// Global transform - should be set automatically by bevy's systems
    pub global_transform: GlobalTransform,
}

impl<T: Clone + Send + Sync + Eq + std::hash::Hash + 'static> Default for NinePatchBundle<T> {
    fn default() -> Self {
        NinePatchBundle {
            style: Default::default(),
            nine_patch_data: Default::default(),
            node: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
        }
    }
}

/// Plugin that will add the system and the resource for nine patch
#[derive(Debug, Clone, Copy)]
pub struct NinePatchPlugin<T: Clone + Send + Sync + 'static = ()> {
    marker: std::marker::PhantomData<T>,
}

trait NinePatchPlugina {
    type Zut;
}

impl<T: Clone + Send + Sync + 'static> NinePatchPlugina for NinePatchPlugin<T> {
    type Zut = NinePatchPlugin<()>;
}

impl<T: Clone + Send + Sync + 'static> Default for NinePatchPlugin<T> {
    fn default() -> Self {
        NinePatchPlugin {
            marker: Default::default(),
        }
    }
}
impl<T: Clone + Send + Sync + Eq + std::hash::Hash + 'static> Plugin for NinePatchPlugin<T> {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<NinePatchBuilder<T>>()
            .add_system(create_ninepatches::<T>.system());
    }
}

#[allow(clippy::type_complexity)]
fn create_ninepatches<T: Clone + Send + Sync + Eq + std::hash::Hash + 'static>(
    commands: &mut Commands,
    mut nine_patches: ResMut<Assets<NinePatchBuilder<T>>>,
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut patches_query: Query<(Entity, &mut NinePatchData<T>, &Style)>,
) {
    for (entity, mut data, style) in patches_query.iter_mut() {
        if !data.loaded {
            if let Some(nine_patch) = nine_patches.get_mut(&data.nine_patch) {
                if textures.get(&data.texture).is_none() {
                    // texture is not available yet, will try next loop
                    continue;
                }
                let np = nine_patch.apply(&data.texture, &mut textures, &mut materials);
                np.add_with_parent(commands, entity, style, &data.content);
                data.loaded = true;
            }
        }
    }
}
