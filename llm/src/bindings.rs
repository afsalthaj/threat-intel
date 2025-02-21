// Generated by `wit-bindgen` 0.36.0. DO NOT EDIT!
// Options used:
//   * runtime_path: "wit_bindgen_rt"
#[rustfmt::skip]
#[allow(dead_code, clippy::all)]
pub mod exports {
    pub mod rag {
        pub mod llm_exports {
            #[allow(dead_code, clippy::all)]
            pub mod api {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[derive(Clone)]
                pub struct Context {
                    pub value: _rt::String,
                }
                impl ::core::fmt::Debug for Context {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Context").field("value", &self.value).finish()
                    }
                }
                #[derive(Clone)]
                pub struct LlmResponse {
                    pub value: _rt::String,
                }
                impl ::core::fmt::Debug for LlmResponse {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("LlmResponse")
                            .field("value", &self.value)
                            .finish()
                    }
                }
                #[derive(Clone)]
                pub struct Prompt {
                    pub description: _rt::String,
                }
                impl ::core::fmt::Debug for Prompt {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Prompt")
                            .field("description", &self.description)
                            .finish()
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_ask_model_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                    arg2: *mut u8,
                    arg3: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let len1 = arg3;
                    let bytes1 = _rt::Vec::from_raw_parts(arg2.cast(), len1, len1);
                    let result2 = T::ask_model(
                        Prompt {
                            description: _rt::string_lift(bytes0),
                        },
                        Context {
                            value: _rt::string_lift(bytes1),
                        },
                    );
                    let ptr3 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result2 {
                        Ok(e) => {
                            *ptr3.add(0).cast::<u8>() = (0i32) as u8;
                            let LlmResponse { value: value4 } = e;
                            let vec5 = (value4.into_bytes()).into_boxed_slice();
                            let ptr5 = vec5.as_ptr().cast::<u8>();
                            let len5 = vec5.len();
                            ::core::mem::forget(vec5);
                            *ptr3.add(8).cast::<usize>() = len5;
                            *ptr3.add(4).cast::<*mut u8>() = ptr5.cast_mut();
                        }
                        Err(e) => {
                            *ptr3.add(0).cast::<u8>() = (1i32) as u8;
                            let vec6 = (e.into_bytes()).into_boxed_slice();
                            let ptr6 = vec6.as_ptr().cast::<u8>();
                            let len6 = vec6.len();
                            ::core::mem::forget(vec6);
                            *ptr3.add(8).cast::<usize>() = len6;
                            *ptr3.add(4).cast::<*mut u8>() = ptr6.cast_mut();
                        }
                    };
                    ptr3
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_ask_model<T: Guest>(arg0: *mut u8) {
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
                    fn ask_model(
                        prompt: Prompt,
                        context: Context,
                    ) -> Result<LlmResponse, _rt::String>;
                }
                #[doc(hidden)]
                macro_rules! __export_rag_llm_exports_api_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name = "rag:llm-exports/api#ask-model"]
                        unsafe extern "C" fn export_ask_model(arg0 : * mut u8, arg1 :
                        usize, arg2 : * mut u8, arg3 : usize,) -> * mut u8 {
                        $($path_to_types)*:: _export_ask_model_cabi::<$ty > (arg0, arg1,
                        arg2, arg3) } #[export_name =
                        "cabi_post_rag:llm-exports/api#ask-model"] unsafe extern "C" fn
                        _post_return_ask_model(arg0 : * mut u8,) { $($path_to_types)*::
                        __post_return_ask_model::<$ty > (arg0) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_rag_llm_exports_api_cabi;
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
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub use alloc_crate::vec::Vec;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    extern crate alloc as alloc_crate;
    pub use alloc_crate::alloc;
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
macro_rules! __export_llm_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::rag::llm_exports::api::__export_rag_llm_exports_api_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::rag::llm_exports::api);
    };
}
#[doc(inline)]
pub(crate) use __export_llm_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.36.0:rag:llm:llm:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 292] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xaa\x01\x01A\x02\x01\
A\x02\x01B\x09\x01r\x01\x05values\x04\0\x07context\x03\0\0\x01r\x01\x05values\x04\
\0\x0cllm-response\x03\0\x02\x01r\x01\x0bdescriptions\x04\0\x06prompt\x03\0\x04\x01\
j\x01\x03\x01s\x01@\x02\x06prompt\x05\x07context\x01\0\x06\x04\0\x09ask-model\x01\
\x07\x04\0\x13rag:llm-exports/api\x05\0\x04\0\x0brag:llm/llm\x04\0\x0b\x09\x01\0\
\x03llm\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.22\
0.0\x10wit-bindgen-rust\x060.36.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
