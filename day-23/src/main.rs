use std::convert::TryInto as _;

fn main() {
    let input = std::env::args()
        .skip(1)
        .next()
        .expect("Input must be supplied");
    let input = input
        .chars()
        .map(|c| c.to_string().parse())
        .collect::<Result<Vec<usize>, _>>()
        .unwrap();
    let mut game = Game::from(&input, 9).unwrap();
    println!("{}", game.output_n(20));
    for i in 0..100 {
        // println!("{}", game.output(i));
        game.perform_move();
        // println!();
    }
    println!("{}", game.after());

    let mut game = Game::from(&input, 1_000_000).unwrap();
    println!("{}", game.output_n(20));
    for i in 0..10_000_000 {
        game.perform_move();
    }
    println!("{}", game.after_part_b());
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Value {
    v: usize,
    max: usize,
}

impl Value {
    fn build(v: usize, max: usize) -> Result<Value, ()> {
        if 1 <= v && v <= max {
            Ok(Value { v: v, max: max })
        } else {
            Err(())
        }
    }

    fn is_one(&self) -> bool {
        self.v == 1
    }

    fn unwrap(self) -> usize {
        self.v
    }

    pub fn pred(self) -> Self {
        if self.v == 1 {
            Value {
                v: self.max,
                max: self.max,
            }
        } else {
            Value {
                v: self.v - 1,
                max: self.max,
            }
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.v.fmt(f)
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
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
    memory: Vec<Node>,
    current: *mut Node,
}

impl Game {
    pub fn from(v: &[usize], max: usize) -> Result<Self, ()> {
        let mut memory = Vec::with_capacity(max);

        let mut next_hm = std::collections::HashMap::new();
        let mut prev_hm = std::collections::HashMap::new();

        for i in 0..v.len() {
            if i > 0 {
                prev_hm.insert(v[i], v[i - 1]);
            } else {
                prev_hm.insert(v[i], max);
            }
            if i < v.len() - 1 {
                next_hm.insert(v[i], v[i + 1]);
            } else if max > v.len() {
                next_hm.insert(v[i], v.len() + 1);
            } else {
                next_hm.insert(v[i], v[0]);
            }
        }

        for i in 1..=max {
            let value = Value::build(i, max)?;
            let node = Node::floating(value);
            memory.push(node)
        }

        let memory_ptr = memory.as_mut_ptr();
        let size = memory.len();
        for index in 0..size {
            let value = index + 1;

            let previous_index = if let Some(prev_value) = prev_hm.get(&value) {
                prev_value - 1
            } else {
                if index == 0 {
                    size - 1
                } else {
                    index - 1
                }
            };
            let next_index = if let Some(next_value) = next_hm.get(&value) {
                next_value - 1
            } else {
                if index == size - 1 {
                    0
                } else {
                    index + 1
                }
            };
            let node = memory.get_mut(index).unwrap();
            let previous_ptr = unsafe { memory_ptr.add(previous_index) };
            let next_ptr = unsafe { memory_ptr.add(next_index) };
            node.prev = previous_ptr;
            node.next = next_ptr;
        }

        let current = unsafe { memory_ptr.add(v[0] - 1) };
        Ok(Game {
            memory: memory,
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

    unsafe fn pointer_for_value(&mut self, value: usize) -> *mut Node {
        let memory_ptr = self.memory.as_mut_ptr();
        memory_ptr.add(value - 1)
    }

    pub fn output_n(&self, mut n: usize) -> String {
        let mut next_output = self.current;

        let mut output = String::new();
        while n > 0 {
            n -= 1;
            let node = unsafe { next_output.as_ref().unwrap() };
            output.push_str(&format!(" {} ", node.value));
            next_output = node.next;
        }
        output
    }

    pub fn output(&self, round: usize) -> String {
        let mut start = self.current;
        for _ in 0..(round % self.memory.len()) {
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

        let mut destination_ptr = unsafe { self.pointer_for_value(destination.unwrap()) };
        /*
        // println!("destination: {}", destination);
        let mut destination_ptr = head.next;
        let mut destination_node = unsafe { destination_ptr.as_ref().unwrap() };
        while destination_node.value != destination {
            destination_ptr = destination_node.next;
            destination_node = unsafe { destination_ptr.as_ref().unwrap() };
        }
        */
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

    fn after(&mut self) -> String {
        /*
        let mut one_ptr = self.current;
        let mut one_node = unsafe { one_ptr.as_ref().unwrap() };
        while !one_node.value.is_one() {
            one_ptr = one_node.next;
            one_node = unsafe { one_ptr.as_ref().unwrap() };
        }
        */
        let one_ptr = unsafe { self.pointer_for_value(1) };
        let one_node = unsafe { one_ptr.as_ref().unwrap() };

        let mut output = String::new();
        let mut c_ptr = one_node.next;
        while c_ptr != one_ptr {
            let c_node = unsafe { c_ptr.as_ref().unwrap() };
            c_ptr = c_node.next;
            output.push_str(&format!("{}", c_node.value));
        }

        output
    }

    fn after_part_b(&mut self) -> usize {
        /*
        let mut one_ptr = self.current;
        let mut one_node = unsafe { one_ptr.as_ref().unwrap() };
        while !one_node.value.is_one() {
            one_ptr = one_node.next;
            one_node = unsafe { one_ptr.as_ref().unwrap() };
        }
        */

        let one_ptr = unsafe { self.pointer_for_value(1) };
        let one_node = unsafe { one_ptr.as_ref().unwrap() };

        let next_a_ptr = unsafe { self.next(one_ptr) };
        let next_b_ptr = unsafe { self.next(next_a_ptr) };
        let next_a_node = unsafe { next_a_ptr.as_ref().unwrap() };
        let next_b_node = unsafe { next_b_ptr.as_ref().unwrap() };
        let next_a_value = next_a_node.value.unwrap();
        let next_b_value = next_b_node.value.unwrap();
        println!(
            "{} {} {}",
            one_node.value.unwrap(),
            next_a_value,
            next_b_value
        );
        next_a_value * next_b_value
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
