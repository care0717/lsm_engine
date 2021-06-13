use std::mem::size_of;
use std::convert::TryInto;
use anyhow::Result;

#[derive(Clone)]
pub struct Value {
    data: String,
    flags: usize,
    exptime: usize,
}

impl Value {
    pub fn new(data: String, flags: usize, exptime: usize) -> Self {
        Value {
            data,
            flags,
            exptime,
        }
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        [
            self.data.clone().into_bytes(),
            self.flags.to_le_bytes().to_vec(),
            self.exptime.to_le_bytes().to_vec(),
        ]
        .iter()
        .fold(Vec::<u8>::new(), |mut acc, x| {
            acc.extend(x);
            acc.extend(&(x.len() as i32).to_le_bytes());
            acc
        })
        .to_vec()
    }
    pub fn from_bytes(vec: Vec<u8>) -> Result<Self>  {
        let mut index = vec.len();
        index -= size_of::<i32>();
        let exptime_len = i32::from_le_bytes(vec[index..(index + size_of::<i32>())].try_into()?) as usize;
        index -= exptime_len;
        let exptime = usize::from_le_bytes(vec[index..(index + exptime_len)].try_into()?);
        index -= size_of::<i32>();
        let flags_len = i32::from_le_bytes(vec[index..(index + size_of::<i32>())].try_into()?) as usize;
        index -= flags_len;
        let flags = usize::from_le_bytes(vec[index..(index + flags_len)].try_into()?);
        index -= size_of::<i32>();
        let data_len = i32::from_le_bytes(vec[index..(index + size_of::<i32>())].try_into()?) as usize;
        index -= data_len;
        let data = String::from_utf8(vec[index..(index + data_len)].to_vec())?;
        Ok(Self::new(data, flags, exptime))
    }

    pub fn format(&self, key: String) -> String {
        format!("VALUE {} {} {} {}\n{}\n", key, self.flags, self.exptime, self.data.len(), self.data)
    }
}
