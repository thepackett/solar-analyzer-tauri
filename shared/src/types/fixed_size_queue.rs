use std::slice::Iter;


//A FIFO queue with a fixed maximum size. Elements beyond its capacity are dropped. Note that this implementation is not optimized for large sizes.
pub struct FixedSizeQueue<T> {
    size: usize,
    elements: Vec<T>,
}

impl<T> FixedSizeQueue<T> {
    pub fn new(size: usize) -> FixedSizeQueue<T>{
        let size = if size == 0 {
            1
        } else {
            size
        };
        FixedSizeQueue { size: size, elements: Vec::with_capacity(size) }
    }

    // A reverse order push function
    // pub fn push(&mut self, element: T) -> Option<T> {
    //     let popped_value = if self.elements.len() == self.size {
    //         self.elements.pop()
    //     } else {
    //         None
    //     };
    //     self.elements.insert(0, element);
    //     popped_value
    // }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&T) -> bool
    {
        self.elements.retain(f);
    }

    pub fn push(&mut self, element: T) -> Option<T> {
        let popped_value = if self.elements.len() == self.size {
            Some(self.elements.remove(0))
        } else {
            None
        };
        self.elements.push(element);
        popped_value
    }

    pub fn iter(&self) -> Iter<T> {
        self.elements.iter()
    }
}