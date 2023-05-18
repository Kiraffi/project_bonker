
@group(0) @binding(0) var textureInput: texture_2d<f32>;
@group(0) @binding(1) var textureOutput: texture_storage_2d<rgba8unorm, write>;

@compute
@workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>)
{
    var color = textureLoad(textureInput, global_id.xy, 0);
    textureStore(textureOutput, global_id.xy, color);
}
