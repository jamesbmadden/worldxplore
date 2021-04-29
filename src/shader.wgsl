[[location(0)]] var<in> position: vec2<f32>;
[[location(1)]] var<in> v_tex_coord: vec2<f32>;

[[builtin(position)]] var<out> out_position: vec4<f32>;
[[location(0)]] var<out> out_color: vec4<f32>;
[[location(1)]] var<out> f_tex_coord: vec2<f32>;

[[stage(vertex)]]
fn vs_main() {
    out_position = vec4<f32>(position.x, position.y, 0.0, 1.0);
    f_tex_coord = v_tex_coord;
}

[[location(1)]] var<in> f_tex_coord: vec2<f32>;

[[stage(fragment)]]
fn fs_main() {
    out_color = vec4<f32>(f_tex_coord.x, f_tex_coord.y, f_tex_coord.x, 1.0);
}