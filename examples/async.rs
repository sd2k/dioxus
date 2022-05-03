use dioxus::prelude::*;

fn main() {
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            h1 { "Random Image" }
            RandomImage {}
        }
        div {
            h1 { "Set async" }
            RandomImage {}
        }
    })
}

async fn fetch_dog() -> String {
    reqwest::get("https://dog.ceo/api/breeds/image/random")
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap()
        .get("message")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string()
}

fn RandomImage(cx: Scope) -> Element {
    // a simple use_future that will eventually resolve to a value
    let random_image = use_future(&cx, (), |_| fetch_dog());

    cx.render(match random_image.value() {
        Some(src) => rsx! { img { src: "{src}", height: "300px" } },
        None => rsx! { "Loading..." },
    })
}

fn SetAsync1(cx: Scope) -> Element {
    let image_src = use_state(&cx, || None);

    cx.render(match image_src.get() {
        Some(src) => rsx! { img { src: "{src}", height: "300px" } },
        None => rsx! { button {
            onclick: move |_| image_src.set_async(fetch_dog()),
            "Load Image"
        } },
    })
}

fn SetAsync2(cx: Scope) -> Element {
    let image_src = use_state(&cx, || None);

    cx.render(match image_src.get() {
        Some(src) => rsx! { img { src: "{src}", height: "300px" } },
        None => rsx! { button {
            onclick: move |_| {
                let image_src = image_src.clone();
                cx.spawn(async move {
                    let src = fetch_dog().await;
                    image_src.set(Some(src));
                })
            },
            "Load Image"
        } },
    })
}
