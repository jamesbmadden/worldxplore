struct VertexOutput {
    [[location(0)]] tex_coord: vec2<f32>;
    [[builtin(position)]] position: vec4<f32>;
};

[[block]]
struct Uniforms {
    camera_offset: vec2<f32>;
    is_swimming: i32;
    time: f32;
};
[[group(1), binding(0)]]
var uniforms: Uniforms;

[[stage(vertex)]]
fn vs_main(
    [[location(0)]] position: vec2<f32>, 
    [[location(1)]] tex_coord: vec2<f32>,
) -> VertexOutput {

    var out: VertexOutput;
    out.position = vec4<f32>(position.x - uniforms.camera_offset.x, position.y + uniforms.camera_offset.y, 0.0, 1.0);
    out.tex_coord = tex_coord;
    return out;
}

[[stage(vertex)]]
fn vs_player(
    [[location(0)]] position: vec2<f32>, 
    [[location(1)]] tex_coord: vec2<f32>,
) -> VertexOutput {

    var out: VertexOutput;
    // if the player is swimming, add a subtle bobbing efffect
    if (uniforms.is_swimming == 1) {
        out.position = vec4<f32>(position.x, position.y + sin(uniforms.time * 2.) * 0.01, 0.0, 1.0);
    } else {
        out.position = vec4<f32>(position.x, position.y, 0.0, 1.0);
    }
    out.tex_coord = tex_coord;
    return out;
}

[[group(0), binding(0)]]
var f_tex_color: texture_2d<f32>;
[[group(0), binding(1)]]
var f_tex_sampler: sampler;

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return textureSample(f_tex_color, f_tex_sampler, in.tex_coord);
}