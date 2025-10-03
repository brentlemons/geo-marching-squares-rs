# Phase 4 Optimization Plan

## Baseline Performance
- Isolines 100x100: 253 µs (39.5 Melem/s)
- Isobands 100x100: 316 µs (31.6 Melem/s)
- Interpolation: 5.9 ns per call
- HRRR-scaled (360x212): 1.7 ms (44.9 Melem/s)

## Identified Hotspots

### 1. Memory Allocations
**Location**: `edge_tracing.rs`
- Line 39: `Vec::new()` in `get_edges_from()` - called frequently during tracing
- Line 93: `Vec::new()` for points accumulation
- Frequent `clone()` calls on Points and Edges

**Location**: `marching_squares.rs`
- Multiple Vec allocations for cell grids
- Feature collection building

**Optimization Strategy**:
- Pre-allocate Vecs with capacity hints
- Use object pools for frequently allocated structures
- Reduce clones with borrows where possible
- Use SmallVec for small collections (< 8 items)

### 2. SIMD Opportunities

**Batch Interpolation** (High Impact):
- Current: Process one cell at a time
- SIMD: Process 4-8 cells in parallel using AVX/SSE
- Interpolation math is embarrassingly parallel

**Grid Value Comparisons** (Medium Impact):
- Cell configuration calculation compares 4 corners
- Can vectorize threshold comparisons

**Point Operations** (Low Impact):
- Single interpolation is already 5.9ns
- SIMD won't help single operations
- But batch operations could benefit

## Implementation Plan

### Phase 4a: Memory Optimizations
1. Add `smallvec` dependency
2. Pre-allocate with capacity hints
3. Reduce unnecessary clones
4. Use `&mut` where possible instead of creating new Vecs

### Phase 4b: SIMD Vectorization
1. Add `packed_simd_2` or use `std::simd` (nightly)
2. Vectorize cell configuration calculation
3. Batch interpolation for multiple cells
4. SIMD threshold comparisons

### Phase 4c: Additional Optimizations
1. Inline hot path functions
2. Profile-guided optimization (PGO)
3. Consider rayon granularity tuning

## Expected Improvements
- Memory: 20-30% reduction in allocations
- SIMD: 15-25% throughput improvement for large grids
- Combined: 30-50% overall performance improvement

## Success Metrics
- Isobands 100x100: Target < 220 µs (40%+ improvement)
- HRRR-scaled: Target < 1.2 ms (30%+ improvement)
- Memory: Reduce allocations by 25%+
