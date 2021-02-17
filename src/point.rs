use crate::utils;
use crate::scalar::Scalar;
use curve25519_dalek;
use rand_core::OsRng;
use sha2::Sha512;
use std::option::Option;


pub struct Point {
    pub bytes: [u8; 32],
    pub dalek: curve25519_dalek::ristretto::RistrettoPoint
}

impl Point {

    pub fn random() -> Point {
        /*
        Return a random point.
         */

        let rp =
            curve25519_dalek::ristretto::RistrettoPoint::random(&mut OsRng);
        let bs = &rp.compress().to_bytes();
        Point {
            bytes: *bs,
            dalek: rp
        }
    }

    pub fn from_bytes(bs: &[u8]) -> Option<Point> {
        /*
        Return point obtained by transforming supplied 64 byte hash digest.
         */

        let to_array = utils::to_array_64(bs);
        if to_array.is_some() {
            let rp =
                curve25519_dalek::ristretto::RistrettoPoint::from_uniform_bytes(
                    &to_array.unwrap()
                );
            let bs = &rp.compress().to_bytes();
            Option::Some(
                Point {
                    bytes: *bs,
                    dalek: rp
                }
            )
        } else {
            Option::None
        }
    }

    pub fn hash(bs: &[u8]) -> Point {
        /*
        Return point object by hashing supplied slice of bytes.
         */

        let to_dalek =
            curve25519_dalek::ristretto::RistrettoPoint::hash_from_bytes::<Sha512>(bs);
        let bs = &to_dalek.compress().to_bytes();
        Point {
            bytes: *bs,
            dalek: to_dalek
        }
    }

    pub fn base(s: &Scalar) -> Point {
        /*
        Return base point multiplied by supplied scalar if the scalar is
        valid; otherwise, return `None`.
         */

        let m = s.dalek * curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
        let bs = &m.compress().to_bytes();
        Point {
            bytes: *bs,
            dalek: m
        }
    }
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Point: {:?}", self.bytes)
    }
}

// Point + Point
impl std::ops::Add<Point> for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {

        let a = self.dalek + other.dalek;
        let bs = &a.compress().to_bytes();
        Point {
            bytes: *bs,
            dalek: a
        }
    }
}

// Point + &Point
impl <'b> std::ops::Add<&'b Point> for Point {
    type Output = Point;

    fn add(self, other: &'b Point) -> Point {

        let out_p = &self.dalek + other.dalek;
        let out_bs = &out_p.compress().to_bytes();
        Point {
            bytes: *out_bs,
            dalek: out_p
        }
    }
}

// &Point + Point
impl <'a> std::ops::Add<Point> for &'a Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {

        let out_p = self.dalek + other.dalek;
        let out_bs = &out_p.compress().to_bytes();
        Point {
            bytes: *out_bs,
            dalek: out_p
        }
    }
}

// &Point + &Point
impl <'a, 'b> std::ops::Add<&'b Point> for &'a Point {
    type Output = Point;

    fn add(self, other: &'b Point) -> Point {

        let out_p = self.dalek + other.dalek;
        let out_bs = &out_p.compress().to_bytes();
        Point {
            bytes: *out_bs,
            dalek: out_p
        }
    }
}

// Point - Point
impl std::ops::Sub<Point> for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {

        let a = self.dalek - other.dalek;
        let bs = &a.compress().to_bytes();

        Point {
            bytes: *bs,
            dalek: a
        }
    }
}

// Point - &Point
impl <'b> std::ops::Sub<&'b Point> for Point {
    type Output = Point;

    fn sub(self, other: &'b Point) -> Point {

        let out_p = &self.dalek - other.dalek;
        let out_bs = &out_p.compress().to_bytes();

        Point {
            bytes: *out_bs,
            dalek: out_p
        }
    }
}

// &Point - Point
impl <'a> std::ops::Sub<Point> for &'a Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {

        let out_p = self.dalek - other.dalek;
        let out_bs = &out_p.compress().to_bytes();

        Point {
            bytes: *out_bs,
            dalek: out_p
        }
    }
}

// &Point - &Point
impl <'a, 'b> std::ops::Sub<&'b Point> for &'a Point {
    type Output = Point;

    fn sub(self, other: &'b Point) -> Point {

        let out_p = self.dalek - other.dalek;
        let out_bs = &out_p.compress().to_bytes();
        Point {
            bytes: *out_bs,
            dalek: out_p
        }
    }
}