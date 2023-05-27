pub const WHITE_COLOR: [f32; 4] = [1.0f32, 1.0f32, 1.0f32, 1.0f32];

pub const VERTICES: &[common::MeshVertex] = &[
    common::MeshVertex { position: [-0.5, -0.5, 0.5, 1.0], normal: [0.0, 0.0, 1.0, 0.0], color: WHITE_COLOR },
    common::MeshVertex { position: [-0.5,  0.5, 0.5, 1.0], normal: [0.0, 0.0, 1.0, 0.0], color: WHITE_COLOR },
    common::MeshVertex { position: [ 0.5,  0.5, 0.5, 1.0], normal: [0.0, 0.0, 1.0, 0.0], color: WHITE_COLOR },
    common::MeshVertex { position: [ 0.5, -0.5, 0.5, 1.0], normal: [0.0, 0.0, 1.0, 0.0], color: WHITE_COLOR },

    common::MeshVertex { position: [ 0.5, -0.5, -0.5, 1.0], normal: [0.0, 0.0, -1.0, 0.0], color: WHITE_COLOR },
    common::MeshVertex { position: [ 0.5,  0.5, -0.5, 1.0], normal: [0.0, 0.0, -1.0, 0.0], color: WHITE_COLOR },
    common::MeshVertex { position: [-0.5,  0.5, -0.5, 1.0], normal: [0.0, 0.0, -1.0, 0.0], color: WHITE_COLOR },
    common::MeshVertex { position: [-0.5, -0.5, -0.5, 1.0], normal: [0.0, 0.0, -1.0, 0.0], color: WHITE_COLOR },

    common::MeshVertex { position: [-0.5, -0.5, -0.5, 1.0], normal: [-1.0, 0.0, 0.0, 0.0], color: WHITE_COLOR },
    common::MeshVertex { position: [-0.5,  0.5, -0.5, 1.0], normal: [-1.0, 0.0, 0.0, 0.0], color: WHITE_COLOR },
    common::MeshVertex { position: [-0.5,  0.5,  0.5, 1.0], normal: [-1.0, 0.0, 0.0, 0.0], color: WHITE_COLOR },
    common::MeshVertex { position: [-0.5, -0.5,  0.5, 1.0], normal: [-1.0, 0.0, 0.0, 0.0], color: WHITE_COLOR },

    common::MeshVertex { position: [0.5, -0.5,  0.5, 1.0], normal: [1.0, 0.0, 0.0, 0.0], color: WHITE_COLOR },
    common::MeshVertex { position: [0.5,  0.5,  0.5, 1.0], normal: [1.0, 0.0, 0.0, 0.0], color: WHITE_COLOR },
    common::MeshVertex { position: [0.5,  0.5, -0.5, 1.0], normal: [1.0, 0.0, 0.0, 0.0], color: WHITE_COLOR },
    common::MeshVertex { position: [0.5, -0.5, -0.5, 1.0], normal: [1.0, 0.0, 0.0, 0.0], color: WHITE_COLOR },

    common::MeshVertex { position: [-0.5,  0.5,  0.5, 1.0], normal: [0.0, 1.0, 0.0, 0.0], color: WHITE_COLOR },
    common::MeshVertex { position: [-0.5,  0.5, -0.5, 1.0], normal: [0.0, 1.0, 0.0, 0.0], color: WHITE_COLOR },
    common::MeshVertex { position: [ 0.5,  0.5, -0.5, 1.0], normal: [0.0, 1.0, 0.0, 0.0], color: WHITE_COLOR },
    common::MeshVertex { position: [ 0.5,  0.5,  0.5, 1.0], normal: [0.0, 1.0, 0.0, 0.0], color: WHITE_COLOR },


    common::MeshVertex { position: [ 0.5, -0.5, -0.5, 1.0], normal: [0.0, -1.0, 0.0, 0.0], color: WHITE_COLOR },
    common::MeshVertex { position: [ 0.5, -0.5, 0.5, 1.0], normal: [0.0, -1.0, 0.0, 0.0], color: WHITE_COLOR },
    common::MeshVertex { position: [-0.5, -0.5, 0.5, 1.0], normal: [0.0, -1.0, 0.0, 0.0], color: WHITE_COLOR },
    common::MeshVertex { position: [-0.5, -0.5, -0.5, 1.0], normal: [0.0, -1.0, 0.0, 0.0], color: WHITE_COLOR },
];

pub const INDICES: &[u32] = &[
    0, 1, 2,
    2, 0, 3,

    4, 5, 6,
    6, 4, 7,

    8, 9, 10,
    10, 8, 11,

    12, 13, 14,
    14, 12, 15,

    16, 17, 18,
    18, 16, 19,

    20, 21, 22,
    22, 20, 23,
//    3, 2, 5,
//    5, 4, 3,
//
//    7, 6, 1,
//    1, 0, 7,
//
//    1, 6, 5,
//    5, 2, 1,
//
//    7, 0, 3,
//    3, 4, 7,
];
