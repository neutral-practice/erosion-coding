use rand::rngs::ThreadRng;
use rand::Rng;
use std::thread;
use std::time::{Duration, SystemTime};
use vulkano::{buffer::BufferContents, pipeline::graphics::vertex_input::Vertex};

mod f32_3;
use f32_3::{
    angle_360_of, angle_of, angular_difference, average_f32_2, dd_f32_3, dot_product, dstnc_f32_3,
    find_points_normal, gen_f32_3, gen_f32_3_on_point_normal_plane, gen_rthgnl_f32_3, mltply_f32_3,
    nrmlz_f32_3, sbtr_f32_3, vector_length,
};

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

pub fn magma(flow: u32, scale: f32) -> Magma {
    let mut rng = rand::thread_rng();

    let mut lava_flow = Magma {
        positions: vec![],
        normals: vec![],
        indices: vec![],
    };

    let mut base = scale;
    let mut cbase = -scale;
    for i in 1..=flow {
        lava_flow.positions.push(Position {
            position: gen_f32_3(cbase, base, &mut rng),
        });
        cbase = cbase + 3.0 * vector_length(lava_flow.positions[(i - 1) as usize].position);

        // randomize graph edges
        if i > 1 {
            lava_flow
                .indices
                .push(rng.gen_range(0..lava_flow.positions.len() - 1) as u32);
            lava_flow.indices.push(i - 1);
        }
    }

    println!("flow: {:#?}", lava_flow);

    return lava_flow;
}

pub fn petrify(flow: Magma) -> Stone {
    if flow.positions.len() > 2 {
        return petrify_flow(flow);
    };

    let mut stone = Stone {
        positions: vec![],
        normals: vec![],
        indices: vec![],
    };

    let mut rng = rand::thread_rng();
    let points_diff = sbtr_f32_3(flow.positions[1].position, flow.positions[0].position);
    let planes_normal: [f32; 3] = nrmlz_f32_3(points_diff);
    let planes_number = 4; // rng.gen_range(4..16);
    let outer_planes = 4; // planes_number / 8;
    let total_planes_number = planes_number + outer_planes;
    let mut planes_points = Vec::new();

    for i in 0..total_planes_number {
        planes_points.push(dd_f32_3(
            mltply_f32_3(
                points_diff,
                ((i as f32) - ((outer_planes / 2) as f32)) / (planes_number as f32),
            ),
            flow.positions[0].position,
        ));
    }

    // println!("{:#?}", planes_points);

    println!("normal: {:#?}", planes_normal);

    let mut previous_plane: [u32; 3] = [0, 0, 0]; // plane number, beginning position, ending position
    let mut points_of_plane: u32 = 3;
    let mut points_range = 50.0;
    let mut points_range_min = 20.0;
    let reference_orthogonal = gen_rthgnl_f32_3(planes_normal, &mut rng);
    let mut pln = 0;
    for planae in planes_points.iter() {
        println!("plane: {:#?}", planae);

        let mut plane = Stone {
            positions: vec![],
            normals: vec![],
            indices: vec![],
        };

        let mut k: usize = 1;
        if previous_plane[2] > 0 {
            k = previous_plane[0] as usize;
        };

        // get random points on plane in range

        for i in 1..=points_of_plane {
            plane.positions.push(Position {
                position: gen_f32_3_on_point_normal_plane(
                    planes_normal,
                    points_range_min,
                    points_range,
                    *planae,
                    &mut rng,
                ),
            });
        }

        // order points on plane by angle

        plane.positions.sort_by(|a, b| {
            angle_360_of(*planae, a.position, reference_orthogonal, planes_normal).total_cmp(
                &angle_360_of(*planae, b.position, reference_orthogonal, planes_normal),
            )
        });

        // add points and normals to stone

        for i in 0..points_of_plane {
            stone.positions.push(Position {
                position: plane.positions[(i as usize)].position,
            });
            let normal =
                find_points_normal(plane.positions[(i as usize)].position, planes_points[k]);
            stone.normals.push(Normal { normal: normal });
        }

        // add indices

        if previous_plane[2] == 0 {
            stone.indices.push(0);
            stone.indices.push(1);
            stone.indices.push(2);
        } else {
            find_indices_between_circles(
                //vertex_plane_one: [u32; 2],
                [previous_plane[1], previous_plane[2] - 1],
                //plane_one: [f32; 3],
                planes_points[(previous_plane[0] as usize)],
                //vertex_plane_two: [u32; 2],
                [previous_plane[2], previous_plane[2] + points_of_plane - 1],
                //plane_two: [f32; 3],
                *planae,
                //reference_orthogonal: [f32; 3],
                reference_orthogonal,
                //planes_normal: [f32;3],
                planes_normal,
                //&mut stone: Stone,
                &mut stone,
            );
        };
        if previous_plane[0] == total_planes_number - 2 {
            stone.indices.push((stone.positions.len() - 3) as u32);
            stone.indices.push((stone.positions.len() - 2) as u32);
            stone.indices.push((stone.positions.len() - 1) as u32);
        }

        // prepare next plane

        previous_plane[0] = pln;
        pln = pln + 1;
        previous_plane[1] = previous_plane[2];
        previous_plane[2] = previous_plane[2] + points_of_plane;

        let points_increase = rng.gen_range(1..8);
        if points_of_plane > 24 {
            points_of_plane = points_of_plane - points_increase;
        } else {
            points_of_plane = points_of_plane + points_increase;
        };

        if previous_plane[0] == total_planes_number - 2 {
            points_of_plane = 3;
        };

        if previous_plane[0] < total_planes_number / 2 {
            points_range = points_range + rng.gen_range(0.1..1.0);
            points_range_min = rng.gen_range(points_range_min..points_range - 2.0);
        } else {
            points_range = points_range - rng.gen_range(0.1..1.0);
            points_range_min = rng.gen_range(points_range_min - 2.0..points_range - 2.0);
        };
    }

    return stone;
}

