#[doc = "CSI"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csi {
    ptr: *mut u8,
}
unsafe impl Send for Csi {}
unsafe impl Sync for Csi {}
impl Csi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "CSI Control Register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "CSI Control Register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "CSI Control Register 3"]
    #[inline(always)]
    pub const fn cr3(self) -> crate::common::Reg<regs::Cr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "CSI Statistic FIFO Register"]
    #[inline(always)]
    pub const fn statfifo(self) -> crate::common::Reg<regs::Statfifo, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "CSI RX FIFO Register"]
    #[inline(always)]
    pub const fn rfifo(self) -> crate::common::Reg<regs::Rfifo, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "CSI RX Count Register"]
    #[inline(always)]
    pub const fn rxcnt(self) -> crate::common::Reg<regs::Rxcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "CSI Status Register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "CSI DMA Start Address Register - for STATFIFO"]
    #[inline(always)]
    pub const fn dmasa_statfifo(
        self,
    ) -> crate::common::Reg<regs::DmasaStatfifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "CSI DMA Transfer Size Register - for STATFIFO"]
    #[inline(always)]
    pub const fn dmats_statfifo(
        self,
    ) -> crate::common::Reg<regs::DmatsStatfifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "CSI DMA Start Address Register - for Frame Buffer1"]
    #[inline(always)]
    pub const fn dmasa_fb1(self) -> crate::common::Reg<regs::DmasaFb1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "CSI DMA Transfer Size Register - for Frame Buffer2"]
    #[inline(always)]
    pub const fn dmasa_fb2(self) -> crate::common::Reg<regs::DmasaFb2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "CSI Frame Buffer Parameter Register"]
    #[inline(always)]
    pub const fn fbuf_para(self) -> crate::common::Reg<regs::FbufPara, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "CSI Image Parameter Register"]
    #[inline(always)]
    pub const fn imag_para(self) -> crate::common::Reg<regs::ImagPara, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "CSI Control Register 18"]
    #[inline(always)]
    pub const fn cr18(self) -> crate::common::Reg<regs::Cr18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "CSI Control Register 19"]
    #[inline(always)]
    pub const fn cr19(self) -> crate::common::Reg<regs::Cr19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
