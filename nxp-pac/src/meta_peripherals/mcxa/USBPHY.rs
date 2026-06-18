#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "Universal Serial Bus 2.0 Integrated PHY."]
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
    #[doc = "Power Down."]
    #[inline(always)]
    pub const fn PWD(self) -> crate::pac::common::Reg<PWD, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Power Down."]
    #[inline(always)]
    pub const fn PWD_SET(self) -> crate::pac::common::Reg<PWD_SET, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Power Down."]
    #[inline(always)]
    pub const fn PWD_CLR(self) -> crate::pac::common::Reg<PWD_CLR, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Power Down."]
    #[inline(always)]
    pub const fn PWD_TOG(self) -> crate::pac::common::Reg<PWD_TOG, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "TX Control."]
    #[inline(always)]
    pub const fn TX(self) -> crate::pac::common::Reg<TX, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "TX Control."]
    #[inline(always)]
    pub const fn TX_SET(self) -> crate::pac::common::Reg<TX_SET, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "TX Control."]
    #[inline(always)]
    pub const fn TX_CLR(self) -> crate::pac::common::Reg<TX_CLR, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "TX Control."]
    #[inline(always)]
    pub const fn TX_TOG(self) -> crate::pac::common::Reg<TX_TOG, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "RX Control."]
    #[inline(always)]
    pub const fn RX(self) -> crate::pac::common::Reg<RX, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "RX Control."]
    #[inline(always)]
    pub const fn RX_SET(self) -> crate::pac::common::Reg<RX_SET, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "RX Control."]
    #[inline(always)]
    pub const fn RX_CLR(self) -> crate::pac::common::Reg<RX_CLR, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "RX Control."]
    #[inline(always)]
    pub const fn RX_TOG(self) -> crate::pac::common::Reg<RX_TOG, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "General Purpose Control."]
    #[inline(always)]
    pub const fn CTRL(self) -> crate::pac::common::Reg<CTRL, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "General Purpose Control."]
    #[inline(always)]
    pub const fn CTRL_SET(self) -> crate::pac::common::Reg<CTRL_SET, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "General Purpose Control."]
    #[inline(always)]
    pub const fn CTRL_CLR(self) -> crate::pac::common::Reg<CTRL_CLR, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "General Purpose Control."]
    #[inline(always)]
    pub const fn CTRL_TOG(self) -> crate::pac::common::Reg<CTRL_TOG, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Status."]
    #[inline(always)]
    pub const fn STATUS(self) -> crate::pac::common::Reg<STATUS, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Debug 0."]
    #[inline(always)]
    pub const fn DEBUG0(self) -> crate::pac::common::Reg<DEBUG0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Debug 0."]
    #[inline(always)]
    pub const fn DEBUG0_SET(self) -> crate::pac::common::Reg<DEBUG0_SET, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Debug 0."]
    #[inline(always)]
    pub const fn DEBUG0_CLR(self) -> crate::pac::common::Reg<DEBUG0_CLR, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Debug 0."]
    #[inline(always)]
    pub const fn DEBUG0_TOG(self) -> crate::pac::common::Reg<DEBUG0_TOG, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "Version."]
    #[inline(always)]
    pub const fn VERSION(self) -> crate::pac::common::Reg<VERSION, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "IP Block."]
    #[inline(always)]
    pub const fn IP(self) -> crate::pac::common::Reg<IP, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "IP Block."]
    #[inline(always)]
    pub const fn IP_SET(self) -> crate::pac::common::Reg<IP_SET, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "IP Block."]
    #[inline(always)]
    pub const fn IP_CLR(self) -> crate::pac::common::Reg<IP_CLR, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "IP Block."]
    #[inline(always)]
    pub const fn IP_TOG(self) -> crate::pac::common::Reg<IP_TOG, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "PLL SIC."]
    #[inline(always)]
    pub const fn PLL_SIC(self) -> crate::pac::common::Reg<PLL_SIC, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "PLL SIC."]
    #[inline(always)]
    pub const fn PLL_SIC_SET(self) -> crate::pac::common::Reg<PLL_SIC_SET, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "PLL SIC."]
    #[inline(always)]
    pub const fn PLL_SIC_CLR(self) -> crate::pac::common::Reg<PLL_SIC_CLR, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "PLL SIC."]
    #[inline(always)]
    pub const fn PLL_SIC_TOG(self) -> crate::pac::common::Reg<PLL_SIC_TOG, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "VBUS Detect."]
    #[inline(always)]
    pub const fn USB1_VBUS_DETECT(
        self,
    ) -> crate::pac::common::Reg<USB1_VBUS_DETECT, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "VBUS Detect."]
    #[inline(always)]
    pub const fn USB1_VBUS_DETECT_SET(
        self,
    ) -> crate::pac::common::Reg<USB1_VBUS_DETECT_SET, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "VBUS Detect."]
    #[inline(always)]
    pub const fn USB1_VBUS_DETECT_CLR(
        self,
    ) -> crate::pac::common::Reg<USB1_VBUS_DETECT_CLR, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "VBUS Detect."]
    #[inline(always)]
    pub const fn USB1_VBUS_DETECT_TOG(
        self,
    ) -> crate::pac::common::Reg<USB1_VBUS_DETECT_TOG, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "VBUS Detect Status."]
    #[inline(always)]
    pub const fn USB1_VBUS_DET_STAT(
        self,
    ) -> crate::pac::common::Reg<USB1_VBUS_DET_STAT, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "VBUS Detect Status."]
    #[inline(always)]
    pub const fn USB1_VBUS_DET_STAT_SET(
        self,
    ) -> crate::pac::common::Reg<USB1_VBUS_DET_STAT_SET, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "VBUS Detect Status."]
    #[inline(always)]
    pub const fn USB1_VBUS_DET_STAT_CLR(
        self,
    ) -> crate::pac::common::Reg<USB1_VBUS_DET_STAT_CLR, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "VBUS Detect Status."]
    #[inline(always)]
    pub const fn USB1_VBUS_DET_STAT_TOG(
        self,
    ) -> crate::pac::common::Reg<USB1_VBUS_DET_STAT_TOG, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "Charger Detect."]
    #[inline(always)]
    pub const fn USB1_CHRG_DETECT(
        self,
    ) -> crate::pac::common::Reg<USB1_CHRG_DETECT, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "Charger Detect."]
    #[inline(always)]
    pub const fn USB1_CHRG_DETECT_SET(
        self,
    ) -> crate::pac::common::Reg<USB1_CHRG_DETECT_SET, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Charger Detect."]
    #[inline(always)]
    pub const fn USB1_CHRG_DETECT_CLR(
        self,
    ) -> crate::pac::common::Reg<USB1_CHRG_DETECT_CLR, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "Charger Detect."]
    #[inline(always)]
    pub const fn USB1_CHRG_DETECT_TOG(
        self,
    ) -> crate::pac::common::Reg<USB1_CHRG_DETECT_TOG, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "Charger Detect Status."]
    #[inline(always)]
    pub const fn USB1_CHRG_DET_STAT(
        self,
    ) -> crate::pac::common::Reg<USB1_CHRG_DET_STAT, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "Charger Detect Status."]
    #[inline(always)]
    pub const fn USB1_CHRG_DET_STAT_SET(
        self,
    ) -> crate::pac::common::Reg<USB1_CHRG_DET_STAT_SET, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "Charger Detect Status."]
    #[inline(always)]
    pub const fn USB1_CHRG_DET_STAT_CLR(
        self,
    ) -> crate::pac::common::Reg<USB1_CHRG_DET_STAT_CLR, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "Charger Detect Status."]
    #[inline(always)]
    pub const fn USB1_CHRG_DET_STAT_TOG(
        self,
    ) -> crate::pac::common::Reg<USB1_CHRG_DET_STAT_TOG, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "Analog Control."]
    #[inline(always)]
    pub const fn ANACTRL(self) -> crate::pac::common::Reg<ANACTRL, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Analog Control."]
    #[inline(always)]
    pub const fn ANACTRL_SET(self) -> crate::pac::common::Reg<ANACTRL_SET, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Analog Control."]
    #[inline(always)]
    pub const fn ANACTRL_CLR(self) -> crate::pac::common::Reg<ANACTRL_CLR, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Analog Control."]
    #[inline(always)]
    pub const fn ANACTRL_TOG(self) -> crate::pac::common::Reg<ANACTRL_TOG, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Trim."]
    #[inline(always)]
    pub const fn TRIM_OVERRIDE_EN(
        self,
    ) -> crate::pac::common::Reg<TRIM_OVERRIDE_EN, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "Trim."]
    #[inline(always)]
    pub const fn TRIM_OVERRIDE_EN_SET(
        self,
    ) -> crate::pac::common::Reg<TRIM_OVERRIDE_EN_SET, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "Trim."]
    #[inline(always)]
    pub const fn TRIM_OVERRIDE_EN_CLR(
        self,
    ) -> crate::pac::common::Reg<TRIM_OVERRIDE_EN_CLR, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "Trim."]
    #[inline(always)]
    pub const fn TRIM_OVERRIDE_EN_TOG(
        self,
    ) -> crate::pac::common::Reg<TRIM_OVERRIDE_EN_TOG, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "PFD A."]
    #[inline(always)]
    pub const fn PFDA(self) -> crate::pac::common::Reg<PFDA, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "PFD A."]
    #[inline(always)]
    pub const fn PFDA_SET(self) -> crate::pac::common::Reg<PFDA_SET, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "PFD A."]
    #[inline(always)]
    pub const fn PFDA_CLR(self) -> crate::pac::common::Reg<PFDA_CLR, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "PFD A."]
    #[inline(always)]
    pub const fn PFDA_TOG(self) -> crate::pac::common::Reg<PFDA_TOG, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
}
#[doc = "Analog Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ANACTRL(pub u32);
impl ANACTRL {
    #[doc = "Internal Low Voltage Detector Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn LVI_EN(&self) -> LVI_EN {
        let val = (self.0 >> 1usize) & 0x01;
        LVI_EN::from_bits(val as u8)
    }
    #[doc = "Internal Low Voltage Detector Enable."]
    #[inline(always)]
    pub const fn set_LVI_EN(&mut self, val: LVI_EN) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "PFD Clock Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn PFD_CLK_SEL(&self) -> PFD_CLK_SEL {
        let val = (self.0 >> 2usize) & 0x03;
        PFD_CLK_SEL::from_bits(val as u8)
    }
    #[doc = "PFD Clock Selection."]
    #[inline(always)]
    pub const fn set_PFD_CLK_SEL(&mut self, val: PFD_CLK_SEL) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Device Pulldown Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn DEV_PULLDOWN(&self) -> DEV_PULLDOWN {
        let val = (self.0 >> 10usize) & 0x01;
        DEV_PULLDOWN::from_bits(val as u8)
    }
    #[doc = "Device Pulldown Enable."]
    #[inline(always)]
    pub const fn set_DEV_PULLDOWN(&mut self, val: DEV_PULLDOWN) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for ANACTRL {
    #[inline(always)]
    fn default() -> ANACTRL {
        ANACTRL(0)
    }
}
impl core::fmt::Debug for ANACTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ANACTRL")
            .field("LVI_EN", &self.LVI_EN())
            .field("PFD_CLK_SEL", &self.PFD_CLK_SEL())
            .field("DEV_PULLDOWN", &self.DEV_PULLDOWN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ANACTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ANACTRL {{ LVI_EN: {:?}, PFD_CLK_SEL: {:?}, DEV_PULLDOWN: {:?} }}",
            self.LVI_EN(),
            self.PFD_CLK_SEL(),
            self.DEV_PULLDOWN()
        )
    }
}
#[doc = "Analog Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ANACTRL_CLR(pub u32);
impl ANACTRL_CLR {
    #[doc = "Internal Low Voltage Detector Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn LVI_EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Internal Low Voltage Detector Enable."]
    #[inline(always)]
    pub const fn set_LVI_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "PFD Clock Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn PFD_CLK_SEL(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "PFD Clock Selection."]
    #[inline(always)]
    pub const fn set_PFD_CLK_SEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Device Pulldown Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn DEV_PULLDOWN(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Device Pulldown Enable."]
    #[inline(always)]
    pub const fn set_DEV_PULLDOWN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for ANACTRL_CLR {
    #[inline(always)]
    fn default() -> ANACTRL_CLR {
        ANACTRL_CLR(0)
    }
}
impl core::fmt::Debug for ANACTRL_CLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ANACTRL_CLR")
            .field("LVI_EN", &self.LVI_EN())
            .field("PFD_CLK_SEL", &self.PFD_CLK_SEL())
            .field("DEV_PULLDOWN", &self.DEV_PULLDOWN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ANACTRL_CLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ANACTRL_CLR {{ LVI_EN: {=bool:?}, PFD_CLK_SEL: {=u8:?}, DEV_PULLDOWN: {=bool:?} }}",
            self.LVI_EN(),
            self.PFD_CLK_SEL(),
            self.DEV_PULLDOWN()
        )
    }
}
#[doc = "Analog Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ANACTRL_SET(pub u32);
impl ANACTRL_SET {
    #[doc = "Internal Low Voltage Detector Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn LVI_EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Internal Low Voltage Detector Enable."]
    #[inline(always)]
    pub const fn set_LVI_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "PFD Clock Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn PFD_CLK_SEL(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "PFD Clock Selection."]
    #[inline(always)]
    pub const fn set_PFD_CLK_SEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Device Pulldown Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn DEV_PULLDOWN(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Device Pulldown Enable."]
    #[inline(always)]
    pub const fn set_DEV_PULLDOWN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for ANACTRL_SET {
    #[inline(always)]
    fn default() -> ANACTRL_SET {
        ANACTRL_SET(0)
    }
}
impl core::fmt::Debug for ANACTRL_SET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ANACTRL_SET")
            .field("LVI_EN", &self.LVI_EN())
            .field("PFD_CLK_SEL", &self.PFD_CLK_SEL())
            .field("DEV_PULLDOWN", &self.DEV_PULLDOWN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ANACTRL_SET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ANACTRL_SET {{ LVI_EN: {=bool:?}, PFD_CLK_SEL: {=u8:?}, DEV_PULLDOWN: {=bool:?} }}",
            self.LVI_EN(),
            self.PFD_CLK_SEL(),
            self.DEV_PULLDOWN()
        )
    }
}
#[doc = "Analog Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ANACTRL_TOG(pub u32);
impl ANACTRL_TOG {
    #[doc = "Internal Low Voltage Detector Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn LVI_EN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Internal Low Voltage Detector Enable."]
    #[inline(always)]
    pub const fn set_LVI_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "PFD Clock Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn PFD_CLK_SEL(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "PFD Clock Selection."]
    #[inline(always)]
    pub const fn set_PFD_CLK_SEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Device Pulldown Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn DEV_PULLDOWN(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Device Pulldown Enable."]
    #[inline(always)]
    pub const fn set_DEV_PULLDOWN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for ANACTRL_TOG {
    #[inline(always)]
    fn default() -> ANACTRL_TOG {
        ANACTRL_TOG(0)
    }
}
impl core::fmt::Debug for ANACTRL_TOG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ANACTRL_TOG")
            .field("LVI_EN", &self.LVI_EN())
            .field("PFD_CLK_SEL", &self.PFD_CLK_SEL())
            .field("DEV_PULLDOWN", &self.DEV_PULLDOWN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ANACTRL_TOG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ANACTRL_TOG {{ LVI_EN: {=bool:?}, PFD_CLK_SEL: {=u8:?}, DEV_PULLDOWN: {=bool:?} }}",
            self.LVI_EN(),
            self.PFD_CLK_SEL(),
            self.DEV_PULLDOWN()
        )
    }
}
#[doc = "General Purpose Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTRL(pub u32);
impl CTRL {
    #[doc = "OTG ID Change Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ENOTG_ID_CHG_IRQ(&self) -> ENOTG_ID_CHG_IRQ {
        let val = (self.0 >> 0usize) & 0x01;
        ENOTG_ID_CHG_IRQ::from_bits(val as u8)
    }
    #[doc = "OTG ID Change Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ENOTG_ID_CHG_IRQ(&mut self, val: ENOTG_ID_CHG_IRQ) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Host Disconnect Detection Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ENHOSTDISCONDETECT(&self) -> ENHOSTDISCONDETECT {
        let val = (self.0 >> 1usize) & 0x01;
        ENHOSTDISCONDETECT::from_bits(val as u8)
    }
    #[doc = "Host Disconnect Detection Enable."]
    #[inline(always)]
    pub const fn set_ENHOSTDISCONDETECT(&mut self, val: ENHOSTDISCONDETECT) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable Interrupt for Host Disconnect."]
    #[must_use]
    #[inline(always)]
    pub const fn ENIRQHOSTDISCON(&self) -> ENIRQHOSTDISCON {
        let val = (self.0 >> 2usize) & 0x01;
        ENIRQHOSTDISCON::from_bits(val as u8)
    }
    #[doc = "Enable Interrupt for Host Disconnect."]
    #[inline(always)]
    pub const fn set_ENIRQHOSTDISCON(&mut self, val: ENIRQHOSTDISCON) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Host Disconnect Detection Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn HOSTDISCONDETECT_IRQ(&self) -> HOSTDISCONDETECT_IRQ {
        let val = (self.0 >> 3usize) & 0x01;
        HOSTDISCONDETECT_IRQ::from_bits(val as u8)
    }
    #[doc = "Host Disconnect Detection Interrupt."]
    #[inline(always)]
    pub const fn set_HOSTDISCONDETECT_IRQ(&mut self, val: HOSTDISCONDETECT_IRQ) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable Nonstandard Resistive Plugged-In Detection."]
    #[must_use]
    #[inline(always)]
    pub const fn ENDEVPLUGINDETECT(&self) -> ENDEVPLUGINDETECT {
        let val = (self.0 >> 4usize) & 0x01;
        ENDEVPLUGINDETECT::from_bits(val as u8)
    }
    #[doc = "Enable Nonstandard Resistive Plugged-In Detection."]
    #[inline(always)]
    pub const fn set_ENDEVPLUGINDETECT(&mut self, val: ENDEVPLUGINDETECT) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Device Plug-In Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn DEVPLUGIN_POLARITY(&self) -> DEVPLUGIN_POLARITY {
        let val = (self.0 >> 5usize) & 0x01;
        DEVPLUGIN_POLARITY::from_bits(val as u8)
    }
    #[doc = "Device Plug-In Polarity."]
    #[inline(always)]
    pub const fn set_DEVPLUGIN_POLARITY(&mut self, val: DEVPLUGIN_POLARITY) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "OTG ID Change Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn OTG_ID_CHG_IRQ(&self) -> OTG_ID_CHG_IRQ {
        let val = (self.0 >> 6usize) & 0x01;
        OTG_ID_CHG_IRQ::from_bits(val as u8)
    }
    #[doc = "OTG ID Change Interrupt."]
    #[inline(always)]
    pub const fn set_OTG_ID_CHG_IRQ(&mut self, val: OTG_ID_CHG_IRQ) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable Internal OTG ID Detector."]
    #[must_use]
    #[inline(always)]
    pub const fn ENOTGIDDETECT(&self) -> ENOTGIDDETECT {
        let val = (self.0 >> 7usize) & 0x01;
        ENOTGIDDETECT::from_bits(val as u8)
    }
    #[doc = "Enable Internal OTG ID Detector."]
    #[inline(always)]
    pub const fn set_ENOTGIDDETECT(&mut self, val: ENOTGIDDETECT) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Resume Interrupt Sticky."]
    #[must_use]
    #[inline(always)]
    pub const fn RESUMEIRQSTICKY(&self) -> RESUMEIRQSTICKY {
        let val = (self.0 >> 8usize) & 0x01;
        RESUMEIRQSTICKY::from_bits(val as u8)
    }
    #[doc = "Resume Interrupt Sticky."]
    #[inline(always)]
    pub const fn set_RESUMEIRQSTICKY(&mut self, val: RESUMEIRQSTICKY) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Resume Detection Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ENIRQRESUMEDETECT(&self) -> ENIRQRESUMEDETECT {
        let val = (self.0 >> 9usize) & 0x01;
        ENIRQRESUMEDETECT::from_bits(val as u8)
    }
    #[doc = "Resume Detection Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ENIRQRESUMEDETECT(&mut self, val: ENIRQRESUMEDETECT) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Resume Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn RESUME_IRQ(&self) -> RESUME_IRQ {
        let val = (self.0 >> 10usize) & 0x01;
        RESUME_IRQ::from_bits(val as u8)
    }
    #[doc = "Resume Interrupt."]
    #[inline(always)]
    pub const fn set_RESUME_IRQ(&mut self, val: RESUME_IRQ) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Enable Interrupt for Nonstandard Resistive Plugged-In Detection."]
    #[must_use]
    #[inline(always)]
    pub const fn ENIRQDEVPLUGIN(&self) -> ENIRQDEVPLUGIN {
        let val = (self.0 >> 11usize) & 0x01;
        ENIRQDEVPLUGIN::from_bits(val as u8)
    }
    #[doc = "Enable Interrupt for Nonstandard Resistive Plugged-In Detection."]
    #[inline(always)]
    pub const fn set_ENIRQDEVPLUGIN(&mut self, val: ENIRQDEVPLUGIN) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Device Plug-In Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn DEVPLUGIN_IRQ(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Device Plug-In Interrupt."]
    #[inline(always)]
    pub const fn set_DEVPLUGIN_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "APB Clock Switch Option."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA_ON_LRADC(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "APB Clock Switch Option."]
    #[inline(always)]
    pub const fn set_DATA_ON_LRADC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "UTMI Level 2 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ENUTMILEVEL2(&self) -> ENUTMILEVEL2 {
        let val = (self.0 >> 14usize) & 0x01;
        ENUTMILEVEL2::from_bits(val as u8)
    }
    #[doc = "UTMI Level 2 Enable."]
    #[inline(always)]
    pub const fn set_ENUTMILEVEL2(&mut self, val: ENUTMILEVEL2) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "UTMI Level 3 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ENUTMILEVEL3(&self) -> ENUTMILEVEL3 {
        let val = (self.0 >> 15usize) & 0x01;
        ENUTMILEVEL3::from_bits(val as u8)
    }
    #[doc = "UTMI Level 3 Enable."]
    #[inline(always)]
    pub const fn set_ENUTMILEVEL3(&mut self, val: ENUTMILEVEL3) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Wake-Up Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ENIRQWAKEUP(&self) -> ENIRQWAKEUP {
        let val = (self.0 >> 16usize) & 0x01;
        ENIRQWAKEUP::from_bits(val as u8)
    }
    #[doc = "Wake-Up Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ENIRQWAKEUP(&mut self, val: ENIRQWAKEUP) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Wake-Up Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn WAKEUP_IRQ(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-Up Interrupt."]
    #[inline(always)]
    pub const fn set_WAKEUP_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Autoresume Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn AUTORESUME_EN(&self) -> AUTORESUME_EN {
        let val = (self.0 >> 18usize) & 0x01;
        AUTORESUME_EN::from_bits(val as u8)
    }
    #[doc = "Autoresume Enable."]
    #[inline(always)]
    pub const fn set_AUTORESUME_EN(&mut self, val: AUTORESUME_EN) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Autoclear Clock Gate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ENAUTOCLR_CLKGATE(&self) -> ENAUTOCLR_CLKGATE {
        let val = (self.0 >> 19usize) & 0x01;
        ENAUTOCLR_CLKGATE::from_bits(val as u8)
    }
    #[doc = "Autoclear Clock Gate Enable."]
    #[inline(always)]
    pub const fn set_ENAUTOCLR_CLKGATE(&mut self, val: ENAUTOCLR_CLKGATE) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "PHY PWD Autoclear Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ENAUTOCLR_PHY_PWD(&self) -> ENAUTOCLR_PHY_PWD {
        let val = (self.0 >> 20usize) & 0x01;
        ENAUTOCLR_PHY_PWD::from_bits(val as u8)
    }
    #[doc = "PHY PWD Autoclear Enable."]
    #[inline(always)]
    pub const fn set_ENAUTOCLR_PHY_PWD(&mut self, val: ENAUTOCLR_PHY_PWD) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "OTG ID Value."]
    #[must_use]
    #[inline(always)]
    pub const fn OTG_ID_VALUE(&self) -> OTG_ID_VALUE {
        let val = (self.0 >> 27usize) & 0x01;
        OTG_ID_VALUE::from_bits(val as u8)
    }
    #[doc = "OTG ID Value."]
    #[inline(always)]
    pub const fn set_OTG_ID_VALUE(&mut self, val: OTG_ID_VALUE) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "UTMI Suspend."]
    #[must_use]
    #[inline(always)]
    pub const fn UTMI_SUSPENDM(&self) -> UTMI_SUSPENDM {
        let val = (self.0 >> 29usize) & 0x01;
        UTMI_SUSPENDM::from_bits(val as u8)
    }
    #[doc = "UTMI Suspend."]
    #[inline(always)]
    pub const fn set_UTMI_SUSPENDM(&mut self, val: UTMI_SUSPENDM) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "UTMI Clock Gate."]
    #[must_use]
    #[inline(always)]
    pub const fn CLKGATE(&self) -> CLKGATE {
        let val = (self.0 >> 30usize) & 0x01;
        CLKGATE::from_bits(val as u8)
    }
    #[doc = "UTMI Clock Gate."]
    #[inline(always)]
    pub const fn set_CLKGATE(&mut self, val: CLKGATE) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Software Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn SFTRST(&self) -> SFTRST {
        let val = (self.0 >> 31usize) & 0x01;
        SFTRST::from_bits(val as u8)
    }
    #[doc = "Software Reset."]
    #[inline(always)]
    pub const fn set_SFTRST(&mut self, val: SFTRST) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for CTRL {
    #[inline(always)]
    fn default() -> CTRL {
        CTRL(0)
    }
}
impl core::fmt::Debug for CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("ENOTG_ID_CHG_IRQ", &self.ENOTG_ID_CHG_IRQ())
            .field("ENHOSTDISCONDETECT", &self.ENHOSTDISCONDETECT())
            .field("ENIRQHOSTDISCON", &self.ENIRQHOSTDISCON())
            .field("HOSTDISCONDETECT_IRQ", &self.HOSTDISCONDETECT_IRQ())
            .field("ENDEVPLUGINDETECT", &self.ENDEVPLUGINDETECT())
            .field("DEVPLUGIN_POLARITY", &self.DEVPLUGIN_POLARITY())
            .field("OTG_ID_CHG_IRQ", &self.OTG_ID_CHG_IRQ())
            .field("ENOTGIDDETECT", &self.ENOTGIDDETECT())
            .field("RESUMEIRQSTICKY", &self.RESUMEIRQSTICKY())
            .field("ENIRQRESUMEDETECT", &self.ENIRQRESUMEDETECT())
            .field("RESUME_IRQ", &self.RESUME_IRQ())
            .field("ENIRQDEVPLUGIN", &self.ENIRQDEVPLUGIN())
            .field("DEVPLUGIN_IRQ", &self.DEVPLUGIN_IRQ())
            .field("DATA_ON_LRADC", &self.DATA_ON_LRADC())
            .field("ENUTMILEVEL2", &self.ENUTMILEVEL2())
            .field("ENUTMILEVEL3", &self.ENUTMILEVEL3())
            .field("ENIRQWAKEUP", &self.ENIRQWAKEUP())
            .field("WAKEUP_IRQ", &self.WAKEUP_IRQ())
            .field("AUTORESUME_EN", &self.AUTORESUME_EN())
            .field("ENAUTOCLR_CLKGATE", &self.ENAUTOCLR_CLKGATE())
            .field("ENAUTOCLR_PHY_PWD", &self.ENAUTOCLR_PHY_PWD())
            .field("OTG_ID_VALUE", &self.OTG_ID_VALUE())
            .field("UTMI_SUSPENDM", &self.UTMI_SUSPENDM())
            .field("CLKGATE", &self.CLKGATE())
            .field("SFTRST", &self.SFTRST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTRL {{ ENOTG_ID_CHG_IRQ: {:?}, ENHOSTDISCONDETECT: {:?}, ENIRQHOSTDISCON: {:?}, HOSTDISCONDETECT_IRQ: {:?}, ENDEVPLUGINDETECT: {:?}, DEVPLUGIN_POLARITY: {:?}, OTG_ID_CHG_IRQ: {:?}, ENOTGIDDETECT: {:?}, RESUMEIRQSTICKY: {:?}, ENIRQRESUMEDETECT: {:?}, RESUME_IRQ: {:?}, ENIRQDEVPLUGIN: {:?}, DEVPLUGIN_IRQ: {=bool:?}, DATA_ON_LRADC: {=bool:?}, ENUTMILEVEL2: {:?}, ENUTMILEVEL3: {:?}, ENIRQWAKEUP: {:?}, WAKEUP_IRQ: {=bool:?}, AUTORESUME_EN: {:?}, ENAUTOCLR_CLKGATE: {:?}, ENAUTOCLR_PHY_PWD: {:?}, OTG_ID_VALUE: {:?}, UTMI_SUSPENDM: {:?}, CLKGATE: {:?}, SFTRST: {:?} }}",
            self.ENOTG_ID_CHG_IRQ(),
            self.ENHOSTDISCONDETECT(),
            self.ENIRQHOSTDISCON(),
            self.HOSTDISCONDETECT_IRQ(),
            self.ENDEVPLUGINDETECT(),
            self.DEVPLUGIN_POLARITY(),
            self.OTG_ID_CHG_IRQ(),
            self.ENOTGIDDETECT(),
            self.RESUMEIRQSTICKY(),
            self.ENIRQRESUMEDETECT(),
            self.RESUME_IRQ(),
            self.ENIRQDEVPLUGIN(),
            self.DEVPLUGIN_IRQ(),
            self.DATA_ON_LRADC(),
            self.ENUTMILEVEL2(),
            self.ENUTMILEVEL3(),
            self.ENIRQWAKEUP(),
            self.WAKEUP_IRQ(),
            self.AUTORESUME_EN(),
            self.ENAUTOCLR_CLKGATE(),
            self.ENAUTOCLR_PHY_PWD(),
            self.OTG_ID_VALUE(),
            self.UTMI_SUSPENDM(),
            self.CLKGATE(),
            self.SFTRST()
        )
    }
}
#[doc = "General Purpose Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTRL_CLR(pub u32);
impl CTRL_CLR {
    #[doc = "OTG ID Change Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ENOTG_ID_CHG_IRQ(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID Change Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ENOTG_ID_CHG_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Host Disconnect Detection Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ENHOSTDISCONDETECT(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Host Disconnect Detection Enable."]
    #[inline(always)]
    pub const fn set_ENHOSTDISCONDETECT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable Interrupt for Host Disconnect."]
    #[must_use]
    #[inline(always)]
    pub const fn ENIRQHOSTDISCON(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Interrupt for Host Disconnect."]
    #[inline(always)]
    pub const fn set_ENIRQHOSTDISCON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Host Disconnect Detection Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn HOSTDISCONDETECT_IRQ(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Host Disconnect Detection Interrupt."]
    #[inline(always)]
    pub const fn set_HOSTDISCONDETECT_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable Nonstandard Resistive Plugged-In Detection."]
    #[must_use]
    #[inline(always)]
    pub const fn ENDEVPLUGINDETECT(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Nonstandard Resistive Plugged-In Detection."]
    #[inline(always)]
    pub const fn set_ENDEVPLUGINDETECT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Device Plug-In Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn DEVPLUGIN_POLARITY(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Device Plug-In Polarity."]
    #[inline(always)]
    pub const fn set_DEVPLUGIN_POLARITY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "OTG ID Change Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn OTG_ID_CHG_IRQ(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID Change Interrupt."]
    #[inline(always)]
    pub const fn set_OTG_ID_CHG_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable Internal OTG ID Detector."]
    #[must_use]
    #[inline(always)]
    pub const fn ENOTGIDDETECT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Internal OTG ID Detector."]
    #[inline(always)]
    pub const fn set_ENOTGIDDETECT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Resume Interrupt Sticky."]
    #[must_use]
    #[inline(always)]
    pub const fn RESUMEIRQSTICKY(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Interrupt Sticky."]
    #[inline(always)]
    pub const fn set_RESUMEIRQSTICKY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Resume Detection Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ENIRQRESUMEDETECT(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Detection Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ENIRQRESUMEDETECT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Resume Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn RESUME_IRQ(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Interrupt."]
    #[inline(always)]
    pub const fn set_RESUME_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enable Interrupt for Nonstandard Resistive Plugged-In Detection."]
    #[must_use]
    #[inline(always)]
    pub const fn ENIRQDEVPLUGIN(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Interrupt for Nonstandard Resistive Plugged-In Detection."]
    #[inline(always)]
    pub const fn set_ENIRQDEVPLUGIN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Device Plug-In Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn DEVPLUGIN_IRQ(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Device Plug-In Interrupt."]
    #[inline(always)]
    pub const fn set_DEVPLUGIN_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "APB Clock Switch Option."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA_ON_LRADC(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "APB Clock Switch Option."]
    #[inline(always)]
    pub const fn set_DATA_ON_LRADC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "UTMI Level 2 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ENUTMILEVEL2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Level 2 Enable."]
    #[inline(always)]
    pub const fn set_ENUTMILEVEL2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "UTMI Level 3 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ENUTMILEVEL3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Level 3 Enable."]
    #[inline(always)]
    pub const fn set_ENUTMILEVEL3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Wake-Up Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ENIRQWAKEUP(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-Up Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ENIRQWAKEUP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Wake-Up Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn WAKEUP_IRQ(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-Up Interrupt."]
    #[inline(always)]
    pub const fn set_WAKEUP_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Autoresume Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn AUTORESUME_EN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Autoresume Enable."]
    #[inline(always)]
    pub const fn set_AUTORESUME_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Autoclear Clock Gate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ENAUTOCLR_CLKGATE(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Autoclear Clock Gate Enable."]
    #[inline(always)]
    pub const fn set_ENAUTOCLR_CLKGATE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "PHY PWD Autoclear Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ENAUTOCLR_PHY_PWD(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "PHY PWD Autoclear Enable."]
    #[inline(always)]
    pub const fn set_ENAUTOCLR_PHY_PWD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "OTG ID Value."]
    #[must_use]
    #[inline(always)]
    pub const fn OTG_ID_VALUE(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID Value."]
    #[inline(always)]
    pub const fn set_OTG_ID_VALUE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "UTMI Suspend."]
    #[must_use]
    #[inline(always)]
    pub const fn UTMI_SUSPENDM(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Suspend."]
    #[inline(always)]
    pub const fn set_UTMI_SUSPENDM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "UTMI Clock Gate."]
    #[must_use]
    #[inline(always)]
    pub const fn CLKGATE(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Clock Gate."]
    #[inline(always)]
    pub const fn set_CLKGATE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Software Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn SFTRST(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset."]
    #[inline(always)]
    pub const fn set_SFTRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CTRL_CLR {
    #[inline(always)]
    fn default() -> CTRL_CLR {
        CTRL_CLR(0)
    }
}
impl core::fmt::Debug for CTRL_CLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL_CLR")
            .field("ENOTG_ID_CHG_IRQ", &self.ENOTG_ID_CHG_IRQ())
            .field("ENHOSTDISCONDETECT", &self.ENHOSTDISCONDETECT())
            .field("ENIRQHOSTDISCON", &self.ENIRQHOSTDISCON())
            .field("HOSTDISCONDETECT_IRQ", &self.HOSTDISCONDETECT_IRQ())
            .field("ENDEVPLUGINDETECT", &self.ENDEVPLUGINDETECT())
            .field("DEVPLUGIN_POLARITY", &self.DEVPLUGIN_POLARITY())
            .field("OTG_ID_CHG_IRQ", &self.OTG_ID_CHG_IRQ())
            .field("ENOTGIDDETECT", &self.ENOTGIDDETECT())
            .field("RESUMEIRQSTICKY", &self.RESUMEIRQSTICKY())
            .field("ENIRQRESUMEDETECT", &self.ENIRQRESUMEDETECT())
            .field("RESUME_IRQ", &self.RESUME_IRQ())
            .field("ENIRQDEVPLUGIN", &self.ENIRQDEVPLUGIN())
            .field("DEVPLUGIN_IRQ", &self.DEVPLUGIN_IRQ())
            .field("DATA_ON_LRADC", &self.DATA_ON_LRADC())
            .field("ENUTMILEVEL2", &self.ENUTMILEVEL2())
            .field("ENUTMILEVEL3", &self.ENUTMILEVEL3())
            .field("ENIRQWAKEUP", &self.ENIRQWAKEUP())
            .field("WAKEUP_IRQ", &self.WAKEUP_IRQ())
            .field("AUTORESUME_EN", &self.AUTORESUME_EN())
            .field("ENAUTOCLR_CLKGATE", &self.ENAUTOCLR_CLKGATE())
            .field("ENAUTOCLR_PHY_PWD", &self.ENAUTOCLR_PHY_PWD())
            .field("OTG_ID_VALUE", &self.OTG_ID_VALUE())
            .field("UTMI_SUSPENDM", &self.UTMI_SUSPENDM())
            .field("CLKGATE", &self.CLKGATE())
            .field("SFTRST", &self.SFTRST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTRL_CLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTRL_CLR {{ ENOTG_ID_CHG_IRQ: {=bool:?}, ENHOSTDISCONDETECT: {=bool:?}, ENIRQHOSTDISCON: {=bool:?}, HOSTDISCONDETECT_IRQ: {=bool:?}, ENDEVPLUGINDETECT: {=bool:?}, DEVPLUGIN_POLARITY: {=bool:?}, OTG_ID_CHG_IRQ: {=bool:?}, ENOTGIDDETECT: {=bool:?}, RESUMEIRQSTICKY: {=bool:?}, ENIRQRESUMEDETECT: {=bool:?}, RESUME_IRQ: {=bool:?}, ENIRQDEVPLUGIN: {=bool:?}, DEVPLUGIN_IRQ: {=bool:?}, DATA_ON_LRADC: {=bool:?}, ENUTMILEVEL2: {=bool:?}, ENUTMILEVEL3: {=bool:?}, ENIRQWAKEUP: {=bool:?}, WAKEUP_IRQ: {=bool:?}, AUTORESUME_EN: {=bool:?}, ENAUTOCLR_CLKGATE: {=bool:?}, ENAUTOCLR_PHY_PWD: {=bool:?}, OTG_ID_VALUE: {=bool:?}, UTMI_SUSPENDM: {=bool:?}, CLKGATE: {=bool:?}, SFTRST: {=bool:?} }}",
            self.ENOTG_ID_CHG_IRQ(),
            self.ENHOSTDISCONDETECT(),
            self.ENIRQHOSTDISCON(),
            self.HOSTDISCONDETECT_IRQ(),
            self.ENDEVPLUGINDETECT(),
            self.DEVPLUGIN_POLARITY(),
            self.OTG_ID_CHG_IRQ(),
            self.ENOTGIDDETECT(),
            self.RESUMEIRQSTICKY(),
            self.ENIRQRESUMEDETECT(),
            self.RESUME_IRQ(),
            self.ENIRQDEVPLUGIN(),
            self.DEVPLUGIN_IRQ(),
            self.DATA_ON_LRADC(),
            self.ENUTMILEVEL2(),
            self.ENUTMILEVEL3(),
            self.ENIRQWAKEUP(),
            self.WAKEUP_IRQ(),
            self.AUTORESUME_EN(),
            self.ENAUTOCLR_CLKGATE(),
            self.ENAUTOCLR_PHY_PWD(),
            self.OTG_ID_VALUE(),
            self.UTMI_SUSPENDM(),
            self.CLKGATE(),
            self.SFTRST()
        )
    }
}
#[doc = "General Purpose Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTRL_SET(pub u32);
impl CTRL_SET {
    #[doc = "OTG ID Change Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ENOTG_ID_CHG_IRQ(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID Change Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ENOTG_ID_CHG_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Host Disconnect Detection Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ENHOSTDISCONDETECT(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Host Disconnect Detection Enable."]
    #[inline(always)]
    pub const fn set_ENHOSTDISCONDETECT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable Interrupt for Host Disconnect."]
    #[must_use]
    #[inline(always)]
    pub const fn ENIRQHOSTDISCON(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Interrupt for Host Disconnect."]
    #[inline(always)]
    pub const fn set_ENIRQHOSTDISCON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Host Disconnect Detection Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn HOSTDISCONDETECT_IRQ(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Host Disconnect Detection Interrupt."]
    #[inline(always)]
    pub const fn set_HOSTDISCONDETECT_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable Nonstandard Resistive Plugged-In Detection."]
    #[must_use]
    #[inline(always)]
    pub const fn ENDEVPLUGINDETECT(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Nonstandard Resistive Plugged-In Detection."]
    #[inline(always)]
    pub const fn set_ENDEVPLUGINDETECT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Device Plug-In Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn DEVPLUGIN_POLARITY(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Device Plug-In Polarity."]
    #[inline(always)]
    pub const fn set_DEVPLUGIN_POLARITY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "OTG ID Change Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn OTG_ID_CHG_IRQ(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID Change Interrupt."]
    #[inline(always)]
    pub const fn set_OTG_ID_CHG_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable Internal OTG ID Detector."]
    #[must_use]
    #[inline(always)]
    pub const fn ENOTGIDDETECT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Internal OTG ID Detector."]
    #[inline(always)]
    pub const fn set_ENOTGIDDETECT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Resume Interrupt Sticky."]
    #[must_use]
    #[inline(always)]
    pub const fn RESUMEIRQSTICKY(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Interrupt Sticky."]
    #[inline(always)]
    pub const fn set_RESUMEIRQSTICKY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Resume Detection Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ENIRQRESUMEDETECT(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Detection Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ENIRQRESUMEDETECT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Resume Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn RESUME_IRQ(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Interrupt."]
    #[inline(always)]
    pub const fn set_RESUME_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enable Interrupt for Nonstandard Resistive Plugged-In Detection."]
    #[must_use]
    #[inline(always)]
    pub const fn ENIRQDEVPLUGIN(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Interrupt for Nonstandard Resistive Plugged-In Detection."]
    #[inline(always)]
    pub const fn set_ENIRQDEVPLUGIN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Device Plug-In Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn DEVPLUGIN_IRQ(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Device Plug-In Interrupt."]
    #[inline(always)]
    pub const fn set_DEVPLUGIN_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "APB Clock Switch Option."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA_ON_LRADC(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "APB Clock Switch Option."]
    #[inline(always)]
    pub const fn set_DATA_ON_LRADC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "UTMI Level 2 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ENUTMILEVEL2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Level 2 Enable."]
    #[inline(always)]
    pub const fn set_ENUTMILEVEL2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "UTMI Level 3 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ENUTMILEVEL3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Level 3 Enable."]
    #[inline(always)]
    pub const fn set_ENUTMILEVEL3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Wake-Up Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ENIRQWAKEUP(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-Up Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ENIRQWAKEUP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Wake-Up Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn WAKEUP_IRQ(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-Up Interrupt."]
    #[inline(always)]
    pub const fn set_WAKEUP_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Autoresume Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn AUTORESUME_EN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Autoresume Enable."]
    #[inline(always)]
    pub const fn set_AUTORESUME_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Autoclear Clock Gate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ENAUTOCLR_CLKGATE(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Autoclear Clock Gate Enable."]
    #[inline(always)]
    pub const fn set_ENAUTOCLR_CLKGATE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "PHY PWD Autoclear Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ENAUTOCLR_PHY_PWD(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "PHY PWD Autoclear Enable."]
    #[inline(always)]
    pub const fn set_ENAUTOCLR_PHY_PWD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "OTG ID Value."]
    #[must_use]
    #[inline(always)]
    pub const fn OTG_ID_VALUE(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID Value."]
    #[inline(always)]
    pub const fn set_OTG_ID_VALUE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "UTMI Suspend."]
    #[must_use]
    #[inline(always)]
    pub const fn UTMI_SUSPENDM(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Suspend."]
    #[inline(always)]
    pub const fn set_UTMI_SUSPENDM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "UTMI Clock Gate."]
    #[must_use]
    #[inline(always)]
    pub const fn CLKGATE(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Clock Gate."]
    #[inline(always)]
    pub const fn set_CLKGATE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Software Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn SFTRST(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset."]
    #[inline(always)]
    pub const fn set_SFTRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CTRL_SET {
    #[inline(always)]
    fn default() -> CTRL_SET {
        CTRL_SET(0)
    }
}
impl core::fmt::Debug for CTRL_SET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL_SET")
            .field("ENOTG_ID_CHG_IRQ", &self.ENOTG_ID_CHG_IRQ())
            .field("ENHOSTDISCONDETECT", &self.ENHOSTDISCONDETECT())
            .field("ENIRQHOSTDISCON", &self.ENIRQHOSTDISCON())
            .field("HOSTDISCONDETECT_IRQ", &self.HOSTDISCONDETECT_IRQ())
            .field("ENDEVPLUGINDETECT", &self.ENDEVPLUGINDETECT())
            .field("DEVPLUGIN_POLARITY", &self.DEVPLUGIN_POLARITY())
            .field("OTG_ID_CHG_IRQ", &self.OTG_ID_CHG_IRQ())
            .field("ENOTGIDDETECT", &self.ENOTGIDDETECT())
            .field("RESUMEIRQSTICKY", &self.RESUMEIRQSTICKY())
            .field("ENIRQRESUMEDETECT", &self.ENIRQRESUMEDETECT())
            .field("RESUME_IRQ", &self.RESUME_IRQ())
            .field("ENIRQDEVPLUGIN", &self.ENIRQDEVPLUGIN())
            .field("DEVPLUGIN_IRQ", &self.DEVPLUGIN_IRQ())
            .field("DATA_ON_LRADC", &self.DATA_ON_LRADC())
            .field("ENUTMILEVEL2", &self.ENUTMILEVEL2())
            .field("ENUTMILEVEL3", &self.ENUTMILEVEL3())
            .field("ENIRQWAKEUP", &self.ENIRQWAKEUP())
            .field("WAKEUP_IRQ", &self.WAKEUP_IRQ())
            .field("AUTORESUME_EN", &self.AUTORESUME_EN())
            .field("ENAUTOCLR_CLKGATE", &self.ENAUTOCLR_CLKGATE())
            .field("ENAUTOCLR_PHY_PWD", &self.ENAUTOCLR_PHY_PWD())
            .field("OTG_ID_VALUE", &self.OTG_ID_VALUE())
            .field("UTMI_SUSPENDM", &self.UTMI_SUSPENDM())
            .field("CLKGATE", &self.CLKGATE())
            .field("SFTRST", &self.SFTRST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTRL_SET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTRL_SET {{ ENOTG_ID_CHG_IRQ: {=bool:?}, ENHOSTDISCONDETECT: {=bool:?}, ENIRQHOSTDISCON: {=bool:?}, HOSTDISCONDETECT_IRQ: {=bool:?}, ENDEVPLUGINDETECT: {=bool:?}, DEVPLUGIN_POLARITY: {=bool:?}, OTG_ID_CHG_IRQ: {=bool:?}, ENOTGIDDETECT: {=bool:?}, RESUMEIRQSTICKY: {=bool:?}, ENIRQRESUMEDETECT: {=bool:?}, RESUME_IRQ: {=bool:?}, ENIRQDEVPLUGIN: {=bool:?}, DEVPLUGIN_IRQ: {=bool:?}, DATA_ON_LRADC: {=bool:?}, ENUTMILEVEL2: {=bool:?}, ENUTMILEVEL3: {=bool:?}, ENIRQWAKEUP: {=bool:?}, WAKEUP_IRQ: {=bool:?}, AUTORESUME_EN: {=bool:?}, ENAUTOCLR_CLKGATE: {=bool:?}, ENAUTOCLR_PHY_PWD: {=bool:?}, OTG_ID_VALUE: {=bool:?}, UTMI_SUSPENDM: {=bool:?}, CLKGATE: {=bool:?}, SFTRST: {=bool:?} }}",
            self.ENOTG_ID_CHG_IRQ(),
            self.ENHOSTDISCONDETECT(),
            self.ENIRQHOSTDISCON(),
            self.HOSTDISCONDETECT_IRQ(),
            self.ENDEVPLUGINDETECT(),
            self.DEVPLUGIN_POLARITY(),
            self.OTG_ID_CHG_IRQ(),
            self.ENOTGIDDETECT(),
            self.RESUMEIRQSTICKY(),
            self.ENIRQRESUMEDETECT(),
            self.RESUME_IRQ(),
            self.ENIRQDEVPLUGIN(),
            self.DEVPLUGIN_IRQ(),
            self.DATA_ON_LRADC(),
            self.ENUTMILEVEL2(),
            self.ENUTMILEVEL3(),
            self.ENIRQWAKEUP(),
            self.WAKEUP_IRQ(),
            self.AUTORESUME_EN(),
            self.ENAUTOCLR_CLKGATE(),
            self.ENAUTOCLR_PHY_PWD(),
            self.OTG_ID_VALUE(),
            self.UTMI_SUSPENDM(),
            self.CLKGATE(),
            self.SFTRST()
        )
    }
}
#[doc = "General Purpose Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTRL_TOG(pub u32);
impl CTRL_TOG {
    #[doc = "OTG ID Change Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ENOTG_ID_CHG_IRQ(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID Change Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ENOTG_ID_CHG_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Host Disconnect Detection Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ENHOSTDISCONDETECT(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Host Disconnect Detection Enable."]
    #[inline(always)]
    pub const fn set_ENHOSTDISCONDETECT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable Interrupt for Host Disconnect."]
    #[must_use]
    #[inline(always)]
    pub const fn ENIRQHOSTDISCON(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Interrupt for Host Disconnect."]
    #[inline(always)]
    pub const fn set_ENIRQHOSTDISCON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Host Disconnect Detection Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn HOSTDISCONDETECT_IRQ(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Host Disconnect Detection Interrupt."]
    #[inline(always)]
    pub const fn set_HOSTDISCONDETECT_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable Nonstandard Resistive Plugged-In Detection."]
    #[must_use]
    #[inline(always)]
    pub const fn ENDEVPLUGINDETECT(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Nonstandard Resistive Plugged-In Detection."]
    #[inline(always)]
    pub const fn set_ENDEVPLUGINDETECT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Device Plug-In Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn DEVPLUGIN_POLARITY(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Device Plug-In Polarity."]
    #[inline(always)]
    pub const fn set_DEVPLUGIN_POLARITY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "OTG ID Change Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn OTG_ID_CHG_IRQ(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID Change Interrupt."]
    #[inline(always)]
    pub const fn set_OTG_ID_CHG_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable Internal OTG ID Detector."]
    #[must_use]
    #[inline(always)]
    pub const fn ENOTGIDDETECT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Internal OTG ID Detector."]
    #[inline(always)]
    pub const fn set_ENOTGIDDETECT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Resume Interrupt Sticky."]
    #[must_use]
    #[inline(always)]
    pub const fn RESUMEIRQSTICKY(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Interrupt Sticky."]
    #[inline(always)]
    pub const fn set_RESUMEIRQSTICKY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Resume Detection Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ENIRQRESUMEDETECT(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Detection Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ENIRQRESUMEDETECT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Resume Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn RESUME_IRQ(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Interrupt."]
    #[inline(always)]
    pub const fn set_RESUME_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enable Interrupt for Nonstandard Resistive Plugged-In Detection."]
    #[must_use]
    #[inline(always)]
    pub const fn ENIRQDEVPLUGIN(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Interrupt for Nonstandard Resistive Plugged-In Detection."]
    #[inline(always)]
    pub const fn set_ENIRQDEVPLUGIN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Device Plug-In Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn DEVPLUGIN_IRQ(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Device Plug-In Interrupt."]
    #[inline(always)]
    pub const fn set_DEVPLUGIN_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "APB Clock Switch Option."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA_ON_LRADC(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "APB Clock Switch Option."]
    #[inline(always)]
    pub const fn set_DATA_ON_LRADC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "UTMI Level 2 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ENUTMILEVEL2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Level 2 Enable."]
    #[inline(always)]
    pub const fn set_ENUTMILEVEL2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "UTMI Level 3 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ENUTMILEVEL3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Level 3 Enable."]
    #[inline(always)]
    pub const fn set_ENUTMILEVEL3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Wake-Up Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ENIRQWAKEUP(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-Up Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ENIRQWAKEUP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Wake-Up Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn WAKEUP_IRQ(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-Up Interrupt."]
    #[inline(always)]
    pub const fn set_WAKEUP_IRQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Autoresume Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn AUTORESUME_EN(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Autoresume Enable."]
    #[inline(always)]
    pub const fn set_AUTORESUME_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Autoclear Clock Gate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ENAUTOCLR_CLKGATE(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Autoclear Clock Gate Enable."]
    #[inline(always)]
    pub const fn set_ENAUTOCLR_CLKGATE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "PHY PWD Autoclear Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ENAUTOCLR_PHY_PWD(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "PHY PWD Autoclear Enable."]
    #[inline(always)]
    pub const fn set_ENAUTOCLR_PHY_PWD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "OTG ID Value."]
    #[must_use]
    #[inline(always)]
    pub const fn OTG_ID_VALUE(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID Value."]
    #[inline(always)]
    pub const fn set_OTG_ID_VALUE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "UTMI Suspend."]
    #[must_use]
    #[inline(always)]
    pub const fn UTMI_SUSPENDM(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Suspend."]
    #[inline(always)]
    pub const fn set_UTMI_SUSPENDM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "UTMI Clock Gate."]
    #[must_use]
    #[inline(always)]
    pub const fn CLKGATE(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Clock Gate."]
    #[inline(always)]
    pub const fn set_CLKGATE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Software Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn SFTRST(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset."]
    #[inline(always)]
    pub const fn set_SFTRST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CTRL_TOG {
    #[inline(always)]
    fn default() -> CTRL_TOG {
        CTRL_TOG(0)
    }
}
impl core::fmt::Debug for CTRL_TOG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL_TOG")
            .field("ENOTG_ID_CHG_IRQ", &self.ENOTG_ID_CHG_IRQ())
            .field("ENHOSTDISCONDETECT", &self.ENHOSTDISCONDETECT())
            .field("ENIRQHOSTDISCON", &self.ENIRQHOSTDISCON())
            .field("HOSTDISCONDETECT_IRQ", &self.HOSTDISCONDETECT_IRQ())
            .field("ENDEVPLUGINDETECT", &self.ENDEVPLUGINDETECT())
            .field("DEVPLUGIN_POLARITY", &self.DEVPLUGIN_POLARITY())
            .field("OTG_ID_CHG_IRQ", &self.OTG_ID_CHG_IRQ())
            .field("ENOTGIDDETECT", &self.ENOTGIDDETECT())
            .field("RESUMEIRQSTICKY", &self.RESUMEIRQSTICKY())
            .field("ENIRQRESUMEDETECT", &self.ENIRQRESUMEDETECT())
            .field("RESUME_IRQ", &self.RESUME_IRQ())
            .field("ENIRQDEVPLUGIN", &self.ENIRQDEVPLUGIN())
            .field("DEVPLUGIN_IRQ", &self.DEVPLUGIN_IRQ())
            .field("DATA_ON_LRADC", &self.DATA_ON_LRADC())
            .field("ENUTMILEVEL2", &self.ENUTMILEVEL2())
            .field("ENUTMILEVEL3", &self.ENUTMILEVEL3())
            .field("ENIRQWAKEUP", &self.ENIRQWAKEUP())
            .field("WAKEUP_IRQ", &self.WAKEUP_IRQ())
            .field("AUTORESUME_EN", &self.AUTORESUME_EN())
            .field("ENAUTOCLR_CLKGATE", &self.ENAUTOCLR_CLKGATE())
            .field("ENAUTOCLR_PHY_PWD", &self.ENAUTOCLR_PHY_PWD())
            .field("OTG_ID_VALUE", &self.OTG_ID_VALUE())
            .field("UTMI_SUSPENDM", &self.UTMI_SUSPENDM())
            .field("CLKGATE", &self.CLKGATE())
            .field("SFTRST", &self.SFTRST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTRL_TOG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTRL_TOG {{ ENOTG_ID_CHG_IRQ: {=bool:?}, ENHOSTDISCONDETECT: {=bool:?}, ENIRQHOSTDISCON: {=bool:?}, HOSTDISCONDETECT_IRQ: {=bool:?}, ENDEVPLUGINDETECT: {=bool:?}, DEVPLUGIN_POLARITY: {=bool:?}, OTG_ID_CHG_IRQ: {=bool:?}, ENOTGIDDETECT: {=bool:?}, RESUMEIRQSTICKY: {=bool:?}, ENIRQRESUMEDETECT: {=bool:?}, RESUME_IRQ: {=bool:?}, ENIRQDEVPLUGIN: {=bool:?}, DEVPLUGIN_IRQ: {=bool:?}, DATA_ON_LRADC: {=bool:?}, ENUTMILEVEL2: {=bool:?}, ENUTMILEVEL3: {=bool:?}, ENIRQWAKEUP: {=bool:?}, WAKEUP_IRQ: {=bool:?}, AUTORESUME_EN: {=bool:?}, ENAUTOCLR_CLKGATE: {=bool:?}, ENAUTOCLR_PHY_PWD: {=bool:?}, OTG_ID_VALUE: {=bool:?}, UTMI_SUSPENDM: {=bool:?}, CLKGATE: {=bool:?}, SFTRST: {=bool:?} }}",
            self.ENOTG_ID_CHG_IRQ(),
            self.ENHOSTDISCONDETECT(),
            self.ENIRQHOSTDISCON(),
            self.HOSTDISCONDETECT_IRQ(),
            self.ENDEVPLUGINDETECT(),
            self.DEVPLUGIN_POLARITY(),
            self.OTG_ID_CHG_IRQ(),
            self.ENOTGIDDETECT(),
            self.RESUMEIRQSTICKY(),
            self.ENIRQRESUMEDETECT(),
            self.RESUME_IRQ(),
            self.ENIRQDEVPLUGIN(),
            self.DEVPLUGIN_IRQ(),
            self.DATA_ON_LRADC(),
            self.ENUTMILEVEL2(),
            self.ENUTMILEVEL3(),
            self.ENIRQWAKEUP(),
            self.WAKEUP_IRQ(),
            self.AUTORESUME_EN(),
            self.ENAUTOCLR_CLKGATE(),
            self.ENAUTOCLR_PHY_PWD(),
            self.OTG_ID_VALUE(),
            self.UTMI_SUSPENDM(),
            self.CLKGATE(),
            self.SFTRST()
        )
    }
}
#[doc = "Debug 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DEBUG0(pub u32);
impl DEBUG0 {
    #[doc = "Hold OTG_ID."]
    #[must_use]
    #[inline(always)]
    pub const fn OTGIDPIOLOCK(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Hold OTG_ID."]
    #[inline(always)]
    pub const fn set_OTGIDPIOLOCK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Host Pulldown Overdrive Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn HSTPULLDOWN(&self) -> HSTPULLDOWN {
        let val = (self.0 >> 2usize) & 0x03;
        HSTPULLDOWN::from_bits(val as u8)
    }
    #[doc = "Host Pulldown Overdrive Mode."]
    #[inline(always)]
    pub const fn set_HSTPULLDOWN(&mut self, val: HSTPULLDOWN) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Enable Host Pulldown Overdrive Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn ENHSTPULLDOWN(&self) -> ENHSTPULLDOWN {
        let val = (self.0 >> 4usize) & 0x03;
        ENHSTPULLDOWN::from_bits(val as u8)
    }
    #[doc = "Enable Host Pulldown Overdrive Mode."]
    #[inline(always)]
    pub const fn set_ENHSTPULLDOWN(&mut self, val: ENHSTPULLDOWN) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
}
impl Default for DEBUG0 {
    #[inline(always)]
    fn default() -> DEBUG0 {
        DEBUG0(0)
    }
}
impl core::fmt::Debug for DEBUG0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEBUG0")
            .field("OTGIDPIOLOCK", &self.OTGIDPIOLOCK())
            .field("HSTPULLDOWN", &self.HSTPULLDOWN())
            .field("ENHSTPULLDOWN", &self.ENHSTPULLDOWN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DEBUG0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DEBUG0 {{ OTGIDPIOLOCK: {=bool:?}, HSTPULLDOWN: {:?}, ENHSTPULLDOWN: {:?} }}",
            self.OTGIDPIOLOCK(),
            self.HSTPULLDOWN(),
            self.ENHSTPULLDOWN()
        )
    }
}
#[doc = "Debug 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DEBUG0_CLR(pub u32);
impl DEBUG0_CLR {
    #[doc = "Hold OTG_ID."]
    #[must_use]
    #[inline(always)]
    pub const fn OTGIDPIOLOCK(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Hold OTG_ID."]
    #[inline(always)]
    pub const fn set_OTGIDPIOLOCK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Host Pulldown Overdrive Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn HSTPULLDOWN(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Host Pulldown Overdrive Mode."]
    #[inline(always)]
    pub const fn set_HSTPULLDOWN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Enable Host Pulldown Overdrive Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn ENHSTPULLDOWN(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Enable Host Pulldown Overdrive Mode."]
    #[inline(always)]
    pub const fn set_ENHSTPULLDOWN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
}
impl Default for DEBUG0_CLR {
    #[inline(always)]
    fn default() -> DEBUG0_CLR {
        DEBUG0_CLR(0)
    }
}
impl core::fmt::Debug for DEBUG0_CLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEBUG0_CLR")
            .field("OTGIDPIOLOCK", &self.OTGIDPIOLOCK())
            .field("HSTPULLDOWN", &self.HSTPULLDOWN())
            .field("ENHSTPULLDOWN", &self.ENHSTPULLDOWN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DEBUG0_CLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DEBUG0_CLR {{ OTGIDPIOLOCK: {=bool:?}, HSTPULLDOWN: {=u8:?}, ENHSTPULLDOWN: {=u8:?} }}",
            self.OTGIDPIOLOCK(),
            self.HSTPULLDOWN(),
            self.ENHSTPULLDOWN()
        )
    }
}
#[doc = "Debug 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DEBUG0_SET(pub u32);
impl DEBUG0_SET {
    #[doc = "Hold OTG_ID."]
    #[must_use]
    #[inline(always)]
    pub const fn OTGIDPIOLOCK(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Hold OTG_ID."]
    #[inline(always)]
    pub const fn set_OTGIDPIOLOCK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Host Pulldown Overdrive Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn HSTPULLDOWN(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Host Pulldown Overdrive Mode."]
    #[inline(always)]
    pub const fn set_HSTPULLDOWN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Enable Host Pulldown Overdrive Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn ENHSTPULLDOWN(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Enable Host Pulldown Overdrive Mode."]
    #[inline(always)]
    pub const fn set_ENHSTPULLDOWN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
}
impl Default for DEBUG0_SET {
    #[inline(always)]
    fn default() -> DEBUG0_SET {
        DEBUG0_SET(0)
    }
}
impl core::fmt::Debug for DEBUG0_SET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEBUG0_SET")
            .field("OTGIDPIOLOCK", &self.OTGIDPIOLOCK())
            .field("HSTPULLDOWN", &self.HSTPULLDOWN())
            .field("ENHSTPULLDOWN", &self.ENHSTPULLDOWN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DEBUG0_SET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DEBUG0_SET {{ OTGIDPIOLOCK: {=bool:?}, HSTPULLDOWN: {=u8:?}, ENHSTPULLDOWN: {=u8:?} }}",
            self.OTGIDPIOLOCK(),
            self.HSTPULLDOWN(),
            self.ENHSTPULLDOWN()
        )
    }
}
#[doc = "Debug 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DEBUG0_TOG(pub u32);
impl DEBUG0_TOG {
    #[doc = "Hold OTG_ID."]
    #[must_use]
    #[inline(always)]
    pub const fn OTGIDPIOLOCK(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Hold OTG_ID."]
    #[inline(always)]
    pub const fn set_OTGIDPIOLOCK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Host Pulldown Overdrive Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn HSTPULLDOWN(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Host Pulldown Overdrive Mode."]
    #[inline(always)]
    pub const fn set_HSTPULLDOWN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Enable Host Pulldown Overdrive Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn ENHSTPULLDOWN(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Enable Host Pulldown Overdrive Mode."]
    #[inline(always)]
    pub const fn set_ENHSTPULLDOWN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
}
impl Default for DEBUG0_TOG {
    #[inline(always)]
    fn default() -> DEBUG0_TOG {
        DEBUG0_TOG(0)
    }
}
impl core::fmt::Debug for DEBUG0_TOG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEBUG0_TOG")
            .field("OTGIDPIOLOCK", &self.OTGIDPIOLOCK())
            .field("HSTPULLDOWN", &self.HSTPULLDOWN())
            .field("ENHSTPULLDOWN", &self.ENHSTPULLDOWN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DEBUG0_TOG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DEBUG0_TOG {{ OTGIDPIOLOCK: {=bool:?}, HSTPULLDOWN: {=u8:?}, ENHSTPULLDOWN: {=u8:?} }}",
            self.OTGIDPIOLOCK(),
            self.HSTPULLDOWN(),
            self.ENHSTPULLDOWN()
        )
    }
}
#[doc = "IP Block."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IP(pub u32);
impl IP {
    #[doc = "Power Control Suspend Option."]
    #[must_use]
    #[inline(always)]
    pub const fn POWER_CONTROL_SUSPEND_OPTION(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Power Control Suspend Option."]
    #[inline(always)]
    pub const fn set_POWER_CONTROL_SUSPEND_OPTION(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for IP {
    #[inline(always)]
    fn default() -> IP {
        IP(0)
    }
}
impl core::fmt::Debug for IP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IP")
            .field(
                "POWER_CONTROL_SUSPEND_OPTION",
                &self.POWER_CONTROL_SUSPEND_OPTION(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IP {{ POWER_CONTROL_SUSPEND_OPTION: {=bool:?} }}",
            self.POWER_CONTROL_SUSPEND_OPTION()
        )
    }
}
#[doc = "IP Block."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IP_CLR(pub u32);
impl IP_CLR {
    #[doc = "Power Control Suspend Option."]
    #[must_use]
    #[inline(always)]
    pub const fn POWER_CONTROL_SUSPEND_OPTION(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Power Control Suspend Option."]
    #[inline(always)]
    pub const fn set_POWER_CONTROL_SUSPEND_OPTION(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for IP_CLR {
    #[inline(always)]
    fn default() -> IP_CLR {
        IP_CLR(0)
    }
}
impl core::fmt::Debug for IP_CLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IP_CLR")
            .field(
                "POWER_CONTROL_SUSPEND_OPTION",
                &self.POWER_CONTROL_SUSPEND_OPTION(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IP_CLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IP_CLR {{ POWER_CONTROL_SUSPEND_OPTION: {=bool:?} }}",
            self.POWER_CONTROL_SUSPEND_OPTION()
        )
    }
}
#[doc = "IP Block."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IP_SET(pub u32);
impl IP_SET {
    #[doc = "Power Control Suspend Option."]
    #[must_use]
    #[inline(always)]
    pub const fn POWER_CONTROL_SUSPEND_OPTION(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Power Control Suspend Option."]
    #[inline(always)]
    pub const fn set_POWER_CONTROL_SUSPEND_OPTION(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for IP_SET {
    #[inline(always)]
    fn default() -> IP_SET {
        IP_SET(0)
    }
}
impl core::fmt::Debug for IP_SET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IP_SET")
            .field(
                "POWER_CONTROL_SUSPEND_OPTION",
                &self.POWER_CONTROL_SUSPEND_OPTION(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IP_SET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IP_SET {{ POWER_CONTROL_SUSPEND_OPTION: {=bool:?} }}",
            self.POWER_CONTROL_SUSPEND_OPTION()
        )
    }
}
#[doc = "IP Block."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IP_TOG(pub u32);
impl IP_TOG {
    #[doc = "Power Control Suspend Option."]
    #[must_use]
    #[inline(always)]
    pub const fn POWER_CONTROL_SUSPEND_OPTION(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Power Control Suspend Option."]
    #[inline(always)]
    pub const fn set_POWER_CONTROL_SUSPEND_OPTION(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for IP_TOG {
    #[inline(always)]
    fn default() -> IP_TOG {
        IP_TOG(0)
    }
}
impl core::fmt::Debug for IP_TOG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IP_TOG")
            .field(
                "POWER_CONTROL_SUSPEND_OPTION",
                &self.POWER_CONTROL_SUSPEND_OPTION(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IP_TOG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IP_TOG {{ POWER_CONTROL_SUSPEND_OPTION: {=bool:?} }}",
            self.POWER_CONTROL_SUSPEND_OPTION()
        )
    }
}
#[doc = "PFD A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PFDA(pub u32);
impl PFDA {
    #[doc = "PFD0 Clock Gate."]
    #[must_use]
    #[inline(always)]
    pub const fn PFD0_CLKGATE(&self) -> PFD0_CLKGATE {
        let val = (self.0 >> 0usize) & 0x01;
        PFD0_CLKGATE::from_bits(val as u8)
    }
    #[doc = "PFD0 Clock Gate."]
    #[inline(always)]
    pub const fn set_PFD0_CLKGATE(&mut self, val: PFD0_CLKGATE) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "PFD0 Fractional Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn PFD0_FRAC(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x3f;
        val as u8
    }
    #[doc = "PFD0 Fractional Divider."]
    #[inline(always)]
    pub const fn set_PFD0_FRAC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 1usize)) | (((val as u32) & 0x3f) << 1usize);
    }
    #[doc = "PFD0 Stable Signal."]
    #[must_use]
    #[inline(always)]
    pub const fn PFD0_STABLE(&self) -> PFD0_STABLE {
        let val = (self.0 >> 7usize) & 0x01;
        PFD0_STABLE::from_bits(val as u8)
    }
    #[doc = "PFD0 Stable Signal."]
    #[inline(always)]
    pub const fn set_PFD0_STABLE(&mut self, val: PFD0_STABLE) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for PFDA {
    #[inline(always)]
    fn default() -> PFDA {
        PFDA(0)
    }
}
impl core::fmt::Debug for PFDA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PFDA")
            .field("PFD0_CLKGATE", &self.PFD0_CLKGATE())
            .field("PFD0_FRAC", &self.PFD0_FRAC())
            .field("PFD0_STABLE", &self.PFD0_STABLE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PFDA {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PFDA {{ PFD0_CLKGATE: {:?}, PFD0_FRAC: {=u8:?}, PFD0_STABLE: {:?} }}",
            self.PFD0_CLKGATE(),
            self.PFD0_FRAC(),
            self.PFD0_STABLE()
        )
    }
}
#[doc = "PFD A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PFDA_CLR(pub u32);
impl PFDA_CLR {
    #[doc = "PFD0 Clock Gate."]
    #[must_use]
    #[inline(always)]
    pub const fn PFD0_CLKGATE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "PFD0 Clock Gate."]
    #[inline(always)]
    pub const fn set_PFD0_CLKGATE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "PFD0 Fractional Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn PFD0_FRAC(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x3f;
        val as u8
    }
    #[doc = "PFD0 Fractional Divider."]
    #[inline(always)]
    pub const fn set_PFD0_FRAC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 1usize)) | (((val as u32) & 0x3f) << 1usize);
    }
    #[doc = "PFD0 Stable Signal."]
    #[must_use]
    #[inline(always)]
    pub const fn PFD0_STABLE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "PFD0 Stable Signal."]
    #[inline(always)]
    pub const fn set_PFD0_STABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for PFDA_CLR {
    #[inline(always)]
    fn default() -> PFDA_CLR {
        PFDA_CLR(0)
    }
}
impl core::fmt::Debug for PFDA_CLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PFDA_CLR")
            .field("PFD0_CLKGATE", &self.PFD0_CLKGATE())
            .field("PFD0_FRAC", &self.PFD0_FRAC())
            .field("PFD0_STABLE", &self.PFD0_STABLE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PFDA_CLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PFDA_CLR {{ PFD0_CLKGATE: {=bool:?}, PFD0_FRAC: {=u8:?}, PFD0_STABLE: {=bool:?} }}",
            self.PFD0_CLKGATE(),
            self.PFD0_FRAC(),
            self.PFD0_STABLE()
        )
    }
}
#[doc = "PFD A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PFDA_SET(pub u32);
impl PFDA_SET {
    #[doc = "PFD0 Clock Gate."]
    #[must_use]
    #[inline(always)]
    pub const fn PFD0_CLKGATE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "PFD0 Clock Gate."]
    #[inline(always)]
    pub const fn set_PFD0_CLKGATE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "PFD0 Fractional Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn PFD0_FRAC(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x3f;
        val as u8
    }
    #[doc = "PFD0 Fractional Divider."]
    #[inline(always)]
    pub const fn set_PFD0_FRAC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 1usize)) | (((val as u32) & 0x3f) << 1usize);
    }
    #[doc = "PFD0 Stable Signal."]
    #[must_use]
    #[inline(always)]
    pub const fn PFD0_STABLE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "PFD0 Stable Signal."]
    #[inline(always)]
    pub const fn set_PFD0_STABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for PFDA_SET {
    #[inline(always)]
    fn default() -> PFDA_SET {
        PFDA_SET(0)
    }
}
impl core::fmt::Debug for PFDA_SET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PFDA_SET")
            .field("PFD0_CLKGATE", &self.PFD0_CLKGATE())
            .field("PFD0_FRAC", &self.PFD0_FRAC())
            .field("PFD0_STABLE", &self.PFD0_STABLE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PFDA_SET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PFDA_SET {{ PFD0_CLKGATE: {=bool:?}, PFD0_FRAC: {=u8:?}, PFD0_STABLE: {=bool:?} }}",
            self.PFD0_CLKGATE(),
            self.PFD0_FRAC(),
            self.PFD0_STABLE()
        )
    }
}
#[doc = "PFD A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PFDA_TOG(pub u32);
impl PFDA_TOG {
    #[doc = "PFD0 Clock Gate."]
    #[must_use]
    #[inline(always)]
    pub const fn PFD0_CLKGATE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "PFD0 Clock Gate."]
    #[inline(always)]
    pub const fn set_PFD0_CLKGATE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "PFD0 Fractional Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn PFD0_FRAC(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x3f;
        val as u8
    }
    #[doc = "PFD0 Fractional Divider."]
    #[inline(always)]
    pub const fn set_PFD0_FRAC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 1usize)) | (((val as u32) & 0x3f) << 1usize);
    }
    #[doc = "PFD0 Stable Signal."]
    #[must_use]
    #[inline(always)]
    pub const fn PFD0_STABLE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "PFD0 Stable Signal."]
    #[inline(always)]
    pub const fn set_PFD0_STABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for PFDA_TOG {
    #[inline(always)]
    fn default() -> PFDA_TOG {
        PFDA_TOG(0)
    }
}
impl core::fmt::Debug for PFDA_TOG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PFDA_TOG")
            .field("PFD0_CLKGATE", &self.PFD0_CLKGATE())
            .field("PFD0_FRAC", &self.PFD0_FRAC())
            .field("PFD0_STABLE", &self.PFD0_STABLE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PFDA_TOG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PFDA_TOG {{ PFD0_CLKGATE: {=bool:?}, PFD0_FRAC: {=u8:?}, PFD0_STABLE: {=bool:?} }}",
            self.PFD0_CLKGATE(),
            self.PFD0_FRAC(),
            self.PFD0_STABLE()
        )
    }
}
#[doc = "PLL SIC."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PLL_SIC(pub u32);
impl PLL_SIC {
    #[doc = "Miscellaneous Control."]
    #[must_use]
    #[inline(always)]
    pub const fn MISC2_CONTROL0(&self) -> MISC2_CONTROL0 {
        let val = (self.0 >> 5usize) & 0x01;
        MISC2_CONTROL0::from_bits(val as u8)
    }
    #[doc = "Miscellaneous Control."]
    #[inline(always)]
    pub const fn set_MISC2_CONTROL0(&mut self, val: MISC2_CONTROL0) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "PLL Multi-Phase Clock Outputs Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_EN_USB_CLKS(&self) -> PLL_EN_USB_CLKS {
        let val = (self.0 >> 6usize) & 0x01;
        PLL_EN_USB_CLKS::from_bits(val as u8)
    }
    #[doc = "PLL Multi-Phase Clock Outputs Enable."]
    #[inline(always)]
    pub const fn set_PLL_EN_USB_CLKS(&mut self, val: PLL_EN_USB_CLKS) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "USB PLL Powerup Control."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_POWER(&self) -> PLL_POWER {
        let val = (self.0 >> 12usize) & 0x01;
        PLL_POWER::from_bits(val as u8)
    }
    #[doc = "USB PLL Powerup Control."]
    #[inline(always)]
    pub const fn set_PLL_POWER(&mut self, val: PLL_POWER) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "PLL Output Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_ENABLE(&self) -> PLL_ENABLE {
        let val = (self.0 >> 13usize) & 0x01;
        PLL_ENABLE::from_bits(val as u8)
    }
    #[doc = "PLL Output Clock Enable."]
    #[inline(always)]
    pub const fn set_PLL_ENABLE(&mut self, val: PLL_ENABLE) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Bypass USB PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_BYPASS(&self) -> PLL_BYPASS {
        let val = (self.0 >> 16usize) & 0x01;
        PLL_BYPASS::from_bits(val as u8)
    }
    #[doc = "Bypass USB PLL."]
    #[inline(always)]
    pub const fn set_PLL_BYPASS(&mut self, val: PLL_BYPASS) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Reference Bias Power Control."]
    #[must_use]
    #[inline(always)]
    pub const fn REFBIAS_PWD_SEL(&self) -> REFBIAS_PWD_SEL {
        let val = (self.0 >> 19usize) & 0x01;
        REFBIAS_PWD_SEL::from_bits(val as u8)
    }
    #[doc = "Reference Bias Power Control."]
    #[inline(always)]
    pub const fn set_REFBIAS_PWD_SEL(&mut self, val: REFBIAS_PWD_SEL) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Power Down Reference Bias."]
    #[must_use]
    #[inline(always)]
    pub const fn REFBIAS_PWD(&self) -> REFBIAS_PWD {
        let val = (self.0 >> 20usize) & 0x01;
        REFBIAS_PWD::from_bits(val as u8)
    }
    #[doc = "Power Down Reference Bias."]
    #[inline(always)]
    pub const fn set_REFBIAS_PWD(&mut self, val: REFBIAS_PWD) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Enable PLL Regulator."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_REG_ENABLE(&self) -> PLL_REG_ENABLE {
        let val = (self.0 >> 21usize) & 0x01;
        PLL_REG_ENABLE::from_bits(val as u8)
    }
    #[doc = "Enable PLL Regulator."]
    #[inline(always)]
    pub const fn set_PLL_REG_ENABLE(&mut self, val: PLL_REG_ENABLE) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "PLL Divider Value Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_DIV_SEL(&self) -> PLL_DIV_SEL {
        let val = (self.0 >> 22usize) & 0x07;
        PLL_DIV_SEL::from_bits(val as u8)
    }
    #[doc = "PLL Divider Value Configuration."]
    #[inline(always)]
    pub const fn set_PLL_DIV_SEL(&mut self, val: PLL_DIV_SEL) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val.to_bits() as u32) & 0x07) << 22usize);
    }
    #[doc = "PLL Pre-Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_PREDIV(&self) -> PLL_PREDIV {
        let val = (self.0 >> 30usize) & 0x01;
        PLL_PREDIV::from_bits(val as u8)
    }
    #[doc = "PLL Pre-Divider."]
    #[inline(always)]
    pub const fn set_PLL_PREDIV(&mut self, val: PLL_PREDIV) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "USB PLL Lock Status Indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_LOCK(&self) -> PLL_LOCK {
        let val = (self.0 >> 31usize) & 0x01;
        PLL_LOCK::from_bits(val as u8)
    }
    #[doc = "USB PLL Lock Status Indicator."]
    #[inline(always)]
    pub const fn set_PLL_LOCK(&mut self, val: PLL_LOCK) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PLL_SIC {
    #[inline(always)]
    fn default() -> PLL_SIC {
        PLL_SIC(0)
    }
}
impl core::fmt::Debug for PLL_SIC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL_SIC")
            .field("MISC2_CONTROL0", &self.MISC2_CONTROL0())
            .field("PLL_EN_USB_CLKS", &self.PLL_EN_USB_CLKS())
            .field("PLL_POWER", &self.PLL_POWER())
            .field("PLL_ENABLE", &self.PLL_ENABLE())
            .field("PLL_BYPASS", &self.PLL_BYPASS())
            .field("REFBIAS_PWD_SEL", &self.REFBIAS_PWD_SEL())
            .field("REFBIAS_PWD", &self.REFBIAS_PWD())
            .field("PLL_REG_ENABLE", &self.PLL_REG_ENABLE())
            .field("PLL_DIV_SEL", &self.PLL_DIV_SEL())
            .field("PLL_PREDIV", &self.PLL_PREDIV())
            .field("PLL_LOCK", &self.PLL_LOCK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PLL_SIC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PLL_SIC {{ MISC2_CONTROL0: {:?}, PLL_EN_USB_CLKS: {:?}, PLL_POWER: {:?}, PLL_ENABLE: {:?}, PLL_BYPASS: {:?}, REFBIAS_PWD_SEL: {:?}, REFBIAS_PWD: {:?}, PLL_REG_ENABLE: {:?}, PLL_DIV_SEL: {:?}, PLL_PREDIV: {:?}, PLL_LOCK: {:?} }}",
            self.MISC2_CONTROL0(),
            self.PLL_EN_USB_CLKS(),
            self.PLL_POWER(),
            self.PLL_ENABLE(),
            self.PLL_BYPASS(),
            self.REFBIAS_PWD_SEL(),
            self.REFBIAS_PWD(),
            self.PLL_REG_ENABLE(),
            self.PLL_DIV_SEL(),
            self.PLL_PREDIV(),
            self.PLL_LOCK()
        )
    }
}
#[doc = "PLL SIC."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PLL_SIC_CLR(pub u32);
impl PLL_SIC_CLR {
    #[doc = "Miscellaneous Control."]
    #[must_use]
    #[inline(always)]
    pub const fn MISC2_CONTROL0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Miscellaneous Control."]
    #[inline(always)]
    pub const fn set_MISC2_CONTROL0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "PLL Multi-Phase Clock Outputs Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_EN_USB_CLKS(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "PLL Multi-Phase Clock Outputs Enable."]
    #[inline(always)]
    pub const fn set_PLL_EN_USB_CLKS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "USB PLL Powerup Control."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_POWER(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "USB PLL Powerup Control."]
    #[inline(always)]
    pub const fn set_PLL_POWER(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "PLL Output Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_ENABLE(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PLL Output Clock Enable."]
    #[inline(always)]
    pub const fn set_PLL_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Bypass USB PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_BYPASS(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass USB PLL."]
    #[inline(always)]
    pub const fn set_PLL_BYPASS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Reference Bias Power Control."]
    #[must_use]
    #[inline(always)]
    pub const fn REFBIAS_PWD_SEL(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Reference Bias Power Control."]
    #[inline(always)]
    pub const fn set_REFBIAS_PWD_SEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Power Down Reference Bias."]
    #[must_use]
    #[inline(always)]
    pub const fn REFBIAS_PWD(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down Reference Bias."]
    #[inline(always)]
    pub const fn set_REFBIAS_PWD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enable PLL Regulator."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_REG_ENABLE(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enable PLL Regulator."]
    #[inline(always)]
    pub const fn set_PLL_REG_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "PLL Divider Value Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_DIV_SEL(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x07;
        val as u8
    }
    #[doc = "PLL Divider Value Configuration."]
    #[inline(always)]
    pub const fn set_PLL_DIV_SEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val as u32) & 0x07) << 22usize);
    }
    #[doc = "PLL Pre-Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_PREDIV(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "PLL Pre-Divider."]
    #[inline(always)]
    pub const fn set_PLL_PREDIV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "USB PLL Lock Status Indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_LOCK(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "USB PLL Lock Status Indicator."]
    #[inline(always)]
    pub const fn set_PLL_LOCK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PLL_SIC_CLR {
    #[inline(always)]
    fn default() -> PLL_SIC_CLR {
        PLL_SIC_CLR(0)
    }
}
impl core::fmt::Debug for PLL_SIC_CLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL_SIC_CLR")
            .field("MISC2_CONTROL0", &self.MISC2_CONTROL0())
            .field("PLL_EN_USB_CLKS", &self.PLL_EN_USB_CLKS())
            .field("PLL_POWER", &self.PLL_POWER())
            .field("PLL_ENABLE", &self.PLL_ENABLE())
            .field("PLL_BYPASS", &self.PLL_BYPASS())
            .field("REFBIAS_PWD_SEL", &self.REFBIAS_PWD_SEL())
            .field("REFBIAS_PWD", &self.REFBIAS_PWD())
            .field("PLL_REG_ENABLE", &self.PLL_REG_ENABLE())
            .field("PLL_DIV_SEL", &self.PLL_DIV_SEL())
            .field("PLL_PREDIV", &self.PLL_PREDIV())
            .field("PLL_LOCK", &self.PLL_LOCK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PLL_SIC_CLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PLL_SIC_CLR {{ MISC2_CONTROL0: {=bool:?}, PLL_EN_USB_CLKS: {=bool:?}, PLL_POWER: {=bool:?}, PLL_ENABLE: {=bool:?}, PLL_BYPASS: {=bool:?}, REFBIAS_PWD_SEL: {=bool:?}, REFBIAS_PWD: {=bool:?}, PLL_REG_ENABLE: {=bool:?}, PLL_DIV_SEL: {=u8:?}, PLL_PREDIV: {=bool:?}, PLL_LOCK: {=bool:?} }}",
            self.MISC2_CONTROL0(),
            self.PLL_EN_USB_CLKS(),
            self.PLL_POWER(),
            self.PLL_ENABLE(),
            self.PLL_BYPASS(),
            self.REFBIAS_PWD_SEL(),
            self.REFBIAS_PWD(),
            self.PLL_REG_ENABLE(),
            self.PLL_DIV_SEL(),
            self.PLL_PREDIV(),
            self.PLL_LOCK()
        )
    }
}
#[doc = "PLL SIC."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PLL_SIC_SET(pub u32);
impl PLL_SIC_SET {
    #[doc = "Miscellaneous Control."]
    #[must_use]
    #[inline(always)]
    pub const fn MISC2_CONTROL0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Miscellaneous Control."]
    #[inline(always)]
    pub const fn set_MISC2_CONTROL0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "PLL Multi-Phase Clock Outputs Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_EN_USB_CLKS(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "PLL Multi-Phase Clock Outputs Enable."]
    #[inline(always)]
    pub const fn set_PLL_EN_USB_CLKS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "USB PLL Powerup Control."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_POWER(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "USB PLL Powerup Control."]
    #[inline(always)]
    pub const fn set_PLL_POWER(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "PLL Output Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_ENABLE(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PLL Output Clock Enable."]
    #[inline(always)]
    pub const fn set_PLL_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Bypass USB PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_BYPASS(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass USB PLL."]
    #[inline(always)]
    pub const fn set_PLL_BYPASS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Reference Bias Power Control."]
    #[must_use]
    #[inline(always)]
    pub const fn REFBIAS_PWD_SEL(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Reference Bias Power Control."]
    #[inline(always)]
    pub const fn set_REFBIAS_PWD_SEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Power Down Reference Bias."]
    #[must_use]
    #[inline(always)]
    pub const fn REFBIAS_PWD(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down Reference Bias."]
    #[inline(always)]
    pub const fn set_REFBIAS_PWD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enable PLL Regulator."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_REG_ENABLE(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enable PLL Regulator."]
    #[inline(always)]
    pub const fn set_PLL_REG_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "PLL Divider Value Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_DIV_SEL(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x07;
        val as u8
    }
    #[doc = "PLL Divider Value Configuration."]
    #[inline(always)]
    pub const fn set_PLL_DIV_SEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val as u32) & 0x07) << 22usize);
    }
    #[doc = "PLL Pre-Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_PREDIV(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "PLL Pre-Divider."]
    #[inline(always)]
    pub const fn set_PLL_PREDIV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "USB PLL Lock Status Indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_LOCK(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "USB PLL Lock Status Indicator."]
    #[inline(always)]
    pub const fn set_PLL_LOCK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PLL_SIC_SET {
    #[inline(always)]
    fn default() -> PLL_SIC_SET {
        PLL_SIC_SET(0)
    }
}
impl core::fmt::Debug for PLL_SIC_SET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL_SIC_SET")
            .field("MISC2_CONTROL0", &self.MISC2_CONTROL0())
            .field("PLL_EN_USB_CLKS", &self.PLL_EN_USB_CLKS())
            .field("PLL_POWER", &self.PLL_POWER())
            .field("PLL_ENABLE", &self.PLL_ENABLE())
            .field("PLL_BYPASS", &self.PLL_BYPASS())
            .field("REFBIAS_PWD_SEL", &self.REFBIAS_PWD_SEL())
            .field("REFBIAS_PWD", &self.REFBIAS_PWD())
            .field("PLL_REG_ENABLE", &self.PLL_REG_ENABLE())
            .field("PLL_DIV_SEL", &self.PLL_DIV_SEL())
            .field("PLL_PREDIV", &self.PLL_PREDIV())
            .field("PLL_LOCK", &self.PLL_LOCK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PLL_SIC_SET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PLL_SIC_SET {{ MISC2_CONTROL0: {=bool:?}, PLL_EN_USB_CLKS: {=bool:?}, PLL_POWER: {=bool:?}, PLL_ENABLE: {=bool:?}, PLL_BYPASS: {=bool:?}, REFBIAS_PWD_SEL: {=bool:?}, REFBIAS_PWD: {=bool:?}, PLL_REG_ENABLE: {=bool:?}, PLL_DIV_SEL: {=u8:?}, PLL_PREDIV: {=bool:?}, PLL_LOCK: {=bool:?} }}",
            self.MISC2_CONTROL0(),
            self.PLL_EN_USB_CLKS(),
            self.PLL_POWER(),
            self.PLL_ENABLE(),
            self.PLL_BYPASS(),
            self.REFBIAS_PWD_SEL(),
            self.REFBIAS_PWD(),
            self.PLL_REG_ENABLE(),
            self.PLL_DIV_SEL(),
            self.PLL_PREDIV(),
            self.PLL_LOCK()
        )
    }
}
#[doc = "PLL SIC."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PLL_SIC_TOG(pub u32);
impl PLL_SIC_TOG {
    #[doc = "Miscellaneous Control."]
    #[must_use]
    #[inline(always)]
    pub const fn MISC2_CONTROL0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Miscellaneous Control."]
    #[inline(always)]
    pub const fn set_MISC2_CONTROL0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "PLL Multi-Phase Clock Outputs Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_EN_USB_CLKS(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "PLL Multi-Phase Clock Outputs Enable."]
    #[inline(always)]
    pub const fn set_PLL_EN_USB_CLKS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "USB PLL Powerup Control."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_POWER(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "USB PLL Powerup Control."]
    #[inline(always)]
    pub const fn set_PLL_POWER(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "PLL Output Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_ENABLE(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PLL Output Clock Enable."]
    #[inline(always)]
    pub const fn set_PLL_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Bypass USB PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_BYPASS(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass USB PLL."]
    #[inline(always)]
    pub const fn set_PLL_BYPASS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Reference Bias Power Control."]
    #[must_use]
    #[inline(always)]
    pub const fn REFBIAS_PWD_SEL(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Reference Bias Power Control."]
    #[inline(always)]
    pub const fn set_REFBIAS_PWD_SEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Power Down Reference Bias."]
    #[must_use]
    #[inline(always)]
    pub const fn REFBIAS_PWD(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down Reference Bias."]
    #[inline(always)]
    pub const fn set_REFBIAS_PWD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enable PLL Regulator."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_REG_ENABLE(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enable PLL Regulator."]
    #[inline(always)]
    pub const fn set_PLL_REG_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "PLL Divider Value Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_DIV_SEL(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x07;
        val as u8
    }
    #[doc = "PLL Divider Value Configuration."]
    #[inline(always)]
    pub const fn set_PLL_DIV_SEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val as u32) & 0x07) << 22usize);
    }
    #[doc = "PLL Pre-Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_PREDIV(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "PLL Pre-Divider."]
    #[inline(always)]
    pub const fn set_PLL_PREDIV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "USB PLL Lock Status Indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_LOCK(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "USB PLL Lock Status Indicator."]
    #[inline(always)]
    pub const fn set_PLL_LOCK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PLL_SIC_TOG {
    #[inline(always)]
    fn default() -> PLL_SIC_TOG {
        PLL_SIC_TOG(0)
    }
}
impl core::fmt::Debug for PLL_SIC_TOG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL_SIC_TOG")
            .field("MISC2_CONTROL0", &self.MISC2_CONTROL0())
            .field("PLL_EN_USB_CLKS", &self.PLL_EN_USB_CLKS())
            .field("PLL_POWER", &self.PLL_POWER())
            .field("PLL_ENABLE", &self.PLL_ENABLE())
            .field("PLL_BYPASS", &self.PLL_BYPASS())
            .field("REFBIAS_PWD_SEL", &self.REFBIAS_PWD_SEL())
            .field("REFBIAS_PWD", &self.REFBIAS_PWD())
            .field("PLL_REG_ENABLE", &self.PLL_REG_ENABLE())
            .field("PLL_DIV_SEL", &self.PLL_DIV_SEL())
            .field("PLL_PREDIV", &self.PLL_PREDIV())
            .field("PLL_LOCK", &self.PLL_LOCK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PLL_SIC_TOG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PLL_SIC_TOG {{ MISC2_CONTROL0: {=bool:?}, PLL_EN_USB_CLKS: {=bool:?}, PLL_POWER: {=bool:?}, PLL_ENABLE: {=bool:?}, PLL_BYPASS: {=bool:?}, REFBIAS_PWD_SEL: {=bool:?}, REFBIAS_PWD: {=bool:?}, PLL_REG_ENABLE: {=bool:?}, PLL_DIV_SEL: {=u8:?}, PLL_PREDIV: {=bool:?}, PLL_LOCK: {=bool:?} }}",
            self.MISC2_CONTROL0(),
            self.PLL_EN_USB_CLKS(),
            self.PLL_POWER(),
            self.PLL_ENABLE(),
            self.PLL_BYPASS(),
            self.REFBIAS_PWD_SEL(),
            self.REFBIAS_PWD(),
            self.PLL_REG_ENABLE(),
            self.PLL_DIV_SEL(),
            self.PLL_PREDIV(),
            self.PLL_LOCK()
        )
    }
}
#[doc = "Power Down."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PWD(pub u32);
impl PWD {
    #[doc = "Power Down USB FS TX Drivers."]
    #[must_use]
    #[inline(always)]
    pub const fn TXPWDFS(&self) -> TXPWDFS {
        let val = (self.0 >> 10usize) & 0x01;
        TXPWDFS::from_bits(val as u8)
    }
    #[doc = "Power Down USB FS TX Drivers."]
    #[inline(always)]
    pub const fn set_TXPWDFS(&mut self, val: TXPWDFS) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Power Down USBPHY TX Current Bias Block."]
    #[must_use]
    #[inline(always)]
    pub const fn TXPWDIBIAS(&self) -> TXPWDIBIAS {
        let val = (self.0 >> 11usize) & 0x01;
        TXPWDIBIAS::from_bits(val as u8)
    }
    #[doc = "Power Down USBPHY TX Current Bias Block."]
    #[inline(always)]
    pub const fn set_TXPWDIBIAS(&mut self, val: TXPWDIBIAS) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Power Down USBPHY TX V-I Converter and Current Mirror."]
    #[must_use]
    #[inline(always)]
    pub const fn TXPWDV2I(&self) -> TXPWDV2I {
        let val = (self.0 >> 12usize) & 0x01;
        TXPWDV2I::from_bits(val as u8)
    }
    #[doc = "Power Down USBPHY TX V-I Converter and Current Mirror."]
    #[inline(always)]
    pub const fn set_TXPWDV2I(&mut self, val: TXPWDV2I) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Power Down USB HS RX Envelope Detector."]
    #[must_use]
    #[inline(always)]
    pub const fn RXPWDENV(&self) -> RXPWDENV {
        let val = (self.0 >> 17usize) & 0x01;
        RXPWDENV::from_bits(val as u8)
    }
    #[doc = "Power Down USB HS RX Envelope Detector."]
    #[inline(always)]
    pub const fn set_RXPWDENV(&mut self, val: RXPWDENV) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Power Down USB FS Differential Receiver."]
    #[must_use]
    #[inline(always)]
    pub const fn RXPWD1PT1(&self) -> RXPWD1PT1 {
        let val = (self.0 >> 18usize) & 0x01;
        RXPWD1PT1::from_bits(val as u8)
    }
    #[doc = "Power Down USB FS Differential Receiver."]
    #[inline(always)]
    pub const fn set_RXPWD1PT1(&mut self, val: RXPWD1PT1) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Power Down USB HS Differential Receiver."]
    #[must_use]
    #[inline(always)]
    pub const fn RXPWDDIFF(&self) -> RXPWDDIFF {
        let val = (self.0 >> 19usize) & 0x01;
        RXPWDDIFF::from_bits(val as u8)
    }
    #[doc = "Power Down USB HS Differential Receiver."]
    #[inline(always)]
    pub const fn set_RXPWDDIFF(&mut self, val: RXPWDDIFF) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Power Down USBPHY Receiver Circuits."]
    #[must_use]
    #[inline(always)]
    pub const fn RXPWDRX(&self) -> RXPWDRX {
        let val = (self.0 >> 20usize) & 0x01;
        RXPWDRX::from_bits(val as u8)
    }
    #[doc = "Power Down USBPHY Receiver Circuits."]
    #[inline(always)]
    pub const fn set_RXPWDRX(&mut self, val: RXPWDRX) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
}
impl Default for PWD {
    #[inline(always)]
    fn default() -> PWD {
        PWD(0)
    }
}
impl core::fmt::Debug for PWD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWD")
            .field("TXPWDFS", &self.TXPWDFS())
            .field("TXPWDIBIAS", &self.TXPWDIBIAS())
            .field("TXPWDV2I", &self.TXPWDV2I())
            .field("RXPWDENV", &self.RXPWDENV())
            .field("RXPWD1PT1", &self.RXPWD1PT1())
            .field("RXPWDDIFF", &self.RXPWDDIFF())
            .field("RXPWDRX", &self.RXPWDRX())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PWD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PWD {{ TXPWDFS: {:?}, TXPWDIBIAS: {:?}, TXPWDV2I: {:?}, RXPWDENV: {:?}, RXPWD1PT1: {:?}, RXPWDDIFF: {:?}, RXPWDRX: {:?} }}",
            self.TXPWDFS(),
            self.TXPWDIBIAS(),
            self.TXPWDV2I(),
            self.RXPWDENV(),
            self.RXPWD1PT1(),
            self.RXPWDDIFF(),
            self.RXPWDRX()
        )
    }
}
#[doc = "Power Down."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PWD_CLR(pub u32);
impl PWD_CLR {
    #[doc = "Power Down USB FS TX Drivers."]
    #[must_use]
    #[inline(always)]
    pub const fn TXPWDFS(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USB FS TX Drivers."]
    #[inline(always)]
    pub const fn set_TXPWDFS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Power Down USBPHY TX Current Bias Block."]
    #[must_use]
    #[inline(always)]
    pub const fn TXPWDIBIAS(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USBPHY TX Current Bias Block."]
    #[inline(always)]
    pub const fn set_TXPWDIBIAS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Power Down USBPHY TX V-I Converter and Current Mirror."]
    #[must_use]
    #[inline(always)]
    pub const fn TXPWDV2I(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USBPHY TX V-I Converter and Current Mirror."]
    #[inline(always)]
    pub const fn set_TXPWDV2I(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Power Down USB HS RX Envelope Detector."]
    #[must_use]
    #[inline(always)]
    pub const fn RXPWDENV(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USB HS RX Envelope Detector."]
    #[inline(always)]
    pub const fn set_RXPWDENV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Power Down USB FS Differential Receiver."]
    #[must_use]
    #[inline(always)]
    pub const fn RXPWD1PT1(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USB FS Differential Receiver."]
    #[inline(always)]
    pub const fn set_RXPWD1PT1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Power Down USB HS Differential Receiver."]
    #[must_use]
    #[inline(always)]
    pub const fn RXPWDDIFF(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USB HS Differential Receiver."]
    #[inline(always)]
    pub const fn set_RXPWDDIFF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Power Down USBPHY Receiver Circuits."]
    #[must_use]
    #[inline(always)]
    pub const fn RXPWDRX(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USBPHY Receiver Circuits."]
    #[inline(always)]
    pub const fn set_RXPWDRX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for PWD_CLR {
    #[inline(always)]
    fn default() -> PWD_CLR {
        PWD_CLR(0)
    }
}
impl core::fmt::Debug for PWD_CLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWD_CLR")
            .field("TXPWDFS", &self.TXPWDFS())
            .field("TXPWDIBIAS", &self.TXPWDIBIAS())
            .field("TXPWDV2I", &self.TXPWDV2I())
            .field("RXPWDENV", &self.RXPWDENV())
            .field("RXPWD1PT1", &self.RXPWD1PT1())
            .field("RXPWDDIFF", &self.RXPWDDIFF())
            .field("RXPWDRX", &self.RXPWDRX())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PWD_CLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PWD_CLR {{ TXPWDFS: {=bool:?}, TXPWDIBIAS: {=bool:?}, TXPWDV2I: {=bool:?}, RXPWDENV: {=bool:?}, RXPWD1PT1: {=bool:?}, RXPWDDIFF: {=bool:?}, RXPWDRX: {=bool:?} }}",
            self.TXPWDFS(),
            self.TXPWDIBIAS(),
            self.TXPWDV2I(),
            self.RXPWDENV(),
            self.RXPWD1PT1(),
            self.RXPWDDIFF(),
            self.RXPWDRX()
        )
    }
}
#[doc = "Power Down."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PWD_SET(pub u32);
impl PWD_SET {
    #[doc = "Power Down USB FS TX Drivers."]
    #[must_use]
    #[inline(always)]
    pub const fn TXPWDFS(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USB FS TX Drivers."]
    #[inline(always)]
    pub const fn set_TXPWDFS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Power Down USBPHY TX Current Bias Block."]
    #[must_use]
    #[inline(always)]
    pub const fn TXPWDIBIAS(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USBPHY TX Current Bias Block."]
    #[inline(always)]
    pub const fn set_TXPWDIBIAS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Power Down USBPHY TX V-I Converter and Current Mirror."]
    #[must_use]
    #[inline(always)]
    pub const fn TXPWDV2I(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USBPHY TX V-I Converter and Current Mirror."]
    #[inline(always)]
    pub const fn set_TXPWDV2I(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Power Down USB HS RX Envelope Detector."]
    #[must_use]
    #[inline(always)]
    pub const fn RXPWDENV(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USB HS RX Envelope Detector."]
    #[inline(always)]
    pub const fn set_RXPWDENV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Power Down USB FS Differential Receiver."]
    #[must_use]
    #[inline(always)]
    pub const fn RXPWD1PT1(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USB FS Differential Receiver."]
    #[inline(always)]
    pub const fn set_RXPWD1PT1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Power Down USB HS Differential Receiver."]
    #[must_use]
    #[inline(always)]
    pub const fn RXPWDDIFF(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USB HS Differential Receiver."]
    #[inline(always)]
    pub const fn set_RXPWDDIFF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Power Down USBPHY Receiver Circuits."]
    #[must_use]
    #[inline(always)]
    pub const fn RXPWDRX(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USBPHY Receiver Circuits."]
    #[inline(always)]
    pub const fn set_RXPWDRX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for PWD_SET {
    #[inline(always)]
    fn default() -> PWD_SET {
        PWD_SET(0)
    }
}
impl core::fmt::Debug for PWD_SET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWD_SET")
            .field("TXPWDFS", &self.TXPWDFS())
            .field("TXPWDIBIAS", &self.TXPWDIBIAS())
            .field("TXPWDV2I", &self.TXPWDV2I())
            .field("RXPWDENV", &self.RXPWDENV())
            .field("RXPWD1PT1", &self.RXPWD1PT1())
            .field("RXPWDDIFF", &self.RXPWDDIFF())
            .field("RXPWDRX", &self.RXPWDRX())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PWD_SET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PWD_SET {{ TXPWDFS: {=bool:?}, TXPWDIBIAS: {=bool:?}, TXPWDV2I: {=bool:?}, RXPWDENV: {=bool:?}, RXPWD1PT1: {=bool:?}, RXPWDDIFF: {=bool:?}, RXPWDRX: {=bool:?} }}",
            self.TXPWDFS(),
            self.TXPWDIBIAS(),
            self.TXPWDV2I(),
            self.RXPWDENV(),
            self.RXPWD1PT1(),
            self.RXPWDDIFF(),
            self.RXPWDRX()
        )
    }
}
#[doc = "Power Down."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PWD_TOG(pub u32);
impl PWD_TOG {
    #[doc = "Power Down USB FS TX Drivers."]
    #[must_use]
    #[inline(always)]
    pub const fn TXPWDFS(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USB FS TX Drivers."]
    #[inline(always)]
    pub const fn set_TXPWDFS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Power Down USBPHY TX Current Bias Block."]
    #[must_use]
    #[inline(always)]
    pub const fn TXPWDIBIAS(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USBPHY TX Current Bias Block."]
    #[inline(always)]
    pub const fn set_TXPWDIBIAS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Power Down USBPHY TX V-I Converter and Current Mirror."]
    #[must_use]
    #[inline(always)]
    pub const fn TXPWDV2I(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USBPHY TX V-I Converter and Current Mirror."]
    #[inline(always)]
    pub const fn set_TXPWDV2I(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Power Down USB HS RX Envelope Detector."]
    #[must_use]
    #[inline(always)]
    pub const fn RXPWDENV(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USB HS RX Envelope Detector."]
    #[inline(always)]
    pub const fn set_RXPWDENV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Power Down USB FS Differential Receiver."]
    #[must_use]
    #[inline(always)]
    pub const fn RXPWD1PT1(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USB FS Differential Receiver."]
    #[inline(always)]
    pub const fn set_RXPWD1PT1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Power Down USB HS Differential Receiver."]
    #[must_use]
    #[inline(always)]
    pub const fn RXPWDDIFF(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USB HS Differential Receiver."]
    #[inline(always)]
    pub const fn set_RXPWDDIFF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Power Down USBPHY Receiver Circuits."]
    #[must_use]
    #[inline(always)]
    pub const fn RXPWDRX(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USBPHY Receiver Circuits."]
    #[inline(always)]
    pub const fn set_RXPWDRX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for PWD_TOG {
    #[inline(always)]
    fn default() -> PWD_TOG {
        PWD_TOG(0)
    }
}
impl core::fmt::Debug for PWD_TOG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWD_TOG")
            .field("TXPWDFS", &self.TXPWDFS())
            .field("TXPWDIBIAS", &self.TXPWDIBIAS())
            .field("TXPWDV2I", &self.TXPWDV2I())
            .field("RXPWDENV", &self.RXPWDENV())
            .field("RXPWD1PT1", &self.RXPWD1PT1())
            .field("RXPWDDIFF", &self.RXPWDDIFF())
            .field("RXPWDRX", &self.RXPWDRX())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PWD_TOG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PWD_TOG {{ TXPWDFS: {=bool:?}, TXPWDIBIAS: {=bool:?}, TXPWDV2I: {=bool:?}, RXPWDENV: {=bool:?}, RXPWD1PT1: {=bool:?}, RXPWDDIFF: {=bool:?}, RXPWDRX: {=bool:?} }}",
            self.TXPWDFS(),
            self.TXPWDIBIAS(),
            self.TXPWDV2I(),
            self.RXPWDENV(),
            self.RXPWD1PT1(),
            self.RXPWDDIFF(),
            self.RXPWDRX()
        )
    }
}
#[doc = "RX Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RX(pub u32);
impl RX {
    #[doc = "Envelope Detector Trip Point."]
    #[must_use]
    #[inline(always)]
    pub const fn ENVADJ(&self) -> ENVADJ {
        let val = (self.0 >> 0usize) & 0x07;
        ENVADJ::from_bits(val as u8)
    }
    #[doc = "Envelope Detector Trip Point."]
    #[inline(always)]
    pub const fn set_ENVADJ(&mut self, val: ENVADJ) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Disconnect Detector Trip Point."]
    #[must_use]
    #[inline(always)]
    pub const fn DISCONADJ(&self) -> DISCONADJ {
        let val = (self.0 >> 4usize) & 0x07;
        DISCONADJ::from_bits(val as u8)
    }
    #[doc = "Disconnect Detector Trip Point."]
    #[inline(always)]
    pub const fn set_DISCONADJ(&mut self, val: DISCONADJ) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
}
impl Default for RX {
    #[inline(always)]
    fn default() -> RX {
        RX(0)
    }
}
impl core::fmt::Debug for RX {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX")
            .field("ENVADJ", &self.ENVADJ())
            .field("DISCONADJ", &self.DISCONADJ())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RX {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RX {{ ENVADJ: {:?}, DISCONADJ: {:?} }}",
            self.ENVADJ(),
            self.DISCONADJ()
        )
    }
}
#[doc = "RX Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RX_CLR(pub u32);
impl RX_CLR {
    #[doc = "Envelope Detector Trip Point."]
    #[must_use]
    #[inline(always)]
    pub const fn ENVADJ(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Envelope Detector Trip Point."]
    #[inline(always)]
    pub const fn set_ENVADJ(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Disconnect Detector Trip Point."]
    #[must_use]
    #[inline(always)]
    pub const fn DISCONADJ(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Disconnect Detector Trip Point."]
    #[inline(always)]
    pub const fn set_DISCONADJ(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
}
impl Default for RX_CLR {
    #[inline(always)]
    fn default() -> RX_CLR {
        RX_CLR(0)
    }
}
impl core::fmt::Debug for RX_CLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_CLR")
            .field("ENVADJ", &self.ENVADJ())
            .field("DISCONADJ", &self.DISCONADJ())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RX_CLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RX_CLR {{ ENVADJ: {=u8:?}, DISCONADJ: {=u8:?} }}",
            self.ENVADJ(),
            self.DISCONADJ()
        )
    }
}
#[doc = "RX Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RX_SET(pub u32);
impl RX_SET {
    #[doc = "Envelope Detector Trip Point."]
    #[must_use]
    #[inline(always)]
    pub const fn ENVADJ(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Envelope Detector Trip Point."]
    #[inline(always)]
    pub const fn set_ENVADJ(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Disconnect Detector Trip Point."]
    #[must_use]
    #[inline(always)]
    pub const fn DISCONADJ(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Disconnect Detector Trip Point."]
    #[inline(always)]
    pub const fn set_DISCONADJ(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
}
impl Default for RX_SET {
    #[inline(always)]
    fn default() -> RX_SET {
        RX_SET(0)
    }
}
impl core::fmt::Debug for RX_SET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_SET")
            .field("ENVADJ", &self.ENVADJ())
            .field("DISCONADJ", &self.DISCONADJ())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RX_SET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RX_SET {{ ENVADJ: {=u8:?}, DISCONADJ: {=u8:?} }}",
            self.ENVADJ(),
            self.DISCONADJ()
        )
    }
}
#[doc = "RX Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RX_TOG(pub u32);
impl RX_TOG {
    #[doc = "Envelope Detector Trip Point."]
    #[must_use]
    #[inline(always)]
    pub const fn ENVADJ(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Envelope Detector Trip Point."]
    #[inline(always)]
    pub const fn set_ENVADJ(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Disconnect Detector Trip Point."]
    #[must_use]
    #[inline(always)]
    pub const fn DISCONADJ(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Disconnect Detector Trip Point."]
    #[inline(always)]
    pub const fn set_DISCONADJ(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
}
impl Default for RX_TOG {
    #[inline(always)]
    fn default() -> RX_TOG {
        RX_TOG(0)
    }
}
impl core::fmt::Debug for RX_TOG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_TOG")
            .field("ENVADJ", &self.ENVADJ())
            .field("DISCONADJ", &self.DISCONADJ())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RX_TOG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RX_TOG {{ ENVADJ: {=u8:?}, DISCONADJ: {=u8:?} }}",
            self.ENVADJ(),
            self.DISCONADJ()
        )
    }
}
#[doc = "Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STATUS(pub u32);
impl STATUS {
    #[doc = "USB 3.3 V and 1.8 V Supply Status."]
    #[must_use]
    #[inline(always)]
    pub const fn OK_STATUS_3V(&self) -> OK_STATUS_3V {
        let val = (self.0 >> 0usize) & 0x01;
        OK_STATUS_3V::from_bits(val as u8)
    }
    #[doc = "USB 3.3 V and 1.8 V Supply Status."]
    #[inline(always)]
    pub const fn set_OK_STATUS_3V(&mut self, val: OK_STATUS_3V) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Host Disconnect Status."]
    #[must_use]
    #[inline(always)]
    pub const fn HOSTDISCONDETECT_STATUS(&self) -> HOSTDISCONDETECT_STATUS {
        let val = (self.0 >> 3usize) & 0x01;
        HOSTDISCONDETECT_STATUS::from_bits(val as u8)
    }
    #[doc = "Host Disconnect Status."]
    #[inline(always)]
    pub const fn set_HOSTDISCONDETECT_STATUS(&mut self, val: HOSTDISCONDETECT_STATUS) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Status Indicator for Nonstandard Resistive Plugged-In Detection."]
    #[must_use]
    #[inline(always)]
    pub const fn DEVPLUGIN_STATUS(&self) -> DEVPLUGIN_STATUS {
        let val = (self.0 >> 6usize) & 0x01;
        DEVPLUGIN_STATUS::from_bits(val as u8)
    }
    #[doc = "Status Indicator for Nonstandard Resistive Plugged-In Detection."]
    #[inline(always)]
    pub const fn set_DEVPLUGIN_STATUS(&mut self, val: DEVPLUGIN_STATUS) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "OTG ID Status."]
    #[must_use]
    #[inline(always)]
    pub const fn OTGID_STATUS(&self) -> OTGID_STATUS {
        let val = (self.0 >> 8usize) & 0x01;
        OTGID_STATUS::from_bits(val as u8)
    }
    #[doc = "OTG ID Status."]
    #[inline(always)]
    pub const fn set_OTGID_STATUS(&mut self, val: OTGID_STATUS) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Resume Status."]
    #[must_use]
    #[inline(always)]
    pub const fn RESUME_STATUS(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Status."]
    #[inline(always)]
    pub const fn set_RESUME_STATUS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for STATUS {
    #[inline(always)]
    fn default() -> STATUS {
        STATUS(0)
    }
}
impl core::fmt::Debug for STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("OK_STATUS_3V", &self.OK_STATUS_3V())
            .field("HOSTDISCONDETECT_STATUS", &self.HOSTDISCONDETECT_STATUS())
            .field("DEVPLUGIN_STATUS", &self.DEVPLUGIN_STATUS())
            .field("OTGID_STATUS", &self.OTGID_STATUS())
            .field("RESUME_STATUS", &self.RESUME_STATUS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STATUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STATUS {{ OK_STATUS_3V: {:?}, HOSTDISCONDETECT_STATUS: {:?}, DEVPLUGIN_STATUS: {:?}, OTGID_STATUS: {:?}, RESUME_STATUS: {=bool:?} }}",
            self.OK_STATUS_3V(),
            self.HOSTDISCONDETECT_STATUS(),
            self.DEVPLUGIN_STATUS(),
            self.OTGID_STATUS(),
            self.RESUME_STATUS()
        )
    }
}
#[doc = "Trim."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TRIM_OVERRIDE_EN(pub u32);
impl TRIM_OVERRIDE_EN {
    #[doc = "Override Enable for PLL Divider Value."]
    #[must_use]
    #[inline(always)]
    pub const fn DIV_SEL_OVERRIDE(&self) -> DIV_SEL_OVERRIDE {
        let val = (self.0 >> 0usize) & 0x01;
        DIV_SEL_OVERRIDE::from_bits(val as u8)
    }
    #[doc = "Override Enable for PLL Divider Value."]
    #[inline(always)]
    pub const fn set_DIV_SEL_OVERRIDE(&mut self, val: DIV_SEL_OVERRIDE) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Override Enable for HS TX Output Current Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn TX_D_CAL_OVERRIDE(&self) -> TX_D_CAL_OVERRIDE {
        let val = (self.0 >> 2usize) & 0x01;
        TX_D_CAL_OVERRIDE::from_bits(val as u8)
    }
    #[doc = "Override Enable for HS TX Output Current Trim."]
    #[inline(always)]
    pub const fn set_TX_D_CAL_OVERRIDE(&mut self, val: TX_D_CAL_OVERRIDE) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Override Enable for USB_DP Series Termination Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn TX_CAL45DP_OVERRIDE(&self) -> TX_CAL45DP_OVERRIDE {
        let val = (self.0 >> 3usize) & 0x01;
        TX_CAL45DP_OVERRIDE::from_bits(val as u8)
    }
    #[doc = "Override Enable for USB_DP Series Termination Trim."]
    #[inline(always)]
    pub const fn set_TX_CAL45DP_OVERRIDE(&mut self, val: TX_CAL45DP_OVERRIDE) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Override Enable for USB_DM Series Termination Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn TX_CAL45DM_OVERRIDE(&self) -> TX_CAL45DM_OVERRIDE {
        let val = (self.0 >> 4usize) & 0x01;
        TX_CAL45DM_OVERRIDE::from_bits(val as u8)
    }
    #[doc = "Override Enable for USB_DM Series Termination Trim."]
    #[inline(always)]
    pub const fn set_TX_CAL45DM_OVERRIDE(&mut self, val: TX_CAL45DM_OVERRIDE) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "PLL Divider Value Configuration Bits from Outside USBPHY."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_CTRL0_DIV_SEL(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[doc = "PLL Divider Value Configuration Bits from Outside USBPHY."]
    #[inline(always)]
    pub const fn set_PLL_CTRL0_DIV_SEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[doc = "HS TX Output Current Trim Bits from Outside USBPHY."]
    #[must_use]
    #[inline(always)]
    pub const fn USBPHY_TX_D_CAL(&self) -> USBPHY_TX_D_CAL {
        let val = (self.0 >> 20usize) & 0x0f;
        USBPHY_TX_D_CAL::from_bits(val as u8)
    }
    #[doc = "HS TX Output Current Trim Bits from Outside USBPHY."]
    #[inline(always)]
    pub const fn set_USBPHY_TX_D_CAL(&mut self, val: USBPHY_TX_D_CAL) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
    #[doc = "DP Series Termination Resistance Trim Bits from Outside USBPHY."]
    #[must_use]
    #[inline(always)]
    pub const fn USBPHY_TX_CAL45DP(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "DP Series Termination Resistance Trim Bits from Outside USBPHY."]
    #[inline(always)]
    pub const fn set_USBPHY_TX_CAL45DP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "DM Series Termination Resistance Trim Bits from Outside USBPHY."]
    #[must_use]
    #[inline(always)]
    pub const fn USBPHY_TX_CAL45DN(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "DM Series Termination Resistance Trim Bits from Outside USBPHY."]
    #[inline(always)]
    pub const fn set_USBPHY_TX_CAL45DN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for TRIM_OVERRIDE_EN {
    #[inline(always)]
    fn default() -> TRIM_OVERRIDE_EN {
        TRIM_OVERRIDE_EN(0)
    }
}
impl core::fmt::Debug for TRIM_OVERRIDE_EN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TRIM_OVERRIDE_EN")
            .field("DIV_SEL_OVERRIDE", &self.DIV_SEL_OVERRIDE())
            .field("TX_D_CAL_OVERRIDE", &self.TX_D_CAL_OVERRIDE())
            .field("TX_CAL45DP_OVERRIDE", &self.TX_CAL45DP_OVERRIDE())
            .field("TX_CAL45DM_OVERRIDE", &self.TX_CAL45DM_OVERRIDE())
            .field("PLL_CTRL0_DIV_SEL", &self.PLL_CTRL0_DIV_SEL())
            .field("USBPHY_TX_D_CAL", &self.USBPHY_TX_D_CAL())
            .field("USBPHY_TX_CAL45DP", &self.USBPHY_TX_CAL45DP())
            .field("USBPHY_TX_CAL45DN", &self.USBPHY_TX_CAL45DN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TRIM_OVERRIDE_EN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TRIM_OVERRIDE_EN {{ DIV_SEL_OVERRIDE: {:?}, TX_D_CAL_OVERRIDE: {:?}, TX_CAL45DP_OVERRIDE: {:?}, TX_CAL45DM_OVERRIDE: {:?}, PLL_CTRL0_DIV_SEL: {=u8:?}, USBPHY_TX_D_CAL: {:?}, USBPHY_TX_CAL45DP: {=u8:?}, USBPHY_TX_CAL45DN: {=u8:?} }}",
            self.DIV_SEL_OVERRIDE(),
            self.TX_D_CAL_OVERRIDE(),
            self.TX_CAL45DP_OVERRIDE(),
            self.TX_CAL45DM_OVERRIDE(),
            self.PLL_CTRL0_DIV_SEL(),
            self.USBPHY_TX_D_CAL(),
            self.USBPHY_TX_CAL45DP(),
            self.USBPHY_TX_CAL45DN()
        )
    }
}
#[doc = "Trim."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TRIM_OVERRIDE_EN_CLR(pub u32);
impl TRIM_OVERRIDE_EN_CLR {
    #[doc = "Override Enable for PLL Divider Value."]
    #[must_use]
    #[inline(always)]
    pub const fn DIV_SEL_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Override Enable for PLL Divider Value."]
    #[inline(always)]
    pub const fn set_DIV_SEL_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Override Enable for HS TX Output Current Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn TX_D_CAL_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Override Enable for HS TX Output Current Trim."]
    #[inline(always)]
    pub const fn set_TX_D_CAL_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Override Enable for USB_DP Series Termination Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn TX_CAL45DP_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Override Enable for USB_DP Series Termination Trim."]
    #[inline(always)]
    pub const fn set_TX_CAL45DP_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Override Enable for USB_DM Series Termination Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn TX_CAL45DM_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Override Enable for USB_DM Series Termination Trim."]
    #[inline(always)]
    pub const fn set_TX_CAL45DM_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "PLL Divider Value Configuration Bits from Outside USBPHY."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_CTRL0_DIV_SEL(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[doc = "PLL Divider Value Configuration Bits from Outside USBPHY."]
    #[inline(always)]
    pub const fn set_PLL_CTRL0_DIV_SEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[doc = "HS TX Output Current Trim Bits from Outside USBPHY."]
    #[must_use]
    #[inline(always)]
    pub const fn USBPHY_TX_D_CAL(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "HS TX Output Current Trim Bits from Outside USBPHY."]
    #[inline(always)]
    pub const fn set_USBPHY_TX_D_CAL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "DP Series Termination Resistance Trim Bits from Outside USBPHY."]
    #[must_use]
    #[inline(always)]
    pub const fn USBPHY_TX_CAL45DP(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "DP Series Termination Resistance Trim Bits from Outside USBPHY."]
    #[inline(always)]
    pub const fn set_USBPHY_TX_CAL45DP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "DM Series Termination Resistance Trim Bits from Outside USBPHY."]
    #[must_use]
    #[inline(always)]
    pub const fn USBPHY_TX_CAL45DN(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "DM Series Termination Resistance Trim Bits from Outside USBPHY."]
    #[inline(always)]
    pub const fn set_USBPHY_TX_CAL45DN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for TRIM_OVERRIDE_EN_CLR {
    #[inline(always)]
    fn default() -> TRIM_OVERRIDE_EN_CLR {
        TRIM_OVERRIDE_EN_CLR(0)
    }
}
impl core::fmt::Debug for TRIM_OVERRIDE_EN_CLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TRIM_OVERRIDE_EN_CLR")
            .field("DIV_SEL_OVERRIDE", &self.DIV_SEL_OVERRIDE())
            .field("TX_D_CAL_OVERRIDE", &self.TX_D_CAL_OVERRIDE())
            .field("TX_CAL45DP_OVERRIDE", &self.TX_CAL45DP_OVERRIDE())
            .field("TX_CAL45DM_OVERRIDE", &self.TX_CAL45DM_OVERRIDE())
            .field("PLL_CTRL0_DIV_SEL", &self.PLL_CTRL0_DIV_SEL())
            .field("USBPHY_TX_D_CAL", &self.USBPHY_TX_D_CAL())
            .field("USBPHY_TX_CAL45DP", &self.USBPHY_TX_CAL45DP())
            .field("USBPHY_TX_CAL45DN", &self.USBPHY_TX_CAL45DN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TRIM_OVERRIDE_EN_CLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TRIM_OVERRIDE_EN_CLR {{ DIV_SEL_OVERRIDE: {=bool:?}, TX_D_CAL_OVERRIDE: {=bool:?}, TX_CAL45DP_OVERRIDE: {=bool:?}, TX_CAL45DM_OVERRIDE: {=bool:?}, PLL_CTRL0_DIV_SEL: {=u8:?}, USBPHY_TX_D_CAL: {=u8:?}, USBPHY_TX_CAL45DP: {=u8:?}, USBPHY_TX_CAL45DN: {=u8:?} }}",
            self.DIV_SEL_OVERRIDE(),
            self.TX_D_CAL_OVERRIDE(),
            self.TX_CAL45DP_OVERRIDE(),
            self.TX_CAL45DM_OVERRIDE(),
            self.PLL_CTRL0_DIV_SEL(),
            self.USBPHY_TX_D_CAL(),
            self.USBPHY_TX_CAL45DP(),
            self.USBPHY_TX_CAL45DN()
        )
    }
}
#[doc = "Trim."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TRIM_OVERRIDE_EN_SET(pub u32);
impl TRIM_OVERRIDE_EN_SET {
    #[doc = "Override Enable for PLL Divider Value."]
    #[must_use]
    #[inline(always)]
    pub const fn DIV_SEL_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Override Enable for PLL Divider Value."]
    #[inline(always)]
    pub const fn set_DIV_SEL_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Override Enable for HS TX Output Current Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn TX_D_CAL_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Override Enable for HS TX Output Current Trim."]
    #[inline(always)]
    pub const fn set_TX_D_CAL_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Override Enable for USB_DP Series Termination Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn TX_CAL45DP_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Override Enable for USB_DP Series Termination Trim."]
    #[inline(always)]
    pub const fn set_TX_CAL45DP_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Override Enable for USB_DM Series Termination Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn TX_CAL45DM_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Override Enable for USB_DM Series Termination Trim."]
    #[inline(always)]
    pub const fn set_TX_CAL45DM_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "PLL Divider Value Configuration Bits from Outside USBPHY."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_CTRL0_DIV_SEL(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[doc = "PLL Divider Value Configuration Bits from Outside USBPHY."]
    #[inline(always)]
    pub const fn set_PLL_CTRL0_DIV_SEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[doc = "HS TX Output Current Trim Bits from Outside USBPHY."]
    #[must_use]
    #[inline(always)]
    pub const fn USBPHY_TX_D_CAL(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "HS TX Output Current Trim Bits from Outside USBPHY."]
    #[inline(always)]
    pub const fn set_USBPHY_TX_D_CAL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "DP Series Termination Resistance Trim Bits from Outside USBPHY."]
    #[must_use]
    #[inline(always)]
    pub const fn USBPHY_TX_CAL45DP(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "DP Series Termination Resistance Trim Bits from Outside USBPHY."]
    #[inline(always)]
    pub const fn set_USBPHY_TX_CAL45DP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "DM Series Termination Resistance Trim Bits from Outside USBPHY."]
    #[must_use]
    #[inline(always)]
    pub const fn USBPHY_TX_CAL45DN(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "DM Series Termination Resistance Trim Bits from Outside USBPHY."]
    #[inline(always)]
    pub const fn set_USBPHY_TX_CAL45DN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for TRIM_OVERRIDE_EN_SET {
    #[inline(always)]
    fn default() -> TRIM_OVERRIDE_EN_SET {
        TRIM_OVERRIDE_EN_SET(0)
    }
}
impl core::fmt::Debug for TRIM_OVERRIDE_EN_SET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TRIM_OVERRIDE_EN_SET")
            .field("DIV_SEL_OVERRIDE", &self.DIV_SEL_OVERRIDE())
            .field("TX_D_CAL_OVERRIDE", &self.TX_D_CAL_OVERRIDE())
            .field("TX_CAL45DP_OVERRIDE", &self.TX_CAL45DP_OVERRIDE())
            .field("TX_CAL45DM_OVERRIDE", &self.TX_CAL45DM_OVERRIDE())
            .field("PLL_CTRL0_DIV_SEL", &self.PLL_CTRL0_DIV_SEL())
            .field("USBPHY_TX_D_CAL", &self.USBPHY_TX_D_CAL())
            .field("USBPHY_TX_CAL45DP", &self.USBPHY_TX_CAL45DP())
            .field("USBPHY_TX_CAL45DN", &self.USBPHY_TX_CAL45DN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TRIM_OVERRIDE_EN_SET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TRIM_OVERRIDE_EN_SET {{ DIV_SEL_OVERRIDE: {=bool:?}, TX_D_CAL_OVERRIDE: {=bool:?}, TX_CAL45DP_OVERRIDE: {=bool:?}, TX_CAL45DM_OVERRIDE: {=bool:?}, PLL_CTRL0_DIV_SEL: {=u8:?}, USBPHY_TX_D_CAL: {=u8:?}, USBPHY_TX_CAL45DP: {=u8:?}, USBPHY_TX_CAL45DN: {=u8:?} }}",
            self.DIV_SEL_OVERRIDE(),
            self.TX_D_CAL_OVERRIDE(),
            self.TX_CAL45DP_OVERRIDE(),
            self.TX_CAL45DM_OVERRIDE(),
            self.PLL_CTRL0_DIV_SEL(),
            self.USBPHY_TX_D_CAL(),
            self.USBPHY_TX_CAL45DP(),
            self.USBPHY_TX_CAL45DN()
        )
    }
}
#[doc = "Trim."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TRIM_OVERRIDE_EN_TOG(pub u32);
impl TRIM_OVERRIDE_EN_TOG {
    #[doc = "Override Enable for PLL Divider Value."]
    #[must_use]
    #[inline(always)]
    pub const fn DIV_SEL_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Override Enable for PLL Divider Value."]
    #[inline(always)]
    pub const fn set_DIV_SEL_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Override Enable for HS TX Output Current Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn TX_D_CAL_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Override Enable for HS TX Output Current Trim."]
    #[inline(always)]
    pub const fn set_TX_D_CAL_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Override Enable for USB_DP Series Termination Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn TX_CAL45DP_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Override Enable for USB_DP Series Termination Trim."]
    #[inline(always)]
    pub const fn set_TX_CAL45DP_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Override Enable for USB_DM Series Termination Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn TX_CAL45DM_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Override Enable for USB_DM Series Termination Trim."]
    #[inline(always)]
    pub const fn set_TX_CAL45DM_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "PLL Divider Value Configuration Bits from Outside USBPHY."]
    #[must_use]
    #[inline(always)]
    pub const fn PLL_CTRL0_DIV_SEL(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[doc = "PLL Divider Value Configuration Bits from Outside USBPHY."]
    #[inline(always)]
    pub const fn set_PLL_CTRL0_DIV_SEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[doc = "HS TX Output Current Trim Bits from Outside USBPHY."]
    #[must_use]
    #[inline(always)]
    pub const fn USBPHY_TX_D_CAL(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "HS TX Output Current Trim Bits from Outside USBPHY."]
    #[inline(always)]
    pub const fn set_USBPHY_TX_D_CAL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "DP Series Termination Resistance Trim Bits from Outside USBPHY."]
    #[must_use]
    #[inline(always)]
    pub const fn USBPHY_TX_CAL45DP(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "DP Series Termination Resistance Trim Bits from Outside USBPHY."]
    #[inline(always)]
    pub const fn set_USBPHY_TX_CAL45DP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "DM Series Termination Resistance Trim Bits from Outside USBPHY."]
    #[must_use]
    #[inline(always)]
    pub const fn USBPHY_TX_CAL45DN(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "DM Series Termination Resistance Trim Bits from Outside USBPHY."]
    #[inline(always)]
    pub const fn set_USBPHY_TX_CAL45DN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for TRIM_OVERRIDE_EN_TOG {
    #[inline(always)]
    fn default() -> TRIM_OVERRIDE_EN_TOG {
        TRIM_OVERRIDE_EN_TOG(0)
    }
}
impl core::fmt::Debug for TRIM_OVERRIDE_EN_TOG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TRIM_OVERRIDE_EN_TOG")
            .field("DIV_SEL_OVERRIDE", &self.DIV_SEL_OVERRIDE())
            .field("TX_D_CAL_OVERRIDE", &self.TX_D_CAL_OVERRIDE())
            .field("TX_CAL45DP_OVERRIDE", &self.TX_CAL45DP_OVERRIDE())
            .field("TX_CAL45DM_OVERRIDE", &self.TX_CAL45DM_OVERRIDE())
            .field("PLL_CTRL0_DIV_SEL", &self.PLL_CTRL0_DIV_SEL())
            .field("USBPHY_TX_D_CAL", &self.USBPHY_TX_D_CAL())
            .field("USBPHY_TX_CAL45DP", &self.USBPHY_TX_CAL45DP())
            .field("USBPHY_TX_CAL45DN", &self.USBPHY_TX_CAL45DN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TRIM_OVERRIDE_EN_TOG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TRIM_OVERRIDE_EN_TOG {{ DIV_SEL_OVERRIDE: {=bool:?}, TX_D_CAL_OVERRIDE: {=bool:?}, TX_CAL45DP_OVERRIDE: {=bool:?}, TX_CAL45DM_OVERRIDE: {=bool:?}, PLL_CTRL0_DIV_SEL: {=u8:?}, USBPHY_TX_D_CAL: {=u8:?}, USBPHY_TX_CAL45DP: {=u8:?}, USBPHY_TX_CAL45DN: {=u8:?} }}",
            self.DIV_SEL_OVERRIDE(),
            self.TX_D_CAL_OVERRIDE(),
            self.TX_CAL45DP_OVERRIDE(),
            self.TX_CAL45DM_OVERRIDE(),
            self.PLL_CTRL0_DIV_SEL(),
            self.USBPHY_TX_D_CAL(),
            self.USBPHY_TX_CAL45DP(),
            self.USBPHY_TX_CAL45DN()
        )
    }
}
#[doc = "TX Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TX(pub u32);
impl TX {
    #[doc = "HS TX Output Current Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn D_CAL(&self) -> D_CAL {
        let val = (self.0 >> 0usize) & 0x0f;
        D_CAL::from_bits(val as u8)
    }
    #[doc = "HS TX Output Current Trim."]
    #[inline(always)]
    pub const fn set_D_CAL(&mut self, val: D_CAL) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "DM Series Termination Resistance Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn TXCAL45DN(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "DM Series Termination Resistance Trim."]
    #[inline(always)]
    pub const fn set_TXCAL45DN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "DP Series Termination Resistance Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn TXCAL45DP(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "DP Series Termination Resistance Trim."]
    #[inline(always)]
    pub const fn set_TXCAL45DP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for TX {
    #[inline(always)]
    fn default() -> TX {
        TX(0)
    }
}
impl core::fmt::Debug for TX {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX")
            .field("D_CAL", &self.D_CAL())
            .field("TXCAL45DN", &self.TXCAL45DN())
            .field("TXCAL45DP", &self.TXCAL45DP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TX {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TX {{ D_CAL: {:?}, TXCAL45DN: {=u8:?}, TXCAL45DP: {=u8:?} }}",
            self.D_CAL(),
            self.TXCAL45DN(),
            self.TXCAL45DP()
        )
    }
}
#[doc = "TX Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TX_CLR(pub u32);
impl TX_CLR {
    #[doc = "HS TX Output Current Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn D_CAL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "HS TX Output Current Trim."]
    #[inline(always)]
    pub const fn set_D_CAL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "DM Series Termination Resistance Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn TXCAL45DN(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "DM Series Termination Resistance Trim."]
    #[inline(always)]
    pub const fn set_TXCAL45DN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "DP Series Termination Resistance Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn TXCAL45DP(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "DP Series Termination Resistance Trim."]
    #[inline(always)]
    pub const fn set_TXCAL45DP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for TX_CLR {
    #[inline(always)]
    fn default() -> TX_CLR {
        TX_CLR(0)
    }
}
impl core::fmt::Debug for TX_CLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_CLR")
            .field("D_CAL", &self.D_CAL())
            .field("TXCAL45DN", &self.TXCAL45DN())
            .field("TXCAL45DP", &self.TXCAL45DP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TX_CLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TX_CLR {{ D_CAL: {=u8:?}, TXCAL45DN: {=u8:?}, TXCAL45DP: {=u8:?} }}",
            self.D_CAL(),
            self.TXCAL45DN(),
            self.TXCAL45DP()
        )
    }
}
#[doc = "TX Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TX_SET(pub u32);
impl TX_SET {
    #[doc = "HS TX Output Current Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn D_CAL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "HS TX Output Current Trim."]
    #[inline(always)]
    pub const fn set_D_CAL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "DM Series Termination Resistance Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn TXCAL45DN(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "DM Series Termination Resistance Trim."]
    #[inline(always)]
    pub const fn set_TXCAL45DN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "DP Series Termination Resistance Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn TXCAL45DP(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "DP Series Termination Resistance Trim."]
    #[inline(always)]
    pub const fn set_TXCAL45DP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for TX_SET {
    #[inline(always)]
    fn default() -> TX_SET {
        TX_SET(0)
    }
}
impl core::fmt::Debug for TX_SET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_SET")
            .field("D_CAL", &self.D_CAL())
            .field("TXCAL45DN", &self.TXCAL45DN())
            .field("TXCAL45DP", &self.TXCAL45DP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TX_SET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TX_SET {{ D_CAL: {=u8:?}, TXCAL45DN: {=u8:?}, TXCAL45DP: {=u8:?} }}",
            self.D_CAL(),
            self.TXCAL45DN(),
            self.TXCAL45DP()
        )
    }
}
#[doc = "TX Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TX_TOG(pub u32);
impl TX_TOG {
    #[doc = "HS TX Output Current Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn D_CAL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "HS TX Output Current Trim."]
    #[inline(always)]
    pub const fn set_D_CAL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "DM Series Termination Resistance Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn TXCAL45DN(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "DM Series Termination Resistance Trim."]
    #[inline(always)]
    pub const fn set_TXCAL45DN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "DP Series Termination Resistance Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn TXCAL45DP(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "DP Series Termination Resistance Trim."]
    #[inline(always)]
    pub const fn set_TXCAL45DP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for TX_TOG {
    #[inline(always)]
    fn default() -> TX_TOG {
        TX_TOG(0)
    }
}
impl core::fmt::Debug for TX_TOG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_TOG")
            .field("D_CAL", &self.D_CAL())
            .field("TXCAL45DN", &self.TXCAL45DN())
            .field("TXCAL45DP", &self.TXCAL45DP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TX_TOG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TX_TOG {{ D_CAL: {=u8:?}, TXCAL45DN: {=u8:?}, TXCAL45DP: {=u8:?} }}",
            self.D_CAL(),
            self.TXCAL45DN(),
            self.TXCAL45DP()
        )
    }
}
#[doc = "Charger Detect."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USB1_CHRG_DETECT(pub u32);
impl USB1_CHRG_DETECT {
    #[doc = "Secondary Detection Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn DETECT_SEC(&self) -> DETECT_SEC {
        let val = (self.0 >> 1usize) & 0x01;
        DETECT_SEC::from_bits(val as u8)
    }
    #[doc = "Secondary Detection Function Enable."]
    #[inline(always)]
    pub const fn set_DETECT_SEC(&mut self, val: DETECT_SEC) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "DP Pullup Resistor Enable Override Control."]
    #[must_use]
    #[inline(always)]
    pub const fn PULLUP_DP(&self) -> PULLUP_DP {
        let val = (self.0 >> 2usize) & 0x01;
        PULLUP_DP::from_bits(val as u8)
    }
    #[doc = "DP Pullup Resistor Enable Override Control."]
    #[inline(always)]
    pub const fn set_PULLUP_DP(&mut self, val: PULLUP_DP) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "VDM_SRC Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn VDM_SRC_ENABLE(&self) -> VDM_SRC_ENABLE {
        let val = (self.0 >> 4usize) & 0x01;
        VDM_SRC_ENABLE::from_bits(val as u8)
    }
    #[doc = "VDM_SRC Function Enable."]
    #[inline(always)]
    pub const fn set_VDM_SRC_ENABLE(&mut self, val: VDM_SRC_ENABLE) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "BC Data Contact Detect Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn CHK_CONTACT(&self) -> CHK_CONTACT {
        let val = (self.0 >> 18usize) & 0x01;
        CHK_CONTACT::from_bits(val as u8)
    }
    #[doc = "BC Data Contact Detect Function Enable."]
    #[inline(always)]
    pub const fn set_CHK_CONTACT(&mut self, val: CHK_CONTACT) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "BC Charger Detection Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn CHK_CHRG_B(&self) -> CHK_CHRG_B {
        let val = (self.0 >> 19usize) & 0x01;
        CHK_CHRG_B::from_bits(val as u8)
    }
    #[doc = "BC Charger Detection Function Enable."]
    #[inline(always)]
    pub const fn set_CHK_CHRG_B(&mut self, val: CHK_CHRG_B) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Selection of BC v1.2 Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn EN_B(&self) -> EN_B {
        let val = (self.0 >> 20usize) & 0x01;
        EN_B::from_bits(val as u8)
    }
    #[doc = "Selection of BC v1.2 Function Enable."]
    #[inline(always)]
    pub const fn set_EN_B(&mut self, val: EN_B) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "DCD Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn DCDSEL(&self) -> DCDSEL {
        let val = (self.0 >> 31usize) & 0x01;
        DCDSEL::from_bits(val as u8)
    }
    #[doc = "DCD Selection."]
    #[inline(always)]
    pub const fn set_DCDSEL(&mut self, val: DCDSEL) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for USB1_CHRG_DETECT {
    #[inline(always)]
    fn default() -> USB1_CHRG_DETECT {
        USB1_CHRG_DETECT(0)
    }
}
impl core::fmt::Debug for USB1_CHRG_DETECT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB1_CHRG_DETECT")
            .field("DETECT_SEC", &self.DETECT_SEC())
            .field("PULLUP_DP", &self.PULLUP_DP())
            .field("VDM_SRC_ENABLE", &self.VDM_SRC_ENABLE())
            .field("CHK_CONTACT", &self.CHK_CONTACT())
            .field("CHK_CHRG_B", &self.CHK_CHRG_B())
            .field("EN_B", &self.EN_B())
            .field("DCDSEL", &self.DCDSEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USB1_CHRG_DETECT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USB1_CHRG_DETECT {{ DETECT_SEC: {:?}, PULLUP_DP: {:?}, VDM_SRC_ENABLE: {:?}, CHK_CONTACT: {:?}, CHK_CHRG_B: {:?}, EN_B: {:?}, DCDSEL: {:?} }}",
            self.DETECT_SEC(),
            self.PULLUP_DP(),
            self.VDM_SRC_ENABLE(),
            self.CHK_CONTACT(),
            self.CHK_CHRG_B(),
            self.EN_B(),
            self.DCDSEL()
        )
    }
}
#[doc = "Charger Detect."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USB1_CHRG_DETECT_CLR(pub u32);
impl USB1_CHRG_DETECT_CLR {
    #[doc = "Secondary Detection Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn DETECT_SEC(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Secondary Detection Function Enable."]
    #[inline(always)]
    pub const fn set_DETECT_SEC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DP Pullup Resistor Enable Override Control."]
    #[must_use]
    #[inline(always)]
    pub const fn PULLUP_DP(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DP Pullup Resistor Enable Override Control."]
    #[inline(always)]
    pub const fn set_PULLUP_DP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "VDM_SRC Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn VDM_SRC_ENABLE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "VDM_SRC Function Enable."]
    #[inline(always)]
    pub const fn set_VDM_SRC_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "BC Data Contact Detect Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn CHK_CONTACT(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "BC Data Contact Detect Function Enable."]
    #[inline(always)]
    pub const fn set_CHK_CONTACT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "BC Charger Detection Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn CHK_CHRG_B(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "BC Charger Detection Function Enable."]
    #[inline(always)]
    pub const fn set_CHK_CHRG_B(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Selection of BC v1.2 Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn EN_B(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Selection of BC v1.2 Function Enable."]
    #[inline(always)]
    pub const fn set_EN_B(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DCD Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn DCDSEL(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DCD Selection."]
    #[inline(always)]
    pub const fn set_DCDSEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for USB1_CHRG_DETECT_CLR {
    #[inline(always)]
    fn default() -> USB1_CHRG_DETECT_CLR {
        USB1_CHRG_DETECT_CLR(0)
    }
}
impl core::fmt::Debug for USB1_CHRG_DETECT_CLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB1_CHRG_DETECT_CLR")
            .field("DETECT_SEC", &self.DETECT_SEC())
            .field("PULLUP_DP", &self.PULLUP_DP())
            .field("VDM_SRC_ENABLE", &self.VDM_SRC_ENABLE())
            .field("CHK_CONTACT", &self.CHK_CONTACT())
            .field("CHK_CHRG_B", &self.CHK_CHRG_B())
            .field("EN_B", &self.EN_B())
            .field("DCDSEL", &self.DCDSEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USB1_CHRG_DETECT_CLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USB1_CHRG_DETECT_CLR {{ DETECT_SEC: {=bool:?}, PULLUP_DP: {=bool:?}, VDM_SRC_ENABLE: {=bool:?}, CHK_CONTACT: {=bool:?}, CHK_CHRG_B: {=bool:?}, EN_B: {=bool:?}, DCDSEL: {=bool:?} }}",
            self.DETECT_SEC(),
            self.PULLUP_DP(),
            self.VDM_SRC_ENABLE(),
            self.CHK_CONTACT(),
            self.CHK_CHRG_B(),
            self.EN_B(),
            self.DCDSEL()
        )
    }
}
#[doc = "Charger Detect."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USB1_CHRG_DETECT_SET(pub u32);
impl USB1_CHRG_DETECT_SET {
    #[doc = "Secondary Detection Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn DETECT_SEC(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Secondary Detection Function Enable."]
    #[inline(always)]
    pub const fn set_DETECT_SEC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DP Pullup Resistor Enable Override Control."]
    #[must_use]
    #[inline(always)]
    pub const fn PULLUP_DP(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DP Pullup Resistor Enable Override Control."]
    #[inline(always)]
    pub const fn set_PULLUP_DP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "VDM_SRC Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn VDM_SRC_ENABLE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "VDM_SRC Function Enable."]
    #[inline(always)]
    pub const fn set_VDM_SRC_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "BC Data Contact Detect Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn CHK_CONTACT(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "BC Data Contact Detect Function Enable."]
    #[inline(always)]
    pub const fn set_CHK_CONTACT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "BC Charger Detection Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn CHK_CHRG_B(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "BC Charger Detection Function Enable."]
    #[inline(always)]
    pub const fn set_CHK_CHRG_B(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Selection of BC v1.2 Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn EN_B(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Selection of BC v1.2 Function Enable."]
    #[inline(always)]
    pub const fn set_EN_B(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DCD Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn DCDSEL(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DCD Selection."]
    #[inline(always)]
    pub const fn set_DCDSEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for USB1_CHRG_DETECT_SET {
    #[inline(always)]
    fn default() -> USB1_CHRG_DETECT_SET {
        USB1_CHRG_DETECT_SET(0)
    }
}
impl core::fmt::Debug for USB1_CHRG_DETECT_SET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB1_CHRG_DETECT_SET")
            .field("DETECT_SEC", &self.DETECT_SEC())
            .field("PULLUP_DP", &self.PULLUP_DP())
            .field("VDM_SRC_ENABLE", &self.VDM_SRC_ENABLE())
            .field("CHK_CONTACT", &self.CHK_CONTACT())
            .field("CHK_CHRG_B", &self.CHK_CHRG_B())
            .field("EN_B", &self.EN_B())
            .field("DCDSEL", &self.DCDSEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USB1_CHRG_DETECT_SET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USB1_CHRG_DETECT_SET {{ DETECT_SEC: {=bool:?}, PULLUP_DP: {=bool:?}, VDM_SRC_ENABLE: {=bool:?}, CHK_CONTACT: {=bool:?}, CHK_CHRG_B: {=bool:?}, EN_B: {=bool:?}, DCDSEL: {=bool:?} }}",
            self.DETECT_SEC(),
            self.PULLUP_DP(),
            self.VDM_SRC_ENABLE(),
            self.CHK_CONTACT(),
            self.CHK_CHRG_B(),
            self.EN_B(),
            self.DCDSEL()
        )
    }
}
#[doc = "Charger Detect."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USB1_CHRG_DETECT_TOG(pub u32);
impl USB1_CHRG_DETECT_TOG {
    #[doc = "Secondary Detection Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn DETECT_SEC(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Secondary Detection Function Enable."]
    #[inline(always)]
    pub const fn set_DETECT_SEC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DP Pullup Resistor Enable Override Control."]
    #[must_use]
    #[inline(always)]
    pub const fn PULLUP_DP(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DP Pullup Resistor Enable Override Control."]
    #[inline(always)]
    pub const fn set_PULLUP_DP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "VDM_SRC Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn VDM_SRC_ENABLE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "VDM_SRC Function Enable."]
    #[inline(always)]
    pub const fn set_VDM_SRC_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "BC Data Contact Detect Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn CHK_CONTACT(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "BC Data Contact Detect Function Enable."]
    #[inline(always)]
    pub const fn set_CHK_CONTACT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "BC Charger Detection Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn CHK_CHRG_B(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "BC Charger Detection Function Enable."]
    #[inline(always)]
    pub const fn set_CHK_CHRG_B(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Selection of BC v1.2 Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn EN_B(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Selection of BC v1.2 Function Enable."]
    #[inline(always)]
    pub const fn set_EN_B(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DCD Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn DCDSEL(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DCD Selection."]
    #[inline(always)]
    pub const fn set_DCDSEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for USB1_CHRG_DETECT_TOG {
    #[inline(always)]
    fn default() -> USB1_CHRG_DETECT_TOG {
        USB1_CHRG_DETECT_TOG(0)
    }
}
impl core::fmt::Debug for USB1_CHRG_DETECT_TOG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB1_CHRG_DETECT_TOG")
            .field("DETECT_SEC", &self.DETECT_SEC())
            .field("PULLUP_DP", &self.PULLUP_DP())
            .field("VDM_SRC_ENABLE", &self.VDM_SRC_ENABLE())
            .field("CHK_CONTACT", &self.CHK_CONTACT())
            .field("CHK_CHRG_B", &self.CHK_CHRG_B())
            .field("EN_B", &self.EN_B())
            .field("DCDSEL", &self.DCDSEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USB1_CHRG_DETECT_TOG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USB1_CHRG_DETECT_TOG {{ DETECT_SEC: {=bool:?}, PULLUP_DP: {=bool:?}, VDM_SRC_ENABLE: {=bool:?}, CHK_CONTACT: {=bool:?}, CHK_CHRG_B: {=bool:?}, EN_B: {=bool:?}, DCDSEL: {=bool:?} }}",
            self.DETECT_SEC(),
            self.PULLUP_DP(),
            self.VDM_SRC_ENABLE(),
            self.CHK_CONTACT(),
            self.CHK_CHRG_B(),
            self.EN_B(),
            self.DCDSEL()
        )
    }
}
#[doc = "Charger Detect Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USB1_CHRG_DET_STAT(pub u32);
impl USB1_CHRG_DET_STAT {
    #[doc = "Battery Charging Data Contact Detection Phase Output."]
    #[must_use]
    #[inline(always)]
    pub const fn PLUG_CONTACT(&self) -> PLUG_CONTACT {
        let val = (self.0 >> 0usize) & 0x01;
        PLUG_CONTACT::from_bits(val as u8)
    }
    #[doc = "Battery Charging Data Contact Detection Phase Output."]
    #[inline(always)]
    pub const fn set_PLUG_CONTACT(&mut self, val: PLUG_CONTACT) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Battery Charging Primary Detection Phase Output."]
    #[must_use]
    #[inline(always)]
    pub const fn CHRG_DETECTED(&self) -> CHRG_DETECTED {
        let val = (self.0 >> 1usize) & 0x01;
        CHRG_DETECTED::from_bits(val as u8)
    }
    #[doc = "Battery Charging Primary Detection Phase Output."]
    #[inline(always)]
    pub const fn set_CHRG_DETECTED(&mut self, val: CHRG_DETECTED) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "DM Voltage."]
    #[must_use]
    #[inline(always)]
    pub const fn DM_STATE(&self) -> DM_STATE {
        let val = (self.0 >> 2usize) & 0x01;
        DM_STATE::from_bits(val as u8)
    }
    #[doc = "DM Voltage."]
    #[inline(always)]
    pub const fn set_DM_STATE(&mut self, val: DM_STATE) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "DP Voltage."]
    #[must_use]
    #[inline(always)]
    pub const fn DP_STATE(&self) -> DP_STATE {
        let val = (self.0 >> 3usize) & 0x01;
        DP_STATE::from_bits(val as u8)
    }
    #[doc = "DP Voltage."]
    #[inline(always)]
    pub const fn set_DP_STATE(&mut self, val: DP_STATE) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Battery Charging Secondary Detection Phase Output."]
    #[must_use]
    #[inline(always)]
    pub const fn SECDET_DCP(&self) -> SECDET_DCP {
        let val = (self.0 >> 4usize) & 0x01;
        SECDET_DCP::from_bits(val as u8)
    }
    #[doc = "Battery Charging Secondary Detection Phase Output."]
    #[inline(always)]
    pub const fn set_SECDET_DCP(&mut self, val: SECDET_DCP) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
}
impl Default for USB1_CHRG_DET_STAT {
    #[inline(always)]
    fn default() -> USB1_CHRG_DET_STAT {
        USB1_CHRG_DET_STAT(0)
    }
}
impl core::fmt::Debug for USB1_CHRG_DET_STAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB1_CHRG_DET_STAT")
            .field("PLUG_CONTACT", &self.PLUG_CONTACT())
            .field("CHRG_DETECTED", &self.CHRG_DETECTED())
            .field("DM_STATE", &self.DM_STATE())
            .field("DP_STATE", &self.DP_STATE())
            .field("SECDET_DCP", &self.SECDET_DCP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USB1_CHRG_DET_STAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USB1_CHRG_DET_STAT {{ PLUG_CONTACT: {:?}, CHRG_DETECTED: {:?}, DM_STATE: {:?}, DP_STATE: {:?}, SECDET_DCP: {:?} }}",
            self.PLUG_CONTACT(),
            self.CHRG_DETECTED(),
            self.DM_STATE(),
            self.DP_STATE(),
            self.SECDET_DCP()
        )
    }
}
#[doc = "Charger Detect Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USB1_CHRG_DET_STAT_CLR(pub u32);
impl USB1_CHRG_DET_STAT_CLR {
    #[doc = "Battery Charging Data Contact Detection Phase Output."]
    #[must_use]
    #[inline(always)]
    pub const fn PLUG_CONTACT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Battery Charging Data Contact Detection Phase Output."]
    #[inline(always)]
    pub const fn set_PLUG_CONTACT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Battery Charging Primary Detection Phase Output."]
    #[must_use]
    #[inline(always)]
    pub const fn CHRG_DETECTED(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Battery Charging Primary Detection Phase Output."]
    #[inline(always)]
    pub const fn set_CHRG_DETECTED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DM Voltage."]
    #[must_use]
    #[inline(always)]
    pub const fn DM_STATE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DM Voltage."]
    #[inline(always)]
    pub const fn set_DM_STATE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DP Voltage."]
    #[must_use]
    #[inline(always)]
    pub const fn DP_STATE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DP Voltage."]
    #[inline(always)]
    pub const fn set_DP_STATE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Battery Charging Secondary Detection Phase Output."]
    #[must_use]
    #[inline(always)]
    pub const fn SECDET_DCP(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Battery Charging Secondary Detection Phase Output."]
    #[inline(always)]
    pub const fn set_SECDET_DCP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for USB1_CHRG_DET_STAT_CLR {
    #[inline(always)]
    fn default() -> USB1_CHRG_DET_STAT_CLR {
        USB1_CHRG_DET_STAT_CLR(0)
    }
}
impl core::fmt::Debug for USB1_CHRG_DET_STAT_CLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB1_CHRG_DET_STAT_CLR")
            .field("PLUG_CONTACT", &self.PLUG_CONTACT())
            .field("CHRG_DETECTED", &self.CHRG_DETECTED())
            .field("DM_STATE", &self.DM_STATE())
            .field("DP_STATE", &self.DP_STATE())
            .field("SECDET_DCP", &self.SECDET_DCP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USB1_CHRG_DET_STAT_CLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USB1_CHRG_DET_STAT_CLR {{ PLUG_CONTACT: {=bool:?}, CHRG_DETECTED: {=bool:?}, DM_STATE: {=bool:?}, DP_STATE: {=bool:?}, SECDET_DCP: {=bool:?} }}",
            self.PLUG_CONTACT(),
            self.CHRG_DETECTED(),
            self.DM_STATE(),
            self.DP_STATE(),
            self.SECDET_DCP()
        )
    }
}
#[doc = "Charger Detect Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USB1_CHRG_DET_STAT_SET(pub u32);
impl USB1_CHRG_DET_STAT_SET {
    #[doc = "Battery Charging Data Contact Detection Phase Output."]
    #[must_use]
    #[inline(always)]
    pub const fn PLUG_CONTACT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Battery Charging Data Contact Detection Phase Output."]
    #[inline(always)]
    pub const fn set_PLUG_CONTACT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Battery Charging Primary Detection Phase Output."]
    #[must_use]
    #[inline(always)]
    pub const fn CHRG_DETECTED(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Battery Charging Primary Detection Phase Output."]
    #[inline(always)]
    pub const fn set_CHRG_DETECTED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DM Voltage."]
    #[must_use]
    #[inline(always)]
    pub const fn DM_STATE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DM Voltage."]
    #[inline(always)]
    pub const fn set_DM_STATE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DP Voltage."]
    #[must_use]
    #[inline(always)]
    pub const fn DP_STATE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DP Voltage."]
    #[inline(always)]
    pub const fn set_DP_STATE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Battery Charging Secondary Detection Phase Output."]
    #[must_use]
    #[inline(always)]
    pub const fn SECDET_DCP(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Battery Charging Secondary Detection Phase Output."]
    #[inline(always)]
    pub const fn set_SECDET_DCP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for USB1_CHRG_DET_STAT_SET {
    #[inline(always)]
    fn default() -> USB1_CHRG_DET_STAT_SET {
        USB1_CHRG_DET_STAT_SET(0)
    }
}
impl core::fmt::Debug for USB1_CHRG_DET_STAT_SET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB1_CHRG_DET_STAT_SET")
            .field("PLUG_CONTACT", &self.PLUG_CONTACT())
            .field("CHRG_DETECTED", &self.CHRG_DETECTED())
            .field("DM_STATE", &self.DM_STATE())
            .field("DP_STATE", &self.DP_STATE())
            .field("SECDET_DCP", &self.SECDET_DCP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USB1_CHRG_DET_STAT_SET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USB1_CHRG_DET_STAT_SET {{ PLUG_CONTACT: {=bool:?}, CHRG_DETECTED: {=bool:?}, DM_STATE: {=bool:?}, DP_STATE: {=bool:?}, SECDET_DCP: {=bool:?} }}",
            self.PLUG_CONTACT(),
            self.CHRG_DETECTED(),
            self.DM_STATE(),
            self.DP_STATE(),
            self.SECDET_DCP()
        )
    }
}
#[doc = "Charger Detect Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USB1_CHRG_DET_STAT_TOG(pub u32);
impl USB1_CHRG_DET_STAT_TOG {
    #[doc = "Battery Charging Data Contact Detection Phase Output."]
    #[must_use]
    #[inline(always)]
    pub const fn PLUG_CONTACT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Battery Charging Data Contact Detection Phase Output."]
    #[inline(always)]
    pub const fn set_PLUG_CONTACT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Battery Charging Primary Detection Phase Output."]
    #[must_use]
    #[inline(always)]
    pub const fn CHRG_DETECTED(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Battery Charging Primary Detection Phase Output."]
    #[inline(always)]
    pub const fn set_CHRG_DETECTED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DM Voltage."]
    #[must_use]
    #[inline(always)]
    pub const fn DM_STATE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DM Voltage."]
    #[inline(always)]
    pub const fn set_DM_STATE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DP Voltage."]
    #[must_use]
    #[inline(always)]
    pub const fn DP_STATE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DP Voltage."]
    #[inline(always)]
    pub const fn set_DP_STATE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Battery Charging Secondary Detection Phase Output."]
    #[must_use]
    #[inline(always)]
    pub const fn SECDET_DCP(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Battery Charging Secondary Detection Phase Output."]
    #[inline(always)]
    pub const fn set_SECDET_DCP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for USB1_CHRG_DET_STAT_TOG {
    #[inline(always)]
    fn default() -> USB1_CHRG_DET_STAT_TOG {
        USB1_CHRG_DET_STAT_TOG(0)
    }
}
impl core::fmt::Debug for USB1_CHRG_DET_STAT_TOG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB1_CHRG_DET_STAT_TOG")
            .field("PLUG_CONTACT", &self.PLUG_CONTACT())
            .field("CHRG_DETECTED", &self.CHRG_DETECTED())
            .field("DM_STATE", &self.DM_STATE())
            .field("DP_STATE", &self.DP_STATE())
            .field("SECDET_DCP", &self.SECDET_DCP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USB1_CHRG_DET_STAT_TOG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USB1_CHRG_DET_STAT_TOG {{ PLUG_CONTACT: {=bool:?}, CHRG_DETECTED: {=bool:?}, DM_STATE: {=bool:?}, DP_STATE: {=bool:?}, SECDET_DCP: {=bool:?} }}",
            self.PLUG_CONTACT(),
            self.CHRG_DETECTED(),
            self.DM_STATE(),
            self.DP_STATE(),
            self.SECDET_DCP()
        )
    }
}
#[doc = "VBUS Detect."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USB1_VBUS_DETECT(pub u32);
impl USB1_VBUS_DETECT {
    #[doc = "VBUS Comparator Threshold."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUSVALID_THRESH(&self) -> VBUSVALID_THRESH {
        let val = (self.0 >> 0usize) & 0x07;
        VBUSVALID_THRESH::from_bits(val as u8)
    }
    #[doc = "VBUS Comparator Threshold."]
    #[inline(always)]
    pub const fn set_VBUSVALID_THRESH(&mut self, val: VBUSVALID_THRESH) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "VBUS Detect Signal Local Override Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUS_OVERRIDE_EN(&self) -> VBUS_OVERRIDE_EN {
        let val = (self.0 >> 3usize) & 0x01;
        VBUS_OVERRIDE_EN::from_bits(val as u8)
    }
    #[doc = "VBUS Detect Signal Local Override Enable."]
    #[inline(always)]
    pub const fn set_VBUS_OVERRIDE_EN(&mut self, val: VBUS_OVERRIDE_EN) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Override Value for SESSEND."]
    #[must_use]
    #[inline(always)]
    pub const fn SESSEND_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for SESSEND."]
    #[inline(always)]
    pub const fn set_SESSEND_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Override Value for B-Device Session Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn BVALID_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for B-Device Session Valid."]
    #[inline(always)]
    pub const fn set_BVALID_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Override Value for A-Device Session Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn AVALID_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for A-Device Session Valid."]
    #[inline(always)]
    pub const fn set_AVALID_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Override Value for the VBUS_VALID Signal."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUSVALID_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for the VBUS_VALID Signal."]
    #[inline(always)]
    pub const fn set_VBUSVALID_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "VBUS_VALID Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUSVALID_SEL(&self) -> VBUSVALID_SEL {
        let val = (self.0 >> 8usize) & 0x01;
        VBUSVALID_SEL::from_bits(val as u8)
    }
    #[doc = "VBUS_VALID Selection."]
    #[inline(always)]
    pub const fn set_VBUSVALID_SEL(&mut self, val: VBUSVALID_SEL) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "VBUS_VALID Source Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUS_SOURCE_SEL(&self) -> VBUS_SOURCE_SEL {
        let val = (self.0 >> 9usize) & 0x03;
        VBUS_SOURCE_SEL::from_bits(val as u8)
    }
    #[doc = "VBUS_VALID Source Selection."]
    #[inline(always)]
    pub const fn set_VBUS_SOURCE_SEL(&mut self, val: VBUS_SOURCE_SEL) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
    }
    #[doc = "Enable Local ID Pin Status Override."]
    #[must_use]
    #[inline(always)]
    pub const fn ID_OVERRIDE_EN(&self) -> ID_OVERRIDE_EN {
        let val = (self.0 >> 11usize) & 0x01;
        ID_OVERRIDE_EN::from_bits(val as u8)
    }
    #[doc = "Enable Local ID Pin Status Override."]
    #[inline(always)]
    pub const fn set_ID_OVERRIDE_EN(&mut self, val: ID_OVERRIDE_EN) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "ID Pin Status Local Override."]
    #[must_use]
    #[inline(always)]
    pub const fn ID_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "ID Pin Status Local Override."]
    #[inline(always)]
    pub const fn set_ID_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "External ID Override Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn EXT_ID_OVERRIDE_EN(&self) -> EXT_ID_OVERRIDE_EN {
        let val = (self.0 >> 13usize) & 0x01;
        EXT_ID_OVERRIDE_EN::from_bits(val as u8)
    }
    #[doc = "External ID Override Enable."]
    #[inline(always)]
    pub const fn set_EXT_ID_OVERRIDE_EN(&mut self, val: EXT_ID_OVERRIDE_EN) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "External VBUS Override Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn EXT_VBUS_OVERRIDE_EN(&self) -> EXT_VBUS_OVERRIDE_EN {
        let val = (self.0 >> 14usize) & 0x01;
        EXT_VBUS_OVERRIDE_EN::from_bits(val as u8)
    }
    #[doc = "External VBUS Override Enable."]
    #[inline(always)]
    pub const fn set_EXT_VBUS_OVERRIDE_EN(&mut self, val: EXT_VBUS_OVERRIDE_EN) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "VBUS_VALID Comparator Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUSVALID_TO_B(&self) -> VBUSVALID_TO_B {
        let val = (self.0 >> 18usize) & 0x01;
        VBUSVALID_TO_B::from_bits(val as u8)
    }
    #[doc = "VBUS_VALID Comparator Selection."]
    #[inline(always)]
    pub const fn set_VBUSVALID_TO_B(&mut self, val: VBUSVALID_TO_B) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "VBUS_VALID Comparator Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUSVALID_PWRUP_CMPS(&self) -> VBUSVALID_PWRUP_CMPS {
        let val = (self.0 >> 20usize) & 0x07;
        VBUSVALID_PWRUP_CMPS::from_bits(val as u8)
    }
    #[doc = "VBUS_VALID Comparator Enable."]
    #[inline(always)]
    pub const fn set_VBUSVALID_PWRUP_CMPS(&mut self, val: VBUSVALID_PWRUP_CMPS) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "VBUS Discharge Resistor."]
    #[must_use]
    #[inline(always)]
    pub const fn DISCHARGE_VBUS(&self) -> DISCHARGE_VBUS {
        let val = (self.0 >> 26usize) & 0x01;
        DISCHARGE_VBUS::from_bits(val as u8)
    }
    #[doc = "VBUS Discharge Resistor."]
    #[inline(always)]
    pub const fn set_DISCHARGE_VBUS(&mut self, val: DISCHARGE_VBUS) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
}
impl Default for USB1_VBUS_DETECT {
    #[inline(always)]
    fn default() -> USB1_VBUS_DETECT {
        USB1_VBUS_DETECT(0)
    }
}
impl core::fmt::Debug for USB1_VBUS_DETECT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB1_VBUS_DETECT")
            .field("VBUSVALID_THRESH", &self.VBUSVALID_THRESH())
            .field("VBUS_OVERRIDE_EN", &self.VBUS_OVERRIDE_EN())
            .field("SESSEND_OVERRIDE", &self.SESSEND_OVERRIDE())
            .field("BVALID_OVERRIDE", &self.BVALID_OVERRIDE())
            .field("AVALID_OVERRIDE", &self.AVALID_OVERRIDE())
            .field("VBUSVALID_OVERRIDE", &self.VBUSVALID_OVERRIDE())
            .field("VBUSVALID_SEL", &self.VBUSVALID_SEL())
            .field("VBUS_SOURCE_SEL", &self.VBUS_SOURCE_SEL())
            .field("ID_OVERRIDE_EN", &self.ID_OVERRIDE_EN())
            .field("ID_OVERRIDE", &self.ID_OVERRIDE())
            .field("EXT_ID_OVERRIDE_EN", &self.EXT_ID_OVERRIDE_EN())
            .field("EXT_VBUS_OVERRIDE_EN", &self.EXT_VBUS_OVERRIDE_EN())
            .field("VBUSVALID_TO_B", &self.VBUSVALID_TO_B())
            .field("VBUSVALID_PWRUP_CMPS", &self.VBUSVALID_PWRUP_CMPS())
            .field("DISCHARGE_VBUS", &self.DISCHARGE_VBUS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USB1_VBUS_DETECT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USB1_VBUS_DETECT {{ VBUSVALID_THRESH: {:?}, VBUS_OVERRIDE_EN: {:?}, SESSEND_OVERRIDE: {=bool:?}, BVALID_OVERRIDE: {=bool:?}, AVALID_OVERRIDE: {=bool:?}, VBUSVALID_OVERRIDE: {=bool:?}, VBUSVALID_SEL: {:?}, VBUS_SOURCE_SEL: {:?}, ID_OVERRIDE_EN: {:?}, ID_OVERRIDE: {=bool:?}, EXT_ID_OVERRIDE_EN: {:?}, EXT_VBUS_OVERRIDE_EN: {:?}, VBUSVALID_TO_B: {:?}, VBUSVALID_PWRUP_CMPS: {:?}, DISCHARGE_VBUS: {:?} }}",
            self.VBUSVALID_THRESH(),
            self.VBUS_OVERRIDE_EN(),
            self.SESSEND_OVERRIDE(),
            self.BVALID_OVERRIDE(),
            self.AVALID_OVERRIDE(),
            self.VBUSVALID_OVERRIDE(),
            self.VBUSVALID_SEL(),
            self.VBUS_SOURCE_SEL(),
            self.ID_OVERRIDE_EN(),
            self.ID_OVERRIDE(),
            self.EXT_ID_OVERRIDE_EN(),
            self.EXT_VBUS_OVERRIDE_EN(),
            self.VBUSVALID_TO_B(),
            self.VBUSVALID_PWRUP_CMPS(),
            self.DISCHARGE_VBUS()
        )
    }
}
#[doc = "VBUS Detect."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USB1_VBUS_DETECT_CLR(pub u32);
impl USB1_VBUS_DETECT_CLR {
    #[doc = "VBUS Comparator Threshold."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUSVALID_THRESH(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "VBUS Comparator Threshold."]
    #[inline(always)]
    pub const fn set_VBUSVALID_THRESH(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "VBUS Detect Signal Local Override Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUS_OVERRIDE_EN(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Detect Signal Local Override Enable."]
    #[inline(always)]
    pub const fn set_VBUS_OVERRIDE_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Override Value for SESSEND."]
    #[must_use]
    #[inline(always)]
    pub const fn SESSEND_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for SESSEND."]
    #[inline(always)]
    pub const fn set_SESSEND_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Override Value for B-Device Session Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn BVALID_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for B-Device Session Valid."]
    #[inline(always)]
    pub const fn set_BVALID_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Override Value for A-Device Session Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn AVALID_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for A-Device Session Valid."]
    #[inline(always)]
    pub const fn set_AVALID_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Override Value for the VBUS_VALID Signal."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUSVALID_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for the VBUS_VALID Signal."]
    #[inline(always)]
    pub const fn set_VBUSVALID_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "VBUS_VALID Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUSVALID_SEL(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS_VALID Selection."]
    #[inline(always)]
    pub const fn set_VBUSVALID_SEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "VBUS_VALID Source Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUS_SOURCE_SEL(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "VBUS_VALID Source Selection."]
    #[inline(always)]
    pub const fn set_VBUS_SOURCE_SEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[doc = "Enable Local ID Pin Status Override."]
    #[must_use]
    #[inline(always)]
    pub const fn ID_OVERRIDE_EN(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Local ID Pin Status Override."]
    #[inline(always)]
    pub const fn set_ID_OVERRIDE_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "ID Pin Status Local Override."]
    #[must_use]
    #[inline(always)]
    pub const fn ID_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "ID Pin Status Local Override."]
    #[inline(always)]
    pub const fn set_ID_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "External ID Override Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn EXT_ID_OVERRIDE_EN(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "External ID Override Enable."]
    #[inline(always)]
    pub const fn set_EXT_ID_OVERRIDE_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "External VBUS Override Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn EXT_VBUS_OVERRIDE_EN(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "External VBUS Override Enable."]
    #[inline(always)]
    pub const fn set_EXT_VBUS_OVERRIDE_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "VBUS_VALID Comparator Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUSVALID_TO_B(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS_VALID Comparator Selection."]
    #[inline(always)]
    pub const fn set_VBUSVALID_TO_B(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "VBUS_VALID Comparator Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUSVALID_PWRUP_CMPS(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "VBUS_VALID Comparator Enable."]
    #[inline(always)]
    pub const fn set_VBUSVALID_PWRUP_CMPS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[doc = "VBUS Discharge Resistor."]
    #[must_use]
    #[inline(always)]
    pub const fn DISCHARGE_VBUS(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Discharge Resistor."]
    #[inline(always)]
    pub const fn set_DISCHARGE_VBUS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for USB1_VBUS_DETECT_CLR {
    #[inline(always)]
    fn default() -> USB1_VBUS_DETECT_CLR {
        USB1_VBUS_DETECT_CLR(0)
    }
}
impl core::fmt::Debug for USB1_VBUS_DETECT_CLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB1_VBUS_DETECT_CLR")
            .field("VBUSVALID_THRESH", &self.VBUSVALID_THRESH())
            .field("VBUS_OVERRIDE_EN", &self.VBUS_OVERRIDE_EN())
            .field("SESSEND_OVERRIDE", &self.SESSEND_OVERRIDE())
            .field("BVALID_OVERRIDE", &self.BVALID_OVERRIDE())
            .field("AVALID_OVERRIDE", &self.AVALID_OVERRIDE())
            .field("VBUSVALID_OVERRIDE", &self.VBUSVALID_OVERRIDE())
            .field("VBUSVALID_SEL", &self.VBUSVALID_SEL())
            .field("VBUS_SOURCE_SEL", &self.VBUS_SOURCE_SEL())
            .field("ID_OVERRIDE_EN", &self.ID_OVERRIDE_EN())
            .field("ID_OVERRIDE", &self.ID_OVERRIDE())
            .field("EXT_ID_OVERRIDE_EN", &self.EXT_ID_OVERRIDE_EN())
            .field("EXT_VBUS_OVERRIDE_EN", &self.EXT_VBUS_OVERRIDE_EN())
            .field("VBUSVALID_TO_B", &self.VBUSVALID_TO_B())
            .field("VBUSVALID_PWRUP_CMPS", &self.VBUSVALID_PWRUP_CMPS())
            .field("DISCHARGE_VBUS", &self.DISCHARGE_VBUS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USB1_VBUS_DETECT_CLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USB1_VBUS_DETECT_CLR {{ VBUSVALID_THRESH: {=u8:?}, VBUS_OVERRIDE_EN: {=bool:?}, SESSEND_OVERRIDE: {=bool:?}, BVALID_OVERRIDE: {=bool:?}, AVALID_OVERRIDE: {=bool:?}, VBUSVALID_OVERRIDE: {=bool:?}, VBUSVALID_SEL: {=bool:?}, VBUS_SOURCE_SEL: {=u8:?}, ID_OVERRIDE_EN: {=bool:?}, ID_OVERRIDE: {=bool:?}, EXT_ID_OVERRIDE_EN: {=bool:?}, EXT_VBUS_OVERRIDE_EN: {=bool:?}, VBUSVALID_TO_B: {=bool:?}, VBUSVALID_PWRUP_CMPS: {=u8:?}, DISCHARGE_VBUS: {=bool:?} }}",
            self.VBUSVALID_THRESH(),
            self.VBUS_OVERRIDE_EN(),
            self.SESSEND_OVERRIDE(),
            self.BVALID_OVERRIDE(),
            self.AVALID_OVERRIDE(),
            self.VBUSVALID_OVERRIDE(),
            self.VBUSVALID_SEL(),
            self.VBUS_SOURCE_SEL(),
            self.ID_OVERRIDE_EN(),
            self.ID_OVERRIDE(),
            self.EXT_ID_OVERRIDE_EN(),
            self.EXT_VBUS_OVERRIDE_EN(),
            self.VBUSVALID_TO_B(),
            self.VBUSVALID_PWRUP_CMPS(),
            self.DISCHARGE_VBUS()
        )
    }
}
#[doc = "VBUS Detect."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USB1_VBUS_DETECT_SET(pub u32);
impl USB1_VBUS_DETECT_SET {
    #[doc = "VBUS Comparator Threshold."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUSVALID_THRESH(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "VBUS Comparator Threshold."]
    #[inline(always)]
    pub const fn set_VBUSVALID_THRESH(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "VBUS Detect Signal Local Override Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUS_OVERRIDE_EN(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Detect Signal Local Override Enable."]
    #[inline(always)]
    pub const fn set_VBUS_OVERRIDE_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Override Value for SESSEND."]
    #[must_use]
    #[inline(always)]
    pub const fn SESSEND_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for SESSEND."]
    #[inline(always)]
    pub const fn set_SESSEND_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Override Value for B-Device Session Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn BVALID_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for B-Device Session Valid."]
    #[inline(always)]
    pub const fn set_BVALID_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Override Value for A-Device Session Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn AVALID_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for A-Device Session Valid."]
    #[inline(always)]
    pub const fn set_AVALID_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Override Value for the VBUS_VALID Signal."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUSVALID_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for the VBUS_VALID Signal."]
    #[inline(always)]
    pub const fn set_VBUSVALID_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "VBUS_VALID Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUSVALID_SEL(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS_VALID Selection."]
    #[inline(always)]
    pub const fn set_VBUSVALID_SEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "VBUS_VALID Source Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUS_SOURCE_SEL(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "VBUS_VALID Source Selection."]
    #[inline(always)]
    pub const fn set_VBUS_SOURCE_SEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[doc = "Enable Local ID Pin Status Override."]
    #[must_use]
    #[inline(always)]
    pub const fn ID_OVERRIDE_EN(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Local ID Pin Status Override."]
    #[inline(always)]
    pub const fn set_ID_OVERRIDE_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "ID Pin Status Local Override."]
    #[must_use]
    #[inline(always)]
    pub const fn ID_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "ID Pin Status Local Override."]
    #[inline(always)]
    pub const fn set_ID_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "External ID Override Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn EXT_ID_OVERRIDE_EN(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "External ID Override Enable."]
    #[inline(always)]
    pub const fn set_EXT_ID_OVERRIDE_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "External VBUS Override Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn EXT_VBUS_OVERRIDE_EN(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "External VBUS Override Enable."]
    #[inline(always)]
    pub const fn set_EXT_VBUS_OVERRIDE_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "VBUS_VALID Comparator Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUSVALID_TO_B(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS_VALID Comparator Selection."]
    #[inline(always)]
    pub const fn set_VBUSVALID_TO_B(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "VBUS_VALID Comparator Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUSVALID_PWRUP_CMPS(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "VBUS_VALID Comparator Enable."]
    #[inline(always)]
    pub const fn set_VBUSVALID_PWRUP_CMPS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[doc = "VBUS Discharge Resistor."]
    #[must_use]
    #[inline(always)]
    pub const fn DISCHARGE_VBUS(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Discharge Resistor."]
    #[inline(always)]
    pub const fn set_DISCHARGE_VBUS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for USB1_VBUS_DETECT_SET {
    #[inline(always)]
    fn default() -> USB1_VBUS_DETECT_SET {
        USB1_VBUS_DETECT_SET(0)
    }
}
impl core::fmt::Debug for USB1_VBUS_DETECT_SET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB1_VBUS_DETECT_SET")
            .field("VBUSVALID_THRESH", &self.VBUSVALID_THRESH())
            .field("VBUS_OVERRIDE_EN", &self.VBUS_OVERRIDE_EN())
            .field("SESSEND_OVERRIDE", &self.SESSEND_OVERRIDE())
            .field("BVALID_OVERRIDE", &self.BVALID_OVERRIDE())
            .field("AVALID_OVERRIDE", &self.AVALID_OVERRIDE())
            .field("VBUSVALID_OVERRIDE", &self.VBUSVALID_OVERRIDE())
            .field("VBUSVALID_SEL", &self.VBUSVALID_SEL())
            .field("VBUS_SOURCE_SEL", &self.VBUS_SOURCE_SEL())
            .field("ID_OVERRIDE_EN", &self.ID_OVERRIDE_EN())
            .field("ID_OVERRIDE", &self.ID_OVERRIDE())
            .field("EXT_ID_OVERRIDE_EN", &self.EXT_ID_OVERRIDE_EN())
            .field("EXT_VBUS_OVERRIDE_EN", &self.EXT_VBUS_OVERRIDE_EN())
            .field("VBUSVALID_TO_B", &self.VBUSVALID_TO_B())
            .field("VBUSVALID_PWRUP_CMPS", &self.VBUSVALID_PWRUP_CMPS())
            .field("DISCHARGE_VBUS", &self.DISCHARGE_VBUS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USB1_VBUS_DETECT_SET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USB1_VBUS_DETECT_SET {{ VBUSVALID_THRESH: {=u8:?}, VBUS_OVERRIDE_EN: {=bool:?}, SESSEND_OVERRIDE: {=bool:?}, BVALID_OVERRIDE: {=bool:?}, AVALID_OVERRIDE: {=bool:?}, VBUSVALID_OVERRIDE: {=bool:?}, VBUSVALID_SEL: {=bool:?}, VBUS_SOURCE_SEL: {=u8:?}, ID_OVERRIDE_EN: {=bool:?}, ID_OVERRIDE: {=bool:?}, EXT_ID_OVERRIDE_EN: {=bool:?}, EXT_VBUS_OVERRIDE_EN: {=bool:?}, VBUSVALID_TO_B: {=bool:?}, VBUSVALID_PWRUP_CMPS: {=u8:?}, DISCHARGE_VBUS: {=bool:?} }}",
            self.VBUSVALID_THRESH(),
            self.VBUS_OVERRIDE_EN(),
            self.SESSEND_OVERRIDE(),
            self.BVALID_OVERRIDE(),
            self.AVALID_OVERRIDE(),
            self.VBUSVALID_OVERRIDE(),
            self.VBUSVALID_SEL(),
            self.VBUS_SOURCE_SEL(),
            self.ID_OVERRIDE_EN(),
            self.ID_OVERRIDE(),
            self.EXT_ID_OVERRIDE_EN(),
            self.EXT_VBUS_OVERRIDE_EN(),
            self.VBUSVALID_TO_B(),
            self.VBUSVALID_PWRUP_CMPS(),
            self.DISCHARGE_VBUS()
        )
    }
}
#[doc = "VBUS Detect."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USB1_VBUS_DETECT_TOG(pub u32);
impl USB1_VBUS_DETECT_TOG {
    #[doc = "VBUS Comparator Threshold."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUSVALID_THRESH(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "VBUS Comparator Threshold."]
    #[inline(always)]
    pub const fn set_VBUSVALID_THRESH(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "VBUS Detect Signal Local Override Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUS_OVERRIDE_EN(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Detect Signal Local Override Enable."]
    #[inline(always)]
    pub const fn set_VBUS_OVERRIDE_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Override Value for SESSEND."]
    #[must_use]
    #[inline(always)]
    pub const fn SESSEND_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for SESSEND."]
    #[inline(always)]
    pub const fn set_SESSEND_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Override Value for B-Device Session Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn BVALID_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for B-Device Session Valid."]
    #[inline(always)]
    pub const fn set_BVALID_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Override Value for A-Device Session Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn AVALID_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for A-Device Session Valid."]
    #[inline(always)]
    pub const fn set_AVALID_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Override Value for the VBUS_VALID Signal."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUSVALID_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for the VBUS_VALID Signal."]
    #[inline(always)]
    pub const fn set_VBUSVALID_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "VBUS_VALID Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUSVALID_SEL(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS_VALID Selection."]
    #[inline(always)]
    pub const fn set_VBUSVALID_SEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "VBUS_VALID Source Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUS_SOURCE_SEL(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "VBUS_VALID Source Selection."]
    #[inline(always)]
    pub const fn set_VBUS_SOURCE_SEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[doc = "Enable Local ID Pin Status Override."]
    #[must_use]
    #[inline(always)]
    pub const fn ID_OVERRIDE_EN(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Local ID Pin Status Override."]
    #[inline(always)]
    pub const fn set_ID_OVERRIDE_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "ID Pin Status Local Override."]
    #[must_use]
    #[inline(always)]
    pub const fn ID_OVERRIDE(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "ID Pin Status Local Override."]
    #[inline(always)]
    pub const fn set_ID_OVERRIDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "External ID Override Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn EXT_ID_OVERRIDE_EN(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "External ID Override Enable."]
    #[inline(always)]
    pub const fn set_EXT_ID_OVERRIDE_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "External VBUS Override Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn EXT_VBUS_OVERRIDE_EN(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "External VBUS Override Enable."]
    #[inline(always)]
    pub const fn set_EXT_VBUS_OVERRIDE_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "VBUS_VALID Comparator Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUSVALID_TO_B(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS_VALID Comparator Selection."]
    #[inline(always)]
    pub const fn set_VBUSVALID_TO_B(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "VBUS_VALID Comparator Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUSVALID_PWRUP_CMPS(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "VBUS_VALID Comparator Enable."]
    #[inline(always)]
    pub const fn set_VBUSVALID_PWRUP_CMPS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[doc = "VBUS Discharge Resistor."]
    #[must_use]
    #[inline(always)]
    pub const fn DISCHARGE_VBUS(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Discharge Resistor."]
    #[inline(always)]
    pub const fn set_DISCHARGE_VBUS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for USB1_VBUS_DETECT_TOG {
    #[inline(always)]
    fn default() -> USB1_VBUS_DETECT_TOG {
        USB1_VBUS_DETECT_TOG(0)
    }
}
impl core::fmt::Debug for USB1_VBUS_DETECT_TOG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB1_VBUS_DETECT_TOG")
            .field("VBUSVALID_THRESH", &self.VBUSVALID_THRESH())
            .field("VBUS_OVERRIDE_EN", &self.VBUS_OVERRIDE_EN())
            .field("SESSEND_OVERRIDE", &self.SESSEND_OVERRIDE())
            .field("BVALID_OVERRIDE", &self.BVALID_OVERRIDE())
            .field("AVALID_OVERRIDE", &self.AVALID_OVERRIDE())
            .field("VBUSVALID_OVERRIDE", &self.VBUSVALID_OVERRIDE())
            .field("VBUSVALID_SEL", &self.VBUSVALID_SEL())
            .field("VBUS_SOURCE_SEL", &self.VBUS_SOURCE_SEL())
            .field("ID_OVERRIDE_EN", &self.ID_OVERRIDE_EN())
            .field("ID_OVERRIDE", &self.ID_OVERRIDE())
            .field("EXT_ID_OVERRIDE_EN", &self.EXT_ID_OVERRIDE_EN())
            .field("EXT_VBUS_OVERRIDE_EN", &self.EXT_VBUS_OVERRIDE_EN())
            .field("VBUSVALID_TO_B", &self.VBUSVALID_TO_B())
            .field("VBUSVALID_PWRUP_CMPS", &self.VBUSVALID_PWRUP_CMPS())
            .field("DISCHARGE_VBUS", &self.DISCHARGE_VBUS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USB1_VBUS_DETECT_TOG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USB1_VBUS_DETECT_TOG {{ VBUSVALID_THRESH: {=u8:?}, VBUS_OVERRIDE_EN: {=bool:?}, SESSEND_OVERRIDE: {=bool:?}, BVALID_OVERRIDE: {=bool:?}, AVALID_OVERRIDE: {=bool:?}, VBUSVALID_OVERRIDE: {=bool:?}, VBUSVALID_SEL: {=bool:?}, VBUS_SOURCE_SEL: {=u8:?}, ID_OVERRIDE_EN: {=bool:?}, ID_OVERRIDE: {=bool:?}, EXT_ID_OVERRIDE_EN: {=bool:?}, EXT_VBUS_OVERRIDE_EN: {=bool:?}, VBUSVALID_TO_B: {=bool:?}, VBUSVALID_PWRUP_CMPS: {=u8:?}, DISCHARGE_VBUS: {=bool:?} }}",
            self.VBUSVALID_THRESH(),
            self.VBUS_OVERRIDE_EN(),
            self.SESSEND_OVERRIDE(),
            self.BVALID_OVERRIDE(),
            self.AVALID_OVERRIDE(),
            self.VBUSVALID_OVERRIDE(),
            self.VBUSVALID_SEL(),
            self.VBUS_SOURCE_SEL(),
            self.ID_OVERRIDE_EN(),
            self.ID_OVERRIDE(),
            self.EXT_ID_OVERRIDE_EN(),
            self.EXT_VBUS_OVERRIDE_EN(),
            self.VBUSVALID_TO_B(),
            self.VBUSVALID_PWRUP_CMPS(),
            self.DISCHARGE_VBUS()
        )
    }
}
#[doc = "VBUS Detect Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USB1_VBUS_DET_STAT(pub u32);
impl USB1_VBUS_DET_STAT {
    #[doc = "Session End Indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn SESSEND(&self) -> SESSEND {
        let val = (self.0 >> 0usize) & 0x01;
        SESSEND::from_bits(val as u8)
    }
    #[doc = "Session End Indicator."]
    #[inline(always)]
    pub const fn set_SESSEND(&mut self, val: SESSEND) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "B-Device Session Valid Status."]
    #[must_use]
    #[inline(always)]
    pub const fn BVALID(&self) -> BVALID {
        let val = (self.0 >> 1usize) & 0x01;
        BVALID::from_bits(val as u8)
    }
    #[doc = "B-Device Session Valid Status."]
    #[inline(always)]
    pub const fn set_BVALID(&mut self, val: BVALID) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "A-Device Session Valid Status."]
    #[must_use]
    #[inline(always)]
    pub const fn AVALID(&self) -> AVALID {
        let val = (self.0 >> 2usize) & 0x01;
        AVALID::from_bits(val as u8)
    }
    #[doc = "A-Device Session Valid Status."]
    #[inline(always)]
    pub const fn set_AVALID(&mut self, val: AVALID) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "VBUS Voltage Status."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUS_VALID(&self) -> VBUS_VALID {
        let val = (self.0 >> 3usize) & 0x01;
        VBUS_VALID::from_bits(val as u8)
    }
    #[doc = "VBUS Voltage Status."]
    #[inline(always)]
    pub const fn set_VBUS_VALID(&mut self, val: VBUS_VALID) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "VBUS_VALID_3V Detector Status."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUS_VALID_3V(&self) -> VBUS_VALID_3V {
        let val = (self.0 >> 4usize) & 0x01;
        VBUS_VALID_3V::from_bits(val as u8)
    }
    #[doc = "VBUS_VALID_3V Detector Status."]
    #[inline(always)]
    pub const fn set_VBUS_VALID_3V(&mut self, val: VBUS_VALID_3V) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "OTG ID External Override Status."]
    #[must_use]
    #[inline(always)]
    pub const fn EXT_ID(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID External Override Status."]
    #[inline(always)]
    pub const fn set_EXT_ID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for USB1_VBUS_DET_STAT {
    #[inline(always)]
    fn default() -> USB1_VBUS_DET_STAT {
        USB1_VBUS_DET_STAT(0)
    }
}
impl core::fmt::Debug for USB1_VBUS_DET_STAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB1_VBUS_DET_STAT")
            .field("SESSEND", &self.SESSEND())
            .field("BVALID", &self.BVALID())
            .field("AVALID", &self.AVALID())
            .field("VBUS_VALID", &self.VBUS_VALID())
            .field("VBUS_VALID_3V", &self.VBUS_VALID_3V())
            .field("EXT_ID", &self.EXT_ID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USB1_VBUS_DET_STAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USB1_VBUS_DET_STAT {{ SESSEND: {:?}, BVALID: {:?}, AVALID: {:?}, VBUS_VALID: {:?}, VBUS_VALID_3V: {:?}, EXT_ID: {=bool:?} }}",
            self.SESSEND(),
            self.BVALID(),
            self.AVALID(),
            self.VBUS_VALID(),
            self.VBUS_VALID_3V(),
            self.EXT_ID()
        )
    }
}
#[doc = "VBUS Detect Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USB1_VBUS_DET_STAT_CLR(pub u32);
impl USB1_VBUS_DET_STAT_CLR {
    #[doc = "Session End Indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn SESSEND(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Session End Indicator."]
    #[inline(always)]
    pub const fn set_SESSEND(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "B-Device Session Valid Status."]
    #[must_use]
    #[inline(always)]
    pub const fn BVALID(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "B-Device Session Valid Status."]
    #[inline(always)]
    pub const fn set_BVALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "A-Device Session Valid Status."]
    #[must_use]
    #[inline(always)]
    pub const fn AVALID(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "A-Device Session Valid Status."]
    #[inline(always)]
    pub const fn set_AVALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "VBUS Voltage Status."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUS_VALID(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Voltage Status."]
    #[inline(always)]
    pub const fn set_VBUS_VALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "VBUS_VALID_3V Detector Status."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUS_VALID_3V(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS_VALID_3V Detector Status."]
    #[inline(always)]
    pub const fn set_VBUS_VALID_3V(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "OTG ID External Override Status."]
    #[must_use]
    #[inline(always)]
    pub const fn EXT_ID(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID External Override Status."]
    #[inline(always)]
    pub const fn set_EXT_ID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for USB1_VBUS_DET_STAT_CLR {
    #[inline(always)]
    fn default() -> USB1_VBUS_DET_STAT_CLR {
        USB1_VBUS_DET_STAT_CLR(0)
    }
}
impl core::fmt::Debug for USB1_VBUS_DET_STAT_CLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB1_VBUS_DET_STAT_CLR")
            .field("SESSEND", &self.SESSEND())
            .field("BVALID", &self.BVALID())
            .field("AVALID", &self.AVALID())
            .field("VBUS_VALID", &self.VBUS_VALID())
            .field("VBUS_VALID_3V", &self.VBUS_VALID_3V())
            .field("EXT_ID", &self.EXT_ID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USB1_VBUS_DET_STAT_CLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USB1_VBUS_DET_STAT_CLR {{ SESSEND: {=bool:?}, BVALID: {=bool:?}, AVALID: {=bool:?}, VBUS_VALID: {=bool:?}, VBUS_VALID_3V: {=bool:?}, EXT_ID: {=bool:?} }}",
            self.SESSEND(),
            self.BVALID(),
            self.AVALID(),
            self.VBUS_VALID(),
            self.VBUS_VALID_3V(),
            self.EXT_ID()
        )
    }
}
#[doc = "VBUS Detect Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USB1_VBUS_DET_STAT_SET(pub u32);
impl USB1_VBUS_DET_STAT_SET {
    #[doc = "Session End Indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn SESSEND(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Session End Indicator."]
    #[inline(always)]
    pub const fn set_SESSEND(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "B-Device Session Valid Status."]
    #[must_use]
    #[inline(always)]
    pub const fn BVALID(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "B-Device Session Valid Status."]
    #[inline(always)]
    pub const fn set_BVALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "A-Device Session Valid Status."]
    #[must_use]
    #[inline(always)]
    pub const fn AVALID(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "A-Device Session Valid Status."]
    #[inline(always)]
    pub const fn set_AVALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "VBUS Voltage Status."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUS_VALID(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Voltage Status."]
    #[inline(always)]
    pub const fn set_VBUS_VALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "VBUS_VALID_3V Detector Status."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUS_VALID_3V(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS_VALID_3V Detector Status."]
    #[inline(always)]
    pub const fn set_VBUS_VALID_3V(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "OTG ID External Override Status."]
    #[must_use]
    #[inline(always)]
    pub const fn EXT_ID(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID External Override Status."]
    #[inline(always)]
    pub const fn set_EXT_ID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for USB1_VBUS_DET_STAT_SET {
    #[inline(always)]
    fn default() -> USB1_VBUS_DET_STAT_SET {
        USB1_VBUS_DET_STAT_SET(0)
    }
}
impl core::fmt::Debug for USB1_VBUS_DET_STAT_SET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB1_VBUS_DET_STAT_SET")
            .field("SESSEND", &self.SESSEND())
            .field("BVALID", &self.BVALID())
            .field("AVALID", &self.AVALID())
            .field("VBUS_VALID", &self.VBUS_VALID())
            .field("VBUS_VALID_3V", &self.VBUS_VALID_3V())
            .field("EXT_ID", &self.EXT_ID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USB1_VBUS_DET_STAT_SET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USB1_VBUS_DET_STAT_SET {{ SESSEND: {=bool:?}, BVALID: {=bool:?}, AVALID: {=bool:?}, VBUS_VALID: {=bool:?}, VBUS_VALID_3V: {=bool:?}, EXT_ID: {=bool:?} }}",
            self.SESSEND(),
            self.BVALID(),
            self.AVALID(),
            self.VBUS_VALID(),
            self.VBUS_VALID_3V(),
            self.EXT_ID()
        )
    }
}
#[doc = "VBUS Detect Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USB1_VBUS_DET_STAT_TOG(pub u32);
impl USB1_VBUS_DET_STAT_TOG {
    #[doc = "Session End Indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn SESSEND(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Session End Indicator."]
    #[inline(always)]
    pub const fn set_SESSEND(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "B-Device Session Valid Status."]
    #[must_use]
    #[inline(always)]
    pub const fn BVALID(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "B-Device Session Valid Status."]
    #[inline(always)]
    pub const fn set_BVALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "A-Device Session Valid Status."]
    #[must_use]
    #[inline(always)]
    pub const fn AVALID(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "A-Device Session Valid Status."]
    #[inline(always)]
    pub const fn set_AVALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "VBUS Voltage Status."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUS_VALID(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Voltage Status."]
    #[inline(always)]
    pub const fn set_VBUS_VALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "VBUS_VALID_3V Detector Status."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUS_VALID_3V(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS_VALID_3V Detector Status."]
    #[inline(always)]
    pub const fn set_VBUS_VALID_3V(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "OTG ID External Override Status."]
    #[must_use]
    #[inline(always)]
    pub const fn EXT_ID(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID External Override Status."]
    #[inline(always)]
    pub const fn set_EXT_ID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for USB1_VBUS_DET_STAT_TOG {
    #[inline(always)]
    fn default() -> USB1_VBUS_DET_STAT_TOG {
        USB1_VBUS_DET_STAT_TOG(0)
    }
}
impl core::fmt::Debug for USB1_VBUS_DET_STAT_TOG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB1_VBUS_DET_STAT_TOG")
            .field("SESSEND", &self.SESSEND())
            .field("BVALID", &self.BVALID())
            .field("AVALID", &self.AVALID())
            .field("VBUS_VALID", &self.VBUS_VALID())
            .field("VBUS_VALID_3V", &self.VBUS_VALID_3V())
            .field("EXT_ID", &self.EXT_ID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USB1_VBUS_DET_STAT_TOG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USB1_VBUS_DET_STAT_TOG {{ SESSEND: {=bool:?}, BVALID: {=bool:?}, AVALID: {=bool:?}, VBUS_VALID: {=bool:?}, VBUS_VALID_3V: {=bool:?}, EXT_ID: {=bool:?} }}",
            self.SESSEND(),
            self.BVALID(),
            self.AVALID(),
            self.VBUS_VALID(),
            self.VBUS_VALID_3V(),
            self.EXT_ID()
        )
    }
}
#[doc = "Version."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VERSION(pub u32);
impl VERSION {
    #[doc = "Step."]
    #[must_use]
    #[inline(always)]
    pub const fn STEP(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Step."]
    #[inline(always)]
    pub const fn set_STEP(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Minor."]
    #[must_use]
    #[inline(always)]
    pub const fn MINOR(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Minor."]
    #[inline(always)]
    pub const fn set_MINOR(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Major."]
    #[must_use]
    #[inline(always)]
    pub const fn MAJOR(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Major."]
    #[inline(always)]
    pub const fn set_MAJOR(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for VERSION {
    #[inline(always)]
    fn default() -> VERSION {
        VERSION(0)
    }
}
impl core::fmt::Debug for VERSION {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VERSION")
            .field("STEP", &self.STEP())
            .field("MINOR", &self.MINOR())
            .field("MAJOR", &self.MAJOR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VERSION {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VERSION {{ STEP: {=u16:?}, MINOR: {=u8:?}, MAJOR: {=u8:?} }}",
            self.STEP(),
            self.MINOR(),
            self.MAJOR()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AUTORESUME_EN {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl AUTORESUME_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AUTORESUME_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AUTORESUME_EN {
    #[inline(always)]
    fn from(val: u8) -> AUTORESUME_EN {
        AUTORESUME_EN::from_bits(val)
    }
}
impl From<AUTORESUME_EN> for u8 {
    #[inline(always)]
    fn from(val: AUTORESUME_EN) -> u8 {
        AUTORESUME_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AVALID {
    #[doc = "Below threshold."]
    AVALID_LO = 0x0,
    #[doc = "Above threshold."]
    AVALID_HI = 0x01,
}
impl AVALID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AVALID {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AVALID {
    #[inline(always)]
    fn from(val: u8) -> AVALID {
        AVALID::from_bits(val)
    }
}
impl From<AVALID> for u8 {
    #[inline(always)]
    fn from(val: AVALID) -> u8 {
        AVALID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BVALID {
    #[doc = "Below threshold."]
    BVALID_LO = 0x0,
    #[doc = "Above threshold."]
    BVALID_HI = 0x01,
}
impl BVALID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BVALID {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BVALID {
    #[inline(always)]
    fn from(val: u8) -> BVALID {
        BVALID::from_bits(val)
    }
}
impl From<BVALID> for u8 {
    #[inline(always)]
    fn from(val: BVALID) -> u8 {
        BVALID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CHK_CHRG_B {
    #[doc = "Enable."]
    BC_CHRGDET_ENABLE = 0x0,
    #[doc = "Disable."]
    BC_CHRGDET_DISABLE = 0x01,
}
impl CHK_CHRG_B {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CHK_CHRG_B {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CHK_CHRG_B {
    #[inline(always)]
    fn from(val: u8) -> CHK_CHRG_B {
        CHK_CHRG_B::from_bits(val)
    }
}
impl From<CHK_CHRG_B> for u8 {
    #[inline(always)]
    fn from(val: CHK_CHRG_B) -> u8 {
        CHK_CHRG_B::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CHK_CONTACT {
    #[doc = "Disable."]
    BC_DCD_DISABLE = 0x0,
    #[doc = "Enable."]
    BC_DCD_ENABLE = 0x01,
}
impl CHK_CONTACT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CHK_CONTACT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CHK_CONTACT {
    #[inline(always)]
    fn from(val: u8) -> CHK_CONTACT {
        CHK_CONTACT::from_bits(val)
    }
}
impl From<CHK_CONTACT> for u8 {
    #[inline(always)]
    fn from(val: CHK_CONTACT) -> u8 {
        CHK_CONTACT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CHRG_DETECTED {
    #[doc = "SDP detected."]
    SDP_DETECT = 0x0,
    #[doc = "Charging port detected."]
    CHRG_PORT_DETECT = 0x01,
}
impl CHRG_DETECTED {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CHRG_DETECTED {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CHRG_DETECTED {
    #[inline(always)]
    fn from(val: u8) -> CHRG_DETECTED {
        CHRG_DETECTED::from_bits(val)
    }
}
impl From<CHRG_DETECTED> for u8 {
    #[inline(always)]
    fn from(val: CHRG_DETECTED) -> u8 {
        CHRG_DETECTED::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CLKGATE {
    #[doc = "Run clocks."]
    RUN_CLOCKS = 0x0,
    #[doc = "Gate clocks."]
    GATE_CLOCKS = 0x01,
}
impl CLKGATE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CLKGATE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CLKGATE {
    #[inline(always)]
    fn from(val: u8) -> CLKGATE {
        CLKGATE::from_bits(val)
    }
}
impl From<CLKGATE> for u8 {
    #[inline(always)]
    fn from(val: CLKGATE) -> u8 {
        CLKGATE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DCDSEL {
    #[doc = "Fields in USB1_CHRG_DETECT."]
    CHRGDET_CTRL = 0x0,
    #[doc = "Fields and state machines in the USBHSDCD module."]
    USBHSDCD_CTRL = 0x01,
}
impl DCDSEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DCDSEL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DCDSEL {
    #[inline(always)]
    fn from(val: u8) -> DCDSEL {
        DCDSEL::from_bits(val)
    }
}
impl From<DCDSEL> for u8 {
    #[inline(always)]
    fn from(val: DCDSEL) -> u8 {
        DCDSEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DETECT_SEC {
    #[doc = "Disable."]
    BC_SECDET_DISABLE = 0x0,
    #[doc = "Enable."]
    BC_SECDET_ENABLE = 0x01,
}
impl DETECT_SEC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DETECT_SEC {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DETECT_SEC {
    #[inline(always)]
    fn from(val: u8) -> DETECT_SEC {
        DETECT_SEC::from_bits(val)
    }
}
impl From<DETECT_SEC> for u8 {
    #[inline(always)]
    fn from(val: DETECT_SEC) -> u8 {
        DETECT_SEC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DEVPLUGIN_POLARITY {
    #[doc = "Plugged in."]
    PLUGGED_IN = 0x0,
    #[doc = "Unplugged."]
    UNPLUGGED = 0x01,
}
impl DEVPLUGIN_POLARITY {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DEVPLUGIN_POLARITY {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DEVPLUGIN_POLARITY {
    #[inline(always)]
    fn from(val: u8) -> DEVPLUGIN_POLARITY {
        DEVPLUGIN_POLARITY::from_bits(val)
    }
}
impl From<DEVPLUGIN_POLARITY> for u8 {
    #[inline(always)]
    fn from(val: DEVPLUGIN_POLARITY) -> u8 {
        DEVPLUGIN_POLARITY::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DEVPLUGIN_STATUS {
    #[doc = "No attachment detected."]
    NO_CABLE = 0x0,
    #[doc = "Cable attachment detected."]
    CABLE_ATTACH = 0x01,
}
impl DEVPLUGIN_STATUS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DEVPLUGIN_STATUS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DEVPLUGIN_STATUS {
    #[inline(always)]
    fn from(val: u8) -> DEVPLUGIN_STATUS {
        DEVPLUGIN_STATUS::from_bits(val)
    }
}
impl From<DEVPLUGIN_STATUS> for u8 {
    #[inline(always)]
    fn from(val: DEVPLUGIN_STATUS) -> u8 {
        DEVPLUGIN_STATUS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DEV_PULLDOWN {
    #[doc = "Disable."]
    DEV_PULLDOWN_DIS = 0x0,
    #[doc = "Enable."]
    DEV_PULLDOWN_EN = 0x01,
}
impl DEV_PULLDOWN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DEV_PULLDOWN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DEV_PULLDOWN {
    #[inline(always)]
    fn from(val: u8) -> DEV_PULLDOWN {
        DEV_PULLDOWN::from_bits(val)
    }
}
impl From<DEV_PULLDOWN> for u8 {
    #[inline(always)]
    fn from(val: DEV_PULLDOWN) -> u8 {
        DEV_PULLDOWN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DISCHARGE_VBUS {
    #[doc = "Disable."]
    VBUS_DCHG_OFF = 0x0,
    #[doc = "Enable."]
    VBUS_DCHG_ON = 0x01,
}
impl DISCHARGE_VBUS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DISCHARGE_VBUS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DISCHARGE_VBUS {
    #[inline(always)]
    fn from(val: u8) -> DISCHARGE_VBUS {
        DISCHARGE_VBUS::from_bits(val)
    }
}
impl From<DISCHARGE_VBUS> for u8 {
    #[inline(always)]
    fn from(val: DISCHARGE_VBUS) -> u8 {
        DISCHARGE_VBUS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DISCONADJ {
    #[doc = "0.56875 V."]
    DISCON_TRIM_NOM = 0x0,
    #[doc = "0.55000 V."]
    DISCON_TRIM_LO = 0x01,
    #[doc = "0.58125 V."]
    DISCON_TRIM_MEDHI = 0x02,
    #[doc = "0.60000 V."]
    DISCON_TRIM_HI = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl DISCONADJ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DISCONADJ {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DISCONADJ {
    #[inline(always)]
    fn from(val: u8) -> DISCONADJ {
        DISCONADJ::from_bits(val)
    }
}
impl From<DISCONADJ> for u8 {
    #[inline(always)]
    fn from(val: DISCONADJ) -> u8 {
        DISCONADJ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DIV_SEL_OVERRIDE {
    #[doc = "TRIM_OVERRIDE_EN."]
    USE_TRIM0_PLLDIV = 0x0,
    #[doc = "PLL_SIC."]
    USE_PLL_SIC_PLLDIV = 0x01,
}
impl DIV_SEL_OVERRIDE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DIV_SEL_OVERRIDE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DIV_SEL_OVERRIDE {
    #[inline(always)]
    fn from(val: u8) -> DIV_SEL_OVERRIDE {
        DIV_SEL_OVERRIDE::from_bits(val)
    }
}
impl From<DIV_SEL_OVERRIDE> for u8 {
    #[inline(always)]
    fn from(val: DIV_SEL_OVERRIDE) -> u8 {
        DIV_SEL_OVERRIDE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DM_STATE {
    #[doc = "USB_DM pin voltage is <= 0.8 V."]
    DM_SERX_LO = 0x0,
    #[doc = "USB_DM pin voltage is >= 2.0 V."]
    DM_SERX_HI = 0x01,
}
impl DM_STATE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DM_STATE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DM_STATE {
    #[inline(always)]
    fn from(val: u8) -> DM_STATE {
        DM_STATE::from_bits(val)
    }
}
impl From<DM_STATE> for u8 {
    #[inline(always)]
    fn from(val: DM_STATE) -> u8 {
        DM_STATE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DP_STATE {
    #[doc = "USB_DP pin voltage is <= 0.8 V."]
    DP_SERX_LO = 0x0,
    #[doc = "USB_DP pin voltage is >= 2.0 V."]
    DP_SERX_HI = 0x01,
}
impl DP_STATE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DP_STATE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DP_STATE {
    #[inline(always)]
    fn from(val: u8) -> DP_STATE {
        DP_STATE::from_bits(val)
    }
}
impl From<DP_STATE> for u8 {
    #[inline(always)]
    fn from(val: DP_STATE) -> u8 {
        DP_STATE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum D_CAL {
    #[doc = "Maximum current, approximately 19% above nominal."]
    MAX_CURRENT = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Nominal."]
    NOMINAL = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Minimum current, approximately 19% below nominal."]
    MIN_CURRENT = 0x0f,
}
impl D_CAL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> D_CAL {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for D_CAL {
    #[inline(always)]
    fn from(val: u8) -> D_CAL {
        D_CAL::from_bits(val)
    }
}
impl From<D_CAL> for u8 {
    #[inline(always)]
    fn from(val: D_CAL) -> u8 {
        D_CAL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENAUTOCLR_CLKGATE {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl ENAUTOCLR_CLKGATE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENAUTOCLR_CLKGATE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENAUTOCLR_CLKGATE {
    #[inline(always)]
    fn from(val: u8) -> ENAUTOCLR_CLKGATE {
        ENAUTOCLR_CLKGATE::from_bits(val)
    }
}
impl From<ENAUTOCLR_CLKGATE> for u8 {
    #[inline(always)]
    fn from(val: ENAUTOCLR_CLKGATE) -> u8 {
        ENAUTOCLR_CLKGATE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENAUTOCLR_PHY_PWD {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl ENAUTOCLR_PHY_PWD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENAUTOCLR_PHY_PWD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENAUTOCLR_PHY_PWD {
    #[inline(always)]
    fn from(val: u8) -> ENAUTOCLR_PHY_PWD {
        ENAUTOCLR_PHY_PWD::from_bits(val)
    }
}
impl From<ENAUTOCLR_PHY_PWD> for u8 {
    #[inline(always)]
    fn from(val: ENAUTOCLR_PHY_PWD) -> u8 {
        ENAUTOCLR_PHY_PWD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDEVPLUGINDETECT {
    #[doc = "Disable."]
    PLUGIN_DISABLE = 0x0,
    #[doc = "Enable."]
    PLUGIN_ENABLE = 0x01,
}
impl ENDEVPLUGINDETECT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDEVPLUGINDETECT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDEVPLUGINDETECT {
    #[inline(always)]
    fn from(val: u8) -> ENDEVPLUGINDETECT {
        ENDEVPLUGINDETECT::from_bits(val)
    }
}
impl From<ENDEVPLUGINDETECT> for u8 {
    #[inline(always)]
    fn from(val: ENDEVPLUGINDETECT) -> u8 {
        ENDEVPLUGINDETECT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENHOSTDISCONDETECT {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl ENHOSTDISCONDETECT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENHOSTDISCONDETECT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENHOSTDISCONDETECT {
    #[inline(always)]
    fn from(val: u8) -> ENHOSTDISCONDETECT {
        ENHOSTDISCONDETECT::from_bits(val)
    }
}
impl From<ENHOSTDISCONDETECT> for u8 {
    #[inline(always)]
    fn from(val: ENHOSTDISCONDETECT) -> u8 {
        ENHOSTDISCONDETECT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENHSTPULLDOWN {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl ENHSTPULLDOWN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENHSTPULLDOWN {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENHSTPULLDOWN {
    #[inline(always)]
    fn from(val: u8) -> ENHSTPULLDOWN {
        ENHSTPULLDOWN::from_bits(val)
    }
}
impl From<ENHSTPULLDOWN> for u8 {
    #[inline(always)]
    fn from(val: ENHSTPULLDOWN) -> u8 {
        ENHSTPULLDOWN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENIRQDEVPLUGIN {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl ENIRQDEVPLUGIN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENIRQDEVPLUGIN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENIRQDEVPLUGIN {
    #[inline(always)]
    fn from(val: u8) -> ENIRQDEVPLUGIN {
        ENIRQDEVPLUGIN::from_bits(val)
    }
}
impl From<ENIRQDEVPLUGIN> for u8 {
    #[inline(always)]
    fn from(val: ENIRQDEVPLUGIN) -> u8 {
        ENIRQDEVPLUGIN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENIRQHOSTDISCON {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl ENIRQHOSTDISCON {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENIRQHOSTDISCON {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENIRQHOSTDISCON {
    #[inline(always)]
    fn from(val: u8) -> ENIRQHOSTDISCON {
        ENIRQHOSTDISCON::from_bits(val)
    }
}
impl From<ENIRQHOSTDISCON> for u8 {
    #[inline(always)]
    fn from(val: ENIRQHOSTDISCON) -> u8 {
        ENIRQHOSTDISCON::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENIRQRESUMEDETECT {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl ENIRQRESUMEDETECT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENIRQRESUMEDETECT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENIRQRESUMEDETECT {
    #[inline(always)]
    fn from(val: u8) -> ENIRQRESUMEDETECT {
        ENIRQRESUMEDETECT::from_bits(val)
    }
}
impl From<ENIRQRESUMEDETECT> for u8 {
    #[inline(always)]
    fn from(val: ENIRQRESUMEDETECT) -> u8 {
        ENIRQRESUMEDETECT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENIRQWAKEUP {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl ENIRQWAKEUP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENIRQWAKEUP {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENIRQWAKEUP {
    #[inline(always)]
    fn from(val: u8) -> ENIRQWAKEUP {
        ENIRQWAKEUP::from_bits(val)
    }
}
impl From<ENIRQWAKEUP> for u8 {
    #[inline(always)]
    fn from(val: ENIRQWAKEUP) -> u8 {
        ENIRQWAKEUP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENOTGIDDETECT {
    #[doc = "Disable."]
    ID_DET_DISABLE = 0x0,
    #[doc = "Enable."]
    ID_DET_ENABLE = 0x01,
}
impl ENOTGIDDETECT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENOTGIDDETECT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENOTGIDDETECT {
    #[inline(always)]
    fn from(val: u8) -> ENOTGIDDETECT {
        ENOTGIDDETECT::from_bits(val)
    }
}
impl From<ENOTGIDDETECT> for u8 {
    #[inline(always)]
    fn from(val: ENOTGIDDETECT) -> u8 {
        ENOTGIDDETECT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENOTG_ID_CHG_IRQ {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl ENOTG_ID_CHG_IRQ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENOTG_ID_CHG_IRQ {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENOTG_ID_CHG_IRQ {
    #[inline(always)]
    fn from(val: u8) -> ENOTG_ID_CHG_IRQ {
        ENOTG_ID_CHG_IRQ::from_bits(val)
    }
}
impl From<ENOTG_ID_CHG_IRQ> for u8 {
    #[inline(always)]
    fn from(val: ENOTG_ID_CHG_IRQ) -> u8 {
        ENOTG_ID_CHG_IRQ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENUTMILEVEL2 {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl ENUTMILEVEL2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENUTMILEVEL2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENUTMILEVEL2 {
    #[inline(always)]
    fn from(val: u8) -> ENUTMILEVEL2 {
        ENUTMILEVEL2::from_bits(val)
    }
}
impl From<ENUTMILEVEL2> for u8 {
    #[inline(always)]
    fn from(val: ENUTMILEVEL2) -> u8 {
        ENUTMILEVEL2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENUTMILEVEL3 {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl ENUTMILEVEL3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENUTMILEVEL3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENUTMILEVEL3 {
    #[inline(always)]
    fn from(val: u8) -> ENUTMILEVEL3 {
        ENUTMILEVEL3::from_bits(val)
    }
}
impl From<ENUTMILEVEL3> for u8 {
    #[inline(always)]
    fn from(val: ENUTMILEVEL3) -> u8 {
        ENUTMILEVEL3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENVADJ {
    #[doc = "0.1000 V."]
    ENV_TRIM_NOM = 0x0,
    #[doc = "0.1125 V."]
    ENV_TRIM_MEDHI = 0x01,
    #[doc = "0.1250 V."]
    ENV_TRIM_HI = 0x02,
    #[doc = "0.0875 V."]
    ENV_TRIM_LO = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl ENVADJ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENVADJ {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENVADJ {
    #[inline(always)]
    fn from(val: u8) -> ENVADJ {
        ENVADJ::from_bits(val)
    }
}
impl From<ENVADJ> for u8 {
    #[inline(always)]
    fn from(val: ENVADJ) -> u8 {
        ENVADJ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EN_B {
    #[doc = "Enable."]
    BC_ENABLE = 0x0,
    #[doc = "Disable."]
    BC_DISABLE = 0x01,
}
impl EN_B {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EN_B {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EN_B {
    #[inline(always)]
    fn from(val: u8) -> EN_B {
        EN_B::from_bits(val)
    }
}
impl From<EN_B> for u8 {
    #[inline(always)]
    fn from(val: EN_B) -> u8 {
        EN_B::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EXT_ID_OVERRIDE_EN {
    #[doc = "Internal detector or local override."]
    USE_PHY_ID = 0x0,
    #[doc = "External ID signal value."]
    USE_EXT_ID = 0x01,
}
impl EXT_ID_OVERRIDE_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EXT_ID_OVERRIDE_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EXT_ID_OVERRIDE_EN {
    #[inline(always)]
    fn from(val: u8) -> EXT_ID_OVERRIDE_EN {
        EXT_ID_OVERRIDE_EN::from_bits(val)
    }
}
impl From<EXT_ID_OVERRIDE_EN> for u8 {
    #[inline(always)]
    fn from(val: EXT_ID_OVERRIDE_EN) -> u8 {
        EXT_ID_OVERRIDE_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EXT_VBUS_OVERRIDE_EN {
    #[doc = "Internal detector or local override."]
    USE_PHY_VBUS = 0x0,
    #[doc = "External VBUS_VALID value."]
    USB_EXT_VBUS = 0x01,
}
impl EXT_VBUS_OVERRIDE_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EXT_VBUS_OVERRIDE_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EXT_VBUS_OVERRIDE_EN {
    #[inline(always)]
    fn from(val: u8) -> EXT_VBUS_OVERRIDE_EN {
        EXT_VBUS_OVERRIDE_EN::from_bits(val)
    }
}
impl From<EXT_VBUS_OVERRIDE_EN> for u8 {
    #[inline(always)]
    fn from(val: EXT_VBUS_OVERRIDE_EN) -> u8 {
        EXT_VBUS_OVERRIDE_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HOSTDISCONDETECT_IRQ {
    #[doc = "Connected."]
    CONNECTED = 0x0,
    #[doc = "Disconnected."]
    DISCONNECTED = 0x01,
}
impl HOSTDISCONDETECT_IRQ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HOSTDISCONDETECT_IRQ {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HOSTDISCONDETECT_IRQ {
    #[inline(always)]
    fn from(val: u8) -> HOSTDISCONDETECT_IRQ {
        HOSTDISCONDETECT_IRQ::from_bits(val)
    }
}
impl From<HOSTDISCONDETECT_IRQ> for u8 {
    #[inline(always)]
    fn from(val: HOSTDISCONDETECT_IRQ) -> u8 {
        HOSTDISCONDETECT_IRQ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HOSTDISCONDETECT_STATUS {
    #[doc = "Not detected."]
    NO_DISCONNECT = 0x0,
    #[doc = "Detected."]
    DISCONNECT = 0x01,
}
impl HOSTDISCONDETECT_STATUS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HOSTDISCONDETECT_STATUS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HOSTDISCONDETECT_STATUS {
    #[inline(always)]
    fn from(val: u8) -> HOSTDISCONDETECT_STATUS {
        HOSTDISCONDETECT_STATUS::from_bits(val)
    }
}
impl From<HOSTDISCONDETECT_STATUS> for u8 {
    #[inline(always)]
    fn from(val: HOSTDISCONDETECT_STATUS) -> u8 {
        HOSTDISCONDETECT_STATUS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HSTPULLDOWN {
    #[doc = "Disconnect."]
    DISCONNECT = 0x0,
    #[doc = "Connect."]
    CONNECT = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl HSTPULLDOWN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HSTPULLDOWN {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HSTPULLDOWN {
    #[inline(always)]
    fn from(val: u8) -> HSTPULLDOWN {
        HSTPULLDOWN::from_bits(val)
    }
}
impl From<HSTPULLDOWN> for u8 {
    #[inline(always)]
    fn from(val: HSTPULLDOWN) -> u8 {
        HSTPULLDOWN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ID_OVERRIDE_EN {
    #[doc = "Use ID pin detector or external override."]
    NO_PHY_ID_OVERRIDE = 0x0,
    #[doc = "Allow local override of ID pin detection status."]
    USE_PHY_ID_OVERRIDE = 0x01,
}
impl ID_OVERRIDE_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ID_OVERRIDE_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ID_OVERRIDE_EN {
    #[inline(always)]
    fn from(val: u8) -> ID_OVERRIDE_EN {
        ID_OVERRIDE_EN::from_bits(val)
    }
}
impl From<ID_OVERRIDE_EN> for u8 {
    #[inline(always)]
    fn from(val: ID_OVERRIDE_EN) -> u8 {
        ID_OVERRIDE_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LVI_EN {
    #[doc = "Disable."]
    LVI_3V_DISABLE = 0x0,
    #[doc = "Enable."]
    LVI_3V_ENABLE = 0x01,
}
impl LVI_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LVI_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LVI_EN {
    #[inline(always)]
    fn from(val: u8) -> LVI_EN {
        LVI_EN::from_bits(val)
    }
}
impl From<LVI_EN> for u8 {
    #[inline(always)]
    fn from(val: LVI_EN) -> u8 {
        LVI_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MISC2_CONTROL0 {
    #[doc = "Power up PLL."]
    PLL_ON_SUSPEND = 0x0,
    #[doc = "Power down PLL."]
    PLL_OFF_SUSPEND = 0x01,
}
impl MISC2_CONTROL0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MISC2_CONTROL0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MISC2_CONTROL0 {
    #[inline(always)]
    fn from(val: u8) -> MISC2_CONTROL0 {
        MISC2_CONTROL0::from_bits(val)
    }
}
impl From<MISC2_CONTROL0> for u8 {
    #[inline(always)]
    fn from(val: MISC2_CONTROL0) -> u8 {
        MISC2_CONTROL0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OK_STATUS_3V {
    #[doc = "Not powered."]
    POWER_3_1P8_OK = 0x0,
    #[doc = "Powered."]
    POWER_3_1P8_BAD = 0x01,
}
impl OK_STATUS_3V {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OK_STATUS_3V {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OK_STATUS_3V {
    #[inline(always)]
    fn from(val: u8) -> OK_STATUS_3V {
        OK_STATUS_3V::from_bits(val)
    }
}
impl From<OK_STATUS_3V> for u8 {
    #[inline(always)]
    fn from(val: OK_STATUS_3V) -> u8 {
        OK_STATUS_3V::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OTGID_STATUS {
    #[doc = "Host."]
    ID_HOST = 0x0,
    #[doc = "Device."]
    ID_DEVICE = 0x01,
}
impl OTGID_STATUS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OTGID_STATUS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OTGID_STATUS {
    #[inline(always)]
    fn from(val: u8) -> OTGID_STATUS {
        OTGID_STATUS::from_bits(val)
    }
}
impl From<OTGID_STATUS> for u8 {
    #[inline(always)]
    fn from(val: OTGID_STATUS) -> u8 {
        OTGID_STATUS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OTG_ID_CHG_IRQ {
    #[doc = "No ID change interrupt."]
    No_ID_CHG_IRQ = 0x0,
    #[doc = "ID change interrupt."]
    ID_CHG_IRQ = 0x01,
}
impl OTG_ID_CHG_IRQ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OTG_ID_CHG_IRQ {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OTG_ID_CHG_IRQ {
    #[inline(always)]
    fn from(val: u8) -> OTG_ID_CHG_IRQ {
        OTG_ID_CHG_IRQ::from_bits(val)
    }
}
impl From<OTG_ID_CHG_IRQ> for u8 {
    #[inline(always)]
    fn from(val: OTG_ID_CHG_IRQ) -> u8 {
        OTG_ID_CHG_IRQ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OTG_ID_VALUE {
    #[doc = "Host."]
    ID_HOST = 0x0,
    #[doc = "Device."]
    ID_DEVICE = 0x01,
}
impl OTG_ID_VALUE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OTG_ID_VALUE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OTG_ID_VALUE {
    #[inline(always)]
    fn from(val: u8) -> OTG_ID_VALUE {
        OTG_ID_VALUE::from_bits(val)
    }
}
impl From<OTG_ID_VALUE> for u8 {
    #[inline(always)]
    fn from(val: OTG_ID_VALUE) -> u8 {
        OTG_ID_VALUE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PFD0_CLKGATE {
    #[doc = "Enable."]
    PFD0_CLK_EN = 0x0,
    #[doc = "Disable."]
    PFD0_CLK_DIS = 0x01,
}
impl PFD0_CLKGATE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PFD0_CLKGATE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PFD0_CLKGATE {
    #[inline(always)]
    fn from(val: u8) -> PFD0_CLKGATE {
        PFD0_CLKGATE::from_bits(val)
    }
}
impl From<PFD0_CLKGATE> for u8 {
    #[inline(always)]
    fn from(val: PFD0_CLKGATE) -> u8 {
        PFD0_CLKGATE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PFD0_STABLE {
    #[doc = "Not stable."]
    NOT_STABLE = 0x0,
    #[doc = "Stable."]
    STABLE = 0x01,
}
impl PFD0_STABLE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PFD0_STABLE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PFD0_STABLE {
    #[inline(always)]
    fn from(val: u8) -> PFD0_STABLE {
        PFD0_STABLE::from_bits(val)
    }
}
impl From<PFD0_STABLE> for u8 {
    #[inline(always)]
    fn from(val: PFD0_STABLE) -> u8 {
        PFD0_STABLE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PFD_CLK_SEL {
    #[doc = "USB1PFDCLK = USB PLL reference clock."]
    PFD_CLK_BYPASS = 0x0,
    #[doc = "USB1PFDCLK = pfd_clk / 4."]
    PFD_CLK_DIV_4 = 0x01,
    #[doc = "USB1PFDCLK frequency = pfd_clk / 2."]
    PFD_CLK_DIV_2 = 0x02,
    #[doc = "USB1PFDCLK = pfd_clk."]
    PFD_CLK_DIV_1 = 0x03,
}
impl PFD_CLK_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PFD_CLK_SEL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PFD_CLK_SEL {
    #[inline(always)]
    fn from(val: u8) -> PFD_CLK_SEL {
        PFD_CLK_SEL::from_bits(val)
    }
}
impl From<PFD_CLK_SEL> for u8 {
    #[inline(always)]
    fn from(val: PFD_CLK_SEL) -> u8 {
        PFD_CLK_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PLL_BYPASS {
    #[doc = "480 MHz output clock."]
    PLL_NO_BYPASS = 0x0,
    #[doc = "Input reference clock."]
    PLL_BYPASS = 0x01,
}
impl PLL_BYPASS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PLL_BYPASS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PLL_BYPASS {
    #[inline(always)]
    fn from(val: u8) -> PLL_BYPASS {
        PLL_BYPASS::from_bits(val)
    }
}
impl From<PLL_BYPASS> for u8 {
    #[inline(always)]
    fn from(val: PLL_BYPASS) -> u8 {
        PLL_BYPASS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PLL_DIV_SEL {
    #[doc = "Configure for a 32 MHz input clock (divide by 15)."]
    PLL_DIV_15 = 0x0,
    #[doc = "Configure for a 30 MHz input clock (divide by 16)."]
    PLL_DIV_16 = 0x01,
    #[doc = "Configure for a 24 MHz input clock (divide by 20)."]
    PLL_DIV_20 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Configure for a 20 MHz input clock (divide by 24)."]
    PLL_DIV_24 = 0x04,
    #[doc = "Configure for a 19.2 MHz input clock (divide by 25)."]
    PLL_DIV_25 = 0x05,
    #[doc = "Configure for a 16 MHz input clock (divide by 30)."]
    PLL_DIV_30 = 0x06,
    #[doc = "Configure for a 12 MHz input clock (divide by 40)."]
    PLL_DIV_32 = 0x07,
}
impl PLL_DIV_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PLL_DIV_SEL {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PLL_DIV_SEL {
    #[inline(always)]
    fn from(val: u8) -> PLL_DIV_SEL {
        PLL_DIV_SEL::from_bits(val)
    }
}
impl From<PLL_DIV_SEL> for u8 {
    #[inline(always)]
    fn from(val: PLL_DIV_SEL) -> u8 {
        PLL_DIV_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PLL_ENABLE {
    #[doc = "Disable."]
    PLL_OUT_DISABLE = 0x0,
    #[doc = "Enable."]
    PLL_OUT_ENABLE = 0x01,
}
impl PLL_ENABLE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PLL_ENABLE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PLL_ENABLE {
    #[inline(always)]
    fn from(val: u8) -> PLL_ENABLE {
        PLL_ENABLE::from_bits(val)
    }
}
impl From<PLL_ENABLE> for u8 {
    #[inline(always)]
    fn from(val: PLL_ENABLE) -> u8 {
        PLL_ENABLE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PLL_EN_USB_CLKS {
    #[doc = "Disable."]
    PLL_MP_DISABLE = 0x0,
    #[doc = "Enable."]
    PLL_MP_ENABLE = 0x01,
}
impl PLL_EN_USB_CLKS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PLL_EN_USB_CLKS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PLL_EN_USB_CLKS {
    #[inline(always)]
    fn from(val: u8) -> PLL_EN_USB_CLKS {
        PLL_EN_USB_CLKS::from_bits(val)
    }
}
impl From<PLL_EN_USB_CLKS> for u8 {
    #[inline(always)]
    fn from(val: PLL_EN_USB_CLKS) -> u8 {
        PLL_EN_USB_CLKS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PLL_LOCK {
    #[doc = "Not locked."]
    PLL_NOT_LOCKED = 0x0,
    #[doc = "Locked."]
    PLL_LOCKED = 0x01,
}
impl PLL_LOCK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PLL_LOCK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PLL_LOCK {
    #[inline(always)]
    fn from(val: u8) -> PLL_LOCK {
        PLL_LOCK::from_bits(val)
    }
}
impl From<PLL_LOCK> for u8 {
    #[inline(always)]
    fn from(val: PLL_LOCK) -> u8 {
        PLL_LOCK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PLL_POWER {
    #[doc = "Power down."]
    PLL_FORCE_PWD = 0x0,
    #[doc = "Allow powerup."]
    PLL_ALLOW_POWERUP = 0x01,
}
impl PLL_POWER {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PLL_POWER {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PLL_POWER {
    #[inline(always)]
    fn from(val: u8) -> PLL_POWER {
        PLL_POWER::from_bits(val)
    }
}
impl From<PLL_POWER> for u8 {
    #[inline(always)]
    fn from(val: PLL_POWER) -> u8 {
        PLL_POWER::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PLL_PREDIV {
    #[doc = "Uses the undivided reference clock for PLL loop."]
    PREDIV_1 = 0x0,
    #[doc = "Divides the reference clock by two for PLL loop."]
    PREDIV_2 = 0x01,
}
impl PLL_PREDIV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PLL_PREDIV {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PLL_PREDIV {
    #[inline(always)]
    fn from(val: u8) -> PLL_PREDIV {
        PLL_PREDIV::from_bits(val)
    }
}
impl From<PLL_PREDIV> for u8 {
    #[inline(always)]
    fn from(val: PLL_PREDIV) -> u8 {
        PLL_PREDIV::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PLL_REG_ENABLE {
    #[doc = "Disable."]
    PLL_REG_DISABLE = 0x0,
    #[doc = "Enable."]
    PLL_REG_ENABLE = 0x01,
}
impl PLL_REG_ENABLE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PLL_REG_ENABLE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PLL_REG_ENABLE {
    #[inline(always)]
    fn from(val: u8) -> PLL_REG_ENABLE {
        PLL_REG_ENABLE::from_bits(val)
    }
}
impl From<PLL_REG_ENABLE> for u8 {
    #[inline(always)]
    fn from(val: PLL_REG_ENABLE) -> u8 {
        PLL_REG_ENABLE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PLUG_CONTACT {
    #[doc = "Not detected."]
    NO_DC_DETECTED = 0x0,
    #[doc = "Detected."]
    DC_DETECED = 0x01,
}
impl PLUG_CONTACT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PLUG_CONTACT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PLUG_CONTACT {
    #[inline(always)]
    fn from(val: u8) -> PLUG_CONTACT {
        PLUG_CONTACT::from_bits(val)
    }
}
impl From<PLUG_CONTACT> for u8 {
    #[inline(always)]
    fn from(val: PLUG_CONTACT) -> u8 {
        PLUG_CONTACT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PULLUP_DP {
    #[doc = "Disable."]
    DP_PUE_NORMAL = 0x0,
    #[doc = "Enable."]
    DP_PUE_OVERRIDE = 0x01,
}
impl PULLUP_DP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PULLUP_DP {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PULLUP_DP {
    #[inline(always)]
    fn from(val: u8) -> PULLUP_DP {
        PULLUP_DP::from_bits(val)
    }
}
impl From<PULLUP_DP> for u8 {
    #[inline(always)]
    fn from(val: PULLUP_DP) -> u8 {
        PULLUP_DP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum REFBIAS_PWD {
    #[doc = "Enable."]
    REFBIAS_ENABLED = 0x0,
    #[doc = "Disable or power down."]
    REFBIAS_PWD = 0x01,
}
impl REFBIAS_PWD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> REFBIAS_PWD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for REFBIAS_PWD {
    #[inline(always)]
    fn from(val: u8) -> REFBIAS_PWD {
        REFBIAS_PWD::from_bits(val)
    }
}
impl From<REFBIAS_PWD> for u8 {
    #[inline(always)]
    fn from(val: REFBIAS_PWD) -> u8 {
        REFBIAS_PWD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum REFBIAS_PWD_SEL {
    #[doc = "PLL_POWER internal state signal."]
    BIAS_PLLPOWER = 0x0,
    #[doc = "REFBIAS_PWD."]
    BIAS_REFBIAS_PWD = 0x01,
}
impl REFBIAS_PWD_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> REFBIAS_PWD_SEL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for REFBIAS_PWD_SEL {
    #[inline(always)]
    fn from(val: u8) -> REFBIAS_PWD_SEL {
        REFBIAS_PWD_SEL::from_bits(val)
    }
}
impl From<REFBIAS_PWD_SEL> for u8 {
    #[inline(always)]
    fn from(val: REFBIAS_PWD_SEL) -> u8 {
        REFBIAS_PWD_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RESUMEIRQSTICKY {
    #[doc = "During the resume or reset state signaling period."]
    DISABLE = 0x0,
    #[doc = "Until you write 0 to it."]
    ENABLE = 0x01,
}
impl RESUMEIRQSTICKY {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RESUMEIRQSTICKY {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RESUMEIRQSTICKY {
    #[inline(always)]
    fn from(val: u8) -> RESUMEIRQSTICKY {
        RESUMEIRQSTICKY::from_bits(val)
    }
}
impl From<RESUMEIRQSTICKY> for u8 {
    #[inline(always)]
    fn from(val: RESUMEIRQSTICKY) -> u8 {
        RESUMEIRQSTICKY::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RESUME_IRQ {
    #[doc = "No resume interrupt."]
    NORESIRQ = 0x0,
    #[doc = "Resume interrupt."]
    RESIRQ = 0x01,
}
impl RESUME_IRQ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RESUME_IRQ {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RESUME_IRQ {
    #[inline(always)]
    fn from(val: u8) -> RESUME_IRQ {
        RESUME_IRQ::from_bits(val)
    }
}
impl From<RESUME_IRQ> for u8 {
    #[inline(always)]
    fn from(val: RESUME_IRQ) -> u8 {
        RESUME_IRQ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RXPWD1PT1 {
    #[doc = "Enable."]
    FS_RXDIFF_ENABLE = 0x0,
    #[doc = "Disable or power down."]
    FS_RXDIFF_PWD = 0x01,
}
impl RXPWD1PT1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RXPWD1PT1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RXPWD1PT1 {
    #[inline(always)]
    fn from(val: u8) -> RXPWD1PT1 {
        RXPWD1PT1::from_bits(val)
    }
}
impl From<RXPWD1PT1> for u8 {
    #[inline(always)]
    fn from(val: RXPWD1PT1) -> u8 {
        RXPWD1PT1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RXPWDDIFF {
    #[doc = "Enable."]
    HS_RXDIFF_ENABLE = 0x0,
    #[doc = "Disable or power down."]
    HS_RXDIFF_PWD = 0x01,
}
impl RXPWDDIFF {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RXPWDDIFF {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RXPWDDIFF {
    #[inline(always)]
    fn from(val: u8) -> RXPWDDIFF {
        RXPWDDIFF::from_bits(val)
    }
}
impl From<RXPWDDIFF> for u8 {
    #[inline(always)]
    fn from(val: RXPWDDIFF) -> u8 {
        RXPWDDIFF::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RXPWDENV {
    #[doc = "Enable."]
    RX_ENVHD_ENABLE = 0x0,
    #[doc = "Disable or power down."]
    RX_ENVHD_PWD = 0x01,
}
impl RXPWDENV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RXPWDENV {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RXPWDENV {
    #[inline(always)]
    fn from(val: u8) -> RXPWDENV {
        RXPWDENV::from_bits(val)
    }
}
impl From<RXPWDENV> for u8 {
    #[inline(always)]
    fn from(val: RXPWDENV) -> u8 {
        RXPWDENV::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RXPWDRX {
    #[doc = "Enable."]
    RX_BIAS_ENABLE = 0x0,
    #[doc = "Disable or power down."]
    RX_BIAS_PWD = 0x01,
}
impl RXPWDRX {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RXPWDRX {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RXPWDRX {
    #[inline(always)]
    fn from(val: u8) -> RXPWDRX {
        RXPWDRX::from_bits(val)
    }
}
impl From<RXPWDRX> for u8 {
    #[inline(always)]
    fn from(val: RXPWDRX) -> u8 {
        RXPWDRX::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SECDET_DCP {
    #[doc = "CDP detected."]
    SECDET_CDP = 0x0,
    #[doc = "DCP detected."]
    SECDET_DCP = 0x01,
}
impl SECDET_DCP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SECDET_DCP {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SECDET_DCP {
    #[inline(always)]
    fn from(val: u8) -> SECDET_DCP {
        SECDET_DCP::from_bits(val)
    }
}
impl From<SECDET_DCP> for u8 {
    #[inline(always)]
    fn from(val: SECDET_DCP) -> u8 {
        SECDET_DCP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SESSEND {
    #[doc = "Above threshold."]
    SESSEND_LO = 0x0,
    #[doc = "Below threshold."]
    SESSEND_HI = 0x01,
}
impl SESSEND {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SESSEND {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SESSEND {
    #[inline(always)]
    fn from(val: u8) -> SESSEND {
        SESSEND::from_bits(val)
    }
}
impl From<SESSEND> for u8 {
    #[inline(always)]
    fn from(val: SESSEND) -> u8 {
        SESSEND::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SFTRST {
    #[doc = "Release from reset."]
    RELEASE_RESET = 0x0,
    #[doc = "Soft-reset."]
    SOFT_RESET = 0x01,
}
impl SFTRST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SFTRST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SFTRST {
    #[inline(always)]
    fn from(val: u8) -> SFTRST {
        SFTRST::from_bits(val)
    }
}
impl From<SFTRST> for u8 {
    #[inline(always)]
    fn from(val: SFTRST) -> u8 {
        SFTRST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TXPWDFS {
    #[doc = "Provide bias to enable."]
    FSTX_BIAS_ENABLE = 0x0,
    #[doc = "Disable or power down."]
    FSTX_BIAS_PWD = 0x01,
}
impl TXPWDFS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TXPWDFS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TXPWDFS {
    #[inline(always)]
    fn from(val: u8) -> TXPWDFS {
        TXPWDFS::from_bits(val)
    }
}
impl From<TXPWDFS> for u8 {
    #[inline(always)]
    fn from(val: TXPWDFS) -> u8 {
        TXPWDFS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TXPWDIBIAS {
    #[doc = "Enable."]
    IBIAS_ENABLE = 0x0,
    #[doc = "Disable or power down."]
    IBIAS_PWD = 0x01,
}
impl TXPWDIBIAS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TXPWDIBIAS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TXPWDIBIAS {
    #[inline(always)]
    fn from(val: u8) -> TXPWDIBIAS {
        TXPWDIBIAS::from_bits(val)
    }
}
impl From<TXPWDIBIAS> for u8 {
    #[inline(always)]
    fn from(val: TXPWDIBIAS) -> u8 {
        TXPWDIBIAS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TXPWDV2I {
    #[doc = "Enable."]
    V2I_BIAS_ENABLE = 0x0,
    #[doc = "Disable or power down."]
    V2I_BIAS_PWD = 0x01,
}
impl TXPWDV2I {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TXPWDV2I {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TXPWDV2I {
    #[inline(always)]
    fn from(val: u8) -> TXPWDV2I {
        TXPWDV2I::from_bits(val)
    }
}
impl From<TXPWDV2I> for u8 {
    #[inline(always)]
    fn from(val: TXPWDV2I) -> u8 {
        TXPWDV2I::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TX_CAL45DM_OVERRIDE {
    #[doc = "TRIM_OVERRIDE_EN."]
    USE_TRIM0_CAL45DN = 0x0,
    #[doc = "TX."]
    USE_TX_CAL45DN = 0x01,
}
impl TX_CAL45DM_OVERRIDE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TX_CAL45DM_OVERRIDE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TX_CAL45DM_OVERRIDE {
    #[inline(always)]
    fn from(val: u8) -> TX_CAL45DM_OVERRIDE {
        TX_CAL45DM_OVERRIDE::from_bits(val)
    }
}
impl From<TX_CAL45DM_OVERRIDE> for u8 {
    #[inline(always)]
    fn from(val: TX_CAL45DM_OVERRIDE) -> u8 {
        TX_CAL45DM_OVERRIDE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TX_CAL45DP_OVERRIDE {
    #[doc = "TRIM_OVERRIDE_EN."]
    USE_TRIM0_CAL45DP = 0x0,
    #[doc = "TX."]
    USE_TX_CAL45DP = 0x01,
}
impl TX_CAL45DP_OVERRIDE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TX_CAL45DP_OVERRIDE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TX_CAL45DP_OVERRIDE {
    #[inline(always)]
    fn from(val: u8) -> TX_CAL45DP_OVERRIDE {
        TX_CAL45DP_OVERRIDE::from_bits(val)
    }
}
impl From<TX_CAL45DP_OVERRIDE> for u8 {
    #[inline(always)]
    fn from(val: TX_CAL45DP_OVERRIDE) -> u8 {
        TX_CAL45DP_OVERRIDE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TX_D_CAL_OVERRIDE {
    #[doc = "TRIM_OVERRIDE_EN."]
    USE_TRIM0_DCAL = 0x0,
    #[doc = "TX."]
    USE_TX_DCAL = 0x01,
}
impl TX_D_CAL_OVERRIDE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TX_D_CAL_OVERRIDE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TX_D_CAL_OVERRIDE {
    #[inline(always)]
    fn from(val: u8) -> TX_D_CAL_OVERRIDE {
        TX_D_CAL_OVERRIDE::from_bits(val)
    }
}
impl From<TX_D_CAL_OVERRIDE> for u8 {
    #[inline(always)]
    fn from(val: TX_D_CAL_OVERRIDE) -> u8 {
        TX_D_CAL_OVERRIDE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USBPHY_TX_D_CAL {
    #[doc = "Maximum current, approximately 19% above nominal."]
    MAX_CURRENT = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Nominal."]
    NOMINAL = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Minimum current, approximately 19% below nominal."]
    MIN_CURRENT = 0x0f,
}
impl USBPHY_TX_D_CAL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USBPHY_TX_D_CAL {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USBPHY_TX_D_CAL {
    #[inline(always)]
    fn from(val: u8) -> USBPHY_TX_D_CAL {
        USBPHY_TX_D_CAL::from_bits(val)
    }
}
impl From<USBPHY_TX_D_CAL> for u8 {
    #[inline(always)]
    fn from(val: USBPHY_TX_D_CAL) -> u8 {
        USBPHY_TX_D_CAL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UTMI_SUSPENDM {
    #[doc = "Not suspended."]
    NOT_SUSPENDED = 0x0,
    #[doc = "Suspended."]
    SUSPENDED = 0x01,
}
impl UTMI_SUSPENDM {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UTMI_SUSPENDM {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UTMI_SUSPENDM {
    #[inline(always)]
    fn from(val: u8) -> UTMI_SUSPENDM {
        UTMI_SUSPENDM::from_bits(val)
    }
}
impl From<UTMI_SUSPENDM> for u8 {
    #[inline(always)]
    fn from(val: UTMI_SUSPENDM) -> u8 {
        UTMI_SUSPENDM::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VBUSVALID_PWRUP_CMPS {
    #[doc = "Disable or power down the VBUS_VALID comparator."]
    VBUS_VALID_DISABLE = 0x0,
    #[doc = "Enable the VBUS_VALID comparator."]
    VBUS_VALID_ENABLE = 0x01,
    #[doc = "Enable the session valid detector."]
    SESS_VLD_ENABLE = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Enable the VBUS_VALID_3V detector."]
    V3V_ENABLE = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl VBUSVALID_PWRUP_CMPS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VBUSVALID_PWRUP_CMPS {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VBUSVALID_PWRUP_CMPS {
    #[inline(always)]
    fn from(val: u8) -> VBUSVALID_PWRUP_CMPS {
        VBUSVALID_PWRUP_CMPS::from_bits(val)
    }
}
impl From<VBUSVALID_PWRUP_CMPS> for u8 {
    #[inline(always)]
    fn from(val: VBUSVALID_PWRUP_CMPS) -> u8 {
        VBUSVALID_PWRUP_CMPS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VBUSVALID_SEL {
    #[doc = "VBUS_VALID comparator result."]
    VBUS_VLD_OUT = 0x0,
    #[doc = "VBUS_VALID_3V comparator result."]
    VBUS_VLD_3V_OUT = 0x01,
}
impl VBUSVALID_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VBUSVALID_SEL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VBUSVALID_SEL {
    #[inline(always)]
    fn from(val: u8) -> VBUSVALID_SEL {
        VBUSVALID_SEL::from_bits(val)
    }
}
impl From<VBUSVALID_SEL> for u8 {
    #[inline(always)]
    fn from(val: VBUSVALID_SEL) -> u8 {
        VBUSVALID_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VBUSVALID_THRESH {
    #[doc = "4.0 V."]
    VBUS_VLD_4P0 = 0x0,
    #[doc = "4.1 V."]
    VBUS_VLD_4P1 = 0x01,
    #[doc = "4.2 V."]
    VBUS_VLD_4P2 = 0x02,
    #[doc = "4.3 V."]
    VBUS_VLD_4P3 = 0x03,
    #[doc = "4.4 V."]
    VBUS_VLD_4P4 = 0x04,
    #[doc = "4.5 V."]
    VBUS_VLD_4P5 = 0x05,
    #[doc = "4.6 V."]
    VBUS_VLD_4P6 = 0x06,
    #[doc = "4.7 V."]
    VBUS_VLD_4P7 = 0x07,
}
impl VBUSVALID_THRESH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VBUSVALID_THRESH {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VBUSVALID_THRESH {
    #[inline(always)]
    fn from(val: u8) -> VBUSVALID_THRESH {
        VBUSVALID_THRESH::from_bits(val)
    }
}
impl From<VBUSVALID_THRESH> for u8 {
    #[inline(always)]
    fn from(val: VBUSVALID_THRESH) -> u8 {
        VBUSVALID_THRESH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VBUSVALID_TO_B {
    #[doc = "VBUS_VALID comparator."]
    USE_VBUS_VLD = 0x0,
    #[doc = "Session valid detector."]
    USE_SESS_VLD = 0x01,
}
impl VBUSVALID_TO_B {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VBUSVALID_TO_B {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VBUSVALID_TO_B {
    #[inline(always)]
    fn from(val: u8) -> VBUSVALID_TO_B {
        VBUSVALID_TO_B::from_bits(val)
    }
}
impl From<VBUSVALID_TO_B> for u8 {
    #[inline(always)]
    fn from(val: VBUSVALID_TO_B) -> u8 {
        VBUSVALID_TO_B::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VBUS_OVERRIDE_EN {
    #[doc = "Results of VBUS_VALID and session valid comparators for VBUS_VALID, AVALID, BVALID, and SESSEND."]
    VBUS_NO_OVERRIDE = 0x0,
    #[doc = "Override values for VBUS_VALID, AVALID, BVALID, and SESSEND."]
    VBUS_OVERRIDE = 0x01,
}
impl VBUS_OVERRIDE_EN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VBUS_OVERRIDE_EN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VBUS_OVERRIDE_EN {
    #[inline(always)]
    fn from(val: u8) -> VBUS_OVERRIDE_EN {
        VBUS_OVERRIDE_EN::from_bits(val)
    }
}
impl From<VBUS_OVERRIDE_EN> for u8 {
    #[inline(always)]
    fn from(val: VBUS_OVERRIDE_EN) -> u8 {
        VBUS_OVERRIDE_EN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VBUS_SOURCE_SEL {
    #[doc = "VBUS_VALID comparator result."]
    USE_VBUS_VLD = 0x0,
    #[doc = "Session valid comparator result."]
    USE_ASESS_VLD = 0x01,
    #[doc = "Session valid comparator result."]
    USE_BSESS_VLD = 0x02,
    _RESERVED_3 = 0x03,
}
impl VBUS_SOURCE_SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VBUS_SOURCE_SEL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VBUS_SOURCE_SEL {
    #[inline(always)]
    fn from(val: u8) -> VBUS_SOURCE_SEL {
        VBUS_SOURCE_SEL::from_bits(val)
    }
}
impl From<VBUS_SOURCE_SEL> for u8 {
    #[inline(always)]
    fn from(val: VBUS_SOURCE_SEL) -> u8 {
        VBUS_SOURCE_SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VBUS_VALID {
    #[doc = "Below threshold."]
    VBUS_LO = 0x0,
    #[doc = "Above threshold."]
    VBUS_HI = 0x01,
}
impl VBUS_VALID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VBUS_VALID {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VBUS_VALID {
    #[inline(always)]
    fn from(val: u8) -> VBUS_VALID {
        VBUS_VALID::from_bits(val)
    }
}
impl From<VBUS_VALID> for u8 {
    #[inline(always)]
    fn from(val: VBUS_VALID) -> u8 {
        VBUS_VALID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VBUS_VALID_3V {
    #[doc = "Below threshold."]
    VBUS_VLD3V_LO = 0x0,
    #[doc = "Above threshold."]
    VBUS_VLD3V_HI = 0x01,
}
impl VBUS_VALID_3V {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VBUS_VALID_3V {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VBUS_VALID_3V {
    #[inline(always)]
    fn from(val: u8) -> VBUS_VALID_3V {
        VBUS_VALID_3V::from_bits(val)
    }
}
impl From<VBUS_VALID_3V> for u8 {
    #[inline(always)]
    fn from(val: VBUS_VALID_3V) -> u8 {
        VBUS_VALID_3V::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VDM_SRC_ENABLE {
    #[doc = "Disable."]
    DCD_VDM_SRC_DISABLE = 0x0,
    #[doc = "Enable."]
    DCD_VDM_SRC_ENABLE = 0x01,
}
impl VDM_SRC_ENABLE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VDM_SRC_ENABLE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VDM_SRC_ENABLE {
    #[inline(always)]
    fn from(val: u8) -> VDM_SRC_ENABLE {
        VDM_SRC_ENABLE::from_bits(val)
    }
}
impl From<VDM_SRC_ENABLE> for u8 {
    #[inline(always)]
    fn from(val: VDM_SRC_ENABLE) -> u8 {
        VDM_SRC_ENABLE::to_bits(val)
    }
}
