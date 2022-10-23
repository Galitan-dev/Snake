pub fn dist(a: [usize; 2], b: [usize; 2]) -> usize {
    let dist_x = a[0].abs_diff(b[0]) as f32;
    let dist_y = a[1].abs_diff(b[1]) as f32;

    (dist_x.powi(2) + dist_y.powi(2)).sqrt() as usize
}