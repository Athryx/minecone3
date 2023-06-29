#import bevy_pbr::mesh_bindings mesh
#import bevy_pbr::mesh_functions mesh_position_to_local_clip

const TEXTURE_MAP_SIZE: u32 = 4;
const BLOCK_UV_SIZE: f32 = 1.0 / TEXTURE_MAP_SIZE as f32;

@group(1) @binding(0)
var texture_map: texture_2d<f32>;
@group(1) @binding(1)
var texture_sampler: sampler;

struct VertexInput {
	@location(0) position: vec3<f32>,
	@location(1) uv_base: vec2<f32>,
	@location(2) face_count: vec2<f32>,
}

struct VertexOutput {
	@builtin(position) clip_position: vec4<f32>,
	@location(0) uv_base: vec2<f32>,
	@location(1) face_count: vec2<f32>,
}

@vertex
fn block_vertex_shader(vertex_input: VertexInput) -> VertexOutput {
	var out: VertexOutput;

	out.clip_position = mesh_position_to_local_clip(
		mesh.model,
		vec4<f32>(vertex.position, 1.0),
	);

	out.uv_base = vertex_input.uv_base;
	out.face_count = vertex_input.face_count;

	return out;
}

@fragment
fn block_fragment_shader(fragment_input: VertexOutput) -> @location(0) vec4<f32> {
	var block_coord = vec2<f32>(
		fragment_input.face_count.x % 1.0,
		fragment_input.face_count.y % 1.0,
	);

	return textureSample(texture_map, texture_sampler, fragment_input.uv_base + BLOCK_UV_SIZE * block_coord);
}
