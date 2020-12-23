use std::convert::TryInto as _;

fn main() {
    let input = std::env::args()
        .skip(1)
        .next()
        .expect("Input must be supplied");
    let input = input
        .chars()
        .map(|c| c.to_string().parse())
        .collect::<Result<Vec<u8>, _>>()
        .unwrap();
    let mut game = Game::from(input).unwrap();
    for i in 0..100 {
        // println!("{}", game.output(i));
        game.perform_move();
        // println!();
    }
    println!("{}", game.after());

    /*
    let whatever = unsafe { ptr2.as_ref() };
    println!("{:?}", whatever);
    let whatever = unsafe { ptr3.as_ref() };
    println!("{:?}", whatever);
    */
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Value(u8);

impl std::convert::TryFrom<u8> for Value {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if 1 <= value && value <= 9 {
            Ok(Value(value))
        } else {
            Err(())
        }
    }
}

impl Value {
    pub fn pred(self) -> Self {
        if self.0 == 1 {
            Value(9)
        } else {
            Value(self.0 - 1)
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug)]
struct Node {
    value: Value,
    prev: *mut Node,
    next: *mut Node,
}

impl Node {
    fn floating(value: Value) -> Node {
        Node {
            value,
            prev: std::ptr::null_mut(),
            next: std::ptr::null_mut(),
        }
    }
}

struct Game {
    _memory: Vec<Node>,
    current: *mut Node,
}

impl Game {
    pub fn from(v: Vec<u8>) -> Result<Self, ()> {
        let mut memory = Vec::with_capacity(v.len());

        for item in v {
            let value = item.try_into()?;
            let node = Node::floating(value);
            memory.push(node);
        }

        let memory_ptr = memory.as_mut_ptr();
        let size = memory.len();
        for i in 0..size {
            let previous_index = if i == 0 { size - 1 } else { i - 1 };
            let next_index = if i == size - 1 { 0 } else { i + 1 };
            let node = memory.get_mut(i).unwrap();
            let previous_ptr = unsafe { memory_ptr.add(previous_index) };
            let next_ptr = unsafe { memory_ptr.add(next_index) };
            node.prev = previous_ptr;
            node.next = next_ptr;
        }

        let current = memory_ptr;
        Ok(Game {
            _memory: memory,
            current,
        })
    }

    unsafe fn prev(&self, cur: *mut Node) -> *mut Node {
        let node = cur.as_ref().unwrap();
        node.prev
    }

    unsafe fn next(&self, cur: *mut Node) -> *mut Node {
        let node = cur.as_ref().unwrap();
        node.next
    }

    pub fn output(&self, round: usize) -> String {
        let mut start = self.current;
        for _ in 0..(round % self._memory.len()) {
            start = unsafe { self.prev(start) };
        }
        let mut next_output = start;
        let mut started = false;

        let mut output = String::new();

        while !started || next_output != start {
            started = true;
            let node = unsafe { next_output.as_ref().unwrap() };
            if next_output == self.current {
                output.push_str(&format!("({})", node.value));
            } else {
                output.push_str(&format!(" {} ", node.value));
            }
            next_output = node.next;
        }

        output
    }

    pub fn perform_move(&mut self) {
        let head_ptr = self.current;
        let start_of_three_ptr = unsafe { self.next(head_ptr) };
        let slice = self.take_n(3, start_of_three_ptr);

        // println!("After removing the three:\n{}", self.output(0));
        // println!("pick up: {:?}", slice);

        let head = unsafe { head_ptr.as_ref().unwrap() };
        let value = head.value;

        let mut destination = value.pred();
        while slice.contains(destination) {
            destination = destination.pred();
        }

        // println!("destination: {}", destination);
        let mut destination_ptr = head.next;
        let mut destination_node = unsafe { destination_ptr.as_ref().unwrap() };
        while destination_node.value != destination {
            destination_ptr = destination_node.next;
            destination_node = unsafe { destination_ptr.as_ref().unwrap() };
        }
        self.insert_slice(destination_ptr, slice);
        self.current = unsafe { self.next(head_ptr) };
    }

    fn take_n(&mut self, n: usize, start_ptr: *mut Node) -> GameSlice {
        let prev_ptr = unsafe { self.prev(start_ptr) };
        let mut end_ptr = start_ptr;
        for _ in 1..n {
            end_ptr = unsafe { self.next(end_ptr) };
        }
        let after_end_ptr = unsafe { self.next(end_ptr) };

        let prev = unsafe { prev_ptr.as_mut().unwrap() };
        let after_end = unsafe { after_end_ptr.as_mut().unwrap() };
        let start = unsafe { start_ptr.as_mut().unwrap() };
        let end = unsafe { end_ptr.as_mut().unwrap() };

        prev.next = after_end_ptr;
        after_end.prev = prev_ptr;
        start.prev = end_ptr;
        end.next = start_ptr;

        GameSlice { head: start_ptr }
    }

    fn insert_slice(&mut self, after_ptr: *mut Node, slice: GameSlice) {
        let before_ptr = unsafe { self.next(after_ptr) };
        let slice_head_ptr = slice.head;
        let slice_tail_ptr = unsafe { self.prev(slice_head_ptr) };

        let mut after = unsafe { after_ptr.as_mut().unwrap() };
        let mut before = unsafe { before_ptr.as_mut().unwrap() };
        let mut slice_head = unsafe { slice_head_ptr.as_mut().unwrap() };
        let mut slice_tail = unsafe { slice_tail_ptr.as_mut().unwrap() };

        after.next = slice_head_ptr;
        slice_head.prev = after_ptr;

        before.prev = slice_tail_ptr;
        slice_tail.next = before_ptr;
    }

    fn after(&self) -> String {
        let one = 1.try_into().unwrap();
        let mut one_ptr = self.current;
        let mut one_node = unsafe { one_ptr.as_ref().unwrap() };
        while one_node.value != one {
            one_ptr = one_node.next;
            one_node = unsafe { one_ptr.as_ref().unwrap() };
        }

        let mut output = String::new();
        let mut c_ptr = one_node.next;
        while c_ptr != one_ptr {
            let c_node = unsafe { c_ptr.as_ref().unwrap() };
            c_ptr = c_node.next;
            output.push_str(&format!("{}", c_node.value));
        }

        output
    }
}

struct GameSlice {
    head: *mut Node,
}

impl GameSlice {
    fn contains(&self, value: Value) -> bool {
        let start = self.head;
        let mut next_check = start;
        let mut started = false;
        while !started || next_check != start {
            started = true;
            let node = unsafe { next_check.as_ref().unwrap() };
            if node.value == value {
                return true;
            }
            next_check = node.next;
        }
        false
    }
}

impl std::fmt::Debug for GameSlice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let start = self.head;
        let mut next_output = start;
        let mut started = false;
        while !started || next_output != start {
            started = true;
            let node = unsafe { next_output.as_ref().unwrap() };
            write!(f, " {} ", node.value)?;
            next_output = node.next;
        }
        Ok(())
    }
}
