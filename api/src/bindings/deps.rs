// Generated by `wit-bindgen` 0.32.0. DO NOT EDIT!
// Options used:
//   * additional derives ["serde::Serialize", "serde::Deserialize", "Hash", "Clone", "PartialEq", "Eq"]
#[allow(dead_code)]
pub mod component {
    #[allow(dead_code)]
    pub mod face_detection_lib {
        #[allow(dead_code, clippy::all)]
        pub mod face_detection {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;

            use super::super::super::_rt;
            #[derive(Clone, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
            pub enum DetectionError {
                ModelError(_rt::String),
                ImageError(_rt::String),
                IoError(_rt::String),
                Unknown(_rt::String),
                Unclassified,
            }
            impl ::core::fmt::Debug for DetectionError {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self {
                        DetectionError::ModelError(e) => f
                            .debug_tuple("DetectionError::ModelError")
                            .field(e)
                            .finish(),
                        DetectionError::ImageError(e) => f
                            .debug_tuple("DetectionError::ImageError")
                            .field(e)
                            .finish(),
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
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    write!(f, "{:?}", self)
                }
            }

            impl std::error::Error for DetectionError {}
            pub type Image = _rt::Vec<u8>;
            #[repr(C)]
            #[derive(Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
            pub struct DetectionResult {
                pub x1: f32,
                pub y1: f32,
                pub x2: f32,
                pub y2: f32,
                pub confidence: f32,
            }
            impl ::core::fmt::Debug for DetectionResult {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("DetectionResult")
                        .field("x1", &self.x1)
                        .field("y1", &self.y1)
                        .field("x2", &self.x2)
                        .field("y2", &self.y2)
                        .field("confidence", &self.confidence)
                        .finish()
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn detect(
                img: &Image,
                confidence_treshold: f32,
            ) -> Result<_rt::Vec<DetectionResult>, DetectionError> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 16]);
                    let vec0 = img;
                    let ptr0 = vec0.as_ptr().cast::<u8>();
                    let len0 = vec0.len();
                    let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "component:face-detection-lib/face-detection")]
                    extern "C" {
                        #[link_name = "detect"]
                        fn wit_import(_: *mut u8, _: usize, _: f32, _: *mut u8);
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8, _: usize, _: f32, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import(
                        ptr0.cast_mut(),
                        len0,
                        _rt::as_f32(&confidence_treshold),
                        ptr1,
                    );
                    let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                    match l2 {
                        0 => {
                            let e = {
                                let l3 = *ptr1.add(4).cast::<*mut u8>();
                                let l4 = *ptr1.add(8).cast::<usize>();
                                let len5 = l4;

                                _rt::Vec::from_raw_parts(l3.cast(), len5, len5)
                            };
                            Ok(e)
                        }
                        1 => {
                            let e = {
                                let l6 = i32::from(*ptr1.add(4).cast::<u8>());
                                let v19 = match l6 {
                                    0 => {
                                        let e19 = {
                                            let l7 = *ptr1.add(8).cast::<*mut u8>();
                                            let l8 = *ptr1.add(12).cast::<usize>();
                                            let len9 = l8;
                                            let bytes9 =
                                                _rt::Vec::from_raw_parts(l7.cast(), len9, len9);

                                            _rt::string_lift(bytes9)
                                        };
                                        DetectionError::ModelError(e19)
                                    }
                                    1 => {
                                        let e19 = {
                                            let l10 = *ptr1.add(8).cast::<*mut u8>();
                                            let l11 = *ptr1.add(12).cast::<usize>();
                                            let len12 = l11;
                                            let bytes12 =
                                                _rt::Vec::from_raw_parts(l10.cast(), len12, len12);

                                            _rt::string_lift(bytes12)
                                        };
                                        DetectionError::ImageError(e19)
                                    }
                                    2 => {
                                        let e19 = {
                                            let l13 = *ptr1.add(8).cast::<*mut u8>();
                                            let l14 = *ptr1.add(12).cast::<usize>();
                                            let len15 = l14;
                                            let bytes15 =
                                                _rt::Vec::from_raw_parts(l13.cast(), len15, len15);

                                            _rt::string_lift(bytes15)
                                        };
                                        DetectionError::IoError(e19)
                                    }
                                    3 => {
                                        let e19 = {
                                            let l16 = *ptr1.add(8).cast::<*mut u8>();
                                            let l17 = *ptr1.add(12).cast::<usize>();
                                            let len18 = l17;
                                            let bytes18 =
                                                _rt::Vec::from_raw_parts(l16.cast(), len18, len18);

                                            _rt::string_lift(bytes18)
                                        };
                                        DetectionError::Unknown(e19)
                                    }
                                    n => {
                                        debug_assert_eq!(n, 4, "invalid enum discriminant");
                                        DetectionError::Unclassified
                                    }
                                };

                                v19
                            };
                            Err(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
        }
    }
}
mod _rt {
    pub use alloc_crate::string::String;
    pub use alloc_crate::vec::Vec;

    pub fn as_f32<T: AsF32>(t: T) -> f32 {
        t.as_f32()
    }

    pub trait AsF32 {
        fn as_f32(self) -> f32;
    }

    impl<'a, T: Copy + AsF32> AsF32 for &'a T {
        fn as_f32(self) -> f32 {
            (*self).as_f32()
        }
    }

    impl AsF32 for f32 {
        #[inline]
        fn as_f32(self) -> f32 {
            self as f32
        }
    }
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub unsafe fn invalid_enum_discriminant<T>() -> T {
        if cfg!(debug_assertions) {
            panic!("invalid enum discriminant")
        } else {
            core::hint::unreachable_unchecked()
        }
    }
    extern crate alloc as alloc_crate;
}

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.32.0:spin-deps:deps@0.1.0:deps:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 420] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xa9\x02\x01A\x02\x01\
A\x02\x01B\x0a\x01q\x05\x0bmodel-error\x01s\0\x0bimage-error\x01s\0\x08io-error\x01\
s\0\x07unknown\x01s\0\x0cunclassified\0\0\x04\0\x0fdetection-error\x03\0\0\x01p}\
\x04\0\x05image\x03\0\x02\x01r\x05\x02x1v\x02y1v\x02x2v\x02y2v\x0aconfidencev\x04\
\0\x10detection-result\x03\0\x04\x01p\x05\x01j\x01\x06\x01\x01\x01@\x02\x03img\x03\
\x13confidence-tresholdv\0\x07\x04\0\x06detect\x01\x08\x03\0+component:face-dete\
ction-lib/face-detection\x05\0\x04\0\x19spin-deps:deps/deps@0.1.0\x04\0\x0b\x0a\x01\
\0\x04deps\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x070\
.217.0\x10wit-bindgen-rust\x060.32.0";

#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen::rt::maybe_link_cabi_realloc();
}
