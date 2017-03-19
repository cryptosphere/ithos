// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DigestAlg {
    SHA256 = 0,
}

impl ::protobuf::ProtobufEnum for DigestAlg {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DigestAlg> {
        match value {
            0 => ::std::option::Option::Some(DigestAlg::SHA256),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DigestAlg] = &[
            DigestAlg::SHA256,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<DigestAlg>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DigestAlg", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DigestAlg {
}

impl ::std::default::Default for DigestAlg {
    fn default() -> Self {
        DigestAlg::SHA256
    }
}

impl ::protobuf::reflect::ProtobufValue for DigestAlg {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EncryptionAlg {
    AES256GCM = 0,
}

impl ::protobuf::ProtobufEnum for EncryptionAlg {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EncryptionAlg> {
        match value {
            0 => ::std::option::Option::Some(EncryptionAlg::AES256GCM),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EncryptionAlg] = &[
            EncryptionAlg::AES256GCM,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<EncryptionAlg>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EncryptionAlg", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EncryptionAlg {
}

impl ::std::default::Default for EncryptionAlg {
    fn default() -> Self {
        EncryptionAlg::AES256GCM
    }
}

impl ::protobuf::reflect::ProtobufValue for EncryptionAlg {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum SignatureAlg {
    Ed25519 = 0,
}

impl ::protobuf::ProtobufEnum for SignatureAlg {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<SignatureAlg> {
        match value {
            0 => ::std::option::Option::Some(SignatureAlg::Ed25519),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [SignatureAlg] = &[
            SignatureAlg::Ed25519,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<SignatureAlg>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("SignatureAlg", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for SignatureAlg {
}

impl ::std::default::Default for SignatureAlg {
    fn default() -> Self {
        SignatureAlg::Ed25519
    }
}

impl ::protobuf::reflect::ProtobufValue for SignatureAlg {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum PasswordAlg {
    SCRYPT = 0,
}

impl ::protobuf::ProtobufEnum for PasswordAlg {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<PasswordAlg> {
        match value {
            0 => ::std::option::Option::Some(PasswordAlg::SCRYPT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [PasswordAlg] = &[
            PasswordAlg::SCRYPT,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<PasswordAlg>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("PasswordAlg", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for PasswordAlg {
}

impl ::std::default::Default for PasswordAlg {
    fn default() -> Self {
        PasswordAlg::SCRYPT
    }
}

impl ::protobuf::reflect::ProtobufValue for PasswordAlg {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CipherSuite {
    Ed25519_AES256GCM_SHA256 = 0,
}

impl ::protobuf::ProtobufEnum for CipherSuite {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CipherSuite> {
        match value {
            0 => ::std::option::Option::Some(CipherSuite::Ed25519_AES256GCM_SHA256),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CipherSuite] = &[
            CipherSuite::Ed25519_AES256GCM_SHA256,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<CipherSuite>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CipherSuite", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CipherSuite {
}

impl ::std::default::Default for CipherSuite {
    fn default() -> Self {
        CipherSuite::Ed25519_AES256GCM_SHA256
    }
}

impl ::protobuf::reflect::ProtobufValue for CipherSuite {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x09, 0x61, 0x6c, 0x67, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x05, 0x69, 0x74, 0x68,
    0x6f, 0x73, 0x2a, 0x17, 0x0a, 0x09, 0x44, 0x69, 0x67, 0x65, 0x73, 0x74, 0x41, 0x6c, 0x67, 0x12,
    0x0a, 0x0a, 0x06, 0x53, 0x48, 0x41, 0x32, 0x35, 0x36, 0x10, 0x00, 0x2a, 0x1e, 0x0a, 0x0d, 0x45,
    0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x41, 0x6c, 0x67, 0x12, 0x0d, 0x0a, 0x09,
    0x41, 0x45, 0x53, 0x32, 0x35, 0x36, 0x47, 0x43, 0x4d, 0x10, 0x00, 0x2a, 0x1b, 0x0a, 0x0c, 0x53,
    0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x41, 0x6c, 0x67, 0x12, 0x0b, 0x0a, 0x07, 0x45,
    0x64, 0x32, 0x35, 0x35, 0x31, 0x39, 0x10, 0x00, 0x2a, 0x19, 0x0a, 0x0b, 0x50, 0x61, 0x73, 0x73,
    0x77, 0x6f, 0x72, 0x64, 0x41, 0x6c, 0x67, 0x12, 0x0a, 0x0a, 0x06, 0x53, 0x43, 0x52, 0x59, 0x50,
    0x54, 0x10, 0x00, 0x2a, 0x2b, 0x0a, 0x0b, 0x43, 0x69, 0x70, 0x68, 0x65, 0x72, 0x53, 0x75, 0x69,
    0x74, 0x65, 0x12, 0x1c, 0x0a, 0x18, 0x45, 0x64, 0x32, 0x35, 0x35, 0x31, 0x39, 0x5f, 0x41, 0x45,
    0x53, 0x32, 0x35, 0x36, 0x47, 0x43, 0x4d, 0x5f, 0x53, 0x48, 0x41, 0x32, 0x35, 0x36, 0x10, 0x00,
    0x4a, 0xe1, 0x02, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x16, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c,
    0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x08, 0x0d, 0x0a,
    0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x04, 0x00, 0x06, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05,
    0x00, 0x01, 0x12, 0x03, 0x04, 0x05, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x05, 0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x05,
    0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x05, 0x0d, 0x0e,
    0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x01, 0x12, 0x04, 0x08, 0x00, 0x0a, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x05, 0x01, 0x01, 0x12, 0x03, 0x08, 0x05, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x00,
    0x12, 0x03, 0x09, 0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x09, 0x04, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x02, 0x12, 0x03, 0x09, 0x10,
    0x11, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x02, 0x12, 0x04, 0x0c, 0x00, 0x0e, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x05, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x05, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02,
    0x00, 0x12, 0x03, 0x0d, 0x04, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x0d, 0x04, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0d,
    0x0e, 0x0f, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x03, 0x12, 0x04, 0x10, 0x00, 0x12, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x05, 0x03, 0x01, 0x12, 0x03, 0x10, 0x05, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x03,
    0x02, 0x00, 0x12, 0x03, 0x11, 0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x11, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x00, 0x02, 0x12, 0x03,
    0x11, 0x0d, 0x0e, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x04, 0x12, 0x04, 0x14, 0x00, 0x16, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x05, 0x04, 0x01, 0x12, 0x03, 0x14, 0x05, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x04, 0x02, 0x00, 0x12, 0x03, 0x15, 0x04, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x15, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x00, 0x02, 0x12,
    0x03, 0x15, 0x1f, 0x20, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
