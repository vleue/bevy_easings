use bevy::{
    app::AppBuilder,
    app::Plugin,
    asset::Assets,
    asset::Handle,
    ecs::{Bundle, Commands, DynamicBundle, Entity, IntoQuerySystem, Query, ResMut},
    render::texture::Texture,
    sprite::ColorMaterial,
    transform::components::{GlobalTransform, Transform},
    transform::hierarchy::BuildChildren,
    ui::{Node, Style},
};

use crate::ninepatch::*;

/// State of the current `NinePatch`
#[derive(Debug, Clone, Copy)]
pub struct NinePatchData<T: Clone + Send + Sync + 'static> {
    /// Handle of the texture
    pub texture: Handle<Texture>,
    /// Handle to the `NinePatchBuilder`
    pub nine_patch: Handle<NinePatchBuilder<T>>,
    /// Is the element already loaded and displayed
    pub loaded: bool,
}

impl<T: Clone + Send + Sync + 'static> Default for NinePatchData<T> {
    fn default() -> Self {
        NinePatchData {
            texture: Default::default(),
            nine_patch: Default::default(),
            loaded: false,
        }
    }
}

/// Component Bundle to place the NinePatch
pub struct NinePatchComponents<T: Clone + Send + Sync + 'static> {
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

impl<T: Clone + Send + Sync + 'static> Default for NinePatchComponents<T> {
    fn default() -> Self {
        NinePatchComponents {
            style: Default::default(),
            nine_patch_data: Default::default(),
            node: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
        }
    }
}

impl<T: Clone + Send + Sync + 'static> Bundle for NinePatchComponents<T> {
    fn with_static_ids<V>(f: impl FnOnce(&[std::any::TypeId]) -> V) -> V {
        const N: usize = 5;
        let mut xs: [(usize, std::any::TypeId); N] = [
            (
                std::mem::align_of::<Style>(),
                std::any::TypeId::of::<Style>(),
            ),
            (
                std::mem::align_of::<NinePatchData<T>>(),
                std::any::TypeId::of::<NinePatchData<T>>(),
            ),
            (std::mem::align_of::<Node>(), std::any::TypeId::of::<Node>()),
            (
                std::mem::align_of::<Transform>(),
                std::any::TypeId::of::<Transform>(),
            ),
            (
                std::mem::align_of::<GlobalTransform>(),
                std::any::TypeId::of::<GlobalTransform>(),
            ),
        ];
        xs.sort_unstable_by(|x, y| x.0.cmp(&y.0).reverse().then(x.1.cmp(&y.1)));
        let mut ids = [std::any::TypeId::of::<()>(); N];
        for (slot, &(_, id)) in ids.iter_mut().zip(xs.iter()) {
            *slot = id;
        }
        f(&ids)
    }

    fn static_type_info() -> Vec<bevy::ecs::TypeInfo> {
        let mut xs = vec![
            bevy::ecs::TypeInfo::of::<Style>(),
            bevy::ecs::TypeInfo::of::<NinePatchData<T>>(),
            bevy::ecs::TypeInfo::of::<Node>(),
            bevy::ecs::TypeInfo::of::<Transform>(),
            bevy::ecs::TypeInfo::of::<GlobalTransform>(),
        ];
        xs.sort_unstable();
        xs
    }

    #[allow(unsafe_code)]
    unsafe fn get(
        mut f: impl FnMut(std::any::TypeId, usize) -> Option<std::ptr::NonNull<u8>>,
    ) -> Result<Self, bevy::ecs::MissingComponent>
    where
        Self: Sized,
    {
        let style = f(
            std::any::TypeId::of::<Style>(),
            std::mem::size_of::<Style>(),
        )
        .ok_or_else(bevy::ecs::MissingComponent::new::<Style>)?
        .as_ptr()
        .cast::<Style>();
        let nine_patch_data = f(
            std::any::TypeId::of::<NinePatchData<T>>(),
            std::mem::size_of::<NinePatchData<T>>(),
        )
        .ok_or_else(bevy::ecs::MissingComponent::new::<NinePatchData<T>>)?
        .as_ptr()
        .cast::<NinePatchData<T>>();
        let node = f(std::any::TypeId::of::<Node>(), std::mem::size_of::<Node>())
            .ok_or_else(bevy::ecs::MissingComponent::new::<Node>)?
            .as_ptr()
            .cast::<Node>();
        let transform = f(
            std::any::TypeId::of::<Transform>(),
            std::mem::size_of::<Transform>(),
        )
        .ok_or_else(bevy::ecs::MissingComponent::new::<Transform>)?
        .as_ptr()
        .cast::<Transform>();
        let global_transform = f(
            std::any::TypeId::of::<GlobalTransform>(),
            std::mem::size_of::<GlobalTransform>(),
        )
        .ok_or_else(bevy::ecs::MissingComponent::new::<GlobalTransform>)?
        .as_ptr()
        .cast::<GlobalTransform>();

        Ok(NinePatchComponents {
            style: style.read(),
            nine_patch_data: nine_patch_data.read(),
            node: node.read(),
            transform: transform.read(),
            global_transform: global_transform.read(),
        })
    }
}