pub fn petrify_flow(flow: Magma) -> Stone {
    return Stone {
        positions: vec![],
        normals: vec![],
        indices: vec![],
    };
}

pub fn find_indices_between_circles(
    vertex_plane_one: [u32; 2],
    plane_one: [f32; 3],
    vertex_plane_two: [u32; 2],
    plane_two: [f32; 3],
    reference_orthogonal: [f32; 3],
    planes_normal: [f32; 3],
    stone: &mut Stone,
) {
    find_indices_double_circle(
        vertex_plane_one,
        plane_one,
        vertex_plane_two,
        plane_two,
        reference_orthogonal,
        planes_normal,
        stone,
    );
    find_indices_double_circle(
        vertex_plane_two,
        plane_two,
        vertex_plane_one,
        plane_one,
        reference_orthogonal,
        planes_normal,
        stone,
    );
}

pub fn find_indices_double_circle(
    single_vertex_plane: [u32; 2],
    single_plane_point: [f32; 3],
    double_vertex_plane: [u32; 2],
    double_plane_point: [f32; 3],
    reference_orthogonal: [f32; 3],
    planes_normal: [f32; 3],
    stone: &mut Stone,
) {
    let points_of_previous_plane = single_vertex_plane[1] - single_vertex_plane[0] + 1;
    println!(
        "Previous plane contained {} points",
        points_of_previous_plane
    );

    let points_of_plane = double_vertex_plane[1] - double_vertex_plane[0] + 1;
    println!("Current plane contains {} points", points_of_plane);

    for i in double_vertex_plane[0]..double_vertex_plane[1] {
        // FULL CIRCLE MISSING
        let mut a_min = f32::MAX;
        let mut a_min_dex = 0;

        let average_point = average_f32_2(vec![
            stone.positions[(i as usize)].position,
            stone.positions[((i + 1) as usize)].position,
        ]);

        for j in single_vertex_plane[0]..=single_vertex_plane[1] {
            println!(
                "Finding distance between point number {}, {} and {}",
                i,
                i + 1,
                j,
            );
            let dist = angular_difference(
                angle_360_of(
                    double_plane_point,
                    average_point,
                    reference_orthogonal,
                    planes_normal,
                ),
                angle_360_of(
                    single_plane_point,
                    stone.positions[(j as usize)].position,
                    reference_orthogonal,
                    planes_normal,
                ),
            );
            // let dist = dstnc_f32_3(
            //     stone.positions[(i as usize)].position,
            //     stone.positions[(j as usize)].position,
            // ) + dstnc_f32_3(
            //     stone.positions[((i + 1) as usize)].position,
            //     stone.positions[(j - 1 as usize)].position,
            // );
            if dist < a_min {
                a_min = dist;
                a_min_dex = j;
            }
        }
        stone.indices.push(i);
        stone.indices.push(i + 1);
        stone.indices.push(a_min_dex);
    }

    // FULL CIRCLE for previous circle
    let mut min_prev_last = f32::MAX;
    let mut min_prev_last_dex = 0;
    let average_point = average_f32_2(vec![
        stone.positions[double_vertex_plane[1] as usize].position,
        stone.positions[double_vertex_plane[0] as usize].position,
    ]);

    for j in single_vertex_plane[0]..=single_vertex_plane[1] {
        println!(
            "Finding distance between point number {}, {} and {}",
            j, double_vertex_plane[0], double_vertex_plane[1],
        );
        let dist = angular_difference(
            angle_360_of(
                double_plane_point,
                average_point,
                reference_orthogonal,
                planes_normal,
            ),
            angle_360_of(
                single_plane_point,
                stone.positions[(j as usize)].position,
                reference_orthogonal,
                planes_normal,
            ),
        );

        if dist < min_prev_last {
            min_prev_last = dist;
            min_prev_last_dex = j;
        }
    }
    stone.indices.push(min_prev_last_dex);
    stone.indices.push(double_vertex_plane[0]);
    stone.indices.push(double_vertex_plane[1]);
}
