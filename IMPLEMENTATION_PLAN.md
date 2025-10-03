# Implementation Plan: Fix Rust Marching Squares Edge Tracing to Match Java

## Problem Statement

The Rust implementation of the marching squares algorithm does not match the Java implementation's behavior. The core issue is **edge management during ring tracing**:

- **Java**: Uses `HashMap<Point, Edge>` and **removes** edges as they're consumed
- **Rust**: Uses `Vec<Edge>` and only **increments a counter** to track usage

This causes incorrect polygon generation because edges can be re-used in Rust when they shouldn't be.

## Goal

Modify the Rust implementation to exactly match the Java algorithm's edge management strategy.

---

## Phase 1: Update Data Structures

### 1.1 Modify Point to Support HashMap Usage

**File**: `/Users/brent/source/geo-marching-squares-rs/src/types.rs`

**Current State**: Point likely doesn't implement Hash + Eq properly for f64 coordinates

**Required Changes**:
```rust
use std::hash::{Hash, Hasher};

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Round to 6 decimals to match GeoJSON output precision
        // This ensures points that are "equal" hash the same
        let x_rounded = (self.x * 1_000_000.0).round() as i64;
        let y_rounded = (self.y * 1_000_000.0).round() as i64;
        x_rounded.hash(state);
        y_rounded.hash(state);
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        const EPSILON: f64 = 1e-6;
        (self.x - other.x).abs() < EPSILON && (self.y - other.y).abs() < EPSILON
    }
}

impl Eq for Point {}
```

**Why**: HashMap requires `Hash + Eq`. We can't derive these for f64, so we implement manually with rounding.

**Test**:
```rust
#[test]
fn test_point_hash_equality() {
    use std::collections::HashMap;
    let p1 = Point::new(1.123456, 2.654321);
    let p2 = Point::new(1.123456, 2.654321);
    let p3 = Point::new(1.123457, 2.654321); // Slightly different

    let mut map = HashMap::new();
    map.insert(p1, "value");

    assert!(map.contains_key(&p2)); // Should find equal point
    assert!(!map.contains_key(&p3)); // Should not find different point
}
```

---

### 1.2 Update CellShape to Use HashMap

**File**: `/Users/brent/source/geo-marching-squares-rs/src/cell_shapes.rs`

**Current State** (line 16-21):
```rust
#[derive(Clone)]
pub struct CellShape {
    /// List of edges in this cell (start point, end point, move direction)
    pub edges: Vec<Edge>,
}
```

**New State**:
```rust
use std::collections::HashMap;

#[derive(Clone)]
pub struct CellShape {
    /// Edges in this cell, keyed by start point (matches Java)
    pub edges: HashMap<Point, Edge>,
}
```

**Impact**: This will break compilation in many places. We'll fix each systematically.

---

### 1.3 Update CellShape Constructor

**File**: `/Users/brent/source/geo-marching-squares-rs/src/cell_shapes.rs`

**Current State** (line 32-35):
```rust
pub fn new(edges: Vec<Edge>) -> Self {
    Self { edges }
}
```

**New State**:
```rust
pub fn new(edges: Vec<Edge>) -> Self {
    let mut edge_map = HashMap::new();
    for edge in edges {
        edge_map.insert(edge.start.clone(), edge);
    }
    Self { edges: edge_map }
}
```

**Alternative** (for direct HashMap construction):
```rust
pub fn new_from_map(edges: HashMap<Point, Edge>) -> Self {
    Self { edges }
}

pub fn new(edges: Vec<Edge>) -> Self {
    Self::new_from_map(
        edges.into_iter()
            .map(|e| (e.start.clone(), e))
            .collect()
    )
}
```

---

## Phase 2: Update CellWithEdges

### 2.1 Modify Edge Retrieval Methods

**File**: `/Users/brent/source/geo-marching-squares-rs/src/edge_tracing.rs`

**Current State** (lines 11-121): CellWithEdges has Vec-based methods

**New Implementation**:

