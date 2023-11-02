use std::iter::Iterator;

#[derive(Clone)]
pub struct CurrentIterator<I>
where
    I: Iterator,
    <I as ::std::iter::Iterator>::Item: ::std::clone::Clone,
{
    iterator: I,
    current: Option<I::Item>,
}

impl<I> CurrentIterator<I>
where
    I: Iterator,
    <I as ::std::iter::Iterator>::Item: ::std::clone::Clone,
{
    pub fn new(iterator: I) -> Self {
        CurrentIterator {
            iterator,
            current: None,
        }
    }

    pub fn current(&self) -> Option<I::Item> {
        self.current.clone()
    }
}

impl<I> Iterator for CurrentIterator<I>
where
    I: Iterator,
    <I as ::std::iter::Iterator>::Item: ::std::clone::Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        self.current = self.iterator.next();
        self.current.clone()
    }
}
