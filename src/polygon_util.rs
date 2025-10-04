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

        // CRITICAL FIX: Don't break early! Even if we found a parent for subject,
        // we must still check if subject contains any existing polygons.
        // This matches Java behavior and handles the case where a ring is both:
        // - A hole in a larger exterior (found_parent = true)
        // - A container for smaller existing rings (needs to trigger reprocessing)

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

        // Only add as a new exterior polygon if we didn't find a parent
        // (i.e., it wasn't added as a hole to an existing polygon)
        if !found_parent {
            result.push((subject, Vec::new()));
        }
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

    #[test]
    fn test_organize_ring_both_hole_and_container() {
        // This is the critical edge case that triggered the early break bug:
        // A ring that is BOTH a hole in a larger polygon AND contains a smaller polygon
        //
        // Scenario: Donut with a plug that itself has a hole
        // - Huge outer (B): 0,0 to 20,20
        // - Medium ring (C): 5,5 to 15,15 (hole in B, but contains A)
        // - Tiny ring (A): 9,9 to 10,10 (should be inside C)
        //
        // Processing order matters! If C is processed before A is properly nested,
        // the old code would find C is a hole in B, set found_parent=true, then
        // skip checking if C contains A.

        let tiny_a = vec![
            Point::new(9.0, 9.0),
            Point::new(10.0, 9.0),
            Point::new(10.0, 10.0),
            Point::new(9.0, 10.0),
        ];

        let huge_b = vec![
            Point::new(0.0, 0.0),
            Point::new(20.0, 0.0),
            Point::new(20.0, 20.0),
            Point::new(0.0, 20.0),
        ];

        let medium_c = vec![
            Point::new(5.0, 5.0),
            Point::new(15.0, 5.0),
            Point::new(15.0, 15.0),
            Point::new(5.0, 15.0),
        ];

        // Process in order: A, B, C
        // This is the problematic order where C finds parent B before checking if it contains A
        let rings = vec![tiny_a.clone(), huge_b.clone(), medium_c.clone()];
        let organized = organize_polygons(rings);

        // Expected result: B is exterior with C as hole, A is separate exterior
        // (because A is inside C which is a hole, making it pushed back out)
        assert_eq!(organized.len(), 2, "Should have 2 top-level polygons");

        // Find which polygon has a hole
        let with_hole = organized.iter().find(|(_, h)| !h.is_empty());
        let without_hole = organized.iter().find(|(_, h)| h.is_empty());

        assert!(with_hole.is_some(), "Should have one polygon with a hole");
        assert!(without_hole.is_some(), "Should have one polygon without holes");

        let (exterior_with_hole, holes) = with_hole.unwrap();
        assert_eq!(holes.len(), 1, "Should have exactly 1 hole");

        // The exterior with hole should be the huge B (area ~400)
        // Calculate approximate area to identify which polygon it is
        let area_b = 20.0 * 20.0; // 400
        let area_c = 10.0 * 10.0; // 100
        let area_a = 1.0 * 1.0;   // 1

        // Simple area calculation for the exterior
        let ext_area = (exterior_with_hole[2].x - exterior_with_hole[0].x)
            * (exterior_with_hole[2].y - exterior_with_hole[0].y);

        assert!(
            (ext_area - area_b).abs() < 1.0,
            "Exterior with hole should be huge B (area ~400), got area {}",
            ext_area
        );

        // The hole should be medium C (area ~100)
        let hole_area = (holes[0][2].x - holes[0][0].x) * (holes[0][2].y - holes[0][0].y);
        assert!(
            (hole_area - area_c).abs() < 1.0,
            "Hole should be medium C (area ~100), got area {}",
            hole_area
        );

        // The separate polygon should be tiny A
        let (separate_exterior, separate_holes) = without_hole.unwrap();
        assert_eq!(separate_holes.len(), 0, "Tiny A should have no holes");

        let sep_area =
            (separate_exterior[2].x - separate_exterior[0].x) * (separate_exterior[2].y - separate_exterior[0].y);
        assert!(
            (sep_area - area_a).abs() < 0.1,
            "Separate polygon should be tiny A (area ~1), got area {}",
            sep_area
        );
    }

    #[test]
    fn test_organize_multiple_contained_polygons() {
        // Edge case: Subject polygon contains 2+ existing polygons
        // Tests that the iteration-during-modification logic works correctly
        //
        // Scenario:
        // - Small A: 1,1 to 2,2
        // - Small B: 5,5 to 6,6
        // - Large C: 0,0 to 10,10 (contains both A and B)

        let small_a = vec![
            Point::new(1.0, 1.0),
            Point::new(2.0, 1.0),
            Point::new(2.0, 2.0),
            Point::new(1.0, 2.0),
        ];

        let small_b = vec![
            Point::new(5.0, 5.0),
            Point::new(6.0, 5.0),
            Point::new(6.0, 6.0),
            Point::new(5.0, 6.0),
        ];

        let large_c = vec![
            Point::new(0.0, 0.0),
            Point::new(10.0, 0.0),
            Point::new(10.0, 10.0),
            Point::new(0.0, 10.0),
        ];

        // Process in order: A, B, C
        // C should trigger reprocessing of both A and B
        let rings = vec![small_a.clone(), small_b.clone(), large_c.clone()];
        let organized = organize_polygons(rings);

        // Expected: C is exterior with A and B as holes
        assert_eq!(organized.len(), 1, "Should have 1 top-level polygon");
        assert_eq!(organized[0].1.len(), 2, "Should have 2 holes");

        // Verify the exterior is the large C
        let ext_area = (organized[0].0[2].x - organized[0].0[0].x)
            * (organized[0].0[2].y - organized[0].0[0].y);
        assert!(
            (ext_area - 100.0).abs() < 1.0,
            "Exterior should be large C (area 100)"
        );

        // Verify both holes are present (areas should be 1.0 each)
        for hole in &organized[0].1 {
            let hole_area = (hole[2].x - hole[0].x) * (hole[2].y - hole[0].y);
            assert!(
                (hole_area - 1.0).abs() < 0.1,
                "Each hole should have area ~1.0, got {}",
                hole_area
            );
        }
    }

    #[test]
    fn test_organize_wrong_processing_order() {
        // Edge case: Hole processed before its container
        // This tests the reprocessing mechanism
        //
        // Process hole FIRST, then outer
        // The algorithm should:
        // 1. Add hole as exterior (nothing else exists yet)
        // 2. Process outer, detect it contains hole
        // 3. Remove hole from results, reprocess it
        // 4. Hole becomes interior ring of outer

        let hole = vec![
            Point::new(5.0, 5.0),
            Point::new(15.0, 5.0),
            Point::new(15.0, 15.0),
            Point::new(5.0, 15.0),
        ];

        let outer = vec![
            Point::new(0.0, 0.0),
            Point::new(20.0, 0.0),
            Point::new(20.0, 20.0),
            Point::new(0.0, 20.0),
        ];

        // Process HOLE first (wrong order)
        let rings = vec![hole.clone(), outer.clone()];
        let organized = organize_polygons(rings);

        // Should still end up with correct nesting
        assert_eq!(organized.len(), 1, "Should have 1 top-level polygon");
        assert_eq!(organized[0].1.len(), 1, "Should have 1 hole");

        // Verify outer is the exterior
        let ext_area = (organized[0].0[2].x - organized[0].0[0].x)
            * (organized[0].0[2].y - organized[0].0[0].y);
        assert!(
            (ext_area - 400.0).abs() < 1.0,
            "Exterior should be outer (area 400)"
        );

        // Verify hole is interior
        let hole_area = (organized[0].1[0][2].x - organized[0].1[0][0].x)
            * (organized[0].1[0][2].y - organized[0].1[0][0].y);
        assert!(
            (hole_area - 100.0).abs() < 1.0,
            "Hole should have area 100"
        );
    }
}
