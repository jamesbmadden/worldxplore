struct VertexOutput {
    @location(0) tex_coord: vec2<f32>,
    @location(1) light_intensity: vec3<f32>,
    @builtin(position) position: vec4<f32>
};

struct Uniforms {
    camera_offset: vec2<f32>,
    is_swimming: i32,
    time: f32,
    light_intensity: vec3<f32>
};
@group(1) @binding(0)
var<uniform> uniforms: Uniforms;

struct InstanceData {
    @location(3) x: f32,
    @location(4) y: f32,
    @location(5) offset_x: i32,
    @location(6) offset_y: i32,
    @location(7) tile_width: f32,
    @location(8) tile_height: f32,
    @location(9) ts_coord_x: u32,
    @location(10) ts_coord_y: u32,
    @location(11) animation_frames: u32,
    @location(12) width: u32,
    @location(13) height: u32,
    @location(14) tx_width: f32,
    @location(15) tx_height: f32
}

fn get_anim_frame_tex_coord(frame1_tex_coord: vec2<f32>, animation_frames: f32) -> vec2<f32> {
    var frame: f32 = floor(uniforms.time) % animation_frames;
    return vec2<f32>(frame1_tex_coord.x, frame1_tex_coord.y + 8. / 80. * frame);
}

@vertex
fn vs_main(
    @location(0) position: vec2<f32>, 
    @location(1) tex_coord: vec2<f32>,
    @location(2) animation_frames: f32,
    instance: InstanceData
) -> VertexOutput {

    // based on all the instance information the position on screen needs to be reassembled
    var x: f32 = -1.0 + (position.x * f32(instance.width) * instance.tile_width + (instance.x + f32(instance.offset_x)) * instance.tile_width) * 2.0;
    var y: f32 = 1.0 - (position.y * f32(instance.height) * instance.tile_height + (instance.y + f32(instance.offset_y)) * instance.tile_height) * 2.0;

    // adjust the tex coord based on the texture position
    var adjusted_tex_coord = vec2<f32>(
      tex_coord.x * f32(instance.width) + f32(instance.ts_coord_x) * instance.tx_width,
      tex_coord.y * f32(instance.height) + f32(instance.ts_coord_y) * instance.tx_height);

    var out: VertexOutput;
    out.position = vec4<f32>(x - uniforms.camera_offset.x, y + uniforms.camera_offset.y, 0.0, 1.0);
    out.tex_coord = get_anim_frame_tex_coord(adjusted_tex_coord, f32(instance.animation_frames));
    out.light_intensity = uniforms.light_intensity;
    return out;
}

@vertex
fn vs_ui(
    @location(0) position: vec2<f32>, 
    @location(1) tex_coord: vec2<f32>,
    @location(2) animation_frames: f32,
) -> VertexOutput {

    var out: VertexOutput;
    out.position = vec4<f32>(position.x, position.y, 0.0, 1.0);
    out.tex_coord = get_anim_frame_tex_coord(tex_coord, animation_frames);
    out.light_intensity = vec3<f32>(1.0, 1.0, 1.0);
    return out;
}

@vertex
fn vs_player(
    @location(0) position: vec2<f32>, 
    @location(1) tex_coord: vec2<f32>,
    @location(2) animation_frames: f32,
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

@group(0) @binding(0)
var f_tex_color: texture_2d<f32>;
@group(0) @binding(1)
var f_tex_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(f_tex_color, f_tex_sampler, in.tex_coord) * vec4<f32>(in.light_intensity.x, in.light_intensity.y, in.light_intensity.z, 1.0);
}