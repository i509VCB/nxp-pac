#[doc = "TMR"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmr {
    ptr: *mut u8,
}
unsafe impl Send for Tmr {}
unsafe impl Sync for Tmr {}
impl Tmr {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Timer Channel Compare Register 1"]
    #[inline(always)]
    pub const fn comp10(self) -> crate::common::Reg<regs::Comp10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Timer Channel Compare Register 2"]
    #[inline(always)]
    pub const fn comp20(self) -> crate::common::Reg<regs::Comp20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "Timer Channel Capture Register"]
    #[inline(always)]
    pub const fn capt0(self) -> crate::common::Reg<regs::Capt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Timer Channel Load Register"]
    #[inline(always)]
    pub const fn load0(self) -> crate::common::Reg<regs::Load0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "Timer Channel Hold Register"]
    #[inline(always)]
    pub const fn hold0(self) -> crate::common::Reg<regs::Hold0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Timer Channel Counter Register"]
    #[inline(always)]
    pub const fn cntr0(self) -> crate::common::Reg<regs::Cntr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "Timer Channel Control Register"]
    #[inline(always)]
    pub const fn ctrl0(self) -> crate::common::Reg<regs::Ctrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Timer Channel Status and Control Register"]
    #[inline(always)]
    pub const fn sctrl0(self) -> crate::common::Reg<regs::Sctrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
    #[doc = "Timer Channel Comparator Load Register 1"]
    #[inline(always)]
    pub const fn cmpld10(self) -> crate::common::Reg<regs::Cmpld10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Timer Channel Comparator Load Register 2"]
    #[inline(always)]
    pub const fn cmpld20(self) -> crate::common::Reg<regs::Cmpld20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12usize) as _) }
    }
    #[doc = "Timer Channel Comparator Status and Control Register"]
    #[inline(always)]
    pub const fn csctrl0(self) -> crate::common::Reg<regs::Csctrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Timer Channel Input Filter Register"]
    #[inline(always)]
    pub const fn filt0(self) -> crate::common::Reg<regs::Filt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x16usize) as _) }
    }
    #[doc = "Timer Channel DMA Enable Register"]
    #[inline(always)]
    pub const fn dma0(self) -> crate::common::Reg<regs::Dma0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Timer Channel Enable Register"]
    #[inline(always)]
    pub const fn enbl(self) -> crate::common::Reg<regs::Enbl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1eusize) as _) }
    }
    #[doc = "Timer Channel Compare Register 1"]
    #[inline(always)]
    pub const fn comp11(self) -> crate::common::Reg<regs::Comp11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Timer Channel Compare Register 2"]
    #[inline(always)]
    pub const fn comp21(self) -> crate::common::Reg<regs::Comp21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x22usize) as _) }
    }
    #[doc = "Timer Channel Capture Register"]
    #[inline(always)]
    pub const fn capt1(self) -> crate::common::Reg<regs::Capt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Timer Channel Load Register"]
    #[inline(always)]
    pub const fn load1(self) -> crate::common::Reg<regs::Load1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x26usize) as _) }
    }
    #[doc = "Timer Channel Hold Register"]
    #[inline(always)]
    pub const fn hold1(self) -> crate::common::Reg<regs::Hold1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Timer Channel Counter Register"]
    #[inline(always)]
    pub const fn cntr1(self) -> crate::common::Reg<regs::Cntr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2ausize) as _) }
    }
    #[doc = "Timer Channel Control Register"]
    #[inline(always)]
    pub const fn ctrl1(self) -> crate::common::Reg<regs::Ctrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Timer Channel Status and Control Register"]
    #[inline(always)]
    pub const fn sctrl1(self) -> crate::common::Reg<regs::Sctrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2eusize) as _) }
    }
    #[doc = "Timer Channel Comparator Load Register 1"]
    #[inline(always)]
    pub const fn cmpld11(self) -> crate::common::Reg<regs::Cmpld11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Timer Channel Comparator Load Register 2"]
    #[inline(always)]
    pub const fn cmpld21(self) -> crate::common::Reg<regs::Cmpld21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x32usize) as _) }
    }
    #[doc = "Timer Channel Comparator Status and Control Register"]
    #[inline(always)]
    pub const fn csctrl1(self) -> crate::common::Reg<regs::Csctrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Timer Channel Input Filter Register"]
    #[inline(always)]
    pub const fn filt1(self) -> crate::common::Reg<regs::Filt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x36usize) as _) }
    }
    #[doc = "Timer Channel DMA Enable Register"]
    #[inline(always)]
    pub const fn dma1(self) -> crate::common::Reg<regs::Dma1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Timer Channel Compare Register 1"]
    #[inline(always)]
    pub const fn comp12(self) -> crate::common::Reg<regs::Comp12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Timer Channel Compare Register 2"]
    #[inline(always)]
    pub const fn comp22(self) -> crate::common::Reg<regs::Comp22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x42usize) as _) }
    }
    #[doc = "Timer Channel Capture Register"]
    #[inline(always)]
    pub const fn capt2(self) -> crate::common::Reg<regs::Capt2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Timer Channel Load Register"]
    #[inline(always)]
    pub const fn load2(self) -> crate::common::Reg<regs::Load2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x46usize) as _) }
    }
    #[doc = "Timer Channel Hold Register"]
    #[inline(always)]
    pub const fn hold2(self) -> crate::common::Reg<regs::Hold2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Timer Channel Counter Register"]
    #[inline(always)]
    pub const fn cntr2(self) -> crate::common::Reg<regs::Cntr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4ausize) as _) }
    }
    #[doc = "Timer Channel Control Register"]
    #[inline(always)]
    pub const fn ctrl2(self) -> crate::common::Reg<regs::Ctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Timer Channel Status and Control Register"]
    #[inline(always)]
    pub const fn sctrl2(self) -> crate::common::Reg<regs::Sctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4eusize) as _) }
    }
    #[doc = "Timer Channel Comparator Load Register 1"]
    #[inline(always)]
    pub const fn cmpld12(self) -> crate::common::Reg<regs::Cmpld12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Timer Channel Comparator Load Register 2"]
    #[inline(always)]
    pub const fn cmpld22(self) -> crate::common::Reg<regs::Cmpld22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x52usize) as _) }
    }
    #[doc = "Timer Channel Comparator Status and Control Register"]
    #[inline(always)]
    pub const fn csctrl2(self) -> crate::common::Reg<regs::Csctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Timer Channel Input Filter Register"]
    #[inline(always)]
    pub const fn filt2(self) -> crate::common::Reg<regs::Filt2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x56usize) as _) }
    }
    #[doc = "Timer Channel DMA Enable Register"]
    #[inline(always)]
    pub const fn dma2(self) -> crate::common::Reg<regs::Dma2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Timer Channel Compare Register 1"]
    #[inline(always)]
    pub const fn comp13(self) -> crate::common::Reg<regs::Comp13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Timer Channel Compare Register 2"]
    #[inline(always)]
    pub const fn comp23(self) -> crate::common::Reg<regs::Comp23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x62usize) as _) }
    }
    #[doc = "Timer Channel Capture Register"]
    #[inline(always)]
    pub const fn capt3(self) -> crate::common::Reg<regs::Capt3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Timer Channel Load Register"]
    #[inline(always)]
    pub const fn load3(self) -> crate::common::Reg<regs::Load3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x66usize) as _) }
    }
    #[doc = "Timer Channel Hold Register"]
    #[inline(always)]
    pub const fn hold3(self) -> crate::common::Reg<regs::Hold3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "Timer Channel Counter Register"]
    #[inline(always)]
    pub const fn cntr3(self) -> crate::common::Reg<regs::Cntr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6ausize) as _) }
    }
    #[doc = "Timer Channel Control Register"]
    #[inline(always)]
    pub const fn ctrl3(self) -> crate::common::Reg<regs::Ctrl3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "Timer Channel Status and Control Register"]
    #[inline(always)]
    pub const fn sctrl3(self) -> crate::common::Reg<regs::Sctrl3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6eusize) as _) }
    }
    #[doc = "Timer Channel Comparator Load Register 1"]
    #[inline(always)]
    pub const fn cmpld13(self) -> crate::common::Reg<regs::Cmpld13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Timer Channel Comparator Load Register 2"]
    #[inline(always)]
    pub const fn cmpld23(self) -> crate::common::Reg<regs::Cmpld23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x72usize) as _) }
    }
    #[doc = "Timer Channel Comparator Status and Control Register"]
    #[inline(always)]
    pub const fn csctrl3(self) -> crate::common::Reg<regs::Csctrl3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "Timer Channel Input Filter Register"]
    #[inline(always)]
    pub const fn filt3(self) -> crate::common::Reg<regs::Filt3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x76usize) as _) }
    }
    #[doc = "Timer Channel DMA Enable Register"]
    #[inline(always)]
    pub const fn dma3(self) -> crate::common::Reg<regs::Dma3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
}
pub mod regs;
pub mod vals;
