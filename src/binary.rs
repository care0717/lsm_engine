use crate::value::Value;
use anyhow::Result;
use std::convert::TryInto;
use std::mem::size_of;

const TOMBSTONE: i32 = -1;

pub fn encode(key: &String, value: Option<&Value>) -> Vec<u8> {
    let mut record = Vec::new();
    if let Some(v) = value {
        record.extend(v.as_bytes());
        let len = v.as_bytes().len() as i32;
        record.extend(len.to_le_bytes().to_vec());
    } else {
        record.extend(TOMBSTONE.to_le_bytes().to_vec());
    }
    record.extend(key.as_bytes());
    record.extend(&(key.len() as i16).to_le_bytes());
    record
}
pub fn decode(vec: Vec<u8>) -> Result<(String, Option<Value>)> {
    let mut index = vec.len();
    index -= size_of::<i16>();
    let key_len = i16::from_le_bytes(vec[index..(index + size_of::<i16>())].try_into()?) as usize;
    index -= key_len;
    let key = String::from_utf8(vec[index..(index + key_len)].to_vec())?;

    index -= size_of::<i32>();
    let value_len = i32::from_le_bytes(vec[index..(index + size_of::<i32>())].try_into()?);
    if value_len >= 0 {
        index -= value_len as usize;
        let value = Value::from_bytes(vec[index..(index + value_len as usize)].to_vec())?;
        Ok((key, Option::from(value)))
    } else {
        Ok((key, None))
    }
}
