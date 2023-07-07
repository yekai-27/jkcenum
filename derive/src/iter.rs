#[derive(Debug, Clone)]
pub struct EnumIterator<T>
{
    pub index: usize,
    pub value: Vec<T>,
}


#[allow(unused)]
impl<T> EnumIterator<T> {
    pub fn new(value: Vec<T>) -> Self {
        Self {
            index: 0,
            value,
        }
    }
}


impl<T: Copy> Iterator for EnumIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        self.index += 1;

        if index < self.value.len() {
            return Some(self.value[index]);
        }

        None
    }
}
