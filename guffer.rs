#[derive(Debug)]
struct Guffer {
    buffer: Vec<char>,
    head: usize,
    tail: usize,
    pos: usize,
}

impl Guffer {
    fn new(src: Vec<char>, pos: usize, buffer_size: usize) -> Self {
        let mut buffer = Vec::with_capacity(src.len() + buffer_size);
        let head = pos + 1;
        let tail = pos + buffer_size;

        let mut src_iter = src.iter();

        for _i in 0..head {
            let c = src_iter.next().unwrap();
            buffer.push(*c);
        }

        for _gb in head..tail + 1 {
            buffer.push('_');
        }

        for c in src_iter {
            buffer.push(*c);
        }

        Self {
            buffer,
            head: head,
            tail: tail,
            pos,
        }
    }

    fn insert(&mut self, input: &[char]) -> Result<bool, String> {
        if input.len() > self.tail - self.head {
            return Err(String::from("Error: Reallocation required"));
        }

        for c in input {
            self.buffer[self.head] = *c;
            self.head += 1;
            self.pos += 1;
        }

        Ok(true)
    }

    fn grow_buffer(&mut self) {}

    fn remove(&mut self) {
        todo!()
    }

    fn move_left(&mut self, move_count: usize) {
        for _ in 0..move_count {
            self.buffer[self.tail] = self.buffer[self.head - 1];
            // TODO: replace the char that already copy to head pointer don't need to replace
            self.buffer[self.head - 1] = '_';

            self.head -= 1;
            self.tail -= 1;
            self.pos -= 1;
        }
    }

    fn move_right(&mut self, move_count: usize) {
        for _ in 0..move_count {
            self.buffer[self.head] = self.buffer[self.tail + 1];
            // TODO: replace the char that already copy to head pointer don't need to replace
            self.buffer[self.tail + 1] = '_';

            self.head += 1;
            self.tail += 1;
            self.pos += 1;
        }
    }

    fn raw_dump(&self) {
        for (_i, item) in self.buffer.iter().enumerate() {
            print!("{item}");
        }

        println!("");
        for (i, _item) in self.buffer.iter().enumerate() {
            if self.pos == i {
                print!("^");
                continue;
            } else if self.head == i {
                print!("H");
            } else if self.tail == i {
                print!("T");
            } else {
                print!(" ");
            }
        }
        println!("");
    }

    fn dump(&self) {
        for (i, item) in self.buffer.iter().enumerate() {
            if i < self.head || i > self.tail {
                print!("{item}");
            }
        }
    }
}

fn main() {
    let text = "string-to-edit-in-guffer-IDE ";

    let mut g: Guffer = Guffer::new(text.chars().collect::<Vec<char>>(), 7, 10);
    g.raw_dump();

    g.move_right(text.len() - 9);
    g.raw_dump();

    g.move_left(4);
    g.raw_dump();

    let test_src: Vec<char> = "this is some zybrish here".chars().collect();
    if g.insert(&test_src).is_err() {}

    g.raw_dump();

    let test_src: Vec<char> = " Guarav".chars().collect();
    let _ = g.insert(&test_src);

    g.raw_dump();

    g.dump();
}
