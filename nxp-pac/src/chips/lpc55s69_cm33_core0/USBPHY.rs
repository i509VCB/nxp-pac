#[doc = "Universal System Bus Physical Layer."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USBPHY {
    ptr: *mut u8,
}
unsafe impl Send for USBPHY {}
unsafe impl Sync for USBPHY {}
impl USBPHY {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "USB PHY Power-Down Register."]
    #[inline(always)]
    pub const fn PWD(self) -> crate::common::Reg<regs::PWD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "USB PHY Power-Down Register."]
    #[inline(always)]
    pub const fn PWD_SET(self) -> crate::common::Reg<regs::PWD_SET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "USB PHY Power-Down Register."]
    #[inline(always)]
    pub const fn PWD_CLR(self) -> crate::common::Reg<regs::PWD_CLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "USB PHY Power-Down Register."]
    #[inline(always)]
    pub const fn PWD_TOG(self) -> crate::common::Reg<regs::PWD_TOG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "USB PHY Transmitter Control Register."]
    #[inline(always)]
    pub const fn TX(self) -> crate::common::Reg<regs::TX, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "USB PHY Transmitter Control Register."]
    #[inline(always)]
    pub const fn TX_SET(self) -> crate::common::Reg<regs::TX_SET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "USB PHY Transmitter Control Register."]
    #[inline(always)]
    pub const fn TX_CLR(self) -> crate::common::Reg<regs::TX_CLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "USB PHY Transmitter Control Register."]
    #[inline(always)]
    pub const fn TX_TOG(self) -> crate::common::Reg<regs::TX_TOG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "USB PHY Receiver Control Register."]
    #[inline(always)]
    pub const fn RX(self) -> crate::common::Reg<regs::RX, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "USB PHY Receiver Control Register."]
    #[inline(always)]
    pub const fn RX_SET(self) -> crate::common::Reg<regs::RX_SET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "USB PHY Receiver Control Register."]
    #[inline(always)]
    pub const fn RX_CLR(self) -> crate::common::Reg<regs::RX_CLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "USB PHY Receiver Control Register."]
    #[inline(always)]
    pub const fn RX_TOG(self) -> crate::common::Reg<regs::RX_TOG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "USB PHY General Control Register."]
    #[inline(always)]
    pub const fn CTRL(self) -> crate::common::Reg<regs::CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "USB PHY General Control Register."]
    #[inline(always)]
    pub const fn CTRL_SET(self) -> crate::common::Reg<regs::CTRL_SET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "USB PHY General Control Register."]
    #[inline(always)]
    pub const fn CTRL_CLR(self) -> crate::common::Reg<regs::CTRL_CLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "USB PHY General Control Register."]
    #[inline(always)]
    pub const fn CTRL_TOG(self) -> crate::common::Reg<regs::CTRL_TOG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "USB PHY Status Register."]
    #[inline(always)]
    pub const fn STATUS(self) -> crate::common::Reg<regs::STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "USB PHY PLL Control/Status Register."]
    #[inline(always)]
    pub const fn PLL_SIC(self) -> crate::common::Reg<regs::PLL_SIC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "USB PHY PLL Control/Status Register."]
    #[inline(always)]
    pub const fn PLL_SIC_SET(self) -> crate::common::Reg<regs::PLL_SIC_SET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "USB PHY PLL Control/Status Register."]
    #[inline(always)]
    pub const fn PLL_SIC_CLR(self) -> crate::common::Reg<regs::PLL_SIC_CLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "USB PHY PLL Control/Status Register."]
    #[inline(always)]
    pub const fn PLL_SIC_TOG(self) -> crate::common::Reg<regs::PLL_SIC_TOG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "USB PHY VBUS Detect Control Register."]
    #[inline(always)]
    pub const fn USB1_VBUS_DETECT(
        self,
    ) -> crate::common::Reg<regs::USB1_VBUS_DETECT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "USB PHY VBUS Detect Control Register."]
    #[inline(always)]
    pub const fn USB1_VBUS_DETECT_SET(
        self,
    ) -> crate::common::Reg<regs::USB1_VBUS_DETECT_SET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "USB PHY VBUS Detect Control Register."]
    #[inline(always)]
    pub const fn USB1_VBUS_DETECT_CLR(
        self,
    ) -> crate::common::Reg<regs::USB1_VBUS_DETECT_CLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "USB PHY VBUS Detect Control Register."]
    #[inline(always)]
    pub const fn USB1_VBUS_DETECT_TOG(
        self,
    ) -> crate::common::Reg<regs::USB1_VBUS_DETECT_TOG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "USB PHY Analog Control Register."]
    #[inline(always)]
    pub const fn ANACTRL(self) -> crate::common::Reg<regs::ANACTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "USB PHY Analog Control Register."]
    #[inline(always)]
    pub const fn ANACTRL_SET(self) -> crate::common::Reg<regs::ANACTRL_SET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "USB PHY Analog Control Register."]
    #[inline(always)]
    pub const fn ANACTRL_CLR(self) -> crate::common::Reg<regs::ANACTRL_CLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "USB PHY Analog Control Register."]
    #[inline(always)]
    pub const fn ANACTRL_TOG(self) -> crate::common::Reg<regs::ANACTRL_TOG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