```rust
#[derive(Debug, Clone)]
pub struct CellWithEdges {
    /// The shape configuration for this cell
    pub shape: CellShape,
    /// Number of edges that have been used (for tracking)
    pub used_edges: usize,
    /// Whether this cell has been fully processed
    pub cleared: bool,
}

impl CellWithEdges {
    pub fn new(shape: CellShape) -> Self {
        Self {
            shape,
            used_edges: 0,
            cleared: false,
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
            pt.clone()
        } else {
            // No start point - use first available edge's start
            // Java iterates through points list to find first edge in HashMap
            match self.shape.edges.keys().next() {
                Some(pt) => pt.clone(),
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
                current_start = edge.end.clone();
            } else {
                break; // No more edges in chain
            }
        }

        result
    }

    /// Remove an edge by its start point (matches Java)
    pub fn remove_edge(&mut self, start_point: &Point) {
        self.shape.edges.remove(start_point);
    }

    /// Increment used edge counter and check if cleared
    pub fn increment_used_edges(&mut self, count: usize) {
        self.used_edges += count;
        if self.shape.edges.is_empty() {
            self.cleared = true;
        }
    }

    /// Check if this cell is cleared
    pub fn is_cleared(&self) -> bool {
        self.cleared || self.shape.edges.is_empty()
    }
}
```

**Key Changes**:
- `get_chained_edges_from()`: Uses HashMap lookups instead of Vec iteration
- `remove_edge()`: New method to actually remove edges (matches Java line 416)
- `increment_used_edges()`: Checks if HashMap is empty to set cleared flag
- `is_cleared()`: Also checks if edges HashMap is empty

---

### 2.2 Update trace_ring Function

**File**: `/Users/brent/source/geo-marching-squares-rs/src/edge_tracing.rs`

**Current State** (lines 140-262): Uses mark_edges_used only

**Key Changes Needed**:

Around line 175 (after getting first edges):
```rust
// Mark initial edges as used
if let Some(Some(cell)) = cells.get_mut(current_row).and_then(|r| r.get_mut(current_col)) {
    cell.increment_used_edges(current_edges.len());
    // REMOVE EDGES (new)
    for edge in &current_edges {
        cell.remove_edge(&edge.start);
    }
}
```

Around line 239 (after getting next edges):
```rust
// Mark edges as used
if let Some(Some(cell)) = cells.get_mut(current_row).and_then(|r| r.get_mut(current_col)) {
    cell.increment_used_edges(next_edges.len());
    // REMOVE EDGES (new)
    for edge in &next_edges {
        cell.remove_edge(&edge.start);
    }
}
```

**Full Updated Function** (lines 140-262 replacement):

```rust
/// Trace a single polygon ring starting from a cell
///
/// Returns the list of points forming a closed ring, or None if tracing fails
///
/// This follows the Java algorithm exactly:
/// 1. Get chained edges from current cell (may be multiple edges)
/// 2. Remove edges from cell as they're consumed
/// 3. Add all edge points to the ring
/// 4. Use the last edge's Move direction to go to next cell
/// 5. Repeat until ring closes
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

    // Get the chained edges from the starting cell (Java: getEdges(null, null))
    let first_edges = start_cell.get_chained_edges_from(None);
    if first_edges.is_empty() {
        return None;
    }

    let start_point = first_edges[0].start.clone();
    let mut all_edges = Vec::new();
    all_edges.extend(first_edges.clone());

    let mut current_row = start_row;
    let mut current_col = start_col;
    let mut current_edges = first_edges;

    let mut iterations = 0;
    const MAX_ITERATIONS: usize = 10000;

    // Mark initial edges as used AND REMOVE THEM (matches Java lines 75-78)
    if let Some(Some(cell)) = cells.get_mut(current_row).and_then(|r| r.get_mut(current_col)) {
        cell.increment_used_edges(current_edges.len());
        for edge in &current_edges {
            cell.remove_edge(&edge.start); // NEW: Actual removal
        }
    }

    loop {
        iterations += 1;
        if iterations > MAX_ITERATIONS {
            return None;
        }

        // Get the last edge from the current batch (determines next move)
        let last_edge = match current_edges.last() {
            Some(edge) => edge,
            None => return None,
        };

        // Check if we've closed the loop (Java line 80)
        if points_equal(&last_edge.end, &start_point) {
            break;
        }

        // Move to the next cell based on the last edge's direction (Java lines 86-97)
        let next_pos = last_edge.move_dir.apply(current_row, current_col);

        match next_pos {
            Some((next_row, next_col)) => {
                if next_row >= rows || next_col >= cols {
                    return None;
                }
                current_row = next_row;
                current_col = next_col;
            }
            None => {
                // Move::None means edge stays in same cell
                continue;
            }
        }

        // Get the next cell
        let next_cell = match cells
            .get(current_row)
            .and_then(|r| r.get(current_col))
            .and_then(|c| c.as_ref())
        {
            Some(cell) => cell,
            None => return None,
        };

        if next_cell.is_cleared() {
            return None;
        }

        // Get chained edges starting from where last edge ended (Java line 73)
        let next_edges = next_cell.get_chained_edges_from(Some(&last_edge.end));

        if next_edges.is_empty() {
            // No matching edge - this ring cannot continue
            return None;
        }

        // Mark edges as used AND REMOVE THEM (matches Java lines 75-78)
        if let Some(Some(cell)) = cells.get_mut(current_row).and_then(|r| r.get_mut(current_col)) {
            cell.increment_used_edges(next_edges.len());
            for edge in &next_edges {
                cell.remove_edge(&edge.start); // NEW: Actual removal
            }
        }

        all_edges.extend(next_edges.clone());
        current_edges = next_edges;
    }

    // Build the points list from all edges (Java style: add start, then all ends)
    // Java lines 100-106
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
```

