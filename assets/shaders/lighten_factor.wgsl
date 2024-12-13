@group(0) @binding(0)
var<uniform> u_lighten_factor: f32; // Uniform to control how much to lighten

@fragment
fn fs_main(@location(0) in_color: vec4<f32>) -> @location(0) vec4<f32> {
    // Lighten the color by mixing it with white
    let lightened_color = mix(in_color, vec4<f32>(1.0, 1.0, 1.0, 1.0), u_lighten_factor);

    return lightened_color;
}