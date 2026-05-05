#[doc = "no description available."]
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
    #[doc = "Byte pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn B_(self, n: usize) -> crate::common::Reg<regs::B_, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 1usize) as _) }
    }
}
#[doc = "General Purpose I/O (GPIO)."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPIO {
    ptr: *mut u8,
}
unsafe impl Send for GPIO {}
unsafe impl Sync for GPIO {}
impl GPIO {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn B(self, n: usize) -> B {
        assert!(n < 2usize);
        unsafe { B::from_ptr(self.ptr.wrapping_add(0x0usize + n * 32usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn W(self, n: usize) -> W {
        assert!(n < 2usize);
        unsafe { W::from_ptr(self.ptr.wrapping_add(0x1000usize + n * 128usize) as _) }
    }
    #[doc = "Direction registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn DIR(self, n: usize) -> crate::common::Reg<regs::DIR, crate::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2000usize + n * 4usize) as _)
        }
    }
    #[doc = "Mask register for all port GPIO pins."]
    #[inline(always)]
    pub const fn MASK(self, n: usize) -> crate::common::Reg<regs::MASK, crate::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2080usize + n * 4usize) as _)
        }
    }
    #[doc = "Port pin register for all port GPIO pins."]
    #[inline(always)]
    pub const fn PIN(self, n: usize) -> crate::common::Reg<regs::PIN, crate::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2100usize + n * 4usize) as _)
        }
    }
    #[doc = "Masked port register for all port GPIO pins."]
    #[inline(always)]
    pub const fn MPIN(self, n: usize) -> crate::common::Reg<regs::MPIN, crate::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2180usize + n * 4usize) as _)
        }
    }
    #[doc = "Write: Set register for port. Read: output bits for port."]
    #[inline(always)]
    pub const fn SET(self, n: usize) -> crate::common::Reg<regs::SET, crate::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2200usize + n * 4usize) as _)
        }
    }
    #[doc = "Clear port for all port GPIO pins."]
    #[inline(always)]
    pub const fn CLR(self, n: usize) -> crate::common::Reg<regs::CLR, crate::common::W> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2280usize + n * 4usize) as _)
        }
    }
    #[doc = "Toggle port for all port GPIO pins."]
    #[inline(always)]
    pub const fn NOT(self, n: usize) -> crate::common::Reg<regs::NOT, crate::common::W> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2300usize + n * 4usize) as _)
        }
    }
    #[doc = "Set pin direction bits for port."]
    #[inline(always)]
    pub const fn DIRSET(self, n: usize) -> crate::common::Reg<regs::DIRSET, crate::common::W> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2380usize + n * 4usize) as _)
        }
    }
    #[doc = "Clear pin direction bits for port."]
    #[inline(always)]
    pub const fn DIRCLR(self, n: usize) -> crate::common::Reg<regs::DIRCLR, crate::common::W> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2400usize + n * 4usize) as _)
        }
    }
    #[doc = "Toggle pin direction bits for port."]
    #[inline(always)]
    pub const fn DIRNOT(self, n: usize) -> crate::common::Reg<regs::DIRNOT, crate::common::W> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2480usize + n * 4usize) as _)
        }
    }
}
#[doc = "no description available."]
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
    #[doc = "Word pin registers for all port GPIO pins."]
    #[inline(always)]
    pub const fn W_(self, n: usize) -> crate::common::Reg<regs::W_, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
}
pub mod regs;
