use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use geo_marching_squares_rs::{GeoGrid, GridPoint};

/// Generate a synthetic grid for benchmarking
fn generate_grid(rows: usize, cols: usize) -> Vec<Vec<GridPoint>> {
    let mut grid = Vec::with_capacity(rows);

    for r in 0..rows {
        let mut row = Vec::with_capacity(cols);
        for c in 0..cols {
            let lon = -100.0 + (c as f64 * 0.1);
            let lat = 40.0 + (r as f64 * 0.1);

            // Create interesting terrain with peaks and valleys
            let x = c as f64 / cols as f64;
            let y = r as f64 / rows as f64;
            let value = (50.0 * (x * std::f64::consts::PI * 3.0).sin()
                + 30.0 * (y * std::f64::consts::PI * 2.0).cos()
                + 20.0 * ((x + y) * std::f64::consts::PI * 4.0).sin()) as f32;

            row.push(GridPoint { lon, lat, value });
        }
        grid.push(row);
    }

    grid
}

/// Benchmark isoline generation
fn bench_isolines(c: &mut Criterion) {
    let mut group = c.benchmark_group("isolines");

    for &size in &[50, 100, 200] {
        let grid_data = generate_grid(size, size);
        let grid = GeoGrid::from_points(grid_data).unwrap();
        let levels = vec![-20.0, 0.0, 20.0, 40.0, 60.0];

        group.throughput(Throughput::Elements((size * size) as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, _| {
            b.iter(|| {
                black_box(grid.isolines(&levels).unwrap());
            });
        });
    }

    group.finish();
}

/// Benchmark isoband generation
fn bench_isobands(c: &mut Criterion) {
    let mut group = c.benchmark_group("isobands");

    for &size in &[50, 100, 200] {
        let grid_data = generate_grid(size, size);
        let grid = GeoGrid::from_points(grid_data).unwrap();
        let thresholds = vec![-40.0, -20.0, 0.0, 20.0, 40.0, 60.0];

        group.throughput(Throughput::Elements((size * size) as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, _| {
            b.iter(|| {
                black_box(grid.isobands(&thresholds).unwrap());
            });
        });
    }

    group.finish();
}

/// Benchmark HRRR-sized grid (1799x1059)
fn bench_hrrr_size(c: &mut Criterion) {
    let mut group = c.benchmark_group("hrrr_realistic");
    group.sample_size(10); // Fewer samples for large dataset

    // Full HRRR size would be 1799x1059, but that's too large for quick benchmarking
    // Use a scaled-down version that's still representative
    let rows = 360;
    let cols = 212;

    let grid_data = generate_grid(rows, cols);
    let grid = GeoGrid::from_points(grid_data).unwrap();
    let thresholds = vec![-40.0, -20.0, 0.0, 20.0, 40.0, 60.0, 80.0, 100.0];

    group.throughput(Throughput::Elements((rows * cols) as u64));
    group.bench_function("isobands_scaled_hrrr", |b| {
        b.iter(|| {
            black_box(grid.isobands(&thresholds).unwrap());
        });
    });

    group.finish();
}

/// Benchmark interpolation (the hot path)
fn bench_interpolation(c: &mut Criterion) {
    use geo_marching_squares_rs::interpolation::interpolate_point;
    use geo_marching_squares_rs::Point;

    let mut group = c.benchmark_group("interpolation");

    let p0 = Point::new(-100.0, 40.0);
    let p1 = Point::new(-99.0, 40.0);
    let v0 = 10.0;
    let v1 = 20.0;
    let level = 15.0;
    let smoothing = 0.999;

    group.bench_function("cosine_interpolation", |b| {
        b.iter(|| {
            black_box(interpolate_point(
                black_box(level),
                black_box(v0),
                black_box(v1),
                black_box(&p0),
                black_box(&p1),
                black_box(smoothing),
            ));
        });
    });

    group.finish();
}

/// Benchmark edge tracing performance
fn bench_edge_tracing(c: &mut Criterion) {
    let mut group = c.benchmark_group("edge_tracing");

    // Create a grid with complex contours (multiple nested rings)
    let size = 100;
    let grid_data = generate_grid(size, size);
    let grid = GeoGrid::from_points(grid_data).unwrap();

    // Use thresholds that will create complex nested polygons
    let thresholds = vec![-50.0, -30.0, -10.0, 10.0, 30.0, 50.0, 70.0];

    group.throughput(Throughput::Elements((size * size) as u64));
    group.bench_function("complex_nesting", |b| {
        b.iter(|| {
            black_box(grid.isobands(&thresholds).unwrap());
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_isolines,
    bench_isobands,
    bench_hrrr_size,
    bench_interpolation,
    bench_edge_tracing
);
criterion_main!(benches);
