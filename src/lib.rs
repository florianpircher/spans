//! This crate allows you to split an iterator into contiguous spans.
//!
//! Import the [`Spans`] trait to extend [`Iterator`]:
//!
//! ```
//! use spans::Spans;
//! ```
//!
//! Now you can use [`spans_by_key`][Spans::spans_by_key] to split an iterator into contiguous spans:
//!
//! ```
//! # use spans::Spans;
//! let vec = vec![1, 2, 5, 6, 7, 11, 13, 14, 15];
//! let mut spans = vec.iter().spans_by_key(|&&x| x, |a, b| a + 1 == b);
//!
//! while let Some(span) = spans.next() {
//!     println!("span = {:?}", span.collect::<Vec<_>>());
//! }
//!```
//!
//! The code above splits the vector into spans where each item is 1 larger than the proceeding item.
//! The following text is printed:
//!
//! ```text
//! span = [1, 2]
//! span = [5, 6, 7]
//! span = [11]
//! span = [13, 14, 15]
//! ```
//!
//! For more information, refer to the [`spans_by_key`][Spans::spans_by_key] documentation.

#![deny(missing_docs)]

use std::iter::Peekable;

/// `SpansBy` wraps an iterator and provides progressive access to contiguous spans of the iterator.
///
/// See [`Spans::spans_by_key`] for more information.
pub struct SpansBy<I: Iterator, K, F> {
    /// The wrapped iterator.
    iter: Peekable<I>,
    /// A function transforming an iterator item to a comparison key.
    key: K,
    /// Whether two iterator items belong to the same span as determined by their respective keys.
    are_connected: F,
}

impl<I, K, C, F> SpansBy<I, K, F>
where
    I: Iterator,
    K: Fn(&I::Item) -> C,
    C: Copy,
    F: Fn(C, C) -> bool,
{
    /// Returns the next span or `None` if the iterator terminated.
    ///
    /// # Example
    ///
    /// ```
    /// use spans::Spans;
    ///
    /// let vec = vec![1, 2, 5];
    /// let mut spans = vec.iter().spans_by_key(|&&x| x, |a, b| a + 1 == b);
    ///
    /// while let Some(span) = spans.next() {
    ///     // do something with `span`
    ///     let count = span.count();
    ///     assert!(count > 0); // true for all spans
    ///     assert!(count < 3); // true for the spans in this example
    /// }
    /// ```
    pub fn next(&mut self) -> Option<Span<'_, I, K, C, F>> {
        if let Some(first) = self.iter.peek() {
            let prev_key = (self.key)(first);
            Some(Span {
                parent: self,
                prev_key,
                is_init: true,
            })
        } else {
            None
        }
    }
}

/// A `Span` is an iterator that iterates over a span of its parent iterator.
///
/// A span is always non-empty; at least one item is provided when iterating over a span.
/// This is because empty spans are handled and discarded by the parent `SpansBy` iterator.
///
/// See [`Spans::spans_by_key`] for more information.
pub struct Span<'a, I: Iterator, K, C, F> {
    /// The parent iterator.
    parent: &'a mut SpansBy<I, K, F>,
    /// The key of the previous iterator item.
    prev_key: C,
    /// Whether no item has been accessed yet.
    ///
    /// `true` initially, `false` after the first `Span::next` invocation.
    is_init: bool,
}

impl<I, K, C, F> Iterator for Span<'_, I, K, C, F>
where
    I: Iterator,
    K: Fn(&I::Item) -> C,
    C: Copy,
    F: Fn(C, C) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_init {
            self.is_init = false;
            return self.parent.iter.next();
        }

        let peek = self.parent.iter.peek()?;
        let peek_key = (self.parent.key)(peek);

        let item = if !(self.parent.are_connected)(self.prev_key, peek_key) {
            None
        } else {
            self.parent.iter.next()
        };

        self.prev_key = peek_key;
        item
    }
}

