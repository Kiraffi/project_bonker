
@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;

@compute
@workgroupsize(8, 8)
fn cs_main(@builtin(global_invocation_id) global_id: vec3<u32>)
{
    t_diffuse[global_id.xy] = 0;
}
