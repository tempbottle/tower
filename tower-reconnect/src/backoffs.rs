use std::time::Duration;
use std::iter;

#[derive(Debug, Copy, Clone)]
pub struct NoBackoffs;

impl<'a> IntoIterator for &'a NoBackoffs {
    type Item = Duration;
    type IntoIter = iter::Empty<Duration>;
    fn into_iter(self) -> Self::IntoIter {
        iter::empty()
    }
}
