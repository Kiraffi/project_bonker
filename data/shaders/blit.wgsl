struct VertexOutput
{
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0)@binding(1)
var s_diffuse: sampler;

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput
{
    // No ternary operator? Make the triangle [{-1, -1}, {-1, 3}, {3, -1}] in clip space
    // to cover whole screen {-1, -1} - {1, 1}
    let x = f32(i32(in_vertex_index & 2u) * 2 - 1);
    let y = f32(i32(in_vertex_index & 1u) * 4 - 1);

    var out: VertexOutput;
    out.tex_coords = vec2<f32>(x * 0.5 + 0.5, -y * 0.5 + 0.5);
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
