use dioxus_core::Event;

use crate::{FileEngine, MouseData};

pub type DragEvent = Event<DragData>;

/// The DragEvent interface is a DOM event that represents a drag and drop interaction. The user initiates a drag by
/// placing a pointer device (such as a mouse) on the touch surface and then dragging the pointer to a new location
/// (such as another DOM element). Applications are free to interpret a drag and drop interaction in an
/// application-specific way.
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub struct DragData {
    /// Inherit mouse data
    pub mouse: MouseData,

    #[cfg_attr(feature = "serialize", serde(skip))]
    pub files: Option<std::sync::Arc<dyn FileEngine>>,
}

impl PartialEq for DragData {
    fn eq(&self, other: &Self) -> bool {
        self.mouse == other.mouse
    }
}

impl std::fmt::Debug for DragData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FormEvent")
            .field("value", &self.mouse)
            .finish()
    }
}

impl_event! {
    DragData;

    /// ondrag
    ondrag

    /// ondragend
    ondragend

    /// ondragenter
    ondragenter

    /// ondragexit
    ondragexit

    /// ondragleave
    ondragleave

    /// ondragover
    ondragover

    /// ondragstart
    ondragstart

    /// ondrop
    ondrop
}
