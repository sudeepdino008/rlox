use std::cell::{Ref, RefCell, RefMut};
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

pub struct Shared<T: ?Sized> {
    pub v: Rc<RefCell<T>>,
}

impl<T> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Self { v: self.v.clone() }
    }
}

impl<T> Shared<T> {
    pub fn new(t: T) -> Shared<T> {
        Shared {
            v: Rc::new(RefCell::new(t)),
        }
    }
}

impl<T: ?Sized> Shared<T> {
    pub fn new_with_rc(t: Rc<RefCell<T>>) -> Shared<T> {
        Shared { v: t.clone() }
    }

    pub fn borrow(&self) -> Ref<T> {
        self.v.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<T> {
        self.v.borrow_mut()
    }

    pub fn as_ptr(&self) -> *mut T {
        self.v.as_ptr()
    }
}

// impl<T> Shared<T> {

// }

impl<T: fmt::Display> fmt::Display for Shared<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}

impl<T: fmt::Debug> fmt::Debug for Shared<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.deref())
    }
}

impl<T: PartialEq> PartialEq for Shared<T> {
    fn eq(&self, other: &Self) -> bool {
        self.deref() == other.deref()
    }
}

impl<'a, T> Deref for Shared<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        unsafe { self.as_ptr().as_ref().unwrap() }
    }
}

impl<'a, T> DerefMut for Shared<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.as_ptr().as_mut().unwrap() }
    }
}
