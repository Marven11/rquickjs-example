use rquickjs::class::Trace;
use rquickjs::{Class, JsLifetime};
use rquickjs::{Context, Runtime};

#[derive(Clone)]
struct MyOriginalClass<'custom> {
    name: &'custom str,
}

#[rquickjs::class]
#[derive(Clone, Trace)]
struct MyClass<'js> {
    #[qjs(skip_trace)]
    original: &'js MyOriginalClass<'js>,
}

unsafe impl<'js> JsLifetime<'js> for MyClass<'js> {
    type Changed<'to> = MyClass<'to>;
}

impl<'js> MyClass<'js> {
    pub fn new<'other: 'js>(original: &'js MyOriginalClass<'other>) -> Self {
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

fn run<'a, 'b: 'a>(r: &'a MyOriginalClass<'b>) {
    let a: MyClass<'a> = MyClass { original: r };
    let runtime = Runtime::new().unwrap();
    let context = Context::full(&runtime).unwrap();

    context.with(|ctx| {
        let global = ctx.globals();

        Class::<MyClass>::define(&global).unwrap();
        unsafe {
            let a = std::mem::transmute::<MyClass<'a>, MyClass<'static>>(a);
            let o = Class::<MyClass>::instance(ctx.clone(), a).unwrap();
            global.set("o", o).unwrap();
            let result: String = ctx.eval(r#"o.name"#).unwrap();
            dbg!(result);
        };
    })
}

fn main() {
    let original = MyOriginalClass { name: "litiansuo" };
    run(&original);
}
