struct RingBuffer<T, const N: usize> {
    data: [Option<T>; N],
    head: usize,
    tail: usize,
    len: usize,
}

impl<T, const N: usize> RingBuffer<T, N> {
    pub fn new() -> Self {
        Self {
            data: [const {None}; N],
            head: 0,
            tail: 0,
            len: 0,
        }
    }

}

pub fn test_task1_1() {
    // Create a stack-allocated ring buffer for integers with capacity 3
    let _buf: RingBuffer<i32, 3> = RingBuffer::new();

    // assert_eq!(buf.pop(), None);

    // assert_eq!(buf.push(10), None);
    // assert_eq!(buf.push(20), None);
    // assert_eq!(buf.push(30), None);

    // // Buffer is now full [1, 6, 7]. Pushing 40 overwrites oldest (10).
    // assert_eq!(buf.push(40), Some(10));

    // // Pop remaining elements in FIFO order
    // assert_eq!(buf.pop(), Some(20));
    // assert_eq!(buf.pop(), Some(30));
    // assert_eq!(buf.pop(), Some(40));
    // assert_eq!(buf.pop(), None); // Empty again

    println!("✅ RingBuffer passed all checks!");
}
