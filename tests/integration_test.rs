//! Integration tests for geo-marching-squares-rs

use geo_marching_squares_rs::{GeoGrid, GridPoint};

#[test]
fn test_simple_isobands() {
    // Create a simple 3x3 grid with values ranging from 10 to 30
    let points = vec![
        vec![
            GridPoint::new(-100.0, 42.0, 10.0),
            GridPoint::new(-99.0, 42.0, 15.0),
            GridPoint::new(-98.0, 42.0, 20.0),
        ],
        vec![
            GridPoint::new(-100.0, 41.0, 12.0),
            GridPoint::new(-99.0, 41.0, 18.0),
            GridPoint::new(-98.0, 41.0, 25.0),
        ],
        vec![
            GridPoint::new(-100.0, 40.0, 15.0),
            GridPoint::new(-99.0, 40.0, 22.0),
            GridPoint::new(-98.0, 40.0, 30.0),
        ],
    ];

    let grid = GeoGrid::from_points(points).expect("Failed to create grid");

    // Generate isobands
    let isobands = grid.isobands(&[10.0, 15.0, 20.0, 25.0, 30.0]).expect("Failed to generate isobands");

    // Should have 4 isobands for 5 thresholds
    assert_eq!(isobands.len(), 4);

    // Check that each isoband has properties
    for feature in &isobands {
        assert!(feature.properties.is_some());
        let props = feature.properties.as_ref().unwrap();
        assert!(props.contains_key("lower_level"));
        assert!(props.contains_key("upper_level"));
    }
}

#[test]
fn test_simple_isolines() {
    // Create a simple grid
    let points = vec![
        vec![
            GridPoint::new(-100.0, 41.0, 10.0),
            GridPoint::new(-99.0, 41.0, 20.0),
            GridPoint::new(-98.0, 41.0, 30.0),
        ],
        vec![
            GridPoint::new(-100.0, 40.0, 15.0),
            GridPoint::new(-99.0, 40.0, 25.0),
            GridPoint::new(-98.0, 40.0, 35.0),
        ],
    ];

    let grid = GeoGrid::from_points(points).expect("Failed to create grid");

    // Generate isolines
    let isolines = grid.isolines(&[15.0, 20.0, 25.0]).expect("Failed to generate isolines");

    // Should have 3 isolines
    assert_eq!(isolines.len(), 3);

    // Check that each isoline has the isovalue property
    for (i, feature) in isolines.iter().enumerate() {
        assert!(feature.properties.is_some());
        let props = feature.properties.as_ref().unwrap();
        assert!(props.contains_key("isovalue"));

        // Verify the isovalue matches what we requested
        let isovalue = props.get("isovalue").unwrap().as_f64().unwrap();
        let expected = [15.0, 20.0, 25.0][i];
        assert_eq!(isovalue, expected);
    }
}

#[test]
fn test_meteorological_grid() {
    // Simulate a small meteorological grid (like HRRR data)
    // Values represent temperature in Celsius, for example
    let rows = 10;
    let cols = 10;

    let mut points = Vec::new();
    for row in 0..rows {
        let mut row_points = Vec::new();
        for col in 0..cols {
            let lon = -100.0 + (col as f64) * 0.1;
            let lat = 40.0 + (row as f64) * 0.1;
            // Create a gradient with some variation
            let value = 10.0 + (row as f32) * 2.0 + (col as f32) * 1.5;
            row_points.push(GridPoint::new(lon, lat, value));
        }
        points.push(row_points);
    }

    let grid = GeoGrid::from_points(points).expect("Failed to create grid");

    // Test grid properties
    assert_eq!(grid.rows(), rows);
    assert_eq!(grid.cols(), cols);

    let (min_lon, min_lat, max_lon, max_lat) = grid.bounds();
    assert_eq!(min_lon, -100.0);
    assert_eq!(min_lat, 40.0);
    assert!((max_lon - (-99.1)).abs() < 0.001);
    assert!((max_lat - 40.9).abs() < 0.001);

    // Generate temperature isobands at 5-degree intervals
    let thresholds = vec![10.0, 15.0, 20.0, 25.0, 30.0, 35.0];
    let isobands = grid.isobands(&thresholds).expect("Failed to generate isobands");

    // Should generate n-1 bands for n thresholds
    assert_eq!(isobands.len(), thresholds.len() - 1);

    // All features should have geometry
    for feature in &isobands {
        assert!(feature.geometry.is_some());
    }
}

#[test]
fn test_edge_cases() {
    // Test minimum grid size (2x2)
    let points = vec![
        vec![
            GridPoint::new(-100.0, 41.0, 10.0),
            GridPoint::new(-99.0, 41.0, 20.0),
        ],
        vec![
            GridPoint::new(-100.0, 40.0, 15.0),
            GridPoint::new(-99.0, 40.0, 25.0),
        ],
    ];

    let grid = GeoGrid::from_points(points).expect("Failed to create 2x2 grid");

    // Should work with minimum size
    let isobands = grid.isobands(&[12.0, 18.0, 22.0]).expect("Failed on 2x2 grid");
    assert_eq!(isobands.len(), 2);
}

#[test]
fn test_uniform_grid() {
    // Test grid with all same values (no contours should be generated)
    let points = vec![
        vec![
            GridPoint::new(-100.0, 41.0, 20.0),
            GridPoint::new(-99.0, 41.0, 20.0),
        ],
        vec![
            GridPoint::new(-100.0, 40.0, 20.0),
            GridPoint::new(-99.0, 40.0, 20.0),
        ],
    ];

    let grid = GeoGrid::from_points(points).expect("Failed to create uniform grid");

    // Isolines at the exact value might produce something, but isolines outside shouldn't
    let isolines = grid.isolines(&[15.0, 25.0]).expect("Failed on uniform grid");

    // The implementation might return empty features or no features
    // Just verify it doesn't crash
    assert!(isolines.len() <= 2);
}

#[test]
fn test_custom_config() {
    let points = vec![
        vec![
            GridPoint::new(-100.0, 41.0, 10.0),
            GridPoint::new(-99.0, 41.0, 20.0),
        ],
        vec![
            GridPoint::new(-100.0, 40.0, 15.0),
            GridPoint::new(-99.0, 40.0, 25.0),
        ],
    ];

    let mut grid = GeoGrid::from_points(points).expect("Failed to create grid");

    // Modify smoothing factor
    grid.config_mut().smoothing_factor = 0.95;

    let isobands = grid.isobands(&[12.0, 18.0, 22.0]).expect("Failed with custom config");
    assert_eq!(isobands.len(), 2);
}
