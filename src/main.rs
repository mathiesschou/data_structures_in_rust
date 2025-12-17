/*
 * DATASTRUCTURES
*/

/*
 * Fixed-sized list
 * docs: 'Primitive Type array'
 * */

const CAPACITY: usize = 8; /* Fixed sized array: 8. 
 *  'usize' is architecture dependent () and is used to find
 *  positions the quickest way possible */

#[derive(Debug)]
struct Fixedlist {
    data: [i32; CAPACITY], // Array with Type: i32 & and Fixed size: CAPACITY
    len: usize,            // The length of the array (contrained to 0 <= len <= 8)
}
impl Fixedlist {
    // Associated function: to create at new empty Fixedlist.
    fn new() -> Self {
        Self {
            data: [0; CAPACITY],
            len: 0,
        }
    }

    // Method: Check if the list is empty
    fn is_empty(&self) -> bool {
        self.len == 0
    }

    // Method: Is it full
    fn is_full(&self) -> bool {
        self.len == CAPACITY
    }

    // Method: Add to list
    fn push(&mut self, val: i32) {
        // Check if it is full
        if self.is_full() {
            return; // just return
        }
        self.data[self.len] = val; // insert value at current len
        self.len += 1; // add to length, so next values is stored
    }

    // Method: Remove last inserted from list
    fn pop(&mut self) -> Option<i32> {
        if self.is_empty() {
            None
        } else {
            self.len -= 1;
            Some(self.data[self.len])
            // len is the truth
            // so we still keep the poppede value, but is overwritten
            // in next push
        }
    }

    // Method: Aceess from list at index
    fn get(&self, index: usize) -> Option<i32> {
        if index >= self.len {
            None
        } else {
            Some(self.data[index])
        }
    }

    // Remove
    fn remove(&mut self, index: usize) -> Option<i32> {
        if index >= self.len {
            return None; // if over length of array return none;
        }

        let removed = self.data[index]; // get the value to return
        for i in index..self.len - 1 {
            // iterate from index to end of array
            self.data[i] = self.data[i + 1]; // shift 1 to left
        }

        self.len -= 1; // update length
        Some(removed)
    }
}

fn main() {
    let mut li = Fixedlist::new();
}
