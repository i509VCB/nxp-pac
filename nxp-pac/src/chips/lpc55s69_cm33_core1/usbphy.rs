#[doc = "Universal System Bus Physical Layer"]
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
    #[doc = "USB PHY PLL Control/Status Register"]
    #[inline(always)]
    pub const fn pll_sic(self) -> crate::common::Reg<regs::PllSic, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "USB PHY PLL Control/Status Register"]
    #[inline(always)]
    pub const fn pll_sic_set(self) -> crate::common::Reg<regs::PllSicSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "USB PHY PLL Control/Status Register"]
    #[inline(always)]
    pub const fn pll_sic_clr(self) -> crate::common::Reg<regs::PllSicClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "USB PHY PLL Control/Status Register"]
    #[inline(always)]
    pub const fn pll_sic_tog(self) -> crate::common::Reg<regs::PllSicTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "USB PHY VBUS Detect Control Register"]
    #[inline(always)]
    pub const fn usb1_vbus_detect(
        self,
    ) -> crate::common::Reg<regs::Usb1VbusDetect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "USB PHY VBUS Detect Control Register"]
    #[inline(always)]
    pub const fn usb1_vbus_detect_set(
        self,
    ) -> crate::common::Reg<regs::Usb1VbusDetectSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "USB PHY VBUS Detect Control Register"]
    #[inline(always)]
    pub const fn usb1_vbus_detect_clr(
        self,
    ) -> crate::common::Reg<regs::Usb1VbusDetectClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "USB PHY VBUS Detect Control Register"]
    #[inline(always)]
    pub const fn usb1_vbus_detect_tog(
        self,
    ) -> crate::common::Reg<regs::Usb1VbusDetectTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "USB PHY Analog Control Register"]
    #[inline(always)]
    pub const fn anactrl(self) -> crate::common::Reg<regs::Anactrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "USB PHY Analog Control Register"]
    #[inline(always)]
    pub const fn anactrl_set(self) -> crate::common::Reg<regs::AnactrlSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "USB PHY Analog Control Register"]
    #[inline(always)]
    pub const fn anactrl_clr(self) -> crate::common::Reg<regs::AnactrlClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "USB PHY Analog Control Register"]
    #[inline(always)]
    pub const fn anactrl_tog(self) -> crate::common::Reg<regs::AnactrlTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
