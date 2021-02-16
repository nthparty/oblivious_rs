use curve25519_dalek;
use rand_core::OsRng;


#[derive(Debug)]
pub struct Scalar {
    pub bytes: [u8; 32]
}

impl Scalar {

    pub fn new(b: &[u8; 32]) -> Scalar {
        /*
        Return scalar object corresponding to supplied bytes-like object.
        No checking is performed to confirm that the bytes-like object
        is a valid scalar.
        */
        Scalar { bytes: *b }
    }

    pub fn random() -> Scalar {
        /*
        Return random non-zero scalar.
         */
        Scalar {
            bytes: curve25519_dalek::scalar::Scalar::random(&mut OsRng).to_bytes()
        }
    }

    pub fn bytes(bs: &[u8; 32]) -> Option<Scalar>{
        /*
        Return scalar object obtained by transforming supplied bytes-like
        object if it is possible to do; otherwise, return `None`.
         */

        let to_dalek =
            curve25519_dalek::scalar::Scalar::from_canonical_bytes(*bs);

        if to_dalek.is_some() {
            Option::Some( Scalar { bytes: to_dalek.unwrap().to_bytes() })
        } else {
            Option::None
        }
    }

    pub fn hash(bs: &[u8]) -> Scalar {
        /*
        Return scalar object by hashing supplied bytes-like object.
        */

        let to_dalek =
            curve25519_dalek::scalar::Scalar::hash_from_bytes(bs);
        Scalar { bytes: to_dalek.to_bytes() }
    }

    pub fn inverse(&self) -> Option<Scalar> {
        /*
        Return inverse if bytes of this scalar represent a valid scalar;
        otherwise, return None.
         */

        let to_dalek =
            curve25519_dalek::scalar::Scalar::from_canonical_bytes(self.bytes);

        if to_dalek.is_some() {
            let inv = to_dalek.unwrap().invert();
            Option::Some( Scalar { bytes: inv.to_bytes() })
        } else {
            Option::None
        }
    }
}
