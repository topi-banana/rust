// Check that a struct update expression inside a closure body consumes
// exactly the fields of the base expression that are not mentioned
// explicitly.
//
// The closure moves the fields that are not mentioned explicitly, so it has to
// capture them by move; if it consumed fewer fields than that, the update
// would move out of a shared reference and fail to compile. Conversely, `a` is
// mentioned explicitly, so it must not be consumed and stays usable
// afterwards.

//@ run-pass
//@ edition: 2021

#![allow(dead_code)]

struct S {
    a: String,
    b: String,
    c: String,
}

fn main() {
    let s = S { a: String::new(), b: String::new(), c: String::new() };

    let f = || S { a: String::new(), ..s };
    let _ = f();

    // `s.a` was not moved into the closure, so it is still usable here.
    let _a = s.a;

    let t = S { a: String::new(), b: String::new(), c: String::new() };

    let g = || S { a: String::new(), b: String::new(), ..t };
    let _ = g();

    let _ta = t.a;
    let _tb = t.b;
}
