use std::marker::PhantomData;

use dioxus::prelude::*;

fn main() {}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            Child::<MyThing> {}
        }
    })
}

trait Message {
    const MSG: &'static str;
}

struct MyThing {}
impl Message for MyThing {
    const MSG: &'static str = "hello";
}

struct MyStruct<T: Message> {
    _p: PhantomData<T>,
}

fn eat<T: Message>() {
    //
}

// fn Child<T: Message>(cx: Scope) -> Element {
//     let me = <T as Message>::MSG;
//     cx.render(rsx! {
//         div {
//             p {
//                 "Hello {me}"
//             }
//         }
//     })
// }
