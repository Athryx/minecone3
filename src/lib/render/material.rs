use bevy::{
    prelude::*,
    render::{
        render_resource::{
            AsBindGroup,
            VertexFormat,
            ShaderRef,
            RenderPipelineDescriptor,
            SpecializedMeshPipelineError,
        },
        mesh::{MeshVertexAttribute, MeshVertexBufferLayout},
    },
    pbr::{MaterialPipeline, MaterialPipelineKey},
    reflect::{TypeUuid, TypePath},
};

#[derive(Debug, Clone, AsBindGroup, TypeUuid, TypePath)]
#[uuid = "059e5370-ad92-4732-91c1-828f20d64025"]
pub struct BlockMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub texture_map: Handle<Image>,
}

impl Material for BlockMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/block_shader.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/block_shader.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            ATTRIBUTE_UV_BASE.at_shader_location(1),
            ATTRIBUTE_FACE_COUNT.at_shader_location(2),
        ])?;

        descriptor.vertex.buffers = vec![vertex_layout];

        Ok(())
    }
}

/// Specifies the base coordinate in the texture map of this face (top left corner)
pub const ATTRIBUTE_UV_BASE: MeshVertexAttribute = MeshVertexAttribute::new(
    "Vertex_Uv_Base",
    307134,
    VertexFormat::Float32x2,
);

/// Used to specify how many faces of a block should be drawn
/// 
/// This is needed so greedy meshed quads can still have multiple faces without stretching them out
pub const ATTRIBUTE_FACE_COUNT: MeshVertexAttribute = MeshVertexAttribute::new(
    "Vertex_Face_Count",
    13298431,
    VertexFormat::Float32x2,
);

#[derive(Debug, Resource)]
pub struct GlobalBlockMaterial(pub Handle<BlockMaterial>);