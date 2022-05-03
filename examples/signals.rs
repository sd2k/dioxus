use dioxus::prelude::*;
use std::future::Future;

fn main() {
    // let hea: &'static dyn MyThing = &header;
    // swall(header())
    // let thing = (&header as &dyn MyThing).eat();
    // let thing = header.eat();
}

fn swall<'a>(f: impl Future<Output = Element<'a>>) {}

trait MyThing {
    fn eat(&self) -> bool {
        true
    }
}

impl<'a, T: Future<Output = Element<'a>>> MyThing for fn(Scope<'a>) -> T {}

async fn header<'a>(cx: Scope<'a>) -> Element<'a> {
    // let name = use_signal(&cx, || 10);

    cx.render(rsx! {
        div {}
    })
}