impl<T: Clone + Send + Sync + 'static> DynamicBundle for NinePatchComponents<T> {
    fn with_ids<V>(&self, _f: impl FnOnce(&[std::any::TypeId]) -> V) -> V {
        Self::with_static_ids(_f)
    }

    fn type_info(&self) -> Vec<bevy::ecs::TypeInfo> {
        Self::static_type_info()
    }

    #[allow(unsafe_code)]
    #[allow(trivial_casts)]
    #[allow(clippy::forget_copy)]
    unsafe fn put(mut self, mut f: impl FnMut(*mut u8, std::any::TypeId, usize) -> bool) {
        if f(
            (&mut self.style as *mut Style).cast::<u8>(),
            std::any::TypeId::of::<Style>(),
            std::mem::size_of::<Style>(),
        ) {
            std::mem::forget(self.style);
        }
        if f(
            (&mut self.nine_patch_data as *mut NinePatchData<T>).cast::<u8>(),
            std::any::TypeId::of::<NinePatchData<T>>(),
            std::mem::size_of::<NinePatchData<T>>(),
        ) {
            std::mem::forget(self.nine_patch_data);
        }
        if f(
            (&mut self.node as *mut Node).cast::<u8>(),
            std::any::TypeId::of::<Node>(),
            std::mem::size_of::<Node>(),
        ) {
            std::mem::forget(self.node);
        }
        if f(
            (&mut self.transform as *mut Transform).cast::<u8>(),
            std::any::TypeId::of::<Transform>(),
            std::mem::size_of::<Transform>(),
        ) {
            std::mem::forget(self.transform);
        }
        if f(
            (&mut self.global_transform as *mut GlobalTransform).cast::<u8>(),
            std::any::TypeId::of::<GlobalTransform>(),
            std::mem::size_of::<GlobalTransform>(),
        ) {
            std::mem::forget(self.global_transform);
        }
    }
}

/// Plugin that will add the system and the resource for nine patch
#[derive(Debug, Clone, Copy)]
pub struct NinePatchPlugin<T: Clone + Send + Sync + 'static> {
    marker: std::marker::PhantomData<T>,
}

impl<T: Clone + Send + Sync + 'static> Default for NinePatchPlugin<T> {
    fn default() -> Self {
        NinePatchPlugin {
            marker: Default::default(),
        }
    }
}

impl<T: Clone + Send + Sync + 'static> Plugin for NinePatchPlugin<T> {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Assets<NinePatchBuilder<T>>>()
            .add_system(create_ninepatches::<T>.system());
    }
}

#[allow(clippy::type_complexity)]
fn create_ninepatches<T: Clone + Send + Sync + 'static>(
    mut commands: Commands,
    mut nine_patches: ResMut<Assets<NinePatchBuilder<T>>>,
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut patches_query: Query<(Entity, &mut NinePatchData<T>, &Style)>,
) {
    for (entity, mut data, style) in &mut patches_query.iter() {
        if !data.loaded {
            if let Some(nine_patch) = nine_patches.get_mut(&data.nine_patch) {
                if textures.get(&data.texture).is_none() {
                    // texture is not available yet, will try next loop
                    continue;
                }
                let np = nine_patch.apply(data.texture, &mut textures, &mut materials);
                np.add_with_parent(&mut commands, entity, style);
                let parent = commands
                    .current_entity()
                    .expect("should have a current entity as one was created just before");
                commands.push_children(entity, &[parent]);
                data.loaded = true;
            }
        }
    }
}
