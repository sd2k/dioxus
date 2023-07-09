use bumpalo::Bump;
use std::cell::{Cell, UnsafeCell};

use crate::{Element, VPlaceholder};

pub(crate) struct BumpFrame {
    pub bump: UnsafeCell<Bump>,
    pub node: Cell<*const Element<'static>>,
    pub placeholder: Cell<Option<VPlaceholder>>,
}

impl BumpFrame {
    pub(crate) fn new(capacity: usize) -> Self {
        let bump = Bump::with_capacity(capacity);
        Self {
            bump: UnsafeCell::new(bump),
            node: Cell::new(std::ptr::null()),
            placeholder: Cell::new(None),
        }
    }

    /// Creates a new lifetime out of thin air
    pub(crate) unsafe fn try_load_node<'b>(&self) -> Option<&'b Element<'b>> {
        let node = self.node.get();

        if node.is_null() {
            return None;
        }

        unsafe { std::mem::transmute(&*node) }
    }

    pub(crate) fn bump(&self) -> &Bump {
        unsafe { &*self.bump.get() }
    }

    #[allow(clippy::mut_from_ref)]
    pub(crate) unsafe fn bump_mut(&self) -> &mut Bump {
        unsafe { &mut *self.bump.get() }
    }
}
