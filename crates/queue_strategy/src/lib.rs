pub mod sort_strategy;
pub mod heap_strategy;

use std::fmt;

pub trait QueueI<T>: fmt::Debug {
    // type Element; // TODO associated type vs generics
    // fn new(lt: fn(&T, &T) -> bool) -> Self;
    fn insert(&mut self, element: T) -> &T;
    fn remove(&mut self, element: &T) -> Result<T, String>;
    fn remove_by<F>(&mut self, strategy: F) -> Result<Vec<T>, String>
    where
        F: Fn(&T) -> bool;
    fn remove_one_by<F>(&mut self, strategy: F) -> Result<T, String>
    where
        F: Fn(&T) -> bool;
    fn pick(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
}
