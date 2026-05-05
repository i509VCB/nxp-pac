#[doc = "ANALOGCTRL."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ANACTRL {
    ptr: *mut u8,
}
unsafe impl Send for ANACTRL {}
unsafe impl Sync for ANACTRL {}
impl ANACTRL {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Various Analog blocks configuration (like FRO 192MHz trimmings source ...)."]
    #[inline(always)]
    pub const fn ANALOG_CTRL_CFG(
        self,
    ) -> crate::common::Reg<regs::ANALOG_CTRL_CFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Analog Macroblock Identity registers, Flash Status registers."]
    #[inline(always)]
    pub const fn ANALOG_CTRL_STATUS(
        self,
    ) -> crate::common::Reg<regs::ANALOG_CTRL_STATUS, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Frequency Measure function control register."]
    #[inline(always)]
    pub const fn FREQ_ME_CTRL(self) -> crate::common::Reg<regs::FREQ_ME_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "192MHz Free Running OScillator (FRO) Control register."]
    #[inline(always)]
    pub const fn FRO192M_CTRL(self) -> crate::common::Reg<regs::FRO192M_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "192MHz Free Running OScillator (FRO) Status register."]
    #[inline(always)]
    pub const fn FRO192M_STATUS(
        self,
    ) -> crate::common::Reg<regs::FRO192M_STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "General Purpose ADC VBAT Divider branch control."]
    #[inline(always)]
    pub const fn ADC_CTRL(self) -> crate::common::Reg<regs::ADC_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "High speed Crystal Oscillator Control register."]
    #[inline(always)]
    pub const fn XO32M_CTRL(self) -> crate::common::Reg<regs::XO32M_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "High speed Crystal Oscillator Status register."]
    #[inline(always)]
    pub const fn XO32M_STATUS(self) -> crate::common::Reg<regs::XO32M_STATUS, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Brown Out Detectors (BoDs) & DCDC interrupts generation control register."]
    #[inline(always)]
    pub const fn BOD_DCDC_INT_CTRL(
        self,
    ) -> crate::common::Reg<regs::BOD_DCDC_INT_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "BoDs & DCDC interrupts status register."]
    #[inline(always)]
    pub const fn BOD_DCDC_INT_STATUS(
        self,
    ) -> crate::common::Reg<regs::BOD_DCDC_INT_STATUS, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "First Ring Oscillator module control register."]
    #[inline(always)]
    pub const fn RINGO0_CTRL(self) -> crate::common::Reg<regs::RINGO0_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Second Ring Oscillator module control register."]
    #[inline(always)]
    pub const fn RINGO1_CTRL(self) -> crate::common::Reg<regs::RINGO1_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Third Ring Oscillator module control register."]
    #[inline(always)]
    pub const fn RINGO2_CTRL(self) -> crate::common::Reg<regs::RINGO2_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "High Speed Crystal Oscillator (12 MHz - 32 MHz) Voltage Source Supply Control register."]
    #[inline(always)]
    pub const fn LDO_XO32M(self) -> crate::common::Reg<regs::LDO_XO32M, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "AUX_BIAS."]
    #[inline(always)]
    pub const fn AUX_BIAS(self) -> crate::common::Reg<regs::AUX_BIAS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "Dummy Control bus to analog modules."]
    #[inline(always)]
    pub const fn DUMMY_CTRL(self) -> crate::common::Reg<regs::DUMMY_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "USB High Speed Phy Control."]
    #[inline(always)]
    pub const fn USBHS_PHY_CTRL(
        self,
    ) -> crate::common::Reg<regs::USBHS_PHY_CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "USB High Speed Phy Trim values."]
    #[inline(always)]
    pub const fn USBHS_PHY_TRIM(
        self,
    ) -> crate::common::Reg<regs::USBHS_PHY_TRIM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
}
pub mod regs;
pub mod vals;
