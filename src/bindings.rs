// Generated by `wit-bindgen` 0.25.0. DO NOT EDIT!
// Options used:
#[allow(dead_code)]
pub mod component {
    #[allow(dead_code)]
    pub mod pdbtbx_wit {
        #[allow(dead_code, clippy::all)]
        pub mod pdbtbx_api {
            #[used]
            #[doc(hidden)]
            #[cfg(target_arch = "wasm32")]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[derive(Clone)]
            pub struct Residueinfo {
                pub seqnumber: u64,
                pub insertioncode: _rt::String,
            }
            impl ::core::fmt::Debug for Residueinfo {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("Residueinfo")
                        .field("seqnumber", &self.seqnumber)
                        .field("insertioncode", &self.insertioncode)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct Residueinfos {
                pub chain: char,
                pub residues: _rt::Vec<Residueinfo>,
            }
            impl ::core::fmt::Debug for Residueinfos {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("Residueinfos")
                        .field("chain", &self.chain)
                        .field("residues", &self.residues)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct Pdbinfo {
                pub identifier: Option<_rt::String>,
                pub chains: _rt::Vec<char>,
                pub residuesequencenumbers: _rt::Vec<u64>,
                pub residuesperchain: _rt::Vec<Residueinfos>,
                pub warnings: _rt::Vec<_rt::String>,
            }
            impl ::core::fmt::Debug for Pdbinfo {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("Pdbinfo")
                        .field("identifier", &self.identifier)
                        .field("chains", &self.chains)
                        .field("residuesequencenumbers", &self.residuesequencenumbers)
                        .field("residuesperchain", &self.residuesperchain)
                        .field("warnings", &self.warnings)
                        .finish()
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn pdb2pdbinfo(content: &str) -> Result<Pdbinfo, _rt::Vec<_rt::String>> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 48]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 48]);
                    let vec0 = content;
                    let ptr0 = vec0.as_ptr().cast::<u8>();
                    let len0 = vec0.len();
                    let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "component:pdbtbx-wit/pdbtbx-api")]
                    extern "C" {
                        #[link_name = "pdb2pdbinfo"]
                        fn wit_import(_: *mut u8, _: usize, _: *mut u8);
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8, _: usize, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0.cast_mut(), len0, ptr1);
                    let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                    match l2 {
                        0 => {
                            let e = {
                                let l3 = i32::from(*ptr1.add(4).cast::<u8>());
                                let l7 = *ptr1.add(16).cast::<*mut u8>();
                                let l8 = *ptr1.add(20).cast::<usize>();
                                let base10 = l7;
                                let len10 = l8;
                                let mut result10 = _rt::Vec::with_capacity(len10);
                                for i in 0..len10 {
                                    let base = base10.add(i * 4);
                                    let e10 = {
                                        let l9 = *base.add(0).cast::<i32>();

                                        _rt::char_lift(l9 as u32)
                                    };
                                    result10.push(e10);
                                }
                                _rt::cabi_dealloc(base10, len10 * 4, 4);
                                let l11 = *ptr1.add(24).cast::<*mut u8>();
                                let l12 = *ptr1.add(28).cast::<usize>();
                                let len13 = l12;
                                let l14 = *ptr1.add(32).cast::<*mut u8>();
                                let l15 = *ptr1.add(36).cast::<usize>();
                                let base24 = l14;
                                let len24 = l15;
                                let mut result24 = _rt::Vec::with_capacity(len24);
                                for i in 0..len24 {
                                    let base = base24.add(i * 12);
                                    let e24 = {
                                        let l16 = *base.add(0).cast::<i32>();
                                        let l17 = *base.add(4).cast::<*mut u8>();
                                        let l18 = *base.add(8).cast::<usize>();
                                        let base23 = l17;
                                        let len23 = l18;
                                        let mut result23 = _rt::Vec::with_capacity(len23);
                                        for i in 0..len23 {
                                            let base = base23.add(i * 16);
                                            let e23 = {
                                                let l19 = *base.add(0).cast::<i64>();
                                                let l20 = *base.add(8).cast::<*mut u8>();
                                                let l21 = *base.add(12).cast::<usize>();
                                                let len22 = l21;
                                                let bytes22 = _rt::Vec::from_raw_parts(
                                                    l20.cast(),
                                                    len22,
                                                    len22,
                                                );

                                                Residueinfo {
                                                    seqnumber: l19 as u64,
                                                    insertioncode: _rt::string_lift(bytes22),
                                                }
                                            };
                                            result23.push(e23);
                                        }
                                        _rt::cabi_dealloc(base23, len23 * 16, 8);

                                        Residueinfos {
                                            chain: _rt::char_lift(l16 as u32),
                                            residues: result23,
                                        }
                                    };
                                    result24.push(e24);
                                }
                                _rt::cabi_dealloc(base24, len24 * 12, 4);
                                let l25 = *ptr1.add(40).cast::<*mut u8>();
                                let l26 = *ptr1.add(44).cast::<usize>();
                                let base30 = l25;
                                let len30 = l26;
                                let mut result30 = _rt::Vec::with_capacity(len30);
                                for i in 0..len30 {
                                    let base = base30.add(i * 8);
                                    let e30 = {
                                        let l27 = *base.add(0).cast::<*mut u8>();
                                        let l28 = *base.add(4).cast::<usize>();
                                        let len29 = l28;
                                        let bytes29 =
                                            _rt::Vec::from_raw_parts(l27.cast(), len29, len29);

                                        _rt::string_lift(bytes29)
                                    };
                                    result30.push(e30);
                                }
                                _rt::cabi_dealloc(base30, len30 * 8, 4);

                                Pdbinfo {
                                    identifier: match l3 {
                                        0 => None,
                                        1 => {
                                            let e = {
                                                let l4 = *ptr1.add(8).cast::<*mut u8>();
                                                let l5 = *ptr1.add(12).cast::<usize>();
                                                let len6 = l5;
                                                let bytes6 =
                                                    _rt::Vec::from_raw_parts(l4.cast(), len6, len6);

                                                _rt::string_lift(bytes6)
                                            };
                                            Some(e)
                                        }
                                        _ => _rt::invalid_enum_discriminant(),
                                    },
                                    chains: result10,
                                    residuesequencenumbers: _rt::Vec::from_raw_parts(
                                        l11.cast(),
                                        len13,
                                        len13,
                                    ),
                                    residuesperchain: result24,
                                    warnings: result30,
                                }
                            };
                            Ok(e)
                        }
                        1 => {
                            let e = {
                                let l31 = *ptr1.add(4).cast::<*mut u8>();
                                let l32 = *ptr1.add(8).cast::<usize>();
                                let base36 = l31;
                                let len36 = l32;
                                let mut result36 = _rt::Vec::with_capacity(len36);
                                for i in 0..len36 {
                                    let base = base36.add(i * 8);
                                    let e36 = {
                                        let l33 = *base.add(0).cast::<*mut u8>();
                                        let l34 = *base.add(4).cast::<usize>();
                                        let len35 = l34;
                                        let bytes35 =
                                            _rt::Vec::from_raw_parts(l33.cast(), len35, len35);

                                        _rt::string_lift(bytes35)
                                    };
                                    result36.push(e36);
                                }
                                _rt::cabi_dealloc(base36, len36 * 8, 4);

                                result36
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
#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod component {
        #[allow(dead_code)]
        pub mod pdbtbx_wit {
            #[allow(dead_code, clippy::all)]
            pub mod pdbtbx_api {
                #[used]
                #[doc(hidden)]
                #[cfg(target_arch = "wasm32")]
                static __FORCE_SECTION_REF: fn() =
                    super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[derive(Clone)]
                pub struct Residueinfo {
                    pub seqnumber: u64,
                    pub insertioncode: _rt::String,
                }
                impl ::core::fmt::Debug for Residueinfo {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        f.debug_struct("Residueinfo")
                            .field("seqnumber", &self.seqnumber)
                            .field("insertioncode", &self.insertioncode)
                            .finish()
                    }
                }
                #[derive(Clone)]
                pub struct Residueinfos {
                    pub chain: char,
                    pub residues: _rt::Vec<Residueinfo>,
                }
                impl ::core::fmt::Debug for Residueinfos {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        f.debug_struct("Residueinfos")
                            .field("chain", &self.chain)
                            .field("residues", &self.residues)
                            .finish()
                    }
                }
                #[derive(Clone)]
                pub struct Pdbinfo {
                    pub identifier: Option<_rt::String>,
                    pub chains: _rt::Vec<char>,
                    pub residuesequencenumbers: _rt::Vec<u64>,
                    pub residuesperchain: _rt::Vec<Residueinfos>,
                    pub warnings: _rt::Vec<_rt::String>,
                }
                impl ::core::fmt::Debug for Pdbinfo {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        f.debug_struct("Pdbinfo")
                            .field("identifier", &self.identifier)
                            .field("chains", &self.chains)
                            .field("residuesequencenumbers", &self.residuesequencenumbers)
                            .field("residuesperchain", &self.residuesperchain)
                            .field("warnings", &self.warnings)
                            .finish()
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_pdb2pdbinfo_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let result1 = T::pdb2pdbinfo(_rt::string_lift(bytes0));
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result1 {
                        Ok(e) => {
                            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                            let Pdbinfo {
                                identifier: identifier3,
                                chains: chains3,
                                residuesequencenumbers: residuesequencenumbers3,
                                residuesperchain: residuesperchain3,
                                warnings: warnings3,
                            } = e;
                            match identifier3 {
                                Some(e) => {
                                    *ptr2.add(4).cast::<u8>() = (1i32) as u8;
                                    let vec4 = (e.into_bytes()).into_boxed_slice();
                                    let ptr4 = vec4.as_ptr().cast::<u8>();
                                    let len4 = vec4.len();
                                    ::core::mem::forget(vec4);
                                    *ptr2.add(12).cast::<usize>() = len4;
                                    *ptr2.add(8).cast::<*mut u8>() = ptr4.cast_mut();
                                }
                                None => {
                                    *ptr2.add(4).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec5 = chains3;
                            let len5 = vec5.len();
                            let layout5 =
                                _rt::alloc::Layout::from_size_align_unchecked(vec5.len() * 4, 4);
                            let result5 = if layout5.size() != 0 {
                                let ptr = _rt::alloc::alloc(layout5).cast::<u8>();
                                if ptr.is_null() {
                                    _rt::alloc::handle_alloc_error(layout5);
                                }
                                ptr
                            } else {
                                {
                                    ::core::ptr::null_mut()
                                }
                            };
                            for (i, e) in vec5.into_iter().enumerate() {
                                let base = result5.add(i * 4);
                                {
                                    *base.add(0).cast::<i32>() = _rt::as_i32(e);
                                }
                            }
                            *ptr2.add(20).cast::<usize>() = len5;
                            *ptr2.add(16).cast::<*mut u8>() = result5;
                            let vec6 = (residuesequencenumbers3).into_boxed_slice();
                            let ptr6 = vec6.as_ptr().cast::<u8>();
                            let len6 = vec6.len();
                            ::core::mem::forget(vec6);
                            *ptr2.add(28).cast::<usize>() = len6;
                            *ptr2.add(24).cast::<*mut u8>() = ptr6.cast_mut();
                            let vec11 = residuesperchain3;
                            let len11 = vec11.len();
                            let layout11 =
                                _rt::alloc::Layout::from_size_align_unchecked(vec11.len() * 12, 4);
                            let result11 = if layout11.size() != 0 {
                                let ptr = _rt::alloc::alloc(layout11).cast::<u8>();
                                if ptr.is_null() {
                                    _rt::alloc::handle_alloc_error(layout11);
                                }
                                ptr
                            } else {
                                {
                                    ::core::ptr::null_mut()
                                }
                            };
                            for (i, e) in vec11.into_iter().enumerate() {
                                let base = result11.add(i * 12);
                                {
                                    let Residueinfos {
                                        chain: chain7,
                                        residues: residues7,
                                    } = e;
                                    *base.add(0).cast::<i32>() = _rt::as_i32(chain7);
                                    let vec10 = residues7;
                                    let len10 = vec10.len();
                                    let layout10 = _rt::alloc::Layout::from_size_align_unchecked(
                                        vec10.len() * 16,
                                        8,
                                    );
                                    let result10 = if layout10.size() != 0 {
                                        let ptr = _rt::alloc::alloc(layout10).cast::<u8>();
                                        if ptr.is_null() {
                                            _rt::alloc::handle_alloc_error(layout10);
                                        }
                                        ptr
                                    } else {
                                        {
                                            ::core::ptr::null_mut()
                                        }
                                    };
                                    for (i, e) in vec10.into_iter().enumerate() {
                                        let base = result10.add(i * 16);
                                        {
                                            let Residueinfo {
                                                seqnumber: seqnumber8,
                                                insertioncode: insertioncode8,
                                            } = e;
                                            *base.add(0).cast::<i64>() = _rt::as_i64(seqnumber8);
                                            let vec9 =
                                                (insertioncode8.into_bytes()).into_boxed_slice();
                                            let ptr9 = vec9.as_ptr().cast::<u8>();
                                            let len9 = vec9.len();
                                            ::core::mem::forget(vec9);
                                            *base.add(12).cast::<usize>() = len9;
                                            *base.add(8).cast::<*mut u8>() = ptr9.cast_mut();
                                        }
                                    }
                                    *base.add(8).cast::<usize>() = len10;
                                    *base.add(4).cast::<*mut u8>() = result10;
                                }
                            }
                            *ptr2.add(36).cast::<usize>() = len11;
                            *ptr2.add(32).cast::<*mut u8>() = result11;
                            let vec13 = warnings3;
                            let len13 = vec13.len();
                            let layout13 =
                                _rt::alloc::Layout::from_size_align_unchecked(vec13.len() * 8, 4);
                            let result13 = if layout13.size() != 0 {
                                let ptr = _rt::alloc::alloc(layout13).cast::<u8>();
                                if ptr.is_null() {
                                    _rt::alloc::handle_alloc_error(layout13);
                                }
                                ptr
                            } else {
                                {
                                    ::core::ptr::null_mut()
                                }
                            };
                            for (i, e) in vec13.into_iter().enumerate() {
                                let base = result13.add(i * 8);
                                {
                                    let vec12 = (e.into_bytes()).into_boxed_slice();
                                    let ptr12 = vec12.as_ptr().cast::<u8>();
                                    let len12 = vec12.len();
                                    ::core::mem::forget(vec12);
                                    *base.add(4).cast::<usize>() = len12;
                                    *base.add(0).cast::<*mut u8>() = ptr12.cast_mut();
                                }
                            }
                            *ptr2.add(44).cast::<usize>() = len13;
                            *ptr2.add(40).cast::<*mut u8>() = result13;
                        }
                        Err(e) => {
                            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                            let vec15 = e;
                            let len15 = vec15.len();
                            let layout15 =
                                _rt::alloc::Layout::from_size_align_unchecked(vec15.len() * 8, 4);
                            let result15 = if layout15.size() != 0 {
                                let ptr = _rt::alloc::alloc(layout15).cast::<u8>();
                                if ptr.is_null() {
                                    _rt::alloc::handle_alloc_error(layout15);
                                }
                                ptr
                            } else {
                                {
                                    ::core::ptr::null_mut()
                                }
                            };
                            for (i, e) in vec15.into_iter().enumerate() {
                                let base = result15.add(i * 8);
                                {
                                    let vec14 = (e.into_bytes()).into_boxed_slice();
                                    let ptr14 = vec14.as_ptr().cast::<u8>();
                                    let len14 = vec14.len();
                                    ::core::mem::forget(vec14);
                                    *base.add(4).cast::<usize>() = len14;
                                    *base.add(0).cast::<*mut u8>() = ptr14.cast_mut();
                                }
                            }
                            *ptr2.add(8).cast::<usize>() = len15;
                            *ptr2.add(4).cast::<*mut u8>() = result15;
                        }
                    };
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_pdb2pdbinfo<T: Guest>(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = i32::from(*arg0.add(4).cast::<u8>());
                            match l1 {
                                0 => (),
                                _ => {
                                    let l2 = *arg0.add(8).cast::<*mut u8>();
                                    let l3 = *arg0.add(12).cast::<usize>();
                                    _rt::cabi_dealloc(l2, l3, 1);
                                }
                            }
                            let l4 = *arg0.add(16).cast::<*mut u8>();
                            let l5 = *arg0.add(20).cast::<usize>();
                            let base6 = l4;
                            let len6 = l5;
                            _rt::cabi_dealloc(base6, len6 * 4, 4);
                            let l7 = *arg0.add(24).cast::<*mut u8>();
                            let l8 = *arg0.add(28).cast::<usize>();
                            let base9 = l7;
                            let len9 = l8;
                            _rt::cabi_dealloc(base9, len9 * 8, 8);
                            let l15 = *arg0.add(32).cast::<*mut u8>();
                            let l16 = *arg0.add(36).cast::<usize>();
                            let base17 = l15;
                            let len17 = l16;
                            for i in 0..len17 {
                                let base = base17.add(i * 12);
                                {
                                    let l12 = *base.add(4).cast::<*mut u8>();
                                    let l13 = *base.add(8).cast::<usize>();
                                    let base14 = l12;
                                    let len14 = l13;
                                    for i in 0..len14 {
                                        let base = base14.add(i * 16);
                                        {
                                            let l10 = *base.add(8).cast::<*mut u8>();
                                            let l11 = *base.add(12).cast::<usize>();
                                            _rt::cabi_dealloc(l10, l11, 1);
                                        }
                                    }
                                    _rt::cabi_dealloc(base14, len14 * 16, 8);
                                }
                            }
                            _rt::cabi_dealloc(base17, len17 * 12, 4);
                            let l20 = *arg0.add(40).cast::<*mut u8>();
                            let l21 = *arg0.add(44).cast::<usize>();
                            let base22 = l20;
                            let len22 = l21;
                            for i in 0..len22 {
                                let base = base22.add(i * 8);
                                {
                                    let l18 = *base.add(0).cast::<*mut u8>();
                                    let l19 = *base.add(4).cast::<usize>();
                                    _rt::cabi_dealloc(l18, l19, 1);
                                }
                            }
                            _rt::cabi_dealloc(base22, len22 * 8, 4);
                        }
                        _ => {
                            let l25 = *arg0.add(4).cast::<*mut u8>();
                            let l26 = *arg0.add(8).cast::<usize>();
                            let base27 = l25;
                            let len27 = l26;
                            for i in 0..len27 {
                                let base = base27.add(i * 8);
                                {
                                    let l23 = *base.add(0).cast::<*mut u8>();
                                    let l24 = *base.add(4).cast::<usize>();
                                    _rt::cabi_dealloc(l23, l24, 1);
                                }
                            }
                            _rt::cabi_dealloc(base27, len27 * 8, 4);
                        }
                    }
                }
                pub trait Guest {
                    fn pdb2pdbinfo(content: _rt::String) -> Result<Pdbinfo, _rt::Vec<_rt::String>>;
                }
                #[doc(hidden)]

                macro_rules! __export_component_pdbtbx_wit_pdbtbx_api_cabi{
        ($ty:ident with_types_in $($path_to_types:tt)*) => (const _: () = {

          #[export_name = "component:pdbtbx-wit/pdbtbx-api#pdb2pdbinfo"]
          unsafe extern "C" fn export_pdb2pdbinfo(arg0: *mut u8,arg1: usize,) -> *mut u8 {
            $($path_to_types)*::_export_pdb2pdbinfo_cabi::<$ty>(arg0, arg1)
          }
          #[export_name = "cabi_post_component:pdbtbx-wit/pdbtbx-api#pdb2pdbinfo"]
          unsafe extern "C" fn _post_return_pdb2pdbinfo(arg0: *mut u8,) {
            $($path_to_types)*::__post_return_pdb2pdbinfo::<$ty>(arg0)
          }
        };);
      }
                #[doc(hidden)]
                pub(crate) use __export_component_pdbtbx_wit_pdbtbx_api_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 48]);
                static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 48]);
            }
        }
    }
}
mod _rt {
    pub use alloc_crate::string::String;
    pub use alloc_crate::vec::Vec;
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
    pub unsafe fn char_lift(val: u32) -> char {
        if cfg!(debug_assertions) {
            core::char::from_u32(val).unwrap()
        } else {
            core::char::from_u32_unchecked(val)
        }
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr as *mut u8, layout);
    }

    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }

    pub fn as_i32<T: AsI32>(t: T) -> i32 {
        t.as_i32()
    }

    pub trait AsI32 {
        fn as_i32(self) -> i32;
    }

    impl<'a, T: Copy + AsI32> AsI32 for &'a T {
        fn as_i32(self) -> i32 {
            (*self).as_i32()
        }
    }

    impl AsI32 for i32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }

    impl AsI32 for u32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }

    impl AsI32 for i16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }

    impl AsI32 for u16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }

    impl AsI32 for i8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }

    impl AsI32 for u8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }

    impl AsI32 for char {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }

    impl AsI32 for usize {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    pub use alloc_crate::alloc;

    pub fn as_i64<T: AsI64>(t: T) -> i64 {
        t.as_i64()
    }

    pub trait AsI64 {
        fn as_i64(self) -> i64;
    }

    impl<'a, T: Copy + AsI64> AsI64 for &'a T {
        fn as_i64(self) -> i64 {
            (*self).as_i64()
        }
    }

    impl AsI64 for i64 {
        #[inline]
        fn as_i64(self) -> i64 {
            self as i64
        }
    }

    impl AsI64 for u64 {
        #[inline]
        fn as_i64(self) -> i64 {
            self as i64
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

macro_rules! __export_pdbtbx_wit_impl {
  ($ty:ident) => (self::export!($ty with_types_in self););
  ($ty:ident with_types_in $($path_to_types_root:tt)*) => (
  $($path_to_types_root)*::exports::component::pdbtbx_wit::pdbtbx_api::__export_component_pdbtbx_wit_pdbtbx_api_cabi!($ty with_types_in $($path_to_types_root)*::exports::component::pdbtbx_wit::pdbtbx_api);
  )
}
#[doc(inline)]
pub(crate) use __export_pdbtbx_wit_impl as export;

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.25.0:pdbtbx-wit:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 701] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xbc\x04\x01A\x02\x01\
A\x04\x01B\x0f\x01r\x02\x09seqnumberw\x0dinsertioncodes\x04\0\x0bresidueinfo\x03\
\0\0\x01p\x01\x01r\x02\x05chaint\x08residues\x02\x04\0\x0cresidueinfos\x03\0\x03\
\x01ks\x01pt\x01pw\x01p\x04\x01ps\x01r\x05\x0aidentifier\x05\x06chains\x06\x16re\
siduesequencenumbers\x07\x10residuesperchain\x08\x08warnings\x09\x04\0\x07pdbinf\
o\x03\0\x0a\x01j\x01\x0b\x01\x09\x01@\x01\x07contents\0\x0c\x04\0\x0bpdb2pdbinfo\
\x01\x0d\x03\x01\x1fcomponent:pdbtbx-wit/pdbtbx-api\x05\0\x01B\x0f\x01r\x02\x09s\
eqnumberw\x0dinsertioncodes\x04\0\x0bresidueinfo\x03\0\0\x01p\x01\x01r\x02\x05ch\
aint\x08residues\x02\x04\0\x0cresidueinfos\x03\0\x03\x01ks\x01pt\x01pw\x01p\x04\x01\
ps\x01r\x05\x0aidentifier\x05\x06chains\x06\x16residuesequencenumbers\x07\x10res\
iduesperchain\x08\x08warnings\x09\x04\0\x07pdbinfo\x03\0\x0a\x01j\x01\x0b\x01\x09\
\x01@\x01\x07contents\0\x0c\x04\0\x0bpdb2pdbinfo\x01\x0d\x04\x01\x1fcomponent:pd\
btbx-wit/pdbtbx-api\x05\x01\x04\x01\x1fcomponent:pdbtbx-wit/pdbtbx-wit\x04\0\x0b\
\x10\x01\0\x0apdbtbx-wit\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-\
component\x070.208.1\x10wit-bindgen-rust\x060.25.0";

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
