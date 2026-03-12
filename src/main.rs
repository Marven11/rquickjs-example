use rquickjs::atom::PredefinedAtom;
use rquickjs::class::Trace;
use rquickjs::{Class, Ctx, JsLifetime, Null, Object, Result, Value};
use rquickjs::{Context, Runtime};

#[rquickjs::class]
#[derive(Clone, Trace, JsLifetime)]
struct MyClass {
    #[qjs(skip_trace)]
    name: &'static str
}

impl MyClass {

    pub fn new(name: &'static str) -> Self {
        Self { name }
    }
}

#[rquickjs::methods]
impl MyClass {
    #[qjs(get)]
    fn name(&self) -> String {
        self.name.to_string()
    }
}

fn main() {
    let runtime = Runtime::new().unwrap();
    let context = Context::full(&runtime).unwrap();
    context.with(|ctx| {

        let global = ctx.globals();

        Class::<MyClass>::define(&global).unwrap();
        // 这里的clone仅增加Rc计数
        let o = Class::<MyClass>::instance(ctx.clone(), MyClass { name: "litiansuo" }).unwrap();
        global.set("o", o).unwrap();

        let result: String = ctx.eval(r#"
        o.name
        "#).unwrap();
        dbg!(result);
    })
}
