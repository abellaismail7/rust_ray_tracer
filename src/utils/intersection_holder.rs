


#[derive(Debug)]
pub struct IntersectionHolder<T> {
    size: usize,
    capacity: usize,
    arr: Vec<T>,
}

impl<T> IntersectionHolder<T> {

    pub fn new(capacity: usize) -> Self {
        Self {
            size: 0,
            capacity,
            arr: Vec::<T>::with_capacity(capacity)
        }
    }

    pub fn nextt(&mut self) -> Option<&mut T> {
        if self.size == self.capacity
        {
            return None;
        }
        Some(&mut self.arr[self.size])
    }

    pub fn push(&mut self, item: T) {
        self.arr.push(item);
    }

    pub fn clear(&mut self)
    {
        self.size = 0;
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        self.arr.get(i)
    }

}


