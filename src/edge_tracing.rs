//! Edge tracing algorithm for marching squares
//!
//! This module implements the cell-to-cell edge following algorithm that traces
//! complete polygon rings from individual cell edges.

use crate::cell_shapes::CellShape;
use crate::types::{Edge, Point};

/// A cell in the grid with its edges
#[derive(Debug, Clone)]
pub struct CellWithEdges {
    /// The shape configuration for this cell
    pub shape: CellShape,
    /// Number of edges that have been used
    pub used_edges: usize,
    /// Whether this cell has been fully processed
    pub cleared: bool,
    /// Total number of edges this cell started with (for Java-compatible cleared logic)
    total_edge_count: usize,
}

impl CellWithEdges {
    /// Create a new cell with edges
    pub fn new(shape: CellShape) -> Self {
        let total_edges = shape.edges.len();
        Self {
            shape,
            used_edges: 0,
            cleared: false,
            total_edge_count: total_edges,
        }
    }

    /// Get chained edges starting from a given point (Java-style)
    ///
    /// Matches Java's getEdges(Point start, Edge.Move prevMove) behavior:
    /// 1. If start is None, find first available edge
    /// 2. Follow chain: edge.end becomes next search point
    /// 3. Return all edges in the chain
    pub fn get_chained_edges_from(&self, start_point: Option<&Point>) -> Vec<Edge> {
        if self.cleared || self.shape.edges.is_empty() {
            return Vec::new();
        }

        // Find starting point
        let mut current_start = if let Some(pt) = start_point {
            *pt
        } else {
            // No start point - use first available edge's start
            // Java iterates through points list to find first edge in HashMap
            match self.shape.edges.keys().next() {
                Some(pt) => *pt,
                None => return Vec::new(),
            }
        };

        let mut result = Vec::new();
        let max_edges = self.shape.edges.len();

        // Follow the chain of edges within this cell
        // Java: while (this.edges.containsKey(start) && edges.size() < this.edges.size())
        while result.len() < max_edges {
            if let Some(edge) = self.shape.edges.get(&current_start) {
                result.push(edge.clone());
                current_start = edge.end;
            } else {
                break; // No more edges in chain
            }
        }

        result
    }

    /// Remove an edge by its start point (matches Java removeEdge)
    pub fn remove_edge(&mut self, start_point: &Point) {
        self.shape.edges.remove(start_point);
    }

    /// Increment used edge counter and check if cleared
    /// Matches Java Shape.java:540-543 behavior exactly
    pub fn increment_used_edges(&mut self, count: usize) {
        self.used_edges += count;
        // CRITICAL FIX: Match Java's cleared logic
        // Java: if (this.usedEdges >= this.edges.size()) this.cleared = true;
        // Since we track total_edge_count at creation, compare against that
        if self.used_edges >= self.total_edge_count {
            self.cleared = true;
        }
    }

    /// Check if this cell is cleared
    pub fn is_cleared(&self) -> bool {
        self.cleared || self.shape.edges.is_empty()
    }
}

/// Compare two points for exact equality (matches Java Double.equals())
/// Since we now use full-precision coordinates during edge tracing,
/// adjacent cells will compute identical edge endpoints bitwise.
fn points_equal(p1: &Point, p2: &Point) -> bool {
    // Use PartialEq which now does exact bitwise comparison
    p1 == p2
}

