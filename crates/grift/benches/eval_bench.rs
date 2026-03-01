use criterion::{Criterion, criterion_group, criterion_main};
use grift::Lisp;

fn bench_eval_arithmetic(c: &mut Criterion) {
    let lisp: Lisp<10000> = Lisp::new();
    c.bench_function("eval_arithmetic", |b| {
        b.iter(|| {
            let _ = lisp.eval("(+ 1 2 3 4 5 6 7 8 9 10)");
        });
    });
}

fn bench_eval_fib_15(c: &mut Criterion) {
    c.bench_function("eval_fib_naive_15", |b| {
        b.iter(|| {
            let lisp: Lisp<50000> = Lisp::new();
            let _ = lisp.eval(
                "(begin (define! fib (lambda (n) (if (<= n 1) n (+ (fib (- n 1)) (fib (- n 2)))))) (fib 15))",
            );
        });
    });
}

fn bench_eval_tco_countdown(c: &mut Criterion) {
    c.bench_function("eval_tco_countdown_10000", |b| {
        b.iter(|| {
            let lisp: Lisp<50000> = Lisp::new();
            let _ = lisp.eval(
                "(begin (define! countdown (lambda (n) (if (= n 0) 0 (countdown (- n 1))))) (countdown 10000))",
            );
        });
    });
}

fn bench_eval_list_build(c: &mut Criterion) {
    c.bench_function("eval_list_build_500", |b| {
        b.iter(|| {
            let lisp: Lisp<50000> = Lisp::new();
            let _ = lisp.eval(
                "(begin (define! build (lambda (n acc) (if (= n 0) acc (build (- n 1) (cons n acc))))) (build 500 (list)))",
            );
        });
    });
}

fn bench_parse(c: &mut Criterion) {
    let lisp: Lisp<10000> = Lisp::new();
    c.bench_function("parse_nested_expr", |b| {
        b.iter(|| {
            let _ = lisp.eval("(+ 1 (+ 2 (+ 3 (+ 4 (+ 5 (+ 6 (+ 7 (+ 8 (+ 9 10)))))))))");
        });
    });
}

criterion_group!(
    benches,
    bench_eval_arithmetic,
    bench_eval_fib_15,
    bench_eval_tco_countdown,
    bench_eval_list_build,
    bench_parse,
);
criterion_main!(benches);
