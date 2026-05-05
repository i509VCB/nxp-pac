#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LUT {
    ptr: *mut u8,
}
unsafe impl Send for LUT {}
unsafe impl Sync for LUT {}
impl LUT {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "LUTn input x MUX."]
    #[inline(always)]
    pub const fn LUT_INP_MUX(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::LUT_INP_MUX, crate::common::RW> {
        assert!(n < 5usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
}
#[doc = "LPC80X Programmable Logic Unit (PLU)."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PLU {
    ptr: *mut u8,
}
unsafe impl Send for PLU {}
unsafe impl Sync for PLU {}
impl PLU {
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
    pub const fn LUT(self, n: usize) -> LUT {
        assert!(n < 26usize);
        unsafe { LUT::from_ptr(self.ptr.wrapping_add(0x0usize + n * 32usize) as _) }
    }
    #[doc = "Specifies the Truth Table contents for LUTLUTn."]
    #[inline(always)]
    pub const fn LUT_TRUTH(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::LUT_TRUTH, crate::common::RW> {
        assert!(n < 26usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0800usize + n * 4usize) as _)
        }
    }
    #[doc = "Provides the current state of the 8 designated PLU Outputs."]
    #[inline(always)]
    pub const fn OUTPUTS(self) -> crate::common::Reg<regs::OUTPUTS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0900usize) as _) }
    }
    #[doc = "Wakeup interrupt control for PLU."]
    #[inline(always)]
    pub const fn WAKEINT_CTRL(self) -> crate::common::Reg<regs::WAKEINT_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0904usize) as _) }
    }
    #[doc = "Selects the source to be connected to PLU Output OUTPUT_n."]
    #[inline(always)]
    pub const fn OUTPUT_MUX(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::OUTPUT_MUX, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c00usize + n * 4usize) as _)
        }
    }
}
pub mod regs;
pub mod vals;
