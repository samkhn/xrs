use std::{fmt, mem};

// Tuples are a collection of values of different types (essentially a struct)
// Functions can use these to return multiple values
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (a, b) = pair;
    (b, a)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

fn transpose(matrix: &Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

// Arrays are fixed length contiguous collection of objects
// Slices are similar to arrays but length is unknown
// Slices are two words: first word is a pointer to data, second word is length
fn analyze_slice(slice: &[i32]) {
    println!("First element of slice is: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn main() {
    // Bitset and integers

    // Tuple
    let sample_tuple = (100i32, false);
    println!(
        "reversing {:?} gives us {:?}",
        sample_tuple,
        reverse(sample_tuple)
    );

    let sample_matrix: Matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("matrix:\n{}", sample_matrix);
    println!("transpose(matrix):\n{}", transpose(&sample_matrix));

    // Array
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];

    println!("xs start with {} and {}", xs[0], xs[1]);
    println!(
        "ys starts with {} and ends with {}",
        ys[0],
        ys[ys.len() - 1]
    );
    println!("ys holds {} bytes of memory", mem::size_of_val(&ys));

    // Arrays can be borrowed as slices
    analyze_slice(&xs);

    // Empty slice
    let empty: [u32; 0] = [];
    assert_eq!(&empty, &[]);
    assert_eq!(&empty, &[][..]);
}
