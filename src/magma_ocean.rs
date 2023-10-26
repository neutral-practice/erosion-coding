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
    pub positions: Vec<Position>,
    pub normals: Vec<Normal>,
    pub indices: Vec<u32>,
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
                position: [50.0, 50.0, 50.0],
            },
            Position {
                position: [100.0, 100.0, 100.0],
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

    let points_diff = sbtr_f32_3(flow.positions[1].position, flow.positions[0].position);

    let planes_normal: [f32; 3] = nrmlz_f32_3(points_diff);

    let mut rng = rand::thread_rng();

    let planes_number = rng.gen_range(15..30);

    let outer_planes = planes_number / 8;

    let total_planes_number = planes_number + outer_planes;

    let mut planes_points = Vec::new();

    for i in 1..=(total_planes_number) {
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

    // let mut angles: Vec<f32> = vec![];
    let mut previous_plane: [u32; 3] = [0, 0, 0]; // plane number, beginning position, ending position
    let mut points_of_plane: u32 = 3;
    let mut points_range = 10.0;
    let mut points_range_min = 3.0;
    let reference_orthogonal = gen_rthgnl_f32_3(&planes_normal, &mut rng);

    for planae in planes_points.iter() {
        let mut plane = Stone {
            positions: vec![],
            normals: vec![],
            indices: vec![],
        };

        let mut k: usize = 1;
        if previous_plane[0] > 0 {
            k = previous_plane[0] as usize;
        };

        // get random points on plane in range

        for i in 1..=points_of_plane {
            let random_vector_on_plane = gen_rthgnl_f32_3(&planes_normal, &mut rng);
            let random_range = rng.gen_range(points_range_min..points_range);
            let random_point_on_plane = dd_f32_3(
                mltply_f32_3(random_vector_on_plane, random_range),
                planes_points[k],
            );
            plane.positions.push(Position {
                position: random_point_on_plane,
            });
        }

        println!("New circle will contain + {} points", plane.positions.len());

        // order points on plane by angle

        plane.positions.sort_by(|a, b| {
            angle_of(*planae, a.position, reference_orthogonal).total_cmp(&angle_of(
                *planae,
                b.position,
                reference_orthogonal,
            ))
        });

        // add points to stone

        for i in 0..points_of_plane {
            stone.positions.push(Position {
                position: plane.positions[(i as usize)].position,
            });
            let normal =
                find_points_normal(plane.positions[(i as usize)].position, planes_points[k]);
            stone.normals.push(Normal { normal: normal });
        }

        println!("Stone points number: {:#?}", stone.positions.len());

        // fill indices

        if previous_plane[0] == 0 {
            stone.indices.push(0);
            stone.indices.push(1);
            stone.indices.push(2);
        } else {
            let points_of_previous_plane = previous_plane[2] - previous_plane[1] + 1;
            println!(
                "Previous plane contained {} points",
                points_of_previous_plane
            );
            println!("Current plane contains {} points", points_of_plane);

            for i in previous_plane[1] - 1..previous_plane[2] - 1 {
                // FULL CIRCLE MISSING
                let mut a_min = f32::MAX;
                let mut a_min_dex = 0;

                for j in 1..=points_of_plane {
                    println!(
                        "Finding distance between point number {}, {} and {}",
                        i,
                        i + 1,
                        previous_plane[2] + j - 1
                    );
                    let dist = dstnc_f32_3(
                        stone.positions[(i as usize)].position,
                        stone.positions[((previous_plane[2] + j - 1) as usize)].position,
                    ) + dstnc_f32_3(
                        stone.positions[((i + 1) as usize)].position,
                        stone.positions[((previous_plane[2] + j - 1) as usize)].position,
                    );
                    if dist < a_min {
                        a_min = dist;
                        a_min_dex = previous_plane[2] + j - 1;
                    }
                }
                stone.indices.push(i);
                stone.indices.push(i + 1);
                stone.indices.push(a_min_dex);
            }

            // FULL CIRCLE for previous circle
            let mut min_prev_last = f32::MAX;
            let mut min_prev_last_dex = 0;
            for j in previous_plane[2]..=previous_plane[2] - 1 + points_of_plane {
                println!(
                    "Finding distance between point number {}, {} and {}",
                    j,
                    previous_plane[1] - 1,
                    previous_plane[2] - 1,
                );
                let dist = dstnc_f32_3(
                    stone.positions[(j as usize)].position,
                    stone.positions[((previous_plane[1] - 1) as usize)].position,
                ) + dstnc_f32_3(
                    stone.positions[(j as usize)].position,
                    stone.positions[((previous_plane[2] - 1) as usize)].position,
                );
                if dist < min_prev_last {
                    min_prev_last = dist;
                    min_prev_last_dex = j;
                }
            }
            stone.indices.push(min_prev_last_dex);
            stone.indices.push(previous_plane[1] - 1);
            stone.indices.push(previous_plane[2] - 1);

            for i in 0..points_of_plane - 1 {
                // FULL CIRCLE MISSING
                let current_point_index = previous_plane[2] + i;

                // get closest points from last plane

                let mut min = f32::MAX;
                let mut min_dex = 0;
                for j in (previous_plane[1] - 1)..=(previous_plane[2] - 1) {
                    println!(
                        "Finding distance between point number {}, {} and {}",
                        j,
                        current_point_index,
                        current_point_index + 1,
                    );
                    let dist = dstnc_f32_3(
                        stone.positions[(j as usize)].position,
                        stone.positions[(current_point_index as usize)].position,
                    ) + dstnc_f32_3(
                        stone.positions[(j as usize)].position,
                        stone.positions[((current_point_index + 1) as usize)].position,
                    );
                    if dist < min {
                        min = dist;
                        min_dex = j;
                    }
                }
                stone.indices.push(min_dex);
                stone.indices.push(current_point_index);
                stone.indices.push(current_point_index + 1);
            }
            let mut min_last = f32::MAX;
            let mut min_last_dex = 0;
            for j in previous_plane[1] - 1..=previous_plane[2] - 1 {
                // FULL CIRCLE FOR CURRENT CIRCLE
                println!(
                    "Finding distance between point number {}, {} and {}",
                    j,
                    previous_plane[2] + points_of_plane - 1,
                    previous_plane[2],
                );
                let dist = dstnc_f32_3(
                    stone.positions[(j as usize)].position,
                    stone.positions[((previous_plane[2] + points_of_plane - 1) as usize)].position,
                ) + dstnc_f32_3(
                    stone.positions[(j as usize)].position,
                    stone.positions[((previous_plane[2]) as usize)].position,
                );
                if dist < min_last {
                    min_last = dist;
                    min_last_dex = j;
                }
            }
            stone.indices.push(min_last_dex);
            stone.indices.push(previous_plane[2] + points_of_plane);
            stone.indices.push(previous_plane[2] + 1);
        }

        // prepare next plane

        previous_plane[0] = previous_plane[0] + 1;
        previous_plane[1] = previous_plane[2] + 1;
        previous_plane[2] = previous_plane[2] + points_of_plane;

        let points_increase = rng.gen_range(5..16);
        if points_of_plane > 30 {
            points_of_plane = points_of_plane - points_increase;
        } else {
            points_of_plane = points_of_plane + points_increase;
        };

        if previous_plane[0] == total_planes_number - 1 {
            points_of_plane = 3;
        };

        if previous_plane[0] < total_planes_number / 2 {
            points_range = rng.gen_range(points_range / 2.0..points_range * 2.0);
            points_range_min = rng.gen_range(points_range / 3.0..points_range);
        } else {
            points_range = rng.gen_range(points_range_min / 3.0..points_range);
            points_range_min = rng.gen_range(points_range / 3.0..points_range);
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

pub fn dstnc_f32_3(a: [f32; 3], b: [f32; 3]) -> f32 {
    return ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt();
}

pub fn vector_length(x: [f32; 3]) -> f32 {
    return (x[0].powi(2) + x[1].powi(2) + x[2].powi(2)).sqrt();
}

pub fn gen_rthgnl_f32_3(a: &[f32; 3], rng: &mut ThreadRng) -> [f32; 3] {
    let x = rng.gen_range(0.0..1.0);
    let y = rng.gen_range(0.0..1.0);
    let z = ((-1.0 * a[0] * x) - (a[1] * y)) / a[2];

    return [x, y, z];
}

pub fn find_longitudinal_plane_normal(c: [f32; 3], x: [f32; 3], y: [f32; 3]) -> ([f32; 3]) {
    return nrmlz_f32_3(dd_f32_3(sbtr_f32_3(x, c), sbtr_f32_3(y, c)));
}

pub fn find_points_normal(x: [f32; 3], y: [f32; 3]) -> [f32; 3] {
    return nrmlz_f32_3(sbtr_f32_3(x, y));
}

pub fn dot_product(a: [f32; 3], b: [f32; 3]) -> f32 {
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
}

pub fn angle_of(c: [f32; 3], x: [f32; 3], r: [f32; 3]) -> f32 {
    // angle of point x compared to center and common random comparison vector
    let vector = find_points_normal(x, c);
    let angle = (dot_product(vector, r) / (vector_length(vector) * vector_length(r))).acos();
    return angle;
}
