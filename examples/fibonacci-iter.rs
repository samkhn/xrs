struct Fibonacci {
    current: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current = self.next;
        self.next = current + self.next;
        Some(current)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci {
        current: 0,
        next: 1,
    }
}

fn main() {
    for i in fibonacci().take(4) {
        println!("{}", i);
    }
    for i in fibonacci().skip(4).take(4) {
        println!("{}", i);
    }
}
