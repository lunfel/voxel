struct Uniforms {
    view_proj: mat4x4<f32>, // View-projection matrix
    model: mat4x4<f32>,     // Model matrix
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec3<f32>
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>
}

@vertex
fn vertex(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    let world_position = uniforms.model * vec4<f32>(input.position, 1.0);
    output.position = uniforms.view_proj * world_position;

    return output;
}

@fragment
fn fragment() -> @location(0) vec4<f32> {
    return vec4<f32>(0.14509803, 0.5882353, 0.74509803, 1.0); // Blue color
}