/// Trace a single polygon ring starting from a cell
///
/// Returns the list of points forming a closed ring, or None if tracing fails
///
/// This follows the Java algorithm exactly (MarchingSquares.java lines 63-109):
/// 1. Get chained edges from current cell
/// 2. Remove edges and check for ring closure after each edge
/// 3. Use the last edge's Move direction to go to next cell
/// 4. Repeat until ring closes
pub fn trace_ring(
    cells: &mut Vec<Vec<Option<CellWithEdges>>>,
    start_row: usize,
    start_col: usize,
) -> Option<Vec<Point>> {
    let rows = cells.len();
    let cols = if rows > 0 { cells[0].len() } else { 0 };

    // Check if starting cell is valid
    let start_cell = cells.get(start_row)?.get(start_col)?.as_ref()?;
    if start_cell.is_cleared() {
        return None;
    }

    let mut current_row = start_row;
    let mut current_col = start_col;
    let mut current_edge: Option<Edge> = None;
    let mut all_edges = Vec::new();
    let mut go_on = true;

    let mut iterations = 0;
    const MAX_ITERATIONS: usize = 10000;

    // Java: while (goOn && !cells[y][x].getEdges(...).isEmpty())
    while go_on {
        iterations += 1;
        if iterations > MAX_ITERATIONS {
            return None;
        }

        // Get the current cell
        let cell = match cells
            .get(current_row)
            .and_then(|r| r.get(current_col))
            .and_then(|c| c.as_ref())
        {
            Some(c) => c,
            None => break,
        };

        if cell.is_cleared() {
            break;
        }

        // Get chained edges from current cell
        // Java: cells[y][x].getEdges(currentEdge==null?null:currentEdge.getEnd(), ...)
        let tmp_edges = if let Some(ref edge) = current_edge {
            cell.get_chained_edges_from(Some(&edge.end))
        } else {
            cell.get_chained_edges_from(None)
        };

        if tmp_edges.is_empty() {
            break;
        }

        // Java: cells[y][x].incrementUsedEdges(tmpEdges.size());
        if let Some(Some(cell_mut)) = cells.get_mut(current_row).and_then(|r| r.get_mut(current_col)) {
            cell_mut.increment_used_edges(tmp_edges.len());

            // Java: for (Edge edge : tmpEdges) { ... }
            for edge in &tmp_edges {
                // Java: cells[y][x].removeEdge(edge.getStart());
                cell_mut.remove_edge(&edge.start);

                // Java: currentEdge = edge; edges.add(edge);
                current_edge = Some(edge.clone());
                all_edges.push(edge.clone());

                // Java: if (currentEdge.getEnd().equals(edges.get(0).getStart()))
                if !all_edges.is_empty() && points_equal(&edge.end, &all_edges[0].start) {
                    go_on = false;
                    break;  // Break from for loop (Java line 82)
                }
            }
        } else {
            break;
        }

        // Java: Move logic happens AFTER the for loop (lines 86-97)
        // This runs even if we broke from the for loop above
        if let Some(ref edge) = current_edge {
            match edge.move_dir {
                crate::types::Move::Right => {
                    if current_col + 1 >= cols {
                        eprintln!("⚠️ BOUNDARY: Can't move RIGHT from ({},{}) - at right edge (cols={})", current_row, current_col, cols);
                        eprintln!("   Ring has {} edges, go_on={}, edge: ({:.3},{:.3}) -> ({:.3},{:.3})",
                            all_edges.len(), go_on, edge.start.x, edge.start.y, edge.end.x, edge.end.y);
                        break;
                    }
                    current_col += 1;
                }
                crate::types::Move::Down => {
                    if current_row + 1 >= rows {
                        eprintln!("⚠️ BOUNDARY: Can't move DOWN from ({},{}) - at bottom edge (rows={})", current_row, current_col, rows);
                        eprintln!("   Ring has {} edges, go_on={}, edge: ({:.3},{:.3}) -> ({:.3},{:.3})",
                            all_edges.len(), go_on, edge.start.x, edge.start.y, edge.end.x, edge.end.y);
                        break;
                    }
                    current_row += 1;
                }
                crate::types::Move::Left => {
                    if current_col == 0 {
                        eprintln!("⚠️ BOUNDARY: Can't move LEFT from ({},{}) - at left edge", current_row, current_col);
                        eprintln!("   Ring has {} edges, go_on={}, edge: ({:.3},{:.3}) -> ({:.3},{:.3})",
                            all_edges.len(), go_on, edge.start.x, edge.start.y, edge.end.x, edge.end.y);
                        break;
                    }
                    current_col -= 1;
                }
                crate::types::Move::Up => {
                    if current_row == 0 {
                        eprintln!("⚠️ BOUNDARY: Can't move UP from ({},{}) - at top edge", current_row, current_col);
                        eprintln!("   Ring has {} edges, go_on={}, edge: ({:.3},{:.3}) -> ({:.3},{:.3})",
                            all_edges.len(), go_on, edge.start.x, edge.start.y, edge.end.x, edge.end.y);
                        break;
                    }
                    current_row -= 1;
                }
                crate::types::Move::None => {
                    // Edge stays in same cell
                    // Continue with while loop
                }
            }
        }

        // If go_on is false, the while condition will fail on next iteration
    }

    // Build the points list from all edges (Java lines 100-106)
    if all_edges.is_empty() {
        return None;
    }

    let mut points = Vec::with_capacity(all_edges.len() + 1);
    points.push(all_edges[0].start.clone());
    for edge in &all_edges {
        points.push(edge.end.clone());
    }

    if points.len() >= 3 {
        Some(points)
    } else {
        None
    }
}

