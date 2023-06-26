use bevy::{prelude::*, render::render_resource::AsBindGroup, reflect::TypeUuid};

#[derive(Debug, Clone, AsBindGroup, TypeUuid)]
#[uuid = "059e5370-ad92-4732-91c1-828f20d64025"]
pub struct BlockMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub texture_map: Handle<Image>,
}

impl Material for BlockMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/block_fragment_shader.wgsl".into()
    }
}

#[derive(Debug, Resource)]
pub struct GlobalTextureMap(pub Handle<Image>);

#[derive(Debug, Resource)]
pub struct GlobalBlockMaterial(pub Handle<BlockMaterial>);

pub fn initialize_block_material(
    mut materials: ResMut<Assets<BlockMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut commands: Commands,
) {
    let texture_map = images.add(Image::default());
    commands.insert_resource(GlobalTextureMap(texture_map.clone()));

    let block_material = materials.add(BlockMaterial {
        texture_map,
    });
    commands.insert_resource(GlobalBlockMaterial(block_material));
}