---

### 2.3 Update trace_all_rings Function

**File**: `/Users/brent/source/geo-marching-squares-rs/src/edge_tracing.rs`

**Current State** (lines 264-316): Check for unused edges using counter

**Update** (around line 297):

```rust
// Check if there are still edges in this cell
if let Some(Some(cell)) = cells.get(row).and_then(|r| r.get(col)) {
    if !cell.is_cleared() && !cell.shape.edges.is_empty() {
        failed_traces += 1;
    }
}
```

**Key Change**: Check `!cell.shape.edges.is_empty()` instead of `cell.used_edges < cell.shape.edges.len()`

---

## Phase 3: Fix All Compilation Errors

### 3.1 Update All Cell Shape Construction Sites

**Files to Check**:
- `/Users/brent/source/geo-marching-squares-rs/src/cell_shapes.rs` (all shape functions that return edges)

**Strategy**: Search for all places that construct `Vec<Edge>` and ensure they're converted to HashMap

**Command to find locations**:
```bash
cd /Users/brent/source/geo-marching-squares-rs
grep -n "vec!\[" src/cell_shapes.rs | head -20
```

**Pattern**: Each shape variant (Triangle, Pentagon, Rectangle, etc.) builds edges as Vec, then returns:
```rust
Some(Self { edges: vec![edge1, edge2, ...] })
```

**Update Pattern**: Keep building as Vec, then convert:
```rust
Some(Self::new(vec![edge1, edge2, ...]))
```

This uses the `new()` constructor which converts Vec → HashMap.

---

### 3.2 Update Tests

**File**: `/Users/brent/source/geo-marching-squares-rs/src/edge_tracing.rs`

**Tests to Update** (lines 318-402):

```rust
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
fn test_get_edges_from() {
    let edge1 = Edge::new(Point::new(0.0, 0.0), Point::new(1.0, 0.0), Move::Right);
    let edge2 = Edge::new(Point::new(1.0, 0.0), Point::new(1.0, 1.0), Move::None);
    let edge3 = Edge::new(Point::new(1.0, 0.0), Point::new(2.0, 0.0), Move::None);

    let shape = CellShape::new(vec![edge1, edge2, edge3]);
    let cell = CellWithEdges::new(shape);

    // Find edges starting from (1.0, 0.0)
    let edges = cell.get_chained_edges_from(Some(&Point::new(1.0, 0.0)));
    assert_eq!(edges.len(), 1); // HashMap can only have one edge per start point!

    // Find edges starting from (0.0, 0.0)
    let edges = cell.get_chained_edges_from(Some(&Point::new(0.0, 0.0)));
    assert_eq!(edges.len(), 1);

    // Find edges starting from non-existent point
    let edges = cell.get_chained_edges_from(Some(&Point::new(5.0, 5.0)));
    assert_eq!(edges.len(), 0);
}
```

**IMPORTANT NOTE**: The `test_get_edges_from` test has a bug! It creates 3 edges, but 2 have the same start point `(1.0, 0.0)`. In a HashMap, **the second one will overwrite the first**. This needs to be fixed:

