#[doc = "USBPHY Register Reference Index"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbphy {
    ptr: *mut u8,
}
unsafe impl Send for Usbphy {}
unsafe impl Sync for Usbphy {}
impl Usbphy {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "USB PHY Power-Down Register"]
    #[inline(always)]
    pub const fn pwd(self) -> crate::common::Reg<regs::Pwd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "USB PHY Power-Down Register"]
    #[inline(always)]
    pub const fn pwd_set(self) -> crate::common::Reg<regs::PwdSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "USB PHY Power-Down Register"]
    #[inline(always)]
    pub const fn pwd_clr(self) -> crate::common::Reg<regs::PwdClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "USB PHY Power-Down Register"]
    #[inline(always)]
    pub const fn pwd_tog(self) -> crate::common::Reg<regs::PwdTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "USB PHY Transmitter Control Register"]
    #[inline(always)]
    pub const fn tx(self) -> crate::common::Reg<regs::Tx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "USB PHY Transmitter Control Register"]
    #[inline(always)]
    pub const fn tx_set(self) -> crate::common::Reg<regs::TxSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "USB PHY Transmitter Control Register"]
    #[inline(always)]
    pub const fn tx_clr(self) -> crate::common::Reg<regs::TxClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "USB PHY Transmitter Control Register"]
    #[inline(always)]
    pub const fn tx_tog(self) -> crate::common::Reg<regs::TxTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "USB PHY Receiver Control Register"]
    #[inline(always)]
    pub const fn rx(self) -> crate::common::Reg<regs::Rx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "USB PHY Receiver Control Register"]
    #[inline(always)]
    pub const fn rx_set(self) -> crate::common::Reg<regs::RxSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "USB PHY Receiver Control Register"]
    #[inline(always)]
    pub const fn rx_clr(self) -> crate::common::Reg<regs::RxClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "USB PHY Receiver Control Register"]
    #[inline(always)]
    pub const fn rx_tog(self) -> crate::common::Reg<regs::RxTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "USB PHY General Control Register"]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "USB PHY General Control Register"]
    #[inline(always)]
    pub const fn ctrl_set(self) -> crate::common::Reg<regs::CtrlSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "USB PHY General Control Register"]
    #[inline(always)]
    pub const fn ctrl_clr(self) -> crate::common::Reg<regs::CtrlClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "USB PHY General Control Register"]
    #[inline(always)]
    pub const fn ctrl_tog(self) -> crate::common::Reg<regs::CtrlTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "USB PHY Status Register"]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "USB PHY Debug Register"]
    #[inline(always)]
    pub const fn debug(self) -> crate::common::Reg<regs::Debug, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "USB PHY Debug Register"]
    #[inline(always)]
    pub const fn debug_set(self) -> crate::common::Reg<regs::DebugSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "USB PHY Debug Register"]
    #[inline(always)]
    pub const fn debug_clr(self) -> crate::common::Reg<regs::DebugClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "USB PHY Debug Register"]
    #[inline(always)]
    pub const fn debug_tog(self) -> crate::common::Reg<regs::DebugTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "UTMI Debug Status Register 0"]
    #[inline(always)]
    pub const fn debug0_status(self) -> crate::common::Reg<regs::Debug0Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "UTMI Debug Status Register 1"]
    #[inline(always)]
    pub const fn debug1(self) -> crate::common::Reg<regs::Debug1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "UTMI Debug Status Register 1"]
    #[inline(always)]
    pub const fn debug1_set(self) -> crate::common::Reg<regs::Debug1Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "UTMI Debug Status Register 1"]
    #[inline(always)]
    pub const fn debug1_clr(self) -> crate::common::Reg<regs::Debug1Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "UTMI Debug Status Register 1"]
    #[inline(always)]
    pub const fn debug1_tog(self) -> crate::common::Reg<regs::Debug1Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "UTMI RTL Version"]
    #[inline(always)]
    pub const fn version(self) -> crate::common::Reg<regs::Version, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
}
pub mod regs;
