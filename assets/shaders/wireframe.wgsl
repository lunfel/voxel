[[group(0), binding(0)]] var<uniform> modelViewProjection: mat4x4<f32>;
[[group(1), binding(0)]] var<uniform> color: vec4<f32>;

[[stage(vertex)]]
fn vs_main(
    [[location(0)]] position: vec3<f32>,
    [[location(1)]] normal: vec3<f32>,
    [[builtin(vertex_index)]] vertex_index: u32,
) -> [[builtin(position)]] vec4<f32> {
    var out_position = vec4<f32>(position, 1.0);
    return modelViewProjection * out_position;
}

[[stage(fragment)]]
fn fs_main(
    [[builtin(position)]] frag_coord: vec4<f32>,
) -> [[location(0)]] vec4<f32> {
    let edge_threshold = 0.02; // Adjust this value to control edge thickness
    let frag_pos = frag_coord.xy / frag_coord.w;
    let distance_to_edge = min(frag_pos.x, 1.0 - frag_pos.x) +
                           min(frag_pos.y, 1.0 - frag_pos.y);

    if (distance_to_edge > edge_threshold) {
        discard;
    }

    return color;
}
