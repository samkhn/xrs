// Closure
//
// fn add_one(x: u32) -> u32 { x + 1 }
// could be
// let add_one_v1 = |x: u32| -> u32 { x + 1};
// let add_one_v2 = |x| { x + 1};
// let add_one_v3 = |x| x + 1;
//
// x can either be mutably borrowed, immutably borrowed, or taken by the closure
//
// let list = vec![1, 2, 3];
// let only_borrows = || println!("From closure: {:?}", list);
// let mutable_borrow = || list.push(7);
// println("Before {:?}", list);
// only_borrows();
// println("After only borrow {:?}", list);  // will print [1, 2, 3]
// mutably_borrows();
// println("After mutable borrow {:?}", list);  // will print [1, 2, 3, 7]

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list: Vec<Rectangle> = vec![
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 5,
            height: 12,
        },
    ];
    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
    let scaled_up: Vec<_> = list
        .iter()
        .map(|r| Rectangle {
            width: r.width * 2,
            height: r.height * 2,
        })
        .collect();
    println!("{:#?}, scaled by 2 is {:#?}", list, scaled_up);
}
