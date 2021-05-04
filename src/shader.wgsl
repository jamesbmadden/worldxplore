struct VertexOutput {
    [[location(0)]] tex_coord: vec2<f32>;
    [[location(1)]] light_intensity: vec3<f32>;
    [[builtin(position)]] position: vec4<f32>;
};

[[block]]
struct Uniforms {
    camera_offset: vec2<f32>;
    is_swimming: i32;
    time: f32;
    light_intensity: vec3<f32>;
};
[[group(1), binding(0)]]
var uniforms: Uniforms;

fn get_anim_frame_tex_coord(frame1_tex_coord: vec2<f32>, animation_frames: f32) -> vec2<f32> {
    var frame: f32 = floor(uniforms.time) % animation_frames;
    return vec2<f32>(frame1_tex_coord.x, frame1_tex_coord.y + 8. / 80. * frame);
}

[[stage(vertex)]]
fn vs_main(
    [[location(0)]] position: vec2<f32>, 
    [[location(1)]] tex_coord: vec2<f32>,
    [[location(2)]] animation_frames: f32,
) -> VertexOutput {

    var out: VertexOutput;
    out.position = vec4<f32>(position.x - uniforms.camera_offset.x, position.y + uniforms.camera_offset.y, 0.0, 1.0);
    out.tex_coord = get_anim_frame_tex_coord(tex_coord, animation_frames);
    out.light_intensity = uniforms.light_intensity;
    return out;
}

[[stage(vertex)]]
fn vs_ui(
    [[location(0)]] position: vec2<f32>, 
    [[location(1)]] tex_coord: vec2<f32>,
    [[location(2)]] animation_frames: f32,
) -> VertexOutput {

    var out: VertexOutput;
    out.position = vec4<f32>(position.x, position.y, 0.0, 1.0);
    out.tex_coord = get_anim_frame_tex_coord(tex_coord, animation_frames);
    out.light_intensity = vec3<f32>(1.0, 1.0, 1.0);
    return out;
}

[[stage(vertex)]]
fn vs_player(
    [[location(0)]] position: vec2<f32>, 
    [[location(1)]] tex_coord: vec2<f32>,
    [[location(2)]] animation_frames: f32,
) -> VertexOutput {

    var out: VertexOutput;
    // if the player is swimming, add a subtle bobbing efffect
    if (uniforms.is_swimming == 1) {
        out.position = vec4<f32>(position.x, position.y + sin(uniforms.time * 2.) * 0.01, 0.0, 1.0);
        // use swimming texture instead of regular texture
        // tileset size is static for now but we'll make it dynamic in the future
        out.tex_coord = vec2<f32>(tex_coord.x + 8. / 128., tex_coord.y);
    } else {
        out.position = vec4<f32>(position.x, position.y, 0.0, 1.0);
        out.tex_coord = tex_coord;
    }
    out.light_intensity = uniforms.light_intensity;
    return out;
}

[[group(0), binding(0)]]
var f_tex_color: texture_2d<f32>;
[[group(0), binding(1)]]
var f_tex_sampler: sampler;

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return textureSample(f_tex_color, f_tex_sampler, in.tex_coord) * vec4<f32>(in.light_intensity.x, in.light_intensity.y, in.light_intensity.z, 1.0);
}