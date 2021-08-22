use super::Nimber;

use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};

impl<T: Clone> Clone for Nimber<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self { x: self.x.clone() }
    }

    #[inline]
    fn clone_from(&mut self, source: &Self) {
        self.x.clone_from(&source.x)
    }
}

impl<T: Copy> Copy for Nimber<T> {}

impl<T: Debug> Debug for Nimber<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Nimber").field("x", &self.x).finish()
    }
}

impl<T: Default> Default for Nimber<T> {
    #[inline]
    fn default() -> Self {
        Nimber { x: T::default() }
    }
}

impl<T> From<T> for Nimber<T> {
    #[inline]
    fn from(x: T) -> Self {
        Self { x }
    }
}

impl<T> Nimber<T> {
    /// Converts to the inner type
    #[inline]
    pub fn unwrap(self) -> T {
        self.x
    }
}

impl<T: Hash> Hash for Nimber<T> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state)
    }
}

impl<T: PartialEq> PartialEq for Nimber<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
    }

    #[inline]
    fn ne(&self, other: &Self) -> bool {
        self.x != other.x
    }
}

impl<T: Eq> Eq for Nimber<T> {}

impl<T: PartialOrd> PartialOrd for Nimber<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.x.partial_cmp(&other.x)
    }

    #[inline]
    fn lt(&self, other: &Self) -> bool {
        self.x.lt(&other.x)
    }

    #[inline]
    fn le(&self, other: &Self) -> bool {
        self.x.le(&other.x)
    }

    #[inline]
    fn gt(&self, other: &Self) -> bool {
        self.x.gt(&other.x)
    }

    #[inline]
    fn ge(&self, other: &Self) -> bool {
        self.x.ge(&other.x)
    }
}

impl<T: Ord> Ord for Nimber<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.x.cmp(&other.x)
    }
}
