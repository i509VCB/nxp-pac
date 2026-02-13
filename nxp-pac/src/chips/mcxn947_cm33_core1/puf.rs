#[doc = "PUF"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Puf {
    ptr: *mut u8,
}
unsafe impl Send for Puf {}
unsafe impl Sync for Puf {}
impl Puf {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Operation Result"]
    #[inline(always)]
    pub const fn orr(self) -> crate::common::Reg<regs::Orr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Status"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Allow"]
    #[inline(always)]
    pub const fn ar(self) -> crate::common::Reg<regs::Ar, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Interrupt Mask"]
    #[inline(always)]
    pub const fn imr(self) -> crate::common::Reg<regs::Imr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Data Destination"]
    #[inline(always)]
    pub const fn data_dest(self) -> crate::common::Reg<regs::DataDest, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Data Source"]
    #[inline(always)]
    pub const fn data_src(self) -> crate::common::Reg<regs::DataSrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Data Input"]
    #[inline(always)]
    pub const fn dir(self) -> crate::common::Reg<regs::Dir, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Data Output"]
    #[inline(always)]
    pub const fn dor(self) -> crate::common::Reg<regs::Dor, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "Miscellaneous"]
    #[inline(always)]
    pub const fn misc(self) -> crate::common::Reg<regs::Misc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Interface Status"]
    #[inline(always)]
    pub const fn if_sr(self) -> crate::common::Reg<regs::IfSr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "PUF Score"]
    #[inline(always)]
    pub const fn psr(self) -> crate::common::Reg<regs::Psr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "Hardware Restrict User Context 0"]
    #[inline(always)]
    pub const fn hw_ruc0(self) -> crate::common::Reg<regs::HwRuc0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "Hardware Restrict User Context 1"]
    #[inline(always)]
    pub const fn hw_ruc1(self) -> crate::common::Reg<regs::HwRuc1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Hardware Information"]
    #[inline(always)]
    pub const fn hw_info(self) -> crate::common::Reg<regs::HwInfo, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "Hardware Identifier"]
    #[inline(always)]
    pub const fn hw_id(self) -> crate::common::Reg<regs::HwId, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "Hardware Version"]
    #[inline(always)]
    pub const fn hw_ver(self) -> crate::common::Reg<regs::HwVer, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "SRAM Configuration"]
    #[inline(always)]
    pub const fn sram_cfg(self) -> crate::common::Reg<regs::SramCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "Status"]
    #[inline(always)]
    pub const fn sram_status(self) -> crate::common::Reg<regs::SramStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
    }
    #[doc = "Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn sram_int_clr_enable(
        self,
    ) -> crate::common::Reg<regs::SramIntClrEnable, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d8usize) as _) }
    }
    #[doc = "Interrupt Enable Set"]
    #[inline(always)]
    pub const fn sram_int_set_enable(
        self,
    ) -> crate::common::Reg<regs::SramIntSetEnable, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03dcusize) as _) }
    }
    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub const fn sram_int_status(
        self,
    ) -> crate::common::Reg<regs::SramIntStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e0usize) as _) }
    }
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub const fn sram_int_enable(
        self,
    ) -> crate::common::Reg<regs::SramIntEnable, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e4usize) as _) }
    }
    #[doc = "Interrupt Status Clear"]
    #[inline(always)]
    pub const fn sram_int_clr_status(
        self,
    ) -> crate::common::Reg<regs::SramIntClrStatus, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e8usize) as _) }
    }
    #[doc = "Interrupt Status set"]
    #[inline(always)]
    pub const fn sram_int_set_status(
        self,
    ) -> crate::common::Reg<regs::SramIntSetStatus, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03ecusize) as _) }
    }
}
pub mod regs;
pub mod vals;
