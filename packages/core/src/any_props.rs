use crate::{
    innerlude::Scoped,
    nodes::RenderReturn,
    scopes::{Scope, ScopeState},
    Element,
};
use std::{any::Any, panic::AssertUnwindSafe};

/// A trait that essentially allows VComponentProps to be used generically
///
/// # Safety
///
/// This should not be implemented outside this module
pub(crate) trait AnyProps<'a> {
    fn render(&'a self, bump: &'a ScopeState) -> RenderReturn;
    fn memoize(&self, other: &dyn AnyProps) -> bool;
    fn props(&self) -> &dyn Any;
}

pub(crate) struct VProps<'a, P: 'static> {
    pub render_fn: fn(Scope<'a, P>) -> Element<'a>,
    pub memo: fn(&P, &P) -> bool,
    pub props: P,
}

impl<'a, P: 'static> VProps<'a, P> {
    pub(crate) fn new(
        render_fn: fn(Scope<'a, P>) -> Element<'a>,
        memo: fn(&P, &P) -> bool,
        props: P,
    ) -> Self {
        Self {
            render_fn,
            memo,
            props,
        }
    }
}

impl<'a, P> AnyProps<'a> for VProps<'a, P> {
    // Safety:
    // this will downcast the other ptr as our swallowed type!
    // you *must* make this check *before* calling this method
    // if your functions are not the same, then you will downcast a pointer into a different type (UB)
    fn memoize(&self, other: &dyn AnyProps) -> bool {
        let real_us = &self.props;
        let real_other = other.props().downcast_ref::<P>().unwrap();
        (self.memo)(real_us, real_other)
    }

    fn render(&'a self, cx: &'a ScopeState) -> RenderReturn<'a> {
        let res = std::panic::catch_unwind(AssertUnwindSafe(move || {
            // Call the render function directly
            let scope: &mut Scoped<P> = cx.bump().alloc(Scoped {
                props: &self.props,
                scope: cx,
            });

            (self.render_fn)(scope)
        }));

        match res {
            Ok(Some(e)) => RenderReturn::Ready(e),
            Ok(None) => RenderReturn::default(),
            Err(err) => {
                let component_name = cx.name();
                log::error!("Error while rendering component `{component_name}`: {err:?}");
                RenderReturn::default()
            }
        }
    }

    fn props(&self) -> &dyn Any {
        &self.props
    }
}
