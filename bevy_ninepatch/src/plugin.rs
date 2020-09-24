use bevy::{
    app::AppBuilder,
    app::Plugin,
    asset::Assets,
    asset::Handle,
    ecs::{Bundle, Commands, DynamicBundle, Entity, IntoQuerySystem, Mutated, Query, Res, ResMut},
    math::{Size, Vec2},
    property::Properties,
    render::color::Color,
    render::draw::Draw,
    render::texture::Texture,
    sprite::ColorMaterial,
    transform::components::{Children, GlobalTransform, Transform},
    transform::hierarchy::BuildChildren,
    ui::{entity::NodeComponents, FocusPolicy, Node, Style, Val},
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
/// Size of the current `NinePatch`
#[derive(Debug, Clone, Copy, Properties)]
pub struct NinePatchSize(pub Vec2);

impl Default for NinePatchSize {
    fn default() -> Self {
        NinePatchSize(Default::default())
    }
}

/// Component Bundle to place the NinePatch
pub struct NinePatchComponents<T: Clone + Send + Sync + 'static> {
    /// Style of this UI node
    pub style: Style,
    /// State of the `NinePatch`
    pub nine_patch_data: NinePatchData<T>,
    /// Size of the `NinePatch`
    pub nine_patch_size: NinePatchSize,
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
            nine_patch_size: Default::default(),
            node: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
        }
    }
}

impl<T: Clone + Send + Sync + 'static> Bundle for NinePatchComponents<T> {
    fn with_static_ids<V>(f: impl FnOnce(&[std::any::TypeId]) -> V) -> V {
        const N: usize = 6;
        let mut xs: [(usize, std::any::TypeId); N] = [
            (
                std::mem::align_of::<Style>(),
                std::any::TypeId::of::<Style>(),
            ),
            (
                std::mem::align_of::<NinePatchData<T>>(),
                std::any::TypeId::of::<NinePatchData<T>>(),
            ),
            (
                std::mem::align_of::<NinePatchSize>(),
                std::any::TypeId::of::<NinePatchSize>(),
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
            bevy::ecs::TypeInfo::of::<NinePatchSize>(),
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
        let nine_patch_size = f(
            std::any::TypeId::of::<NinePatchSize>(),
            std::mem::size_of::<NinePatchSize>(),
        )
        .ok_or_else(bevy::ecs::MissingComponent::new::<NinePatchSize>)?
        .as_ptr()
        .cast::<NinePatchSize>();
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
            nine_patch_size: nine_patch_size.read(),
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
            (&mut self.nine_patch_size as *mut NinePatchSize).cast::<u8>(),
            std::any::TypeId::of::<NinePatchSize>(),
            std::mem::size_of::<NinePatchSize>(),
        ) {
            std::mem::forget(self.nine_patch_size);
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
            .add_stage_after(bevy::app::stage::UPDATE, "NINEPATCH_CHECK_UPDATED_SIZE")
            .add_system(create_ninepatches::<T>.system())
            .add_system_to_stage("NINEPATCH_CHECK_UPDATED_SIZE", update_sizes::<T>.system());
    }
}

#[allow(clippy::type_complexity)]
fn create_ninepatches<T: Clone + Send + Sync + 'static>(
    mut commands: Commands,
    nine_patches: Res<Assets<NinePatchBuilder<T>>>,
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut patches_query: Query<(Entity, &mut NinePatchData<T>, &NinePatchSize)>,
) {
    for (entity, mut data, size) in &mut patches_query.iter() {
        if !data.loaded {
            if let Some(nine_patch) = nine_patches.get(&data.nine_patch) {
                commands
                    .spawn(NodeComponents {
                        draw: Draw {
                            is_transparent: true,
                            ..Default::default()
                        },
                        material: materials.add(Color::NONE.into()),
                        ..Default::default()
                    })
                    .with(FocusPolicy::Pass);
                let parent = commands
                    .current_entity()
                    .expect("should have a current entity as one was created just before");
                let mut id = 0;
                commands.with_children(|p| {
                    id = nine_patch
                        .apply(data.texture, &mut textures, &mut materials)
                        .add(p, size.0.x(), size.0.y(), |_, _| {});
                });
                commands.push_children(entity, &[parent]);
                commands.with(NinePatchId(id));
                data.loaded = true;
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn update_sizes<T: Clone + Send + Sync + 'static>(
    mut patches_query: Query<(&NinePatchData<T>, Mutated<NinePatchSize>, &Children)>,
    id_query: Query<&NinePatchId>,
    mut growth_info: Query<(&NinePatchId, &BuildedNinePatchGrowth, &mut Style)>,
) {
    for (data, new_size, children) in &mut patches_query.iter() {
        if data.loaded {
            let id = children
                .iter()
                .filter_map(|entity| id_query.get::<NinePatchId>(*entity).ok())
                .next()
                .expect("should have a child with component `NinePatchId`");
            for (children_id, growth, mut style) in &mut growth_info.iter() {
                if id.0 == children_id.0 {
                    *style = Style {
                        size: Size::new(
                            match growth.x {
                                None => style.size.width,
                                Some(BuildedNinePatchGrowthAxis { fixed, ratio }) => {
                                    Val::Px((new_size.0.x() - fixed) * ratio)
                                }
                            },
                            match growth.y {
                                None => style.size.height,
                                Some(BuildedNinePatchGrowthAxis { fixed, ratio }) => {
                                    Val::Px((new_size.0.y() - fixed) * ratio)
                                }
                            },
                        ),
                        ..*style
                    };
                }
            }
        }
    }
}
