use bevy::{
    app::AppBuilder,
    app::Plugin,
    asset::Assets,
    asset::Handle,
    ecs::{Bundle, Commands, DynamicBundle, Entity, IntoQuerySystem, Query, Res, ResMut},
    math::Vec2,
    property::Properties,
    render::color::Color,
    render::draw::Draw,
    render::texture::Texture,
    sprite::ColorMaterial,
    transform::components::{GlobalTransform, Transform},
    transform::hierarchy::BuildChildren,
    ui::FocusPolicy,
    ui::{entity::NodeComponents, Node, Style},
};

use crate::ninepatch::*;

/// State of the current `NinePatch`
#[derive(Debug, Clone, Copy, Properties)]
pub struct NinePatchData {
    /// Handle of the texture
    pub texture: Handle<Texture>,
    /// Size for the target UI element
    pub size: Vec2,
    /// Is the element already loaded and displayed
    pub loaded: bool,
}

impl Default for NinePatchData {
    fn default() -> Self {
        NinePatchData {
            texture: Default::default(),
            size: Default::default(),
            loaded: false,
        }
    }
}

/// Component Bundle to place the NinePatch
pub struct NinePatchComponents {
    /// Handle to the `NinePatchBuilder`
    pub nine_patch: Handle<NinePatchBuilder<()>>,
    /// Style of this UI node
    pub style: Style,
    /// State of the `NinePatch`
    pub data: NinePatchData,
    /// UI node
    pub node: Node,
    /// Transform
    pub transform: Transform,
    /// Global transform - should be set automatically by bevy's systems
    pub global_transform: GlobalTransform,
}

impl Default for NinePatchComponents {
    fn default() -> Self {
        NinePatchComponents {
            nine_patch: Default::default(),
            style: Default::default(),
            data: Default::default(),
            node: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
        }
    }
}

impl Bundle for NinePatchComponents {
    fn with_static_ids<V>(f: impl FnOnce(&[std::any::TypeId]) -> V) -> V {
        const N: usize = 6;
        let mut xs: [(usize, std::any::TypeId); N] = [
            (
                std::mem::align_of::<Handle<NinePatchBuilder<()>>>(),
                std::any::TypeId::of::<Handle<NinePatchBuilder<()>>>(),
            ),
            (
                std::mem::align_of::<Style>(),
                std::any::TypeId::of::<Style>(),
            ),
            (
                std::mem::align_of::<NinePatchData>(),
                std::any::TypeId::of::<NinePatchData>(),
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
            bevy::ecs::TypeInfo::of::<Handle<NinePatchBuilder<()>>>(),
            bevy::ecs::TypeInfo::of::<Style>(),
            bevy::ecs::TypeInfo::of::<NinePatchData>(),
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
        let nine_patch = f(
            std::any::TypeId::of::<Handle<NinePatchBuilder<()>>>(),
            std::mem::size_of::<Handle<NinePatchBuilder<()>>>(),
        )
        .ok_or_else(bevy::ecs::MissingComponent::new::<Handle<NinePatchBuilder<()>>>)?
        .as_ptr()
        .cast::<Handle<NinePatchBuilder<()>>>();
        let style = f(
            std::any::TypeId::of::<Style>(),
            std::mem::size_of::<Style>(),
        )
        .ok_or_else(bevy::ecs::MissingComponent::new::<Style>)?
        .as_ptr()
        .cast::<Style>();
        let data = f(
            std::any::TypeId::of::<NinePatchData>(),
            std::mem::size_of::<NinePatchData>(),
        )
        .ok_or_else(bevy::ecs::MissingComponent::new::<NinePatchData>)?
        .as_ptr()
        .cast::<NinePatchData>();
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
            nine_patch: nine_patch.read(),
            style: style.read(),
            data: data.read(),
            node: node.read(),
            transform: transform.read(),
            global_transform: global_transform.read(),
        })
    }
}

impl DynamicBundle for NinePatchComponents {
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
            (&mut self.nine_patch as *mut Handle<NinePatchBuilder<()>>).cast::<u8>(),
            std::any::TypeId::of::<Handle<NinePatchBuilder<()>>>(),
            std::mem::size_of::<Handle<NinePatchBuilder<()>>>(),
        ) {
            std::mem::forget(self.nine_patch);
        }
        if f(
            (&mut self.style as *mut Style).cast::<u8>(),
            std::any::TypeId::of::<Style>(),
            std::mem::size_of::<Style>(),
        ) {
            std::mem::forget(self.style);
        }
        if f(
            (&mut self.data as *mut NinePatchData).cast::<u8>(),
            std::any::TypeId::of::<NinePatchData>(),
            std::mem::size_of::<NinePatchData>(),
        ) {
            std::mem::forget(self.data);
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
pub struct NinePatchPlugin;

impl Plugin for NinePatchPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Assets<NinePatchBuilder<()>>>()
            .add_system(setup.system());
    }
}

#[allow(clippy::type_complexity)]
fn setup(
    mut commands: Commands,
    nine_patches: Res<Assets<NinePatchBuilder<()>>>,
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut patches_query: Query<(Entity, &mut NinePatchData, &Handle<NinePatchBuilder<()>>)>,
) {
    for (entity, mut data, nine_patch) in &mut patches_query.iter() {
        if !data.loaded {
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
                    .with(FocusPolicy::Pass);
                let parent = commands.current_entity().unwrap();
                commands.with_children(|p| {
                    nine_patch
                        .apply(data.texture, &mut textures, &mut materials)
                        .add(p, data.size.x(), data.size.y(), |_, _| {})
                });
                commands.push_children(entity, &[parent]);
                data.loaded = true;
            }
        }
    }
}