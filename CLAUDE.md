# CLAUDE.md - Context for geo-marching-squares-rs

## 🚀 Quick Start (For Context Restore)

```bash
cd ~/source/geo-marching-squares-rs

# Build
cargo build

# Run all tests
cargo test

# Run examples
cargo run --example simple_contours
cargo run --example phase2_demo

# Check status
git status
ls -la src/
```

---

## Project Status

**CURRENT STATE**: ⚠️ Core algorithm has bugs - undergoing rewrite based on exact Java port

**Reference Documents** (READ THESE):
- **JAVA_PORT_REFERENCE.md** - ⭐ START HERE - Definitive porting guide with exact implementation steps
- **JAVA_ANALYSIS.md** - 1,460-line deep analysis of Java implementation (for details)

**Known Issues**:
- ❌ Diagonal artifacts in output
- ❌ Incorrect polygon generation
- ❌ Edge tracing failures

**Root Cause**: Current implementation doesn't exactly match Java's Point structure and deduplication logic

---

## Critical Algorithm Details (From Java Analysis)

### The Key Difference: Point Structure

**Java Point** can be:
- **Actual Point**: `x` and `y` coordinates set (for corners in band)
  - Example: `Point { x: -99.0, y: 41.0, ... }`
- **Placeholder Point**: `x=null`, `y=null`, has `value`, `limit`, `side` (to be interpolated later)
  - Example: `Point { x: null, y: null, value: 15.0, limit: 20.0, side: TOP }`

**Rust currently**: Only uses actual Points - missing the placeholder concept entirely!

### Deduplication Logic (Critical Bug)

**Java** `.distinct()` compares ALL fields:
```java
Point.equals() checks: x, y, value, limit, side
```

**Rust currently**: Uses epsilon comparison on x,y only - **WRONG!**
```rust
// Current (BROKEN):
(existing.x - pt.x).abs() < EPSILON && (existing.y - pt.y).abs() < EPSILON

// Should be (matching Java):
self.x == other.x && self.y == other.y &&
self.value == other.value && self.limit == other.limit &&
self.side == other.side
```

### 8-Point Array Order (Must Be Exact)

Java `Shape.java:229-236` - Clockwise from top-right area:
1. Position 0: Top edge at TR corner
2. Position 1: Right edge at TR corner
3. Position 2: Right edge at BR corner
4. Position 3: Bottom edge at BR corner
5. Position 4: Bottom edge at BL corner
6. Position 5: Left edge at BL corner
7. Position 6: Left edge at TL corner
8. Position 7: Top edge at TL corner

Currently may not match exact order - must verify!

### Interpolation Formula (Check 0.999 Factor)

```rust
let mu = (level - value0) / (value1 - value0);
let mu2 = (1.0 - (mu * PI).cos()) / 2.0;
let center_diff = (mu2 - 0.5) * 0.999;  // ← CRITICAL: Must be 0.999
let new_mu = 0.5 + center_diff;
```

### Coordinate Rounding (Timing Matters)

**Java**: Rounds ONLY at final GeoJSON output (MarchingSquares.java:101-105)
- Uses `BigDecimal.setScale(5, HALF_UP)`
- 5 decimal places

**Rust**: Must NOT round during interpolation or edge creation!

---

## Implementation Plan

### Phase 1: Fix Core Algorithm (IN PROGRESS)
1. ✅ Create JAVA_ANALYSIS.md - comprehensive Java analysis
2. ✅ Create JAVA_PORT_REFERENCE.md - exact porting guide
3. ⏳ Rewrite `types.rs` - Point with `Option<f64>` fields for placeholder support
4. ⏳ Rewrite point generation - exact 8-point array matching Java order
5. ⏳ Rewrite deduplication - `Point::eq()` comparing ALL fields
6. ⏳ Fix edge tracing - exact HashMap lookups with bitwise equality
7. ⏳ Verify interpolation - confirm 0.999 factor and no early rounding
8. ⏳ Test against Java output - pixel-perfect match

### Phase 2: Integration (PENDING)
- Integrate with grib-inspector
- Performance benchmarking
- Production testing with HRRR data

---

