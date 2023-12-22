use atomic_refcell::*;
use downcast_rs::{impl_downcast, Downcast};

use crate::AccessError;

/// A type that can be stored in the resource store.
///
/// This trait is automatically implemented for types which meet the requirements.
pub trait Resource: Downcast + 'static {}

impl<T> Resource for T where T: 'static {}

impl_downcast!(Resource);

pub(crate) struct ResourceCell {
    inner: AtomicRefCell<Box<dyn Resource>>,
}

impl ResourceCell {
    pub fn new(resource: Box<dyn Resource>) -> Self {
        Self {
            inner: AtomicRefCell::new(resource),
        }
    }

    pub fn into_inner(self) -> Box<dyn Resource> {
        self.inner.into_inner()
    }

    pub fn try_borrow<T: Resource>(&self) -> Result<AtomicRef<T>, AccessError> {
        match self.inner.try_borrow() {
            Ok(borrow) => Ok(AtomicRef::map(borrow, |inner| {
                inner.downcast_ref().unwrap()
            })),
            Err(_) => Err(AccessError::AlreadyBorrowed),
        }
    }

    pub fn try_borrow_mut<T: Resource>(&self) -> Result<AtomicRefMut<T>, AccessError> {
        match self.inner.try_borrow_mut() {
            Ok(borrow) => Ok(AtomicRefMut::map(borrow, |inner| {
                inner.downcast_mut().unwrap()
            })),
            Err(_) => Err(AccessError::AlreadyBorrowed),
        }
    }
}