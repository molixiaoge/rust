// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// xfail-test

trait X {
    fn call(&self);
}

struct Y;

struct Z<T> {
    x: T
}

impl X for Y {
    fn call(&self) {
    }
}

impl<T: X> Drop for Z<T> {
    fn drop(&mut self) {
        self.x.call(); // Adding this statement causes an ICE.
    }
}

pub fn main() {
    let y = Y;
    let _z = Z{x: y};
}
