pub trait QueueI {
    type Element; // TODO associated type vs generics
    fn new(lt: fn(&Self::Element, &Self::Element) -> bool) -> Self;
    fn insert(&mut self, element: Self::Element);
    fn remove(&mut self, element: &Self::Element) -> Result<Self::Element, String>;
    fn remove_by<F>(&mut self, strategy: F) -> Result<Vec<Self::Element>, String>
    where
        F: Fn(&Self::Element) -> bool;
    fn remove_one_by<F>(&mut self, strategy: F) -> Result<Self::Element, String>
    where
        F: Fn(&Self::Element) -> bool;
    fn pick(&mut self) -> Option<Self::Element>;
    fn peek(&self) -> Option<&Self::Element>;
}

pub struct SortStrategy<T: PartialEq> {
    container: Vec<T>,
    lt: fn(&T, &T) -> bool,
}

impl<T: PartialEq> QueueI for SortStrategy<T> {
    type Element = T;
    fn new(lt: fn(&T, &T) -> bool) -> Self {
        Self {
            container: Vec::new(),
            lt,
        }
    }
    fn insert(&mut self, element: T) {
        let idx = self.container.partition_point(|x| (self.lt)(x, &element));
        self.container.insert(idx, element);
    }
    fn remove(&mut self, element: &T) -> Result<T, String> {
        let idx = self.container.iter().position(|e| e == element);
        Ok(self.container.remove(idx.ok_or("Item not found")?))
    }
    fn remove_by<F>(&mut self, strategy: F) -> Result<Vec<T>, String>
    where
        F: Fn(&T) -> bool,
    {
        // Until drain_filter is stable https://github.com/rust-lang/rust/issues/43244
        let mut drained = Vec::new();
        for i in 0..drained.len() {
            if strategy(&self.container[i]) {
                drained.push(self.container.remove(i));
            }
        }
        if drained.is_empty() {
            Err("No item has been removed".to_string())
        } else {
            Ok(drained)
        }
    }
    fn remove_one_by<F>(&mut self, strategy: F) -> Result<T, String>
    where
        F: Fn(&T) -> bool,
    {
        let idx = self.container.iter().position(strategy);
        Ok(self.container.remove(idx.ok_or("Item not found")?))
    }
    fn pick(&mut self) -> Option<T> {
        self.container.pop()
    }
    fn peek(&self) -> Option<&T> {
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