```rust
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
}
```

---

## Phase 4: Testing & Validation

### 4.1 Run Unit Tests

```bash
cd /Users/brent/source/geo-marching-squares-rs
cargo test
```

**Expected**: All 34 tests should pass (may need adjustments)

### 4.2 Run Examples

```bash
cargo run --example simple_contours
cargo run --example phase2_demo
```

**Verify**: Output GeoJSON should be identical to previous runs (or close enough)

### 4.3 Compare with Java Output

**Create comparison test**:

```rust
// tests/java_parity.rs
#[test]
fn test_matches_java_output() {
    // Use same test data as Java
    // Compare polygon counts, point counts, ring structure
    // This test will initially fail - use it to debug
}
```

---

## Phase 5: Performance Testing (Optional)

### 5.1 Benchmark

```bash
cargo bench
```

### 5.2 Profile Memory Usage

```bash
cargo build --release
/usr/bin/time -l ./target/release/examples/phase2_demo
```

Check "maximum resident set size" before/after HashMap change.

---

## Rollback Plan

If HashMap causes issues:

1. Git commit before starting: `git commit -m "Before HashMap refactor"`
2. Keep Vec-based implementation in a branch: `git branch vec-based-backup`
3. Can revert with: `git reset --hard HEAD~1`

---

## Success Criteria

1. ✅ All tests pass
2. ✅ Examples run without errors
3. ✅ GeoJSON output is valid
4. ✅ Edge removal happens correctly (add debug logs to verify)
5. ✅ No edge duplication (compare polygon counts with Java)

---

## Expected Challenges

### Challenge 1: HashMap Key Collisions
**Problem**: Floating point rounding may cause different points to hash the same
**Solution**: Adjust EPSILON or rounding precision in Point::hash()

### Challenge 2: Edge Ordering
**Problem**: HashMap iteration order is non-deterministic
**Solution**: Doesn't matter! Java's HashMap is also non-deterministic. As long as edges are **found** by start point, order doesn't matter.

### Challenge 3: Performance Regression
**Problem**: HashMap might be slower than Vec for small edge counts
**Solution**: Accept it for now, optimize later if profiling shows it matters

### Challenge 4: Multiple Edges Per Start Point
**Problem**: Some cells might have >1 edge with same start point
**Solution**: This shouldn't happen in valid marching squares, but if it does, we need to investigate the cell shape generation logic

---

## Debug Logging

Add logging to verify edge removal:

```rust
// In trace_ring, after removing edges:
#[cfg(debug_assertions)]
eprintln!("Cell [{},{}]: Removed {} edges, {} remaining",
    current_row, current_col, current_edges.len(),
    cell.shape.edges.len());
```

Enable with:
```bash
RUST_LOG=debug cargo run --example phase2_demo
```

---

## File Checklist

- [ ] `/Users/brent/source/geo-marching-squares-rs/src/types.rs` - Add Hash + Eq for Point
- [ ] `/Users/brent/source/geo-marching-squares-rs/src/cell_shapes.rs` - Change edges to HashMap
- [ ] `/Users/brent/source/geo-marching-squares-rs/src/edge_tracing.rs` - Update CellWithEdges methods
- [ ] `/Users/brent/source/geo-marching-squares-rs/src/edge_tracing.rs` - Update trace_ring
- [ ] `/Users/brent/source/geo-marching-squares-rs/src/edge_tracing.rs` - Update trace_all_rings
- [ ] `/Users/brent/source/geo-marching-squares-rs/src/edge_tracing.rs` - Fix tests
- [ ] Run `cargo test` 
- [ ] Run `cargo run --example simple_contours` 
- [ ] Run `cargo run --example phase2_demo` 
- [ ] Git commit

---

## Next Session Recovery

If this session is lost, the next Claude should:

1. Read this implementation plan (IMPLEMENTATION_PLAN.md)
2. Read `/Users/brent/source/geo-marching-squares-rs/CLAUDE.md` for context
3. Check git status to see what's been done
4. Continue from the checklist above
5. Refer to Java files in `/tmp/marching-squares-java/` for reference

The core change is: **Replace Vec<Edge> with HashMap<Point, Edge> in CellShape, and actually remove edges as they're consumed during ring tracing.**
