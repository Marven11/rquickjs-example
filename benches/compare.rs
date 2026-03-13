use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rquickjs_example::{OriginalHTTPRequest, OriginalQueryParam, EvalContext};

fn bench_filename_decode(c: &mut Criterion) {
    let query_params = vec![
        OriginalQueryParam {
            key: b"key1",
            value: b"value1",
        },
    ];
    let http_request = OriginalHTTPRequest {
        method: b"GET",
        url_path: b"/shell.php",
        query_params,
        port: 8080,
    };

    let ctx = EvalContext::new().unwrap();

    let mut group = c.benchmark_group("quickjs_vs_rhai");
    
    group.bench_with_input("filename_decode", &ctx, |b, ctx| {
        b.iter(|| {
            let result = ctx.eval(black_box(&http_request), black_box(r#"decode_utf8(request.url_path) == "/shell.php""#));
            assert!(matches!(result, Ok(true)));
        })
    });

    group.finish();
}

criterion_group!(benches, bench_filename_decode);
criterion_main!(benches);
