use rand::Rng;
use std::thread;
use std::time::{Duration, SystemTime};
use vulkano::{buffer::BufferContents, pipeline::graphics::vertex_input::Vertex};

#[derive(BufferContents, Vertex, Debug)]
#[repr(C)]
pub struct Position {
    #[format(R32G32B32_SFLOAT)]
    position: [f32; 3],
}

#[derive(BufferContents, Vertex, Debug)]
#[repr(C)]
pub struct Normal {
    #[format(R32G32B32_SFLOAT)]
    normal: [f32; 3],
}

#[derive(Debug)]
pub struct Magma {
    positions: Vec<Position>,
    normals: Vec<Normal>,
    indices: Vec<u32>,
}

#[derive(Debug)]
pub struct Stone {
    positions: Vec<Position>,
    normals: Vec<Normal>,
    indices: Vec<u32>,
}

pub fn wait_for_a_minute() {
    let tsn3 = Duration::from_secs(60);
    // Print text to the console.

    thread::sleep(tsn3);
}

pub fn magma(flow: u32) -> Magma {
    let lava_flow = Magma {
        positions: vec![
            Position {
                position: [0.0, 0.0, 0.0],
            },
            Position {
                position: [1.0, 1.0, 1.0],
            },
            //  Position {
            //      position: [0.0, 2.0, 0.0],
            //  },
        ],
        normals: vec![
            Normal {
                normal: [-0.1, -0.3, -0.5],
            },
            Normal {
                normal: [1.0, 1.0, 1.0],
            },
            // Normal {
            //     normal: [0.5, 1.0, 0.5],
            // },
        ],
        indices: vec![1, 2, 2, 3],
    };

    return lava_flow;
}

pub fn petrify(flow: Magma) -> Stone {
    if flow.positions.len() > 2 {
        return petrify_flow(flow);
    };

    let points_diff = sbtr_f32_3(flow.positions[0].position, flow.positions[1].position);

    let planes_normal: [f32; 3] = nrmlz_f32_3(points_diff);

    let mut rng = rand::thread_rng();

    let planes_number = rng.gen_range(16..128);

    let outer_planes = planes_number / 8;

    let mut planes_points = Vec::new();

    for i in 1..(planes_number + outer_planes) {
        planes_points.push(mltply_f32_3(
            points_diff,
            ((i as f32) - ((outer_planes / 2) as f32)) / (planes_number as f32),
        ));
    }

    println!("{:#?}", planes_points);

    let stone = Stone {
        positions: vec![],
        normals: vec![],
        indices: vec![],
    };

    //    let i = 0;
    //    for magmae in flow.positions.iter() {
    //        for i in 1..18 {}
    //    }

    return stone;
}

pub fn petrify_flow(flow: Magma) -> Stone {
    return Stone {
        positions: vec![],
        normals: vec![],
        indices: vec![],
    };
}

pub fn sbtr_f32_3(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    return [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
}

pub fn dd_f32_3(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    return [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
}

pub fn mltply_f32_3(a: [f32; 3], b: f32) -> [f32; 3] {
    return [a[0] * b, a[1] * b, a[2] * b];
}

pub fn nrmlz_f32_3(a: [f32; 3]) -> [f32; 3] {
    let m = a[0].abs().max(a[1].abs()).max(a[2].abs());

    //let m = max(max(abs(a[0]), abs(a[1])), abs(a[2]));
    return [a[0] / m, a[1] / m, a[2] / m];
}
