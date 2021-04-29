[[location(0)]] var<in> position: vec3<f32>;

[[builtin(position)]] var<out> out_position: vec4<f32>;
[[location(0)]] var<out> out_color: vec4<f32>;

[[stage(vertex)]]
fn vs_main() {
    out_position = vec4<f32>(position.x, position.y, 0.0, 1.0);
}

[[stage(fragment)]]
fn fs_main() {
    out_color = vec4<f32>(1.0, 1.0, 1.0, 1.0);
}