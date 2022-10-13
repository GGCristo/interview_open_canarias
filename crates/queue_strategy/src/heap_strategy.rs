use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt;

struct OrdWraper<T: PartialEq> {
    value: T,
    cmp: fn(&T, &T) -> Ordering,
}

impl<T: PartialEq> OrdWraper<T> {
    fn new(value: T, cmp: fn(&T, &T) -> Ordering) -> Self {
        Self { value, cmp }
    }
    #[inline]
    fn unwrap(&self) -> &T {
        &self.value
    }
    fn into_inner(self) -> T {
        self.value
    }
}

impl<T: PartialEq> Ord for OrdWraper<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.cmp)(&self.value, &other.value)
    }
}

impl<T: PartialEq> PartialOrd for OrdWraper<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((self.cmp)(&self.value, &other.value))
    }
}

impl<T: PartialEq> PartialEq for OrdWraper<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: PartialEq> Eq for OrdWraper<T> {}

pub struct HeapStrategy<T: PartialEq> {
    container: BinaryHeap<OrdWraper<T>>,
    cmp: fn(&T, &T) -> Ordering,
}

impl<T: PartialEq> HeapStrategy<T> {
    pub fn new(cmp: fn(&T, &T) -> Ordering) -> Self {
        Self {
            container: BinaryHeap::new(),
            cmp,
        }
    }
}

impl<T: PartialEq + Clone + fmt::Debug> super::QueueI<T> for HeapStrategy<T> {
    fn insert(&mut self, element: T) -> &T {
        let copy = element.clone();
        self.container.push(OrdWraper::new(element, self.cmp));
        for e in &self.container {
            if e.unwrap() == &copy {
                return e.unwrap();
            }
        }
        panic!("The element should have been in the container");
    }
    // Rust's binary heap is very simple, there is no way to do removes
    // with low complexity time
    fn remove(&mut self, _element: &T) -> Result<T, String> {
        Err("Todo".to_string())
    }
    fn remove_by<F>(&mut self, _strategy: F) -> Result<Vec<T>, String>
    where
        F: Fn(&T) -> bool,
    {
        Err("Todo".to_string())
    }
    fn remove_one_by<F>(&mut self, _strategy: F) -> Result<T, String>
    where
        F: Fn(&T) -> bool,
    {
        Err("Todo".to_string())
    }
    fn pick(&mut self) -> Option<T> {
        self.container
            .pop()
            .and_then(|wrapped| Some(wrapped.into_inner()))
    }
    fn peek(&self) -> Option<&T> {
        self.container
            .peek()
            .and_then(|wrapped| Some(wrapped.unwrap()))
    }
}

impl<T: PartialEq + fmt::Debug> fmt::Debug for HeapStrategy<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        let mut separator = "";
        for e in &self.container {
            result.push_str(&format!("{separator}{:?}", e.unwrap()));
            separator = ", ";
        }
        write!(f, "{result}")
    }
}
