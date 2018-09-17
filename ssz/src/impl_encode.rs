extern crate bytes;

use super::{
    Encodable,
    SszStream
};
use super::ethereum_types::{ H256, U256 };
use self::bytes::{ BytesMut, BufMut };

/*
 * Note: there is a "to_bytes" function for integers
 * in Rust nightly. When it is in stable, we should
 * use it instead.
 */
macro_rules! impl_encodable_for_uint {
    ($type: ident, $bit_size: expr) => {
        impl Encodable for $type {
            fn ssz_append(&self, s: &mut SszStream)
            {
                // Ensure bit size is valid
                assert!((0 < $bit_size) &&
                        ($bit_size % 8 == 0) &&
                        (2_u128.pow($bit_size) > *self as u128));

                // Serialize to bytes
                let mut buf = BytesMut::with_capacity($bit_size/8);

                // Match bit size with encoding
                match $bit_size {
                    8 => buf.put_u8(*self as u8),
                    16 => buf.put_u16_be(*self as u16),
                    32 => buf.put_u32_be(*self as u32),
                    64 => buf.put_u64_be(*self as u64),
                    _ => { ; }
                }

                // Append bytes to the SszStream
                s.append_encoded_raw(&mut buf.to_vec());
            }
        }
    }
}

impl_encodable_for_uint!(u8, 8);
impl_encodable_for_uint!(u16, 16);
impl_encodable_for_uint!(u32, 32);
impl_encodable_for_uint!(u64, 64);
impl_encodable_for_uint!(usize, 64);

impl Encodable for H256 {
    fn ssz_append(&self, s: &mut SszStream) {
        s.append_encoded_val(&self.to_vec());
    }
}

impl Encodable for U256 {
    fn ssz_append(&self, s: &mut SszStream) {
        let mut a = [0; 32];
        self.to_big_endian(&mut a);
        s.append_encoded_val(&a.to_vec());
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ssz_encode_u8() {
        let x: u8 = 0;
        let mut ssz = SszStream::new();
        ssz.append(&x);
        assert_eq!(ssz.drain(), vec![0]);

        let x: u8 = 1;
        let mut ssz = SszStream::new();
        ssz.append(&x);
        assert_eq!(ssz.drain(), vec![1]);

        let x: u8 = 100;
        let mut ssz = SszStream::new();
        ssz.append(&x);
        assert_eq!(ssz.drain(), vec![100]);

        let x: u8 = 255;
        let mut ssz = SszStream::new();
        ssz.append(&x);
        assert_eq!(ssz.drain(), vec![255]);
    }

    #[test]
    fn test_ssz_encode_u16() {
        let x: u16 = 1;
        let mut ssz = SszStream::new();
        ssz.append(&x);
        assert_eq!(ssz.drain(), vec![0, 1]);

        let x: u16 = 100;
        let mut ssz = SszStream::new();
        ssz.append(&x);
        assert_eq!(ssz.drain(), vec![0, 100]);

        let x: u16 = 1 << 8;
        let mut ssz = SszStream::new();
        ssz.append(&x);
        assert_eq!(ssz.drain(), vec![1, 0]);

        let x: u16 = 65535;
        let mut ssz = SszStream::new();
        ssz.append(&x);
        assert_eq!(ssz.drain(), vec![255, 255]);
    }

    #[test]
    fn test_ssz_encode_u32() {
        let x: u32 = 1;
        let mut ssz = SszStream::new();
        ssz.append(&x);
        assert_eq!(ssz.drain(), vec![0, 0, 0, 1]);

        let x: u32 = 100;
        let mut ssz = SszStream::new();
        ssz.append(&x);
        assert_eq!(ssz.drain(), vec![0, 0, 0, 100]);

        let x: u32 = 1 << 16;
        let mut ssz = SszStream::new();
        ssz.append(&x);
        assert_eq!(ssz.drain(), vec![0, 1, 0, 0]);

        let x: u32 = 1 << 24;
        let mut ssz = SszStream::new();
        ssz.append(&x);
        assert_eq!(ssz.drain(), vec![1, 0, 0, 0]);

        let x: u32 = !0;
        let mut ssz = SszStream::new();
        ssz.append(&x);
        assert_eq!(ssz.drain(), vec![255, 255, 255, 255]);
    }

    #[test]
    fn test_ssz_encode_u64() {
        let x: u64 = 1;
        let mut ssz = SszStream::new();
        ssz.append(&x);
        assert_eq!(ssz.drain(), vec![0, 0, 0, 0, 0, 0, 0, 1]);

        let x: u64 = 100;
        let mut ssz = SszStream::new();
        ssz.append(&x);
        assert_eq!(ssz.drain(), vec![0, 0, 0, 0, 0, 0, 0, 100]);

        let x: u64 = 1 << 32;
        let mut ssz = SszStream::new();
        ssz.append(&x);
        assert_eq!(ssz.drain(), vec![0, 0, 0, 1, 0, 0, 0, 0]);

        let x: u64 = !0;
        let mut ssz = SszStream::new();
        ssz.append(&x);
        assert_eq!(ssz.drain(), vec![255, 255, 255, 255, 255, 255, 255, 255]);
    }

    #[test]
    fn test_ssz_encode_usize() {
        let x: usize = 1;
        let mut ssz = SszStream::new();
        ssz.append(&x);
        assert_eq!(ssz.drain(), vec![0, 0, 0, 0, 0, 0, 0, 1]);

        let x: usize = 100;
        let mut ssz = SszStream::new();
        ssz.append(&x);
        assert_eq!(ssz.drain(), vec![0, 0, 0, 0, 0, 0, 0, 100]);

        let x: usize = 1 << 32;
        let mut ssz = SszStream::new();
        ssz.append(&x);
        assert_eq!(ssz.drain(), vec![0, 0, 0, 1, 0, 0, 0, 0]);

        let x: usize = !0;
        let mut ssz = SszStream::new();
        ssz.append(&x);
        assert_eq!(ssz.drain(), vec![255, 255, 255, 255, 255, 255, 255, 255]);
    }
}
