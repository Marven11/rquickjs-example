use rquickjs::class::Trace;
use rquickjs::prelude::Func;
use rquickjs::{Array, Class, Context, JsLifetime, Runtime, TypedArray};

#[derive(Clone)]
pub struct OriginalQueryParam<'a> {
    pub key: &'a [u8],
    pub value: &'a [u8],
}

#[derive(Clone)]
pub struct OriginalHTTPRequest<'a> {
    pub method: &'a [u8],
    pub url_path: &'a [u8],
    pub query_params: Vec<OriginalQueryParam<'a>>,
    pub port: u16,
}

#[rquickjs::class]
#[derive(Clone, Trace)]
pub struct QueryParam<'js> {
    #[qjs(skip_trace)]
    original: &'js OriginalQueryParam<'js>,
}

#[rquickjs::class]
#[derive(Clone, Trace)]
pub struct HTTPRequest<'js> {
    #[qjs(skip_trace)]
    original: &'js OriginalHTTPRequest<'js>,
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

pub struct EvalContext {
    _runtime: Runtime,
    context: Context,
}

impl EvalContext {
    pub fn new() -> rquickjs::Result<Self> {
        let runtime = Runtime::new()?;
        let context = Context::full(&runtime)?;

        context.with(|ctx| {
            let global = ctx.globals();

            Class::<QueryParam>::define(&global).unwrap();
            Class::<HTTPRequest>::define(&global).unwrap();

            global
                .set(
                    "decode_utf8",
                    Func::from(|data: TypedArray<u8>| {
                        String::from_utf8_lossy(data.as_ref()).to_string()
                    }),
                )
                .unwrap();

            Ok::<_, rquickjs::Error>(())
        })?;

        Ok(Self {
            _runtime: runtime,
            context,
        })
    }

    pub fn eval(&self, req: &OriginalHTTPRequest, expr: &str) -> rquickjs::Result<bool> {
        self.context.with(|ctx| {
            let global = ctx.globals();

            unsafe {
                let request = HTTPRequest::new(req);
                let request_static: HTTPRequest<'static> = std::mem::transmute(request);
                let o = Class::<HTTPRequest>::instance(ctx.clone(), request_static).unwrap();
                global.set("request", o).unwrap();
            }

            ctx.eval(expr)
        })
    }
}