## Project Structure

### Key Files to Examine (In Priority Order)

**Reference Documents** (Start Here):
- `JAVA_PORT_REFERENCE.md` - Implementation guide with exact steps
- `JAVA_ANALYSIS.md` - Deep Java analysis (1,460 lines)

**Core Implementation** (Needs Rewrite):
- `src/types.rs` - Point, Edge, Move, Side (MUST ADD PLACEHOLDER SUPPORT)
- `src/cell_shapes/mod.rs` - 8-point generation (VERIFY ORDER)
- `src/interpolation.rs` - Cosine formula (VERIFY 0.999 FACTOR)
- `src/edge_tracing.rs` - HashMap lookups (VERIFY POINT EQUALITY)

**Shape Implementations**:
- `src/cell_shapes/triangles.rs`
- `src/cell_shapes/pentagons.rs`
- `src/cell_shapes/rectangles.rs`
- `src/cell_shapes/trapezoids.rs`
- `src/cell_shapes/hexagons.rs`
- `src/cell_shapes/saddles.rs`
- `src/cell_shapes/square.rs`

**Supporting Modules**:
- `src/marching_squares.rs` - Main orchestration
- `src/polygon_util.rs` - Hole detection
- `src/grid.rs` - Grid data structure

---

## Dependencies

```toml
geojson = "0.24"        # GeoJSON output
geo-types = "0.7"       # Geometric types
anyhow = "1.0"          # Error handling
thiserror = "1.0"       # Error macros
rayon = { version = "1.8", optional = true }  # Parallel processing
serde = { version = "1.0", features = ["derive"] }  # Serialization
```

### Features
- `parallel`: Enable rayon-based parallel processing (default)
- `great-circle`: Enable spherical interpolation for ultra-high precision

---

## Testing Strategy

See `JAVA_PORT_REFERENCE.md` "Testing Priorities" section for detailed checklist.

### Priority Tests:
1. **Config Calculation** - Verify 3-level encoding (0/1/2)
2. **8-Point Array** - Exact order and placeholder vs actual Points
3. **Deduplication** - Point.equals() on ALL fields
4. **Interpolation** - 0.999 factor, unrounded output
5. **Edge Tracing** - HashMap lookup with bitwise equality
6. **Coordinate Rounding** - Only at GeoJSON output

### Current Test Status:
- 34 tests exist but may pass with buggy implementation
- Need new tests based on JAVA_PORT_REFERENCE.md checklist

---

## Background Context

### Original Problem (grib-inspector)
This crate was created to solve performance bottlenecks in the grib-inspector project when generating meteorological contours. The key innovation is pre-transforming the entire grid once, then running marching squares in geographic space.

### Java Reference Implementation
- Location: `/tmp/marching-squares-java/`
- Proven production implementation
- All Rust behavior must match Java exactly

### Algorithm Resources
- Wikipedia: https://en.wikipedia.org/w/index.php?title=Marching_squares&oldid=992833944
- Isoband diagrams (study these):
  - https://upload.wikimedia.org/wikipedia/commons/a/a6/Marching-squares-isoband-1.png
  - https://upload.wikimedia.org/wikipedia/commons/d/d5/Marching-squares-isoband-2.png
  - https://upload.wikimedia.org/wikipedia/commons/c/c7/Ms-isoband-3.png

---

## Related Context

- **Parent project**: `/Users/brent/source/grib-inspector` (meteorological data processing)
- **Original Java implementation**: https://github.com/brentlemons/marching-squares-java
- **Alternative libraries**: contour-rs v0.13, contour-isobands-rs v0.4 (what we're replacing)

---

## When Context Compresses (Recovery Instructions)

If you lose context and need to resume:

1. **Read JAVA_PORT_REFERENCE.md first** - This is the source of truth
2. Read JAVA_ANALYSIS.md for deep details when needed
3. Check inline comments with `// Java: Shape.java:XXX` line references
4. Run tests to verify current state
5. **DO NOT GUESS** - Always reference the Java source at `/tmp/marching-squares-java/`

**Never assume the current Rust implementation is correct** - it has fundamental bugs that need fixing by matching Java exactly.
