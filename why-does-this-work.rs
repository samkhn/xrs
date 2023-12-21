// Non-copyable types.
struct Empty;
struct Null;

// drops by consuming both self and whatever was passed in
trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

// Implement `DoubleDrop<T>` for any generic parameter `T` and
// caller `U`.
impl<T, U> DoubleDrop<T> for U {
    // This method takes ownership of both passed arguments,
    // deallocating both.
    fn double_drop(self, _: T) {}
}

fn main() {
    let empty = Empty;
    let null = Null;

    // (Q) How did the Double Drop trait get implemented/declares for Empty Null???
    // (Q) Does calling this generate the underlying implementation?
    empty.double_drop(null);

    //empty;
    //null;
    // ^ TODO: Try uncommenting these lines.
}
