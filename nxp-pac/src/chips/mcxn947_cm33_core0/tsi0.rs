#[doc = "TSI"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsi0 {
    ptr: *mut u8,
}
unsafe impl Send for Tsi0 {}
unsafe impl Sync for Tsi0 {}
impl Tsi0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "TSI CONFIG (TSI_CONFIG) for Self-Capacitor"]
    #[inline(always)]
    pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "TSI CONFIG (TSI_CONFIG) for Mutual-Capacitor"]
    #[inline(always)]
    pub const fn config_mutual(self) -> crate::common::Reg<regs::ConfigMutual, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "TSI Threshold"]
    #[inline(always)]
    pub const fn tshd(self) -> crate::common::Reg<regs::Tshd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "TSI General Control and Status"]
    #[inline(always)]
    pub const fn gencs(self) -> crate::common::Reg<regs::Gencs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "TSI Mutual-Capacitance"]
    #[inline(always)]
    pub const fn mul(self) -> crate::common::Reg<regs::Mul, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "TSI SINC Filter"]
    #[inline(always)]
    pub const fn sinc(self) -> crate::common::Reg<regs::Sinc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "TSI SSC 0"]
    #[inline(always)]
    pub const fn ssc0(self) -> crate::common::Reg<regs::Ssc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "TSI SSC 1"]
    #[inline(always)]
    pub const fn ssc1(self) -> crate::common::Reg<regs::Ssc1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "TSI SSC 2"]
    #[inline(always)]
    pub const fn ssc2(self) -> crate::common::Reg<regs::Ssc2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "TSI Baseline"]
    #[inline(always)]
    pub const fn baseline(self) -> crate::common::Reg<regs::Baseline, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "TSI Channel Merge"]
    #[inline(always)]
    pub const fn chmerge(self) -> crate::common::Reg<regs::Chmerge, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "TSI Shield"]
    #[inline(always)]
    pub const fn shield(self) -> crate::common::Reg<regs::Shield, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "TSI Data and Status"]
    #[inline(always)]
    pub const fn data(self) -> crate::common::Reg<regs::Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "TSI Miscellaneous"]
    #[inline(always)]
    pub const fn misc(self) -> crate::common::Reg<regs::Misc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "TSI AUTO TRIG"]
    #[inline(always)]
    pub const fn trig(self) -> crate::common::Reg<regs::Trig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
