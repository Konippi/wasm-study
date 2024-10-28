#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod wasm_study {
        #[allow(dead_code)]
        pub mod say {
            #[allow(dead_code, clippy::all)]
            pub mod sayable {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_say_cabi<T: Guest>() -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::say();
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec2 = (result0.into_bytes()).into_boxed_slice();
                    let ptr2 = vec2.as_ptr().cast::<u8>();
                    let len2 = vec2.len();
                    ::core::mem::forget(vec2);
                    *ptr1.add(4).cast::<usize>() = len2;
                    *ptr1.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_say<T: Guest>(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    _rt::cabi_dealloc(l0, l1, 1);
                }
                pub trait Guest {
                    fn say() -> _rt::String;
                }
                #[doc(hidden)]
                macro_rules! __export_wasm_study_say_sayable_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name = "wasm-study:say/sayable#say"]
                        unsafe extern "C" fn export_say() -> * mut u8 {
                        $($path_to_types)*:: _export_say_cabi::<$ty > () } #[export_name
                        = "cabi_post_wasm-study:say/sayable#say"] unsafe extern "C" fn
                        _post_return_say(arg0 : * mut u8,) { $($path_to_types)*::
                        __post_return_say::<$ty > (arg0) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_wasm_study_say_sayable_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 8]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 8],
                );
            }
        }
    }
}
mod _rt {
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    pub use alloc_crate::string::String;
    pub use alloc_crate::alloc;
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
macro_rules! __export_sayable_provider_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::wasm_study::say::sayable::__export_wasm_study_say_sayable_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::wasm_study::say::sayable);
    };
}
#[doc(inline)]
pub(crate) use __export_sayable_provider_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.31.0:wasm-study:say:sayable-provider:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 219] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07U\x01A\x02\x01A\x02\x01\
B\x02\x01@\0\0s\x04\0\x03say\x01\0\x04\x01\x16wasm-study:say/sayable\x05\0\x04\x01\
\x1fwasm-study:say/sayable-provider\x04\0\x0b\x16\x01\0\x10sayable-provider\x03\0\
\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.216.0\x10wit-bi\
ndgen-rust\x060.31.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
