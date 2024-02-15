#![no_std]
use core::ptr::NonNull;

pub struct RO<const OFFSET: usize, T: Sized + Copy>(NonNull<T>);
pub struct WO<const OFFSET: usize, T: Sized + Copy>(NonNull<T>);
pub struct RW<const OFFSET: usize, T: Sized + Copy>(NonNull<T>);

pub trait VolatileWrap<T: Sized> {
    fn action_immutable<Return, F>(&self, func: F) -> Return
    where
        F: FnOnce(*const T) -> Return;

    fn action_mutable<Return, F>(&mut self, func: F) -> Return
    where
        F: FnOnce(*mut T) -> Return;
}

pub trait VolatileRead<T>: VolatileWrap<T> {
    #[inline(always)]
    fn read(&self) -> T {
        self.action_immutable(|ptr| unsafe { core::ptr::read_volatile(ptr) })
    }
}

pub trait VolatileWrite<T>: VolatileWrap<T> {
    #[inline(always)]
    fn write(&mut self, value: T) {
        self.action_mutable(|ptr| unsafe { core::ptr::write_volatile(ptr, value) })
    }
}

impl<T: Sized + Copy, const OFFSET: usize> VolatileWrap<T> for RO<OFFSET, T> {
    #[inline(always)]
    fn action_immutable<Return, F>(&self, func: F) -> Return
    where
        F: FnOnce(*const T) -> Return,
    {
        func(self.0.as_ptr())
    }

    #[inline(always)]
    fn action_mutable<Return, F>(&mut self, _func: F) -> Return
    where
        F: FnOnce(*mut T) -> Return,
    {
        unreachable!("Cannot write to RO Register")
    }
}

impl<T: Sized + Copy, const OFFSET: usize> VolatileWrap<T> for WO<OFFSET, T> {
    #[inline(always)]
    fn action_immutable<Return, F>(&self, _func: F) -> Return
    where
        F: FnOnce(*const T) -> Return,
    {
        unreachable!("Cannot read from WO Register")
    }

    #[inline(always)]
    fn action_mutable<Return, F>(&mut self, func: F) -> Return
    where
        F: FnOnce(*mut T) -> Return,
    {
        func(self.0.as_ptr())
    }
}

impl<T: Sized + Copy, const OFFSET: usize> VolatileWrap<T> for RW<OFFSET, T> {
    #[inline(always)]
    fn action_immutable<Return, F>(&self, func: F) -> Return
    where
        F: FnOnce(*const T) -> Return,
    {
        func(self.0.as_ptr())
    }

    #[inline(always)]
    fn action_mutable<Return, F>(&mut self, func: F) -> Return
    where
        F: FnOnce(*mut T) -> Return,
    {
        func(self.0.as_ptr())
    }
}

impl<T: Sized + Copy, const OFFSET: usize> VolatileRead<T> for RO<OFFSET, T> {}
impl<T: Sized + Copy, const OFFSET: usize> VolatileRead<T> for RW<OFFSET, T> {}

impl<T: Sized + Copy, const OFFSET: usize> VolatileWrite<T> for WO<OFFSET, T> {}
impl<T: Sized + Copy, const OFFSET: usize> VolatileWrite<T> for RW<OFFSET, T> {}

impl<T: Sized + Copy, const OFFSET: usize> RO<OFFSET, T> {
    pub fn new(base_ptr: usize) -> Option<Self> {
        Some(Self(NonNull::new((base_ptr + OFFSET) as *mut T)?))
    }
}

impl<T: Sized + Copy, const OFFSET: usize> WO<OFFSET, T> {
    pub fn new(base_ptr: usize) -> Option<Self> {
        Some(Self(NonNull::new((base_ptr + OFFSET) as *mut T)?))
    }
}

impl<T: Sized + Copy, const OFFSET: usize> RW<OFFSET, T> {
    pub fn new(base_ptr: usize) -> Option<Self> {
        Some(Self(NonNull::new((base_ptr + OFFSET) as *mut T)?))
    }
}
