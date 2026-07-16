#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Class {
    Universal = 0x00,
    Application = 0x40,
    ContextSpecific = 0x80,
    Private = 0xC0,
}

impl From<u8> for Class {
    fn from(val: u8) -> Self {
        match val & 0xC0 {
            0x00 => Class::Universal,
            0x40 => Class::Application,
            0x80 => Class::ContextSpecific,
            0xC0 => Class::Private,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Form {
    Primitive = 0x00,
    Constructed = 0x20,
}

impl From<u8> for Form {
    fn from(val: u8) -> Self {
        match val & 0x20 {
            0x00 => Form::Primitive,
            0x20 => Form::Constructed,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tag {
    pub class: Class,
    pub form: Form,
    pub value: u32,
}

impl Tag {
    pub fn new(class: Class, form: Form, value: u32) -> Self {
        Self { class, form, value }
    }
}

// Common Universal Tags
pub const TAG_BOOLEAN: u32 = 1;
pub const TAG_INTEGER: u32 = 2;
pub const TAG_OCTET_STRING: u32 = 4;
pub const TAG_NULL: u32 = 5;
pub const TAG_ENUMERATED: u32 = 10;
pub const TAG_SEQUENCE: u32 = 16;
pub const TAG_SET: u32 = 17;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Length {
    Definite(usize),
    Indefinite,
}

#[derive(Debug, Clone)]
pub struct Tlv<'a> {
    pub tag: Tag,
    pub length: Length,
    pub value: &'a [u8], // Will be empty for Indefinite length when parsing
}
