// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::{ffi};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_};

glib::wrapper! {
    /// This class is the basic building block for the zypp glib API. It defines the path of the
    /// root filesystem we are operating on. This is usually "/" but to support chroot use cases it
    /// can point to any directory in a filesystem where packages should be installed into. If the rootfs
    /// is not defined as "/" then zypp will install packages using chroot into the directory.
    ///
    /// Settings for zypp are loaded from the rootfs directory and locks are also applied relative to it.
    /// Meaning that one context can operate on "/" while another one can operate on "/tmp/rootfs".
    ///
    /// \note Currently only one ZyppContext is supported until we have refactored the underlying code to support
    ///  having multiple of them. Mixing them atm will not work due to locks and libsolv limitations
    ///
    /// ## Properties
    ///
    ///
    /// #### `versionprop`
    ///  Readable
    ///
    ///
    /// #### `zypp-cppObj`
    ///  Writeable | Construct Only
    #[doc(alias = "ZyppContext")]
    pub struct Context(Object<ffi::ZyppContext, ffi::ZyppContextClass>);

    match fn {
        type_ => || ffi::zypp_context_get_type(),
    }
}

impl Context {
            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`Context`] objects.
            ///
            /// This method returns an instance of [`ContextBuilder`](crate::builders::ContextBuilder) which can be used to create [`Context`] objects.
            pub fn builder() -> ContextBuilder {
                ContextBuilder::new()
            }
        

    /// Loads the system at the given sysroot, returns TRUE on success, otherwise FALSE
    /// ## `sysRoot`
    /// The system sysroot to load, if a nullptr is given "/" is used
    #[doc(alias = "zypp_context_load_system")]
    pub fn load_system(&self, sysRoot: Option<&str>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::zypp_context_load_system(self.to_glib_none().0, sysRoot.to_glib_none().0, &mut error);
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    ///
    /// # Returns
    ///
    /// The context root as requested when loading the system
    #[doc(alias = "zypp_context_sysroot")]
    pub fn sysroot(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::zypp_context_sysroot(self.to_glib_none().0))
        }
    }

    #[doc(alias = "zypp_context_version")]
    pub fn version(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::zypp_context_version(self.to_glib_none().0))
        }
    }

    pub fn versionprop(&self) -> Option<glib::GString> {
        ObjectExt::property(self, "versionprop")
    }

    #[doc(alias = "versionprop")]
    pub fn connect_versionprop_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_versionprop_trampoline<F: Fn(&Context) + 'static>(this: *mut ffi::ZyppContext, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::versionprop\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(notify_versionprop_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }
}

// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`Context`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ContextBuilder {
            builder: glib::object::ObjectBuilder<'static, Context>,
        }

        impl ContextBuilder {
        fn new() -> Self {
            Self { builder: glib::object::Object::builder() }
        }

                            //pub fn zypp_cppObj(self, zypp_cppObj: /*Unimplemented*/Basic: Pointer) -> Self {
                        //    Self { builder: self.builder.property("zypp-cppObj", zypp_cppObj), }
                        //}

    // rustdoc-stripper-ignore-next
    /// Build the [`Context`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Context {
    self.builder.build() }
}
