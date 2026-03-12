use rquickjs::atom::PredefinedAtom;
use rquickjs::class::Trace;
use rquickjs::{Class, Ctx, JsLifetime, Null, Object, Result, Value};
use rquickjs::{Context, Runtime};

#[rquickjs::class]
#[derive(Clone, Trace, JsLifetime)]
struct MyClass {
    x: u32,
    y: u32,
}

#[rquickjs::methods(rename_all = "camelCase")]
impl MyClass {
    #[qjs(constructor)]
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    #[qjs(get)]
    fn x(&self) -> u32 {
        self.x
    }
}

fn main() {
    let runtime = Runtime::new().unwrap();
    let context = Context::full(&runtime).unwrap();
    context.with(|ctx| {

        let global = ctx.globals();

        Class::<MyClass>::define(&global).unwrap();
        // 这里的clone仅增加Rc计数
        let o = Class::<MyClass>::instance(ctx.clone(), MyClass { x: 114, y: 514 }).unwrap();
        global.set("o", o).unwrap();

        let result: u32 = ctx.eval(r#"
        o.x
        "#).unwrap();
        dbg!(result);
    })
}
