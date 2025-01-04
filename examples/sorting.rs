fn bubble_sort(vec: &mut Vec<i32>) -> &mut Vec<i32> {
    vec
}

fn quick_sort(vec: &mut Vec<i32>) -> &mut Vec<i32> {
    vec
}

fn merge_sort(vec: &mut Vec<i32>) -> &mut Vec<i32> {
    vec
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: &str, age: u32) -> Self {
        Person {
            name: name.to_string(),
            age,
        }
    }
}

fn main() {
    // std sort
    let mut vec = vec![1, 5, 10, 2, 15];
    vec.sort();
    assert_eq!(vec, vec![1, 2, 5, 10, 15]);

    // sorting floats
    let mut vec_f = vec![1.1, 1.15, 5.5, 1.123, 2.0];
    vec_f.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(vec_f, vec![1.1, 1.123, 1.15, 2.0, 5.5]);

    // sorting structs
    // In order to sort a struct, you need four traits: Eq, PartialEq, Ord, PartialOrd
    // Or you can provide a custom comparator to vec:sort_by
    let mut people = vec![
        Person::new("Zoe", 25),
        Person::new("Al", 50),
        Person::new("John", 1),
    ];
    people.sort();
    assert_eq!(
        people,
        vec![
            Person::new("Al", 50),
            Person::new("John", 1),
            Person::new("Zoe", 25)
        ]
    );
    people.sort_by(|a, b| b.age.cmp(&a.age));
    assert_eq!(
        people,
        vec![
            Person::new("Al", 50),
            Person::new("Zoe", 25),
            Person::new("John", 1)
        ]
    );

    // custom sorts
    // TODO: is there a better way to pass the data without cloning
    let mut vec2 = vec![1, 10, 3, 2, 40];
    println!(
        "bubble_sort({:?}) is {:?}",
        vec2.clone(),
        bubble_sort(&mut vec2)
    );
    println!(
        "quick_sort({:?}) is {:?}",
        vec2.clone(),
        quick_sort(&mut vec2)
    );
    println!(
        "merge_sort({:?}) is {:?}",
        vec2.clone(),
        merge_sort(&mut vec2)
    );
}
