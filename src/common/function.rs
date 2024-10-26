use bytes::{BytesMut, BufMut};

// funtion to conver from vec to BytesMut
pub fn vec_to_bytes_mut(vec: Vec<u8>) -> BytesMut {
    let sliced_data = &vec[0..];
    let mut bytes_mut = BytesMut::with_capacity(sliced_data.len());
    bytes_mut.extend_from_slice(sliced_data);
    bytes_mut
}