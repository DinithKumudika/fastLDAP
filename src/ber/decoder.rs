use nom::{
    bytes::streaming::take,
    number::streaming::be_u8,
    IResult,
};
use crate::utils::error::BerError;
use super::types::*;

pub struct BerDecoder<'a> {
    pub input: &'a [u8],
}

impl<'a> BerDecoder<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Self { input }
    }

    pub fn is_empty(&self) -> bool {
        self.input.is_empty()
    }
}

pub fn parse_tag(i: &[u8]) -> IResult<&[u8], Tag> {
    let (mut i, byte) = be_u8(i)?;
    let class = Class::from(byte);
    let form = Form::from(byte);
    let mut value = (byte & 0x1F) as u32;

    if value == 0x1F {
        value = 0;
        loop {
            let (next_i, b) = be_u8(i)?;
            i = next_i;
            value = (value << 7) | ((b & 0x7F) as u32);
            if (b & 0x80) == 0 {
                break;
            }
        }
    }

    Ok((i, Tag::new(class, form, value)))
}

pub fn parse_length(i: &[u8]) -> IResult<&[u8], Length> {
    let (i, byte) = be_u8(i)?;
    if byte == 0x80 {
        return Ok((i, Length::Indefinite));
    }
    if byte < 0x80 {
        return Ok((i, Length::Definite(byte as usize)));
    }
    let num_bytes = byte & 0x7F;
    if num_bytes > 4 {
        return Err(nom::Err::Error(nom::error::Error::new(i, nom::error::ErrorKind::TooLarge)));
    }
    let (i, len_bytes) = take(num_bytes)(i)?;
    let mut len = 0usize;
    for &b in len_bytes {
        len = (len << 8) | (b as usize);
    }
    Ok((i, Length::Definite(len)))
}

pub fn parse_tlv(i: &[u8]) -> IResult<&[u8], Tlv> {
    let (i, tag) = parse_tag(i)?;
    let (i, length) = parse_length(i)?;
    match length {
        Length::Definite(len) => {
            let (i, value) = take(len)(i)?;
            Ok((i, Tlv { tag, length, value }))
        }
        Length::Indefinite => {
            Ok((i, Tlv { tag, length, value: &[] }))
        }
    }
}

pub fn decode_integer(value: &[u8]) -> Result<i64, BerError> {
    if value.is_empty() {
        return Err(BerError::InvalidLength);
    }
    if value.len() > 8 {
        return Err(BerError::UnsupportedLength);
    }
    let mut res = if (value[0] & 0x80) != 0 { -1i64 } else { 0i64 };
    for &b in value {
        res = (res << 8) | (b as i64);
    }
    Ok(res)
}

pub fn decode_boolean(value: &[u8]) -> Result<bool, BerError> {
    if value.len() != 1 {
        return Err(BerError::InvalidLength);
    }
    Ok(value[0] != 0)
}

pub fn decode_string(value: &[u8]) -> Result<String, BerError> {
    String::from_utf8(value.to_vec()).map_err(|e| e.into())
}
