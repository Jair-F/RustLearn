
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

    pub fn push(&mut self, item: T) -> Option<T> {
        let overwritten = if self.len == N {
            let old = self.data[self.head].take();
            self.head = (self.head + 1) % N;
            old
        }
        else {
            self.len += 1;
            None
        };

        self.data[self.tail] = Some(item);
        self.tail = (self.tail + 1) % N;
        overwritten
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        let ret = self.data[self.head].take();
        self.head = (self.head + 1) % N;
        self.len -= 1;
        ret
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

pub fn test_task1_1() {
    // Create a stack-allocated ring buffer for integers with capacity 3
    const BUFF_SIZE:usize = 3;
    let mut buf: RingBuffer<i32, BUFF_SIZE> = RingBuffer::new();

    assert_eq!(buf.pop(), None);

    assert_eq!(buf.push(10), None);
    assert_eq!(buf.push(20), None);
    assert_eq!(buf.push(30), None);

    // Buffer is now full [1, 6, 7]. Pushing 40 overwrites oldest (10).
    assert_eq!(buf.push(40), Some(10));

    assert_eq!(buf.is_empty(), false);
    assert_eq!(buf.len(), BUFF_SIZE);

    // Pop remaining elements in FIFO order
    assert_eq!(buf.pop(), Some(20));
    assert_eq!(buf.pop(), Some(30));
    assert_eq!(buf.pop(), Some(40));
    assert_eq!(buf.pop(), None); // Empty again

    assert_eq!(buf.is_empty(), true);

    println!("✅ RingBuffer passed all checks!");
}
