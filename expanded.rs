#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
pub mod error {
    use std::error;
    use std::fmt::Display;
    use std::io;
    pub enum Error {
        IO(io::Error),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Error {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&Error::IO(ref __self_0),) => {
                    let mut debug_trait_builder = f.debug_tuple("IO");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl From<io::Error> for Error {
        fn from(err: io::Error) -> Self {
            Error::IO(err)
        }
    }
    impl Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
            match self {
                Error::IO(e) => f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["IO error: "],
                    &match (&e,) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)],
                    },
                )),
            }
        }
    }
    impl error::Error for Error {
        fn cause(&self) -> Option<&dyn error::Error> {
            match self {
                Error::IO(e) => Some(e),
            }
        }
    }
}
pub mod midi {
    mod file {
        use super::{Format, Timing};
        use crate::midi::Track;
        use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
        use serde::{Deserialize, Serialize};
        use std::io::{Error, Read, Seek, Write};
        pub struct File {
            pub format: Format,
            timing: Timing,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for File {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    File {
                        format: ref __self_0_0,
                        timing: ref __self_0_1,
                    } => {
                        let mut debug_trait_builder = f.debug_struct("File");
                        let _ = debug_trait_builder.field("format", &&(*__self_0_0));
                        let _ = debug_trait_builder.field("timing", &&(*__self_0_1));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _IMPL_DESERIALIZE_FOR_File: () = {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for File {
                fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::export::Formatter,
                        ) -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::export::Ok(__Field::__field0),
                                1u64 => _serde::export::Ok(__Field::__field1),
                                _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"field index 0 <= i < 2",
                                )),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "format" => _serde::export::Ok(__Field::__field0),
                                "timing" => _serde::export::Ok(__Field::__field1),
                                _ => _serde::export::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"format" => _serde::export::Ok(__Field::__field0),
                                b"timing" => _serde::export::Ok(__Field::__field1),
                                _ => _serde::export::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::export::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::export::PhantomData<File>,
                        lifetime: _serde::export::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = File;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::export::Formatter,
                        ) -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter, "struct File")
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::export::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<Format>(
                                &mut __seq,
                            ) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct File with 2 elements",
                                    ));
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<Timing>(
                                &mut __seq,
                            ) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct File with 2 elements",
                                    ));
                                }
                            };
                            _serde::export::Ok(File {
                                format: __field0,
                                timing: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::export::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::export::Option<Format> = _serde::export::None;
                            let mut __field1: _serde::export::Option<Timing> = _serde::export::None;
                            while let _serde::export::Some(__key) =
                                match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::export::Option::is_some(&__field0) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "format",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<Format>(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::export::Option::is_some(&__field1) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "timing",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<Timing>(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(
                                            &mut __map
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::export::Some(__field0) => __field0,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("format") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::export::Some(__field1) => __field1,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("timing") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::export::Ok(File {
                                format: __field0,
                                timing: __field1,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["format", "timing"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "File",
                        FIELDS,
                        __Visitor {
                            marker: _serde::export::PhantomData::<File>,
                            lifetime: _serde::export::PhantomData,
                        },
                    )
                }
            }
        };
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _IMPL_SERIALIZE_FOR_File: () = {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for File {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::export::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "File",
                        false as usize + 1 + 1,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "format",
                        &self.format,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "timing",
                        &self.timing,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        impl File {
            pub fn get_timing(&self) -> Timing {
                self.timing
            }
        }
    }
    mod format {
        use super::Track;
        use serde::{Deserialize, Serialize};
        pub enum Format {
            SingleTrack(Track),
            MultipleTrack(Vec<Track>),
            ParallelTrack(Vec<Track>),
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for Format {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match (&*self,) {
                    (&Format::SingleTrack(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("SingleTrack");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&Format::MultipleTrack(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("MultipleTrack");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&Format::ParallelTrack(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("ParallelTrack");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _IMPL_DESERIALIZE_FOR_Format: () = {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Format {
                fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::export::Formatter,
                        ) -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter, "variant identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::export::Ok(__Field::__field0),
                                1u64 => _serde::export::Ok(__Field::__field1),
                                2u64 => _serde::export::Ok(__Field::__field2),
                                _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"variant index 0 <= i < 3",
                                )),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "SingleTrack" => _serde::export::Ok(__Field::__field0),
                                "MultipleTrack" => _serde::export::Ok(__Field::__field1),
                                "ParallelTrack" => _serde::export::Ok(__Field::__field2),
                                _ => _serde::export::Err(_serde::de::Error::unknown_variant(
                                    __value, VARIANTS,
                                )),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"SingleTrack" => _serde::export::Ok(__Field::__field0),
                                b"MultipleTrack" => _serde::export::Ok(__Field::__field1),
                                b"ParallelTrack" => _serde::export::Ok(__Field::__field2),
                                _ => {
                                    let __value = &_serde::export::from_utf8_lossy(__value);
                                    _serde::export::Err(_serde::de::Error::unknown_variant(
                                        __value, VARIANTS,
                                    ))
                                }
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::export::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::export::PhantomData<Format>,
                        lifetime: _serde::export::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Format;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::export::Formatter,
                        ) -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter, "enum Format")
                        }
                        fn visit_enum<__A>(
                            self,
                            __data: __A,
                        ) -> _serde::export::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::EnumAccess<'de>,
                        {
                            match match _serde::de::EnumAccess::variant(__data) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                (__Field::__field0, __variant) => _serde::export::Result::map(
                                    _serde::de::VariantAccess::newtype_variant::<Track>(__variant),
                                    Format::SingleTrack,
                                ),
                                (__Field::__field1, __variant) => _serde::export::Result::map(
                                    _serde::de::VariantAccess::newtype_variant::<Vec<Track>>(
                                        __variant,
                                    ),
                                    Format::MultipleTrack,
                                ),
                                (__Field::__field2, __variant) => _serde::export::Result::map(
                                    _serde::de::VariantAccess::newtype_variant::<Vec<Track>>(
                                        __variant,
                                    ),
                                    Format::ParallelTrack,
                                ),
                            }
                        }
                    }
                    const VARIANTS: &'static [&'static str] =
                        &["SingleTrack", "MultipleTrack", "ParallelTrack"];
                    _serde::Deserializer::deserialize_enum(
                        __deserializer,
                        "Format",
                        VARIANTS,
                        __Visitor {
                            marker: _serde::export::PhantomData::<Format>,
                            lifetime: _serde::export::PhantomData,
                        },
                    )
                }
            }
        };
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _IMPL_SERIALIZE_FOR_Format: () = {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Format {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::export::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    match *self {
                        Format::SingleTrack(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "Format",
                                0u32,
                                "SingleTrack",
                                __field0,
                            )
                        }
                        Format::MultipleTrack(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "Format",
                                1u32,
                                "MultipleTrack",
                                __field0,
                            )
                        }
                        Format::ParallelTrack(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "Format",
                                2u32,
                                "ParallelTrack",
                                __field0,
                            )
                        }
                    }
                }
            }
        };
    }
    mod smpte_timecode {
        use serde::{Deserialize, Serialize};
        pub enum SMPTETimecode {
            FPS24 = 24,
            FPS25 = 25,
            FPS29 = 29,
            FPS30 = 30,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for SMPTETimecode {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match (&*self,) {
                    (&SMPTETimecode::FPS24,) => {
                        let mut debug_trait_builder = f.debug_tuple("FPS24");
                        debug_trait_builder.finish()
                    }
                    (&SMPTETimecode::FPS25,) => {
                        let mut debug_trait_builder = f.debug_tuple("FPS25");
                        debug_trait_builder.finish()
                    }
                    (&SMPTETimecode::FPS29,) => {
                        let mut debug_trait_builder = f.debug_tuple("FPS29");
                        debug_trait_builder.finish()
                    }
                    (&SMPTETimecode::FPS30,) => {
                        let mut debug_trait_builder = f.debug_tuple("FPS30");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for SMPTETimecode {
            #[inline]
            fn clone(&self) -> SMPTETimecode {
                {
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::marker::Copy for SMPTETimecode {}
        impl ::core::marker::StructuralPartialEq for SMPTETimecode {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for SMPTETimecode {
            #[inline]
            fn eq(&self, other: &SMPTETimecode) -> bool {
                {
                    let __self_vi =
                        unsafe { ::core::intrinsics::discriminant_value(&*self) } as isize;
                    let __arg_1_vi =
                        unsafe { ::core::intrinsics::discriminant_value(&*other) } as isize;
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*other) {
                            _ => true,
                        }
                    } else {
                        false
                    }
                }
            }
        }
        impl ::core::marker::StructuralEq for SMPTETimecode {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for SMPTETimecode {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                {}
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialOrd for SMPTETimecode {
            #[inline]
            fn partial_cmp(
                &self,
                other: &SMPTETimecode,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                {
                    let __self_vi =
                        unsafe { ::core::intrinsics::discriminant_value(&*self) } as isize;
                    let __arg_1_vi =
                        unsafe { ::core::intrinsics::discriminant_value(&*other) } as isize;
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*other) {
                            _ => ::core::option::Option::Some(::core::cmp::Ordering::Equal),
                        }
                    } else {
                        __self_vi.partial_cmp(&__arg_1_vi)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Ord for SMPTETimecode {
            #[inline]
            fn cmp(&self, other: &SMPTETimecode) -> ::core::cmp::Ordering {
                {
                    let __self_vi =
                        unsafe { ::core::intrinsics::discriminant_value(&*self) } as isize;
                    let __arg_1_vi =
                        unsafe { ::core::intrinsics::discriminant_value(&*other) } as isize;
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*other) {
                            _ => ::core::cmp::Ordering::Equal,
                        }
                    } else {
                        __self_vi.cmp(&__arg_1_vi)
                    }
                }
            }
        }
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _IMPL_DESERIALIZE_FOR_SMPTETimecode: () = {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for SMPTETimecode {
                fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::export::Formatter,
                        ) -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter, "variant identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::export::Ok(__Field::__field0),
                                1u64 => _serde::export::Ok(__Field::__field1),
                                2u64 => _serde::export::Ok(__Field::__field2),
                                3u64 => _serde::export::Ok(__Field::__field3),
                                _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"variant index 0 <= i < 4",
                                )),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "FPS24" => _serde::export::Ok(__Field::__field0),
                                "FPS25" => _serde::export::Ok(__Field::__field1),
                                "FPS29" => _serde::export::Ok(__Field::__field2),
                                "FPS30" => _serde::export::Ok(__Field::__field3),
                                _ => _serde::export::Err(_serde::de::Error::unknown_variant(
                                    __value, VARIANTS,
                                )),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"FPS24" => _serde::export::Ok(__Field::__field0),
                                b"FPS25" => _serde::export::Ok(__Field::__field1),
                                b"FPS29" => _serde::export::Ok(__Field::__field2),
                                b"FPS30" => _serde::export::Ok(__Field::__field3),
                                _ => {
                                    let __value = &_serde::export::from_utf8_lossy(__value);
                                    _serde::export::Err(_serde::de::Error::unknown_variant(
                                        __value, VARIANTS,
                                    ))
                                }
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::export::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::export::PhantomData<SMPTETimecode>,
                        lifetime: _serde::export::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = SMPTETimecode;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::export::Formatter,
                        ) -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter, "enum SMPTETimecode")
                        }
                        fn visit_enum<__A>(
                            self,
                            __data: __A,
                        ) -> _serde::export::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::EnumAccess<'de>,
                        {
                            match match _serde::de::EnumAccess::variant(__data) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                (__Field::__field0, __variant) => {
                                    match _serde::de::VariantAccess::unit_variant(__variant) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                    _serde::export::Ok(SMPTETimecode::FPS24)
                                }
                                (__Field::__field1, __variant) => {
                                    match _serde::de::VariantAccess::unit_variant(__variant) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                    _serde::export::Ok(SMPTETimecode::FPS25)
                                }
                                (__Field::__field2, __variant) => {
                                    match _serde::de::VariantAccess::unit_variant(__variant) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                    _serde::export::Ok(SMPTETimecode::FPS29)
                                }
                                (__Field::__field3, __variant) => {
                                    match _serde::de::VariantAccess::unit_variant(__variant) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                    _serde::export::Ok(SMPTETimecode::FPS30)
                                }
                            }
                        }
                    }
                    const VARIANTS: &'static [&'static str] = &["FPS24", "FPS25", "FPS29", "FPS30"];
                    _serde::Deserializer::deserialize_enum(
                        __deserializer,
                        "SMPTETimecode",
                        VARIANTS,
                        __Visitor {
                            marker: _serde::export::PhantomData::<SMPTETimecode>,
                            lifetime: _serde::export::PhantomData,
                        },
                    )
                }
            }
        };
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _IMPL_SERIALIZE_FOR_SMPTETimecode: () = {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for SMPTETimecode {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::export::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    match *self {
                        SMPTETimecode::FPS24 => _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "SMPTETimecode",
                            0u32,
                            "FPS24",
                        ),
                        SMPTETimecode::FPS25 => _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "SMPTETimecode",
                            1u32,
                            "FPS25",
                        ),
                        SMPTETimecode::FPS29 => _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "SMPTETimecode",
                            2u32,
                            "FPS29",
                        ),
                        SMPTETimecode::FPS30 => _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "SMPTETimecode",
                            3u32,
                            "FPS30",
                        ),
                    }
                }
            }
        };
        impl SMPTETimecode {
            pub fn from(value: u32) -> Result<Self, ()> {
                match value {
                    24 => Ok(SMPTETimecode::FPS24),
                    25 => Ok(SMPTETimecode::FPS25),
                    29 => Ok(SMPTETimecode::FPS29),
                    30 => Ok(SMPTETimecode::FPS30),
                    _ => Err(()),
                }
            }
        }
    }
    mod timing {
        use crate::midi::SMPTETimecode;
        use binread::{BinRead, BinReaderExt, BinResult, ReadOptions};
        use binread::io::{Read, Seek, Error, ErrorKind};
        use serde::{Deserialize, Serialize};
        pub enum Timing {
            # [ br ( assert ( self_0 > 0 ) ) ]
            Metrical(u16),
            Real(u8, u8),
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for Timing {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match (&*self,) {
                    (&Timing::Metrical(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("Metrical");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&Timing::Real(ref __self_0, ref __self_1),) => {
                        let mut debug_trait_builder = f.debug_tuple("Real");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        let _ = debug_trait_builder.field(&&(*__self_1));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for Timing {
            #[inline]
            fn clone(&self) -> Timing {
                {
                    let _: ::core::clone::AssertParamIsClone<u16>;
                    let _: ::core::clone::AssertParamIsClone<u8>;
                    let _: ::core::clone::AssertParamIsClone<u8>;
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::marker::Copy for Timing {}
        impl ::core::marker::StructuralPartialEq for Timing {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for Timing {
            #[inline]
            fn eq(&self, other: &Timing) -> bool {
                {
                    let __self_vi =
                        unsafe { ::core::intrinsics::discriminant_value(&*self) } as isize;
                    let __arg_1_vi =
                        unsafe { ::core::intrinsics::discriminant_value(&*other) } as isize;
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*other) {
                            (&Timing::Metrical(ref __self_0), &Timing::Metrical(ref __arg_1_0)) => {
                                (*__self_0) == (*__arg_1_0)
                            }
                            (
                                &Timing::Real(ref __self_0, ref __self_1),
                                &Timing::Real(ref __arg_1_0, ref __arg_1_1),
                            ) => (*__self_0) == (*__arg_1_0) && (*__self_1) == (*__arg_1_1),
                            _ => unsafe { ::core::intrinsics::unreachable() },
                        }
                    } else {
                        false
                    }
                }
            }
            #[inline]
            fn ne(&self, other: &Timing) -> bool {
                {
                    let __self_vi =
                        unsafe { ::core::intrinsics::discriminant_value(&*self) } as isize;
                    let __arg_1_vi =
                        unsafe { ::core::intrinsics::discriminant_value(&*other) } as isize;
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*other) {
                            (&Timing::Metrical(ref __self_0), &Timing::Metrical(ref __arg_1_0)) => {
                                (*__self_0) != (*__arg_1_0)
                            }
                            (
                                &Timing::Real(ref __self_0, ref __self_1),
                                &Timing::Real(ref __arg_1_0, ref __arg_1_1),
                            ) => (*__self_0) != (*__arg_1_0) || (*__self_1) != (*__arg_1_1),
                            _ => unsafe { ::core::intrinsics::unreachable() },
                        }
                    } else {
                        true
                    }
                }
            }
        }
        impl ::core::marker::StructuralEq for Timing {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for Timing {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::core::cmp::AssertParamIsEq<u16>;
                    let _: ::core::cmp::AssertParamIsEq<u8>;
                    let _: ::core::cmp::AssertParamIsEq<u8>;
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialOrd for Timing {
            #[inline]
            fn partial_cmp(&self, other: &Timing) -> ::core::option::Option<::core::cmp::Ordering> {
                {
                    let __self_vi =
                        unsafe { ::core::intrinsics::discriminant_value(&*self) } as isize;
                    let __arg_1_vi =
                        unsafe { ::core::intrinsics::discriminant_value(&*other) } as isize;
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*other) {
                            (&Timing::Metrical(ref __self_0), &Timing::Metrical(ref __arg_1_0)) => {
                                match ::core::cmp::PartialOrd::partial_cmp(
                                    &(*__self_0),
                                    &(*__arg_1_0),
                                ) {
                                    ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                        ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                                    }
                                    cmp => cmp,
                                }
                            }
                            (
                                &Timing::Real(ref __self_0, ref __self_1),
                                &Timing::Real(ref __arg_1_0, ref __arg_1_1),
                            ) => match ::core::cmp::PartialOrd::partial_cmp(
                                &(*__self_0),
                                &(*__arg_1_0),
                            ) {
                                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                    match ::core::cmp::PartialOrd::partial_cmp(
                                        &(*__self_1),
                                        &(*__arg_1_1),
                                    ) {
                                        ::core::option::Option::Some(
                                            ::core::cmp::Ordering::Equal,
                                        ) => ::core::option::Option::Some(
                                            ::core::cmp::Ordering::Equal,
                                        ),
                                        cmp => cmp,
                                    }
                                }
                                cmp => cmp,
                            },
                            _ => unsafe { ::core::intrinsics::unreachable() },
                        }
                    } else {
                        __self_vi.partial_cmp(&__arg_1_vi)
                    }
                }
            }
            #[inline]
            fn lt(&self, other: &Timing) -> bool {
                {
                    let __self_vi =
                        unsafe { ::core::intrinsics::discriminant_value(&*self) } as isize;
                    let __arg_1_vi =
                        unsafe { ::core::intrinsics::discriminant_value(&*other) } as isize;
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*other) {
                            (&Timing::Metrical(ref __self_0), &Timing::Metrical(ref __arg_1_0)) => {
                                ::core::option::Option::unwrap_or(
                                    ::core::cmp::PartialOrd::partial_cmp(
                                        &(*__self_0),
                                        &(*__arg_1_0),
                                    ),
                                    ::core::cmp::Ordering::Greater,
                                ) == ::core::cmp::Ordering::Less
                            }
                            (
                                &Timing::Real(ref __self_0, ref __self_1),
                                &Timing::Real(ref __arg_1_0, ref __arg_1_1),
                            ) => {
                                ::core::cmp::Ordering::then_with(
                                    ::core::option::Option::unwrap_or(
                                        ::core::cmp::PartialOrd::partial_cmp(
                                            &(*__self_0),
                                            &(*__arg_1_0),
                                        ),
                                        ::core::cmp::Ordering::Equal,
                                    ),
                                    || {
                                        ::core::option::Option::unwrap_or(
                                            ::core::cmp::PartialOrd::partial_cmp(
                                                &(*__self_1),
                                                &(*__arg_1_1),
                                            ),
                                            ::core::cmp::Ordering::Greater,
                                        )
                                    },
                                ) == ::core::cmp::Ordering::Less
                            }
                            _ => unsafe { ::core::intrinsics::unreachable() },
                        }
                    } else {
                        __self_vi.lt(&__arg_1_vi)
                    }
                }
            }
            #[inline]
            fn le(&self, other: &Timing) -> bool {
                {
                    let __self_vi =
                        unsafe { ::core::intrinsics::discriminant_value(&*self) } as isize;
                    let __arg_1_vi =
                        unsafe { ::core::intrinsics::discriminant_value(&*other) } as isize;
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*other) {
                            (&Timing::Metrical(ref __self_0), &Timing::Metrical(ref __arg_1_0)) => {
                                ::core::option::Option::unwrap_or(
                                    ::core::cmp::PartialOrd::partial_cmp(
                                        &(*__self_0),
                                        &(*__arg_1_0),
                                    ),
                                    ::core::cmp::Ordering::Greater,
                                ) != ::core::cmp::Ordering::Greater
                            }
                            (
                                &Timing::Real(ref __self_0, ref __self_1),
                                &Timing::Real(ref __arg_1_0, ref __arg_1_1),
                            ) => {
                                ::core::cmp::Ordering::then_with(
                                    ::core::option::Option::unwrap_or(
                                        ::core::cmp::PartialOrd::partial_cmp(
                                            &(*__self_0),
                                            &(*__arg_1_0),
                                        ),
                                        ::core::cmp::Ordering::Equal,
                                    ),
                                    || {
                                        ::core::option::Option::unwrap_or(
                                            ::core::cmp::PartialOrd::partial_cmp(
                                                &(*__self_1),
                                                &(*__arg_1_1),
                                            ),
                                            ::core::cmp::Ordering::Greater,
                                        )
                                    },
                                ) != ::core::cmp::Ordering::Greater
                            }
                            _ => unsafe { ::core::intrinsics::unreachable() },
                        }
                    } else {
                        __self_vi.le(&__arg_1_vi)
                    }
                }
            }
            #[inline]
            fn gt(&self, other: &Timing) -> bool {
                {
                    let __self_vi =
                        unsafe { ::core::intrinsics::discriminant_value(&*self) } as isize;
                    let __arg_1_vi =
                        unsafe { ::core::intrinsics::discriminant_value(&*other) } as isize;
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*other) {
                            (&Timing::Metrical(ref __self_0), &Timing::Metrical(ref __arg_1_0)) => {
                                ::core::option::Option::unwrap_or(
                                    ::core::cmp::PartialOrd::partial_cmp(
                                        &(*__self_0),
                                        &(*__arg_1_0),
                                    ),
                                    ::core::cmp::Ordering::Less,
                                ) == ::core::cmp::Ordering::Greater
                            }
                            (
                                &Timing::Real(ref __self_0, ref __self_1),
                                &Timing::Real(ref __arg_1_0, ref __arg_1_1),
                            ) => {
                                ::core::cmp::Ordering::then_with(
                                    ::core::option::Option::unwrap_or(
                                        ::core::cmp::PartialOrd::partial_cmp(
                                            &(*__self_0),
                                            &(*__arg_1_0),
                                        ),
                                        ::core::cmp::Ordering::Equal,
                                    ),
                                    || {
                                        ::core::option::Option::unwrap_or(
                                            ::core::cmp::PartialOrd::partial_cmp(
                                                &(*__self_1),
                                                &(*__arg_1_1),
                                            ),
                                            ::core::cmp::Ordering::Less,
                                        )
                                    },
                                ) == ::core::cmp::Ordering::Greater
                            }
                            _ => unsafe { ::core::intrinsics::unreachable() },
                        }
                    } else {
                        __self_vi.gt(&__arg_1_vi)
                    }
                }
            }
            #[inline]
            fn ge(&self, other: &Timing) -> bool {
                {
                    let __self_vi =
                        unsafe { ::core::intrinsics::discriminant_value(&*self) } as isize;
                    let __arg_1_vi =
                        unsafe { ::core::intrinsics::discriminant_value(&*other) } as isize;
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*other) {
                            (&Timing::Metrical(ref __self_0), &Timing::Metrical(ref __arg_1_0)) => {
                                ::core::option::Option::unwrap_or(
                                    ::core::cmp::PartialOrd::partial_cmp(
                                        &(*__self_0),
                                        &(*__arg_1_0),
                                    ),
                                    ::core::cmp::Ordering::Less,
                                ) != ::core::cmp::Ordering::Less
                            }
                            (
                                &Timing::Real(ref __self_0, ref __self_1),
                                &Timing::Real(ref __arg_1_0, ref __arg_1_1),
                            ) => {
                                ::core::cmp::Ordering::then_with(
                                    ::core::option::Option::unwrap_or(
                                        ::core::cmp::PartialOrd::partial_cmp(
                                            &(*__self_0),
                                            &(*__arg_1_0),
                                        ),
                                        ::core::cmp::Ordering::Equal,
                                    ),
                                    || {
                                        ::core::option::Option::unwrap_or(
                                            ::core::cmp::PartialOrd::partial_cmp(
                                                &(*__self_1),
                                                &(*__arg_1_1),
                                            ),
                                            ::core::cmp::Ordering::Less,
                                        )
                                    },
                                ) != ::core::cmp::Ordering::Less
                            }
                            _ => unsafe { ::core::intrinsics::unreachable() },
                        }
                    } else {
                        __self_vi.ge(&__arg_1_vi)
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Ord for Timing {
            #[inline]
            fn cmp(&self, other: &Timing) -> ::core::cmp::Ordering {
                {
                    let __self_vi =
                        unsafe { ::core::intrinsics::discriminant_value(&*self) } as isize;
                    let __arg_1_vi =
                        unsafe { ::core::intrinsics::discriminant_value(&*other) } as isize;
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*other) {
                            (&Timing::Metrical(ref __self_0), &Timing::Metrical(ref __arg_1_0)) => {
                                match ::core::cmp::Ord::cmp(&(*__self_0), &(*__arg_1_0)) {
                                    ::core::cmp::Ordering::Equal => ::core::cmp::Ordering::Equal,
                                    cmp => cmp,
                                }
                            }
                            (
                                &Timing::Real(ref __self_0, ref __self_1),
                                &Timing::Real(ref __arg_1_0, ref __arg_1_1),
                            ) => match ::core::cmp::Ord::cmp(&(*__self_0), &(*__arg_1_0)) {
                                ::core::cmp::Ordering::Equal => {
                                    match ::core::cmp::Ord::cmp(&(*__self_1), &(*__arg_1_1)) {
                                        ::core::cmp::Ordering::Equal => {
                                            ::core::cmp::Ordering::Equal
                                        }
                                        cmp => cmp,
                                    }
                                }
                                cmp => cmp,
                            },
                            _ => unsafe { ::core::intrinsics::unreachable() },
                        }
                    } else {
                        __self_vi.cmp(&__arg_1_vi)
                    }
                }
            }
        }
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _IMPL_DESERIALIZE_FOR_Timing: () = {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Timing {
                fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::export::Formatter,
                        ) -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter, "variant identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::export::Ok(__Field::__field0),
                                1u64 => _serde::export::Ok(__Field::__field1),
                                _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"variant index 0 <= i < 2",
                                )),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "Metrical" => _serde::export::Ok(__Field::__field0),
                                "Real" => _serde::export::Ok(__Field::__field1),
                                _ => _serde::export::Err(_serde::de::Error::unknown_variant(
                                    __value, VARIANTS,
                                )),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"Metrical" => _serde::export::Ok(__Field::__field0),
                                b"Real" => _serde::export::Ok(__Field::__field1),
                                _ => {
                                    let __value = &_serde::export::from_utf8_lossy(__value);
                                    _serde::export::Err(_serde::de::Error::unknown_variant(
                                        __value, VARIANTS,
                                    ))
                                }
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::export::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::export::PhantomData<Timing>,
                        lifetime: _serde::export::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Timing;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::export::Formatter,
                        ) -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter, "enum Timing")
                        }
                        fn visit_enum<__A>(
                            self,
                            __data: __A,
                        ) -> _serde::export::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::EnumAccess<'de>,
                        {
                            match match _serde::de::EnumAccess::variant(__data) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                (__Field::__field0, __variant) => _serde::export::Result::map(
                                    _serde::de::VariantAccess::newtype_variant::<u16>(__variant),
                                    Timing::Metrical,
                                ),
                                (__Field::__field1, __variant) => {
                                    struct __Visitor<'de> {
                                        marker: _serde::export::PhantomData<Timing>,
                                        lifetime: _serde::export::PhantomData<&'de ()>,
                                    }
                                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                        type Value = Timing;
                                        fn expecting(
                                            &self,
                                            __formatter: &mut _serde::export::Formatter,
                                        ) -> _serde::export::fmt::Result
                                        {
                                            _serde::export::Formatter::write_str(
                                                __formatter,
                                                "tuple variant Timing::Real",
                                            )
                                        }
                                        #[inline]
                                        fn visit_seq<__A>(
                                            self,
                                            mut __seq: __A,
                                        ) -> _serde::export::Result<Self::Value, __A::Error>
                                        where
                                            __A: _serde::de::SeqAccess<'de>,
                                        {
                                            let __field0 =
                                                match match _serde::de::SeqAccess::next_element::<u8>(
                                                    &mut __seq,
                                                ) {
                                                    _serde::export::Ok(__val) => __val,
                                                    _serde::export::Err(__err) => {
                                                        return _serde::export::Err(__err);
                                                    }
                                                } {
                                                    _serde::export::Some(__value) => __value,
                                                    _serde::export::None => {
                                                        return _serde :: export :: Err ( _serde :: de :: Error :: invalid_length ( 0usize , & "tuple variant Timing::Real with 2 elements" ) ) ;
                                                    }
                                                };
                                            let __field1 =
                                                match match _serde::de::SeqAccess::next_element::<u8>(
                                                    &mut __seq,
                                                ) {
                                                    _serde::export::Ok(__val) => __val,
                                                    _serde::export::Err(__err) => {
                                                        return _serde::export::Err(__err);
                                                    }
                                                } {
                                                    _serde::export::Some(__value) => __value,
                                                    _serde::export::None => {
                                                        return _serde :: export :: Err ( _serde :: de :: Error :: invalid_length ( 1usize , & "tuple variant Timing::Real with 2 elements" ) ) ;
                                                    }
                                                };
                                            _serde::export::Ok(Timing::Real(__field0, __field1))
                                        }
                                    }
                                    _serde::de::VariantAccess::tuple_variant(
                                        __variant,
                                        2usize,
                                        __Visitor {
                                            marker: _serde::export::PhantomData::<Timing>,
                                            lifetime: _serde::export::PhantomData,
                                        },
                                    )
                                }
                            }
                        }
                    }
                    const VARIANTS: &'static [&'static str] = &["Metrical", "Real"];
                    _serde::Deserializer::deserialize_enum(
                        __deserializer,
                        "Timing",
                        VARIANTS,
                        __Visitor {
                            marker: _serde::export::PhantomData::<Timing>,
                            lifetime: _serde::export::PhantomData,
                        },
                    )
                }
            }
        };
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _IMPL_SERIALIZE_FOR_Timing: () = {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Timing {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::export::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    match *self {
                        Timing::Metrical(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "Timing",
                                0u32,
                                "Metrical",
                                __field0,
                            )
                        }
                        Timing::Real(ref __field0, ref __field1) => {
                            let mut __serde_state =
                                match _serde::Serializer::serialize_tuple_variant(
                                    __serializer,
                                    "Timing",
                                    1u32,
                                    "Real",
                                    0 + 1 + 1,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                            match _serde::ser::SerializeTupleVariant::serialize_field(
                                &mut __serde_state,
                                __field0,
                            ) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                            match _serde::ser::SerializeTupleVariant::serialize_field(
                                &mut __serde_state,
                                __field1,
                            ) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                            _serde::ser::SerializeTupleVariant::end(__serde_state)
                        }
                    }
                }
            }
        };
        #[allow(warnings)]
        impl ::binread::BinRead for Timing {
            type Args = ();
            fn read_options<R: ::binread::io::Read + ::binread::io::Seek>(
                __binread_generated_var_reader: &mut R,
                __binread_generated_var_options: &::binread::ReadOptions,
                __binread_generated_var_arguments: Self::Args,
            ) -> ::binread::BinResult<Self> {
                let mut __binread_generated_error_basket: Vec<(&'static str, ::binread::Error)> =
                    ::alloc::vec::Vec::new();
                let __binread_generated_pos_before_variant = ::binread::io::Seek::seek(
                    __binread_generated_var_reader,
                    ::binread::io::SeekFrom::Current(0),
                )?;
                fn __binread_generated_parse_enum_Timing_variant_Metrical<
                    R: ::binread::io::Read + ::binread::io::Seek,
                >(
                    __binread_generated_var_reader: &mut R,
                    __binread_generated_var_options: &::binread::ReadOptions,
                    __binread_generated_var_arguments: <Timing as ::binread::BinRead>::Args,
                ) -> ::binread::BinResult<Timing> {
                    let () = __binread_generated_var_arguments;
                    let __binread_generated_var_options = __binread_generated_var_options;
                    let __self_0_binwrite_generated_args = ();
                    let __self_0_binwrite_generated_options = __binread_generated_var_options;
                    let mut self_0: u16 = {
                        let __binread__temp = (::binread::error::identity_after_parse(
                            ::binread::error::nop5,
                            ((::binread::BinRead::read_options(
                                __binread_generated_var_reader,
                                __self_0_binwrite_generated_options,
                                __self_0_binwrite_generated_args,
                            ))?),
                            __binread_generated_var_reader,
                            __self_0_binwrite_generated_options,
                            __self_0_binwrite_generated_args,
                        )?);
                        __binread__temp
                    };
                    let __binread_generated_saved_position = ::binread::io::Seek::seek(
                        __binread_generated_var_reader,
                        ::binread::io::SeekFrom::Current(0),
                    )?;
                    {
                        ::binread::BinRead::after_parse(
                            &mut self_0,
                            __binread_generated_var_reader,
                            __self_0_binwrite_generated_options,
                            __self_0_binwrite_generated_args,
                        )?
                    };
                    ::binread::io::Seek::seek(
                        __binread_generated_var_reader,
                        ::binread::io::SeekFrom::Start(__binread_generated_saved_position),
                    )?;
                    Ok(Timing::Metrical(self_0))
                }
                fn __binread_generated_parse_enum_Timing_variant_Real<
                    R: ::binread::io::Read + ::binread::io::Seek,
                >(
                    __binread_generated_var_reader: &mut R,
                    __binread_generated_var_options: &::binread::ReadOptions,
                    __binread_generated_var_arguments: <Timing as ::binread::BinRead>::Args,
                ) -> ::binread::BinResult<Timing> {
                    let () = __binread_generated_var_arguments;
                    let __binread_generated_var_options = __binread_generated_var_options;
                    let __self_0_binwrite_generated_args = ();
                    let __self_0_binwrite_generated_options = __binread_generated_var_options;
                    let mut self_0: u8 = {
                        let __binread__temp = (::binread::error::identity_after_parse(
                            ::binread::error::nop5,
                            ((::binread::BinRead::read_options(
                                __binread_generated_var_reader,
                                __self_0_binwrite_generated_options,
                                __self_0_binwrite_generated_args,
                            ))?),
                            __binread_generated_var_reader,
                            __self_0_binwrite_generated_options,
                            __self_0_binwrite_generated_args,
                        )?);
                        __binread__temp
                    };
                    let __self_1_binwrite_generated_args = ();
                    let __self_1_binwrite_generated_options = __binread_generated_var_options;
                    let mut self_1: u8 = {
                        let __binread__temp = (::binread::error::identity_after_parse(
                            ::binread::error::nop5,
                            ((::binread::BinRead::read_options(
                                __binread_generated_var_reader,
                                __self_1_binwrite_generated_options,
                                __self_1_binwrite_generated_args,
                            ))?),
                            __binread_generated_var_reader,
                            __self_1_binwrite_generated_options,
                            __self_1_binwrite_generated_args,
                        )?);
                        __binread__temp
                    };
                    let __binread_generated_saved_position = ::binread::io::Seek::seek(
                        __binread_generated_var_reader,
                        ::binread::io::SeekFrom::Current(0),
                    )?;
                    {
                        ::binread::BinRead::after_parse(
                            &mut self_0,
                            __binread_generated_var_reader,
                            __self_0_binwrite_generated_options,
                            __self_0_binwrite_generated_args,
                        )?
                    };
                    {
                        ::binread::BinRead::after_parse(
                            &mut self_1,
                            __binread_generated_var_reader,
                            __self_1_binwrite_generated_options,
                            __self_1_binwrite_generated_args,
                        )?
                    };
                    ::binread::io::Seek::seek(
                        __binread_generated_var_reader,
                        ::binread::io::SeekFrom::Start(__binread_generated_saved_position),
                    )?;
                    Ok(Timing::Real(self_0, self_1))
                }
                let __binread_generated_last_attempt =
                    __binread_generated_parse_enum_Timing_variant_Metrical(
                        __binread_generated_var_reader,
                        __binread_generated_var_options,
                        __binread_generated_var_arguments,
                    );
                if __binread_generated_last_attempt.is_ok() {
                    return __binread_generated_last_attempt;
                } else {
                    __binread_generated_error_basket
                        .push(("Metrical", __binread_generated_last_attempt.err().unwrap()));
                    ::binread::io::Seek::seek(
                        __binread_generated_var_reader,
                        ::binread::io::SeekFrom::Start(__binread_generated_pos_before_variant),
                    )?;
                }
                let __binread_generated_last_attempt =
                    __binread_generated_parse_enum_Timing_variant_Real(
                        __binread_generated_var_reader,
                        __binread_generated_var_options,
                        __binread_generated_var_arguments,
                    );
                if __binread_generated_last_attempt.is_ok() {
                    return __binread_generated_last_attempt;
                } else {
                    __binread_generated_error_basket
                        .push(("Real", __binread_generated_last_attempt.err().unwrap()));
                    ::binread::io::Seek::seek(
                        __binread_generated_var_reader,
                        ::binread::io::SeekFrom::Start(__binread_generated_pos_before_variant),
                    )?;
                }
                Err(::binread::Error::EnumErrors {
                    pos: __binread_generated_pos_before_variant as usize,
                    variant_errors: __binread_generated_error_basket,
                })
            }
            fn after_parse<R: ::binread::io::Read + ::binread::io::Seek>(
                &mut self,
                __binread_generated_var_reader: &mut R,
                __binread_generated_var_options: &::binread::ReadOptions,
                __binread_generated_var_arguments: Self::Args,
            ) -> ::binread::BinResult<()> {
                Ok(())
            }
        }
    }
    mod track {
        use super::event::Event;
        use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
        use serde::{Deserialize, Serialize};
        use std::io::{Error, Read, Seek, SeekFrom, Write};
        pub struct Track {
            events: Vec<Event>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for Track {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Track {
                        events: ref __self_0_0,
                    } => {
                        let mut debug_trait_builder = f.debug_struct("Track");
                        let _ = debug_trait_builder.field("events", &&(*__self_0_0));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _IMPL_DESERIALIZE_FOR_Track: () = {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Track {
                fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::export::Formatter,
                        ) -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::export::Ok(__Field::__field0),
                                _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"field index 0 <= i < 1",
                                )),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "events" => _serde::export::Ok(__Field::__field0),
                                _ => _serde::export::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"events" => _serde::export::Ok(__Field::__field0),
                                _ => _serde::export::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::export::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::export::PhantomData<Track>,
                        lifetime: _serde::export::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Track;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::export::Formatter,
                        ) -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter, "struct Track")
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::export::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                Vec<Event>,
                            >(&mut __seq)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Track with 1 element",
                                    ));
                                }
                            };
                            _serde::export::Ok(Track { events: __field0 })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::export::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::export::Option<Vec<Event>> =
                                _serde::export::None;
                            while let _serde::export::Some(__key) =
                                match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::export::Option::is_some(&__field0) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "events",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<Vec<Event>>(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(
                                            &mut __map
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::export::Some(__field0) => __field0,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("events") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::export::Ok(Track { events: __field0 })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["events"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Track",
                        FIELDS,
                        __Visitor {
                            marker: _serde::export::PhantomData::<Track>,
                            lifetime: _serde::export::PhantomData,
                        },
                    )
                }
            }
        };
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _IMPL_SERIALIZE_FOR_Track: () = {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Track {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::export::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "Track",
                        false as usize + 1,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "events",
                        &self.events,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
    }
    mod var_length_value {
        use crate::utils::Bounded;
        use binread::{BinRead, BinResult, ReadOptions};
        use binread::io::{Read, Seek};
        use serde::{Deserialize, Serialize};
        pub struct VarLengthValue(# [ br ( parse_with = from_stream ) ] pub(crate) u32);
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for VarLengthValue {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    VarLengthValue(ref __self_0_0) => {
                        let mut debug_trait_builder = f.debug_tuple("VarLengthValue");
                        let _ = debug_trait_builder.field(&&(*__self_0_0));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for VarLengthValue {
            #[inline]
            fn clone(&self) -> VarLengthValue {
                {
                    let _: ::core::clone::AssertParamIsClone<u32>;
                    *self
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::marker::Copy for VarLengthValue {}
        impl ::core::marker::StructuralPartialEq for VarLengthValue {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for VarLengthValue {
            #[inline]
            fn eq(&self, other: &VarLengthValue) -> bool {
                match *other {
                    VarLengthValue(ref __self_1_0) => match *self {
                        VarLengthValue(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &VarLengthValue) -> bool {
                match *other {
                    VarLengthValue(ref __self_1_0) => match *self {
                        VarLengthValue(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                    },
                }
            }
        }
        impl ::core::marker::StructuralEq for VarLengthValue {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for VarLengthValue {
            #[inline]
            #[doc(hidden)]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::core::cmp::AssertParamIsEq<u32>;
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialOrd for VarLengthValue {
            #[inline]
            fn partial_cmp(
                &self,
                other: &VarLengthValue,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                match *other {
                    VarLengthValue(ref __self_1_0) => match *self {
                        VarLengthValue(ref __self_0_0) => {
                            match ::core::cmp::PartialOrd::partial_cmp(
                                &(*__self_0_0),
                                &(*__self_1_0),
                            ) {
                                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                                    ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                                }
                                cmp => cmp,
                            }
                        }
                    },
                }
            }
            #[inline]
            fn lt(&self, other: &VarLengthValue) -> bool {
                match *other {
                    VarLengthValue(ref __self_1_0) => match *self {
                        VarLengthValue(ref __self_0_0) => {
                            ::core::option::Option::unwrap_or(
                                ::core::cmp::PartialOrd::partial_cmp(
                                    &(*__self_0_0),
                                    &(*__self_1_0),
                                ),
                                ::core::cmp::Ordering::Greater,
                            ) == ::core::cmp::Ordering::Less
                        }
                    },
                }
            }
            #[inline]
            fn le(&self, other: &VarLengthValue) -> bool {
                match *other {
                    VarLengthValue(ref __self_1_0) => match *self {
                        VarLengthValue(ref __self_0_0) => {
                            ::core::option::Option::unwrap_or(
                                ::core::cmp::PartialOrd::partial_cmp(
                                    &(*__self_0_0),
                                    &(*__self_1_0),
                                ),
                                ::core::cmp::Ordering::Greater,
                            ) != ::core::cmp::Ordering::Greater
                        }
                    },
                }
            }
            #[inline]
            fn gt(&self, other: &VarLengthValue) -> bool {
                match *other {
                    VarLengthValue(ref __self_1_0) => match *self {
                        VarLengthValue(ref __self_0_0) => {
                            ::core::option::Option::unwrap_or(
                                ::core::cmp::PartialOrd::partial_cmp(
                                    &(*__self_0_0),
                                    &(*__self_1_0),
                                ),
                                ::core::cmp::Ordering::Less,
                            ) == ::core::cmp::Ordering::Greater
                        }
                    },
                }
            }
            #[inline]
            fn ge(&self, other: &VarLengthValue) -> bool {
                match *other {
                    VarLengthValue(ref __self_1_0) => match *self {
                        VarLengthValue(ref __self_0_0) => {
                            ::core::option::Option::unwrap_or(
                                ::core::cmp::PartialOrd::partial_cmp(
                                    &(*__self_0_0),
                                    &(*__self_1_0),
                                ),
                                ::core::cmp::Ordering::Less,
                            ) != ::core::cmp::Ordering::Less
                        }
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Ord for VarLengthValue {
            #[inline]
            fn cmp(&self, other: &VarLengthValue) -> ::core::cmp::Ordering {
                match *other {
                    VarLengthValue(ref __self_1_0) => match *self {
                        VarLengthValue(ref __self_0_0) => {
                            match ::core::cmp::Ord::cmp(&(*__self_0_0), &(*__self_1_0)) {
                                ::core::cmp::Ordering::Equal => ::core::cmp::Ordering::Equal,
                                cmp => cmp,
                            }
                        }
                    },
                }
            }
        }
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _IMPL_DESERIALIZE_FOR_VarLengthValue: () = {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for VarLengthValue {
                fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    struct __Visitor<'de> {
                        marker: _serde::export::PhantomData<VarLengthValue>,
                        lifetime: _serde::export::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = VarLengthValue;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::export::Formatter,
                        ) -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(
                                __formatter,
                                "tuple struct VarLengthValue",
                            )
                        }
                        #[inline]
                        fn visit_newtype_struct<__E>(
                            self,
                            __e: __E,
                        ) -> _serde::export::Result<Self::Value, __E::Error>
                        where
                            __E: _serde::Deserializer<'de>,
                        {
                            let __field0: u32 = match <u32 as _serde::Deserialize>::deserialize(__e)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                            _serde::export::Ok(VarLengthValue(__field0))
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::export::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<u32>(
                                &mut __seq,
                            ) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        0usize,
                                        &"tuple struct VarLengthValue with 1 element",
                                    ));
                                }
                            };
                            _serde::export::Ok(VarLengthValue(__field0))
                        }
                    }
                    _serde::Deserializer::deserialize_newtype_struct(
                        __deserializer,
                        "VarLengthValue",
                        __Visitor {
                            marker: _serde::export::PhantomData::<VarLengthValue>,
                            lifetime: _serde::export::PhantomData,
                        },
                    )
                }
            }
        };
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _IMPL_SERIALIZE_FOR_VarLengthValue: () = {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for VarLengthValue {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::export::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    _serde::Serializer::serialize_newtype_struct(
                        __serializer,
                        "VarLengthValue",
                        &self.0,
                    )
                }
            }
        };
        #[allow(warnings)]
        impl ::binread::BinRead for VarLengthValue {
            type Args = ();
            fn read_options<R: ::binread::io::Read + ::binread::io::Seek>(
                __binread_generated_var_reader: &mut R,
                __binread_generated_var_options: &::binread::ReadOptions,
                __binread_generated_var_arguments: Self::Args,
            ) -> ::binread::BinResult<Self> {
                let () = __binread_generated_var_arguments;
                let __binread_generated_var_options = __binread_generated_var_options;
                let __self_0_binwrite_generated_args = ();
                let __self_0_binwrite_generated_options = __binread_generated_var_options;
                let mut self_0: u32 = {
                    let __binread__temp = (::binread::error::identity_after_parse(
                        ::binread::error::nop5,
                        ((from_stream(
                            __binread_generated_var_reader,
                            __self_0_binwrite_generated_options,
                            __self_0_binwrite_generated_args,
                        ))?),
                        __binread_generated_var_reader,
                        __self_0_binwrite_generated_options,
                        __self_0_binwrite_generated_args,
                    )?);
                    __binread__temp
                };
                let __binread_generated_saved_position = ::binread::io::Seek::seek(
                    __binread_generated_var_reader,
                    ::binread::io::SeekFrom::Current(0),
                )?;
                {
                    ::binread::error::nop5(
                        &mut self_0,
                        __binread_generated_var_reader,
                        __self_0_binwrite_generated_options,
                        __self_0_binwrite_generated_args,
                    )?
                };
                ::binread::io::Seek::seek(
                    __binread_generated_var_reader,
                    ::binread::io::SeekFrom::Start(__binread_generated_saved_position),
                )?;
                Ok(Self(self_0))
            }
            fn after_parse<R: ::binread::io::Read + ::binread::io::Seek>(
                &mut self,
                __binread_generated_var_reader: &mut R,
                __binread_generated_var_options: &::binread::ReadOptions,
                __binread_generated_var_arguments: Self::Args,
            ) -> ::binread::BinResult<()> {
                Ok(())
            }
        }
        fn from_stream<R: Read + Seek>(reader: &mut R, _: &ReadOptions, _: ()) -> BinResult<u32> {
            let mut value = 0u32;
            let max_bytes = 4;
            for _ in 0..max_bytes {
                let byte = u8::read(reader)? as u32;
                value |= byte & 0x7f;
                if (byte & 0x80) != 0 {
                    value <<= 7;
                } else {
                    break;
                }
            }
            Ok(VarLengthValue::bounded(value))
        }
        impl From<u32> for VarLengthValue {
            fn from(value: u32) -> VarLengthValue {
                Self(Self::bounded(value))
            }
        }
        impl Bounded<u32> for VarLengthValue {
            const MIN: u32 = 0;
            const MAX: u32 = 0x0fff_ffff;
        }
    }
    pub mod event {
        mod meta_event {
            use crate::midi::VarLengthValue;
            use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
            use serde::{Deserialize, Serialize};
            use std::io::{Error, ErrorKind, Read, Seek, SeekFrom, Write};
            pub enum MetaEvent {
                SequenceNumber(Option<u16>),
                Text(String),
                Copyright(String),
                TrackName(String),
                Instrument(String),
                Lyric(String),
                Marker(String),
                CuePoint(String),
                ProgramName(String),
                DeviceName(String),
                MidiChannelPrefix(u8),
                MidiPort(u8),
                EndOfTrack,
                Tempo {
                    ms_per_beat: u32,
                },
                SMPTEOffset {
                    hr: u8,
                    mn: u8,
                    se: u8,
                    fr: u8,
                    ff: u8,
                },
                TimeSignature {
                    numerator: u8,
                    denominator: u8,
                    clocks_per_metronome: u8,
                    something: u8,
                },
                KeySignature {
                    sf: i8,
                    mi: u8,
                },
                Unsupported(u8, Vec<u8>),
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::fmt::Debug for MetaEvent {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match (&*self,) {
                        (&MetaEvent::SequenceNumber(ref __self_0),) => {
                            let mut debug_trait_builder = f.debug_tuple("SequenceNumber");
                            let _ = debug_trait_builder.field(&&(*__self_0));
                            debug_trait_builder.finish()
                        }
                        (&MetaEvent::Text(ref __self_0),) => {
                            let mut debug_trait_builder = f.debug_tuple("Text");
                            let _ = debug_trait_builder.field(&&(*__self_0));
                            debug_trait_builder.finish()
                        }
                        (&MetaEvent::Copyright(ref __self_0),) => {
                            let mut debug_trait_builder = f.debug_tuple("Copyright");
                            let _ = debug_trait_builder.field(&&(*__self_0));
                            debug_trait_builder.finish()
                        }
                        (&MetaEvent::TrackName(ref __self_0),) => {
                            let mut debug_trait_builder = f.debug_tuple("TrackName");
                            let _ = debug_trait_builder.field(&&(*__self_0));
                            debug_trait_builder.finish()
                        }
                        (&MetaEvent::Instrument(ref __self_0),) => {
                            let mut debug_trait_builder = f.debug_tuple("Instrument");
                            let _ = debug_trait_builder.field(&&(*__self_0));
                            debug_trait_builder.finish()
                        }
                        (&MetaEvent::Lyric(ref __self_0),) => {
                            let mut debug_trait_builder = f.debug_tuple("Lyric");
                            let _ = debug_trait_builder.field(&&(*__self_0));
                            debug_trait_builder.finish()
                        }
                        (&MetaEvent::Marker(ref __self_0),) => {
                            let mut debug_trait_builder = f.debug_tuple("Marker");
                            let _ = debug_trait_builder.field(&&(*__self_0));
                            debug_trait_builder.finish()
                        }
                        (&MetaEvent::CuePoint(ref __self_0),) => {
                            let mut debug_trait_builder = f.debug_tuple("CuePoint");
                            let _ = debug_trait_builder.field(&&(*__self_0));
                            debug_trait_builder.finish()
                        }
                        (&MetaEvent::ProgramName(ref __self_0),) => {
                            let mut debug_trait_builder = f.debug_tuple("ProgramName");
                            let _ = debug_trait_builder.field(&&(*__self_0));
                            debug_trait_builder.finish()
                        }
                        (&MetaEvent::DeviceName(ref __self_0),) => {
                            let mut debug_trait_builder = f.debug_tuple("DeviceName");
                            let _ = debug_trait_builder.field(&&(*__self_0));
                            debug_trait_builder.finish()
                        }
                        (&MetaEvent::MidiChannelPrefix(ref __self_0),) => {
                            let mut debug_trait_builder = f.debug_tuple("MidiChannelPrefix");
                            let _ = debug_trait_builder.field(&&(*__self_0));
                            debug_trait_builder.finish()
                        }
                        (&MetaEvent::MidiPort(ref __self_0),) => {
                            let mut debug_trait_builder = f.debug_tuple("MidiPort");
                            let _ = debug_trait_builder.field(&&(*__self_0));
                            debug_trait_builder.finish()
                        }
                        (&MetaEvent::EndOfTrack,) => {
                            let mut debug_trait_builder = f.debug_tuple("EndOfTrack");
                            debug_trait_builder.finish()
                        }
                        (&MetaEvent::Tempo {
                            ms_per_beat: ref __self_0,
                        },) => {
                            let mut debug_trait_builder = f.debug_struct("Tempo");
                            let _ = debug_trait_builder.field("ms_per_beat", &&(*__self_0));
                            debug_trait_builder.finish()
                        }
                        (&MetaEvent::SMPTEOffset {
                            hr: ref __self_0,
                            mn: ref __self_1,
                            se: ref __self_2,
                            fr: ref __self_3,
                            ff: ref __self_4,
                        },) => {
                            let mut debug_trait_builder = f.debug_struct("SMPTEOffset");
                            let _ = debug_trait_builder.field("hr", &&(*__self_0));
                            let _ = debug_trait_builder.field("mn", &&(*__self_1));
                            let _ = debug_trait_builder.field("se", &&(*__self_2));
                            let _ = debug_trait_builder.field("fr", &&(*__self_3));
                            let _ = debug_trait_builder.field("ff", &&(*__self_4));
                            debug_trait_builder.finish()
                        }
                        (&MetaEvent::TimeSignature {
                            numerator: ref __self_0,
                            denominator: ref __self_1,
                            clocks_per_metronome: ref __self_2,
                            something: ref __self_3,
                        },) => {
                            let mut debug_trait_builder = f.debug_struct("TimeSignature");
                            let _ = debug_trait_builder.field("numerator", &&(*__self_0));
                            let _ = debug_trait_builder.field("denominator", &&(*__self_1));
                            let _ =
                                debug_trait_builder.field("clocks_per_metronome", &&(*__self_2));
                            let _ = debug_trait_builder.field("something", &&(*__self_3));
                            debug_trait_builder.finish()
                        }
                        (&MetaEvent::KeySignature {
                            sf: ref __self_0,
                            mi: ref __self_1,
                        },) => {
                            let mut debug_trait_builder = f.debug_struct("KeySignature");
                            let _ = debug_trait_builder.field("sf", &&(*__self_0));
                            let _ = debug_trait_builder.field("mi", &&(*__self_1));
                            debug_trait_builder.finish()
                        }
                        (&MetaEvent::Unsupported(ref __self_0, ref __self_1),) => {
                            let mut debug_trait_builder = f.debug_tuple("Unsupported");
                            let _ = debug_trait_builder.field(&&(*__self_0));
                            let _ = debug_trait_builder.field(&&(*__self_1));
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _IMPL_DESERIALIZE_FOR_MetaEvent: () = {
                #[allow(unknown_lints)]
                #[allow(rust_2018_idioms)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for MetaEvent {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        enum __Field {
                            __field0,
                            __field1,
                            __field2,
                            __field3,
                            __field4,
                            __field5,
                            __field6,
                            __field7,
                            __field8,
                            __field9,
                            __field10,
                            __field11,
                            __field12,
                            __field13,
                            __field14,
                            __field15,
                            __field16,
                            __field17,
                        }
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::export::Formatter,
                            ) -> _serde::export::fmt::Result {
                                _serde::export::Formatter::write_str(
                                    __formatter,
                                    "variant identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::export::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::export::Ok(__Field::__field0),
                                    1u64 => _serde::export::Ok(__Field::__field1),
                                    2u64 => _serde::export::Ok(__Field::__field2),
                                    3u64 => _serde::export::Ok(__Field::__field3),
                                    4u64 => _serde::export::Ok(__Field::__field4),
                                    5u64 => _serde::export::Ok(__Field::__field5),
                                    6u64 => _serde::export::Ok(__Field::__field6),
                                    7u64 => _serde::export::Ok(__Field::__field7),
                                    8u64 => _serde::export::Ok(__Field::__field8),
                                    9u64 => _serde::export::Ok(__Field::__field9),
                                    10u64 => _serde::export::Ok(__Field::__field10),
                                    11u64 => _serde::export::Ok(__Field::__field11),
                                    12u64 => _serde::export::Ok(__Field::__field12),
                                    13u64 => _serde::export::Ok(__Field::__field13),
                                    14u64 => _serde::export::Ok(__Field::__field14),
                                    15u64 => _serde::export::Ok(__Field::__field15),
                                    16u64 => _serde::export::Ok(__Field::__field16),
                                    17u64 => _serde::export::Ok(__Field::__field17),
                                    _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 18",
                                    )),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::export::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "SequenceNumber" => _serde::export::Ok(__Field::__field0),
                                    "Text" => _serde::export::Ok(__Field::__field1),
                                    "Copyright" => _serde::export::Ok(__Field::__field2),
                                    "TrackName" => _serde::export::Ok(__Field::__field3),
                                    "Instrument" => _serde::export::Ok(__Field::__field4),
                                    "Lyric" => _serde::export::Ok(__Field::__field5),
                                    "Marker" => _serde::export::Ok(__Field::__field6),
                                    "CuePoint" => _serde::export::Ok(__Field::__field7),
                                    "ProgramName" => _serde::export::Ok(__Field::__field8),
                                    "DeviceName" => _serde::export::Ok(__Field::__field9),
                                    "MidiChannelPrefix" => _serde::export::Ok(__Field::__field10),
                                    "MidiPort" => _serde::export::Ok(__Field::__field11),
                                    "EndOfTrack" => _serde::export::Ok(__Field::__field12),
                                    "Tempo" => _serde::export::Ok(__Field::__field13),
                                    "SMPTEOffset" => _serde::export::Ok(__Field::__field14),
                                    "TimeSignature" => _serde::export::Ok(__Field::__field15),
                                    "KeySignature" => _serde::export::Ok(__Field::__field16),
                                    "Unsupported" => _serde::export::Ok(__Field::__field17),
                                    _ => _serde::export::Err(_serde::de::Error::unknown_variant(
                                        __value, VARIANTS,
                                    )),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::export::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"SequenceNumber" => _serde::export::Ok(__Field::__field0),
                                    b"Text" => _serde::export::Ok(__Field::__field1),
                                    b"Copyright" => _serde::export::Ok(__Field::__field2),
                                    b"TrackName" => _serde::export::Ok(__Field::__field3),
                                    b"Instrument" => _serde::export::Ok(__Field::__field4),
                                    b"Lyric" => _serde::export::Ok(__Field::__field5),
                                    b"Marker" => _serde::export::Ok(__Field::__field6),
                                    b"CuePoint" => _serde::export::Ok(__Field::__field7),
                                    b"ProgramName" => _serde::export::Ok(__Field::__field8),
                                    b"DeviceName" => _serde::export::Ok(__Field::__field9),
                                    b"MidiChannelPrefix" => _serde::export::Ok(__Field::__field10),
                                    b"MidiPort" => _serde::export::Ok(__Field::__field11),
                                    b"EndOfTrack" => _serde::export::Ok(__Field::__field12),
                                    b"Tempo" => _serde::export::Ok(__Field::__field13),
                                    b"SMPTEOffset" => _serde::export::Ok(__Field::__field14),
                                    b"TimeSignature" => _serde::export::Ok(__Field::__field15),
                                    b"KeySignature" => _serde::export::Ok(__Field::__field16),
                                    b"Unsupported" => _serde::export::Ok(__Field::__field17),
                                    _ => {
                                        let __value = &_serde::export::from_utf8_lossy(__value);
                                        _serde::export::Err(_serde::de::Error::unknown_variant(
                                            __value, VARIANTS,
                                        ))
                                    }
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::export::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        struct __Visitor<'de> {
                            marker: _serde::export::PhantomData<MetaEvent>,
                            lifetime: _serde::export::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = MetaEvent;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::export::Formatter,
                            ) -> _serde::export::fmt::Result {
                                _serde::export::Formatter::write_str(__formatter, "enum MetaEvent")
                            }
                            fn visit_enum<__A>(
                                self,
                                __data: __A,
                            ) -> _serde::export::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::EnumAccess<'de>,
                            {
                                match match _serde::de::EnumAccess::variant(__data) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                } {
                                    (__Field::__field0, __variant) => _serde::export::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<Option<u16>>(
                                            __variant,
                                        ),
                                        MetaEvent::SequenceNumber,
                                    ),
                                    (__Field::__field1, __variant) => _serde::export::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<String>(
                                            __variant,
                                        ),
                                        MetaEvent::Text,
                                    ),
                                    (__Field::__field2, __variant) => _serde::export::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<String>(
                                            __variant,
                                        ),
                                        MetaEvent::Copyright,
                                    ),
                                    (__Field::__field3, __variant) => _serde::export::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<String>(
                                            __variant,
                                        ),
                                        MetaEvent::TrackName,
                                    ),
                                    (__Field::__field4, __variant) => _serde::export::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<String>(
                                            __variant,
                                        ),
                                        MetaEvent::Instrument,
                                    ),
                                    (__Field::__field5, __variant) => _serde::export::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<String>(
                                            __variant,
                                        ),
                                        MetaEvent::Lyric,
                                    ),
                                    (__Field::__field6, __variant) => _serde::export::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<String>(
                                            __variant,
                                        ),
                                        MetaEvent::Marker,
                                    ),
                                    (__Field::__field7, __variant) => _serde::export::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<String>(
                                            __variant,
                                        ),
                                        MetaEvent::CuePoint,
                                    ),
                                    (__Field::__field8, __variant) => _serde::export::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<String>(
                                            __variant,
                                        ),
                                        MetaEvent::ProgramName,
                                    ),
                                    (__Field::__field9, __variant) => _serde::export::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<String>(
                                            __variant,
                                        ),
                                        MetaEvent::DeviceName,
                                    ),
                                    (__Field::__field10, __variant) => _serde::export::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<u8>(__variant),
                                        MetaEvent::MidiChannelPrefix,
                                    ),
                                    (__Field::__field11, __variant) => _serde::export::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<u8>(__variant),
                                        MetaEvent::MidiPort,
                                    ),
                                    (__Field::__field12, __variant) => {
                                        match _serde::de::VariantAccess::unit_variant(__variant) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        };
                                        _serde::export::Ok(MetaEvent::EndOfTrack)
                                    }
                                    (__Field::__field13, __variant) => {
                                        #[allow(non_camel_case_types)]
                                        enum __Field {
                                            __field0,
                                            __ignore,
                                        }
                                        struct __FieldVisitor;
                                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                            type Value = __Field;
                                            fn expecting(
                                                &self,
                                                __formatter: &mut _serde::export::Formatter,
                                            ) -> _serde::export::fmt::Result
                                            {
                                                _serde::export::Formatter::write_str(
                                                    __formatter,
                                                    "field identifier",
                                                )
                                            }
                                            fn visit_u64<__E>(
                                                self,
                                                __value: u64,
                                            ) -> _serde::export::Result<Self::Value, __E>
                                            where
                                                __E: _serde::de::Error,
                                            {
                                                match __value {
                                                    0u64 => _serde::export::Ok(__Field::__field0),
                                                    _ => _serde::export::Err(
                                                        _serde::de::Error::invalid_value(
                                                            _serde::de::Unexpected::Unsigned(
                                                                __value,
                                                            ),
                                                            &"field index 0 <= i < 1",
                                                        ),
                                                    ),
                                                }
                                            }
                                            fn visit_str<__E>(
                                                self,
                                                __value: &str,
                                            ) -> _serde::export::Result<Self::Value, __E>
                                            where
                                                __E: _serde::de::Error,
                                            {
                                                match __value {
                                                    "ms_per_beat" => {
                                                        _serde::export::Ok(__Field::__field0)
                                                    }
                                                    _ => _serde::export::Ok(__Field::__ignore),
                                                }
                                            }
                                            fn visit_bytes<__E>(
                                                self,
                                                __value: &[u8],
                                            ) -> _serde::export::Result<Self::Value, __E>
                                            where
                                                __E: _serde::de::Error,
                                            {
                                                match __value {
                                                    b"ms_per_beat" => {
                                                        _serde::export::Ok(__Field::__field0)
                                                    }
                                                    _ => _serde::export::Ok(__Field::__ignore),
                                                }
                                            }
                                        }
                                        impl<'de> _serde::Deserialize<'de> for __Field {
                                            #[inline]
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::export::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::Deserializer::deserialize_identifier(
                                                    __deserializer,
                                                    __FieldVisitor,
                                                )
                                            }
                                        }
                                        struct __Visitor<'de> {
                                            marker: _serde::export::PhantomData<MetaEvent>,
                                            lifetime: _serde::export::PhantomData<&'de ()>,
                                        }
                                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                            type Value = MetaEvent;
                                            fn expecting(
                                                &self,
                                                __formatter: &mut _serde::export::Formatter,
                                            ) -> _serde::export::fmt::Result
                                            {
                                                _serde::export::Formatter::write_str(
                                                    __formatter,
                                                    "struct variant MetaEvent::Tempo",
                                                )
                                            }
                                            #[inline]
                                            fn visit_seq<__A>(
                                                self,
                                                mut __seq: __A,
                                            ) -> _serde::export::Result<Self::Value, __A::Error>
                                            where
                                                __A: _serde::de::SeqAccess<'de>,
                                            {
                                                let __field0 =
                                                    match match _serde::de::SeqAccess::next_element::<
                                                        u32,
                                                    >(
                                                        &mut __seq
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    } {
                                                        _serde::export::Some(__value) => __value,
                                                        _serde::export::None => {
                                                            return _serde :: export :: Err ( _serde :: de :: Error :: invalid_length ( 0usize , & "struct variant MetaEvent::Tempo with 1 element" ) ) ;
                                                        }
                                                    };
                                                _serde::export::Ok(MetaEvent::Tempo {
                                                    ms_per_beat: __field0,
                                                })
                                            }
                                            #[inline]
                                            fn visit_map<__A>(
                                                self,
                                                mut __map: __A,
                                            ) -> _serde::export::Result<Self::Value, __A::Error>
                                            where
                                                __A: _serde::de::MapAccess<'de>,
                                            {
                                                let mut __field0: _serde::export::Option<u32> =
                                                    _serde::export::None;
                                                while let _serde::export::Some(__key) =
                                                    match _serde::de::MapAccess::next_key::<__Field>(
                                                        &mut __map,
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    }
                                                {
                                                    match __key {
                                                        __Field::__field0 => {
                                                            if _serde::export::Option::is_some(
                                                                &__field0,
                                                            ) {
                                                                return _serde :: export :: Err ( < __A :: Error as _serde :: de :: Error > :: duplicate_field ( "ms_per_beat" ) ) ;
                                                            }
                                                            __field0 = _serde :: export :: Some ( match _serde :: de :: MapAccess :: next_value :: < u32 > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ) ;
                                                        }
                                                        _ => {
                                                            let _ = match _serde :: de :: MapAccess :: next_value :: < _serde :: de :: IgnoredAny > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ;
                                                        }
                                                    }
                                                }
                                                let __field0 = match __field0 {
                                                    _serde::export::Some(__field0) => __field0,
                                                    _serde::export::None => {
                                                        match _serde::private::de::missing_field(
                                                            "ms_per_beat",
                                                        ) {
                                                            _serde::export::Ok(__val) => __val,
                                                            _serde::export::Err(__err) => {
                                                                return _serde::export::Err(__err);
                                                            }
                                                        }
                                                    }
                                                };
                                                _serde::export::Ok(MetaEvent::Tempo {
                                                    ms_per_beat: __field0,
                                                })
                                            }
                                        }
                                        const FIELDS: &'static [&'static str] = &["ms_per_beat"];
                                        _serde::de::VariantAccess::struct_variant(
                                            __variant,
                                            FIELDS,
                                            __Visitor {
                                                marker: _serde::export::PhantomData::<MetaEvent>,
                                                lifetime: _serde::export::PhantomData,
                                            },
                                        )
                                    }
                                    (__Field::__field14, __variant) => {
                                        #[allow(non_camel_case_types)]
                                        enum __Field {
                                            __field0,
                                            __field1,
                                            __field2,
                                            __field3,
                                            __field4,
                                            __ignore,
                                        }
                                        struct __FieldVisitor;
                                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                            type Value = __Field;
                                            fn expecting(
                                                &self,
                                                __formatter: &mut _serde::export::Formatter,
                                            ) -> _serde::export::fmt::Result
                                            {
                                                _serde::export::Formatter::write_str(
                                                    __formatter,
                                                    "field identifier",
                                                )
                                            }
                                            fn visit_u64<__E>(
                                                self,
                                                __value: u64,
                                            ) -> _serde::export::Result<Self::Value, __E>
                                            where
                                                __E: _serde::de::Error,
                                            {
                                                match __value {
                                                    0u64 => _serde::export::Ok(__Field::__field0),
                                                    1u64 => _serde::export::Ok(__Field::__field1),
                                                    2u64 => _serde::export::Ok(__Field::__field2),
                                                    3u64 => _serde::export::Ok(__Field::__field3),
                                                    4u64 => _serde::export::Ok(__Field::__field4),
                                                    _ => _serde::export::Err(
                                                        _serde::de::Error::invalid_value(
                                                            _serde::de::Unexpected::Unsigned(
                                                                __value,
                                                            ),
                                                            &"field index 0 <= i < 5",
                                                        ),
                                                    ),
                                                }
                                            }
                                            fn visit_str<__E>(
                                                self,
                                                __value: &str,
                                            ) -> _serde::export::Result<Self::Value, __E>
                                            where
                                                __E: _serde::de::Error,
                                            {
                                                match __value {
                                                    "hr" => _serde::export::Ok(__Field::__field0),
                                                    "mn" => _serde::export::Ok(__Field::__field1),
                                                    "se" => _serde::export::Ok(__Field::__field2),
                                                    "fr" => _serde::export::Ok(__Field::__field3),
                                                    "ff" => _serde::export::Ok(__Field::__field4),
                                                    _ => _serde::export::Ok(__Field::__ignore),
                                                }
                                            }
                                            fn visit_bytes<__E>(
                                                self,
                                                __value: &[u8],
                                            ) -> _serde::export::Result<Self::Value, __E>
                                            where
                                                __E: _serde::de::Error,
                                            {
                                                match __value {
                                                    b"hr" => _serde::export::Ok(__Field::__field0),
                                                    b"mn" => _serde::export::Ok(__Field::__field1),
                                                    b"se" => _serde::export::Ok(__Field::__field2),
                                                    b"fr" => _serde::export::Ok(__Field::__field3),
                                                    b"ff" => _serde::export::Ok(__Field::__field4),
                                                    _ => _serde::export::Ok(__Field::__ignore),
                                                }
                                            }
                                        }
                                        impl<'de> _serde::Deserialize<'de> for __Field {
                                            #[inline]
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::export::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::Deserializer::deserialize_identifier(
                                                    __deserializer,
                                                    __FieldVisitor,
                                                )
                                            }
                                        }
                                        struct __Visitor<'de> {
                                            marker: _serde::export::PhantomData<MetaEvent>,
                                            lifetime: _serde::export::PhantomData<&'de ()>,
                                        }
                                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                            type Value = MetaEvent;
                                            fn expecting(
                                                &self,
                                                __formatter: &mut _serde::export::Formatter,
                                            ) -> _serde::export::fmt::Result
                                            {
                                                _serde::export::Formatter::write_str(
                                                    __formatter,
                                                    "struct variant MetaEvent::SMPTEOffset",
                                                )
                                            }
                                            #[inline]
                                            fn visit_seq<__A>(
                                                self,
                                                mut __seq: __A,
                                            ) -> _serde::export::Result<Self::Value, __A::Error>
                                            where
                                                __A: _serde::de::SeqAccess<'de>,
                                            {
                                                let __field0 =
                                                    match match _serde::de::SeqAccess::next_element::<
                                                        u8,
                                                    >(
                                                        &mut __seq
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    } {
                                                        _serde::export::Some(__value) => __value,
                                                        _serde::export::None => {
                                                            return _serde :: export :: Err ( _serde :: de :: Error :: invalid_length ( 0usize , & "struct variant MetaEvent::SMPTEOffset with 5 elements" ) ) ;
                                                        }
                                                    };
                                                let __field1 =
                                                    match match _serde::de::SeqAccess::next_element::<
                                                        u8,
                                                    >(
                                                        &mut __seq
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    } {
                                                        _serde::export::Some(__value) => __value,
                                                        _serde::export::None => {
                                                            return _serde :: export :: Err ( _serde :: de :: Error :: invalid_length ( 1usize , & "struct variant MetaEvent::SMPTEOffset with 5 elements" ) ) ;
                                                        }
                                                    };
                                                let __field2 =
                                                    match match _serde::de::SeqAccess::next_element::<
                                                        u8,
                                                    >(
                                                        &mut __seq
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    } {
                                                        _serde::export::Some(__value) => __value,
                                                        _serde::export::None => {
                                                            return _serde :: export :: Err ( _serde :: de :: Error :: invalid_length ( 2usize , & "struct variant MetaEvent::SMPTEOffset with 5 elements" ) ) ;
                                                        }
                                                    };
                                                let __field3 =
                                                    match match _serde::de::SeqAccess::next_element::<
                                                        u8,
                                                    >(
                                                        &mut __seq
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    } {
                                                        _serde::export::Some(__value) => __value,
                                                        _serde::export::None => {
                                                            return _serde :: export :: Err ( _serde :: de :: Error :: invalid_length ( 3usize , & "struct variant MetaEvent::SMPTEOffset with 5 elements" ) ) ;
                                                        }
                                                    };
                                                let __field4 =
                                                    match match _serde::de::SeqAccess::next_element::<
                                                        u8,
                                                    >(
                                                        &mut __seq
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    } {
                                                        _serde::export::Some(__value) => __value,
                                                        _serde::export::None => {
                                                            return _serde :: export :: Err ( _serde :: de :: Error :: invalid_length ( 4usize , & "struct variant MetaEvent::SMPTEOffset with 5 elements" ) ) ;
                                                        }
                                                    };
                                                _serde::export::Ok(MetaEvent::SMPTEOffset {
                                                    hr: __field0,
                                                    mn: __field1,
                                                    se: __field2,
                                                    fr: __field3,
                                                    ff: __field4,
                                                })
                                            }
                                            #[inline]
                                            fn visit_map<__A>(
                                                self,
                                                mut __map: __A,
                                            ) -> _serde::export::Result<Self::Value, __A::Error>
                                            where
                                                __A: _serde::de::MapAccess<'de>,
                                            {
                                                let mut __field0: _serde::export::Option<u8> =
                                                    _serde::export::None;
                                                let mut __field1: _serde::export::Option<u8> =
                                                    _serde::export::None;
                                                let mut __field2: _serde::export::Option<u8> =
                                                    _serde::export::None;
                                                let mut __field3: _serde::export::Option<u8> =
                                                    _serde::export::None;
                                                let mut __field4: _serde::export::Option<u8> =
                                                    _serde::export::None;
                                                while let _serde::export::Some(__key) =
                                                    match _serde::de::MapAccess::next_key::<__Field>(
                                                        &mut __map,
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    }
                                                {
                                                    match __key {
                                                        __Field::__field0 => {
                                                            if _serde::export::Option::is_some(
                                                                &__field0,
                                                            ) {
                                                                return _serde :: export :: Err ( < __A :: Error as _serde :: de :: Error > :: duplicate_field ( "hr" ) ) ;
                                                            }
                                                            __field0 = _serde :: export :: Some ( match _serde :: de :: MapAccess :: next_value :: < u8 > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ) ;
                                                        }
                                                        __Field::__field1 => {
                                                            if _serde::export::Option::is_some(
                                                                &__field1,
                                                            ) {
                                                                return _serde :: export :: Err ( < __A :: Error as _serde :: de :: Error > :: duplicate_field ( "mn" ) ) ;
                                                            }
                                                            __field1 = _serde :: export :: Some ( match _serde :: de :: MapAccess :: next_value :: < u8 > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ) ;
                                                        }
                                                        __Field::__field2 => {
                                                            if _serde::export::Option::is_some(
                                                                &__field2,
                                                            ) {
                                                                return _serde :: export :: Err ( < __A :: Error as _serde :: de :: Error > :: duplicate_field ( "se" ) ) ;
                                                            }
                                                            __field2 = _serde :: export :: Some ( match _serde :: de :: MapAccess :: next_value :: < u8 > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ) ;
                                                        }
                                                        __Field::__field3 => {
                                                            if _serde::export::Option::is_some(
                                                                &__field3,
                                                            ) {
                                                                return _serde :: export :: Err ( < __A :: Error as _serde :: de :: Error > :: duplicate_field ( "fr" ) ) ;
                                                            }
                                                            __field3 = _serde :: export :: Some ( match _serde :: de :: MapAccess :: next_value :: < u8 > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ) ;
                                                        }
                                                        __Field::__field4 => {
                                                            if _serde::export::Option::is_some(
                                                                &__field4,
                                                            ) {
                                                                return _serde :: export :: Err ( < __A :: Error as _serde :: de :: Error > :: duplicate_field ( "ff" ) ) ;
                                                            }
                                                            __field4 = _serde :: export :: Some ( match _serde :: de :: MapAccess :: next_value :: < u8 > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ) ;
                                                        }
                                                        _ => {
                                                            let _ = match _serde :: de :: MapAccess :: next_value :: < _serde :: de :: IgnoredAny > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ;
                                                        }
                                                    }
                                                }
                                                let __field0 = match __field0 {
                                                    _serde::export::Some(__field0) => __field0,
                                                    _serde::export::None => {
                                                        match _serde::private::de::missing_field(
                                                            "hr",
                                                        ) {
                                                            _serde::export::Ok(__val) => __val,
                                                            _serde::export::Err(__err) => {
                                                                return _serde::export::Err(__err);
                                                            }
                                                        }
                                                    }
                                                };
                                                let __field1 = match __field1 {
                                                    _serde::export::Some(__field1) => __field1,
                                                    _serde::export::None => {
                                                        match _serde::private::de::missing_field(
                                                            "mn",
                                                        ) {
                                                            _serde::export::Ok(__val) => __val,
                                                            _serde::export::Err(__err) => {
                                                                return _serde::export::Err(__err);
                                                            }
                                                        }
                                                    }
                                                };
                                                let __field2 = match __field2 {
                                                    _serde::export::Some(__field2) => __field2,
                                                    _serde::export::None => {
                                                        match _serde::private::de::missing_field(
                                                            "se",
                                                        ) {
                                                            _serde::export::Ok(__val) => __val,
                                                            _serde::export::Err(__err) => {
                                                                return _serde::export::Err(__err);
                                                            }
                                                        }
                                                    }
                                                };
                                                let __field3 = match __field3 {
                                                    _serde::export::Some(__field3) => __field3,
                                                    _serde::export::None => {
                                                        match _serde::private::de::missing_field(
                                                            "fr",
                                                        ) {
                                                            _serde::export::Ok(__val) => __val,
                                                            _serde::export::Err(__err) => {
                                                                return _serde::export::Err(__err);
                                                            }
                                                        }
                                                    }
                                                };
                                                let __field4 = match __field4 {
                                                    _serde::export::Some(__field4) => __field4,
                                                    _serde::export::None => {
                                                        match _serde::private::de::missing_field(
                                                            "ff",
                                                        ) {
                                                            _serde::export::Ok(__val) => __val,
                                                            _serde::export::Err(__err) => {
                                                                return _serde::export::Err(__err);
                                                            }
                                                        }
                                                    }
                                                };
                                                _serde::export::Ok(MetaEvent::SMPTEOffset {
                                                    hr: __field0,
                                                    mn: __field1,
                                                    se: __field2,
                                                    fr: __field3,
                                                    ff: __field4,
                                                })
                                            }
                                        }
                                        const FIELDS: &'static [&'static str] =
                                            &["hr", "mn", "se", "fr", "ff"];
                                        _serde::de::VariantAccess::struct_variant(
                                            __variant,
                                            FIELDS,
                                            __Visitor {
                                                marker: _serde::export::PhantomData::<MetaEvent>,
                                                lifetime: _serde::export::PhantomData,
                                            },
                                        )
                                    }
                                    (__Field::__field15, __variant) => {
                                        #[allow(non_camel_case_types)]
                                        enum __Field {
                                            __field0,
                                            __field1,
                                            __field2,
                                            __field3,
                                            __ignore,
                                        }
                                        struct __FieldVisitor;
                                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                            type Value = __Field;
                                            fn expecting(
                                                &self,
                                                __formatter: &mut _serde::export::Formatter,
                                            ) -> _serde::export::fmt::Result
                                            {
                                                _serde::export::Formatter::write_str(
                                                    __formatter,
                                                    "field identifier",
                                                )
                                            }
                                            fn visit_u64<__E>(
                                                self,
                                                __value: u64,
                                            ) -> _serde::export::Result<Self::Value, __E>
                                            where
                                                __E: _serde::de::Error,
                                            {
                                                match __value {
                                                    0u64 => _serde::export::Ok(__Field::__field0),
                                                    1u64 => _serde::export::Ok(__Field::__field1),
                                                    2u64 => _serde::export::Ok(__Field::__field2),
                                                    3u64 => _serde::export::Ok(__Field::__field3),
                                                    _ => _serde::export::Err(
                                                        _serde::de::Error::invalid_value(
                                                            _serde::de::Unexpected::Unsigned(
                                                                __value,
                                                            ),
                                                            &"field index 0 <= i < 4",
                                                        ),
                                                    ),
                                                }
                                            }
                                            fn visit_str<__E>(
                                                self,
                                                __value: &str,
                                            ) -> _serde::export::Result<Self::Value, __E>
                                            where
                                                __E: _serde::de::Error,
                                            {
                                                match __value {
                                                    "numerator" => {
                                                        _serde::export::Ok(__Field::__field0)
                                                    }
                                                    "denominator" => {
                                                        _serde::export::Ok(__Field::__field1)
                                                    }
                                                    "clocks_per_metronome" => {
                                                        _serde::export::Ok(__Field::__field2)
                                                    }
                                                    "something" => {
                                                        _serde::export::Ok(__Field::__field3)
                                                    }
                                                    _ => _serde::export::Ok(__Field::__ignore),
                                                }
                                            }
                                            fn visit_bytes<__E>(
                                                self,
                                                __value: &[u8],
                                            ) -> _serde::export::Result<Self::Value, __E>
                                            where
                                                __E: _serde::de::Error,
                                            {
                                                match __value {
                                                    b"numerator" => {
                                                        _serde::export::Ok(__Field::__field0)
                                                    }
                                                    b"denominator" => {
                                                        _serde::export::Ok(__Field::__field1)
                                                    }
                                                    b"clocks_per_metronome" => {
                                                        _serde::export::Ok(__Field::__field2)
                                                    }
                                                    b"something" => {
                                                        _serde::export::Ok(__Field::__field3)
                                                    }
                                                    _ => _serde::export::Ok(__Field::__ignore),
                                                }
                                            }
                                        }
                                        impl<'de> _serde::Deserialize<'de> for __Field {
                                            #[inline]
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::export::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::Deserializer::deserialize_identifier(
                                                    __deserializer,
                                                    __FieldVisitor,
                                                )
                                            }
                                        }
                                        struct __Visitor<'de> {
                                            marker: _serde::export::PhantomData<MetaEvent>,
                                            lifetime: _serde::export::PhantomData<&'de ()>,
                                        }
                                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                            type Value = MetaEvent;
                                            fn expecting(
                                                &self,
                                                __formatter: &mut _serde::export::Formatter,
                                            ) -> _serde::export::fmt::Result
                                            {
                                                _serde::export::Formatter::write_str(
                                                    __formatter,
                                                    "struct variant MetaEvent::TimeSignature",
                                                )
                                            }
                                            #[inline]
                                            fn visit_seq<__A>(
                                                self,
                                                mut __seq: __A,
                                            ) -> _serde::export::Result<Self::Value, __A::Error>
                                            where
                                                __A: _serde::de::SeqAccess<'de>,
                                            {
                                                let __field0 =
                                                    match match _serde::de::SeqAccess::next_element::<
                                                        u8,
                                                    >(
                                                        &mut __seq
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    } {
                                                        _serde::export::Some(__value) => __value,
                                                        _serde::export::None => {
                                                            return _serde :: export :: Err ( _serde :: de :: Error :: invalid_length ( 0usize , & "struct variant MetaEvent::TimeSignature with 4 elements" ) ) ;
                                                        }
                                                    };
                                                let __field1 =
                                                    match match _serde::de::SeqAccess::next_element::<
                                                        u8,
                                                    >(
                                                        &mut __seq
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    } {
                                                        _serde::export::Some(__value) => __value,
                                                        _serde::export::None => {
                                                            return _serde :: export :: Err ( _serde :: de :: Error :: invalid_length ( 1usize , & "struct variant MetaEvent::TimeSignature with 4 elements" ) ) ;
                                                        }
                                                    };
                                                let __field2 =
                                                    match match _serde::de::SeqAccess::next_element::<
                                                        u8,
                                                    >(
                                                        &mut __seq
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    } {
                                                        _serde::export::Some(__value) => __value,
                                                        _serde::export::None => {
                                                            return _serde :: export :: Err ( _serde :: de :: Error :: invalid_length ( 2usize , & "struct variant MetaEvent::TimeSignature with 4 elements" ) ) ;
                                                        }
                                                    };
                                                let __field3 =
                                                    match match _serde::de::SeqAccess::next_element::<
                                                        u8,
                                                    >(
                                                        &mut __seq
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    } {
                                                        _serde::export::Some(__value) => __value,
                                                        _serde::export::None => {
                                                            return _serde :: export :: Err ( _serde :: de :: Error :: invalid_length ( 3usize , & "struct variant MetaEvent::TimeSignature with 4 elements" ) ) ;
                                                        }
                                                    };
                                                _serde::export::Ok(MetaEvent::TimeSignature {
                                                    numerator: __field0,
                                                    denominator: __field1,
                                                    clocks_per_metronome: __field2,
                                                    something: __field3,
                                                })
                                            }
                                            #[inline]
                                            fn visit_map<__A>(
                                                self,
                                                mut __map: __A,
                                            ) -> _serde::export::Result<Self::Value, __A::Error>
                                            where
                                                __A: _serde::de::MapAccess<'de>,
                                            {
                                                let mut __field0: _serde::export::Option<u8> =
                                                    _serde::export::None;
                                                let mut __field1: _serde::export::Option<u8> =
                                                    _serde::export::None;
                                                let mut __field2: _serde::export::Option<u8> =
                                                    _serde::export::None;
                                                let mut __field3: _serde::export::Option<u8> =
                                                    _serde::export::None;
                                                while let _serde::export::Some(__key) =
                                                    match _serde::de::MapAccess::next_key::<__Field>(
                                                        &mut __map,
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    }
                                                {
                                                    match __key {
                                                        __Field::__field0 => {
                                                            if _serde::export::Option::is_some(
                                                                &__field0,
                                                            ) {
                                                                return _serde :: export :: Err ( < __A :: Error as _serde :: de :: Error > :: duplicate_field ( "numerator" ) ) ;
                                                            }
                                                            __field0 = _serde :: export :: Some ( match _serde :: de :: MapAccess :: next_value :: < u8 > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ) ;
                                                        }
                                                        __Field::__field1 => {
                                                            if _serde::export::Option::is_some(
                                                                &__field1,
                                                            ) {
                                                                return _serde :: export :: Err ( < __A :: Error as _serde :: de :: Error > :: duplicate_field ( "denominator" ) ) ;
                                                            }
                                                            __field1 = _serde :: export :: Some ( match _serde :: de :: MapAccess :: next_value :: < u8 > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ) ;
                                                        }
                                                        __Field::__field2 => {
                                                            if _serde::export::Option::is_some(
                                                                &__field2,
                                                            ) {
                                                                return _serde :: export :: Err ( < __A :: Error as _serde :: de :: Error > :: duplicate_field ( "clocks_per_metronome" ) ) ;
                                                            }
                                                            __field2 = _serde :: export :: Some ( match _serde :: de :: MapAccess :: next_value :: < u8 > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ) ;
                                                        }
                                                        __Field::__field3 => {
                                                            if _serde::export::Option::is_some(
                                                                &__field3,
                                                            ) {
                                                                return _serde :: export :: Err ( < __A :: Error as _serde :: de :: Error > :: duplicate_field ( "something" ) ) ;
                                                            }
                                                            __field3 = _serde :: export :: Some ( match _serde :: de :: MapAccess :: next_value :: < u8 > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ) ;
                                                        }
                                                        _ => {
                                                            let _ = match _serde :: de :: MapAccess :: next_value :: < _serde :: de :: IgnoredAny > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ;
                                                        }
                                                    }
                                                }
                                                let __field0 = match __field0 {
                                                    _serde::export::Some(__field0) => __field0,
                                                    _serde::export::None => {
                                                        match _serde::private::de::missing_field(
                                                            "numerator",
                                                        ) {
                                                            _serde::export::Ok(__val) => __val,
                                                            _serde::export::Err(__err) => {
                                                                return _serde::export::Err(__err);
                                                            }
                                                        }
                                                    }
                                                };
                                                let __field1 = match __field1 {
                                                    _serde::export::Some(__field1) => __field1,
                                                    _serde::export::None => {
                                                        match _serde::private::de::missing_field(
                                                            "denominator",
                                                        ) {
                                                            _serde::export::Ok(__val) => __val,
                                                            _serde::export::Err(__err) => {
                                                                return _serde::export::Err(__err);
                                                            }
                                                        }
                                                    }
                                                };
                                                let __field2 = match __field2 {
                                                    _serde::export::Some(__field2) => __field2,
                                                    _serde::export::None => {
                                                        match _serde::private::de::missing_field(
                                                            "clocks_per_metronome",
                                                        ) {
                                                            _serde::export::Ok(__val) => __val,
                                                            _serde::export::Err(__err) => {
                                                                return _serde::export::Err(__err);
                                                            }
                                                        }
                                                    }
                                                };
                                                let __field3 = match __field3 {
                                                    _serde::export::Some(__field3) => __field3,
                                                    _serde::export::None => {
                                                        match _serde::private::de::missing_field(
                                                            "something",
                                                        ) {
                                                            _serde::export::Ok(__val) => __val,
                                                            _serde::export::Err(__err) => {
                                                                return _serde::export::Err(__err);
                                                            }
                                                        }
                                                    }
                                                };
                                                _serde::export::Ok(MetaEvent::TimeSignature {
                                                    numerator: __field0,
                                                    denominator: __field1,
                                                    clocks_per_metronome: __field2,
                                                    something: __field3,
                                                })
                                            }
                                        }
                                        const FIELDS: &'static [&'static str] = &[
                                            "numerator",
                                            "denominator",
                                            "clocks_per_metronome",
                                            "something",
                                        ];
                                        _serde::de::VariantAccess::struct_variant(
                                            __variant,
                                            FIELDS,
                                            __Visitor {
                                                marker: _serde::export::PhantomData::<MetaEvent>,
                                                lifetime: _serde::export::PhantomData,
                                            },
                                        )
                                    }
                                    (__Field::__field16, __variant) => {
                                        #[allow(non_camel_case_types)]
                                        enum __Field {
                                            __field0,
                                            __field1,
                                            __ignore,
                                        }
                                        struct __FieldVisitor;
                                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                            type Value = __Field;
                                            fn expecting(
                                                &self,
                                                __formatter: &mut _serde::export::Formatter,
                                            ) -> _serde::export::fmt::Result
                                            {
                                                _serde::export::Formatter::write_str(
                                                    __formatter,
                                                    "field identifier",
                                                )
                                            }
                                            fn visit_u64<__E>(
                                                self,
                                                __value: u64,
                                            ) -> _serde::export::Result<Self::Value, __E>
                                            where
                                                __E: _serde::de::Error,
                                            {
                                                match __value {
                                                    0u64 => _serde::export::Ok(__Field::__field0),
                                                    1u64 => _serde::export::Ok(__Field::__field1),
                                                    _ => _serde::export::Err(
                                                        _serde::de::Error::invalid_value(
                                                            _serde::de::Unexpected::Unsigned(
                                                                __value,
                                                            ),
                                                            &"field index 0 <= i < 2",
                                                        ),
                                                    ),
                                                }
                                            }
                                            fn visit_str<__E>(
                                                self,
                                                __value: &str,
                                            ) -> _serde::export::Result<Self::Value, __E>
                                            where
                                                __E: _serde::de::Error,
                                            {
                                                match __value {
                                                    "sf" => _serde::export::Ok(__Field::__field0),
                                                    "mi" => _serde::export::Ok(__Field::__field1),
                                                    _ => _serde::export::Ok(__Field::__ignore),
                                                }
                                            }
                                            fn visit_bytes<__E>(
                                                self,
                                                __value: &[u8],
                                            ) -> _serde::export::Result<Self::Value, __E>
                                            where
                                                __E: _serde::de::Error,
                                            {
                                                match __value {
                                                    b"sf" => _serde::export::Ok(__Field::__field0),
                                                    b"mi" => _serde::export::Ok(__Field::__field1),
                                                    _ => _serde::export::Ok(__Field::__ignore),
                                                }
                                            }
                                        }
                                        impl<'de> _serde::Deserialize<'de> for __Field {
                                            #[inline]
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::export::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::Deserializer::deserialize_identifier(
                                                    __deserializer,
                                                    __FieldVisitor,
                                                )
                                            }
                                        }
                                        struct __Visitor<'de> {
                                            marker: _serde::export::PhantomData<MetaEvent>,
                                            lifetime: _serde::export::PhantomData<&'de ()>,
                                        }
                                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                            type Value = MetaEvent;
                                            fn expecting(
                                                &self,
                                                __formatter: &mut _serde::export::Formatter,
                                            ) -> _serde::export::fmt::Result
                                            {
                                                _serde::export::Formatter::write_str(
                                                    __formatter,
                                                    "struct variant MetaEvent::KeySignature",
                                                )
                                            }
                                            #[inline]
                                            fn visit_seq<__A>(
                                                self,
                                                mut __seq: __A,
                                            ) -> _serde::export::Result<Self::Value, __A::Error>
                                            where
                                                __A: _serde::de::SeqAccess<'de>,
                                            {
                                                let __field0 =
                                                    match match _serde::de::SeqAccess::next_element::<
                                                        i8,
                                                    >(
                                                        &mut __seq
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    } {
                                                        _serde::export::Some(__value) => __value,
                                                        _serde::export::None => {
                                                            return _serde :: export :: Err ( _serde :: de :: Error :: invalid_length ( 0usize , & "struct variant MetaEvent::KeySignature with 2 elements" ) ) ;
                                                        }
                                                    };
                                                let __field1 =
                                                    match match _serde::de::SeqAccess::next_element::<
                                                        u8,
                                                    >(
                                                        &mut __seq
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    } {
                                                        _serde::export::Some(__value) => __value,
                                                        _serde::export::None => {
                                                            return _serde :: export :: Err ( _serde :: de :: Error :: invalid_length ( 1usize , & "struct variant MetaEvent::KeySignature with 2 elements" ) ) ;
                                                        }
                                                    };
                                                _serde::export::Ok(MetaEvent::KeySignature {
                                                    sf: __field0,
                                                    mi: __field1,
                                                })
                                            }
                                            #[inline]
                                            fn visit_map<__A>(
                                                self,
                                                mut __map: __A,
                                            ) -> _serde::export::Result<Self::Value, __A::Error>
                                            where
                                                __A: _serde::de::MapAccess<'de>,
                                            {
                                                let mut __field0: _serde::export::Option<i8> =
                                                    _serde::export::None;
                                                let mut __field1: _serde::export::Option<u8> =
                                                    _serde::export::None;
                                                while let _serde::export::Some(__key) =
                                                    match _serde::de::MapAccess::next_key::<__Field>(
                                                        &mut __map,
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    }
                                                {
                                                    match __key {
                                                        __Field::__field0 => {
                                                            if _serde::export::Option::is_some(
                                                                &__field0,
                                                            ) {
                                                                return _serde :: export :: Err ( < __A :: Error as _serde :: de :: Error > :: duplicate_field ( "sf" ) ) ;
                                                            }
                                                            __field0 = _serde :: export :: Some ( match _serde :: de :: MapAccess :: next_value :: < i8 > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ) ;
                                                        }
                                                        __Field::__field1 => {
                                                            if _serde::export::Option::is_some(
                                                                &__field1,
                                                            ) {
                                                                return _serde :: export :: Err ( < __A :: Error as _serde :: de :: Error > :: duplicate_field ( "mi" ) ) ;
                                                            }
                                                            __field1 = _serde :: export :: Some ( match _serde :: de :: MapAccess :: next_value :: < u8 > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ) ;
                                                        }
                                                        _ => {
                                                            let _ = match _serde :: de :: MapAccess :: next_value :: < _serde :: de :: IgnoredAny > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ;
                                                        }
                                                    }
                                                }
                                                let __field0 = match __field0 {
                                                    _serde::export::Some(__field0) => __field0,
                                                    _serde::export::None => {
                                                        match _serde::private::de::missing_field(
                                                            "sf",
                                                        ) {
                                                            _serde::export::Ok(__val) => __val,
                                                            _serde::export::Err(__err) => {
                                                                return _serde::export::Err(__err);
                                                            }
                                                        }
                                                    }
                                                };
                                                let __field1 = match __field1 {
                                                    _serde::export::Some(__field1) => __field1,
                                                    _serde::export::None => {
                                                        match _serde::private::de::missing_field(
                                                            "mi",
                                                        ) {
                                                            _serde::export::Ok(__val) => __val,
                                                            _serde::export::Err(__err) => {
                                                                return _serde::export::Err(__err);
                                                            }
                                                        }
                                                    }
                                                };
                                                _serde::export::Ok(MetaEvent::KeySignature {
                                                    sf: __field0,
                                                    mi: __field1,
                                                })
                                            }
                                        }
                                        const FIELDS: &'static [&'static str] = &["sf", "mi"];
                                        _serde::de::VariantAccess::struct_variant(
                                            __variant,
                                            FIELDS,
                                            __Visitor {
                                                marker: _serde::export::PhantomData::<MetaEvent>,
                                                lifetime: _serde::export::PhantomData,
                                            },
                                        )
                                    }
                                    (__Field::__field17, __variant) => {
                                        struct __Visitor<'de> {
                                            marker: _serde::export::PhantomData<MetaEvent>,
                                            lifetime: _serde::export::PhantomData<&'de ()>,
                                        }
                                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                            type Value = MetaEvent;
                                            fn expecting(
                                                &self,
                                                __formatter: &mut _serde::export::Formatter,
                                            ) -> _serde::export::fmt::Result
                                            {
                                                _serde::export::Formatter::write_str(
                                                    __formatter,
                                                    "tuple variant MetaEvent::Unsupported",
                                                )
                                            }
                                            #[inline]
                                            fn visit_seq<__A>(
                                                self,
                                                mut __seq: __A,
                                            ) -> _serde::export::Result<Self::Value, __A::Error>
                                            where
                                                __A: _serde::de::SeqAccess<'de>,
                                            {
                                                let __field0 =
                                                    match match _serde::de::SeqAccess::next_element::<
                                                        u8,
                                                    >(
                                                        &mut __seq
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    } {
                                                        _serde::export::Some(__value) => __value,
                                                        _serde::export::None => {
                                                            return _serde :: export :: Err ( _serde :: de :: Error :: invalid_length ( 0usize , & "tuple variant MetaEvent::Unsupported with 2 elements" ) ) ;
                                                        }
                                                    };
                                                let __field1 =
                                                    match match _serde::de::SeqAccess::next_element::<
                                                        Vec<u8>,
                                                    >(
                                                        &mut __seq
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    } {
                                                        _serde::export::Some(__value) => __value,
                                                        _serde::export::None => {
                                                            return _serde :: export :: Err ( _serde :: de :: Error :: invalid_length ( 1usize , & "tuple variant MetaEvent::Unsupported with 2 elements" ) ) ;
                                                        }
                                                    };
                                                _serde::export::Ok(MetaEvent::Unsupported(
                                                    __field0, __field1,
                                                ))
                                            }
                                        }
                                        _serde::de::VariantAccess::tuple_variant(
                                            __variant,
                                            2usize,
                                            __Visitor {
                                                marker: _serde::export::PhantomData::<MetaEvent>,
                                                lifetime: _serde::export::PhantomData,
                                            },
                                        )
                                    }
                                }
                            }
                        }
                        const VARIANTS: &'static [&'static str] = &[
                            "SequenceNumber",
                            "Text",
                            "Copyright",
                            "TrackName",
                            "Instrument",
                            "Lyric",
                            "Marker",
                            "CuePoint",
                            "ProgramName",
                            "DeviceName",
                            "MidiChannelPrefix",
                            "MidiPort",
                            "EndOfTrack",
                            "Tempo",
                            "SMPTEOffset",
                            "TimeSignature",
                            "KeySignature",
                            "Unsupported",
                        ];
                        _serde::Deserializer::deserialize_enum(
                            __deserializer,
                            "MetaEvent",
                            VARIANTS,
                            __Visitor {
                                marker: _serde::export::PhantomData::<MetaEvent>,
                                lifetime: _serde::export::PhantomData,
                            },
                        )
                    }
                }
            };
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _IMPL_SERIALIZE_FOR_MetaEvent: () = {
                #[allow(unknown_lints)]
                #[allow(rust_2018_idioms)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for MetaEvent {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::export::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        match *self {
                            MetaEvent::SequenceNumber(ref __field0) => {
                                _serde::Serializer::serialize_newtype_variant(
                                    __serializer,
                                    "MetaEvent",
                                    0u32,
                                    "SequenceNumber",
                                    __field0,
                                )
                            }
                            MetaEvent::Text(ref __field0) => {
                                _serde::Serializer::serialize_newtype_variant(
                                    __serializer,
                                    "MetaEvent",
                                    1u32,
                                    "Text",
                                    __field0,
                                )
                            }
                            MetaEvent::Copyright(ref __field0) => {
                                _serde::Serializer::serialize_newtype_variant(
                                    __serializer,
                                    "MetaEvent",
                                    2u32,
                                    "Copyright",
                                    __field0,
                                )
                            }
                            MetaEvent::TrackName(ref __field0) => {
                                _serde::Serializer::serialize_newtype_variant(
                                    __serializer,
                                    "MetaEvent",
                                    3u32,
                                    "TrackName",
                                    __field0,
                                )
                            }
                            MetaEvent::Instrument(ref __field0) => {
                                _serde::Serializer::serialize_newtype_variant(
                                    __serializer,
                                    "MetaEvent",
                                    4u32,
                                    "Instrument",
                                    __field0,
                                )
                            }
                            MetaEvent::Lyric(ref __field0) => {
                                _serde::Serializer::serialize_newtype_variant(
                                    __serializer,
                                    "MetaEvent",
                                    5u32,
                                    "Lyric",
                                    __field0,
                                )
                            }
                            MetaEvent::Marker(ref __field0) => {
                                _serde::Serializer::serialize_newtype_variant(
                                    __serializer,
                                    "MetaEvent",
                                    6u32,
                                    "Marker",
                                    __field0,
                                )
                            }
                            MetaEvent::CuePoint(ref __field0) => {
                                _serde::Serializer::serialize_newtype_variant(
                                    __serializer,
                                    "MetaEvent",
                                    7u32,
                                    "CuePoint",
                                    __field0,
                                )
                            }
                            MetaEvent::ProgramName(ref __field0) => {
                                _serde::Serializer::serialize_newtype_variant(
                                    __serializer,
                                    "MetaEvent",
                                    8u32,
                                    "ProgramName",
                                    __field0,
                                )
                            }
                            MetaEvent::DeviceName(ref __field0) => {
                                _serde::Serializer::serialize_newtype_variant(
                                    __serializer,
                                    "MetaEvent",
                                    9u32,
                                    "DeviceName",
                                    __field0,
                                )
                            }
                            MetaEvent::MidiChannelPrefix(ref __field0) => {
                                _serde::Serializer::serialize_newtype_variant(
                                    __serializer,
                                    "MetaEvent",
                                    10u32,
                                    "MidiChannelPrefix",
                                    __field0,
                                )
                            }
                            MetaEvent::MidiPort(ref __field0) => {
                                _serde::Serializer::serialize_newtype_variant(
                                    __serializer,
                                    "MetaEvent",
                                    11u32,
                                    "MidiPort",
                                    __field0,
                                )
                            }
                            MetaEvent::EndOfTrack => _serde::Serializer::serialize_unit_variant(
                                __serializer,
                                "MetaEvent",
                                12u32,
                                "EndOfTrack",
                            ),
                            MetaEvent::Tempo { ref ms_per_beat } => {
                                let mut __serde_state =
                                    match _serde::Serializer::serialize_struct_variant(
                                        __serializer,
                                        "MetaEvent",
                                        13u32,
                                        "Tempo",
                                        0 + 1,
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                match _serde::ser::SerializeStructVariant::serialize_field(
                                    &mut __serde_state,
                                    "ms_per_beat",
                                    ms_per_beat,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                _serde::ser::SerializeStructVariant::end(__serde_state)
                            }
                            MetaEvent::SMPTEOffset {
                                ref hr,
                                ref mn,
                                ref se,
                                ref fr,
                                ref ff,
                            } => {
                                let mut __serde_state =
                                    match _serde::Serializer::serialize_struct_variant(
                                        __serializer,
                                        "MetaEvent",
                                        14u32,
                                        "SMPTEOffset",
                                        0 + 1 + 1 + 1 + 1 + 1,
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                match _serde::ser::SerializeStructVariant::serialize_field(
                                    &mut __serde_state,
                                    "hr",
                                    hr,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                match _serde::ser::SerializeStructVariant::serialize_field(
                                    &mut __serde_state,
                                    "mn",
                                    mn,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                match _serde::ser::SerializeStructVariant::serialize_field(
                                    &mut __serde_state,
                                    "se",
                                    se,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                match _serde::ser::SerializeStructVariant::serialize_field(
                                    &mut __serde_state,
                                    "fr",
                                    fr,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                match _serde::ser::SerializeStructVariant::serialize_field(
                                    &mut __serde_state,
                                    "ff",
                                    ff,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                _serde::ser::SerializeStructVariant::end(__serde_state)
                            }
                            MetaEvent::TimeSignature {
                                ref numerator,
                                ref denominator,
                                ref clocks_per_metronome,
                                ref something,
                            } => {
                                let mut __serde_state =
                                    match _serde::Serializer::serialize_struct_variant(
                                        __serializer,
                                        "MetaEvent",
                                        15u32,
                                        "TimeSignature",
                                        0 + 1 + 1 + 1 + 1,
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                match _serde::ser::SerializeStructVariant::serialize_field(
                                    &mut __serde_state,
                                    "numerator",
                                    numerator,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                match _serde::ser::SerializeStructVariant::serialize_field(
                                    &mut __serde_state,
                                    "denominator",
                                    denominator,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                match _serde::ser::SerializeStructVariant::serialize_field(
                                    &mut __serde_state,
                                    "clocks_per_metronome",
                                    clocks_per_metronome,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                match _serde::ser::SerializeStructVariant::serialize_field(
                                    &mut __serde_state,
                                    "something",
                                    something,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                _serde::ser::SerializeStructVariant::end(__serde_state)
                            }
                            MetaEvent::KeySignature { ref sf, ref mi } => {
                                let mut __serde_state =
                                    match _serde::Serializer::serialize_struct_variant(
                                        __serializer,
                                        "MetaEvent",
                                        16u32,
                                        "KeySignature",
                                        0 + 1 + 1,
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                match _serde::ser::SerializeStructVariant::serialize_field(
                                    &mut __serde_state,
                                    "sf",
                                    sf,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                match _serde::ser::SerializeStructVariant::serialize_field(
                                    &mut __serde_state,
                                    "mi",
                                    mi,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                _serde::ser::SerializeStructVariant::end(__serde_state)
                            }
                            MetaEvent::Unsupported(ref __field0, ref __field1) => {
                                let mut __serde_state =
                                    match _serde::Serializer::serialize_tuple_variant(
                                        __serializer,
                                        "MetaEvent",
                                        17u32,
                                        "Unsupported",
                                        0 + 1 + 1,
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                match _serde::ser::SerializeTupleVariant::serialize_field(
                                    &mut __serde_state,
                                    __field0,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                match _serde::ser::SerializeTupleVariant::serialize_field(
                                    &mut __serde_state,
                                    __field1,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                _serde::ser::SerializeTupleVariant::end(__serde_state)
                            }
                        }
                    }
                }
            };
        }
        mod midi_event {
            use byteorder::{ReadBytesExt, WriteBytesExt};
            use serde::{Deserialize, Serialize};
            use std::io::{Error, Read, Seek, Write};
            pub enum MidiEvent {
                NoteOff {
                    channel: u8,
                    note: u8,
                    velocity: u8,
                },
                NoteOn {
                    channel: u8,
                    note: u8,
                    velocity: u8,
                },
                NotePressure {
                    channel: u8,
                    note: u8,
                    pressure: u8,
                },
                Controller {
                    channel: u8,
                    controller: u8,
                    value: u8,
                },
                Program {
                    channel: u8,
                    program: u8,
                },
                Pressure {
                    channel: u8,
                    pressure: u8,
                },
                PitchBend {
                    channel: u8,
                    lsb: u8,
                    msb: u8,
                },
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::fmt::Debug for MidiEvent {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match (&*self,) {
                        (&MidiEvent::NoteOff {
                            channel: ref __self_0,
                            note: ref __self_1,
                            velocity: ref __self_2,
                        },) => {
                            let mut debug_trait_builder = f.debug_struct("NoteOff");
                            let _ = debug_trait_builder.field("channel", &&(*__self_0));
                            let _ = debug_trait_builder.field("note", &&(*__self_1));
                            let _ = debug_trait_builder.field("velocity", &&(*__self_2));
                            debug_trait_builder.finish()
                        }
                        (&MidiEvent::NoteOn {
                            channel: ref __self_0,
                            note: ref __self_1,
                            velocity: ref __self_2,
                        },) => {
                            let mut debug_trait_builder = f.debug_struct("NoteOn");
                            let _ = debug_trait_builder.field("channel", &&(*__self_0));
                            let _ = debug_trait_builder.field("note", &&(*__self_1));
                            let _ = debug_trait_builder.field("velocity", &&(*__self_2));
                            debug_trait_builder.finish()
                        }
                        (&MidiEvent::NotePressure {
                            channel: ref __self_0,
                            note: ref __self_1,
                            pressure: ref __self_2,
                        },) => {
                            let mut debug_trait_builder = f.debug_struct("NotePressure");
                            let _ = debug_trait_builder.field("channel", &&(*__self_0));
                            let _ = debug_trait_builder.field("note", &&(*__self_1));
                            let _ = debug_trait_builder.field("pressure", &&(*__self_2));
                            debug_trait_builder.finish()
                        }
                        (&MidiEvent::Controller {
                            channel: ref __self_0,
                            controller: ref __self_1,
                            value: ref __self_2,
                        },) => {
                            let mut debug_trait_builder = f.debug_struct("Controller");
                            let _ = debug_trait_builder.field("channel", &&(*__self_0));
                            let _ = debug_trait_builder.field("controller", &&(*__self_1));
                            let _ = debug_trait_builder.field("value", &&(*__self_2));
                            debug_trait_builder.finish()
                        }
                        (&MidiEvent::Program {
                            channel: ref __self_0,
                            program: ref __self_1,
                        },) => {
                            let mut debug_trait_builder = f.debug_struct("Program");
                            let _ = debug_trait_builder.field("channel", &&(*__self_0));
                            let _ = debug_trait_builder.field("program", &&(*__self_1));
                            debug_trait_builder.finish()
                        }
                        (&MidiEvent::Pressure {
                            channel: ref __self_0,
                            pressure: ref __self_1,
                        },) => {
                            let mut debug_trait_builder = f.debug_struct("Pressure");
                            let _ = debug_trait_builder.field("channel", &&(*__self_0));
                            let _ = debug_trait_builder.field("pressure", &&(*__self_1));
                            debug_trait_builder.finish()
                        }
                        (&MidiEvent::PitchBend {
                            channel: ref __self_0,
                            lsb: ref __self_1,
                            msb: ref __self_2,
                        },) => {
                            let mut debug_trait_builder = f.debug_struct("PitchBend");
                            let _ = debug_trait_builder.field("channel", &&(*__self_0));
                            let _ = debug_trait_builder.field("lsb", &&(*__self_1));
                            let _ = debug_trait_builder.field("msb", &&(*__self_2));
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _IMPL_DESERIALIZE_FOR_MidiEvent: () = {
                #[allow(unknown_lints)]
                #[allow(rust_2018_idioms)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for MidiEvent {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        enum __Field {
                            __field0,
                            __field1,
                            __field2,
                            __field3,
                            __field4,
                            __field5,
                            __field6,
                        }
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::export::Formatter,
                            ) -> _serde::export::fmt::Result {
                                _serde::export::Formatter::write_str(
                                    __formatter,
                                    "variant identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::export::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::export::Ok(__Field::__field0),
                                    1u64 => _serde::export::Ok(__Field::__field1),
                                    2u64 => _serde::export::Ok(__Field::__field2),
                                    3u64 => _serde::export::Ok(__Field::__field3),
                                    4u64 => _serde::export::Ok(__Field::__field4),
                                    5u64 => _serde::export::Ok(__Field::__field5),
                                    6u64 => _serde::export::Ok(__Field::__field6),
                                    _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 7",
                                    )),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::export::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "NoteOff" => _serde::export::Ok(__Field::__field0),
                                    "NoteOn" => _serde::export::Ok(__Field::__field1),
                                    "NotePressure" => _serde::export::Ok(__Field::__field2),
                                    "Controller" => _serde::export::Ok(__Field::__field3),
                                    "Program" => _serde::export::Ok(__Field::__field4),
                                    "Pressure" => _serde::export::Ok(__Field::__field5),
                                    "PitchBend" => _serde::export::Ok(__Field::__field6),
                                    _ => _serde::export::Err(_serde::de::Error::unknown_variant(
                                        __value, VARIANTS,
                                    )),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::export::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"NoteOff" => _serde::export::Ok(__Field::__field0),
                                    b"NoteOn" => _serde::export::Ok(__Field::__field1),
                                    b"NotePressure" => _serde::export::Ok(__Field::__field2),
                                    b"Controller" => _serde::export::Ok(__Field::__field3),
                                    b"Program" => _serde::export::Ok(__Field::__field4),
                                    b"Pressure" => _serde::export::Ok(__Field::__field5),
                                    b"PitchBend" => _serde::export::Ok(__Field::__field6),
                                    _ => {
                                        let __value = &_serde::export::from_utf8_lossy(__value);
                                        _serde::export::Err(_serde::de::Error::unknown_variant(
                                            __value, VARIANTS,
                                        ))
                                    }
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::export::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        struct __Visitor<'de> {
                            marker: _serde::export::PhantomData<MidiEvent>,
                            lifetime: _serde::export::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = MidiEvent;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::export::Formatter,
                            ) -> _serde::export::fmt::Result {
                                _serde::export::Formatter::write_str(__formatter, "enum MidiEvent")
                            }
                            fn visit_enum<__A>(
                                self,
                                __data: __A,
                            ) -> _serde::export::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::EnumAccess<'de>,
                            {
                                match match _serde::de::EnumAccess::variant(__data) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                } {
                                    (__Field::__field0, __variant) => {
                                        #[allow(non_camel_case_types)]
                                        enum __Field {
                                            __field0,
                                            __field1,
                                            __field2,
                                            __ignore,
                                        }
                                        struct __FieldVisitor;
                                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                            type Value = __Field;
                                            fn expecting(
                                                &self,
                                                __formatter: &mut _serde::export::Formatter,
                                            ) -> _serde::export::fmt::Result
                                            {
                                                _serde::export::Formatter::write_str(
                                                    __formatter,
                                                    "field identifier",
                                                )
                                            }
                                            fn visit_u64<__E>(
                                                self,
                                                __value: u64,
                                            ) -> _serde::export::Result<Self::Value, __E>
                                            where
                                                __E: _serde::de::Error,
                                            {
                                                match __value {
                                                    0u64 => _serde::export::Ok(__Field::__field0),
                                                    1u64 => _serde::export::Ok(__Field::__field1),
                                                    2u64 => _serde::export::Ok(__Field::__field2),
                                                    _ => _serde::export::Err(
                                                        _serde::de::Error::invalid_value(
                                                            _serde::de::Unexpected::Unsigned(
                                                                __value,
                                                            ),
                                                            &"field index 0 <= i < 3",
                                                        ),
                                                    ),
                                                }
                                            }
                                            fn visit_str<__E>(
                                                self,
                                                __value: &str,
                                            ) -> _serde::export::Result<Self::Value, __E>
                                            where
                                                __E: _serde::de::Error,
                                            {
                                                match __value {
                                                    "channel" => {
                                                        _serde::export::Ok(__Field::__field0)
                                                    }
                                                    "note" => _serde::export::Ok(__Field::__field1),
                                                    "velocity" => {
                                                        _serde::export::Ok(__Field::__field2)
                                                    }
                                                    _ => _serde::export::Ok(__Field::__ignore),
                                                }
                                            }
                                            fn visit_bytes<__E>(
                                                self,
                                                __value: &[u8],
                                            ) -> _serde::export::Result<Self::Value, __E>
                                            where
                                                __E: _serde::de::Error,
                                            {
                                                match __value {
                                                    b"channel" => {
                                                        _serde::export::Ok(__Field::__field0)
                                                    }
                                                    b"note" => {
                                                        _serde::export::Ok(__Field::__field1)
                                                    }
                                                    b"velocity" => {
                                                        _serde::export::Ok(__Field::__field2)
                                                    }
                                                    _ => _serde::export::Ok(__Field::__ignore),
                                                }
                                            }
                                        }
                                        impl<'de> _serde::Deserialize<'de> for __Field {
                                            #[inline]
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::export::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::Deserializer::deserialize_identifier(
                                                    __deserializer,
                                                    __FieldVisitor,
                                                )
                                            }
                                        }
                                        struct __Visitor<'de> {
                                            marker: _serde::export::PhantomData<MidiEvent>,
                                            lifetime: _serde::export::PhantomData<&'de ()>,
                                        }
                                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                            type Value = MidiEvent;
                                            fn expecting(
                                                &self,
                                                __formatter: &mut _serde::export::Formatter,
                                            ) -> _serde::export::fmt::Result
                                            {
                                                _serde::export::Formatter::write_str(
                                                    __formatter,
                                                    "struct variant MidiEvent::NoteOff",
                                                )
                                            }
                                            #[inline]
                                            fn visit_seq<__A>(
                                                self,
                                                mut __seq: __A,
                                            ) -> _serde::export::Result<Self::Value, __A::Error>
                                            where
                                                __A: _serde::de::SeqAccess<'de>,
                                            {
                                                let __field0 =
                                                    match match _serde::de::SeqAccess::next_element::<
                                                        u8,
                                                    >(
                                                        &mut __seq
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    } {
                                                        _serde::export::Some(__value) => __value,
                                                        _serde::export::None => {
                                                            return _serde :: export :: Err ( _serde :: de :: Error :: invalid_length ( 0usize , & "struct variant MidiEvent::NoteOff with 3 elements" ) ) ;
                                                        }
                                                    };
                                                let __field1 =
                                                    match match _serde::de::SeqAccess::next_element::<
                                                        u8,
                                                    >(
                                                        &mut __seq
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    } {
                                                        _serde::export::Some(__value) => __value,
                                                        _serde::export::None => {
                                                            return _serde :: export :: Err ( _serde :: de :: Error :: invalid_length ( 1usize , & "struct variant MidiEvent::NoteOff with 3 elements" ) ) ;
                                                        }
                                                    };
                                                let __field2 =
                                                    match match _serde::de::SeqAccess::next_element::<
                                                        u8,
                                                    >(
                                                        &mut __seq
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    } {
                                                        _serde::export::Some(__value) => __value,
                                                        _serde::export::None => {
                                                            return _serde :: export :: Err ( _serde :: de :: Error :: invalid_length ( 2usize , & "struct variant MidiEvent::NoteOff with 3 elements" ) ) ;
                                                        }
                                                    };
                                                _serde::export::Ok(MidiEvent::NoteOff {
                                                    channel: __field0,
                                                    note: __field1,
                                                    velocity: __field2,
                                                })
                                            }
                                            #[inline]
                                            fn visit_map<__A>(
                                                self,
                                                mut __map: __A,
                                            ) -> _serde::export::Result<Self::Value, __A::Error>
                                            where
                                                __A: _serde::de::MapAccess<'de>,
                                            {
                                                let mut __field0: _serde::export::Option<u8> =
                                                    _serde::export::None;
                                                let mut __field1: _serde::export::Option<u8> =
                                                    _serde::export::None;
                                                let mut __field2: _serde::export::Option<u8> =
                                                    _serde::export::None;
                                                while let _serde::export::Some(__key) =
                                                    match _serde::de::MapAccess::next_key::<__Field>(
                                                        &mut __map,
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    }
                                                {
                                                    match __key {
                                                        __Field::__field0 => {
                                                            if _serde::export::Option::is_some(
                                                                &__field0,
                                                            ) {
                                                                return _serde :: export :: Err ( < __A :: Error as _serde :: de :: Error > :: duplicate_field ( "channel" ) ) ;
                                                            }
                                                            __field0 = _serde :: export :: Some ( match _serde :: de :: MapAccess :: next_value :: < u8 > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ) ;
                                                        }
                                                        __Field::__field1 => {
                                                            if _serde::export::Option::is_some(
                                                                &__field1,
                                                            ) {
                                                                return _serde :: export :: Err ( < __A :: Error as _serde :: de :: Error > :: duplicate_field ( "note" ) ) ;
                                                            }
                                                            __field1 = _serde :: export :: Some ( match _serde :: de :: MapAccess :: next_value :: < u8 > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ) ;
                                                        }
                                                        __Field::__field2 => {
                                                            if _serde::export::Option::is_some(
                                                                &__field2,
                                                            ) {
                                                                return _serde :: export :: Err ( < __A :: Error as _serde :: de :: Error > :: duplicate_field ( "velocity" ) ) ;
                                                            }
                                                            __field2 = _serde :: export :: Some ( match _serde :: de :: MapAccess :: next_value :: < u8 > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ) ;
                                                        }
                                                        _ => {
                                                            let _ = match _serde :: de :: MapAccess :: next_value :: < _serde :: de :: IgnoredAny > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ;
                                                        }
                                                    }
                                                }
                                                let __field0 = match __field0 {
                                                    _serde::export::Some(__field0) => __field0,
                                                    _serde::export::None => {
                                                        match _serde::private::de::missing_field(
                                                            "channel",
                                                        ) {
                                                            _serde::export::Ok(__val) => __val,
                                                            _serde::export::Err(__err) => {
                                                                return _serde::export::Err(__err);
                                                            }
                                                        }
                                                    }
                                                };
                                                let __field1 = match __field1 {
                                                    _serde::export::Some(__field1) => __field1,
                                                    _serde::export::None => {
                                                        match _serde::private::de::missing_field(
                                                            "note",
                                                        ) {
                                                            _serde::export::Ok(__val) => __val,
                                                            _serde::export::Err(__err) => {
                                                                return _serde::export::Err(__err);
                                                            }
                                                        }
                                                    }
                                                };
                                                let __field2 = match __field2 {
                                                    _serde::export::Some(__field2) => __field2,
                                                    _serde::export::None => {
                                                        match _serde::private::de::missing_field(
                                                            "velocity",
                                                        ) {
                                                            _serde::export::Ok(__val) => __val,
                                                            _serde::export::Err(__err) => {
                                                                return _serde::export::Err(__err);
                                                            }
                                                        }
                                                    }
                                                };
                                                _serde::export::Ok(MidiEvent::NoteOff {
                                                    channel: __field0,
                                                    note: __field1,
                                                    velocity: __field2,
                                                })
                                            }
                                        }
                                        const FIELDS: &'static [&'static str] =
                                            &["channel", "note", "velocity"];
                                        _serde::de::VariantAccess::struct_variant(
                                            __variant,
                                            FIELDS,
                                            __Visitor {
                                                marker: _serde::export::PhantomData::<MidiEvent>,
                                                lifetime: _serde::export::PhantomData,
                                            },
                                        )
                                    }
                                    (__Field::__field1, __variant) => {
                                        #[allow(non_camel_case_types)]
                                        enum __Field {
                                            __field0,
                                            __field1,
                                            __field2,
                                            __ignore,
                                        }
                                        struct __FieldVisitor;
                                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                            type Value = __Field;
                                            fn expecting(
                                                &self,
                                                __formatter: &mut _serde::export::Formatter,
                                            ) -> _serde::export::fmt::Result
                                            {
                                                _serde::export::Formatter::write_str(
                                                    __formatter,
                                                    "field identifier",
                                                )
                                            }
                                            fn visit_u64<__E>(
                                                self,
                                                __value: u64,
                                            ) -> _serde::export::Result<Self::Value, __E>
                                            where
                                                __E: _serde::de::Error,
                                            {
                                                match __value {
                                                    0u64 => _serde::export::Ok(__Field::__field0),
                                                    1u64 => _serde::export::Ok(__Field::__field1),
                                                    2u64 => _serde::export::Ok(__Field::__field2),
                                                    _ => _serde::export::Err(
                                                        _serde::de::Error::invalid_value(
                                                            _serde::de::Unexpected::Unsigned(
                                                                __value,
                                                            ),
                                                            &"field index 0 <= i < 3",
                                                        ),
                                                    ),
                                                }
                                            }
                                            fn visit_str<__E>(
                                                self,
                                                __value: &str,
                                            ) -> _serde::export::Result<Self::Value, __E>
                                            where
                                                __E: _serde::de::Error,
                                            {
                                                match __value {
                                                    "channel" => {
                                                        _serde::export::Ok(__Field::__field0)
                                                    }
                                                    "note" => _serde::export::Ok(__Field::__field1),
                                                    "velocity" => {
                                                        _serde::export::Ok(__Field::__field2)
                                                    }
                                                    _ => _serde::export::Ok(__Field::__ignore),
                                                }
                                            }
                                            fn visit_bytes<__E>(
                                                self,
                                                __value: &[u8],
                                            ) -> _serde::export::Result<Self::Value, __E>
                                            where
                                                __E: _serde::de::Error,
                                            {
                                                match __value {
                                                    b"channel" => {
                                                        _serde::export::Ok(__Field::__field0)
                                                    }
                                                    b"note" => {
                                                        _serde::export::Ok(__Field::__field1)
                                                    }
                                                    b"velocity" => {
                                                        _serde::export::Ok(__Field::__field2)
                                                    }
                                                    _ => _serde::export::Ok(__Field::__ignore),
                                                }
                                            }
                                        }
                                        impl<'de> _serde::Deserialize<'de> for __Field {
                                            #[inline]
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::export::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::Deserializer::deserialize_identifier(
                                                    __deserializer,
                                                    __FieldVisitor,
                                                )
                                            }
                                        }
                                        struct __Visitor<'de> {
                                            marker: _serde::export::PhantomData<MidiEvent>,
                                            lifetime: _serde::export::PhantomData<&'de ()>,
                                        }
                                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                            type Value = MidiEvent;
                                            fn expecting(
                                                &self,
                                                __formatter: &mut _serde::export::Formatter,
                                            ) -> _serde::export::fmt::Result
                                            {
                                                _serde::export::Formatter::write_str(
                                                    __formatter,
                                                    "struct variant MidiEvent::NoteOn",
                                                )
                                            }
                                            #[inline]
                                            fn visit_seq<__A>(
                                                self,
                                                mut __seq: __A,
                                            ) -> _serde::export::Result<Self::Value, __A::Error>
                                            where
                                                __A: _serde::de::SeqAccess<'de>,
                                            {
                                                let __field0 =
                                                    match match _serde::de::SeqAccess::next_element::<
                                                        u8,
                                                    >(
                                                        &mut __seq
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    } {
                                                        _serde::export::Some(__value) => __value,
                                                        _serde::export::None => {
                                                            return _serde :: export :: Err ( _serde :: de :: Error :: invalid_length ( 0usize , & "struct variant MidiEvent::NoteOn with 3 elements" ) ) ;
                                                        }
                                                    };
                                                let __field1 =
                                                    match match _serde::de::SeqAccess::next_element::<
                                                        u8,
                                                    >(
                                                        &mut __seq
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    } {
                                                        _serde::export::Some(__value) => __value,
                                                        _serde::export::None => {
                                                            return _serde :: export :: Err ( _serde :: de :: Error :: invalid_length ( 1usize , & "struct variant MidiEvent::NoteOn with 3 elements" ) ) ;
                                                        }
                                                    };
                                                let __field2 =
                                                    match match _serde::de::SeqAccess::next_element::<
                                                        u8,
                                                    >(
                                                        &mut __seq
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    } {
                                                        _serde::export::Some(__value) => __value,
                                                        _serde::export::None => {
                                                            return _serde :: export :: Err ( _serde :: de :: Error :: invalid_length ( 2usize , & "struct variant MidiEvent::NoteOn with 3 elements" ) ) ;
                                                        }
                                                    };
                                                _serde::export::Ok(MidiEvent::NoteOn {
                                                    channel: __field0,
                                                    note: __field1,
                                                    velocity: __field2,
                                                })
                                            }
                                            #[inline]
                                            fn visit_map<__A>(
                                                self,
                                                mut __map: __A,
                                            ) -> _serde::export::Result<Self::Value, __A::Error>
                                            where
                                                __A: _serde::de::MapAccess<'de>,
                                            {
                                                let mut __field0: _serde::export::Option<u8> =
                                                    _serde::export::None;
                                                let mut __field1: _serde::export::Option<u8> =
                                                    _serde::export::None;
                                                let mut __field2: _serde::export::Option<u8> =
                                                    _serde::export::None;
                                                while let _serde::export::Some(__key) =
                                                    match _serde::de::MapAccess::next_key::<__Field>(
                                                        &mut __map,
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    }
                                                {
                                                    match __key {
                                                        __Field::__field0 => {
                                                            if _serde::export::Option::is_some(
                                                                &__field0,
                                                            ) {
                                                                return _serde :: export :: Err ( < __A :: Error as _serde :: de :: Error > :: duplicate_field ( "channel" ) ) ;
                                                            }
                                                            __field0 = _serde :: export :: Some ( match _serde :: de :: MapAccess :: next_value :: < u8 > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ) ;
                                                        }
                                                        __Field::__field1 => {
                                                            if _serde::export::Option::is_some(
                                                                &__field1,
                                                            ) {
                                                                return _serde :: export :: Err ( < __A :: Error as _serde :: de :: Error > :: duplicate_field ( "note" ) ) ;
                                                            }
                                                            __field1 = _serde :: export :: Some ( match _serde :: de :: MapAccess :: next_value :: < u8 > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ) ;
                                                        }
                                                        __Field::__field2 => {
                                                            if _serde::export::Option::is_some(
                                                                &__field2,
                                                            ) {
                                                                return _serde :: export :: Err ( < __A :: Error as _serde :: de :: Error > :: duplicate_field ( "velocity" ) ) ;
                                                            }
                                                            __field2 = _serde :: export :: Some ( match _serde :: de :: MapAccess :: next_value :: < u8 > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ) ;
                                                        }
                                                        _ => {
                                                            let _ = match _serde :: de :: MapAccess :: next_value :: < _serde :: de :: IgnoredAny > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ;
                                                        }
                                                    }
                                                }
                                                let __field0 = match __field0 {
                                                    _serde::export::Some(__field0) => __field0,
                                                    _serde::export::None => {
                                                        match _serde::private::de::missing_field(
                                                            "channel",
                                                        ) {
                                                            _serde::export::Ok(__val) => __val,
                                                            _serde::export::Err(__err) => {
                                                                return _serde::export::Err(__err);
                                                            }
                                                        }
                                                    }
                                                };
                                                let __field1 = match __field1 {
                                                    _serde::export::Some(__field1) => __field1,
                                                    _serde::export::None => {
                                                        match _serde::private::de::missing_field(
                                                            "note",
                                                        ) {
                                                            _serde::export::Ok(__val) => __val,
                                                            _serde::export::Err(__err) => {
                                                                return _serde::export::Err(__err);
                                                            }
                                                        }
                                                    }
                                                };
                                                let __field2 = match __field2 {
                                                    _serde::export::Some(__field2) => __field2,
                                                    _serde::export::None => {
                                                        match _serde::private::de::missing_field(
                                                            "velocity",
                                                        ) {
                                                            _serde::export::Ok(__val) => __val,
                                                            _serde::export::Err(__err) => {
                                                                return _serde::export::Err(__err);
                                                            }
                                                        }
                                                    }
                                                };
                                                _serde::export::Ok(MidiEvent::NoteOn {
                                                    channel: __field0,
                                                    note: __field1,
                                                    velocity: __field2,
                                                })
                                            }
                                        }
                                        const FIELDS: &'static [&'static str] =
                                            &["channel", "note", "velocity"];
                                        _serde::de::VariantAccess::struct_variant(
                                            __variant,
                                            FIELDS,
                                            __Visitor {
                                                marker: _serde::export::PhantomData::<MidiEvent>,
                                                lifetime: _serde::export::PhantomData,
                                            },
                                        )
                                    }
                                    (__Field::__field2, __variant) => {
                                        #[allow(non_camel_case_types)]
                                        enum __Field {
                                            __field0,
                                            __field1,
                                            __field2,
                                            __ignore,
                                        }
                                        struct __FieldVisitor;
                                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                            type Value = __Field;
                                            fn expecting(
                                                &self,
                                                __formatter: &mut _serde::export::Formatter,
                                            ) -> _serde::export::fmt::Result
                                            {
                                                _serde::export::Formatter::write_str(
                                                    __formatter,
                                                    "field identifier",
                                                )
                                            }
                                            fn visit_u64<__E>(
                                                self,
                                                __value: u64,
                                            ) -> _serde::export::Result<Self::Value, __E>
                                            where
                                                __E: _serde::de::Error,
                                            {
                                                match __value {
                                                    0u64 => _serde::export::Ok(__Field::__field0),
                                                    1u64 => _serde::export::Ok(__Field::__field1),
                                                    2u64 => _serde::export::Ok(__Field::__field2),
                                                    _ => _serde::export::Err(
                                                        _serde::de::Error::invalid_value(
                                                            _serde::de::Unexpected::Unsigned(
                                                                __value,
                                                            ),
                                                            &"field index 0 <= i < 3",
                                                        ),
                                                    ),
                                                }
                                            }
                                            fn visit_str<__E>(
                                                self,
                                                __value: &str,
                                            ) -> _serde::export::Result<Self::Value, __E>
                                            where
                                                __E: _serde::de::Error,
                                            {
                                                match __value {
                                                    "channel" => {
                                                        _serde::export::Ok(__Field::__field0)
                                                    }
                                                    "note" => _serde::export::Ok(__Field::__field1),
                                                    "pressure" => {
                                                        _serde::export::Ok(__Field::__field2)
                                                    }
                                                    _ => _serde::export::Ok(__Field::__ignore),
                                                }
                                            }
                                            fn visit_bytes<__E>(
                                                self,
                                                __value: &[u8],
                                            ) -> _serde::export::Result<Self::Value, __E>
                                            where
                                                __E: _serde::de::Error,
                                            {
                                                match __value {
                                                    b"channel" => {
                                                        _serde::export::Ok(__Field::__field0)
                                                    }
                                                    b"note" => {
                                                        _serde::export::Ok(__Field::__field1)
                                                    }
                                                    b"pressure" => {
                                                        _serde::export::Ok(__Field::__field2)
                                                    }
                                                    _ => _serde::export::Ok(__Field::__ignore),
                                                }
                                            }
                                        }
                                        impl<'de> _serde::Deserialize<'de> for __Field {
                                            #[inline]
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::export::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::Deserializer::deserialize_identifier(
                                                    __deserializer,
                                                    __FieldVisitor,
                                                )
                                            }
                                        }
                                        struct __Visitor<'de> {
                                            marker: _serde::export::PhantomData<MidiEvent>,
                                            lifetime: _serde::export::PhantomData<&'de ()>,
                                        }
                                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                            type Value = MidiEvent;
                                            fn expecting(
                                                &self,
                                                __formatter: &mut _serde::export::Formatter,
                                            ) -> _serde::export::fmt::Result
                                            {
                                                _serde::export::Formatter::write_str(
                                                    __formatter,
                                                    "struct variant MidiEvent::NotePressure",
                                                )
                                            }
                                            #[inline]
                                            fn visit_seq<__A>(
                                                self,
                                                mut __seq: __A,
                                            ) -> _serde::export::Result<Self::Value, __A::Error>
                                            where
                                                __A: _serde::de::SeqAccess<'de>,
                                            {
                                                let __field0 =
                                                    match match _serde::de::SeqAccess::next_element::<
                                                        u8,
                                                    >(
                                                        &mut __seq
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    } {
                                                        _serde::export::Some(__value) => __value,
                                                        _serde::export::None => {
                                                            return _serde :: export :: Err ( _serde :: de :: Error :: invalid_length ( 0usize , & "struct variant MidiEvent::NotePressure with 3 elements" ) ) ;
                                                        }
                                                    };
                                                let __field1 =
                                                    match match _serde::de::SeqAccess::next_element::<
                                                        u8,
                                                    >(
                                                        &mut __seq
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    } {
                                                        _serde::export::Some(__value) => __value,
                                                        _serde::export::None => {
                                                            return _serde :: export :: Err ( _serde :: de :: Error :: invalid_length ( 1usize , & "struct variant MidiEvent::NotePressure with 3 elements" ) ) ;
                                                        }
                                                    };
                                                let __field2 =
                                                    match match _serde::de::SeqAccess::next_element::<
                                                        u8,
                                                    >(
                                                        &mut __seq
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    } {
                                                        _serde::export::Some(__value) => __value,
                                                        _serde::export::None => {
                                                            return _serde :: export :: Err ( _serde :: de :: Error :: invalid_length ( 2usize , & "struct variant MidiEvent::NotePressure with 3 elements" ) ) ;
                                                        }
                                                    };
                                                _serde::export::Ok(MidiEvent::NotePressure {
                                                    channel: __field0,
                                                    note: __field1,
                                                    pressure: __field2,
                                                })
                                            }
                                            #[inline]
                                            fn visit_map<__A>(
                                                self,
                                                mut __map: __A,
                                            ) -> _serde::export::Result<Self::Value, __A::Error>
                                            where
                                                __A: _serde::de::MapAccess<'de>,
                                            {
                                                let mut __field0: _serde::export::Option<u8> =
                                                    _serde::export::None;
                                                let mut __field1: _serde::export::Option<u8> =
                                                    _serde::export::None;
                                                let mut __field2: _serde::export::Option<u8> =
                                                    _serde::export::None;
                                                while let _serde::export::Some(__key) =
                                                    match _serde::de::MapAccess::next_key::<__Field>(
                                                        &mut __map,
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    }
                                                {
                                                    match __key {
                                                        __Field::__field0 => {
                                                            if _serde::export::Option::is_some(
                                                                &__field0,
                                                            ) {
                                                                return _serde :: export :: Err ( < __A :: Error as _serde :: de :: Error > :: duplicate_field ( "channel" ) ) ;
                                                            }
                                                            __field0 = _serde :: export :: Some ( match _serde :: de :: MapAccess :: next_value :: < u8 > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ) ;
                                                        }
                                                        __Field::__field1 => {
                                                            if _serde::export::Option::is_some(
                                                                &__field1,
                                                            ) {
                                                                return _serde :: export :: Err ( < __A :: Error as _serde :: de :: Error > :: duplicate_field ( "note" ) ) ;
                                                            }
                                                            __field1 = _serde :: export :: Some ( match _serde :: de :: MapAccess :: next_value :: < u8 > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ) ;
                                                        }
                                                        __Field::__field2 => {
                                                            if _serde::export::Option::is_some(
                                                                &__field2,
                                                            ) {
                                                                return _serde :: export :: Err ( < __A :: Error as _serde :: de :: Error > :: duplicate_field ( "pressure" ) ) ;
                                                            }
                                                            __field2 = _serde :: export :: Some ( match _serde :: de :: MapAccess :: next_value :: < u8 > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ) ;
                                                        }
                                                        _ => {
                                                            let _ = match _serde :: de :: MapAccess :: next_value :: < _serde :: de :: IgnoredAny > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ;
                                                        }
                                                    }
                                                }
                                                let __field0 = match __field0 {
                                                    _serde::export::Some(__field0) => __field0,
                                                    _serde::export::None => {
                                                        match _serde::private::de::missing_field(
                                                            "channel",
                                                        ) {
                                                            _serde::export::Ok(__val) => __val,
                                                            _serde::export::Err(__err) => {
                                                                return _serde::export::Err(__err);
                                                            }
                                                        }
                                                    }
                                                };
                                                let __field1 = match __field1 {
                                                    _serde::export::Some(__field1) => __field1,
                                                    _serde::export::None => {
                                                        match _serde::private::de::missing_field(
                                                            "note",
                                                        ) {
                                                            _serde::export::Ok(__val) => __val,
                                                            _serde::export::Err(__err) => {
                                                                return _serde::export::Err(__err);
                                                            }
                                                        }
                                                    }
                                                };
                                                let __field2 = match __field2 {
                                                    _serde::export::Some(__field2) => __field2,
                                                    _serde::export::None => {
                                                        match _serde::private::de::missing_field(
                                                            "pressure",
                                                        ) {
                                                            _serde::export::Ok(__val) => __val,
                                                            _serde::export::Err(__err) => {
                                                                return _serde::export::Err(__err);
                                                            }
                                                        }
                                                    }
                                                };
                                                _serde::export::Ok(MidiEvent::NotePressure {
                                                    channel: __field0,
                                                    note: __field1,
                                                    pressure: __field2,
                                                })
                                            }
                                        }
                                        const FIELDS: &'static [&'static str] =
                                            &["channel", "note", "pressure"];
                                        _serde::de::VariantAccess::struct_variant(
                                            __variant,
                                            FIELDS,
                                            __Visitor {
                                                marker: _serde::export::PhantomData::<MidiEvent>,
                                                lifetime: _serde::export::PhantomData,
                                            },
                                        )
                                    }
                                    (__Field::__field3, __variant) => {
                                        #[allow(non_camel_case_types)]
                                        enum __Field {
                                            __field0,
                                            __field1,
                                            __field2,
                                            __ignore,
                                        }
                                        struct __FieldVisitor;
                                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                            type Value = __Field;
                                            fn expecting(
                                                &self,
                                                __formatter: &mut _serde::export::Formatter,
                                            ) -> _serde::export::fmt::Result
                                            {
                                                _serde::export::Formatter::write_str(
                                                    __formatter,
                                                    "field identifier",
                                                )
                                            }
                                            fn visit_u64<__E>(
                                                self,
                                                __value: u64,
                                            ) -> _serde::export::Result<Self::Value, __E>
                                            where
                                                __E: _serde::de::Error,
                                            {
                                                match __value {
                                                    0u64 => _serde::export::Ok(__Field::__field0),
                                                    1u64 => _serde::export::Ok(__Field::__field1),
                                                    2u64 => _serde::export::Ok(__Field::__field2),
                                                    _ => _serde::export::Err(
                                                        _serde::de::Error::invalid_value(
                                                            _serde::de::Unexpected::Unsigned(
                                                                __value,
                                                            ),
                                                            &"field index 0 <= i < 3",
                                                        ),
                                                    ),
                                                }
                                            }
                                            fn visit_str<__E>(
                                                self,
                                                __value: &str,
                                            ) -> _serde::export::Result<Self::Value, __E>
                                            where
                                                __E: _serde::de::Error,
                                            {
                                                match __value {
                                                    "channel" => {
                                                        _serde::export::Ok(__Field::__field0)
                                                    }
                                                    "controller" => {
                                                        _serde::export::Ok(__Field::__field1)
                                                    }
                                                    "value" => {
                                                        _serde::export::Ok(__Field::__field2)
                                                    }
                                                    _ => _serde::export::Ok(__Field::__ignore),
                                                }
                                            }
                                            fn visit_bytes<__E>(
                                                self,
                                                __value: &[u8],
                                            ) -> _serde::export::Result<Self::Value, __E>
                                            where
                                                __E: _serde::de::Error,
                                            {
                                                match __value {
                                                    b"channel" => {
                                                        _serde::export::Ok(__Field::__field0)
                                                    }
                                                    b"controller" => {
                                                        _serde::export::Ok(__Field::__field1)
                                                    }
                                                    b"value" => {
                                                        _serde::export::Ok(__Field::__field2)
                                                    }
                                                    _ => _serde::export::Ok(__Field::__ignore),
                                                }
                                            }
                                        }
                                        impl<'de> _serde::Deserialize<'de> for __Field {
                                            #[inline]
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::export::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::Deserializer::deserialize_identifier(
                                                    __deserializer,
                                                    __FieldVisitor,
                                                )
                                            }
                                        }
                                        struct __Visitor<'de> {
                                            marker: _serde::export::PhantomData<MidiEvent>,
                                            lifetime: _serde::export::PhantomData<&'de ()>,
                                        }
                                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                            type Value = MidiEvent;
                                            fn expecting(
                                                &self,
                                                __formatter: &mut _serde::export::Formatter,
                                            ) -> _serde::export::fmt::Result
                                            {
                                                _serde::export::Formatter::write_str(
                                                    __formatter,
                                                    "struct variant MidiEvent::Controller",
                                                )
                                            }
                                            #[inline]
                                            fn visit_seq<__A>(
                                                self,
                                                mut __seq: __A,
                                            ) -> _serde::export::Result<Self::Value, __A::Error>
                                            where
                                                __A: _serde::de::SeqAccess<'de>,
                                            {
                                                let __field0 =
                                                    match match _serde::de::SeqAccess::next_element::<
                                                        u8,
                                                    >(
                                                        &mut __seq
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    } {
                                                        _serde::export::Some(__value) => __value,
                                                        _serde::export::None => {
                                                            return _serde :: export :: Err ( _serde :: de :: Error :: invalid_length ( 0usize , & "struct variant MidiEvent::Controller with 3 elements" ) ) ;
                                                        }
                                                    };
                                                let __field1 =
                                                    match match _serde::de::SeqAccess::next_element::<
                                                        u8,
                                                    >(
                                                        &mut __seq
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    } {
                                                        _serde::export::Some(__value) => __value,
                                                        _serde::export::None => {
                                                            return _serde :: export :: Err ( _serde :: de :: Error :: invalid_length ( 1usize , & "struct variant MidiEvent::Controller with 3 elements" ) ) ;
                                                        }
                                                    };
                                                let __field2 =
                                                    match match _serde::de::SeqAccess::next_element::<
                                                        u8,
                                                    >(
                                                        &mut __seq
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    } {
                                                        _serde::export::Some(__value) => __value,
                                                        _serde::export::None => {
                                                            return _serde :: export :: Err ( _serde :: de :: Error :: invalid_length ( 2usize , & "struct variant MidiEvent::Controller with 3 elements" ) ) ;
                                                        }
                                                    };
                                                _serde::export::Ok(MidiEvent::Controller {
                                                    channel: __field0,
                                                    controller: __field1,
                                                    value: __field2,
                                                })
                                            }
                                            #[inline]
                                            fn visit_map<__A>(
                                                self,
                                                mut __map: __A,
                                            ) -> _serde::export::Result<Self::Value, __A::Error>
                                            where
                                                __A: _serde::de::MapAccess<'de>,
                                            {
                                                let mut __field0: _serde::export::Option<u8> =
                                                    _serde::export::None;
                                                let mut __field1: _serde::export::Option<u8> =
                                                    _serde::export::None;
                                                let mut __field2: _serde::export::Option<u8> =
                                                    _serde::export::None;
                                                while let _serde::export::Some(__key) =
                                                    match _serde::de::MapAccess::next_key::<__Field>(
                                                        &mut __map,
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    }
                                                {
                                                    match __key {
                                                        __Field::__field0 => {
                                                            if _serde::export::Option::is_some(
                                                                &__field0,
                                                            ) {
                                                                return _serde :: export :: Err ( < __A :: Error as _serde :: de :: Error > :: duplicate_field ( "channel" ) ) ;
                                                            }
                                                            __field0 = _serde :: export :: Some ( match _serde :: de :: MapAccess :: next_value :: < u8 > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ) ;
                                                        }
                                                        __Field::__field1 => {
                                                            if _serde::export::Option::is_some(
                                                                &__field1,
                                                            ) {
                                                                return _serde :: export :: Err ( < __A :: Error as _serde :: de :: Error > :: duplicate_field ( "controller" ) ) ;
                                                            }
                                                            __field1 = _serde :: export :: Some ( match _serde :: de :: MapAccess :: next_value :: < u8 > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ) ;
                                                        }
                                                        __Field::__field2 => {
                                                            if _serde::export::Option::is_some(
                                                                &__field2,
                                                            ) {
                                                                return _serde :: export :: Err ( < __A :: Error as _serde :: de :: Error > :: duplicate_field ( "value" ) ) ;
                                                            }
                                                            __field2 = _serde :: export :: Some ( match _serde :: de :: MapAccess :: next_value :: < u8 > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ) ;
                                                        }
                                                        _ => {
                                                            let _ = match _serde :: de :: MapAccess :: next_value :: < _serde :: de :: IgnoredAny > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ;
                                                        }
                                                    }
                                                }
                                                let __field0 = match __field0 {
                                                    _serde::export::Some(__field0) => __field0,
                                                    _serde::export::None => {
                                                        match _serde::private::de::missing_field(
                                                            "channel",
                                                        ) {
                                                            _serde::export::Ok(__val) => __val,
                                                            _serde::export::Err(__err) => {
                                                                return _serde::export::Err(__err);
                                                            }
                                                        }
                                                    }
                                                };
                                                let __field1 = match __field1 {
                                                    _serde::export::Some(__field1) => __field1,
                                                    _serde::export::None => {
                                                        match _serde::private::de::missing_field(
                                                            "controller",
                                                        ) {
                                                            _serde::export::Ok(__val) => __val,
                                                            _serde::export::Err(__err) => {
                                                                return _serde::export::Err(__err);
                                                            }
                                                        }
                                                    }
                                                };
                                                let __field2 = match __field2 {
                                                    _serde::export::Some(__field2) => __field2,
                                                    _serde::export::None => {
                                                        match _serde::private::de::missing_field(
                                                            "value",
                                                        ) {
                                                            _serde::export::Ok(__val) => __val,
                                                            _serde::export::Err(__err) => {
                                                                return _serde::export::Err(__err);
                                                            }
                                                        }
                                                    }
                                                };
                                                _serde::export::Ok(MidiEvent::Controller {
                                                    channel: __field0,
                                                    controller: __field1,
                                                    value: __field2,
                                                })
                                            }
                                        }
                                        const FIELDS: &'static [&'static str] =
                                            &["channel", "controller", "value"];
                                        _serde::de::VariantAccess::struct_variant(
                                            __variant,
                                            FIELDS,
                                            __Visitor {
                                                marker: _serde::export::PhantomData::<MidiEvent>,
                                                lifetime: _serde::export::PhantomData,
                                            },
                                        )
                                    }
                                    (__Field::__field4, __variant) => {
                                        #[allow(non_camel_case_types)]
                                        enum __Field {
                                            __field0,
                                            __field1,
                                            __ignore,
                                        }
                                        struct __FieldVisitor;
                                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                            type Value = __Field;
                                            fn expecting(
                                                &self,
                                                __formatter: &mut _serde::export::Formatter,
                                            ) -> _serde::export::fmt::Result
                                            {
                                                _serde::export::Formatter::write_str(
                                                    __formatter,
                                                    "field identifier",
                                                )
                                            }
                                            fn visit_u64<__E>(
                                                self,
                                                __value: u64,
                                            ) -> _serde::export::Result<Self::Value, __E>
                                            where
                                                __E: _serde::de::Error,
                                            {
                                                match __value {
                                                    0u64 => _serde::export::Ok(__Field::__field0),
                                                    1u64 => _serde::export::Ok(__Field::__field1),
                                                    _ => _serde::export::Err(
                                                        _serde::de::Error::invalid_value(
                                                            _serde::de::Unexpected::Unsigned(
                                                                __value,
                                                            ),
                                                            &"field index 0 <= i < 2",
                                                        ),
                                                    ),
                                                }
                                            }
                                            fn visit_str<__E>(
                                                self,
                                                __value: &str,
                                            ) -> _serde::export::Result<Self::Value, __E>
                                            where
                                                __E: _serde::de::Error,
                                            {
                                                match __value {
                                                    "channel" => {
                                                        _serde::export::Ok(__Field::__field0)
                                                    }
                                                    "program" => {
                                                        _serde::export::Ok(__Field::__field1)
                                                    }
                                                    _ => _serde::export::Ok(__Field::__ignore),
                                                }
                                            }
                                            fn visit_bytes<__E>(
                                                self,
                                                __value: &[u8],
                                            ) -> _serde::export::Result<Self::Value, __E>
                                            where
                                                __E: _serde::de::Error,
                                            {
                                                match __value {
                                                    b"channel" => {
                                                        _serde::export::Ok(__Field::__field0)
                                                    }
                                                    b"program" => {
                                                        _serde::export::Ok(__Field::__field1)
                                                    }
                                                    _ => _serde::export::Ok(__Field::__ignore),
                                                }
                                            }
                                        }
                                        impl<'de> _serde::Deserialize<'de> for __Field {
                                            #[inline]
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::export::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::Deserializer::deserialize_identifier(
                                                    __deserializer,
                                                    __FieldVisitor,
                                                )
                                            }
                                        }
                                        struct __Visitor<'de> {
                                            marker: _serde::export::PhantomData<MidiEvent>,
                                            lifetime: _serde::export::PhantomData<&'de ()>,
                                        }
                                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                            type Value = MidiEvent;
                                            fn expecting(
                                                &self,
                                                __formatter: &mut _serde::export::Formatter,
                                            ) -> _serde::export::fmt::Result
                                            {
                                                _serde::export::Formatter::write_str(
                                                    __formatter,
                                                    "struct variant MidiEvent::Program",
                                                )
                                            }
                                            #[inline]
                                            fn visit_seq<__A>(
                                                self,
                                                mut __seq: __A,
                                            ) -> _serde::export::Result<Self::Value, __A::Error>
                                            where
                                                __A: _serde::de::SeqAccess<'de>,
                                            {
                                                let __field0 =
                                                    match match _serde::de::SeqAccess::next_element::<
                                                        u8,
                                                    >(
                                                        &mut __seq
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    } {
                                                        _serde::export::Some(__value) => __value,
                                                        _serde::export::None => {
                                                            return _serde :: export :: Err ( _serde :: de :: Error :: invalid_length ( 0usize , & "struct variant MidiEvent::Program with 2 elements" ) ) ;
                                                        }
                                                    };
                                                let __field1 =
                                                    match match _serde::de::SeqAccess::next_element::<
                                                        u8,
                                                    >(
                                                        &mut __seq
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    } {
                                                        _serde::export::Some(__value) => __value,
                                                        _serde::export::None => {
                                                            return _serde :: export :: Err ( _serde :: de :: Error :: invalid_length ( 1usize , & "struct variant MidiEvent::Program with 2 elements" ) ) ;
                                                        }
                                                    };
                                                _serde::export::Ok(MidiEvent::Program {
                                                    channel: __field0,
                                                    program: __field1,
                                                })
                                            }
                                            #[inline]
                                            fn visit_map<__A>(
                                                self,
                                                mut __map: __A,
                                            ) -> _serde::export::Result<Self::Value, __A::Error>
                                            where
                                                __A: _serde::de::MapAccess<'de>,
                                            {
                                                let mut __field0: _serde::export::Option<u8> =
                                                    _serde::export::None;
                                                let mut __field1: _serde::export::Option<u8> =
                                                    _serde::export::None;
                                                while let _serde::export::Some(__key) =
                                                    match _serde::de::MapAccess::next_key::<__Field>(
                                                        &mut __map,
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    }
                                                {
                                                    match __key {
                                                        __Field::__field0 => {
                                                            if _serde::export::Option::is_some(
                                                                &__field0,
                                                            ) {
                                                                return _serde :: export :: Err ( < __A :: Error as _serde :: de :: Error > :: duplicate_field ( "channel" ) ) ;
                                                            }
                                                            __field0 = _serde :: export :: Some ( match _serde :: de :: MapAccess :: next_value :: < u8 > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ) ;
                                                        }
                                                        __Field::__field1 => {
                                                            if _serde::export::Option::is_some(
                                                                &__field1,
                                                            ) {
                                                                return _serde :: export :: Err ( < __A :: Error as _serde :: de :: Error > :: duplicate_field ( "program" ) ) ;
                                                            }
                                                            __field1 = _serde :: export :: Some ( match _serde :: de :: MapAccess :: next_value :: < u8 > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ) ;
                                                        }
                                                        _ => {
                                                            let _ = match _serde :: de :: MapAccess :: next_value :: < _serde :: de :: IgnoredAny > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ;
                                                        }
                                                    }
                                                }
                                                let __field0 = match __field0 {
                                                    _serde::export::Some(__field0) => __field0,
                                                    _serde::export::None => {
                                                        match _serde::private::de::missing_field(
                                                            "channel",
                                                        ) {
                                                            _serde::export::Ok(__val) => __val,
                                                            _serde::export::Err(__err) => {
                                                                return _serde::export::Err(__err);
                                                            }
                                                        }
                                                    }
                                                };
                                                let __field1 = match __field1 {
                                                    _serde::export::Some(__field1) => __field1,
                                                    _serde::export::None => {
                                                        match _serde::private::de::missing_field(
                                                            "program",
                                                        ) {
                                                            _serde::export::Ok(__val) => __val,
                                                            _serde::export::Err(__err) => {
                                                                return _serde::export::Err(__err);
                                                            }
                                                        }
                                                    }
                                                };
                                                _serde::export::Ok(MidiEvent::Program {
                                                    channel: __field0,
                                                    program: __field1,
                                                })
                                            }
                                        }
                                        const FIELDS: &'static [&'static str] =
                                            &["channel", "program"];
                                        _serde::de::VariantAccess::struct_variant(
                                            __variant,
                                            FIELDS,
                                            __Visitor {
                                                marker: _serde::export::PhantomData::<MidiEvent>,
                                                lifetime: _serde::export::PhantomData,
                                            },
                                        )
                                    }
                                    (__Field::__field5, __variant) => {
                                        #[allow(non_camel_case_types)]
                                        enum __Field {
                                            __field0,
                                            __field1,
                                            __ignore,
                                        }
                                        struct __FieldVisitor;
                                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                            type Value = __Field;
                                            fn expecting(
                                                &self,
                                                __formatter: &mut _serde::export::Formatter,
                                            ) -> _serde::export::fmt::Result
                                            {
                                                _serde::export::Formatter::write_str(
                                                    __formatter,
                                                    "field identifier",
                                                )
                                            }
                                            fn visit_u64<__E>(
                                                self,
                                                __value: u64,
                                            ) -> _serde::export::Result<Self::Value, __E>
                                            where
                                                __E: _serde::de::Error,
                                            {
                                                match __value {
                                                    0u64 => _serde::export::Ok(__Field::__field0),
                                                    1u64 => _serde::export::Ok(__Field::__field1),
                                                    _ => _serde::export::Err(
                                                        _serde::de::Error::invalid_value(
                                                            _serde::de::Unexpected::Unsigned(
                                                                __value,
                                                            ),
                                                            &"field index 0 <= i < 2",
                                                        ),
                                                    ),
                                                }
                                            }
                                            fn visit_str<__E>(
                                                self,
                                                __value: &str,
                                            ) -> _serde::export::Result<Self::Value, __E>
                                            where
                                                __E: _serde::de::Error,
                                            {
                                                match __value {
                                                    "channel" => {
                                                        _serde::export::Ok(__Field::__field0)
                                                    }
                                                    "pressure" => {
                                                        _serde::export::Ok(__Field::__field1)
                                                    }
                                                    _ => _serde::export::Ok(__Field::__ignore),
                                                }
                                            }
                                            fn visit_bytes<__E>(
                                                self,
                                                __value: &[u8],
                                            ) -> _serde::export::Result<Self::Value, __E>
                                            where
                                                __E: _serde::de::Error,
                                            {
                                                match __value {
                                                    b"channel" => {
                                                        _serde::export::Ok(__Field::__field0)
                                                    }
                                                    b"pressure" => {
                                                        _serde::export::Ok(__Field::__field1)
                                                    }
                                                    _ => _serde::export::Ok(__Field::__ignore),
                                                }
                                            }
                                        }
                                        impl<'de> _serde::Deserialize<'de> for __Field {
                                            #[inline]
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::export::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::Deserializer::deserialize_identifier(
                                                    __deserializer,
                                                    __FieldVisitor,
                                                )
                                            }
                                        }
                                        struct __Visitor<'de> {
                                            marker: _serde::export::PhantomData<MidiEvent>,
                                            lifetime: _serde::export::PhantomData<&'de ()>,
                                        }
                                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                            type Value = MidiEvent;
                                            fn expecting(
                                                &self,
                                                __formatter: &mut _serde::export::Formatter,
                                            ) -> _serde::export::fmt::Result
                                            {
                                                _serde::export::Formatter::write_str(
                                                    __formatter,
                                                    "struct variant MidiEvent::Pressure",
                                                )
                                            }
                                            #[inline]
                                            fn visit_seq<__A>(
                                                self,
                                                mut __seq: __A,
                                            ) -> _serde::export::Result<Self::Value, __A::Error>
                                            where
                                                __A: _serde::de::SeqAccess<'de>,
                                            {
                                                let __field0 =
                                                    match match _serde::de::SeqAccess::next_element::<
                                                        u8,
                                                    >(
                                                        &mut __seq
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    } {
                                                        _serde::export::Some(__value) => __value,
                                                        _serde::export::None => {
                                                            return _serde :: export :: Err ( _serde :: de :: Error :: invalid_length ( 0usize , & "struct variant MidiEvent::Pressure with 2 elements" ) ) ;
                                                        }
                                                    };
                                                let __field1 =
                                                    match match _serde::de::SeqAccess::next_element::<
                                                        u8,
                                                    >(
                                                        &mut __seq
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    } {
                                                        _serde::export::Some(__value) => __value,
                                                        _serde::export::None => {
                                                            return _serde :: export :: Err ( _serde :: de :: Error :: invalid_length ( 1usize , & "struct variant MidiEvent::Pressure with 2 elements" ) ) ;
                                                        }
                                                    };
                                                _serde::export::Ok(MidiEvent::Pressure {
                                                    channel: __field0,
                                                    pressure: __field1,
                                                })
                                            }
                                            #[inline]
                                            fn visit_map<__A>(
                                                self,
                                                mut __map: __A,
                                            ) -> _serde::export::Result<Self::Value, __A::Error>
                                            where
                                                __A: _serde::de::MapAccess<'de>,
                                            {
                                                let mut __field0: _serde::export::Option<u8> =
                                                    _serde::export::None;
                                                let mut __field1: _serde::export::Option<u8> =
                                                    _serde::export::None;
                                                while let _serde::export::Some(__key) =
                                                    match _serde::de::MapAccess::next_key::<__Field>(
                                                        &mut __map,
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    }
                                                {
                                                    match __key {
                                                        __Field::__field0 => {
                                                            if _serde::export::Option::is_some(
                                                                &__field0,
                                                            ) {
                                                                return _serde :: export :: Err ( < __A :: Error as _serde :: de :: Error > :: duplicate_field ( "channel" ) ) ;
                                                            }
                                                            __field0 = _serde :: export :: Some ( match _serde :: de :: MapAccess :: next_value :: < u8 > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ) ;
                                                        }
                                                        __Field::__field1 => {
                                                            if _serde::export::Option::is_some(
                                                                &__field1,
                                                            ) {
                                                                return _serde :: export :: Err ( < __A :: Error as _serde :: de :: Error > :: duplicate_field ( "pressure" ) ) ;
                                                            }
                                                            __field1 = _serde :: export :: Some ( match _serde :: de :: MapAccess :: next_value :: < u8 > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ) ;
                                                        }
                                                        _ => {
                                                            let _ = match _serde :: de :: MapAccess :: next_value :: < _serde :: de :: IgnoredAny > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ;
                                                        }
                                                    }
                                                }
                                                let __field0 = match __field0 {
                                                    _serde::export::Some(__field0) => __field0,
                                                    _serde::export::None => {
                                                        match _serde::private::de::missing_field(
                                                            "channel",
                                                        ) {
                                                            _serde::export::Ok(__val) => __val,
                                                            _serde::export::Err(__err) => {
                                                                return _serde::export::Err(__err);
                                                            }
                                                        }
                                                    }
                                                };
                                                let __field1 = match __field1 {
                                                    _serde::export::Some(__field1) => __field1,
                                                    _serde::export::None => {
                                                        match _serde::private::de::missing_field(
                                                            "pressure",
                                                        ) {
                                                            _serde::export::Ok(__val) => __val,
                                                            _serde::export::Err(__err) => {
                                                                return _serde::export::Err(__err);
                                                            }
                                                        }
                                                    }
                                                };
                                                _serde::export::Ok(MidiEvent::Pressure {
                                                    channel: __field0,
                                                    pressure: __field1,
                                                })
                                            }
                                        }
                                        const FIELDS: &'static [&'static str] =
                                            &["channel", "pressure"];
                                        _serde::de::VariantAccess::struct_variant(
                                            __variant,
                                            FIELDS,
                                            __Visitor {
                                                marker: _serde::export::PhantomData::<MidiEvent>,
                                                lifetime: _serde::export::PhantomData,
                                            },
                                        )
                                    }
                                    (__Field::__field6, __variant) => {
                                        #[allow(non_camel_case_types)]
                                        enum __Field {
                                            __field0,
                                            __field1,
                                            __field2,
                                            __ignore,
                                        }
                                        struct __FieldVisitor;
                                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                            type Value = __Field;
                                            fn expecting(
                                                &self,
                                                __formatter: &mut _serde::export::Formatter,
                                            ) -> _serde::export::fmt::Result
                                            {
                                                _serde::export::Formatter::write_str(
                                                    __formatter,
                                                    "field identifier",
                                                )
                                            }
                                            fn visit_u64<__E>(
                                                self,
                                                __value: u64,
                                            ) -> _serde::export::Result<Self::Value, __E>
                                            where
                                                __E: _serde::de::Error,
                                            {
                                                match __value {
                                                    0u64 => _serde::export::Ok(__Field::__field0),
                                                    1u64 => _serde::export::Ok(__Field::__field1),
                                                    2u64 => _serde::export::Ok(__Field::__field2),
                                                    _ => _serde::export::Err(
                                                        _serde::de::Error::invalid_value(
                                                            _serde::de::Unexpected::Unsigned(
                                                                __value,
                                                            ),
                                                            &"field index 0 <= i < 3",
                                                        ),
                                                    ),
                                                }
                                            }
                                            fn visit_str<__E>(
                                                self,
                                                __value: &str,
                                            ) -> _serde::export::Result<Self::Value, __E>
                                            where
                                                __E: _serde::de::Error,
                                            {
                                                match __value {
                                                    "channel" => {
                                                        _serde::export::Ok(__Field::__field0)
                                                    }
                                                    "lsb" => _serde::export::Ok(__Field::__field1),
                                                    "msb" => _serde::export::Ok(__Field::__field2),
                                                    _ => _serde::export::Ok(__Field::__ignore),
                                                }
                                            }
                                            fn visit_bytes<__E>(
                                                self,
                                                __value: &[u8],
                                            ) -> _serde::export::Result<Self::Value, __E>
                                            where
                                                __E: _serde::de::Error,
                                            {
                                                match __value {
                                                    b"channel" => {
                                                        _serde::export::Ok(__Field::__field0)
                                                    }
                                                    b"lsb" => _serde::export::Ok(__Field::__field1),
                                                    b"msb" => _serde::export::Ok(__Field::__field2),
                                                    _ => _serde::export::Ok(__Field::__ignore),
                                                }
                                            }
                                        }
                                        impl<'de> _serde::Deserialize<'de> for __Field {
                                            #[inline]
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::export::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::Deserializer::deserialize_identifier(
                                                    __deserializer,
                                                    __FieldVisitor,
                                                )
                                            }
                                        }
                                        struct __Visitor<'de> {
                                            marker: _serde::export::PhantomData<MidiEvent>,
                                            lifetime: _serde::export::PhantomData<&'de ()>,
                                        }
                                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                            type Value = MidiEvent;
                                            fn expecting(
                                                &self,
                                                __formatter: &mut _serde::export::Formatter,
                                            ) -> _serde::export::fmt::Result
                                            {
                                                _serde::export::Formatter::write_str(
                                                    __formatter,
                                                    "struct variant MidiEvent::PitchBend",
                                                )
                                            }
                                            #[inline]
                                            fn visit_seq<__A>(
                                                self,
                                                mut __seq: __A,
                                            ) -> _serde::export::Result<Self::Value, __A::Error>
                                            where
                                                __A: _serde::de::SeqAccess<'de>,
                                            {
                                                let __field0 =
                                                    match match _serde::de::SeqAccess::next_element::<
                                                        u8,
                                                    >(
                                                        &mut __seq
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    } {
                                                        _serde::export::Some(__value) => __value,
                                                        _serde::export::None => {
                                                            return _serde :: export :: Err ( _serde :: de :: Error :: invalid_length ( 0usize , & "struct variant MidiEvent::PitchBend with 3 elements" ) ) ;
                                                        }
                                                    };
                                                let __field1 =
                                                    match match _serde::de::SeqAccess::next_element::<
                                                        u8,
                                                    >(
                                                        &mut __seq
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    } {
                                                        _serde::export::Some(__value) => __value,
                                                        _serde::export::None => {
                                                            return _serde :: export :: Err ( _serde :: de :: Error :: invalid_length ( 1usize , & "struct variant MidiEvent::PitchBend with 3 elements" ) ) ;
                                                        }
                                                    };
                                                let __field2 =
                                                    match match _serde::de::SeqAccess::next_element::<
                                                        u8,
                                                    >(
                                                        &mut __seq
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    } {
                                                        _serde::export::Some(__value) => __value,
                                                        _serde::export::None => {
                                                            return _serde :: export :: Err ( _serde :: de :: Error :: invalid_length ( 2usize , & "struct variant MidiEvent::PitchBend with 3 elements" ) ) ;
                                                        }
                                                    };
                                                _serde::export::Ok(MidiEvent::PitchBend {
                                                    channel: __field0,
                                                    lsb: __field1,
                                                    msb: __field2,
                                                })
                                            }
                                            #[inline]
                                            fn visit_map<__A>(
                                                self,
                                                mut __map: __A,
                                            ) -> _serde::export::Result<Self::Value, __A::Error>
                                            where
                                                __A: _serde::de::MapAccess<'de>,
                                            {
                                                let mut __field0: _serde::export::Option<u8> =
                                                    _serde::export::None;
                                                let mut __field1: _serde::export::Option<u8> =
                                                    _serde::export::None;
                                                let mut __field2: _serde::export::Option<u8> =
                                                    _serde::export::None;
                                                while let _serde::export::Some(__key) =
                                                    match _serde::de::MapAccess::next_key::<__Field>(
                                                        &mut __map,
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    }
                                                {
                                                    match __key {
                                                        __Field::__field0 => {
                                                            if _serde::export::Option::is_some(
                                                                &__field0,
                                                            ) {
                                                                return _serde :: export :: Err ( < __A :: Error as _serde :: de :: Error > :: duplicate_field ( "channel" ) ) ;
                                                            }
                                                            __field0 = _serde :: export :: Some ( match _serde :: de :: MapAccess :: next_value :: < u8 > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ) ;
                                                        }
                                                        __Field::__field1 => {
                                                            if _serde::export::Option::is_some(
                                                                &__field1,
                                                            ) {
                                                                return _serde :: export :: Err ( < __A :: Error as _serde :: de :: Error > :: duplicate_field ( "lsb" ) ) ;
                                                            }
                                                            __field1 = _serde :: export :: Some ( match _serde :: de :: MapAccess :: next_value :: < u8 > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ) ;
                                                        }
                                                        __Field::__field2 => {
                                                            if _serde::export::Option::is_some(
                                                                &__field2,
                                                            ) {
                                                                return _serde :: export :: Err ( < __A :: Error as _serde :: de :: Error > :: duplicate_field ( "msb" ) ) ;
                                                            }
                                                            __field2 = _serde :: export :: Some ( match _serde :: de :: MapAccess :: next_value :: < u8 > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ) ;
                                                        }
                                                        _ => {
                                                            let _ = match _serde :: de :: MapAccess :: next_value :: < _serde :: de :: IgnoredAny > ( & mut __map ) { _serde :: export :: Ok ( __val ) => __val , _serde :: export :: Err ( __err ) => { return _serde :: export :: Err ( __err ) ; } } ;
                                                        }
                                                    }
                                                }
                                                let __field0 = match __field0 {
                                                    _serde::export::Some(__field0) => __field0,
                                                    _serde::export::None => {
                                                        match _serde::private::de::missing_field(
                                                            "channel",
                                                        ) {
                                                            _serde::export::Ok(__val) => __val,
                                                            _serde::export::Err(__err) => {
                                                                return _serde::export::Err(__err);
                                                            }
                                                        }
                                                    }
                                                };
                                                let __field1 = match __field1 {
                                                    _serde::export::Some(__field1) => __field1,
                                                    _serde::export::None => {
                                                        match _serde::private::de::missing_field(
                                                            "lsb",
                                                        ) {
                                                            _serde::export::Ok(__val) => __val,
                                                            _serde::export::Err(__err) => {
                                                                return _serde::export::Err(__err);
                                                            }
                                                        }
                                                    }
                                                };
                                                let __field2 = match __field2 {
                                                    _serde::export::Some(__field2) => __field2,
                                                    _serde::export::None => {
                                                        match _serde::private::de::missing_field(
                                                            "msb",
                                                        ) {
                                                            _serde::export::Ok(__val) => __val,
                                                            _serde::export::Err(__err) => {
                                                                return _serde::export::Err(__err);
                                                            }
                                                        }
                                                    }
                                                };
                                                _serde::export::Ok(MidiEvent::PitchBend {
                                                    channel: __field0,
                                                    lsb: __field1,
                                                    msb: __field2,
                                                })
                                            }
                                        }
                                        const FIELDS: &'static [&'static str] =
                                            &["channel", "lsb", "msb"];
                                        _serde::de::VariantAccess::struct_variant(
                                            __variant,
                                            FIELDS,
                                            __Visitor {
                                                marker: _serde::export::PhantomData::<MidiEvent>,
                                                lifetime: _serde::export::PhantomData,
                                            },
                                        )
                                    }
                                }
                            }
                        }
                        const VARIANTS: &'static [&'static str] = &[
                            "NoteOff",
                            "NoteOn",
                            "NotePressure",
                            "Controller",
                            "Program",
                            "Pressure",
                            "PitchBend",
                        ];
                        _serde::Deserializer::deserialize_enum(
                            __deserializer,
                            "MidiEvent",
                            VARIANTS,
                            __Visitor {
                                marker: _serde::export::PhantomData::<MidiEvent>,
                                lifetime: _serde::export::PhantomData,
                            },
                        )
                    }
                }
            };
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _IMPL_SERIALIZE_FOR_MidiEvent: () = {
                #[allow(unknown_lints)]
                #[allow(rust_2018_idioms)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for MidiEvent {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::export::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        match *self {
                            MidiEvent::NoteOff {
                                ref channel,
                                ref note,
                                ref velocity,
                            } => {
                                let mut __serde_state =
                                    match _serde::Serializer::serialize_struct_variant(
                                        __serializer,
                                        "MidiEvent",
                                        0u32,
                                        "NoteOff",
                                        0 + 1 + 1 + 1,
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                match _serde::ser::SerializeStructVariant::serialize_field(
                                    &mut __serde_state,
                                    "channel",
                                    channel,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                match _serde::ser::SerializeStructVariant::serialize_field(
                                    &mut __serde_state,
                                    "note",
                                    note,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                match _serde::ser::SerializeStructVariant::serialize_field(
                                    &mut __serde_state,
                                    "velocity",
                                    velocity,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                _serde::ser::SerializeStructVariant::end(__serde_state)
                            }
                            MidiEvent::NoteOn {
                                ref channel,
                                ref note,
                                ref velocity,
                            } => {
                                let mut __serde_state =
                                    match _serde::Serializer::serialize_struct_variant(
                                        __serializer,
                                        "MidiEvent",
                                        1u32,
                                        "NoteOn",
                                        0 + 1 + 1 + 1,
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                match _serde::ser::SerializeStructVariant::serialize_field(
                                    &mut __serde_state,
                                    "channel",
                                    channel,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                match _serde::ser::SerializeStructVariant::serialize_field(
                                    &mut __serde_state,
                                    "note",
                                    note,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                match _serde::ser::SerializeStructVariant::serialize_field(
                                    &mut __serde_state,
                                    "velocity",
                                    velocity,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                _serde::ser::SerializeStructVariant::end(__serde_state)
                            }
                            MidiEvent::NotePressure {
                                ref channel,
                                ref note,
                                ref pressure,
                            } => {
                                let mut __serde_state =
                                    match _serde::Serializer::serialize_struct_variant(
                                        __serializer,
                                        "MidiEvent",
                                        2u32,
                                        "NotePressure",
                                        0 + 1 + 1 + 1,
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                match _serde::ser::SerializeStructVariant::serialize_field(
                                    &mut __serde_state,
                                    "channel",
                                    channel,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                match _serde::ser::SerializeStructVariant::serialize_field(
                                    &mut __serde_state,
                                    "note",
                                    note,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                match _serde::ser::SerializeStructVariant::serialize_field(
                                    &mut __serde_state,
                                    "pressure",
                                    pressure,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                _serde::ser::SerializeStructVariant::end(__serde_state)
                            }
                            MidiEvent::Controller {
                                ref channel,
                                ref controller,
                                ref value,
                            } => {
                                let mut __serde_state =
                                    match _serde::Serializer::serialize_struct_variant(
                                        __serializer,
                                        "MidiEvent",
                                        3u32,
                                        "Controller",
                                        0 + 1 + 1 + 1,
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                match _serde::ser::SerializeStructVariant::serialize_field(
                                    &mut __serde_state,
                                    "channel",
                                    channel,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                match _serde::ser::SerializeStructVariant::serialize_field(
                                    &mut __serde_state,
                                    "controller",
                                    controller,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                match _serde::ser::SerializeStructVariant::serialize_field(
                                    &mut __serde_state,
                                    "value",
                                    value,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                _serde::ser::SerializeStructVariant::end(__serde_state)
                            }
                            MidiEvent::Program {
                                ref channel,
                                ref program,
                            } => {
                                let mut __serde_state =
                                    match _serde::Serializer::serialize_struct_variant(
                                        __serializer,
                                        "MidiEvent",
                                        4u32,
                                        "Program",
                                        0 + 1 + 1,
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                match _serde::ser::SerializeStructVariant::serialize_field(
                                    &mut __serde_state,
                                    "channel",
                                    channel,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                match _serde::ser::SerializeStructVariant::serialize_field(
                                    &mut __serde_state,
                                    "program",
                                    program,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                _serde::ser::SerializeStructVariant::end(__serde_state)
                            }
                            MidiEvent::Pressure {
                                ref channel,
                                ref pressure,
                            } => {
                                let mut __serde_state =
                                    match _serde::Serializer::serialize_struct_variant(
                                        __serializer,
                                        "MidiEvent",
                                        5u32,
                                        "Pressure",
                                        0 + 1 + 1,
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                match _serde::ser::SerializeStructVariant::serialize_field(
                                    &mut __serde_state,
                                    "channel",
                                    channel,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                match _serde::ser::SerializeStructVariant::serialize_field(
                                    &mut __serde_state,
                                    "pressure",
                                    pressure,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                _serde::ser::SerializeStructVariant::end(__serde_state)
                            }
                            MidiEvent::PitchBend {
                                ref channel,
                                ref lsb,
                                ref msb,
                            } => {
                                let mut __serde_state =
                                    match _serde::Serializer::serialize_struct_variant(
                                        __serializer,
                                        "MidiEvent",
                                        6u32,
                                        "PitchBend",
                                        0 + 1 + 1 + 1,
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    };
                                match _serde::ser::SerializeStructVariant::serialize_field(
                                    &mut __serde_state,
                                    "channel",
                                    channel,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                match _serde::ser::SerializeStructVariant::serialize_field(
                                    &mut __serde_state,
                                    "lsb",
                                    lsb,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                match _serde::ser::SerializeStructVariant::serialize_field(
                                    &mut __serde_state,
                                    "msb",
                                    msb,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                                _serde::ser::SerializeStructVariant::end(__serde_state)
                            }
                        }
                    }
                }
            };
        }
        mod sysex_event {}
        pub use meta_event::*;
        pub use midi_event::*;
        pub use sysex_event::*;
        use super::VarLengthValue;
        use crate::error::Error;
        use byteorder::{ReadBytesExt, WriteBytesExt};
        use serde::{Deserialize, Serialize};
        use std::io::{Read, Seek, SeekFrom, Write};
        /// A top-level event struct in a Track, containing a delta time and one of 3 event types
        pub struct Event {
            /// The number of Timing units since the last event
            delta: VarLengthValue,
            event_type: EventType,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for Event {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Event {
                        delta: ref __self_0_0,
                        event_type: ref __self_0_1,
                    } => {
                        let mut debug_trait_builder = f.debug_struct("Event");
                        let _ = debug_trait_builder.field("delta", &&(*__self_0_0));
                        let _ = debug_trait_builder.field("event_type", &&(*__self_0_1));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _IMPL_DESERIALIZE_FOR_Event: () = {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Event {
                fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::export::Formatter,
                        ) -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::export::Ok(__Field::__field0),
                                1u64 => _serde::export::Ok(__Field::__field1),
                                _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"field index 0 <= i < 2",
                                )),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "delta" => _serde::export::Ok(__Field::__field0),
                                "event_type" => _serde::export::Ok(__Field::__field1),
                                _ => _serde::export::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"delta" => _serde::export::Ok(__Field::__field0),
                                b"event_type" => _serde::export::Ok(__Field::__field1),
                                _ => _serde::export::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::export::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::export::PhantomData<Event>,
                        lifetime: _serde::export::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Event;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::export::Formatter,
                        ) -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter, "struct Event")
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::export::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                VarLengthValue,
                            >(&mut __seq)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Event with 2 elements",
                                    ));
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                EventType,
                            >(&mut __seq)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Event with 2 elements",
                                    ));
                                }
                            };
                            _serde::export::Ok(Event {
                                delta: __field0,
                                event_type: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::export::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::export::Option<VarLengthValue> =
                                _serde::export::None;
                            let mut __field1: _serde::export::Option<EventType> =
                                _serde::export::None;
                            while let _serde::export::Some(__key) =
                                match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                }
                            {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::export::Option::is_some(&__field0) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "delta",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<VarLengthValue>(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::export::Option::is_some(&__field1) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "event_type",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<EventType>(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(
                                            &mut __map
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::export::Some(__field0) => __field0,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("delta") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::export::Some(__field1) => __field1,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("event_type") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::export::Ok(Event {
                                delta: __field0,
                                event_type: __field1,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["delta", "event_type"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Event",
                        FIELDS,
                        __Visitor {
                            marker: _serde::export::PhantomData::<Event>,
                            lifetime: _serde::export::PhantomData,
                        },
                    )
                }
            }
        };
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _IMPL_SERIALIZE_FOR_Event: () = {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Event {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::export::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "Event",
                        false as usize + 1 + 1,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "delta",
                        &self.delta,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "event_type",
                        &self.event_type,
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        pub enum EventType {
            Midi(MidiEvent),
            Meta(MetaEvent),
            Unsupported(u8, Vec<u8>),
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for EventType {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match (&*self,) {
                    (&EventType::Midi(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("Midi");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&EventType::Meta(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("Meta");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&EventType::Unsupported(ref __self_0, ref __self_1),) => {
                        let mut debug_trait_builder = f.debug_tuple("Unsupported");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        let _ = debug_trait_builder.field(&&(*__self_1));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _IMPL_DESERIALIZE_FOR_EventType: () = {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for EventType {
                fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::export::Formatter,
                        ) -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter, "variant identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::export::Ok(__Field::__field0),
                                1u64 => _serde::export::Ok(__Field::__field1),
                                2u64 => _serde::export::Ok(__Field::__field2),
                                _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"variant index 0 <= i < 3",
                                )),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "Midi" => _serde::export::Ok(__Field::__field0),
                                "Meta" => _serde::export::Ok(__Field::__field1),
                                "Unsupported" => _serde::export::Ok(__Field::__field2),
                                _ => _serde::export::Err(_serde::de::Error::unknown_variant(
                                    __value, VARIANTS,
                                )),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"Midi" => _serde::export::Ok(__Field::__field0),
                                b"Meta" => _serde::export::Ok(__Field::__field1),
                                b"Unsupported" => _serde::export::Ok(__Field::__field2),
                                _ => {
                                    let __value = &_serde::export::from_utf8_lossy(__value);
                                    _serde::export::Err(_serde::de::Error::unknown_variant(
                                        __value, VARIANTS,
                                    ))
                                }
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::export::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::export::PhantomData<EventType>,
                        lifetime: _serde::export::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = EventType;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::export::Formatter,
                        ) -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter, "enum EventType")
                        }
                        fn visit_enum<__A>(
                            self,
                            __data: __A,
                        ) -> _serde::export::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::EnumAccess<'de>,
                        {
                            match match _serde::de::EnumAccess::variant(__data) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                (__Field::__field0, __variant) => _serde::export::Result::map(
                                    _serde::de::VariantAccess::newtype_variant::<MidiEvent>(
                                        __variant,
                                    ),
                                    EventType::Midi,
                                ),
                                (__Field::__field1, __variant) => _serde::export::Result::map(
                                    _serde::de::VariantAccess::newtype_variant::<MetaEvent>(
                                        __variant,
                                    ),
                                    EventType::Meta,
                                ),
                                (__Field::__field2, __variant) => {
                                    struct __Visitor<'de> {
                                        marker: _serde::export::PhantomData<EventType>,
                                        lifetime: _serde::export::PhantomData<&'de ()>,
                                    }
                                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                        type Value = EventType;
                                        fn expecting(
                                            &self,
                                            __formatter: &mut _serde::export::Formatter,
                                        ) -> _serde::export::fmt::Result
                                        {
                                            _serde::export::Formatter::write_str(
                                                __formatter,
                                                "tuple variant EventType::Unsupported",
                                            )
                                        }
                                        #[inline]
                                        fn visit_seq<__A>(
                                            self,
                                            mut __seq: __A,
                                        ) -> _serde::export::Result<Self::Value, __A::Error>
                                        where
                                            __A: _serde::de::SeqAccess<'de>,
                                        {
                                            let __field0 =
                                                match match _serde::de::SeqAccess::next_element::<u8>(
                                                    &mut __seq,
                                                ) {
                                                    _serde::export::Ok(__val) => __val,
                                                    _serde::export::Err(__err) => {
                                                        return _serde::export::Err(__err);
                                                    }
                                                } {
                                                    _serde::export::Some(__value) => __value,
                                                    _serde::export::None => {
                                                        return _serde :: export :: Err ( _serde :: de :: Error :: invalid_length ( 0usize , & "tuple variant EventType::Unsupported with 2 elements" ) ) ;
                                                    }
                                                };
                                            let __field1 =
                                                match match _serde::de::SeqAccess::next_element::<
                                                    Vec<u8>,
                                                >(
                                                    &mut __seq
                                                ) {
                                                    _serde::export::Ok(__val) => __val,
                                                    _serde::export::Err(__err) => {
                                                        return _serde::export::Err(__err);
                                                    }
                                                } {
                                                    _serde::export::Some(__value) => __value,
                                                    _serde::export::None => {
                                                        return _serde :: export :: Err ( _serde :: de :: Error :: invalid_length ( 1usize , & "tuple variant EventType::Unsupported with 2 elements" ) ) ;
                                                    }
                                                };
                                            _serde::export::Ok(EventType::Unsupported(
                                                __field0, __field1,
                                            ))
                                        }
                                    }
                                    _serde::de::VariantAccess::tuple_variant(
                                        __variant,
                                        2usize,
                                        __Visitor {
                                            marker: _serde::export::PhantomData::<EventType>,
                                            lifetime: _serde::export::PhantomData,
                                        },
                                    )
                                }
                            }
                        }
                    }
                    const VARIANTS: &'static [&'static str] = &["Midi", "Meta", "Unsupported"];
                    _serde::Deserializer::deserialize_enum(
                        __deserializer,
                        "EventType",
                        VARIANTS,
                        __Visitor {
                            marker: _serde::export::PhantomData::<EventType>,
                            lifetime: _serde::export::PhantomData,
                        },
                    )
                }
            }
        };
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _IMPL_SERIALIZE_FOR_EventType: () = {
            #[allow(unknown_lints)]
            #[allow(rust_2018_idioms)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for EventType {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::export::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    match *self {
                        EventType::Midi(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "EventType",
                                0u32,
                                "Midi",
                                __field0,
                            )
                        }
                        EventType::Meta(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "EventType",
                                1u32,
                                "Meta",
                                __field0,
                            )
                        }
                        EventType::Unsupported(ref __field0, ref __field1) => {
                            let mut __serde_state =
                                match _serde::Serializer::serialize_tuple_variant(
                                    __serializer,
                                    "EventType",
                                    2u32,
                                    "Unsupported",
                                    0 + 1 + 1,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                };
                            match _serde::ser::SerializeTupleVariant::serialize_field(
                                &mut __serde_state,
                                __field0,
                            ) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                            match _serde::ser::SerializeTupleVariant::serialize_field(
                                &mut __serde_state,
                                __field1,
                            ) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                            _serde::ser::SerializeTupleVariant::end(__serde_state)
                        }
                    }
                }
            }
        };
    }
    pub use file::*;
    pub use format::*;
    pub use smpte_timecode::*;
    pub use timing::*;
    pub use track::*;
    pub use var_length_value::*;
}
pub mod utils {
    mod bounded {
        pub trait Bounded<T>
        where
            T: Copy + PartialOrd,
        {
            const MIN: T;
            const MAX: T;
            fn bounded(value: T) -> T {
                if value > Self::MAX {
                    Self::MAX
                } else if value < Self::MIN {
                    Self::MIN
                } else {
                    value
                }
            }
            fn is_in_range(value: T) -> bool {
                value <= Self::MAX || value >= Self::MIN
            }
        }
    }
    pub use bounded::*;
}
