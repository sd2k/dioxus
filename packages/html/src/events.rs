macro_rules! impl_event {
    (
        $data:ty;
        $(
            $( #[$attr:meta] )*
            $name:ident
        )*
    ) => {
        $(
            $( #[$attr] )*
            pub fn $name<'a>(_cx: &'a ::dioxus_core::ScopeState, _f: impl FnMut(::dioxus_core::Event<$data>) + 'a) -> ::dioxus_core::Attribute<'a> {
                ::dioxus_core::Attribute {
                    name: stringify!($name),
                    value: _cx.listener(_f),
                    namespace: None,
                    mounted_element: Default::default(),
                    volatile: false,
                }
            }
        )*
    };
}

mod animation;
mod clipboard;
mod composition;
mod drag;
mod focus;
mod form;
mod image;
mod keyboard;
mod media;
mod mouse;
mod pointer;
mod scroll;
mod selection;
mod toggle;
mod touch;
mod transition;
mod wheel;

pub use animation::*;
pub use clipboard::*;
pub use composition::*;
pub use drag::*;
pub use focus::*;
pub use form::*;
pub use image::*;
pub use keyboard::*;
pub use media::*;
pub use mouse::*;
pub use pointer::*;
pub use scroll::*;
pub use selection::*;
pub use toggle::*;
pub use touch::*;
pub use transition::*;
pub use wheel::*;
