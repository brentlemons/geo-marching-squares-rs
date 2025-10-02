//! Polygon utility functions for hole detection and nesting
//!
//! Implements point-in-polygon testing and polygon nesting organization

use crate::types::Point;

/// Test if a point is inside a polygon using ray casting algorithm
///
/// Based on: http://www.ecse.rpi.edu/Homepages/wrf/Research/Short_Notes/pnpoly.html
pub fn point_in_polygon(point: &Point, polygon: &[Point]) -> bool {
    let mut inside = false;
    let n = polygon.len();

    let mut j = n - 1;
    for i in 0..n {
        let pi = &polygon[i];
        let pj = &polygon[j];

        if ((pi.y > point.y) != (pj.y > point.y))
            && (point.x < (pj.x - pi.x) * (point.y - pi.y) / (pj.y - pi.y) + pi.x)
        {
            inside = !inside;
        }
        j = i;
    }

    inside
}

/// Test if all points of subject polygon are inside the test polygon
pub fn polygon_in_polygon(subject: &[Point], polygon: &[Point]) -> bool {
    if subject.is_empty() || polygon.is_empty() {
        return false;
    }

    // All points of subject must be inside polygon
    for point in subject {
        if !point_in_polygon(point, polygon) {
            return false;
        }
    }

    true
}

/// Organize a list of polygon rings into properly nested structures
///
/// Returns Vec<(exterior_ring, Vec<interior_rings>)>
pub fn organize_polygons(mut rings: Vec<Vec<Point>>) -> Vec<(Vec<Point>, Vec<Vec<Point>>)> {
    let mut result: Vec<(Vec<Point>, Vec<Vec<Point>>)> = Vec::new();

    while !rings.is_empty() {
        let subject = rings.remove(0);
        let mut found_parent = false;

        // Check if this polygon is inside any existing polygon
        for (exterior, interior_rings) in result.iter_mut() {
            if polygon_in_polygon(&subject, exterior) {
                // Check if it's inside any of the interior rings (holes)
                let mut inside_hole = false;
                for hole in interior_rings.iter() {
                    if polygon_in_polygon(&subject, hole) {
                        inside_hole = true;
                        break;
                    }
                }

                if !inside_hole {
                    // It's a hole in the exterior polygon
                    interior_rings.push(subject.clone());
                    found_parent = true;
                    break;
                }
            }
        }

        if found_parent {
            continue;
        }

        // Check if any existing polygons should be inside this one
        let mut i = 0;
        while i < result.len() {
            let (existing_exterior, _existing_holes) = &result[i];

            if polygon_in_polygon(existing_exterior, &subject) {
                // This existing polygon should be a child of subject
                // Remove it and we'll re-process
                let removed = result.remove(i);

                // Put the exterior back into rings for reprocessing
                rings.push(removed.0);

                // Put all its holes back too
                for hole in removed.1 {
                    rings.push(hole);
                }

                // Don't increment i since we removed an element
            } else {
                i += 1;
            }
        }

        // Add as a new exterior polygon
        result.push((subject, Vec::new()));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_in_simple_square() {
        let square = vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(1.0, 1.0),
            Point::new(0.0, 1.0),
        ];

        // Inside
        assert!(point_in_polygon(&Point::new(0.5, 0.5), &square));

        // Outside
        assert!(!point_in_polygon(&Point::new(1.5, 0.5), &square));
        assert!(!point_in_polygon(&Point::new(-0.5, 0.5), &square));

        // On edge (algorithm may vary)
        let _on_edge = point_in_polygon(&Point::new(1.0, 0.5), &square);
        // Don't assert specific behavior for edge cases
    }

    #[test]
    fn test_point_in_triangle() {
        let triangle = vec![
            Point::new(0.0, 0.0),
            Point::new(2.0, 0.0),
            Point::new(1.0, 2.0),
        ];

        // Inside
        assert!(point_in_polygon(&Point::new(1.0, 0.5), &triangle));

        // Outside
        assert!(!point_in_polygon(&Point::new(0.0, 1.5), &triangle));
        assert!(!point_in_polygon(&Point::new(2.0, 1.5), &triangle));
    }

    #[test]
    fn test_polygon_in_polygon_basic() {
        let outer = vec![
            Point::new(0.0, 0.0),
            Point::new(4.0, 0.0),
            Point::new(4.0, 4.0),
            Point::new(0.0, 4.0),
        ];

        let inner = vec![
            Point::new(1.0, 1.0),
            Point::new(2.0, 1.0),
            Point::new(2.0, 2.0),
            Point::new(1.0, 2.0),
        ];

        assert!(polygon_in_polygon(&inner, &outer));
        assert!(!polygon_in_polygon(&outer, &inner));
    }

    #[test]
    fn test_organize_simple() {
        let ring1 = vec![
            Point::new(0.0, 0.0),
            Point::new(4.0, 0.0),
            Point::new(4.0, 4.0),
            Point::new(0.0, 4.0),
        ];

        let ring2 = vec![
            Point::new(10.0, 10.0),
            Point::new(14.0, 10.0),
            Point::new(14.0, 14.0),
            Point::new(10.0, 14.0),
        ];

        let rings = vec![ring1.clone(), ring2.clone()];
        let organized = organize_polygons(rings);

        // Should have 2 separate polygons with no holes
        assert_eq!(organized.len(), 2);
        assert_eq!(organized[0].1.len(), 0);
        assert_eq!(organized[1].1.len(), 0);
    }

    #[test]
    fn test_organize_with_hole() {
        let outer = vec![
            Point::new(0.0, 0.0),
            Point::new(10.0, 0.0),
            Point::new(10.0, 10.0),
            Point::new(0.0, 10.0),
        ];

        let hole = vec![
            Point::new(2.0, 2.0),
            Point::new(8.0, 2.0),
            Point::new(8.0, 8.0),
            Point::new(2.0, 8.0),
        ];

        let rings = vec![outer.clone(), hole.clone()];
        let organized = organize_polygons(rings);

        // Should have 1 polygon with 1 hole
        assert_eq!(organized.len(), 1);
        assert_eq!(organized[0].1.len(), 1);
    }

    #[test]
    fn test_organize_nested() {
        // Large outer square
        let outer = vec![
            Point::new(0.0, 0.0),
            Point::new(20.0, 0.0),
            Point::new(20.0, 20.0),
            Point::new(0.0, 20.0),
        ];

        // Hole in outer
        let hole = vec![
            Point::new(5.0, 5.0),
            Point::new(15.0, 5.0),
            Point::new(15.0, 15.0),
            Point::new(5.0, 15.0),
        ];

        // Filled area inside the hole (island)
        let island = vec![
            Point::new(8.0, 8.0),
            Point::new(12.0, 8.0),
            Point::new(12.0, 12.0),
            Point::new(8.0, 12.0),
        ];

        let rings = vec![outer.clone(), hole.clone(), island.clone()];
        let organized = organize_polygons(rings);

        // Should have 2 top-level polygons: outer (with hole) and island (no holes)
        assert_eq!(organized.len(), 2);

        // One should have a hole, one shouldn't
        let with_holes: Vec<_> = organized.iter().filter(|(_, h)| !h.is_empty()).collect();
        let without_holes: Vec<_> = organized.iter().filter(|(_, h)| h.is_empty()).collect();

        assert_eq!(with_holes.len(), 1);
        assert_eq!(without_holes.len(), 1);
        assert_eq!(with_holes[0].1.len(), 1);
    }
}
