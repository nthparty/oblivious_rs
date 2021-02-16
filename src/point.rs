use crate::utils;
use curve25519_dalek;
use rand_core::OsRng;
use std::option::Option;


#[derive(Debug)]
pub struct Point {
    pub bytes: [u8; 32]
}

impl Point {

    pub fn new(b: &[u8; 32]) -> Point {
        /*
        Return point object corresponding to supplied bytes-like object.
        No checking is performed to confirm that the bytes-like object
        is a valid point.
        */
        Point { bytes: *b }
    }

    pub fn random() -> Point {
        /*
        Return a random point.
         */

        let rp =
            curve25519_dalek::ristretto::RistrettoPoint::random(&mut OsRng).compress();
        Point { bytes: rp.to_bytes() }
    }

    pub fn bytes(bs: &[u8]) -> Option<Point> {
        /*
        Return point obtained by transforming supplied 64 byte hash digest.
         */

        let to_array = utils::to_array_64(bs);
        if to_array.is_some() {
            let rp =
                curve25519_dalek::ristretto::RistrettoPoint::from_uniform_bytes(
                    &to_array.unwrap()
                ).compress();
            Option::Some(
                Point { bytes: rp.to_bytes() }
            )
        } else {
            Option::None
        }
    }

    pub fn hash(bs: &[u8]) -> Point {

        let to_dalek =
            curve25519_dalek::ristretto::RistrettoPoint::hash_from_bytes(bs);
        Point { bytes: to_dalek.compress().to_bytes() }
    }

    pub fn base(s: &[u8; 32]) -> Option<Point> {
        /*
        Return base point multiplied by supplied scalar if the scalar is
        valid; otherwise, return `None`.
         */

        let as_scalar =
            curve25519_dalek::scalar::Scalar::from_canonical_bytes(*s);

        if as_scalar.is_some() {
            let m =
                as_scalar.unwrap() * curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
            Option::Some(
                Point { bytes: m.compress().to_bytes() }
            )
        } else {
            Option::None
        }
    }
}