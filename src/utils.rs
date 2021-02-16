use std::option::Option;
use std::convert::TryFrom;


#[derive(Debug)]
struct Array64([u8; 64]);

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