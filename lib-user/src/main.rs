use vstd::prelude::*;

verus! {

fn main() {
    assert(lib::is_even(2));
    assert(!lib::is_even(3));
}

}
