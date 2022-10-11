pub mod sort_strategy;
use std::fmt;

pub trait QueueI: fmt::Display {
    type Element; // TODO associated type vs generics
    fn new(lt: fn(&Self::Element, &Self::Element) -> bool) -> Self;
    fn insert(&mut self, element: Self::Element) -> &Self::Element;
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
