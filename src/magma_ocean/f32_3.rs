use rand::rngs::ThreadRng;
use rand::Rng;

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