/// `Spans` provides an iterator adapter for `SpansBy`.
pub trait Spans: Iterator {
    /// Splits the iterator into contiguous spans.
    ///
    /// `are_connected` returns `true` if the two given adjacent items are part of the same span, `false` otherwise.
    /// Items are not compared directly; instead, a key is made for each item using the `key` function.
    /// `are_connected` is given the key of the previous item and the key of the current item.
    ///
    /// # Examples
    ///
    /// Create spans for items increasing in value by 1:
    ///
    /// ```
    /// use spans::Spans;
    /// # fn test() -> Option<()> {
    ///
    /// let vec = vec![1, 2, 5, 6, 7, 11, 13, 14, 15];
    /// let mut spans = vec.iter().spans_by_key(|&&x| x, |a, b| a + 1 == b);
    ///
    /// assert_eq!(spans.next()?.collect::<Vec<_>>(), vec![&1, &2]);
    /// assert_eq!(spans.next()?.collect::<Vec<_>>(), vec![&5, &6, &7]);
    /// assert_eq!(spans.next()?.collect::<Vec<_>>(), vec![&11]);
    /// assert_eq!(spans.next()?.collect::<Vec<_>>(), vec![&13, &14, &15]);
    /// assert!(spans.next().is_none());
    /// # Some(())
    /// # }
    /// # fn main() { assert_eq!(test(), Some(())) }
    /// ```
    ///
    /// Create spans for strings of the same length:
    ///
    /// ```
    /// use spans::Spans;
    /// # fn test() -> Option<()> {
    ///
    /// let vec = vec!["abc", "run", "tag", "go", "be", "ring", "zip", "zap", "put"];
    /// let mut spans = vec.iter().spans_by_key(|x| x.len(), |a, b| a == b);
    ///
    /// assert_eq!(
    ///     spans.next()?.collect::<Vec<_>>(),
    ///     vec![&"abc", &"run", &"tag"]
    /// );
    /// assert_eq!(
    ///     spans.next()?.collect::<Vec<_>>(),
    ///     vec![&"go", &"be"]
    /// );
    /// assert_eq!(
    ///     spans.next()?.collect::<Vec<_>>(),
    ///     vec![&"ring"]
    /// );
    /// assert_eq!(
    ///     spans.next()?.collect::<Vec<_>>(),
    ///     vec![&"zip", &"zap", &"put"]
    /// );
    /// assert!(spans.next().is_none());
    /// # Some(())
    /// # }
    /// # fn main() { assert_eq!(test(), Some(())) }
    /// ```
    fn spans_by_key<K, C, F>(self, key: K, are_connected: F) -> SpansBy<Self, K, F>
    where
        K: Fn(&Self::Item) -> C,
        C: Copy,
        F: Fn(C, C) -> bool,
        Self: Sized,
    {
        SpansBy {
            iter: self.peekable(),
            key,
            are_connected,
        }
    }
}

impl<I: Iterator> Spans for I {}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_eq_span {
        ($next:expr, [ $($item:expr),* ]) => {
            assert_eq!(
                $next.collect::<Vec<_>>(),
                vec![$(&$item),*]
            );
        };
    }

    macro_rules! assert_eq_spans {
        ($spans:expr, [ $([ $($item:expr),* ]),* ]) => {
            $(
                assert_eq_span!($spans.next().unwrap(), [$($item),*]);
            )*
            assert!($spans.next().is_none());
        };
    }

    #[test]
    fn test_empty() {
        let vec: Vec<&str> = Vec::new();
        let mut spans = vec.iter().spans_by_key(|x| x.len(), |a, b| a == b);
        assert_eq_spans!(spans, []);
    }

    #[test]
    fn test_single_item() {
        let vec = vec!["abc"];
        let mut spans = vec.iter().spans_by_key(|x| x.len(), |a, b| a == b);
        assert_eq_spans!(spans, [["abc"]]);
    }

    #[test]
    fn test_two_items_one_span() {
        let vec = vec!["abc", "xyz"];
        let mut spans = vec.iter().spans_by_key(|x| x.len(), |a, b| a == b);
        assert_eq_spans!(spans, [["abc", "xyz"]]);
    }

    #[test]
    fn test_two_items_two_spans() {
        let vec = vec!["abc", "xyz"];
        let mut spans = vec
            .iter()
            .spans_by_key(|x| x.chars().nth(0).unwrap(), |a, b| a == b);
        assert_eq_spans!(spans, [["abc"], ["xyz"]]);
    }

    #[test]
    fn test_many_items() {
        let vec = vec![
            "abc", "run", "tag", "go", "be", "ring", "zip", "zap", "put", "", "", "end",
        ];
        let mut spans = vec.iter().spans_by_key(|x| x.len(), |a, b| a == b);
        assert_eq_spans!(
            spans,
            [
                ["abc", "run", "tag"],
                ["go", "be"],
                ["ring"],
                ["zip", "zap", "put"],
                ["", ""],
                ["end"]
            ]
        );
    }

    #[test]
    fn test_many_items_numbers() {
        let vec = vec![
            0, 1, 2, 3, 4, 10, 11, 12, 20, 30, 40, 50, 51, 60, 61, 62, 63, 64, 65, 70,
        ];
        let mut spans = vec.iter().spans_by_key(|&&x| x, |a, b| a + 1 == b);

        assert_eq_spans!(
            spans,
            [
                [0, 1, 2, 3, 4],
                [10, 11, 12],
                [20],
                [30],
                [40],
                [50, 51],
                [60, 61, 62, 63, 64, 65],
                [70]
            ]
        );
    }
}
