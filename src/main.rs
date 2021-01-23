use std::marker::PhantomData;

fn main() {
    println!("Hello, world!");
}

// traits

pub trait BookIterator<'a>: Iterator<Item=f64> {}

pub trait BookSide<'a> {
    type BookIteratorType: BookIterator<'a>;

    fn book_iterator(&self) -> Self::BookIteratorType;
}

// implementation 1: stateful

pub struct ArrayBookSide {
    quotes: Vec<f64>,
}

pub struct ArrayBookSideIterator<'a> {
    quotes_iter: std::slice::Iter<'a, f64>,
}

impl<'a> BookSide<'a> for ArrayBookSide {
    type BookIteratorType = ArrayBookSideIterator<'a>;

    fn book_iterator(&self) -> Self::BookIteratorType {
        ArrayBookSideIterator { quotes_iter: self.quotes.iter() }
    }
}

impl<'a> Iterator for ArrayBookSideIterator<'a> {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        self.quotes_iter.next().map(|&quote| quote)
    }
}

impl<'a> BookIterator<'a> for ArrayBookSideIterator<'a> {}

// implementation 2: delegating

pub struct CommissionBookSide<'a, B>
    where B: BookSide<'a> {
    base_book_side: B,
    multiplier: f64,
    _marker: PhantomData<&'a B>,
}

impl<'a, B> CommissionBookSide<'a, B>
    where B: BookSide<'a> {
    pub fn new(base_book_side: B) -> CommissionBookSide<'a, B> {
        CommissionBookSide { base_book_side, multiplier: 1.1, _marker: PhantomData {} }
    }
}

impl<'a, B> BookSide<'a> for CommissionBookSide<'a, B>
    where B: BookSide<'a> {
    type BookIteratorType = CommissionIterator<'a, B::BookIteratorType>;

    fn book_iterator(&self) -> Self::BookIteratorType {
        CommissionIterator {
            base_iterator: self.base_book_side.book_iterator(),
            multiplier: self.multiplier,
            _marker: PhantomData {},
        }
    }
}

pub struct CommissionIterator<'a, BI>
    where BI: BookIterator<'a> {
    base_iterator: BI,
    multiplier: f64,
    _marker: PhantomData<&'a BI>,
}

impl<'a, BI> Iterator for CommissionIterator<'a, BI>
    where BI: BookIterator<'a> {
    type Item = BI::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.base_iterator.next().map(|quote| quote * self.multiplier)
    }
}

impl<'a, BI> BookIterator<'a> for CommissionIterator<'a, BI>
    where BI: BookIterator<'a> {}