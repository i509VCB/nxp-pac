#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NVIC {
    ptr: *mut u8,
}
unsafe impl Send for NVIC {}
unsafe impl Sync for NVIC {}
impl NVIC {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Interrupt Set Enable Register."]
    #[inline(always)]
    pub const fn ISER(self, n: usize) -> crate::common::Reg<regs::ISER, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "Interrupt Clear Enable Register."]
    #[inline(always)]
    pub const fn ICER(self, n: usize) -> crate::common::Reg<regs::ICER, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 4usize) as _) }
    }
    #[doc = "Interrupt Set Pending Register."]
    #[inline(always)]
    pub const fn ISPR(self, n: usize) -> crate::common::Reg<regs::ISPR, crate::common::RW> {
        assert!(n < 16usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 4usize) as _)
        }
    }
    #[doc = "Interrupt Clear Pending Register."]
    #[inline(always)]
    pub const fn ICPR(self, n: usize) -> crate::common::Reg<regs::ICPR, crate::common::RW> {
        assert!(n < 16usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize + n * 4usize) as _)
        }
    }
    #[doc = "Interrupt Active Bit Register."]
    #[inline(always)]
    pub const fn IABR(self, n: usize) -> crate::common::Reg<regs::IABR, crate::common::RW> {
        assert!(n < 16usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 4usize) as _)
        }
    }
    #[doc = "Interrupt Target Non-secure Register."]
    #[inline(always)]
    pub const fn ITNS(self, n: usize) -> crate::common::Reg<regs::ITNS, crate::common::RW> {
        assert!(n < 16usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize + n * 4usize) as _)
        }
    }
    #[doc = "Interrupt Priority Register."]
    #[inline(always)]
    pub const fn IPR(self, n: usize) -> crate::common::Reg<regs::IPR, crate::common::RW> {
        assert!(n < 120usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize + n * 4usize) as _)
        }
    }
    #[doc = "Software Trigger Interrupt Register."]
    #[inline(always)]
    pub const fn STIR(self) -> crate::common::Reg<regs::STIR, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e00usize) as _) }
    }
}
pub mod regs;
pub mod vals;
