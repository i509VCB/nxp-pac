#[doc = "no description available"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B {
    ptr: *mut u8,
}
unsafe impl Send for B {}
unsafe impl Sync for B {}
impl B {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Byte pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn b_(self, n: usize) -> crate::common::Reg<regs::B, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 1usize) as _) }
    }
}
#[doc = "General Purpose I/O (GPIO)"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpio {
    ptr: *mut u8,
}
unsafe impl Send for Gpio {}
unsafe impl Sync for Gpio {}
impl Gpio {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn b(self, n: usize) -> B {
        assert!(n < 2usize);
        unsafe { B::from_ptr(self.ptr.wrapping_add(0x0usize + n * 32usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn w(self, n: usize) -> W {
        assert!(n < 2usize);
        unsafe { W::from_ptr(self.ptr.wrapping_add(0x1000usize + n * 128usize) as _) }
    }
    #[doc = "Direction registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn dir(self, n: usize) -> crate::common::Reg<regs::Dir, crate::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2000usize + n * 4usize) as _)
        }
    }
    #[doc = "Mask register for all port GPIO pins"]
    #[inline(always)]
    pub const fn mask(self, n: usize) -> crate::common::Reg<regs::Mask, crate::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2080usize + n * 4usize) as _)
        }
    }
    #[doc = "Port pin register for all port GPIO pins"]
    #[inline(always)]
    pub const fn pin(self, n: usize) -> crate::common::Reg<regs::Pin, crate::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2100usize + n * 4usize) as _)
        }
    }
    #[doc = "Masked port register for all port GPIO pins"]
    #[inline(always)]
    pub const fn mpin(self, n: usize) -> crate::common::Reg<regs::Mpin, crate::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2180usize + n * 4usize) as _)
        }
    }
    #[doc = "Write: Set register for port. Read: output bits for port"]
    #[inline(always)]
    pub const fn set(self, n: usize) -> crate::common::Reg<regs::Set, crate::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2200usize + n * 4usize) as _)
        }
    }
    #[doc = "Clear port for all port GPIO pins"]
    #[inline(always)]
    pub const fn clr(self, n: usize) -> crate::common::Reg<regs::Clr, crate::common::W> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2280usize + n * 4usize) as _)
        }
    }
    #[doc = "Toggle port for all port GPIO pins"]
    #[inline(always)]
    pub const fn not(self, n: usize) -> crate::common::Reg<regs::Not, crate::common::W> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2300usize + n * 4usize) as _)
        }
    }
    #[doc = "Set pin direction bits for port"]
    #[inline(always)]
    pub const fn dirset(self, n: usize) -> crate::common::Reg<regs::Dirset, crate::common::W> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2380usize + n * 4usize) as _)
        }
    }
    #[doc = "Clear pin direction bits for port"]
    #[inline(always)]
    pub const fn dirclr(self, n: usize) -> crate::common::Reg<regs::Dirclr, crate::common::W> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2400usize + n * 4usize) as _)
        }
    }
    #[doc = "Toggle pin direction bits for port"]
    #[inline(always)]
    pub const fn dirnot(self, n: usize) -> crate::common::Reg<regs::Dirnot, crate::common::W> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2480usize + n * 4usize) as _)
        }
    }
}
#[doc = "no description available"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W {
    ptr: *mut u8,
}
unsafe impl Send for W {}
unsafe impl Sync for W {}
impl W {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Word pin registers for all port GPIO pins"]
    #[inline(always)]
    pub const fn w_(self, n: usize) -> crate::common::Reg<regs::W, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
}
pub mod regs;
