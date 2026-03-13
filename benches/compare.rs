use criterion::{Criterion, black_box, criterion_group, criterion_main};
use rquickjs_example::{EvalContext, OriginalHTTPRequest, OriginalQueryParam};

fn bench_filename_decode(c: &mut Criterion) {
    let query_params = vec![OriginalQueryParam {
        key: b"key1",
        value: b"value1",
    }];
    let http_request = OriginalHTTPRequest {
        method: b"GET",
        url_path: b"/shell.php",
        query_params,
        port: 8080,
    };

    let ctx = EvalContext::new().unwrap();

    let mut group = c.benchmark_group("quickjs_vs_wirefilter");

    let bytes = ctx.compile(r#"decode_utf8(request.url_path) == "/shell.php""#).unwrap();

    group.bench_with_input("rquickjs", &(&ctx, &bytes), |b, (ctx, bytes)| {
        b.iter(|| {
            let result = ctx.eval_precompiled(black_box(&http_request), black_box(bytes));
            assert!(matches!(result, Ok(true)));
        })
    });

    group.finish();
}

fn bench_wirefilter(c: &mut Criterion) {
    use wirefilter::{ExecutionContext, Scheme};

    let scheme = Scheme! {
        http.method: Bytes,
        http.ua: Bytes,
        port: Int,
    };

    let ast = scheme.parse(
        r#"
            http.method != "POST" &&
            not http.ua matches "(googlebot|facebook)" &&
            port in {80 443}
        "#,
    ).unwrap();

    let filter = ast.compile();

    let mut ctx = ExecutionContext::new(&scheme);

    ctx.set_field_value("http.method", "GET").unwrap();

    ctx.set_field_value(
        "http.ua",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:66.0) Gecko/20100101 Firefox/66.0",
    )
    .unwrap();

    ctx.set_field_value("port", 443).unwrap();

    let mut group = c.benchmark_group("quickjs_vs_wirefilter");

    group.bench_with_input("wirefilter", &ctx, |b, ctx| {
        b.iter(|| {
            filter.execute(&ctx).unwrap();
        })
    });

    group.finish();
}

criterion_group!(benches, bench_filename_decode, bench_wirefilter);
criterion_main!(benches);