/// Trace all polygon rings from a grid of cells
///
/// Returns a list of polygon rings (each ring is a Vec<Point>)
/// Only returns rings with at least 3 points (valid polygons per GeoJSON spec)
pub fn trace_all_rings(cells: &mut Vec<Vec<Option<CellWithEdges>>>) -> Vec<Vec<Point>> {
    let mut rings = Vec::new();
    let mut failed_traces = 0;
    let mut total_attempts = 0;

    let rows = cells.len();
    if rows == 0 {
        return rings;
    }
    let cols = cells[0].len();

    // Iterate through all cells looking for unprocessed edges
    for row in 0..rows {
        for col in 0..cols {
            // Keep tracing from this cell until all its edges are used
            loop {
                total_attempts += 1;
                match trace_ring(cells, row, col) {
                    Some(ring) => {
                        // Only include rings with at least 3 points
                        // (GeoJSON requires at least 4 coordinates for a valid polygon ring,
                        // with the first and last being identical. Since we don't duplicate
                        // the closing point, we need at least 3 distinct points)
                        if ring.len() >= 3 {
                            rings.push(ring);
                        }
                    }
                    None => {
                        // Check if there are still edges in this cell
                        if let Some(Some(cell)) = cells.get(row).and_then(|r| r.get(col)) {
                            if !cell.is_cleared() && !cell.shape.edges.is_empty() {
                                failed_traces += 1;
                            }
                        }
                        break;
                    }
                }
            }
        }
    }

    // Optional: Uncomment for debugging
    // eprintln!("\n📊 EDGE TRACING SUMMARY:");
    // eprintln!("   Total rings traced: {}", rings.len());
    // eprintln!("   Total trace attempts: {}", total_attempts);
    // eprintln!("   Failed traces: {}", failed_traces);

    rings
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Move;

    #[test]
    fn test_points_equal() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(1.0, 2.0);
        let p3 = Point::new(1.0, 2.1);

        assert!(points_equal(&p1, &p2));
        assert!(!points_equal(&p1, &p3));
    }

    #[test]
    fn test_cell_with_edges() {
        let edge1 = Edge::new(Point::new(0.0, 0.0), Point::new(1.0, 0.0), Move::Right);
        let edge2 = Edge::new(Point::new(1.0, 0.0), Point::new(1.0, 1.0), Move::None);

        let shape = CellShape::new(vec![edge1.clone(), edge2.clone()]);
        let mut cell = CellWithEdges::new(shape);

        assert!(!cell.is_cleared());
        assert_eq!(cell.used_edges, 0);

        // Get first edge
        let first = cell.get_chained_edges_from(None);
        assert!(!first.is_empty());

        // Remove the edge
        cell.remove_edge(&first[0].start);
        cell.increment_used_edges(1);
        assert_eq!(cell.used_edges, 1);
        assert!(!cell.is_cleared());

        // Remove second edge
        let second = cell.get_chained_edges_from(None);
        if !second.is_empty() {
            cell.remove_edge(&second[0].start);
            cell.increment_used_edges(1);
        }
        assert_eq!(cell.used_edges, 2);
        assert!(cell.is_cleared());
    }

    #[test]
    fn test_edge_chaining() {
        // Create a chain of edges: (0,0) -> (1,0) -> (1,1) -> (0,1)
        let edge1 = Edge::new(Point::new(0.0, 0.0), Point::new(1.0, 0.0), Move::Right);
        let edge2 = Edge::new(Point::new(1.0, 0.0), Point::new(1.0, 1.0), Move::None);
        let edge3 = Edge::new(Point::new(1.0, 1.0), Point::new(0.0, 1.0), Move::None);

        let shape = CellShape::new(vec![edge1, edge2, edge3]);
        let cell = CellWithEdges::new(shape);

        // Start from (0,0), should follow chain through all 3 edges
        let edges = cell.get_chained_edges_from(Some(&Point::new(0.0, 0.0)));
        assert_eq!(edges.len(), 3);

        // Verify chain order
        assert_eq!(edges[0].start, Point::new(0.0, 0.0));
        assert_eq!(edges[0].end, Point::new(1.0, 0.0));
        assert_eq!(edges[1].end, Point::new(1.0, 1.0));
        assert_eq!(edges[2].end, Point::new(0.0, 1.0));

        // Find edges starting from non-existent point
        let edges = cell.get_chained_edges_from(Some(&Point::new(5.0, 5.0)));
        assert_eq!(edges.len(), 0);
    }

    #[test]
    fn test_simple_ring_trace() {
        // Create a simple single-cell ring
        // One cell with edges that form a closed loop
        let edge1 = Edge::new(Point::new(0.0, 0.0), Point::new(1.0, 0.0), Move::None);
        let edge2 = Edge::new(Point::new(1.0, 0.0), Point::new(1.0, 1.0), Move::None);
        let edge3 = Edge::new(Point::new(1.0, 1.0), Point::new(0.0, 1.0), Move::None);
        let edge4 = Edge::new(Point::new(0.0, 1.0), Point::new(0.0, 0.0), Move::None);

        let cell = CellWithEdges::new(CellShape::new(vec![edge1, edge2, edge3, edge4]));

        let mut cells = vec![vec![Some(cell)]];

        let ring = trace_ring(&mut cells, 0, 0);

        assert!(ring.is_some(), "Ring tracing should succeed");
        let points = ring.unwrap();
        assert!(points.len() >= 4, "Should have at least 4 points");
        // First and last should be the same (closed loop)
        assert!(points_equal(&points[0], points.last().unwrap()));
    }
}
