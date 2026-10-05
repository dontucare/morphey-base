use std::time::Instant;

fn main() {
    let points = vec![[0.0, 0.0]; 100]; // Smaller slice to represent typical polygon
    let iterations = 100000;

    // Baseline: Current behavior
    let start = Instant::now();
    for _ in 0..iterations {
        let points_vec: Vec<[f32; 2]> = points.to_vec();
        let p_clone = points_vec.clone();

        let mut points_out: Vec<[f32; 2]> = points.to_vec();
        points_out.push(points[0]);

        std::hint::black_box((p_clone, points_out));
    }
    let dur1 = start.elapsed();
    println!("Baseline ({} iter): {:?}", iterations, dur1);

    // Optimized v2
    let start = Instant::now();
    for _ in 0..iterations {
        let mut points_out: Vec<[f32; 2]> = Vec::with_capacity(points.len() + 1);
        points_out.extend_from_slice(&points);
        let p_clone = points_out.clone();

        points_out.push(points[0]);

        std::hint::black_box((p_clone, points_out));
    }
    let dur2 = start.elapsed();
    println!("Optimized ({} iter): {:?}", iterations, dur2);
}
