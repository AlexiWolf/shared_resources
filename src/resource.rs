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

impl std::fmt::Debug for ResourceCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_id = self.as_any().type_id();
        f.debug_struct("ResourceCell")
            .field("inner", &type_id)
            .finish()
    }
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

    pub fn try_borrow<T: Resource>(&self) -> Result<Ref<T>, AccessError> {
        match self.inner.try_borrow() {
            Ok(borrow) => Ok(AtomicRef::map(borrow, |inner| {
                Ref::from_atomic_ref(inner.downcast_ref().unwrap())
            })),
            Err(_) => Err(AccessError::AlreadyBorrowed),
        }
    }

    pub fn try_borrow_mut<T: Resource>(&self) -> Result<RefMut<T>, AccessError> {
        match self.inner.try_borrow_mut() {
            Ok(borrow) => Ok(AtomicRefMut::map(borrow, |inner| {
                RefMut::from_atomic_ref(inner.downcast_mut().unwrap())
            })),
            Err(_) => Err(AccessError::AlreadyBorrowed),
        }
    }
}

/// An immutable reference to a [`Resource`] stored in the [`Resources`](crate::Resources) 
/// container.
#[derive(Debug)]
pub struct Ref<'a, T: Resource + 'static> {
    inner: AtomicRef<'a, T>,
}

impl<'a, T> Ref<'a, T> {
    pub(crate) fn from_atomic_ref(atomic_ref: AtomicRef<'a, T>) -> Self {
        Self { inner: atomic_ref }
    }
}

impl<'a, T> std::ops::Deref for Ref<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

/// A mutable reference to a [`Resource`] stored in the [`Resources`](crate::Resources) container.
#[derive(Debug)]
pub struct RefMut<'a, T: Resource + 'static> {
    inner: AtomicRefMut<'a, T>,
}

impl<'a, T> RefMut<'a, T> {
    pub(crate) fn from_atomic_ref(atomic_ref: AtomicRefMut<'a, T>) -> Self {
        Self { inner: atomic_ref }
    }
}

impl<'a, T> std::ops::Deref for RefMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl<'a, T> std::ops::DerefMut for RefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.deref_mut()
    }
}
