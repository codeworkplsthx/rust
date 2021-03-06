// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// run-pass
#![allow(unused_mut)] // under NLL we get warning about `bar` below
fn baz() -> ! { panic!(); }

fn foo() {
    match Some::<isize>(5) {
      Some::<isize>(_x) => {
        let mut bar;
        match None::<isize> { None::<isize> => { bar = 5; } _ => { baz(); } }
        println!("{}", bar);
      }
      None::<isize> => { println!("hello"); }
    }
}

pub fn main() { foo(); }
