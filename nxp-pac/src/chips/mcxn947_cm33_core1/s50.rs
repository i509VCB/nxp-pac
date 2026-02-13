#[doc = "no description available"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct S50 {
    ptr: *mut u8,
}
unsafe impl Send for S50 {}
unsafe impl Sync for S50 {}
impl S50 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Control Register"]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Command Configuration"]
    #[inline(always)]
    pub const fn cmdcfg0(self) -> crate::common::Reg<regs::Cmdcfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Configuration Register"]
    #[inline(always)]
    pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Keystore Index 0"]
    #[inline(always)]
    pub const fn kidx0(self) -> crate::common::Reg<regs::Kidx0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Keystore Index 1"]
    #[inline(always)]
    pub const fn kidx1(self) -> crate::common::Reg<regs::Kidx1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Key Properties Request"]
    #[inline(always)]
    pub const fn kpropin(self) -> crate::common::Reg<regs::Kpropin, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "DMA Source 0"]
    #[inline(always)]
    pub const fn dma_src0(self) -> crate::common::Reg<regs::DmaSrc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "DMA Source 0 Length"]
    #[inline(always)]
    pub const fn dma_src0_len(self) -> crate::common::Reg<regs::DmaSrc0Len, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "DMA Source 1"]
    #[inline(always)]
    pub const fn dma_src1(self) -> crate::common::Reg<regs::DmaSrc1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "DMA Source 2"]
    #[inline(always)]
    pub const fn dma_src2(self) -> crate::common::Reg<regs::DmaSrc2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "DMA Source 2 Length"]
    #[inline(always)]
    pub const fn dma_src2_len(self) -> crate::common::Reg<regs::DmaSrc2Len, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "DMA Result 0"]
    #[inline(always)]
    pub const fn dma_res0(self) -> crate::common::Reg<regs::DmaRes0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "DMA Result 0 Length"]
    #[inline(always)]
    pub const fn dma_res0_len(self) -> crate::common::Reg<regs::DmaRes0Len, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub const fn int_enable(self) -> crate::common::Reg<regs::IntEnable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Interrupt Status Clear"]
    #[inline(always)]
    pub const fn int_status_clr(self) -> crate::common::Reg<regs::IntStatusClr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Interrupt Status Set"]
    #[inline(always)]
    pub const fn int_status_set(self) -> crate::common::Reg<regs::IntStatusSet, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Error Status"]
    #[inline(always)]
    pub const fn err_status(self) -> crate::common::Reg<regs::ErrStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Error Status Clear"]
    #[inline(always)]
    pub const fn err_status_clr(self) -> crate::common::Reg<regs::ErrStatusClr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Version Register"]
    #[inline(always)]
    pub const fn version(self) -> crate::common::Reg<regs::Version, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "PRNG SW Read Out"]
    #[inline(always)]
    pub const fn prng_datout(self) -> crate::common::Reg<regs::PrngDatout, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "CRC Configuration"]
    #[inline(always)]
    pub const fn cmdcrc_ctrl(self) -> crate::common::Reg<regs::CmdcrcCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Command CRC Value"]
    #[inline(always)]
    pub const fn cmdcrc(self) -> crate::common::Reg<regs::Cmdcrc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Session ID"]
    #[inline(always)]
    pub const fn session_id(self) -> crate::common::Reg<regs::SessionId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "Final DMA Address"]
    #[inline(always)]
    pub const fn dma_fin_addr(self) -> crate::common::Reg<regs::DmaFinAddr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Master ID"]
    #[inline(always)]
    pub const fn master_id(self) -> crate::common::Reg<regs::MasterId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "Keystore Index 2"]
    #[inline(always)]
    pub const fn kidx2(self) -> crate::common::Reg<regs::Kidx2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn els_ks0(self) -> crate::common::Reg<regs::ElsKs0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn els_ks1(self) -> crate::common::Reg<regs::ElsKs1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn els_ks2(self) -> crate::common::Reg<regs::ElsKs2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn els_ks3(self) -> crate::common::Reg<regs::ElsKs3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn els_ks4(self) -> crate::common::Reg<regs::ElsKs4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn els_ks5(self) -> crate::common::Reg<regs::ElsKs5, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn els_ks6(self) -> crate::common::Reg<regs::ElsKs6, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn els_ks7(self) -> crate::common::Reg<regs::ElsKs7, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn els_ks8(self) -> crate::common::Reg<regs::ElsKs8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn els_ks9(self) -> crate::common::Reg<regs::ElsKs9, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn els_ks10(self) -> crate::common::Reg<regs::ElsKs10, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn els_ks11(self) -> crate::common::Reg<regs::ElsKs11, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x017cusize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn els_ks12(self) -> crate::common::Reg<regs::ElsKs12, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn els_ks13(self) -> crate::common::Reg<regs::ElsKs13, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn els_ks14(self) -> crate::common::Reg<regs::ElsKs14, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn els_ks15(self) -> crate::common::Reg<regs::ElsKs15, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn els_ks16(self) -> crate::common::Reg<regs::ElsKs16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn els_ks17(self) -> crate::common::Reg<regs::ElsKs17, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn els_ks18(self) -> crate::common::Reg<regs::ElsKs18, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn els_ks19(self) -> crate::common::Reg<regs::ElsKs19, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x019cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
