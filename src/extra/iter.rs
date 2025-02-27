#[napi]
pub struct Iterable {}

mod iterable {
    use std::vec::IntoIter;

    pub struct JsIterator<T> {
        iter: dyn Iterator<Item = T>
    }
    
    pub struct IteratorResult<T> {
        done: bool,
        value: Option<T>,
    }

    impl<T> JsIterator<T> {
        fn next(&mut self) -> IteratorResult<T> {
            let next = self.iter.next();
            IteratorResult { done: next.is_none(), value: next }
        }
    }
}

#[napi]
impl Iterable {
    #[napi(js_name = "[Symbol.iterator]")]
    pub fn iter(&self) {}
}