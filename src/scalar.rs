use curve25519_dalek;
use rand_core::OsRng;
use sha2::Sha512;
use crate::point::Point;


pub struct Scalar {
    pub bytes: [u8; 32],
    pub dalek: curve25519_dalek::scalar::Scalar
}

impl Scalar {

    pub fn random() -> Scalar {
        /*
        Return random non-zero scalar.
         */

        let rs = curve25519_dalek::scalar::Scalar::random(&mut OsRng);
        Scalar {
            bytes: *rs.as_bytes(),
            dalek: rs
        }
    }

    pub fn from_bytes(bs: &[u8; 32]) -> Option<Scalar>{
        /*
        Return scalar object obtained by transforming supplied bytes-like
        object if it is possible to do; otherwise, return `None`.
         */

        let to_dalek =
            curve25519_dalek::scalar::Scalar::from_canonical_bytes(*bs);

        if to_dalek.is_some() {
            let uw = to_dalek.unwrap();
            Option::Some( Scalar { bytes: *uw.as_bytes(), dalek: uw} )
        } else {
            Option::None
        }
    }

    pub fn hash(bs: &[u8]) -> Scalar {
        /*
        Return scalar object by hashing supplied bytes-like object.
        */

        let to_dalek =
            curve25519_dalek::scalar::Scalar::hash_from_bytes::<Sha512>(bs);
        Scalar {
            bytes: *to_dalek.as_bytes(),
            dalek: to_dalek
        }
    }

    pub fn inverse(&self) -> Scalar{
        /*
        Return inverse if bytes of this scalar represent a valid scalar;
        otherwise, return None.
         */

        let inv = self.dalek.invert();
        Scalar {
            bytes: *inv.as_bytes(),
            dalek: inv
        }

    }
}

impl std::fmt::Debug for Scalar {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Scalar: {:?}", self.bytes)
    }
}

// Scalar * Scalar
impl std::ops::Mul<Scalar> for Scalar {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {

        let out_s = self.dalek * rhs.dalek;
        Scalar {
            bytes: *out_s.as_bytes(),
            dalek: out_s
        }
    }
}

// Scalar * &Scalar
impl <'b> std::ops::Mul<&'b Scalar> for Scalar {
    type Output = Scalar;

    fn mul(self, rhs: &'b Scalar) -> Scalar {

        let out_s = &self.dalek * rhs.dalek;
        Scalar {
            bytes: *out_s.as_bytes(),
            dalek: out_s
        }
    }
}

// &Scalar * Scalar
impl <'a> std::ops::Mul<Scalar> for &'a Scalar {
    type Output = Scalar;

    fn mul(self, rhs: Scalar) -> Scalar {

        let out_s = self.dalek * rhs.dalek;
        Scalar {
            bytes: *out_s.as_bytes(),
            dalek: out_s
        }
    }
}

// &Scalar * &Scalar
impl <'a, 'b> std::ops::Mul<&'b Scalar> for &'a Scalar {
    type Output = Scalar;

    fn mul(self, rhs: &'b Scalar) -> Scalar {

        let out_s = self.dalek * rhs.dalek;
        Scalar {
            bytes: *out_s.as_bytes(),
            dalek: out_s
        }
    }
}

// Scalar * Point
impl std::ops::Mul<Point> for Scalar {
    type Output = Point;

    fn mul(self, rhs: Point) -> Point {

        let m = self.dalek * rhs.dalek;
        let bs = &m.compress().to_bytes();
        Point {
            bytes: *bs,
            dalek: m
        }
    }
}

// Scalar * &Point
impl <'b> std::ops::Mul<&'b Point> for Scalar {
    type Output = Point;

    fn mul(self, rhs: &'b Point) -> Point {

        let m = &self.dalek * rhs.dalek;
        let bs = &m.compress().to_bytes();
        Point {
            bytes: *bs,
            dalek: m
        }
    }
}

// &Scalar * Point
impl <'a> std::ops::Mul<Point> for &'a Scalar {
    type Output = Point;

    fn mul(self, rhs: Point) -> Point {

        let m = self.dalek * rhs.dalek;
        let bs = &m.compress().to_bytes();
        Point {
            bytes: *bs,
            dalek: m
        }
    }
}

// &Scalar * &Point
impl <'a, 'b> std::ops::Mul<&'b Point> for &'a Scalar {
    type Output = Point;

    fn mul(self, rhs: &'b Point) -> Point {

        let m = self.dalek * rhs.dalek;
        let bs = &m.compress().to_bytes();
        Point {
            bytes: *bs,
            dalek: m
        }
    }
}


