use vstd::prelude::*;


verus! {

pub fn is_even(v: u64) -> (b: bool)
    ensures b <==> v % 2 == 0
{
    v % 2 == 0
}

}
