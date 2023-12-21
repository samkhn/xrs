// Problem that motivates associated items
// trait Contains {
//     fn contains(&self, _: &A, _: &B) -> bool;
//     fn first(&self) -> i32;
//     fn last(&self) -> i32;
// }
//
// impl Contains<i32, i32> for Container {
//     fn contains(&self, num1: &i32, num2: &i32) -> bool {
// 	(&self.0 == num1) && (&self.1 == num2)
//     }
//
//     fn first(&self) -> i32 {
// 	self.0
//     }
//
//     fn last(&self) -> i32 {
// 	self.1
//     }
// }
//
// // Problem is here
// // C already contains A and B, so expressing A and B again is a nuisance
// fn difference<A, B, C>(container: &C) -> i32 where
//     C: Contains<A, B> {
//     container.last() - container.first()
// }

struct Container(i32, i32);

trait Contains {
    // Solution: associate the types with the trait
    type A;
    type B;
    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    type A = i32;
    type B = i32;
    fn contains(&self, num1: &Self::A, num2: &Self::B) -> bool {
        (&self.0 == num1) && (&self.1 == num2)
    }
    fn first(&self) -> i32 {
        self.0
    }
    fn last(&self) -> i32 {
        self.1
    }
}

fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}

fn main() {
    let num1 = 3;
    let num2 = 10;
    let num3 = 15;
    let container = Container(num1, num2);
    println!(
        "Does container contain {} and {}: {}",
        num1,
        num2,
        container.contains(&num1, &num2)
    );
    println!(
        "Does container contain {} and {}: {}",
        num1,
        num3,
        container.contains(&num1, &num3)
    );
    println!("first: {}", container.first());
    println!("last: {}", container.last());
    println!("diff: {}", difference(&container));
}
