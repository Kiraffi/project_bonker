struct TestStruct
{
    atom_uvar: atomic<u32>,
    atom_ivar: atomic<i32>,
}

@group(0) @binding(0) var<storage, read_write> test_struct: TestStruct;
@group(0) @binding(1) var<storage, read_write> write_arr: array<u32>;

var<workgroup> item_index: atomic<u32>;

@compute
@workgroup_size(64, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>)
{
    let global_index = global_id.x;
    let local_index = global_id.x % 64u;
    if(local_index == 0u)
    {
        var pos = atomicAdd(&test_struct.atom_uvar, 1u);
        atomicStore(&item_index, pos);
    }
    workgroupBarrier();
    storageBarrier();

    var pos = atomicLoad(&item_index);

    write_arr[global_index] =  local_index + (pos * 1000u);

    //var color = textureLoad(textureInput, global_id.xy, 0);
    //textureStore(textureOutput, global_id.xy, color);
}


@compute
@workgroup_size(1, 1, 1)
fn main_reset(@builtin(global_invocation_id) global_id: vec3<u32>)
{
    atomicStore(&test_struct.atom_uvar, 0u);
    atomicStore(&test_struct.atom_ivar, 0);
}
