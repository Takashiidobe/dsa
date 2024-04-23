use std::{
    borrow::Borrow,
    fmt::{self, Debug},
};

//@ A CopyOnWrite can be in either of two states:
pub enum CopyOnWrite<'a, B: 'a>
where
    B: ToOwned,
{
    //@ Borrowed, where it is a reference to the underlying data.
    Borrowed(&'a B),
    //@ Owned, where it uniquely owns the data.
    Owned(<B as ToOwned>::Owned),
}

use CopyOnWrite::*;

//@ CopyOnWrite requires a manual implementation of Clone:
impl<B: ToOwned> Clone for CopyOnWrite<'_, B> {
    fn clone(&self) -> Self {
        match *self {
            //@ If the CoW is Borrowed, the clone returns the borrowed data.
            Borrowed(b) => Borrowed(b),
            //@ If the Cow is Owned, the clone returns a new copy of the data.
            Owned(ref owned) => Owned(owned.borrow().to_owned()),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        match (self, source) {
            (&mut Owned(ref mut dest), Owned(ref owned)) => owned.borrow().clone_into(dest),
            (t, s) => *t = s.clone(),
        }
    }
}

impl<B: ToOwned> CopyOnWrite<'_, B> {
    pub fn to_mut(&mut self) -> &mut <B as ToOwned>::Owned {
        match self {
            Borrowed(borrowed) => {
                *self = Owned(borrowed.to_owned());
                match self {
                    Borrowed(..) => unreachable!(),
                    Owned(ref mut owned) => owned,
                }
            }
            Owned(ref mut owned) => owned,
        }
    }

    pub fn into_owned(self) -> <B as ToOwned>::Owned {
        match self {
            Borrowed(borrowed) => borrowed.to_owned(),
            Owned(owned) => owned,
        }
    }
}

impl<B> Debug for CopyOnWrite<'_, B>
where
    B: Debug + ToOwned<Owned: Debug>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Borrowed(ref b) => Debug::fmt(b, f),
            Owned(ref o) => Debug::fmt(o, f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_clones() {
        let mut cow: CopyOnWrite<String> = CopyOnWrite::Owned("Hi".to_string());
        let mut cow1 = cow.clone();

        let mut mutable_cow = cow1.into_owned();
        mutable_cow.push_str(" world");
        assert_eq!(cow.clone().into_owned(), "Hi".to_string());
        assert_ne!(mutable_cow, cow.into_owned());
    }
}
