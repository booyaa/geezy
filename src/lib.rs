// Copyright 2016 Mark Sta Ana.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0>, at your option.
// This file may not be copied, modified, or distributed except
// according to those terms.

//! A collection of Geodetical functions
//!
//! # Usage
//!
//! This crate is [on crates.io](https://crates.io/crates/geezy) and can be
//! used by adding `wordsworth` to the dependencies in your project's `Cargo.toml`.
//!
//! ```toml
//! [dependencies]
//! geezy = "0.1.*"
//! ```
//!
//! and this to your crate root:
//!
//! ```rust
//! extern crate geezy;
//! ```
//!
//! # Example
//!
//! ```rust
//! use geezy;
//! assert_eq!(true, geezy::Coords::is_valid(-90_f64, -180_f64));
//! ```
//!

#[derive(Debug,PartialEq)]
#[allow(dead_code)]
pub struct Coords {
    latitude: f64,
    longtitude: f64,
}


impl Coords {
    /// Returns a Result<Coords,String> that has been validated
    pub fn new(latitude: f64, longtitude: f64) -> Result<Coords, String> {
        if !Self::is_valid(latitude, longtitude) {
            return Err("Invalid coordinates!".to_string());
        }

        Ok(Coords {
            latitude: latitude,
            longtitude: longtitude,
        })
    }

    // Returns a bool based on the validality of the latitude and longtitude coordinates give.
    pub fn is_valid(latitude: f64, longtitude: f64) -> bool {
        (latitude >= -90f64 && latitude <= 90f64) && (longtitude >= -180f64 && longtitude <= 180f64)
    }
}

#[test]
fn is_valid_coords() {
    let result = Coords::new(-90_f64, -180_f64).unwrap();
    let expected = Coords {
        latitude: -90_f64,
        longtitude: -180_f64,
    };
    assert_eq!(expected, result);

}
#[test]
fn boundary_tests() {
    // FIXME: Expand this, better still quickcheck it
    assert_eq!(true, Coords::is_valid(-90_f64, -180_f64));
    assert_eq!(true, Coords::is_valid(-90_f64, 180_f64));
    assert_eq!(true, Coords::is_valid(90_f64, 180_f64));
    assert_eq!(true, Coords::is_valid(90_f64, -180_f64));


    assert_eq!(false, Coords::is_valid(-91_f64, -180_f64));
    assert_eq!(false, Coords::is_valid(91_f64, 180_f64));

    assert_eq!(false, Coords::is_valid(-90.99, -180f64));
}
