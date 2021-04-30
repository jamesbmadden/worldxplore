[[block]]
struct Uniforms {
    camera_offset: vec2<f32>;
};

[[location(0)]] var<in> position: vec2<f32>;
[[location(1)]] var<in> v_tex_coord: vec2<f32>;
[[group(1), binding(0)]]
var<uniform> uniforms: Uniforms;

[[builtin(position)]] var<out> out_position: vec4<f32>;
[[location(0)]] var<out> out_color: vec4<f32>;
[[location(1)]] var<out> f_tex_coord: vec2<f32>;

[[stage(vertex)]]
fn vs_main() {
    out_position = vec4<f32>(position.x - uniforms.camera_offset.x, position.y + uniforms.camera_offset.y, 0.0, 1.0);
    f_tex_coord = v_tex_coord;
}

[[location(1)]] var<in> f_tex_coord: vec2<f32>;
[[group(0), binding(0)]]
var f_tex_color: texture_2d<f32>;
[[group(0), binding(1)]]
var f_tex_sampler: sampler;

[[stage(fragment)]]
fn fs_main() {
    out_color = textureSample(f_tex_color, f_tex_sampler, f_tex_coord);
}