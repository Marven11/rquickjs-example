use rquickjs::class::Trace;
use rquickjs::prelude::Func;
use rquickjs::Array;
use rquickjs::{Class, JsLifetime, TypedArray};
use rquickjs::{Context, Runtime};

#[derive(Clone)]
struct OriginalQueryParam<'a> {
    key: &'a [u8],
    value: &'a [u8],
}

#[derive(Clone)]
struct OriginalHTTPRequest<'a> {
    method: &'a [u8],
    url_path: &'a [u8],
    query_params: Vec<OriginalQueryParam<'a>>,
    port: u16,
}

#[rquickjs::class]
#[derive(Clone, Trace)]
struct QueryParam<'js> {
    #[qjs(skip_trace)]
    original: &'js OriginalQueryParam<'js>,
}

unsafe impl<'js> JsLifetime<'js> for QueryParam<'js> {
    type Changed<'to> = QueryParam<'to>;
}

impl<'js> QueryParam<'js> {
    pub fn new<'other: 'js>(original: &'js OriginalQueryParam<'other>) -> Self {
        Self { original }
    }
}

#[rquickjs::methods]
impl<'js> QueryParam<'js> {
    #[qjs(get)]
    fn key(&self, ctx: rquickjs::Ctx<'js>) -> rquickjs::Result<TypedArray<'js, u8>> {
        TypedArray::new(ctx, self.original.key.to_vec())
    }

    #[qjs(get)]
    fn value(&self, ctx: rquickjs::Ctx<'js>) -> rquickjs::Result<TypedArray<'js, u8>> {
        TypedArray::new(ctx, self.original.value.to_vec())
    }
}

#[rquickjs::class]
#[derive(Clone, Trace)]
struct HTTPRequest<'js> {
    #[qjs(skip_trace)]
    original: &'js OriginalHTTPRequest<'js>,
}

unsafe impl<'js> JsLifetime<'js> for HTTPRequest<'js> {
    type Changed<'to> = HTTPRequest<'to>;
}

impl<'js> HTTPRequest<'js> {
    pub fn new<'other: 'js>(original: &'js OriginalHTTPRequest<'other>) -> Self {
        Self { original }
    }
}

#[rquickjs::methods]
impl<'js> HTTPRequest<'js> {
    #[qjs(get)]
    fn method(&self, ctx: rquickjs::Ctx<'js>) -> rquickjs::Result<TypedArray<'js, u8>> {
        TypedArray::new(ctx, self.original.method.to_vec())
    }

    #[qjs(get)]
    fn url_path(&self, ctx: rquickjs::Ctx<'js>) -> rquickjs::Result<TypedArray<'js, u8>> {
        TypedArray::new(ctx, self.original.url_path.to_vec())
    }

    #[qjs(get)]
    fn query_params(&self, ctx: rquickjs::Ctx<'js>) -> rquickjs::Result<Array<'js>> {
        let arr = Array::new(ctx.clone())?;
        for (i, param) in self.original.query_params.iter().enumerate() {
            let qp = QueryParam::new(param);
            let instance = unsafe {
                let qp_static: QueryParam<'static> = std::mem::transmute(qp);
                Class::<QueryParam>::instance(ctx.clone(), qp_static)?
            };
            arr.set(i, instance)?;
        }
        Ok(arr)
    }

    #[qjs(get)]
    fn port(&self) -> u16 {
        self.original.port
    }
}

fn run_http_request<'a, 'b: 'a>(req: &'a OriginalHTTPRequest<'b>) {
    let request = HTTPRequest::new(req);
    let runtime = Runtime::new().unwrap();
    let context = Context::full(&runtime).unwrap();

    context.with(|ctx| {
        let global = ctx.globals();

        Class::<QueryParam>::define(&global).unwrap();
        Class::<HTTPRequest>::define(&global).unwrap();

        unsafe {
            let request_static: HTTPRequest<'static> = std::mem::transmute(request);
            let o = Class::<HTTPRequest>::instance(ctx.clone(), request_static).unwrap();
            global.set("request", o).unwrap();

            global
                .set(
                    "decode_utf8",
                    Func::from(|data: TypedArray<u8>| {
                        String::from_utf8_lossy(data.as_ref()).to_string()
                    }),
                )
                .unwrap();

            let result: String = ctx
                .eval(
                    r#"
                const method = decode_utf8(request.method);
                const url_path = decode_utf8(request.url_path);
                const port = request.port;
                const params = [];
                for (let i = 0; i < request.query_params.length; i++) {
                    const p = request.query_params[i];
                    params.push({
                        key: decode_utf8(p.key),
                        value: decode_utf8(p.value)
                    });
                }
                JSON.stringify({method, url_path, port, params})
            "#,
                )
                .unwrap();
            dbg!(result);
        };
    })
}

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
        url_path: b"/api/v1/users",
        query_params,
        port: 8080,
    };
    run_http_request(&http_request);
}
