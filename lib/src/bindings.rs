#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod component {
        #[allow(dead_code)]
        pub mod face_detection_lib {
            #[allow(dead_code, clippy::all)]
            pub mod face_detection {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                /// Error returned by image detection components.
                #[derive(Clone)]
                pub enum DetectionError {
                    ModelError(_rt::String),
                    ImageError(_rt::String),
                    IoError(_rt::String),
                    Unknown(_rt::String),
                    Unclassified,
                }
                impl ::core::fmt::Debug for DetectionError {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        match self {
                            DetectionError::ModelError(e) => {
                                f.debug_tuple("DetectionError::ModelError")
                                    .field(e)
                                    .finish()
                            }
                            DetectionError::ImageError(e) => {
                                f.debug_tuple("DetectionError::ImageError")
                                    .field(e)
                                    .finish()
                            }
                            DetectionError::IoError(e) => {
                                f.debug_tuple("DetectionError::IoError").field(e).finish()
                            }
                            DetectionError::Unknown(e) => {
                                f.debug_tuple("DetectionError::Unknown").field(e).finish()
                            }
                            DetectionError::Unclassified => {
                                f.debug_tuple("DetectionError::Unclassified").finish()
                            }
                        }
                    }
                }
                impl ::core::fmt::Display for DetectionError {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        write!(f, "{:?}", self)
                    }
                }
                impl std::error::Error for DetectionError {}
                /// Type representing an image.
                pub type Image = _rt::Vec<u8>;
                /// Resulting box of a detection operation which describes the coordinates
                /// of the surrounding box.
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct DetectionResult {
                    pub x1: f32,
                    pub y1: f32,
                    pub x2: f32,
                    pub y2: f32,
                    pub confidence: f32,
                }
                impl ::core::fmt::Debug for DetectionResult {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("DetectionResult")
                            .field("x1", &self.x1)
                            .field("y1", &self.y1)
                            .field("x2", &self.x2)
                            .field("y2", &self.y2)
                            .field("confidence", &self.confidence)
                            .finish()
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_detect_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                    arg2: f32,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let result1 = T::detect(
                        _rt::Vec::from_raw_parts(arg0.cast(), len0, len0),
                        arg2,
                    );
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result1 {
                        Ok(e) => {
                            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                            let vec3 = (e).into_boxed_slice();
                            let ptr3 = vec3.as_ptr().cast::<u8>();
                            let len3 = vec3.len();
                            ::core::mem::forget(vec3);
                            *ptr2.add(8).cast::<usize>() = len3;
                            *ptr2.add(4).cast::<*mut u8>() = ptr3.cast_mut();
                        }
                        Err(e) => {
                            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                            match e {
                                DetectionError::ModelError(e) => {
                                    *ptr2.add(4).cast::<u8>() = (0i32) as u8;
                                    let vec4 = (e.into_bytes()).into_boxed_slice();
                                    let ptr4 = vec4.as_ptr().cast::<u8>();
                                    let len4 = vec4.len();
                                    ::core::mem::forget(vec4);
                                    *ptr2.add(12).cast::<usize>() = len4;
                                    *ptr2.add(8).cast::<*mut u8>() = ptr4.cast_mut();
                                }
                                DetectionError::ImageError(e) => {
                                    *ptr2.add(4).cast::<u8>() = (1i32) as u8;
                                    let vec5 = (e.into_bytes()).into_boxed_slice();
                                    let ptr5 = vec5.as_ptr().cast::<u8>();
                                    let len5 = vec5.len();
                                    ::core::mem::forget(vec5);
                                    *ptr2.add(12).cast::<usize>() = len5;
                                    *ptr2.add(8).cast::<*mut u8>() = ptr5.cast_mut();
                                }
                                DetectionError::IoError(e) => {
                                    *ptr2.add(4).cast::<u8>() = (2i32) as u8;
                                    let vec6 = (e.into_bytes()).into_boxed_slice();
                                    let ptr6 = vec6.as_ptr().cast::<u8>();
                                    let len6 = vec6.len();
                                    ::core::mem::forget(vec6);
                                    *ptr2.add(12).cast::<usize>() = len6;
                                    *ptr2.add(8).cast::<*mut u8>() = ptr6.cast_mut();
                                }
                                DetectionError::Unknown(e) => {
                                    *ptr2.add(4).cast::<u8>() = (3i32) as u8;
                                    let vec7 = (e.into_bytes()).into_boxed_slice();
                                    let ptr7 = vec7.as_ptr().cast::<u8>();
                                    let len7 = vec7.len();
                                    ::core::mem::forget(vec7);
                                    *ptr2.add(12).cast::<usize>() = len7;
                                    *ptr2.add(8).cast::<*mut u8>() = ptr7.cast_mut();
                                }
                                DetectionError::Unclassified => {
                                    *ptr2.add(4).cast::<u8>() = (4i32) as u8;
                                }
                            }
                        }
                    };
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_detect<T: Guest>(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            let base3 = l1;
                            let len3 = l2;
                            _rt::cabi_dealloc(base3, len3 * 20, 4);
                        }
                        _ => {
                            let l4 = i32::from(*arg0.add(4).cast::<u8>());
                            match l4 {
                                0 => {
                                    let l5 = *arg0.add(8).cast::<*mut u8>();
                                    let l6 = *arg0.add(12).cast::<usize>();
                                    _rt::cabi_dealloc(l5, l6, 1);
                                }
                                1 => {
                                    let l7 = *arg0.add(8).cast::<*mut u8>();
                                    let l8 = *arg0.add(12).cast::<usize>();
                                    _rt::cabi_dealloc(l7, l8, 1);
                                }
                                2 => {
                                    let l9 = *arg0.add(8).cast::<*mut u8>();
                                    let l10 = *arg0.add(12).cast::<usize>();
                                    _rt::cabi_dealloc(l9, l10, 1);
                                }
                                3 => {
                                    let l11 = *arg0.add(8).cast::<*mut u8>();
                                    let l12 = *arg0.add(12).cast::<usize>();
                                    _rt::cabi_dealloc(l11, l12, 1);
                                }
                                _ => {}
                            }
                        }
                    }
                }
                pub trait Guest {
                    /// Detect faces on a given image.
                    fn detect(
                        img: Image,
                        confidence_treshold: f32,
                    ) -> Result<_rt::Vec<DetectionResult>, DetectionError>;
                }
                #[doc(hidden)]
                macro_rules! __export_component_face_detection_lib_face_detection_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "component:face-detection-lib/face-detection#detect"] unsafe
                        extern "C" fn export_detect(arg0 : * mut u8, arg1 : usize, arg2 :
                        f32,) -> * mut u8 { $($path_to_types)*::
                        _export_detect_cabi::<$ty > (arg0, arg1, arg2) } #[export_name =
                        "cabi_post_component:face-detection-lib/face-detection#detect"]
                        unsafe extern "C" fn _post_return_detect(arg0 : * mut u8,) {
                        $($path_to_types)*:: __post_return_detect::<$ty > (arg0) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_component_face_detection_lib_face_detection_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 16]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 16],
                );
            }
        }
    }
}
mod _rt {
    pub use alloc_crate::string::String;
    pub use alloc_crate::vec::Vec;
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
macro_rules! __export_classification_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::component::face_detection_lib::face_detection::__export_component_face_detection_lib_face_detection_cabi!($ty
        with_types_in $($path_to_types_root)*::
        exports::component::face_detection_lib::face_detection);
    };
}
#[doc(inline)]
pub(crate) use __export_classification_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.31.0:component:face-detection-lib:classification:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 448] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xbb\x02\x01A\x02\x01\
A\x02\x01B\x0a\x01q\x05\x0bmodel-error\x01s\0\x0bimage-error\x01s\0\x08io-error\x01\
s\0\x07unknown\x01s\0\x0cunclassified\0\0\x04\0\x0fdetection-error\x03\0\0\x01p}\
\x04\0\x05image\x03\0\x02\x01r\x05\x02x1v\x02y1v\x02x2v\x02y2v\x0aconfidencev\x04\
\0\x10detection-result\x03\0\x04\x01p\x05\x01j\x01\x06\x01\x01\x01@\x02\x03img\x03\
\x13confidence-tresholdv\0\x07\x04\0\x06detect\x01\x08\x04\x01+component:face-de\
tection-lib/face-detection\x05\0\x04\x01+component:face-detection-lib/classifica\
tion\x04\0\x0b\x14\x01\0\x0eclassification\x03\0\0\0G\x09producers\x01\x0cproces\
sed-by\x02\x0dwit-component\x070.216.0\x10wit-bindgen-rust\x060.31.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
