use vstd::prelude::*;

verus! {

fn main() {
    assert(foo::is_even(2));
    assert(!foo::is_even(3));
}

}
