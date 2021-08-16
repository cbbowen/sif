use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sif::*;

struct PCG32 {
	state: u64,
	inc: u64,
}

impl PCG32 {
	fn new() -> Self {
		Self::with_seed(0)
	}

	fn with_seed(seed: u64) -> Self {
		PCG32 {
			state: seed,
			inc: 0,
		}
	}
}

impl Iterator for PCG32 {
	type Item = u32;
	fn next(&mut self) -> Option<Self::Item> {
		let state = self.state;
		self.state = state * 6364136223846793005u64 + self.inc;
		let xorshifted = (((state >> 18) ^ state) >> 27) as u32;
		let rot = (state >> 59) as u32;
		Some(xorshifted.rotate_right(rot & 31))
	}
}

fn random_graph<G: InsertGraph>(mut rng: PCG32) -> G {
	let mut g = G::default();
	let mut verts = Vec::new();
	for _ in 0..100 {
		verts.push(g.insert_vert());
	}
	for _ in 0..1000 {
		g.insert_edge(
			verts[rng.next().unwrap() as usize % verts.len()],
			verts[rng.next().unwrap() as usize % verts.len()],
		);
	}
	g
}

fn depth_first_out_benchmark_routine(g: &impl OutGraph) {
	let mut start_tree = 0usize;
	let mut end_tree = 0usize;
	let mut open_edge = 0usize;
	let mut cross_edge = 0usize;
	let mut back_edge = 0usize;
	let mut close_edge = 0usize;
	for event in g.depth_first_out() {
		use DepthFirstEvent::*;
		match event {
			StartTree(_) => start_tree += 1,
			EndTree => end_tree += 1,
			OpenEdge(_) => open_edge += 1,
			CrossEdge(_) => cross_edge += 1,
			BackEdge(_) => back_edge += 1,
			CloseEdge(_) => close_edge += 1,
			_ => {}
		}
	}
	assert_eq!(start_tree, end_tree);
	assert_eq!(open_edge, close_edge);
	assert_eq!(open_edge + cross_edge + back_edge, g.edges().count());
}

fn depth_first_out_benchmark(c: &mut Criterion) {
	c.bench_function("DenseOutAdjacencyList", |b| {
		let g = random_graph::<DenseOutAdjacencyList>(PCG32::new());
		b.iter(|| depth_first_out_benchmark_routine(black_box(&g)))
	});

	c.bench_function("DenseBiAdjacencyList", |b| {
		let g = random_graph::<DenseBiAdjacencyList>(PCG32::new());
		b.iter(|| depth_first_out_benchmark_routine(black_box(&g)))
	});

	c.bench_function("SparseOutAdjacencyList", |b| {
		let g = random_graph::<SparseOutAdjacencyList>(PCG32::new());
		b.iter(|| depth_first_out_benchmark_routine(black_box(&g)))
	});

	c.bench_function("SparseBiAdjacencyList", |b| {
		let g = random_graph::<SparseBiAdjacencyList>(PCG32::new());
		b.iter(|| depth_first_out_benchmark_routine(black_box(&g)))
	});
}

criterion_group!(benches, depth_first_out_benchmark);
criterion_main!(benches);
