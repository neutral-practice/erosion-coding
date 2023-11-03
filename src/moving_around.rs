use crate::f32_3::{
    angle_360_of, angle_of, angular_difference, average_f32_2, dd_f32_3, dot_product, dstnc_f32_3,
    find_points_normal, gen_f32_3, gen_f32_3_on_point_normal_plane, gen_rthgnl_f32_3, mltply_f32_3,
    nrmlz_f32_3, sbtr_f32_3, vector_length,
};

use crate::magma_ocean::Position;

// mod u_modular;
// use u_modular::{
//     modular_difference, modular_difference_in_range, modular_offset, modular_offset_in_range,
// };
pub fn move_forwards(
    view_point: &mut Position,
    center: &mut Position,
    up_direction: &mut Position,
    rate: f32,
) {
    let direction = mltply_f32_3(
        find_points_normal(view_point.position, center.position),
        rate,
    );
    view_point.position = dd_f32_3(view_point.position, direction);
    center.position = dd_f32_3(center.position, direction);
    // up_direction.position = dd_f32_3(center.position, direction);
}
