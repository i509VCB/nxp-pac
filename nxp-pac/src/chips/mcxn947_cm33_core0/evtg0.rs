#[doc = "EVTG"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evtg0 {
    ptr: *mut u8,
}
unsafe impl Send for Evtg0 {}
unsafe impl Sync for Evtg0 {}
impl Evtg0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Array of registers: EVTG_AOI0_BFT01, EVTG_AOI0_BFT23, EVTG_AOI1_BFT01, EVTG_AOI1_BFT23, EVTG_CTRL, EVTG_AOI0_FILT, EVTG_AOI1_FILT"]
    #[inline(always)]
    pub const fn evtg_inst(self, n: usize) -> EvtgInst {
        assert!(n < 4usize);
        unsafe { EvtgInst::from_ptr(self.ptr.wrapping_add(0x0usize + n * 16usize) as _) }
    }
}
#[doc = "Array of registers: EVTG_AOI0_BFT01, EVTG_AOI0_BFT23, EVTG_AOI1_BFT01, EVTG_AOI1_BFT23, EVTG_CTRL, EVTG_AOI0_FILT, EVTG_AOI1_FILT"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EvtgInst {
    ptr: *mut u8,
}
unsafe impl Send for EvtgInst {}
unsafe impl Sync for EvtgInst {}
impl EvtgInst {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "AOI0 Boolean Function Term 0 and 1 Configuration"]
    #[inline(always)]
    pub const fn evtg_aoi0_bft01(
        self,
    ) -> crate::common::Reg<regs::EvtgAoi0Bft01, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "AOI0 Boolean Function Term 2 and 3 Configuration"]
    #[inline(always)]
    pub const fn evtg_aoi0_bft23(
        self,
    ) -> crate::common::Reg<regs::EvtgAoi0Bft23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "AOI1 Boolean Function Term 0 and 1 Configuration"]
    #[inline(always)]
    pub const fn evtg_aoi1_bft01(
        self,
    ) -> crate::common::Reg<regs::EvtgAoi1Bft01, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "AOI1 Boolean Function Term 2 and 3 Configuration"]
    #[inline(always)]
    pub const fn evtg_aoi1_bft23(
        self,
    ) -> crate::common::Reg<regs::EvtgAoi1Bft23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "Control and Status"]
    #[inline(always)]
    pub const fn evtg_ctrl(self) -> crate::common::Reg<regs::EvtgCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "AOI0 Output Filter"]
    #[inline(always)]
    pub const fn evtg_aoi0_filt(self) -> crate::common::Reg<regs::EvtgAoi0Filt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "AOI1 Output Filter"]
    #[inline(always)]
    pub const fn evtg_aoi1_filt(self) -> crate::common::Reg<regs::EvtgAoi1Filt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
}
pub mod regs;
pub mod vals;
