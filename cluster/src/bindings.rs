// Generated by `wit-bindgen` 0.36.0. DO NOT EDIT!
// Options used:
//   * runtime_path: "wit_bindgen_rt"
#[rustfmt::skip]
#[allow(dead_code, clippy::all)]
pub mod exports {
    pub mod rag {
        pub mod cluster_exports {
            #[allow(dead_code, clippy::all)]
            pub mod api {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[derive(Clone)]
                pub struct ClusterInput {
                    pub log_line: _rt::String,
                    pub embedding: _rt::Vec<f32>,
                }
                impl ::core::fmt::Debug for ClusterInput {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("ClusterInput")
                            .field("log-line", &self.log_line)
                            .field("embedding", &self.embedding)
                            .finish()
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_get_alert_messages_cabi<T: Guest>() -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::get_alert_messages();
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result0 {
                        Ok(e) => {
                            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
                            let vec3 = e;
                            let len3 = vec3.len();
                            let layout3 = _rt::alloc::Layout::from_size_align_unchecked(
                                vec3.len() * 8,
                                4,
                            );
                            let result3 = if layout3.size() != 0 {
                                let ptr = _rt::alloc::alloc(layout3).cast::<u8>();
                                if ptr.is_null() {
                                    _rt::alloc::handle_alloc_error(layout3);
                                }
                                ptr
                            } else {
                                ::core::ptr::null_mut()
                            };
                            for (i, e) in vec3.into_iter().enumerate() {
                                let base = result3.add(i * 8);
                                {
                                    let vec2 = (e.into_bytes()).into_boxed_slice();
                                    let ptr2 = vec2.as_ptr().cast::<u8>();
                                    let len2 = vec2.len();
                                    ::core::mem::forget(vec2);
                                    *base.add(4).cast::<usize>() = len2;
                                    *base.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                                }
                            }
                            *ptr1.add(8).cast::<usize>() = len3;
                            *ptr1.add(4).cast::<*mut u8>() = result3;
                        }
                        Err(e) => {
                            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
                            let vec4 = (e.into_bytes()).into_boxed_slice();
                            let ptr4 = vec4.as_ptr().cast::<u8>();
                            let len4 = vec4.len();
                            ::core::mem::forget(vec4);
                            *ptr1.add(8).cast::<usize>() = len4;
                            *ptr1.add(4).cast::<*mut u8>() = ptr4.cast_mut();
                        }
                    };
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_get_alert_messages<T: Guest>(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            let base5 = l1;
                            let len5 = l2;
                            for i in 0..len5 {
                                let base = base5.add(i * 8);
                                {
                                    let l3 = *base.add(0).cast::<*mut u8>();
                                    let l4 = *base.add(4).cast::<usize>();
                                    _rt::cabi_dealloc(l3, l4, 1);
                                }
                            }
                            _rt::cabi_dealloc(base5, len5 * 8, 4);
                        }
                        _ => {
                            let l6 = *arg0.add(4).cast::<*mut u8>();
                            let l7 = *arg0.add(8).cast::<usize>();
                            _rt::cabi_dealloc(l6, l7, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_process_cluster_input_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                    arg2: *mut u8,
                    arg3: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let len1 = arg3;
                    let result2 = T::process_cluster_input(ClusterInput {
                        log_line: _rt::string_lift(bytes0),
                        embedding: _rt::Vec::from_raw_parts(arg2.cast(), len1, len1),
                    });
                    let ptr3 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result2 {
                        Ok(e) => {
                            *ptr3.add(0).cast::<u8>() = (0i32) as u8;
                            let vec4 = (e.into_bytes()).into_boxed_slice();
                            let ptr4 = vec4.as_ptr().cast::<u8>();
                            let len4 = vec4.len();
                            ::core::mem::forget(vec4);
                            *ptr3.add(8).cast::<usize>() = len4;
                            *ptr3.add(4).cast::<*mut u8>() = ptr4.cast_mut();
                        }
                        Err(e) => {
                            *ptr3.add(0).cast::<u8>() = (1i32) as u8;
                            let vec5 = (e.into_bytes()).into_boxed_slice();
                            let ptr5 = vec5.as_ptr().cast::<u8>();
                            let len5 = vec5.len();
                            ::core::mem::forget(vec5);
                            *ptr3.add(8).cast::<usize>() = len5;
                            *ptr3.add(4).cast::<*mut u8>() = ptr5.cast_mut();
                        }
                    };
                    ptr3
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_process_cluster_input<T: Guest>(
                    arg0: *mut u8,
                ) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                        _ => {
                            let l3 = *arg0.add(4).cast::<*mut u8>();
                            let l4 = *arg0.add(8).cast::<usize>();
                            _rt::cabi_dealloc(l3, l4, 1);
                        }
                    }
                }
                pub trait Guest {
                    fn get_alert_messages() -> Result<
                        _rt::Vec<_rt::String>,
                        _rt::String,
                    >;
                    fn process_cluster_input(
                        log: ClusterInput,
                    ) -> Result<_rt::String, _rt::String>;
                }
                #[doc(hidden)]
                macro_rules! __export_rag_cluster_exports_api_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "rag:cluster-exports/api#get-alert-messages"] unsafe extern "C"
                        fn export_get_alert_messages() -> * mut u8 { $($path_to_types)*::
                        _export_get_alert_messages_cabi::<$ty > () } #[export_name =
                        "cabi_post_rag:cluster-exports/api#get-alert-messages"] unsafe
                        extern "C" fn _post_return_get_alert_messages(arg0 : * mut u8,) {
                        $($path_to_types)*:: __post_return_get_alert_messages::<$ty >
                        (arg0) } #[export_name =
                        "rag:cluster-exports/api#process-cluster-input"] unsafe extern
                        "C" fn export_process_cluster_input(arg0 : * mut u8, arg1 :
                        usize, arg2 : * mut u8, arg3 : usize,) -> * mut u8 {
                        $($path_to_types)*:: _export_process_cluster_input_cabi::<$ty >
                        (arg0, arg1, arg2, arg3) } #[export_name =
                        "cabi_post_rag:cluster-exports/api#process-cluster-input"] unsafe
                        extern "C" fn _post_return_process_cluster_input(arg0 : * mut
                        u8,) { $($path_to_types)*::
                        __post_return_process_cluster_input::<$ty > (arg0) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_rag_cluster_exports_api_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 12]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 12],
                );
            }
        }
    }
}
#[rustfmt::skip]
mod _rt {
    pub use alloc_crate::string::String;
    pub use alloc_crate::vec::Vec;
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub use alloc_crate::alloc;
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    extern crate alloc as alloc_crate;
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_cluster_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::rag::cluster_exports::api::__export_rag_cluster_exports_api_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::rag::cluster_exports::api);
    };
}
#[doc(inline)]
pub(crate) use __export_cluster_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.36.0:rag:cluster:cluster:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 312] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xba\x01\x01A\x02\x01\
A\x02\x01B\x0a\x01pv\x01r\x02\x08log-lines\x09embedding\0\x04\0\x0dcluster-input\
\x03\0\x01\x01ps\x01j\x01\x03\x01s\x01@\0\0\x04\x04\0\x12get-alert-messages\x01\x05\
\x01j\x01s\x01s\x01@\x01\x03log\x02\0\x06\x04\0\x15process-cluster-input\x01\x07\
\x04\0\x17rag:cluster-exports/api\x05\0\x04\0\x13rag:cluster/cluster\x04\0\x0b\x0d\
\x01\0\x07cluster\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-compone\
nt\x070.220.0\x10wit-bindgen-rust\x060.36.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
