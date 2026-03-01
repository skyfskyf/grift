use criterion::{Criterion, criterion_group, criterion_main};
use grift_arena::{Arena, ArenaIndex, Trace};

#[derive(Clone, Copy)]
enum Tree {
    Leaf(isize),
    Branch(ArenaIndex, ArenaIndex),
}

impl<const N: usize> Trace<Tree, N> for Tree {
    fn trace<F: FnMut(ArenaIndex)>(&self, mut tracer: F) {
        if let Tree::Branch(l, r) = *self {
            tracer(l);
            tracer(r);
        }
    }
}

fn bench_alloc(c: &mut Criterion) {
    c.bench_function("arena_alloc_1000", |b| {
        b.iter(|| {
            let arena: Arena<Tree, 2000> = Arena::new(Tree::Leaf(0));
            for i in 0..1000 {
                let _ = arena.alloc(Tree::Leaf(i));
            }
        });
    });
}

fn bench_alloc_free(c: &mut Criterion) {
    c.bench_function("arena_alloc_free_cycle", |b| {
        let arena: Arena<Tree, 200> = Arena::new(Tree::Leaf(0));
        b.iter(|| {
            let mut indices = [ArenaIndex::NIL; 100];
            for i in 0..100 {
                indices[i] = arena.alloc(Tree::Leaf(i as isize)).unwrap();
            }
            for idx in &indices {
                let _ = arena.free(*idx);
            }
        });
    });
}

fn bench_gc_collect(c: &mut Criterion) {
    c.bench_function("gc_collect_50pct_garbage", |b| {
        b.iter(|| {
            let arena: Arena<Tree, 2000> = Arena::new(Tree::Leaf(0));
            let mut roots = [ArenaIndex::NIL; 500];
            // Allocate 500 reachable leaves
            for (i, root) in roots.iter_mut().enumerate() {
                *root = arena.alloc(Tree::Leaf(i as isize)).unwrap();
            }
            // Allocate 500 garbage leaves
            for i in 0..500 {
                let _ = arena.alloc(Tree::Leaf(i + 10000));
            }
            arena.collect_garbage(&roots);
        });
    });
}

fn bench_gc_tree(c: &mut Criterion) {
    c.bench_function("gc_collect_tree", |b| {
        b.iter(|| {
            let arena: Arena<Tree, 4000> = Arena::new(Tree::Leaf(0));
            // Build a binary tree of depth 9 (511 nodes)
            fn build_tree(arena: &Arena<Tree, 4000>, depth: usize) -> ArenaIndex {
                if depth == 0 {
                    arena.alloc(Tree::Leaf(0)).unwrap()
                } else {
                    let l = build_tree(arena, depth - 1);
                    let r = build_tree(arena, depth - 1);
                    arena.alloc(Tree::Branch(l, r)).unwrap()
                }
            }
            let root = build_tree(&arena, 9);
            // Add some garbage
            for _ in 0..500 {
                let _ = arena.alloc(Tree::Leaf(999));
            }
            arena.collect_garbage(&[root]);
        });
    });
}

criterion_group!(benches, bench_alloc, bench_alloc_free, bench_gc_collect, bench_gc_tree);
criterion_main!(benches);
