#[doc = "LPC_Next0 Frequency Measurement (FREQME)"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Freqme {
    ptr: *mut u8,
}
unsafe impl Send for Freqme {}
unsafe impl Sync for Freqme {}
impl Freqme {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Frequency Measurement (in Read mode)"]
    #[inline(always)]
    pub const fn freqmectrl_r(self) -> crate::common::Reg<regs::FreqmectrlR, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Freqeuncy Measurement (in Write mode)"]
    #[inline(always)]
    pub const fn freqmectrl_w(self) -> crate::common::Reg<regs::FreqmectrlW, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
}
pub mod regs;
pub mod vals;
