#[doc = "EMVSIM"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Emvsim {
    ptr: *mut u8,
}
unsafe impl Send for Emvsim {}
unsafe impl Sync for Emvsim {}
impl Emvsim {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Version ID"]
    #[inline(always)]
    pub const fn ver_id(self) -> crate::common::Reg<regs::VerId, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Parameters"]
    #[inline(always)]
    pub const fn param(self) -> crate::common::Reg<regs::Param, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Clock Configuration"]
    #[inline(always)]
    pub const fn clkcfg(self) -> crate::common::Reg<regs::Clkcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Baud Rate Divisor"]
    #[inline(always)]
    pub const fn divisor(self) -> crate::common::Reg<regs::Divisor, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Control"]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Interrupt Mask"]
    #[inline(always)]
    pub const fn int_mask(self) -> crate::common::Reg<regs::IntMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Receiver Threshold"]
    #[inline(always)]
    pub const fn rx_thd(self) -> crate::common::Reg<regs::RxThd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Transmitter Threshold"]
    #[inline(always)]
    pub const fn tx_thd(self) -> crate::common::Reg<regs::TxThd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Receive Status"]
    #[inline(always)]
    pub const fn rx_status(self) -> crate::common::Reg<regs::RxStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Transmitter Status"]
    #[inline(always)]
    pub const fn tx_status(self) -> crate::common::Reg<regs::TxStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Port Control and Status"]
    #[inline(always)]
    pub const fn pcsr(self) -> crate::common::Reg<regs::Pcsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Receive Data Read Buffer"]
    #[inline(always)]
    pub const fn rx_buf(self) -> crate::common::Reg<regs::RxBuf, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Transmit Data Buffer"]
    #[inline(always)]
    pub const fn tx_buf(self) -> crate::common::Reg<regs::TxBuf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Transmitter Guard ETU Value"]
    #[inline(always)]
    pub const fn tx_getu(self) -> crate::common::Reg<regs::TxGetu, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Character Wait Time Value"]
    #[inline(always)]
    pub const fn cwt_val(self) -> crate::common::Reg<regs::CwtVal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Block Wait Time Value"]
    #[inline(always)]
    pub const fn bwt_val(self) -> crate::common::Reg<regs::BwtVal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Block Guard Time Value"]
    #[inline(always)]
    pub const fn bgt_val(self) -> crate::common::Reg<regs::BgtVal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "General Purpose Counter 0 Timeout Value"]
    #[inline(always)]
    pub const fn gpcnt0_val(self) -> crate::common::Reg<regs::Gpcnt0Val, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "General Purpose Counter 1 Timeout Value"]
    #[inline(always)]
    pub const fn gpcnt1_val(self) -> crate::common::Reg<regs::Gpcnt1Val, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
}
pub mod regs;
pub mod vals;
