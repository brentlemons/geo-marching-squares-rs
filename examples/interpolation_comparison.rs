/// Example demonstrating different interpolation methods
///
/// This example compares cosine interpolation (default, fast) with
/// great circle interpolation (more accurate for large distances).

use geo_marching_squares_rs::{GeoGrid, GridPoint, MarchingSquaresConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Interpolation Method Comparison\n");
    println!("================================\n");

    // Create a simple 3x3 grid
    let points = vec![
        vec![
            GridPoint::new(-100.0, 41.0, 10.0),
            GridPoint::new(-99.0, 41.0, 15.0),
            GridPoint::new(-98.0, 41.0, 20.0),
        ],
        vec![
            GridPoint::new(-100.0, 40.0, 12.0),
            GridPoint::new(-99.0, 40.0, 18.0),
            GridPoint::new(-98.0, 40.0, 22.0),
        ],
        vec![
            GridPoint::new(-100.0, 39.0, 8.0),
            GridPoint::new(-99.0, 39.0, 14.0),
            GridPoint::new(-98.0, 39.0, 16.0),
        ],
    ];

    // Generate isolines with default (cosine) interpolation
    println!("1. Cosine Interpolation (Default)");
    println!("   - Fast and accurate for typical grid spacings (3-10km)");
    println!("   - Uses cosine smoothing with center bias\n");

    let grid_cosine = GeoGrid::from_points(points.clone())?;
    let levels = vec![12.0, 15.0, 18.0];
    let isolines_cosine = grid_cosine.isolines(&levels)?;
    println!("   Generated {} isolines", isolines_cosine.len());

    // Generate isolines with great circle interpolation
    println!("\n2. Great Circle Interpolation");
    println!("   - More accurate for large distances or polar regions");
    println!("   - Slower due to spherical calculations\n");

    let config = MarchingSquaresConfig::with_great_circle();
    let grid_gc = GeoGrid::from_points_with_config(points, config)?;

    let isolines_gc = grid_gc.isolines(&levels)?;
    println!("   Generated {} isolines", isolines_gc.len());

    // For typical grid spacing, the results are very similar
    println!("\n3. Comparison");
    println!("   For small distances (typical meteorological grids):");
    println!("   - Difference is typically < 1 meter");
    println!("   - Cosine is recommended for performance");
    println!("\n   Use great circle when:");
    println!("   - Grid spacing > 100km");
    println!("   - Working near poles");
    println!("   - Extreme accuracy requirements");

    println!("\nNote: Both methods produce topologically correct contours.");
    println!("The choice affects only the precise position of interpolated points.\n");

    Ok(())
}
