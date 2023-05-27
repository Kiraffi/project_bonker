struct TestStruct
{
    atom_uvar: atomic<u32>,
    atom_ivar: atomic<i32>,
}

@group(0) @binding(0) var<storage, read_write> test_struct: TestStruct;
@group(0) @binding(0) var<storage, write> write_arr: array<u32>;

var<workgroup> item_index: atomic<u32>;

@compute
@workgroup_size(64, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>)
{
    var local_index = global_invocation_id.x % 64;
    if(local_index % 64 == workgroup)
    {
        var pos = atomicAdd(test_struct.atom_uvar);
        atomicStore(item_index, pos);
    }
    workgroupBarrier();

    var pos = atomicLoad(item_index);

    write_arr[pos + ]

    var color = textureLoad(textureInput, global_id.xy, 0);
    textureStore(textureOutput, global_id.xy, color);
}
