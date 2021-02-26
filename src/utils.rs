use std::option::Option;
use std::convert::TryFrom;


#[derive(Debug)]
struct Array64([u8; 64]);

#[derive(Debug)]
struct Array32([u8; 32]);

impl TryFrom<&[u8]> for Array64 {
    type Error = ();

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() == 64 {
            let mut ret: [u8; 64] = [0; 64];
            for i in 0..64 {
                ret[i] = value[i];
            }
            Ok(Array64(ret))
        } else {
            Err(())
        }
    }
}

impl TryFrom<&[u8]> for Array32 {
    type Error = ();

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() == 32 {
            let mut ret: [u8; 32] = [0; 32];
            for i in 0..32 {
                ret[i] = value[i];
            }
            Ok(Array32(ret))
        } else {
            Err(())
        }
    }
}

pub fn to_array_64(s: &[u8]) -> Option<[u8; 64]> {
    /*
    Convert slice to [u8; 64] if it is a valid 64 byte
    slice; otherwise, return None
     */

    let res = Array64::try_from(s);
    if res.is_ok() {
        Option::Some(res.unwrap().0)
    } else {
        Option::None
    }
}

pub fn to_array_32(s: &[u8]) -> Option<[u8; 32]> {
    /*
    Convert slice to [u8; 32] if it is a valid 64 byte
    slice; otherwise, return None
     */

    let res = Array32::try_from(s);
    if res.is_ok() {
        Option::Some(res.unwrap().0)
    } else {
        Option::None
    }
}