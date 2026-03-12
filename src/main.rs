use rquickjs::atom::PredefinedAtom;
use rquickjs::class::Trace;
use rquickjs::{Class, Ctx, JsLifetime, Null, Object, Result, Value};
use rquickjs::{Context, Runtime};

#[derive(Clone)]
struct MyOriginalClass<'custom> {
    name: &'custom str,
}

#[rquickjs::class]
#[derive(Clone, Trace)]
struct MyClass<'js> {
    #[qjs(skip_trace)]
    original: MyOriginalClass<'js>,
}

unsafe impl<'js> JsLifetime<'js> for MyClass<'js> {
    type Changed<'to> = MyClass<'to>;
}

impl<'js> MyClass<'js> {
    pub fn new<'custom: 'js>(original: MyOriginalClass<'custom>) -> Self {
        Self { original }
    }
}

#[rquickjs::methods]
impl<'js> MyClass<'js> {
    #[qjs(get)]
    fn name(&self) -> String {
        self.original.name.to_string()
    }
}

fn main() {
    let runtime = Runtime::new().unwrap();
    let context = Context::full(&runtime).unwrap();
    context.with(|ctx| {
        let global = ctx.globals();

        Class::<MyClass>::define(&global).unwrap();
        // 这里的clone仅增加Rc计数
        let o = Class::<MyClass>::instance(
            ctx.clone(),
            MyClass {
                original: MyOriginalClass { name: "litiansuo" },
            },
        )
        .unwrap();
        global.set("o", o).unwrap();

        let result: String = ctx
            .eval(
                r#"
        o.name
        "#,
            )
            .unwrap();
        dbg!(result);
    })
}
