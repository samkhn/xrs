// wheres
//
// Instead of:
//  impl <A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}
// do:
//  impl <A, D> MyTrait<A, D> for YourType where
//      A: TraitB + TraitC,
//      D: TraitE + TraitF {}
//
// another use of where:
use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}

impl<T> PrintInOption for T
where
    Option<T>: Debug,
{
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

fn main() {
    let v = vec![1, 2, 3];
    v.print_in_option();
}
