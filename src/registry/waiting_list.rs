pub trait QueueStrategy {
    type Element;
    fn new(lt: fn(&Self::Element, &Self::Element) -> bool) -> Self;
    fn insert(&mut self, element: Self::Element);
    fn remove_one(&mut self, strategy: fn(&Self::Element) -> bool) -> Result<(), String>;
    fn remove(&mut self, strategy: fn(&Self::Element) -> bool) -> Result<(), String>;
    fn pick(&mut self) -> Option<Self::Element>;
    fn peek(&self) -> Option<&Self::Element>;
}

pub struct SortStrategy<T> {
    container: Vec<T>,
    lt: fn(&T, &T) -> bool,
}

impl<T> QueueStrategy for SortStrategy<T> {
    type Element = T;
    fn new(lt: fn(&T, &T) -> bool) -> Self {
        Self {
            container: Vec::new(),
            lt,
        }
    }
    fn insert(&mut self, element: T) {
        let idx = self.container.partition_point(|x| (self.lt)(&x, &element));
        self.container.insert(idx, element);
    }
    fn remove_one(&mut self, strategy: fn(&Self::Element) -> bool) -> Result<(), String> {
        let idx = self.container.iter().position(|x| strategy(x));
        self.container.remove(idx.ok_or("Item not found")?);
        Ok(())
    }
    fn remove(&mut self, strategy: fn(&Self::Element) -> bool) -> Result<(), String> {
        let n = self.container.len();
        self.container.retain(|x| !strategy(x));
        if n != self.container.len() {
            Ok(())
        } else {
            Err("No item has been removed".to_string())
        }
    }
    fn pick(&mut self) -> Option<Self::Element> {
        self.container.pop()
    }
    fn peek(&self) -> Option<&Self::Element> {
        self.container.last()
    }
}

// pub struct HeapStrategy<T: Ord> {
//     container: BinaryHeap<T>,
// }
//
// impl<T: Ord> Default for HeapStrategy<T> {
//     fn default() -> Self {
//         Self {
//             container: BinaryHeap::new(),
//         }
//     }
// }
