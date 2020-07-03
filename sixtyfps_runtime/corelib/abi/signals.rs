/*!
Signal that can be connected to  one sigle handler.

TODO: reconsider if we should rename that to `Event`
but then it should also be renamed everywhere, including in the language grammar
*/

use super::properties::EvaluationContext;

/// A Signal that can be connected to a handler.
///
/// The Arg represents the argument. It should always be a tuple
///
#[derive(Default)]
#[repr(C)]
pub struct Signal<Arg> {
    /// FIXME: Box<dyn> is a fat object and we probaly want to put an erased type in there
    handler: Option<Box<dyn Fn(&EvaluationContext, Arg)>>,
}

impl<Arg> Signal<Arg> {
    /// Emit the signal with the given argument.
    ///
    /// The constext must be a context corresponding to the component in which the signal is contained.
    pub fn emit(&self, context: &EvaluationContext, a: Arg) {
        if let Some(h) = &self.handler {
            h(context, a);
        }
    }

    /// Set an handler to be called when the signal is emited
    ///
    /// There can only be one single handler per signal.
    pub fn set_handler(&mut self, f: impl Fn(&EvaluationContext, Arg) + 'static) {
        self.handler = Some(Box::new(f));
    }
}

#[test]
fn signal_simple_test() {
    use std::pin::Pin;
    #[derive(Default)]
    struct Component {
        pressed: core::cell::Cell<bool>,
        clicked: Signal<()>,
    }
    impl crate::abi::datastructures::Component for Component {
        fn visit_children_item(
            self: Pin<&Self>,
            _: isize,
            _: crate::abi::datastructures::ItemVisitorRefMut,
        ) {
        }
        fn layout_info(self: Pin<&Self>) -> crate::abi::datastructures::LayoutInfo {
            unimplemented!()
        }
        fn compute_layout(self: Pin<&Self>, _: &crate::EvaluationContext) {}
    }
    use crate::abi::datastructures::ComponentVTable;
    let mut c = Component::default();
    c.clicked.set_handler(|c, ()| unsafe {
        (*(c.component.as_ptr() as *const Component)).pressed.set(true)
    });
    let vtable = ComponentVTable::new::<Component>();
    let ctx = super::properties::EvaluationContext::for_root_component(unsafe {
        Pin::new_unchecked(vtable::VRef::from_raw(
            core::ptr::NonNull::from(&vtable),
            core::ptr::NonNull::from(&c).cast(),
        ))
    });
    c.clicked.emit(&ctx, ());
    assert_eq!(c.pressed.get(), true);
}

#[allow(non_camel_case_types)]
type c_void = ();
#[repr(C)]
/// Has the same layout as Signal<()>
pub struct SignalOpaque(*const c_void, *const c_void);

static_assertions::assert_eq_align!(SignalOpaque, Signal<()>);
static_assertions::assert_eq_size!(SignalOpaque, Signal<()>);

/// Initialize the signal.
/// sixtyfps_signal_drop must be called.
#[no_mangle]
pub unsafe extern "C" fn sixtyfps_signal_init(out: *mut SignalOpaque) {
    assert_eq!(core::mem::size_of::<SignalOpaque>(), core::mem::size_of::<Signal<()>>());
    core::ptr::write(out as *mut Signal<()>, Default::default());
}

/// Emit the signal
#[no_mangle]
pub unsafe extern "C" fn sixtyfps_signal_emit(
    sig: *const SignalOpaque,
    component: &EvaluationContext,
) {
    let sig = &*(sig as *const Signal<()>);
    sig.emit(component, ());
}

/// Set signal handler.
///
/// The binding has signature fn(user_data, context)
#[no_mangle]
pub unsafe extern "C" fn sixtyfps_signal_set_handler(
    sig: *mut SignalOpaque,
    binding: extern "C" fn(*mut c_void, &EvaluationContext),
    user_data: *mut c_void,
    drop_user_data: Option<extern "C" fn(*mut c_void)>,
) {
    let sig = &mut *(sig as *mut Signal<()>);

    struct UserData {
        user_data: *mut c_void,
        drop_user_data: Option<extern "C" fn(*mut c_void)>,
    }

    impl Drop for UserData {
        fn drop(&mut self) {
            if let Some(x) = self.drop_user_data {
                x(self.user_data)
            }
        }
    }
    let ud = UserData { user_data, drop_user_data };

    let real_binding = move |compo: &EvaluationContext, ()| {
        binding(ud.user_data, compo);
    };
    sig.set_handler(real_binding);
}

/// Destroy signal
#[no_mangle]
pub unsafe extern "C" fn sixtyfps_signal_drop(handle: *mut SignalOpaque) {
    core::ptr::read(handle as *mut Signal<()>);
}

/// Somehow this is required for the extern "C" things to be exported in a dependent dynlib
#[doc(hidden)]
pub fn dummy() {
    #[derive(Clone)]
    struct Foo;
    foo(Foo);
    fn foo(f: impl Clone) {
        let _ = f.clone();
    }
}
