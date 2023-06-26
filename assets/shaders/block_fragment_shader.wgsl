@group(1) @binding(0)
var texture_map: texture_2d<f32>;
@group(1) @binding(1)
var texture_sampler: sampler;

fn wrap_pos(n: f32) -> f32 {
	if (n >= 0.0) {
		return n % 1.0;
	} else {
		return 1.0 - (-n % 1.0);
	}
}

struct VertexOutput {
	@builtin(position) clip_position: vec4<f32>,
	@location(0) world_pos: vec3<f32>,
	@location(1) world_normal: vec3<f32>,
	@location(2) color: vec3<f32>,
}

@fragment
fn fs_main(fragment_in: VertexOutput) -> @location(0) vec4<f32> {
	var offset: vec2<f32>;
	var sample_pos: vec2<f32>;

	if (fragment_in.world_normal.x > 0.0) {
		offset.x = 0.5;
		offset.y = 0.66666;
		sample_pos.x = 0.25 * wrap_pos(fragment_in.world_pos.z);
		sample_pos.y = 0.33333 * wrap_pos(fragment_in.world_pos.y);
	} else if (fragment_in.world_normal.x < 0.0) {
		offset.x = 0.5;
		offset.y = 0.33333;
		sample_pos.x = 0.25 * wrap_pos(fragment_in.world_pos.z);
		sample_pos.y = -0.33333 * wrap_pos(fragment_in.world_pos.y);
	} else if (fragment_in.world_normal.y > 0.0) {
		offset.x = 0.25;
		offset.y = 0.33333;
		sample_pos.x = -0.25 * wrap_pos(fragment_in.world_pos.z);
		sample_pos.y = 0.33333 * wrap_pos(fragment_in.world_pos.x);
	} else if (fragment_in.world_normal.y < 0.0) {
		offset.x = 0.5;
		offset.y = 0.33333;
		sample_pos.x = 0.25 * wrap_pos(fragment_in.world_pos.z);
		sample_pos.y = 0.33333 * wrap_pos(fragment_in.world_pos.x);
	} else if (fragment_in.world_normal.z > 0.0) {
		offset.x = 0.75;
		offset.y = 0.33333;
		sample_pos.x = 0.25 * wrap_pos(fragment_in.world_pos.y);
		sample_pos.y = 0.33333 * wrap_pos(fragment_in.world_pos.x);
	} else {
		offset.x = 0.5;
		offset.y = 0.33333;
		sample_pos.x = -0.25 * wrap_pos(fragment_in.world_pos.y);
		sample_pos.y = 0.33333 * wrap_pos(fragment_in.world_pos.x);
	}

	return vec4<f32>(fragment_in.color, 1.0) * textureSample(texture_map, texture_sampler, offset + sample_pos);
}
