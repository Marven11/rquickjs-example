use rquickjs_example::{OriginalHTTPRequest, OriginalQueryParam, EvalContext};

fn main() {
    let query_params = vec![
        OriginalQueryParam {
            key: b"key1",
            value: b"value1",
        },
        OriginalQueryParam {
            key: b"key2",
            value: b"value2",
        },
    ];
    let http_request = OriginalHTTPRequest {
        method: b"GET",
        url_path: b"/shell.php",
        query_params,
        port: 8080,
    };
    
    let ctx = EvalContext::new().unwrap();
    let result = ctx.eval(&http_request, r#"decode_utf8(request.url_path) == "/shell.php""#).unwrap();
    dbg!(result);
}
