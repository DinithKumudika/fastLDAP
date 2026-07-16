use super::types::*;
use bytes::{BufMut, BytesMut};

pub fn encode_tag(buf: &mut BytesMut, tag: Tag) {
    let mut first_byte = (tag.class as u8) | (tag.form as u8);
    if tag.value < 31 {
        first_byte |= tag.value as u8;
        buf.put_u8(first_byte);
    } else {
        first_byte |= 0x1F;
        buf.put_u8(first_byte);
        
        let mut val = tag.value;
        let mut bytes = vec![];
        bytes.push((val & 0x7F) as u8);
        val >>= 7;
        while val > 0 {
            bytes.push(((val & 0x7F) | 0x80) as u8);
            val >>= 7;
        }
        for b in bytes.into_iter().rev() {
            buf.put_u8(b);
        }
    }
}

pub fn encode_length(buf: &mut BytesMut, len: usize) {
    if len < 128 {
        buf.put_u8(len as u8);
    } else {
        let mut len_bytes = vec![];
        let mut l = len;
        while l > 0 {
            len_bytes.push((l & 0xFF) as u8);
            l >>= 8;
        }
        buf.put_u8(0x80 | (len_bytes.len() as u8));
        for b in len_bytes.into_iter().rev() {
            buf.put_u8(b);
        }
    }
}

pub fn encode_tlv(buf: &mut BytesMut, tag: Tag, value: &[u8]) {
    encode_tag(buf, tag);
    encode_length(buf, value.len());
    buf.put_slice(value);
}

pub fn encode_integer(buf: &mut BytesMut, tag: Tag, value: i64) {
    let bytes = value.to_be_bytes();
    let mut start = 0;
    if value >= 0 {
        while start < 7 && bytes[start] == 0 {
            start += 1;
        }
        if bytes[start] & 0x80 != 0 {
            let mut nb = vec![0];
            nb.extend_from_slice(&bytes[start..]);
            encode_tlv(buf, tag, &nb);
            return;
        }
    } else {
        while start < 7 && bytes[start] == 0xFF && (bytes[start+1] & 0x80) != 0 {
            start += 1;
        }
    }
    encode_tlv(buf, tag, &bytes[start..]);
}

pub fn encode_boolean(buf: &mut BytesMut, tag: Tag, value: bool) {
    encode_tlv(buf, tag, if value { &[0xFF] } else { &[0x00] });
}
