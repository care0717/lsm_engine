use anyhow::Result;
use std::convert::TryInto;
use std::mem::size_of;

#[derive(Clone)]
pub struct Value {
    data: String,
    flags: usize,
    exptime: usize,
}
impl Value {
    pub fn new(data: String, flags: usize, exptime: usize) -> Self {
        Self {
            data,
            flags,
            exptime,
        }
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut value_bytes: Vec<u8> = Vec::new();
        value_bytes.extend(self.flags.to_le_bytes().to_vec());
        value_bytes.extend(self.exptime.to_le_bytes().to_vec());
        let data_bytes = self.data.clone().into_bytes();
        value_bytes.extend(&data_bytes);
        value_bytes.extend(&(data_bytes.len() as i32).to_le_bytes());
        value_bytes
    }
    pub fn from_bytes(vec: Vec<u8>) -> Result<Self> {
        let mut index = vec.len();
        index -= size_of::<i32>();
        let data_len =
            i32::from_le_bytes(vec[index..(index + size_of::<i32>())].try_into()?) as usize;
        index -= data_len;
        let data = String::from_utf8(vec[index..(index + data_len)].to_vec())?;

        index -= size_of::<usize>();
        let exptime = usize::from_le_bytes(vec[index..(index + size_of::<usize>())].try_into()?);

        index -= size_of::<usize>();
        let flags = usize::from_le_bytes(vec[index..(index + size_of::<usize>())].try_into()?);

        Ok(Self::new(data, flags, exptime))
    }

    pub fn to_string(&self, key: String) -> String {
        format!(
            "VALUE {} {} {} {}\n{}\n",
            key,
            self.flags,
            self.exptime,
            self.data.len(),
            self.data
        )
    }
}
