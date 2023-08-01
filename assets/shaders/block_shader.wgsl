#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings
#import bevy_pbr::mesh_functions

// 1 / 4 (because texture map is size 4)
const BLOCK_UV_SIZE: f32 = 0.25;

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
fn vertex(vertex_input: VertexInput) -> VertexOutput {
	var out: VertexOutput;

	out.clip_position = mesh_position_local_to_clip(
		mesh.model,
		vec4<f32>(vertex_input.position, 1.0),
	);

	out.uv_base = vertex_input.uv_base;
	out.face_count = vertex_input.face_count;

	return out;
}

@fragment
fn fragment(fragment_input: VertexOutput) -> @location(0) vec4<f32> {
	var block_coord = vec2<f32>(
		fragment_input.face_count.x % 1.0,
		fragment_input.face_count.y % 1.0,
	);

	return textureSample(texture_map, texture_sampler, fragment_input.uv_base + BLOCK_UV_SIZE * block_coord);
}
