use rand::rngs::ThreadRng;
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
                position: [10.0, 10.0, 10.0],
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
        indices: vec![0, 1],
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

    let planes_number = rng.gen_range(16..64);

    let outer_planes = planes_number / 8;

    let mut planes_points = Vec::new();

    for i in 1..(planes_number + outer_planes) {
        planes_points.push(mltply_f32_3(
            points_diff,
            ((i as f32) - ((outer_planes / 2) as f32)) / (planes_number as f32),
        ));
    }

    println!("{:#?}", planes_points);

    let mut stone = Stone {
        positions: vec![],
        normals: vec![],
        indices: vec![],
    };

    let mut previous_plane: [u32; 3] = [0, 0, 0]; // plane number, beginning position, ending position
    let mut points_of_plane = 3;
    let mut points_range = 1.0;
    let mut points_range_min = 0.1;

    for planae in planes_points.iter() {
        for i in 1..points_of_plane {
            let random_vector_on_plane = gen_rthgnl_f32_3(planae, &mut rng);
            let random_range = rng.gen_range(points_range_min..points_range);
            let random_point_on_plane =
                mltply_f32_3(nrmlz_f32_3(random_vector_on_plane), random_range);
            stone.positions.push(Position {
                position: random_point_on_plane,
            });

            let mut k: usize = 1;
            if previous_plane[0] > 0 {
                k = previous_plane[0] as usize;
            };

            let normal = nrmlz_f32_3(sbtr_f32_3(random_point_on_plane, planes_points[k]));

            stone.normals.push(Normal { normal: normal });
        }

        if previous_plane[0] == 0 {
            stone.indices.push(0);
            stone.indices.push(1);
            stone.indices.push(2);
        } else {
            for i in 1..points_of_plane {}
        }

        // prepare next plane

        previous_plane[0] = previous_plane[0] + 1;
        previous_plane[1] = previous_plane[2];
        previous_plane[2] = previous_plane[2] + points_of_plane - 1;

        let points_increase = rng.gen_range(3..8);
        if points_of_plane > 20 {
            points_of_plane = points_of_plane - points_increase;
        } else {
            points_of_plane = points_of_plane + points_increase;
        };

        if previous_plane[0] == planes_number - 1 {
            points_of_plane = 3;
        };

        if previous_plane[0] < planes_number / 2 {
            points_range = rng.gen_range(points_range / 2.0..points_range * 2.0);
            points_range_min = rng.gen_range(0.1..points_range);
        } else {
            points_range_min = rng.gen_range(0.1..points_range);
            points_range = rng.gen_range(points_range_min..points_range * 1.5);
        };
    }

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

pub fn gen_rthgnl_f32_3(a: &[f32; 3], rng: &mut ThreadRng) -> [f32; 3] {
    let x = rng.gen_range(0.0..1.0);
    let y = rng.gen_range(0.0..1.0);
    let z = ((-1.0 * a[0] * x) - (a[1] * y)) / a[2];

    return [x, y, z];
}
