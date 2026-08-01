#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "USBPHY."]
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
    pub const fn pwd(self) -> crate::pac::common::Reg<Pwd, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Power Down."]
    #[inline(always)]
    pub const fn pwd_set(self) -> crate::pac::common::Reg<PwdSet, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Power Down."]
    #[inline(always)]
    pub const fn pwd_clr(self) -> crate::pac::common::Reg<PwdClr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Power Down."]
    #[inline(always)]
    pub const fn pwd_tog(self) -> crate::pac::common::Reg<PwdTog, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "TX Control."]
    #[inline(always)]
    pub const fn tx(self) -> crate::pac::common::Reg<Tx, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "TX Control."]
    #[inline(always)]
    pub const fn tx_set(self) -> crate::pac::common::Reg<TxSet, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "TX Control."]
    #[inline(always)]
    pub const fn tx_clr(self) -> crate::pac::common::Reg<TxClr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "TX Control."]
    #[inline(always)]
    pub const fn tx_tog(self) -> crate::pac::common::Reg<TxTog, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "RX Control."]
    #[inline(always)]
    pub const fn rx(self) -> crate::pac::common::Reg<Rx, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "RX Control."]
    #[inline(always)]
    pub const fn rx_set(self) -> crate::pac::common::Reg<RxSet, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "RX Control."]
    #[inline(always)]
    pub const fn rx_clr(self) -> crate::pac::common::Reg<RxClr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "RX Control."]
    #[inline(always)]
    pub const fn rx_tog(self) -> crate::pac::common::Reg<RxTog, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "General Purpose Control."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::pac::common::Reg<Ctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "General Purpose Control."]
    #[inline(always)]
    pub const fn ctrl_set(self) -> crate::pac::common::Reg<CtrlSet, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "General Purpose Control."]
    #[inline(always)]
    pub const fn ctrl_clr(self) -> crate::pac::common::Reg<CtrlClr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "General Purpose Control."]
    #[inline(always)]
    pub const fn ctrl_tog(self) -> crate::pac::common::Reg<CtrlTog, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Status."]
    #[inline(always)]
    pub const fn status(self) -> crate::pac::common::Reg<Status, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Debug 0."]
    #[inline(always)]
    pub const fn debug0(self) -> crate::pac::common::Reg<Debug0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Debug 0."]
    #[inline(always)]
    pub const fn debug0_set(self) -> crate::pac::common::Reg<Debug0Set, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Debug 0."]
    #[inline(always)]
    pub const fn debug0_clr(self) -> crate::pac::common::Reg<Debug0Clr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Debug 0."]
    #[inline(always)]
    pub const fn debug0_tog(self) -> crate::pac::common::Reg<Debug0Tog, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "Version."]
    #[inline(always)]
    pub const fn version(self) -> crate::pac::common::Reg<Version, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "IP Block."]
    #[inline(always)]
    pub const fn ip(self) -> crate::pac::common::Reg<Ip, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "IP Block."]
    #[inline(always)]
    pub const fn ip_set(self) -> crate::pac::common::Reg<IpSet, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "IP Block."]
    #[inline(always)]
    pub const fn ip_clr(self) -> crate::pac::common::Reg<IpClr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "IP Block."]
    #[inline(always)]
    pub const fn ip_tog(self) -> crate::pac::common::Reg<IpTog, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "PLL SIC."]
    #[inline(always)]
    pub const fn pll_sic(self) -> crate::pac::common::Reg<PllSic, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "PLL SIC."]
    #[inline(always)]
    pub const fn pll_sic_set(self) -> crate::pac::common::Reg<PllSicSet, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "PLL SIC."]
    #[inline(always)]
    pub const fn pll_sic_clr(self) -> crate::pac::common::Reg<PllSicClr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "PLL SIC."]
    #[inline(always)]
    pub const fn pll_sic_tog(self) -> crate::pac::common::Reg<PllSicTog, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "VBUS Detect."]
    #[inline(always)]
    pub const fn usb1_vbus_detect(
        self,
    ) -> crate::pac::common::Reg<Usb1VbusDetect, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "VBUS Detect."]
    #[inline(always)]
    pub const fn usb1_vbus_detect_set(
        self,
    ) -> crate::pac::common::Reg<Usb1VbusDetectSet, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "VBUS Detect."]
    #[inline(always)]
    pub const fn usb1_vbus_detect_clr(
        self,
    ) -> crate::pac::common::Reg<Usb1VbusDetectClr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "VBUS Detect."]
    #[inline(always)]
    pub const fn usb1_vbus_detect_tog(
        self,
    ) -> crate::pac::common::Reg<Usb1VbusDetectTog, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "VBUS Detect Status."]
    #[inline(always)]
    pub const fn usb1_vbus_det_stat(
        self,
    ) -> crate::pac::common::Reg<Usb1VbusDetStat, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "VBUS Detect Status."]
    #[inline(always)]
    pub const fn usb1_vbus_det_stat_set(
        self,
    ) -> crate::pac::common::Reg<Usb1VbusDetStatSet, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "VBUS Detect Status."]
    #[inline(always)]
    pub const fn usb1_vbus_det_stat_clr(
        self,
    ) -> crate::pac::common::Reg<Usb1VbusDetStatClr, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "VBUS Detect Status."]
    #[inline(always)]
    pub const fn usb1_vbus_det_stat_tog(
        self,
    ) -> crate::pac::common::Reg<Usb1VbusDetStatTog, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "Charger Detect."]
    #[inline(always)]
    pub const fn usb1_chrg_detect(
        self,
    ) -> crate::pac::common::Reg<Usb1ChrgDetect, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "Charger Detect."]
    #[inline(always)]
    pub const fn usb1_chrg_detect_set(
        self,
    ) -> crate::pac::common::Reg<Usb1ChrgDetectSet, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Charger Detect."]
    #[inline(always)]
    pub const fn usb1_chrg_detect_clr(
        self,
    ) -> crate::pac::common::Reg<Usb1ChrgDetectClr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "Charger Detect."]
    #[inline(always)]
    pub const fn usb1_chrg_detect_tog(
        self,
    ) -> crate::pac::common::Reg<Usb1ChrgDetectTog, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "Charger Detect Status."]
    #[inline(always)]
    pub const fn usb1_chrg_det_stat(
        self,
    ) -> crate::pac::common::Reg<Usb1ChrgDetStat, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "Charger Detect Status."]
    #[inline(always)]
    pub const fn usb1_chrg_det_stat_set(
        self,
    ) -> crate::pac::common::Reg<Usb1ChrgDetStatSet, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "Charger Detect Status."]
    #[inline(always)]
    pub const fn usb1_chrg_det_stat_clr(
        self,
    ) -> crate::pac::common::Reg<Usb1ChrgDetStatClr, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "Charger Detect Status."]
    #[inline(always)]
    pub const fn usb1_chrg_det_stat_tog(
        self,
    ) -> crate::pac::common::Reg<Usb1ChrgDetStatTog, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "Analog Control."]
    #[inline(always)]
    pub const fn anactrl(self) -> crate::pac::common::Reg<Anactrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Analog Control."]
    #[inline(always)]
    pub const fn anactrl_set(self) -> crate::pac::common::Reg<AnactrlSet, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Analog Control."]
    #[inline(always)]
    pub const fn anactrl_clr(self) -> crate::pac::common::Reg<AnactrlClr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Analog Control."]
    #[inline(always)]
    pub const fn anactrl_tog(self) -> crate::pac::common::Reg<AnactrlTog, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Trim."]
    #[inline(always)]
    pub const fn trim_override_en(
        self,
    ) -> crate::pac::common::Reg<TrimOverrideEn, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "Trim."]
    #[inline(always)]
    pub const fn trim_override_en_set(
        self,
    ) -> crate::pac::common::Reg<TrimOverrideEnSet, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "Trim."]
    #[inline(always)]
    pub const fn trim_override_en_clr(
        self,
    ) -> crate::pac::common::Reg<TrimOverrideEnClr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "Trim."]
    #[inline(always)]
    pub const fn trim_override_en_tog(
        self,
    ) -> crate::pac::common::Reg<TrimOverrideEnTog, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "PFD A."]
    #[inline(always)]
    pub const fn pfda(self) -> crate::pac::common::Reg<Pfda, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "PFD A."]
    #[inline(always)]
    pub const fn pfda_set(self) -> crate::pac::common::Reg<PfdaSet, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "PFD A."]
    #[inline(always)]
    pub const fn pfda_clr(self) -> crate::pac::common::Reg<PfdaClr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "PFD A."]
    #[inline(always)]
    pub const fn pfda_tog(self) -> crate::pac::common::Reg<PfdaTog, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
}
#[doc = "Analog Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Anactrl(pub u32);
impl Anactrl {
    #[doc = "Internal Low Voltage Detector Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lvi_en(&self) -> LviEn {
        let val = (self.0 >> 1usize) & 0x01;
        LviEn::from_bits(val as u8)
    }
    #[doc = "Internal Low Voltage Detector Enable."]
    #[inline(always)]
    pub const fn set_lvi_en(&mut self, val: LviEn) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "PFD Clock Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pfd_clk_sel(&self) -> PfdClkSel {
        let val = (self.0 >> 2usize) & 0x03;
        PfdClkSel::from_bits(val as u8)
    }
    #[doc = "PFD Clock Selection."]
    #[inline(always)]
    pub const fn set_pfd_clk_sel(&mut self, val: PfdClkSel) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Device Pulldown Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dev_pulldown(&self) -> DevPulldown {
        let val = (self.0 >> 10usize) & 0x01;
        DevPulldown::from_bits(val as u8)
    }
    #[doc = "Device Pulldown Enable."]
    #[inline(always)]
    pub const fn set_dev_pulldown(&mut self, val: DevPulldown) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for Anactrl {
    #[inline(always)]
    fn default() -> Anactrl {
        Anactrl(0)
    }
}
impl core::fmt::Debug for Anactrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Anactrl")
            .field("lvi_en", &self.lvi_en())
            .field("pfd_clk_sel", &self.pfd_clk_sel())
            .field("dev_pulldown", &self.dev_pulldown())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Anactrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Anactrl {{ lvi_en: {:?}, pfd_clk_sel: {:?}, dev_pulldown: {:?} }}",
            self.lvi_en(),
            self.pfd_clk_sel(),
            self.dev_pulldown()
        )
    }
}
#[doc = "Analog Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnactrlClr(pub u32);
impl AnactrlClr {
    #[doc = "Internal Low Voltage Detector Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lvi_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Internal Low Voltage Detector Enable."]
    #[inline(always)]
    pub const fn set_lvi_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "PFD Clock Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pfd_clk_sel(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "PFD Clock Selection."]
    #[inline(always)]
    pub const fn set_pfd_clk_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Device Pulldown Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dev_pulldown(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Device Pulldown Enable."]
    #[inline(always)]
    pub const fn set_dev_pulldown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for AnactrlClr {
    #[inline(always)]
    fn default() -> AnactrlClr {
        AnactrlClr(0)
    }
}
impl core::fmt::Debug for AnactrlClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AnactrlClr")
            .field("lvi_en", &self.lvi_en())
            .field("pfd_clk_sel", &self.pfd_clk_sel())
            .field("dev_pulldown", &self.dev_pulldown())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AnactrlClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AnactrlClr {{ lvi_en: {=bool:?}, pfd_clk_sel: {=u8:?}, dev_pulldown: {=bool:?} }}",
            self.lvi_en(),
            self.pfd_clk_sel(),
            self.dev_pulldown()
        )
    }
}
#[doc = "Analog Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnactrlSet(pub u32);
impl AnactrlSet {
    #[doc = "Internal Low Voltage Detector Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lvi_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Internal Low Voltage Detector Enable."]
    #[inline(always)]
    pub const fn set_lvi_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "PFD Clock Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pfd_clk_sel(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "PFD Clock Selection."]
    #[inline(always)]
    pub const fn set_pfd_clk_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Device Pulldown Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dev_pulldown(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Device Pulldown Enable."]
    #[inline(always)]
    pub const fn set_dev_pulldown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for AnactrlSet {
    #[inline(always)]
    fn default() -> AnactrlSet {
        AnactrlSet(0)
    }
}
impl core::fmt::Debug for AnactrlSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AnactrlSet")
            .field("lvi_en", &self.lvi_en())
            .field("pfd_clk_sel", &self.pfd_clk_sel())
            .field("dev_pulldown", &self.dev_pulldown())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AnactrlSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AnactrlSet {{ lvi_en: {=bool:?}, pfd_clk_sel: {=u8:?}, dev_pulldown: {=bool:?} }}",
            self.lvi_en(),
            self.pfd_clk_sel(),
            self.dev_pulldown()
        )
    }
}
#[doc = "Analog Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnactrlTog(pub u32);
impl AnactrlTog {
    #[doc = "Internal Low Voltage Detector Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lvi_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Internal Low Voltage Detector Enable."]
    #[inline(always)]
    pub const fn set_lvi_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "PFD Clock Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn pfd_clk_sel(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "PFD Clock Selection."]
    #[inline(always)]
    pub const fn set_pfd_clk_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Device Pulldown Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dev_pulldown(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Device Pulldown Enable."]
    #[inline(always)]
    pub const fn set_dev_pulldown(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for AnactrlTog {
    #[inline(always)]
    fn default() -> AnactrlTog {
        AnactrlTog(0)
    }
}
impl core::fmt::Debug for AnactrlTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AnactrlTog")
            .field("lvi_en", &self.lvi_en())
            .field("pfd_clk_sel", &self.pfd_clk_sel())
            .field("dev_pulldown", &self.dev_pulldown())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AnactrlTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AnactrlTog {{ lvi_en: {=bool:?}, pfd_clk_sel: {=u8:?}, dev_pulldown: {=bool:?} }}",
            self.lvi_en(),
            self.pfd_clk_sel(),
            self.dev_pulldown()
        )
    }
}
#[doc = "General Purpose Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "OTG ID Change Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enotg_id_chg_irq(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID Change Interrupt Enable."]
    #[inline(always)]
    pub const fn set_enotg_id_chg_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Host Disconnect Detection Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enhostdiscondetect(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Host Disconnect Detection Enable."]
    #[inline(always)]
    pub const fn set_enhostdiscondetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable Interrupt for Host Disconnect."]
    #[must_use]
    #[inline(always)]
    pub const fn enirqhostdiscon(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Interrupt for Host Disconnect."]
    #[inline(always)]
    pub const fn set_enirqhostdiscon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Host Disconnect Detection Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn hostdiscondetect_irq(&self) -> HostdiscondetectIrq {
        let val = (self.0 >> 3usize) & 0x01;
        HostdiscondetectIrq::from_bits(val as u8)
    }
    #[doc = "Host Disconnect Detection Interrupt."]
    #[inline(always)]
    pub const fn set_hostdiscondetect_irq(&mut self, val: HostdiscondetectIrq) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable Nonstandard Resistive Plugged-In Detection."]
    #[must_use]
    #[inline(always)]
    pub const fn endevplugindetect(&self) -> Endevplugindetect {
        let val = (self.0 >> 4usize) & 0x01;
        Endevplugindetect::from_bits(val as u8)
    }
    #[doc = "Enable Nonstandard Resistive Plugged-In Detection."]
    #[inline(always)]
    pub const fn set_endevplugindetect(&mut self, val: Endevplugindetect) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Device Plug-In Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn devplugin_polarity(&self) -> DevpluginPolarity {
        let val = (self.0 >> 5usize) & 0x01;
        DevpluginPolarity::from_bits(val as u8)
    }
    #[doc = "Device Plug-In Polarity."]
    #[inline(always)]
    pub const fn set_devplugin_polarity(&mut self, val: DevpluginPolarity) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "OTG ID Change Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn otg_id_chg_irq(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID Change Interrupt."]
    #[inline(always)]
    pub const fn set_otg_id_chg_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable Internal OTG ID Detector."]
    #[must_use]
    #[inline(always)]
    pub const fn enotgiddetect(&self) -> Enotgiddetect {
        let val = (self.0 >> 7usize) & 0x01;
        Enotgiddetect::from_bits(val as u8)
    }
    #[doc = "Enable Internal OTG ID Detector."]
    #[inline(always)]
    pub const fn set_enotgiddetect(&mut self, val: Enotgiddetect) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Resume Interrupt Sticky."]
    #[must_use]
    #[inline(always)]
    pub const fn resumeirqsticky(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Interrupt Sticky."]
    #[inline(always)]
    pub const fn set_resumeirqsticky(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Resume Detection Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enirqresumedetect(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Detection Interrupt Enable."]
    #[inline(always)]
    pub const fn set_enirqresumedetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Resume Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn resume_irq(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Interrupt."]
    #[inline(always)]
    pub const fn set_resume_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enable Interrupt for Nonstandard Resistive Plugged-In Detection."]
    #[must_use]
    #[inline(always)]
    pub const fn enirqdevplugin(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Interrupt for Nonstandard Resistive Plugged-In Detection."]
    #[inline(always)]
    pub const fn set_enirqdevplugin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Device Plug-In Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn devplugin_irq(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Device Plug-In Interrupt."]
    #[inline(always)]
    pub const fn set_devplugin_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "APB Clock Switch Option."]
    #[must_use]
    #[inline(always)]
    pub const fn data_on_lradc(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "APB Clock Switch Option."]
    #[inline(always)]
    pub const fn set_data_on_lradc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "UTMI Level 2 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enutmilevel2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Level 2 Enable."]
    #[inline(always)]
    pub const fn set_enutmilevel2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "UTMI Level 3 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enutmilevel3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Level 3 Enable."]
    #[inline(always)]
    pub const fn set_enutmilevel3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Wake-Up Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enirqwakeup(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-Up Interrupt Enable."]
    #[inline(always)]
    pub const fn set_enirqwakeup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Wake-Up Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup_irq(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-Up Interrupt."]
    #[inline(always)]
    pub const fn set_wakeup_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Autoresume Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn autoresume_en(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Autoresume Enable."]
    #[inline(always)]
    pub const fn set_autoresume_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Autoclear Clock Gate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enautoclr_clkgate(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Autoclear Clock Gate Enable."]
    #[inline(always)]
    pub const fn set_enautoclr_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "PHY PWD Autoclear Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enautoclr_phy_pwd(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "PHY PWD Autoclear Enable."]
    #[inline(always)]
    pub const fn set_enautoclr_phy_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "OTG ID Value."]
    #[must_use]
    #[inline(always)]
    pub const fn otg_id_value(&self) -> OtgIdValue {
        let val = (self.0 >> 27usize) & 0x01;
        OtgIdValue::from_bits(val as u8)
    }
    #[doc = "OTG ID Value."]
    #[inline(always)]
    pub const fn set_otg_id_value(&mut self, val: OtgIdValue) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "UTMI Suspend."]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_suspendm(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Suspend."]
    #[inline(always)]
    pub const fn set_utmi_suspendm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "UTMI Clock Gate."]
    #[must_use]
    #[inline(always)]
    pub const fn clkgate(&self) -> Clkgate {
        let val = (self.0 >> 30usize) & 0x01;
        Clkgate::from_bits(val as u8)
    }
    #[doc = "UTMI Clock Gate."]
    #[inline(always)]
    pub const fn set_clkgate(&mut self, val: Clkgate) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Software Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn sftrst(&self) -> Sftrst {
        let val = (self.0 >> 31usize) & 0x01;
        Sftrst::from_bits(val as u8)
    }
    #[doc = "Software Reset."]
    #[inline(always)]
    pub const fn set_sftrst(&mut self, val: Sftrst) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
impl core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl")
            .field("enotg_id_chg_irq", &self.enotg_id_chg_irq())
            .field("enhostdiscondetect", &self.enhostdiscondetect())
            .field("enirqhostdiscon", &self.enirqhostdiscon())
            .field("hostdiscondetect_irq", &self.hostdiscondetect_irq())
            .field("endevplugindetect", &self.endevplugindetect())
            .field("devplugin_polarity", &self.devplugin_polarity())
            .field("otg_id_chg_irq", &self.otg_id_chg_irq())
            .field("enotgiddetect", &self.enotgiddetect())
            .field("resumeirqsticky", &self.resumeirqsticky())
            .field("enirqresumedetect", &self.enirqresumedetect())
            .field("resume_irq", &self.resume_irq())
            .field("enirqdevplugin", &self.enirqdevplugin())
            .field("devplugin_irq", &self.devplugin_irq())
            .field("data_on_lradc", &self.data_on_lradc())
            .field("enutmilevel2", &self.enutmilevel2())
            .field("enutmilevel3", &self.enutmilevel3())
            .field("enirqwakeup", &self.enirqwakeup())
            .field("wakeup_irq", &self.wakeup_irq())
            .field("autoresume_en", &self.autoresume_en())
            .field("enautoclr_clkgate", &self.enautoclr_clkgate())
            .field("enautoclr_phy_pwd", &self.enautoclr_phy_pwd())
            .field("otg_id_value", &self.otg_id_value())
            .field("utmi_suspendm", &self.utmi_suspendm())
            .field("clkgate", &self.clkgate())
            .field("sftrst", &self.sftrst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ enotg_id_chg_irq: {=bool:?}, enhostdiscondetect: {=bool:?}, enirqhostdiscon: {=bool:?}, hostdiscondetect_irq: {:?}, endevplugindetect: {:?}, devplugin_polarity: {:?}, otg_id_chg_irq: {=bool:?}, enotgiddetect: {:?}, resumeirqsticky: {=bool:?}, enirqresumedetect: {=bool:?}, resume_irq: {=bool:?}, enirqdevplugin: {=bool:?}, devplugin_irq: {=bool:?}, data_on_lradc: {=bool:?}, enutmilevel2: {=bool:?}, enutmilevel3: {=bool:?}, enirqwakeup: {=bool:?}, wakeup_irq: {=bool:?}, autoresume_en: {=bool:?}, enautoclr_clkgate: {=bool:?}, enautoclr_phy_pwd: {=bool:?}, otg_id_value: {:?}, utmi_suspendm: {=bool:?}, clkgate: {:?}, sftrst: {:?} }}",
            self.enotg_id_chg_irq(),
            self.enhostdiscondetect(),
            self.enirqhostdiscon(),
            self.hostdiscondetect_irq(),
            self.endevplugindetect(),
            self.devplugin_polarity(),
            self.otg_id_chg_irq(),
            self.enotgiddetect(),
            self.resumeirqsticky(),
            self.enirqresumedetect(),
            self.resume_irq(),
            self.enirqdevplugin(),
            self.devplugin_irq(),
            self.data_on_lradc(),
            self.enutmilevel2(),
            self.enutmilevel3(),
            self.enirqwakeup(),
            self.wakeup_irq(),
            self.autoresume_en(),
            self.enautoclr_clkgate(),
            self.enautoclr_phy_pwd(),
            self.otg_id_value(),
            self.utmi_suspendm(),
            self.clkgate(),
            self.sftrst()
        )
    }
}
#[doc = "General Purpose Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtrlClr(pub u32);
impl CtrlClr {
    #[doc = "OTG ID Change Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enotg_id_chg_irq(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID Change Interrupt Enable."]
    #[inline(always)]
    pub const fn set_enotg_id_chg_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Host Disconnect Detection Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enhostdiscondetect(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Host Disconnect Detection Enable."]
    #[inline(always)]
    pub const fn set_enhostdiscondetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable Interrupt for Host Disconnect."]
    #[must_use]
    #[inline(always)]
    pub const fn enirqhostdiscon(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Interrupt for Host Disconnect."]
    #[inline(always)]
    pub const fn set_enirqhostdiscon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Host Disconnect Detection Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn hostdiscondetect_irq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Host Disconnect Detection Interrupt."]
    #[inline(always)]
    pub const fn set_hostdiscondetect_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable Nonstandard Resistive Plugged-In Detection."]
    #[must_use]
    #[inline(always)]
    pub const fn endevplugindetect(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Nonstandard Resistive Plugged-In Detection."]
    #[inline(always)]
    pub const fn set_endevplugindetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Device Plug-In Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn devplugin_polarity(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Device Plug-In Polarity."]
    #[inline(always)]
    pub const fn set_devplugin_polarity(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "OTG ID Change Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn otg_id_chg_irq(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID Change Interrupt."]
    #[inline(always)]
    pub const fn set_otg_id_chg_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable Internal OTG ID Detector."]
    #[must_use]
    #[inline(always)]
    pub const fn enotgiddetect(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Internal OTG ID Detector."]
    #[inline(always)]
    pub const fn set_enotgiddetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Resume Interrupt Sticky."]
    #[must_use]
    #[inline(always)]
    pub const fn resumeirqsticky(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Interrupt Sticky."]
    #[inline(always)]
    pub const fn set_resumeirqsticky(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Resume Detection Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enirqresumedetect(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Detection Interrupt Enable."]
    #[inline(always)]
    pub const fn set_enirqresumedetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Resume Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn resume_irq(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Interrupt."]
    #[inline(always)]
    pub const fn set_resume_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enable Interrupt for Nonstandard Resistive Plugged-In Detection."]
    #[must_use]
    #[inline(always)]
    pub const fn enirqdevplugin(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Interrupt for Nonstandard Resistive Plugged-In Detection."]
    #[inline(always)]
    pub const fn set_enirqdevplugin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Device Plug-In Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn devplugin_irq(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Device Plug-In Interrupt."]
    #[inline(always)]
    pub const fn set_devplugin_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "APB Clock Switch Option."]
    #[must_use]
    #[inline(always)]
    pub const fn data_on_lradc(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "APB Clock Switch Option."]
    #[inline(always)]
    pub const fn set_data_on_lradc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "UTMI Level 2 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enutmilevel2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Level 2 Enable."]
    #[inline(always)]
    pub const fn set_enutmilevel2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "UTMI Level 3 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enutmilevel3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Level 3 Enable."]
    #[inline(always)]
    pub const fn set_enutmilevel3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Wake-Up Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enirqwakeup(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-Up Interrupt Enable."]
    #[inline(always)]
    pub const fn set_enirqwakeup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Wake-Up Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup_irq(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-Up Interrupt."]
    #[inline(always)]
    pub const fn set_wakeup_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Autoresume Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn autoresume_en(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Autoresume Enable."]
    #[inline(always)]
    pub const fn set_autoresume_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Autoclear Clock Gate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enautoclr_clkgate(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Autoclear Clock Gate Enable."]
    #[inline(always)]
    pub const fn set_enautoclr_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "PHY PWD Autoclear Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enautoclr_phy_pwd(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "PHY PWD Autoclear Enable."]
    #[inline(always)]
    pub const fn set_enautoclr_phy_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "OTG ID Value."]
    #[must_use]
    #[inline(always)]
    pub const fn otg_id_value(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID Value."]
    #[inline(always)]
    pub const fn set_otg_id_value(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "UTMI Suspend."]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_suspendm(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Suspend."]
    #[inline(always)]
    pub const fn set_utmi_suspendm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "UTMI Clock Gate."]
    #[must_use]
    #[inline(always)]
    pub const fn clkgate(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Clock Gate."]
    #[inline(always)]
    pub const fn set_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Software Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn sftrst(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset."]
    #[inline(always)]
    pub const fn set_sftrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CtrlClr {
    #[inline(always)]
    fn default() -> CtrlClr {
        CtrlClr(0)
    }
}
impl core::fmt::Debug for CtrlClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CtrlClr")
            .field("enotg_id_chg_irq", &self.enotg_id_chg_irq())
            .field("enhostdiscondetect", &self.enhostdiscondetect())
            .field("enirqhostdiscon", &self.enirqhostdiscon())
            .field("hostdiscondetect_irq", &self.hostdiscondetect_irq())
            .field("endevplugindetect", &self.endevplugindetect())
            .field("devplugin_polarity", &self.devplugin_polarity())
            .field("otg_id_chg_irq", &self.otg_id_chg_irq())
            .field("enotgiddetect", &self.enotgiddetect())
            .field("resumeirqsticky", &self.resumeirqsticky())
            .field("enirqresumedetect", &self.enirqresumedetect())
            .field("resume_irq", &self.resume_irq())
            .field("enirqdevplugin", &self.enirqdevplugin())
            .field("devplugin_irq", &self.devplugin_irq())
            .field("data_on_lradc", &self.data_on_lradc())
            .field("enutmilevel2", &self.enutmilevel2())
            .field("enutmilevel3", &self.enutmilevel3())
            .field("enirqwakeup", &self.enirqwakeup())
            .field("wakeup_irq", &self.wakeup_irq())
            .field("autoresume_en", &self.autoresume_en())
            .field("enautoclr_clkgate", &self.enautoclr_clkgate())
            .field("enautoclr_phy_pwd", &self.enautoclr_phy_pwd())
            .field("otg_id_value", &self.otg_id_value())
            .field("utmi_suspendm", &self.utmi_suspendm())
            .field("clkgate", &self.clkgate())
            .field("sftrst", &self.sftrst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CtrlClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CtrlClr {{ enotg_id_chg_irq: {=bool:?}, enhostdiscondetect: {=bool:?}, enirqhostdiscon: {=bool:?}, hostdiscondetect_irq: {=bool:?}, endevplugindetect: {=bool:?}, devplugin_polarity: {=bool:?}, otg_id_chg_irq: {=bool:?}, enotgiddetect: {=bool:?}, resumeirqsticky: {=bool:?}, enirqresumedetect: {=bool:?}, resume_irq: {=bool:?}, enirqdevplugin: {=bool:?}, devplugin_irq: {=bool:?}, data_on_lradc: {=bool:?}, enutmilevel2: {=bool:?}, enutmilevel3: {=bool:?}, enirqwakeup: {=bool:?}, wakeup_irq: {=bool:?}, autoresume_en: {=bool:?}, enautoclr_clkgate: {=bool:?}, enautoclr_phy_pwd: {=bool:?}, otg_id_value: {=bool:?}, utmi_suspendm: {=bool:?}, clkgate: {=bool:?}, sftrst: {=bool:?} }}",
            self.enotg_id_chg_irq(),
            self.enhostdiscondetect(),
            self.enirqhostdiscon(),
            self.hostdiscondetect_irq(),
            self.endevplugindetect(),
            self.devplugin_polarity(),
            self.otg_id_chg_irq(),
            self.enotgiddetect(),
            self.resumeirqsticky(),
            self.enirqresumedetect(),
            self.resume_irq(),
            self.enirqdevplugin(),
            self.devplugin_irq(),
            self.data_on_lradc(),
            self.enutmilevel2(),
            self.enutmilevel3(),
            self.enirqwakeup(),
            self.wakeup_irq(),
            self.autoresume_en(),
            self.enautoclr_clkgate(),
            self.enautoclr_phy_pwd(),
            self.otg_id_value(),
            self.utmi_suspendm(),
            self.clkgate(),
            self.sftrst()
        )
    }
}
#[doc = "General Purpose Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtrlSet(pub u32);
impl CtrlSet {
    #[doc = "OTG ID Change Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enotg_id_chg_irq(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID Change Interrupt Enable."]
    #[inline(always)]
    pub const fn set_enotg_id_chg_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Host Disconnect Detection Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enhostdiscondetect(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Host Disconnect Detection Enable."]
    #[inline(always)]
    pub const fn set_enhostdiscondetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable Interrupt for Host Disconnect."]
    #[must_use]
    #[inline(always)]
    pub const fn enirqhostdiscon(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Interrupt for Host Disconnect."]
    #[inline(always)]
    pub const fn set_enirqhostdiscon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Host Disconnect Detection Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn hostdiscondetect_irq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Host Disconnect Detection Interrupt."]
    #[inline(always)]
    pub const fn set_hostdiscondetect_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable Nonstandard Resistive Plugged-In Detection."]
    #[must_use]
    #[inline(always)]
    pub const fn endevplugindetect(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Nonstandard Resistive Plugged-In Detection."]
    #[inline(always)]
    pub const fn set_endevplugindetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Device Plug-In Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn devplugin_polarity(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Device Plug-In Polarity."]
    #[inline(always)]
    pub const fn set_devplugin_polarity(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "OTG ID Change Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn otg_id_chg_irq(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID Change Interrupt."]
    #[inline(always)]
    pub const fn set_otg_id_chg_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable Internal OTG ID Detector."]
    #[must_use]
    #[inline(always)]
    pub const fn enotgiddetect(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Internal OTG ID Detector."]
    #[inline(always)]
    pub const fn set_enotgiddetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Resume Interrupt Sticky."]
    #[must_use]
    #[inline(always)]
    pub const fn resumeirqsticky(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Interrupt Sticky."]
    #[inline(always)]
    pub const fn set_resumeirqsticky(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Resume Detection Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enirqresumedetect(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Detection Interrupt Enable."]
    #[inline(always)]
    pub const fn set_enirqresumedetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Resume Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn resume_irq(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Interrupt."]
    #[inline(always)]
    pub const fn set_resume_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enable Interrupt for Nonstandard Resistive Plugged-In Detection."]
    #[must_use]
    #[inline(always)]
    pub const fn enirqdevplugin(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Interrupt for Nonstandard Resistive Plugged-In Detection."]
    #[inline(always)]
    pub const fn set_enirqdevplugin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Device Plug-In Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn devplugin_irq(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Device Plug-In Interrupt."]
    #[inline(always)]
    pub const fn set_devplugin_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "APB Clock Switch Option."]
    #[must_use]
    #[inline(always)]
    pub const fn data_on_lradc(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "APB Clock Switch Option."]
    #[inline(always)]
    pub const fn set_data_on_lradc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "UTMI Level 2 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enutmilevel2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Level 2 Enable."]
    #[inline(always)]
    pub const fn set_enutmilevel2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "UTMI Level 3 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enutmilevel3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Level 3 Enable."]
    #[inline(always)]
    pub const fn set_enutmilevel3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Wake-Up Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enirqwakeup(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-Up Interrupt Enable."]
    #[inline(always)]
    pub const fn set_enirqwakeup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Wake-Up Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup_irq(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-Up Interrupt."]
    #[inline(always)]
    pub const fn set_wakeup_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Autoresume Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn autoresume_en(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Autoresume Enable."]
    #[inline(always)]
    pub const fn set_autoresume_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Autoclear Clock Gate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enautoclr_clkgate(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Autoclear Clock Gate Enable."]
    #[inline(always)]
    pub const fn set_enautoclr_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "PHY PWD Autoclear Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enautoclr_phy_pwd(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "PHY PWD Autoclear Enable."]
    #[inline(always)]
    pub const fn set_enautoclr_phy_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "OTG ID Value."]
    #[must_use]
    #[inline(always)]
    pub const fn otg_id_value(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID Value."]
    #[inline(always)]
    pub const fn set_otg_id_value(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "UTMI Suspend."]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_suspendm(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Suspend."]
    #[inline(always)]
    pub const fn set_utmi_suspendm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "UTMI Clock Gate."]
    #[must_use]
    #[inline(always)]
    pub const fn clkgate(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Clock Gate."]
    #[inline(always)]
    pub const fn set_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Software Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn sftrst(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset."]
    #[inline(always)]
    pub const fn set_sftrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CtrlSet {
    #[inline(always)]
    fn default() -> CtrlSet {
        CtrlSet(0)
    }
}
impl core::fmt::Debug for CtrlSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CtrlSet")
            .field("enotg_id_chg_irq", &self.enotg_id_chg_irq())
            .field("enhostdiscondetect", &self.enhostdiscondetect())
            .field("enirqhostdiscon", &self.enirqhostdiscon())
            .field("hostdiscondetect_irq", &self.hostdiscondetect_irq())
            .field("endevplugindetect", &self.endevplugindetect())
            .field("devplugin_polarity", &self.devplugin_polarity())
            .field("otg_id_chg_irq", &self.otg_id_chg_irq())
            .field("enotgiddetect", &self.enotgiddetect())
            .field("resumeirqsticky", &self.resumeirqsticky())
            .field("enirqresumedetect", &self.enirqresumedetect())
            .field("resume_irq", &self.resume_irq())
            .field("enirqdevplugin", &self.enirqdevplugin())
            .field("devplugin_irq", &self.devplugin_irq())
            .field("data_on_lradc", &self.data_on_lradc())
            .field("enutmilevel2", &self.enutmilevel2())
            .field("enutmilevel3", &self.enutmilevel3())
            .field("enirqwakeup", &self.enirqwakeup())
            .field("wakeup_irq", &self.wakeup_irq())
            .field("autoresume_en", &self.autoresume_en())
            .field("enautoclr_clkgate", &self.enautoclr_clkgate())
            .field("enautoclr_phy_pwd", &self.enautoclr_phy_pwd())
            .field("otg_id_value", &self.otg_id_value())
            .field("utmi_suspendm", &self.utmi_suspendm())
            .field("clkgate", &self.clkgate())
            .field("sftrst", &self.sftrst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CtrlSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CtrlSet {{ enotg_id_chg_irq: {=bool:?}, enhostdiscondetect: {=bool:?}, enirqhostdiscon: {=bool:?}, hostdiscondetect_irq: {=bool:?}, endevplugindetect: {=bool:?}, devplugin_polarity: {=bool:?}, otg_id_chg_irq: {=bool:?}, enotgiddetect: {=bool:?}, resumeirqsticky: {=bool:?}, enirqresumedetect: {=bool:?}, resume_irq: {=bool:?}, enirqdevplugin: {=bool:?}, devplugin_irq: {=bool:?}, data_on_lradc: {=bool:?}, enutmilevel2: {=bool:?}, enutmilevel3: {=bool:?}, enirqwakeup: {=bool:?}, wakeup_irq: {=bool:?}, autoresume_en: {=bool:?}, enautoclr_clkgate: {=bool:?}, enautoclr_phy_pwd: {=bool:?}, otg_id_value: {=bool:?}, utmi_suspendm: {=bool:?}, clkgate: {=bool:?}, sftrst: {=bool:?} }}",
            self.enotg_id_chg_irq(),
            self.enhostdiscondetect(),
            self.enirqhostdiscon(),
            self.hostdiscondetect_irq(),
            self.endevplugindetect(),
            self.devplugin_polarity(),
            self.otg_id_chg_irq(),
            self.enotgiddetect(),
            self.resumeirqsticky(),
            self.enirqresumedetect(),
            self.resume_irq(),
            self.enirqdevplugin(),
            self.devplugin_irq(),
            self.data_on_lradc(),
            self.enutmilevel2(),
            self.enutmilevel3(),
            self.enirqwakeup(),
            self.wakeup_irq(),
            self.autoresume_en(),
            self.enautoclr_clkgate(),
            self.enautoclr_phy_pwd(),
            self.otg_id_value(),
            self.utmi_suspendm(),
            self.clkgate(),
            self.sftrst()
        )
    }
}
#[doc = "General Purpose Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtrlTog(pub u32);
impl CtrlTog {
    #[doc = "OTG ID Change Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enotg_id_chg_irq(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID Change Interrupt Enable."]
    #[inline(always)]
    pub const fn set_enotg_id_chg_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Host Disconnect Detection Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enhostdiscondetect(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Host Disconnect Detection Enable."]
    #[inline(always)]
    pub const fn set_enhostdiscondetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable Interrupt for Host Disconnect."]
    #[must_use]
    #[inline(always)]
    pub const fn enirqhostdiscon(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Interrupt for Host Disconnect."]
    #[inline(always)]
    pub const fn set_enirqhostdiscon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Host Disconnect Detection Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn hostdiscondetect_irq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Host Disconnect Detection Interrupt."]
    #[inline(always)]
    pub const fn set_hostdiscondetect_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable Nonstandard Resistive Plugged-In Detection."]
    #[must_use]
    #[inline(always)]
    pub const fn endevplugindetect(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Nonstandard Resistive Plugged-In Detection."]
    #[inline(always)]
    pub const fn set_endevplugindetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Device Plug-In Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn devplugin_polarity(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Device Plug-In Polarity."]
    #[inline(always)]
    pub const fn set_devplugin_polarity(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "OTG ID Change Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn otg_id_chg_irq(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID Change Interrupt."]
    #[inline(always)]
    pub const fn set_otg_id_chg_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable Internal OTG ID Detector."]
    #[must_use]
    #[inline(always)]
    pub const fn enotgiddetect(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Internal OTG ID Detector."]
    #[inline(always)]
    pub const fn set_enotgiddetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Resume Interrupt Sticky."]
    #[must_use]
    #[inline(always)]
    pub const fn resumeirqsticky(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Interrupt Sticky."]
    #[inline(always)]
    pub const fn set_resumeirqsticky(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Resume Detection Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enirqresumedetect(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Detection Interrupt Enable."]
    #[inline(always)]
    pub const fn set_enirqresumedetect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Resume Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn resume_irq(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Interrupt."]
    #[inline(always)]
    pub const fn set_resume_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enable Interrupt for Nonstandard Resistive Plugged-In Detection."]
    #[must_use]
    #[inline(always)]
    pub const fn enirqdevplugin(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Interrupt for Nonstandard Resistive Plugged-In Detection."]
    #[inline(always)]
    pub const fn set_enirqdevplugin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Device Plug-In Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn devplugin_irq(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Device Plug-In Interrupt."]
    #[inline(always)]
    pub const fn set_devplugin_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "APB Clock Switch Option."]
    #[must_use]
    #[inline(always)]
    pub const fn data_on_lradc(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "APB Clock Switch Option."]
    #[inline(always)]
    pub const fn set_data_on_lradc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "UTMI Level 2 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enutmilevel2(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Level 2 Enable."]
    #[inline(always)]
    pub const fn set_enutmilevel2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "UTMI Level 3 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enutmilevel3(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Level 3 Enable."]
    #[inline(always)]
    pub const fn set_enutmilevel3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Wake-Up Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enirqwakeup(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-Up Interrupt Enable."]
    #[inline(always)]
    pub const fn set_enirqwakeup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Wake-Up Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup_irq(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-Up Interrupt."]
    #[inline(always)]
    pub const fn set_wakeup_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Autoresume Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn autoresume_en(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Autoresume Enable."]
    #[inline(always)]
    pub const fn set_autoresume_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Autoclear Clock Gate Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enautoclr_clkgate(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Autoclear Clock Gate Enable."]
    #[inline(always)]
    pub const fn set_enautoclr_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "PHY PWD Autoclear Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn enautoclr_phy_pwd(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "PHY PWD Autoclear Enable."]
    #[inline(always)]
    pub const fn set_enautoclr_phy_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "OTG ID Value."]
    #[must_use]
    #[inline(always)]
    pub const fn otg_id_value(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID Value."]
    #[inline(always)]
    pub const fn set_otg_id_value(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "UTMI Suspend."]
    #[must_use]
    #[inline(always)]
    pub const fn utmi_suspendm(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Suspend."]
    #[inline(always)]
    pub const fn set_utmi_suspendm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "UTMI Clock Gate."]
    #[must_use]
    #[inline(always)]
    pub const fn clkgate(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "UTMI Clock Gate."]
    #[inline(always)]
    pub const fn set_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Software Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn sftrst(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset."]
    #[inline(always)]
    pub const fn set_sftrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CtrlTog {
    #[inline(always)]
    fn default() -> CtrlTog {
        CtrlTog(0)
    }
}
impl core::fmt::Debug for CtrlTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CtrlTog")
            .field("enotg_id_chg_irq", &self.enotg_id_chg_irq())
            .field("enhostdiscondetect", &self.enhostdiscondetect())
            .field("enirqhostdiscon", &self.enirqhostdiscon())
            .field("hostdiscondetect_irq", &self.hostdiscondetect_irq())
            .field("endevplugindetect", &self.endevplugindetect())
            .field("devplugin_polarity", &self.devplugin_polarity())
            .field("otg_id_chg_irq", &self.otg_id_chg_irq())
            .field("enotgiddetect", &self.enotgiddetect())
            .field("resumeirqsticky", &self.resumeirqsticky())
            .field("enirqresumedetect", &self.enirqresumedetect())
            .field("resume_irq", &self.resume_irq())
            .field("enirqdevplugin", &self.enirqdevplugin())
            .field("devplugin_irq", &self.devplugin_irq())
            .field("data_on_lradc", &self.data_on_lradc())
            .field("enutmilevel2", &self.enutmilevel2())
            .field("enutmilevel3", &self.enutmilevel3())
            .field("enirqwakeup", &self.enirqwakeup())
            .field("wakeup_irq", &self.wakeup_irq())
            .field("autoresume_en", &self.autoresume_en())
            .field("enautoclr_clkgate", &self.enautoclr_clkgate())
            .field("enautoclr_phy_pwd", &self.enautoclr_phy_pwd())
            .field("otg_id_value", &self.otg_id_value())
            .field("utmi_suspendm", &self.utmi_suspendm())
            .field("clkgate", &self.clkgate())
            .field("sftrst", &self.sftrst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CtrlTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CtrlTog {{ enotg_id_chg_irq: {=bool:?}, enhostdiscondetect: {=bool:?}, enirqhostdiscon: {=bool:?}, hostdiscondetect_irq: {=bool:?}, endevplugindetect: {=bool:?}, devplugin_polarity: {=bool:?}, otg_id_chg_irq: {=bool:?}, enotgiddetect: {=bool:?}, resumeirqsticky: {=bool:?}, enirqresumedetect: {=bool:?}, resume_irq: {=bool:?}, enirqdevplugin: {=bool:?}, devplugin_irq: {=bool:?}, data_on_lradc: {=bool:?}, enutmilevel2: {=bool:?}, enutmilevel3: {=bool:?}, enirqwakeup: {=bool:?}, wakeup_irq: {=bool:?}, autoresume_en: {=bool:?}, enautoclr_clkgate: {=bool:?}, enautoclr_phy_pwd: {=bool:?}, otg_id_value: {=bool:?}, utmi_suspendm: {=bool:?}, clkgate: {=bool:?}, sftrst: {=bool:?} }}",
            self.enotg_id_chg_irq(),
            self.enhostdiscondetect(),
            self.enirqhostdiscon(),
            self.hostdiscondetect_irq(),
            self.endevplugindetect(),
            self.devplugin_polarity(),
            self.otg_id_chg_irq(),
            self.enotgiddetect(),
            self.resumeirqsticky(),
            self.enirqresumedetect(),
            self.resume_irq(),
            self.enirqdevplugin(),
            self.devplugin_irq(),
            self.data_on_lradc(),
            self.enutmilevel2(),
            self.enutmilevel3(),
            self.enirqwakeup(),
            self.wakeup_irq(),
            self.autoresume_en(),
            self.enautoclr_clkgate(),
            self.enautoclr_phy_pwd(),
            self.otg_id_value(),
            self.utmi_suspendm(),
            self.clkgate(),
            self.sftrst()
        )
    }
}
#[doc = "Debug 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug0(pub u32);
impl Debug0 {
    #[doc = "Hold OTG_ID."]
    #[must_use]
    #[inline(always)]
    pub const fn otgidpiolock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Hold OTG_ID."]
    #[inline(always)]
    pub const fn set_otgidpiolock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Host Pulldown Overdrive Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn hstpulldown(&self) -> Hstpulldown {
        let val = (self.0 >> 2usize) & 0x03;
        Hstpulldown::from_bits(val as u8)
    }
    #[doc = "Host Pulldown Overdrive Mode."]
    #[inline(always)]
    pub const fn set_hstpulldown(&mut self, val: Hstpulldown) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Enable Host Pulldown Overdrive Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn enhstpulldown(&self) -> Enhstpulldown {
        let val = (self.0 >> 4usize) & 0x03;
        Enhstpulldown::from_bits(val as u8)
    }
    #[doc = "Enable Host Pulldown Overdrive Mode."]
    #[inline(always)]
    pub const fn set_enhstpulldown(&mut self, val: Enhstpulldown) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
}
impl Default for Debug0 {
    #[inline(always)]
    fn default() -> Debug0 {
        Debug0(0)
    }
}
impl core::fmt::Debug for Debug0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Debug0")
            .field("otgidpiolock", &self.otgidpiolock())
            .field("hstpulldown", &self.hstpulldown())
            .field("enhstpulldown", &self.enhstpulldown())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Debug0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Debug0 {{ otgidpiolock: {=bool:?}, hstpulldown: {:?}, enhstpulldown: {:?} }}",
            self.otgidpiolock(),
            self.hstpulldown(),
            self.enhstpulldown()
        )
    }
}
#[doc = "Debug 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug0Clr(pub u32);
impl Debug0Clr {
    #[doc = "Hold OTG_ID."]
    #[must_use]
    #[inline(always)]
    pub const fn otgidpiolock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Hold OTG_ID."]
    #[inline(always)]
    pub const fn set_otgidpiolock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Host Pulldown Overdrive Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn hstpulldown(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Host Pulldown Overdrive Mode."]
    #[inline(always)]
    pub const fn set_hstpulldown(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Enable Host Pulldown Overdrive Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn enhstpulldown(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Enable Host Pulldown Overdrive Mode."]
    #[inline(always)]
    pub const fn set_enhstpulldown(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
}
impl Default for Debug0Clr {
    #[inline(always)]
    fn default() -> Debug0Clr {
        Debug0Clr(0)
    }
}
impl core::fmt::Debug for Debug0Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Debug0Clr")
            .field("otgidpiolock", &self.otgidpiolock())
            .field("hstpulldown", &self.hstpulldown())
            .field("enhstpulldown", &self.enhstpulldown())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Debug0Clr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Debug0Clr {{ otgidpiolock: {=bool:?}, hstpulldown: {=u8:?}, enhstpulldown: {=u8:?} }}",
            self.otgidpiolock(),
            self.hstpulldown(),
            self.enhstpulldown()
        )
    }
}
#[doc = "Debug 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug0Set(pub u32);
impl Debug0Set {
    #[doc = "Hold OTG_ID."]
    #[must_use]
    #[inline(always)]
    pub const fn otgidpiolock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Hold OTG_ID."]
    #[inline(always)]
    pub const fn set_otgidpiolock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Host Pulldown Overdrive Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn hstpulldown(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Host Pulldown Overdrive Mode."]
    #[inline(always)]
    pub const fn set_hstpulldown(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Enable Host Pulldown Overdrive Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn enhstpulldown(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Enable Host Pulldown Overdrive Mode."]
    #[inline(always)]
    pub const fn set_enhstpulldown(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
}
impl Default for Debug0Set {
    #[inline(always)]
    fn default() -> Debug0Set {
        Debug0Set(0)
    }
}
impl core::fmt::Debug for Debug0Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Debug0Set")
            .field("otgidpiolock", &self.otgidpiolock())
            .field("hstpulldown", &self.hstpulldown())
            .field("enhstpulldown", &self.enhstpulldown())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Debug0Set {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Debug0Set {{ otgidpiolock: {=bool:?}, hstpulldown: {=u8:?}, enhstpulldown: {=u8:?} }}",
            self.otgidpiolock(),
            self.hstpulldown(),
            self.enhstpulldown()
        )
    }
}
#[doc = "Debug 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debug0Tog(pub u32);
impl Debug0Tog {
    #[doc = "Hold OTG_ID."]
    #[must_use]
    #[inline(always)]
    pub const fn otgidpiolock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Hold OTG_ID."]
    #[inline(always)]
    pub const fn set_otgidpiolock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Host Pulldown Overdrive Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn hstpulldown(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Host Pulldown Overdrive Mode."]
    #[inline(always)]
    pub const fn set_hstpulldown(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Enable Host Pulldown Overdrive Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn enhstpulldown(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Enable Host Pulldown Overdrive Mode."]
    #[inline(always)]
    pub const fn set_enhstpulldown(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
}
impl Default for Debug0Tog {
    #[inline(always)]
    fn default() -> Debug0Tog {
        Debug0Tog(0)
    }
}
impl core::fmt::Debug for Debug0Tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Debug0Tog")
            .field("otgidpiolock", &self.otgidpiolock())
            .field("hstpulldown", &self.hstpulldown())
            .field("enhstpulldown", &self.enhstpulldown())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Debug0Tog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Debug0Tog {{ otgidpiolock: {=bool:?}, hstpulldown: {=u8:?}, enhstpulldown: {=u8:?} }}",
            self.otgidpiolock(),
            self.hstpulldown(),
            self.enhstpulldown()
        )
    }
}
#[doc = "IP Block."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ip(pub u32);
impl Ip {
    #[doc = "Power Control Suspend Option."]
    #[must_use]
    #[inline(always)]
    pub const fn power_control_suspend_option(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Power Control Suspend Option."]
    #[inline(always)]
    pub const fn set_power_control_suspend_option(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Ip {
    #[inline(always)]
    fn default() -> Ip {
        Ip(0)
    }
}
impl core::fmt::Debug for Ip {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ip")
            .field(
                "power_control_suspend_option",
                &self.power_control_suspend_option(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ip {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ip {{ power_control_suspend_option: {=bool:?} }}",
            self.power_control_suspend_option()
        )
    }
}
#[doc = "IP Block."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IpClr(pub u32);
impl IpClr {
    #[doc = "Power Control Suspend Option."]
    #[must_use]
    #[inline(always)]
    pub const fn power_control_suspend_option(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Power Control Suspend Option."]
    #[inline(always)]
    pub const fn set_power_control_suspend_option(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for IpClr {
    #[inline(always)]
    fn default() -> IpClr {
        IpClr(0)
    }
}
impl core::fmt::Debug for IpClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IpClr")
            .field(
                "power_control_suspend_option",
                &self.power_control_suspend_option(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IpClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IpClr {{ power_control_suspend_option: {=bool:?} }}",
            self.power_control_suspend_option()
        )
    }
}
#[doc = "IP Block."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IpSet(pub u32);
impl IpSet {
    #[doc = "Power Control Suspend Option."]
    #[must_use]
    #[inline(always)]
    pub const fn power_control_suspend_option(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Power Control Suspend Option."]
    #[inline(always)]
    pub const fn set_power_control_suspend_option(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for IpSet {
    #[inline(always)]
    fn default() -> IpSet {
        IpSet(0)
    }
}
impl core::fmt::Debug for IpSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IpSet")
            .field(
                "power_control_suspend_option",
                &self.power_control_suspend_option(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IpSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IpSet {{ power_control_suspend_option: {=bool:?} }}",
            self.power_control_suspend_option()
        )
    }
}
#[doc = "IP Block."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IpTog(pub u32);
impl IpTog {
    #[doc = "Power Control Suspend Option."]
    #[must_use]
    #[inline(always)]
    pub const fn power_control_suspend_option(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Power Control Suspend Option."]
    #[inline(always)]
    pub const fn set_power_control_suspend_option(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for IpTog {
    #[inline(always)]
    fn default() -> IpTog {
        IpTog(0)
    }
}
impl core::fmt::Debug for IpTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IpTog")
            .field(
                "power_control_suspend_option",
                &self.power_control_suspend_option(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IpTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IpTog {{ power_control_suspend_option: {=bool:?} }}",
            self.power_control_suspend_option()
        )
    }
}
#[doc = "PFD A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pfda(pub u32);
impl Pfda {
    #[doc = "PFD0 Clock Gate."]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_clkgate(&self) -> Pfd0Clkgate {
        let val = (self.0 >> 0usize) & 0x01;
        Pfd0Clkgate::from_bits(val as u8)
    }
    #[doc = "PFD0 Clock Gate."]
    #[inline(always)]
    pub const fn set_pfd0_clkgate(&mut self, val: Pfd0Clkgate) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "PFD0 Fractional Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_frac(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x3f;
        val as u8
    }
    #[doc = "PFD0 Fractional Divider."]
    #[inline(always)]
    pub const fn set_pfd0_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 1usize)) | (((val as u32) & 0x3f) << 1usize);
    }
    #[doc = "PFD0 Stable Signal."]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_stable(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "PFD0 Stable Signal."]
    #[inline(always)]
    pub const fn set_pfd0_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Pfda {
    #[inline(always)]
    fn default() -> Pfda {
        Pfda(0)
    }
}
impl core::fmt::Debug for Pfda {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pfda")
            .field("pfd0_clkgate", &self.pfd0_clkgate())
            .field("pfd0_frac", &self.pfd0_frac())
            .field("pfd0_stable", &self.pfd0_stable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pfda {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pfda {{ pfd0_clkgate: {:?}, pfd0_frac: {=u8:?}, pfd0_stable: {=bool:?} }}",
            self.pfd0_clkgate(),
            self.pfd0_frac(),
            self.pfd0_stable()
        )
    }
}
#[doc = "PFD A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PfdaClr(pub u32);
impl PfdaClr {
    #[doc = "PFD0 Clock Gate."]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_clkgate(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "PFD0 Clock Gate."]
    #[inline(always)]
    pub const fn set_pfd0_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "PFD0 Fractional Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_frac(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x3f;
        val as u8
    }
    #[doc = "PFD0 Fractional Divider."]
    #[inline(always)]
    pub const fn set_pfd0_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 1usize)) | (((val as u32) & 0x3f) << 1usize);
    }
    #[doc = "PFD0 Stable Signal."]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_stable(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "PFD0 Stable Signal."]
    #[inline(always)]
    pub const fn set_pfd0_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for PfdaClr {
    #[inline(always)]
    fn default() -> PfdaClr {
        PfdaClr(0)
    }
}
impl core::fmt::Debug for PfdaClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PfdaClr")
            .field("pfd0_clkgate", &self.pfd0_clkgate())
            .field("pfd0_frac", &self.pfd0_frac())
            .field("pfd0_stable", &self.pfd0_stable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PfdaClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PfdaClr {{ pfd0_clkgate: {=bool:?}, pfd0_frac: {=u8:?}, pfd0_stable: {=bool:?} }}",
            self.pfd0_clkgate(),
            self.pfd0_frac(),
            self.pfd0_stable()
        )
    }
}
#[doc = "PFD A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PfdaSet(pub u32);
impl PfdaSet {
    #[doc = "PFD0 Clock Gate."]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_clkgate(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "PFD0 Clock Gate."]
    #[inline(always)]
    pub const fn set_pfd0_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "PFD0 Fractional Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_frac(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x3f;
        val as u8
    }
    #[doc = "PFD0 Fractional Divider."]
    #[inline(always)]
    pub const fn set_pfd0_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 1usize)) | (((val as u32) & 0x3f) << 1usize);
    }
    #[doc = "PFD0 Stable Signal."]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_stable(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "PFD0 Stable Signal."]
    #[inline(always)]
    pub const fn set_pfd0_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for PfdaSet {
    #[inline(always)]
    fn default() -> PfdaSet {
        PfdaSet(0)
    }
}
impl core::fmt::Debug for PfdaSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PfdaSet")
            .field("pfd0_clkgate", &self.pfd0_clkgate())
            .field("pfd0_frac", &self.pfd0_frac())
            .field("pfd0_stable", &self.pfd0_stable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PfdaSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PfdaSet {{ pfd0_clkgate: {=bool:?}, pfd0_frac: {=u8:?}, pfd0_stable: {=bool:?} }}",
            self.pfd0_clkgate(),
            self.pfd0_frac(),
            self.pfd0_stable()
        )
    }
}
#[doc = "PFD A."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PfdaTog(pub u32);
impl PfdaTog {
    #[doc = "PFD0 Clock Gate."]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_clkgate(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "PFD0 Clock Gate."]
    #[inline(always)]
    pub const fn set_pfd0_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "PFD0 Fractional Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_frac(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x3f;
        val as u8
    }
    #[doc = "PFD0 Fractional Divider."]
    #[inline(always)]
    pub const fn set_pfd0_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 1usize)) | (((val as u32) & 0x3f) << 1usize);
    }
    #[doc = "PFD0 Stable Signal."]
    #[must_use]
    #[inline(always)]
    pub const fn pfd0_stable(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "PFD0 Stable Signal."]
    #[inline(always)]
    pub const fn set_pfd0_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for PfdaTog {
    #[inline(always)]
    fn default() -> PfdaTog {
        PfdaTog(0)
    }
}
impl core::fmt::Debug for PfdaTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PfdaTog")
            .field("pfd0_clkgate", &self.pfd0_clkgate())
            .field("pfd0_frac", &self.pfd0_frac())
            .field("pfd0_stable", &self.pfd0_stable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PfdaTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PfdaTog {{ pfd0_clkgate: {=bool:?}, pfd0_frac: {=u8:?}, pfd0_stable: {=bool:?} }}",
            self.pfd0_clkgate(),
            self.pfd0_frac(),
            self.pfd0_stable()
        )
    }
}
#[doc = "PLL SIC."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllSic(pub u32);
impl PllSic {
    #[doc = "Miscellaneous Control."]
    #[must_use]
    #[inline(always)]
    pub const fn misc2_control0(&self) -> Misc2Control0 {
        let val = (self.0 >> 5usize) & 0x01;
        Misc2Control0::from_bits(val as u8)
    }
    #[doc = "Miscellaneous Control."]
    #[inline(always)]
    pub const fn set_misc2_control0(&mut self, val: Misc2Control0) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "PLL Multi-Phase Clock Outputs Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_en_usb_clks(&self) -> PllEnUsbClks {
        let val = (self.0 >> 6usize) & 0x01;
        PllEnUsbClks::from_bits(val as u8)
    }
    #[doc = "PLL Multi-Phase Clock Outputs Enable."]
    #[inline(always)]
    pub const fn set_pll_en_usb_clks(&mut self, val: PllEnUsbClks) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "USB PLL Powerup Control."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_power(&self) -> PllPower {
        let val = (self.0 >> 12usize) & 0x01;
        PllPower::from_bits(val as u8)
    }
    #[doc = "USB PLL Powerup Control."]
    #[inline(always)]
    pub const fn set_pll_power(&mut self, val: PllPower) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "PLL Output Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_enable(&self) -> PllEnable {
        let val = (self.0 >> 13usize) & 0x01;
        PllEnable::from_bits(val as u8)
    }
    #[doc = "PLL Output Clock Enable."]
    #[inline(always)]
    pub const fn set_pll_enable(&mut self, val: PllEnable) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Bypass USB PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_bypass(&self) -> PllBypass {
        let val = (self.0 >> 16usize) & 0x01;
        PllBypass::from_bits(val as u8)
    }
    #[doc = "Bypass USB PLL."]
    #[inline(always)]
    pub const fn set_pll_bypass(&mut self, val: PllBypass) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Reference Bias Power Control."]
    #[must_use]
    #[inline(always)]
    pub const fn refbias_pwd_sel(&self) -> RefbiasPwdSel {
        let val = (self.0 >> 19usize) & 0x01;
        RefbiasPwdSel::from_bits(val as u8)
    }
    #[doc = "Reference Bias Power Control."]
    #[inline(always)]
    pub const fn set_refbias_pwd_sel(&mut self, val: RefbiasPwdSel) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Power Down Reference Bias."]
    #[must_use]
    #[inline(always)]
    pub const fn refbias_pwd(&self) -> RefbiasPwd {
        let val = (self.0 >> 20usize) & 0x01;
        RefbiasPwd::from_bits(val as u8)
    }
    #[doc = "Power Down Reference Bias."]
    #[inline(always)]
    pub const fn set_refbias_pwd(&mut self, val: RefbiasPwd) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Enable PLL Regulator."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_reg_enable(&self) -> PllRegEnable {
        let val = (self.0 >> 21usize) & 0x01;
        PllRegEnable::from_bits(val as u8)
    }
    #[doc = "Enable PLL Regulator."]
    #[inline(always)]
    pub const fn set_pll_reg_enable(&mut self, val: PllRegEnable) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "PLL Divider Value Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_div_sel(&self) -> PllDivSel {
        let val = (self.0 >> 22usize) & 0x07;
        PllDivSel::from_bits(val as u8)
    }
    #[doc = "PLL Divider Value Configuration."]
    #[inline(always)]
    pub const fn set_pll_div_sel(&mut self, val: PllDivSel) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val.to_bits() as u32) & 0x07) << 22usize);
    }
    #[doc = "USB PLL Lock Status Indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_lock(&self) -> PllLock {
        let val = (self.0 >> 31usize) & 0x01;
        PllLock::from_bits(val as u8)
    }
    #[doc = "USB PLL Lock Status Indicator."]
    #[inline(always)]
    pub const fn set_pll_lock(&mut self, val: PllLock) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PllSic {
    #[inline(always)]
    fn default() -> PllSic {
        PllSic(0)
    }
}
impl core::fmt::Debug for PllSic {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllSic")
            .field("misc2_control0", &self.misc2_control0())
            .field("pll_en_usb_clks", &self.pll_en_usb_clks())
            .field("pll_power", &self.pll_power())
            .field("pll_enable", &self.pll_enable())
            .field("pll_bypass", &self.pll_bypass())
            .field("refbias_pwd_sel", &self.refbias_pwd_sel())
            .field("refbias_pwd", &self.refbias_pwd())
            .field("pll_reg_enable", &self.pll_reg_enable())
            .field("pll_div_sel", &self.pll_div_sel())
            .field("pll_lock", &self.pll_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllSic {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllSic {{ misc2_control0: {:?}, pll_en_usb_clks: {:?}, pll_power: {:?}, pll_enable: {:?}, pll_bypass: {:?}, refbias_pwd_sel: {:?}, refbias_pwd: {:?}, pll_reg_enable: {:?}, pll_div_sel: {:?}, pll_lock: {:?} }}",
            self.misc2_control0(),
            self.pll_en_usb_clks(),
            self.pll_power(),
            self.pll_enable(),
            self.pll_bypass(),
            self.refbias_pwd_sel(),
            self.refbias_pwd(),
            self.pll_reg_enable(),
            self.pll_div_sel(),
            self.pll_lock()
        )
    }
}
#[doc = "PLL SIC."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllSicClr(pub u32);
impl PllSicClr {
    #[doc = "Miscellaneous Control."]
    #[must_use]
    #[inline(always)]
    pub const fn misc2_control0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Miscellaneous Control."]
    #[inline(always)]
    pub const fn set_misc2_control0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "PLL Multi-Phase Clock Outputs Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_en_usb_clks(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "PLL Multi-Phase Clock Outputs Enable."]
    #[inline(always)]
    pub const fn set_pll_en_usb_clks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "USB PLL Powerup Control."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_power(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "USB PLL Powerup Control."]
    #[inline(always)]
    pub const fn set_pll_power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "PLL Output Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_enable(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PLL Output Clock Enable."]
    #[inline(always)]
    pub const fn set_pll_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Bypass USB PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass USB PLL."]
    #[inline(always)]
    pub const fn set_pll_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Reference Bias Power Control."]
    #[must_use]
    #[inline(always)]
    pub const fn refbias_pwd_sel(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Reference Bias Power Control."]
    #[inline(always)]
    pub const fn set_refbias_pwd_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Power Down Reference Bias."]
    #[must_use]
    #[inline(always)]
    pub const fn refbias_pwd(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down Reference Bias."]
    #[inline(always)]
    pub const fn set_refbias_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enable PLL Regulator."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_reg_enable(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enable PLL Regulator."]
    #[inline(always)]
    pub const fn set_pll_reg_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "PLL Divider Value Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_div_sel(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x07;
        val as u8
    }
    #[doc = "PLL Divider Value Configuration."]
    #[inline(always)]
    pub const fn set_pll_div_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val as u32) & 0x07) << 22usize);
    }
    #[doc = "USB PLL Lock Status Indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "USB PLL Lock Status Indicator."]
    #[inline(always)]
    pub const fn set_pll_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PllSicClr {
    #[inline(always)]
    fn default() -> PllSicClr {
        PllSicClr(0)
    }
}
impl core::fmt::Debug for PllSicClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllSicClr")
            .field("misc2_control0", &self.misc2_control0())
            .field("pll_en_usb_clks", &self.pll_en_usb_clks())
            .field("pll_power", &self.pll_power())
            .field("pll_enable", &self.pll_enable())
            .field("pll_bypass", &self.pll_bypass())
            .field("refbias_pwd_sel", &self.refbias_pwd_sel())
            .field("refbias_pwd", &self.refbias_pwd())
            .field("pll_reg_enable", &self.pll_reg_enable())
            .field("pll_div_sel", &self.pll_div_sel())
            .field("pll_lock", &self.pll_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllSicClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllSicClr {{ misc2_control0: {=bool:?}, pll_en_usb_clks: {=bool:?}, pll_power: {=bool:?}, pll_enable: {=bool:?}, pll_bypass: {=bool:?}, refbias_pwd_sel: {=bool:?}, refbias_pwd: {=bool:?}, pll_reg_enable: {=bool:?}, pll_div_sel: {=u8:?}, pll_lock: {=bool:?} }}",
            self.misc2_control0(),
            self.pll_en_usb_clks(),
            self.pll_power(),
            self.pll_enable(),
            self.pll_bypass(),
            self.refbias_pwd_sel(),
            self.refbias_pwd(),
            self.pll_reg_enable(),
            self.pll_div_sel(),
            self.pll_lock()
        )
    }
}
#[doc = "PLL SIC."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllSicSet(pub u32);
impl PllSicSet {
    #[doc = "Miscellaneous Control."]
    #[must_use]
    #[inline(always)]
    pub const fn misc2_control0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Miscellaneous Control."]
    #[inline(always)]
    pub const fn set_misc2_control0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "PLL Multi-Phase Clock Outputs Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_en_usb_clks(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "PLL Multi-Phase Clock Outputs Enable."]
    #[inline(always)]
    pub const fn set_pll_en_usb_clks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "USB PLL Powerup Control."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_power(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "USB PLL Powerup Control."]
    #[inline(always)]
    pub const fn set_pll_power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "PLL Output Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_enable(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PLL Output Clock Enable."]
    #[inline(always)]
    pub const fn set_pll_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Bypass USB PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass USB PLL."]
    #[inline(always)]
    pub const fn set_pll_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Reference Bias Power Control."]
    #[must_use]
    #[inline(always)]
    pub const fn refbias_pwd_sel(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Reference Bias Power Control."]
    #[inline(always)]
    pub const fn set_refbias_pwd_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Power Down Reference Bias."]
    #[must_use]
    #[inline(always)]
    pub const fn refbias_pwd(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down Reference Bias."]
    #[inline(always)]
    pub const fn set_refbias_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enable PLL Regulator."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_reg_enable(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enable PLL Regulator."]
    #[inline(always)]
    pub const fn set_pll_reg_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "PLL Divider Value Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_div_sel(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x07;
        val as u8
    }
    #[doc = "PLL Divider Value Configuration."]
    #[inline(always)]
    pub const fn set_pll_div_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val as u32) & 0x07) << 22usize);
    }
    #[doc = "USB PLL Lock Status Indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "USB PLL Lock Status Indicator."]
    #[inline(always)]
    pub const fn set_pll_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PllSicSet {
    #[inline(always)]
    fn default() -> PllSicSet {
        PllSicSet(0)
    }
}
impl core::fmt::Debug for PllSicSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllSicSet")
            .field("misc2_control0", &self.misc2_control0())
            .field("pll_en_usb_clks", &self.pll_en_usb_clks())
            .field("pll_power", &self.pll_power())
            .field("pll_enable", &self.pll_enable())
            .field("pll_bypass", &self.pll_bypass())
            .field("refbias_pwd_sel", &self.refbias_pwd_sel())
            .field("refbias_pwd", &self.refbias_pwd())
            .field("pll_reg_enable", &self.pll_reg_enable())
            .field("pll_div_sel", &self.pll_div_sel())
            .field("pll_lock", &self.pll_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllSicSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllSicSet {{ misc2_control0: {=bool:?}, pll_en_usb_clks: {=bool:?}, pll_power: {=bool:?}, pll_enable: {=bool:?}, pll_bypass: {=bool:?}, refbias_pwd_sel: {=bool:?}, refbias_pwd: {=bool:?}, pll_reg_enable: {=bool:?}, pll_div_sel: {=u8:?}, pll_lock: {=bool:?} }}",
            self.misc2_control0(),
            self.pll_en_usb_clks(),
            self.pll_power(),
            self.pll_enable(),
            self.pll_bypass(),
            self.refbias_pwd_sel(),
            self.refbias_pwd(),
            self.pll_reg_enable(),
            self.pll_div_sel(),
            self.pll_lock()
        )
    }
}
#[doc = "PLL SIC."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllSicTog(pub u32);
impl PllSicTog {
    #[doc = "Miscellaneous Control."]
    #[must_use]
    #[inline(always)]
    pub const fn misc2_control0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Miscellaneous Control."]
    #[inline(always)]
    pub const fn set_misc2_control0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "PLL Multi-Phase Clock Outputs Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_en_usb_clks(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "PLL Multi-Phase Clock Outputs Enable."]
    #[inline(always)]
    pub const fn set_pll_en_usb_clks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "USB PLL Powerup Control."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_power(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "USB PLL Powerup Control."]
    #[inline(always)]
    pub const fn set_pll_power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "PLL Output Clock Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_enable(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PLL Output Clock Enable."]
    #[inline(always)]
    pub const fn set_pll_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Bypass USB PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass USB PLL."]
    #[inline(always)]
    pub const fn set_pll_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Reference Bias Power Control."]
    #[must_use]
    #[inline(always)]
    pub const fn refbias_pwd_sel(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Reference Bias Power Control."]
    #[inline(always)]
    pub const fn set_refbias_pwd_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Power Down Reference Bias."]
    #[must_use]
    #[inline(always)]
    pub const fn refbias_pwd(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down Reference Bias."]
    #[inline(always)]
    pub const fn set_refbias_pwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enable PLL Regulator."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_reg_enable(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enable PLL Regulator."]
    #[inline(always)]
    pub const fn set_pll_reg_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "PLL Divider Value Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_div_sel(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x07;
        val as u8
    }
    #[doc = "PLL Divider Value Configuration."]
    #[inline(always)]
    pub const fn set_pll_div_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val as u32) & 0x07) << 22usize);
    }
    #[doc = "USB PLL Lock Status Indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "USB PLL Lock Status Indicator."]
    #[inline(always)]
    pub const fn set_pll_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PllSicTog {
    #[inline(always)]
    fn default() -> PllSicTog {
        PllSicTog(0)
    }
}
impl core::fmt::Debug for PllSicTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllSicTog")
            .field("misc2_control0", &self.misc2_control0())
            .field("pll_en_usb_clks", &self.pll_en_usb_clks())
            .field("pll_power", &self.pll_power())
            .field("pll_enable", &self.pll_enable())
            .field("pll_bypass", &self.pll_bypass())
            .field("refbias_pwd_sel", &self.refbias_pwd_sel())
            .field("refbias_pwd", &self.refbias_pwd())
            .field("pll_reg_enable", &self.pll_reg_enable())
            .field("pll_div_sel", &self.pll_div_sel())
            .field("pll_lock", &self.pll_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllSicTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllSicTog {{ misc2_control0: {=bool:?}, pll_en_usb_clks: {=bool:?}, pll_power: {=bool:?}, pll_enable: {=bool:?}, pll_bypass: {=bool:?}, refbias_pwd_sel: {=bool:?}, refbias_pwd: {=bool:?}, pll_reg_enable: {=bool:?}, pll_div_sel: {=u8:?}, pll_lock: {=bool:?} }}",
            self.misc2_control0(),
            self.pll_en_usb_clks(),
            self.pll_power(),
            self.pll_enable(),
            self.pll_bypass(),
            self.refbias_pwd_sel(),
            self.refbias_pwd(),
            self.pll_reg_enable(),
            self.pll_div_sel(),
            self.pll_lock()
        )
    }
}
#[doc = "Power Down."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwd(pub u32);
impl Pwd {
    #[doc = "Power Down USB FS TX Drivers."]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdfs(&self) -> Txpwdfs {
        let val = (self.0 >> 10usize) & 0x01;
        Txpwdfs::from_bits(val as u8)
    }
    #[doc = "Power Down USB FS TX Drivers."]
    #[inline(always)]
    pub const fn set_txpwdfs(&mut self, val: Txpwdfs) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Power Down USBPHY TX Current Bias Block."]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdibias(&self) -> Txpwdibias {
        let val = (self.0 >> 11usize) & 0x01;
        Txpwdibias::from_bits(val as u8)
    }
    #[doc = "Power Down USBPHY TX Current Bias Block."]
    #[inline(always)]
    pub const fn set_txpwdibias(&mut self, val: Txpwdibias) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Power Down USBPHY TX V-I Converter and Current Mirror."]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdv2i(&self) -> Txpwdv2i {
        let val = (self.0 >> 12usize) & 0x01;
        Txpwdv2i::from_bits(val as u8)
    }
    #[doc = "Power Down USBPHY TX V-I Converter and Current Mirror."]
    #[inline(always)]
    pub const fn set_txpwdv2i(&mut self, val: Txpwdv2i) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Power Down USB HS RX Envelope Detector."]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwdenv(&self) -> Rxpwdenv {
        let val = (self.0 >> 17usize) & 0x01;
        Rxpwdenv::from_bits(val as u8)
    }
    #[doc = "Power Down USB HS RX Envelope Detector."]
    #[inline(always)]
    pub const fn set_rxpwdenv(&mut self, val: Rxpwdenv) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Power Down USB FS Differential Receiver."]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwd1pt1(&self) -> Rxpwd1pt1 {
        let val = (self.0 >> 18usize) & 0x01;
        Rxpwd1pt1::from_bits(val as u8)
    }
    #[doc = "Power Down USB FS Differential Receiver."]
    #[inline(always)]
    pub const fn set_rxpwd1pt1(&mut self, val: Rxpwd1pt1) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Power Down USB HS Differential Receiver."]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwddiff(&self) -> Rxpwddiff {
        let val = (self.0 >> 19usize) & 0x01;
        Rxpwddiff::from_bits(val as u8)
    }
    #[doc = "Power Down USB HS Differential Receiver."]
    #[inline(always)]
    pub const fn set_rxpwddiff(&mut self, val: Rxpwddiff) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Power Down USBPHY Receiver Circuits."]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwdrx(&self) -> Rxpwdrx {
        let val = (self.0 >> 20usize) & 0x01;
        Rxpwdrx::from_bits(val as u8)
    }
    #[doc = "Power Down USBPHY Receiver Circuits."]
    #[inline(always)]
    pub const fn set_rxpwdrx(&mut self, val: Rxpwdrx) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
}
impl Default for Pwd {
    #[inline(always)]
    fn default() -> Pwd {
        Pwd(0)
    }
}
impl core::fmt::Debug for Pwd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pwd")
            .field("txpwdfs", &self.txpwdfs())
            .field("txpwdibias", &self.txpwdibias())
            .field("txpwdv2i", &self.txpwdv2i())
            .field("rxpwdenv", &self.rxpwdenv())
            .field("rxpwd1pt1", &self.rxpwd1pt1())
            .field("rxpwddiff", &self.rxpwddiff())
            .field("rxpwdrx", &self.rxpwdrx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pwd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pwd {{ txpwdfs: {:?}, txpwdibias: {:?}, txpwdv2i: {:?}, rxpwdenv: {:?}, rxpwd1pt1: {:?}, rxpwddiff: {:?}, rxpwdrx: {:?} }}",
            self.txpwdfs(),
            self.txpwdibias(),
            self.txpwdv2i(),
            self.rxpwdenv(),
            self.rxpwd1pt1(),
            self.rxpwddiff(),
            self.rxpwdrx()
        )
    }
}
#[doc = "Power Down."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwdClr(pub u32);
impl PwdClr {
    #[doc = "Power Down USB FS TX Drivers."]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdfs(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USB FS TX Drivers."]
    #[inline(always)]
    pub const fn set_txpwdfs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Power Down USBPHY TX Current Bias Block."]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdibias(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USBPHY TX Current Bias Block."]
    #[inline(always)]
    pub const fn set_txpwdibias(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Power Down USBPHY TX V-I Converter and Current Mirror."]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdv2i(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USBPHY TX V-I Converter and Current Mirror."]
    #[inline(always)]
    pub const fn set_txpwdv2i(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Power Down USB HS RX Envelope Detector."]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwdenv(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USB HS RX Envelope Detector."]
    #[inline(always)]
    pub const fn set_rxpwdenv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Power Down USB FS Differential Receiver."]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwd1pt1(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USB FS Differential Receiver."]
    #[inline(always)]
    pub const fn set_rxpwd1pt1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Power Down USB HS Differential Receiver."]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwddiff(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USB HS Differential Receiver."]
    #[inline(always)]
    pub const fn set_rxpwddiff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Power Down USBPHY Receiver Circuits."]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwdrx(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USBPHY Receiver Circuits."]
    #[inline(always)]
    pub const fn set_rxpwdrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for PwdClr {
    #[inline(always)]
    fn default() -> PwdClr {
        PwdClr(0)
    }
}
impl core::fmt::Debug for PwdClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PwdClr")
            .field("txpwdfs", &self.txpwdfs())
            .field("txpwdibias", &self.txpwdibias())
            .field("txpwdv2i", &self.txpwdv2i())
            .field("rxpwdenv", &self.rxpwdenv())
            .field("rxpwd1pt1", &self.rxpwd1pt1())
            .field("rxpwddiff", &self.rxpwddiff())
            .field("rxpwdrx", &self.rxpwdrx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PwdClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PwdClr {{ txpwdfs: {=bool:?}, txpwdibias: {=bool:?}, txpwdv2i: {=bool:?}, rxpwdenv: {=bool:?}, rxpwd1pt1: {=bool:?}, rxpwddiff: {=bool:?}, rxpwdrx: {=bool:?} }}",
            self.txpwdfs(),
            self.txpwdibias(),
            self.txpwdv2i(),
            self.rxpwdenv(),
            self.rxpwd1pt1(),
            self.rxpwddiff(),
            self.rxpwdrx()
        )
    }
}
#[doc = "Power Down."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwdSet(pub u32);
impl PwdSet {
    #[doc = "Power Down USB FS TX Drivers."]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdfs(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USB FS TX Drivers."]
    #[inline(always)]
    pub const fn set_txpwdfs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Power Down USBPHY TX Current Bias Block."]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdibias(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USBPHY TX Current Bias Block."]
    #[inline(always)]
    pub const fn set_txpwdibias(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Power Down USBPHY TX V-I Converter and Current Mirror."]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdv2i(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USBPHY TX V-I Converter and Current Mirror."]
    #[inline(always)]
    pub const fn set_txpwdv2i(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Power Down USB HS RX Envelope Detector."]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwdenv(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USB HS RX Envelope Detector."]
    #[inline(always)]
    pub const fn set_rxpwdenv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Power Down USB FS Differential Receiver."]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwd1pt1(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USB FS Differential Receiver."]
    #[inline(always)]
    pub const fn set_rxpwd1pt1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Power Down USB HS Differential Receiver."]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwddiff(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USB HS Differential Receiver."]
    #[inline(always)]
    pub const fn set_rxpwddiff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Power Down USBPHY Receiver Circuits."]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwdrx(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USBPHY Receiver Circuits."]
    #[inline(always)]
    pub const fn set_rxpwdrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for PwdSet {
    #[inline(always)]
    fn default() -> PwdSet {
        PwdSet(0)
    }
}
impl core::fmt::Debug for PwdSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PwdSet")
            .field("txpwdfs", &self.txpwdfs())
            .field("txpwdibias", &self.txpwdibias())
            .field("txpwdv2i", &self.txpwdv2i())
            .field("rxpwdenv", &self.rxpwdenv())
            .field("rxpwd1pt1", &self.rxpwd1pt1())
            .field("rxpwddiff", &self.rxpwddiff())
            .field("rxpwdrx", &self.rxpwdrx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PwdSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PwdSet {{ txpwdfs: {=bool:?}, txpwdibias: {=bool:?}, txpwdv2i: {=bool:?}, rxpwdenv: {=bool:?}, rxpwd1pt1: {=bool:?}, rxpwddiff: {=bool:?}, rxpwdrx: {=bool:?} }}",
            self.txpwdfs(),
            self.txpwdibias(),
            self.txpwdv2i(),
            self.rxpwdenv(),
            self.rxpwd1pt1(),
            self.rxpwddiff(),
            self.rxpwdrx()
        )
    }
}
#[doc = "Power Down."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwdTog(pub u32);
impl PwdTog {
    #[doc = "Power Down USB FS TX Drivers."]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdfs(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USB FS TX Drivers."]
    #[inline(always)]
    pub const fn set_txpwdfs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Power Down USBPHY TX Current Bias Block."]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdibias(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USBPHY TX Current Bias Block."]
    #[inline(always)]
    pub const fn set_txpwdibias(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Power Down USBPHY TX V-I Converter and Current Mirror."]
    #[must_use]
    #[inline(always)]
    pub const fn txpwdv2i(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USBPHY TX V-I Converter and Current Mirror."]
    #[inline(always)]
    pub const fn set_txpwdv2i(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Power Down USB HS RX Envelope Detector."]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwdenv(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USB HS RX Envelope Detector."]
    #[inline(always)]
    pub const fn set_rxpwdenv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Power Down USB FS Differential Receiver."]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwd1pt1(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USB FS Differential Receiver."]
    #[inline(always)]
    pub const fn set_rxpwd1pt1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Power Down USB HS Differential Receiver."]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwddiff(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USB HS Differential Receiver."]
    #[inline(always)]
    pub const fn set_rxpwddiff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Power Down USBPHY Receiver Circuits."]
    #[must_use]
    #[inline(always)]
    pub const fn rxpwdrx(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Power Down USBPHY Receiver Circuits."]
    #[inline(always)]
    pub const fn set_rxpwdrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for PwdTog {
    #[inline(always)]
    fn default() -> PwdTog {
        PwdTog(0)
    }
}
impl core::fmt::Debug for PwdTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PwdTog")
            .field("txpwdfs", &self.txpwdfs())
            .field("txpwdibias", &self.txpwdibias())
            .field("txpwdv2i", &self.txpwdv2i())
            .field("rxpwdenv", &self.rxpwdenv())
            .field("rxpwd1pt1", &self.rxpwd1pt1())
            .field("rxpwddiff", &self.rxpwddiff())
            .field("rxpwdrx", &self.rxpwdrx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PwdTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PwdTog {{ txpwdfs: {=bool:?}, txpwdibias: {=bool:?}, txpwdv2i: {=bool:?}, rxpwdenv: {=bool:?}, rxpwd1pt1: {=bool:?}, rxpwddiff: {=bool:?}, rxpwdrx: {=bool:?} }}",
            self.txpwdfs(),
            self.txpwdibias(),
            self.txpwdv2i(),
            self.rxpwdenv(),
            self.rxpwd1pt1(),
            self.rxpwddiff(),
            self.rxpwdrx()
        )
    }
}
#[doc = "RX Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rx(pub u32);
impl Rx {
    #[doc = "Envelope Detector Trip Point."]
    #[must_use]
    #[inline(always)]
    pub const fn envadj(&self) -> Envadj {
        let val = (self.0 >> 0usize) & 0x07;
        Envadj::from_bits(val as u8)
    }
    #[doc = "Envelope Detector Trip Point."]
    #[inline(always)]
    pub const fn set_envadj(&mut self, val: Envadj) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Disconnect Detector Trip Point."]
    #[must_use]
    #[inline(always)]
    pub const fn disconadj(&self) -> Disconadj {
        let val = (self.0 >> 4usize) & 0x07;
        Disconadj::from_bits(val as u8)
    }
    #[doc = "Disconnect Detector Trip Point."]
    #[inline(always)]
    pub const fn set_disconadj(&mut self, val: Disconadj) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
}
impl Default for Rx {
    #[inline(always)]
    fn default() -> Rx {
        Rx(0)
    }
}
impl core::fmt::Debug for Rx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rx")
            .field("envadj", &self.envadj())
            .field("disconadj", &self.disconadj())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rx {{ envadj: {:?}, disconadj: {:?} }}",
            self.envadj(),
            self.disconadj()
        )
    }
}
#[doc = "RX Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxClr(pub u32);
impl RxClr {
    #[doc = "Envelope Detector Trip Point."]
    #[must_use]
    #[inline(always)]
    pub const fn envadj(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Envelope Detector Trip Point."]
    #[inline(always)]
    pub const fn set_envadj(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Disconnect Detector Trip Point."]
    #[must_use]
    #[inline(always)]
    pub const fn disconadj(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Disconnect Detector Trip Point."]
    #[inline(always)]
    pub const fn set_disconadj(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
}
impl Default for RxClr {
    #[inline(always)]
    fn default() -> RxClr {
        RxClr(0)
    }
}
impl core::fmt::Debug for RxClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RxClr")
            .field("envadj", &self.envadj())
            .field("disconadj", &self.disconadj())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RxClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RxClr {{ envadj: {=u8:?}, disconadj: {=u8:?} }}",
            self.envadj(),
            self.disconadj()
        )
    }
}
#[doc = "RX Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxSet(pub u32);
impl RxSet {
    #[doc = "Envelope Detector Trip Point."]
    #[must_use]
    #[inline(always)]
    pub const fn envadj(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Envelope Detector Trip Point."]
    #[inline(always)]
    pub const fn set_envadj(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Disconnect Detector Trip Point."]
    #[must_use]
    #[inline(always)]
    pub const fn disconadj(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Disconnect Detector Trip Point."]
    #[inline(always)]
    pub const fn set_disconadj(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
}
impl Default for RxSet {
    #[inline(always)]
    fn default() -> RxSet {
        RxSet(0)
    }
}
impl core::fmt::Debug for RxSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RxSet")
            .field("envadj", &self.envadj())
            .field("disconadj", &self.disconadj())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RxSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RxSet {{ envadj: {=u8:?}, disconadj: {=u8:?} }}",
            self.envadj(),
            self.disconadj()
        )
    }
}
#[doc = "RX Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxTog(pub u32);
impl RxTog {
    #[doc = "Envelope Detector Trip Point."]
    #[must_use]
    #[inline(always)]
    pub const fn envadj(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Envelope Detector Trip Point."]
    #[inline(always)]
    pub const fn set_envadj(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Disconnect Detector Trip Point."]
    #[must_use]
    #[inline(always)]
    pub const fn disconadj(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Disconnect Detector Trip Point."]
    #[inline(always)]
    pub const fn set_disconadj(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
}
impl Default for RxTog {
    #[inline(always)]
    fn default() -> RxTog {
        RxTog(0)
    }
}
impl core::fmt::Debug for RxTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RxTog")
            .field("envadj", &self.envadj())
            .field("disconadj", &self.disconadj())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RxTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RxTog {{ envadj: {=u8:?}, disconadj: {=u8:?} }}",
            self.envadj(),
            self.disconadj()
        )
    }
}
#[doc = "Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "USB 3.3 V and 1.8 V Supply Status."]
    #[must_use]
    #[inline(always)]
    pub const fn ok_status_3v(&self) -> OkStatus3v {
        let val = (self.0 >> 0usize) & 0x01;
        OkStatus3v::from_bits(val as u8)
    }
    #[doc = "USB 3.3 V and 1.8 V Supply Status."]
    #[inline(always)]
    pub const fn set_ok_status_3v(&mut self, val: OkStatus3v) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Host Disconnect Status."]
    #[must_use]
    #[inline(always)]
    pub const fn hostdiscondetect_status(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Host Disconnect Status."]
    #[inline(always)]
    pub const fn set_hostdiscondetect_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Status Indicator for Nonstandard Resistive Plugged-In Detection."]
    #[must_use]
    #[inline(always)]
    pub const fn devplugin_status(&self) -> DevpluginStatus {
        let val = (self.0 >> 6usize) & 0x01;
        DevpluginStatus::from_bits(val as u8)
    }
    #[doc = "Status Indicator for Nonstandard Resistive Plugged-In Detection."]
    #[inline(always)]
    pub const fn set_devplugin_status(&mut self, val: DevpluginStatus) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "OTG ID Status."]
    #[must_use]
    #[inline(always)]
    pub const fn otgid_status(&self) -> OtgidStatus {
        let val = (self.0 >> 8usize) & 0x01;
        OtgidStatus::from_bits(val as u8)
    }
    #[doc = "OTG ID Status."]
    #[inline(always)]
    pub const fn set_otgid_status(&mut self, val: OtgidStatus) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Resume Status."]
    #[must_use]
    #[inline(always)]
    pub const fn resume_status(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Resume Status."]
    #[inline(always)]
    pub const fn set_resume_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
impl core::fmt::Debug for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Status")
            .field("ok_status_3v", &self.ok_status_3v())
            .field("hostdiscondetect_status", &self.hostdiscondetect_status())
            .field("devplugin_status", &self.devplugin_status())
            .field("otgid_status", &self.otgid_status())
            .field("resume_status", &self.resume_status())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Status {{ ok_status_3v: {:?}, hostdiscondetect_status: {=bool:?}, devplugin_status: {:?}, otgid_status: {:?}, resume_status: {=bool:?} }}",
            self.ok_status_3v(),
            self.hostdiscondetect_status(),
            self.devplugin_status(),
            self.otgid_status(),
            self.resume_status()
        )
    }
}
#[doc = "Trim."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrimOverrideEn(pub u32);
impl TrimOverrideEn {
    #[doc = "Override Enable for PLL Divider Value."]
    #[must_use]
    #[inline(always)]
    pub const fn div_sel_override(&self) -> DivSelOverride {
        let val = (self.0 >> 0usize) & 0x01;
        DivSelOverride::from_bits(val as u8)
    }
    #[doc = "Override Enable for PLL Divider Value."]
    #[inline(always)]
    pub const fn set_div_sel_override(&mut self, val: DivSelOverride) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Override Enable for HS TX Output Current Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn tx_d_cal_override(&self) -> TxDCalOverride {
        let val = (self.0 >> 2usize) & 0x01;
        TxDCalOverride::from_bits(val as u8)
    }
    #[doc = "Override Enable for HS TX Output Current Trim."]
    #[inline(always)]
    pub const fn set_tx_d_cal_override(&mut self, val: TxDCalOverride) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Override Enable for USB_DP Series Termination Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn tx_cal45dp_override(&self) -> TxCal45dpOverride {
        let val = (self.0 >> 3usize) & 0x01;
        TxCal45dpOverride::from_bits(val as u8)
    }
    #[doc = "Override Enable for USB_DP Series Termination Trim."]
    #[inline(always)]
    pub const fn set_tx_cal45dp_override(&mut self, val: TxCal45dpOverride) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Override Enable for USB_DM Series Termination Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn tx_cal45dm_override(&self) -> TxCal45dmOverride {
        let val = (self.0 >> 4usize) & 0x01;
        TxCal45dmOverride::from_bits(val as u8)
    }
    #[doc = "Override Enable for USB_DM Series Termination Trim."]
    #[inline(always)]
    pub const fn set_tx_cal45dm_override(&mut self, val: TxCal45dmOverride) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "PLL Divider Value Configuration Bits from Outside USBPHY."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_ctrl0_div_sel(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[doc = "PLL Divider Value Configuration Bits from Outside USBPHY."]
    #[inline(always)]
    pub const fn set_pll_ctrl0_div_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[doc = "HS TX Output Current Trim Bits from Outside USBPHY."]
    #[must_use]
    #[inline(always)]
    pub const fn usbphy_tx_d_cal(&self) -> UsbphyTxDCal {
        let val = (self.0 >> 20usize) & 0x0f;
        UsbphyTxDCal::from_bits(val as u8)
    }
    #[doc = "HS TX Output Current Trim Bits from Outside USBPHY."]
    #[inline(always)]
    pub const fn set_usbphy_tx_d_cal(&mut self, val: UsbphyTxDCal) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
    #[doc = "DP Series Termination Resistance Trim Bits from Outside USBPHY."]
    #[must_use]
    #[inline(always)]
    pub const fn usbphy_tx_cal45dp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "DP Series Termination Resistance Trim Bits from Outside USBPHY."]
    #[inline(always)]
    pub const fn set_usbphy_tx_cal45dp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "DM Series Termination Resistance Trim Bits from Outside USBPHY."]
    #[must_use]
    #[inline(always)]
    pub const fn usbphy_tx_cal45dn(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "DM Series Termination Resistance Trim Bits from Outside USBPHY."]
    #[inline(always)]
    pub const fn set_usbphy_tx_cal45dn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for TrimOverrideEn {
    #[inline(always)]
    fn default() -> TrimOverrideEn {
        TrimOverrideEn(0)
    }
}
impl core::fmt::Debug for TrimOverrideEn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrimOverrideEn")
            .field("div_sel_override", &self.div_sel_override())
            .field("tx_d_cal_override", &self.tx_d_cal_override())
            .field("tx_cal45dp_override", &self.tx_cal45dp_override())
            .field("tx_cal45dm_override", &self.tx_cal45dm_override())
            .field("pll_ctrl0_div_sel", &self.pll_ctrl0_div_sel())
            .field("usbphy_tx_d_cal", &self.usbphy_tx_d_cal())
            .field("usbphy_tx_cal45dp", &self.usbphy_tx_cal45dp())
            .field("usbphy_tx_cal45dn", &self.usbphy_tx_cal45dn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrimOverrideEn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TrimOverrideEn {{ div_sel_override: {:?}, tx_d_cal_override: {:?}, tx_cal45dp_override: {:?}, tx_cal45dm_override: {:?}, pll_ctrl0_div_sel: {=u8:?}, usbphy_tx_d_cal: {:?}, usbphy_tx_cal45dp: {=u8:?}, usbphy_tx_cal45dn: {=u8:?} }}",
            self.div_sel_override(),
            self.tx_d_cal_override(),
            self.tx_cal45dp_override(),
            self.tx_cal45dm_override(),
            self.pll_ctrl0_div_sel(),
            self.usbphy_tx_d_cal(),
            self.usbphy_tx_cal45dp(),
            self.usbphy_tx_cal45dn()
        )
    }
}
#[doc = "Trim."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrimOverrideEnClr(pub u32);
impl TrimOverrideEnClr {
    #[doc = "Override Enable for PLL Divider Value."]
    #[must_use]
    #[inline(always)]
    pub const fn div_sel_override(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Override Enable for PLL Divider Value."]
    #[inline(always)]
    pub const fn set_div_sel_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Override Enable for HS TX Output Current Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn tx_d_cal_override(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Override Enable for HS TX Output Current Trim."]
    #[inline(always)]
    pub const fn set_tx_d_cal_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Override Enable for USB_DP Series Termination Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn tx_cal45dp_override(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Override Enable for USB_DP Series Termination Trim."]
    #[inline(always)]
    pub const fn set_tx_cal45dp_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Override Enable for USB_DM Series Termination Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn tx_cal45dm_override(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Override Enable for USB_DM Series Termination Trim."]
    #[inline(always)]
    pub const fn set_tx_cal45dm_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "PLL Divider Value Configuration Bits from Outside USBPHY."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_ctrl0_div_sel(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[doc = "PLL Divider Value Configuration Bits from Outside USBPHY."]
    #[inline(always)]
    pub const fn set_pll_ctrl0_div_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[doc = "HS TX Output Current Trim Bits from Outside USBPHY."]
    #[must_use]
    #[inline(always)]
    pub const fn usbphy_tx_d_cal(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "HS TX Output Current Trim Bits from Outside USBPHY."]
    #[inline(always)]
    pub const fn set_usbphy_tx_d_cal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "DP Series Termination Resistance Trim Bits from Outside USBPHY."]
    #[must_use]
    #[inline(always)]
    pub const fn usbphy_tx_cal45dp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "DP Series Termination Resistance Trim Bits from Outside USBPHY."]
    #[inline(always)]
    pub const fn set_usbphy_tx_cal45dp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "DM Series Termination Resistance Trim Bits from Outside USBPHY."]
    #[must_use]
    #[inline(always)]
    pub const fn usbphy_tx_cal45dn(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "DM Series Termination Resistance Trim Bits from Outside USBPHY."]
    #[inline(always)]
    pub const fn set_usbphy_tx_cal45dn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for TrimOverrideEnClr {
    #[inline(always)]
    fn default() -> TrimOverrideEnClr {
        TrimOverrideEnClr(0)
    }
}
impl core::fmt::Debug for TrimOverrideEnClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrimOverrideEnClr")
            .field("div_sel_override", &self.div_sel_override())
            .field("tx_d_cal_override", &self.tx_d_cal_override())
            .field("tx_cal45dp_override", &self.tx_cal45dp_override())
            .field("tx_cal45dm_override", &self.tx_cal45dm_override())
            .field("pll_ctrl0_div_sel", &self.pll_ctrl0_div_sel())
            .field("usbphy_tx_d_cal", &self.usbphy_tx_d_cal())
            .field("usbphy_tx_cal45dp", &self.usbphy_tx_cal45dp())
            .field("usbphy_tx_cal45dn", &self.usbphy_tx_cal45dn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrimOverrideEnClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TrimOverrideEnClr {{ div_sel_override: {=bool:?}, tx_d_cal_override: {=bool:?}, tx_cal45dp_override: {=bool:?}, tx_cal45dm_override: {=bool:?}, pll_ctrl0_div_sel: {=u8:?}, usbphy_tx_d_cal: {=u8:?}, usbphy_tx_cal45dp: {=u8:?}, usbphy_tx_cal45dn: {=u8:?} }}",
            self.div_sel_override(),
            self.tx_d_cal_override(),
            self.tx_cal45dp_override(),
            self.tx_cal45dm_override(),
            self.pll_ctrl0_div_sel(),
            self.usbphy_tx_d_cal(),
            self.usbphy_tx_cal45dp(),
            self.usbphy_tx_cal45dn()
        )
    }
}
#[doc = "Trim."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrimOverrideEnSet(pub u32);
impl TrimOverrideEnSet {
    #[doc = "Override Enable for PLL Divider Value."]
    #[must_use]
    #[inline(always)]
    pub const fn div_sel_override(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Override Enable for PLL Divider Value."]
    #[inline(always)]
    pub const fn set_div_sel_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Override Enable for HS TX Output Current Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn tx_d_cal_override(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Override Enable for HS TX Output Current Trim."]
    #[inline(always)]
    pub const fn set_tx_d_cal_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Override Enable for USB_DP Series Termination Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn tx_cal45dp_override(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Override Enable for USB_DP Series Termination Trim."]
    #[inline(always)]
    pub const fn set_tx_cal45dp_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Override Enable for USB_DM Series Termination Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn tx_cal45dm_override(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Override Enable for USB_DM Series Termination Trim."]
    #[inline(always)]
    pub const fn set_tx_cal45dm_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "PLL Divider Value Configuration Bits from Outside USBPHY."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_ctrl0_div_sel(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[doc = "PLL Divider Value Configuration Bits from Outside USBPHY."]
    #[inline(always)]
    pub const fn set_pll_ctrl0_div_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[doc = "HS TX Output Current Trim Bits from Outside USBPHY."]
    #[must_use]
    #[inline(always)]
    pub const fn usbphy_tx_d_cal(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "HS TX Output Current Trim Bits from Outside USBPHY."]
    #[inline(always)]
    pub const fn set_usbphy_tx_d_cal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "DP Series Termination Resistance Trim Bits from Outside USBPHY."]
    #[must_use]
    #[inline(always)]
    pub const fn usbphy_tx_cal45dp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "DP Series Termination Resistance Trim Bits from Outside USBPHY."]
    #[inline(always)]
    pub const fn set_usbphy_tx_cal45dp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "DM Series Termination Resistance Trim Bits from Outside USBPHY."]
    #[must_use]
    #[inline(always)]
    pub const fn usbphy_tx_cal45dn(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "DM Series Termination Resistance Trim Bits from Outside USBPHY."]
    #[inline(always)]
    pub const fn set_usbphy_tx_cal45dn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for TrimOverrideEnSet {
    #[inline(always)]
    fn default() -> TrimOverrideEnSet {
        TrimOverrideEnSet(0)
    }
}
impl core::fmt::Debug for TrimOverrideEnSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrimOverrideEnSet")
            .field("div_sel_override", &self.div_sel_override())
            .field("tx_d_cal_override", &self.tx_d_cal_override())
            .field("tx_cal45dp_override", &self.tx_cal45dp_override())
            .field("tx_cal45dm_override", &self.tx_cal45dm_override())
            .field("pll_ctrl0_div_sel", &self.pll_ctrl0_div_sel())
            .field("usbphy_tx_d_cal", &self.usbphy_tx_d_cal())
            .field("usbphy_tx_cal45dp", &self.usbphy_tx_cal45dp())
            .field("usbphy_tx_cal45dn", &self.usbphy_tx_cal45dn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrimOverrideEnSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TrimOverrideEnSet {{ div_sel_override: {=bool:?}, tx_d_cal_override: {=bool:?}, tx_cal45dp_override: {=bool:?}, tx_cal45dm_override: {=bool:?}, pll_ctrl0_div_sel: {=u8:?}, usbphy_tx_d_cal: {=u8:?}, usbphy_tx_cal45dp: {=u8:?}, usbphy_tx_cal45dn: {=u8:?} }}",
            self.div_sel_override(),
            self.tx_d_cal_override(),
            self.tx_cal45dp_override(),
            self.tx_cal45dm_override(),
            self.pll_ctrl0_div_sel(),
            self.usbphy_tx_d_cal(),
            self.usbphy_tx_cal45dp(),
            self.usbphy_tx_cal45dn()
        )
    }
}
#[doc = "Trim."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrimOverrideEnTog(pub u32);
impl TrimOverrideEnTog {
    #[doc = "Override Enable for PLL Divider Value."]
    #[must_use]
    #[inline(always)]
    pub const fn div_sel_override(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Override Enable for PLL Divider Value."]
    #[inline(always)]
    pub const fn set_div_sel_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Override Enable for HS TX Output Current Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn tx_d_cal_override(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Override Enable for HS TX Output Current Trim."]
    #[inline(always)]
    pub const fn set_tx_d_cal_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Override Enable for USB_DP Series Termination Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn tx_cal45dp_override(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Override Enable for USB_DP Series Termination Trim."]
    #[inline(always)]
    pub const fn set_tx_cal45dp_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Override Enable for USB_DM Series Termination Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn tx_cal45dm_override(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Override Enable for USB_DM Series Termination Trim."]
    #[inline(always)]
    pub const fn set_tx_cal45dm_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "PLL Divider Value Configuration Bits from Outside USBPHY."]
    #[must_use]
    #[inline(always)]
    pub const fn pll_ctrl0_div_sel(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[doc = "PLL Divider Value Configuration Bits from Outside USBPHY."]
    #[inline(always)]
    pub const fn set_pll_ctrl0_div_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[doc = "HS TX Output Current Trim Bits from Outside USBPHY."]
    #[must_use]
    #[inline(always)]
    pub const fn usbphy_tx_d_cal(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "HS TX Output Current Trim Bits from Outside USBPHY."]
    #[inline(always)]
    pub const fn set_usbphy_tx_d_cal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "DP Series Termination Resistance Trim Bits from Outside USBPHY."]
    #[must_use]
    #[inline(always)]
    pub const fn usbphy_tx_cal45dp(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "DP Series Termination Resistance Trim Bits from Outside USBPHY."]
    #[inline(always)]
    pub const fn set_usbphy_tx_cal45dp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "DM Series Termination Resistance Trim Bits from Outside USBPHY."]
    #[must_use]
    #[inline(always)]
    pub const fn usbphy_tx_cal45dn(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "DM Series Termination Resistance Trim Bits from Outside USBPHY."]
    #[inline(always)]
    pub const fn set_usbphy_tx_cal45dn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for TrimOverrideEnTog {
    #[inline(always)]
    fn default() -> TrimOverrideEnTog {
        TrimOverrideEnTog(0)
    }
}
impl core::fmt::Debug for TrimOverrideEnTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrimOverrideEnTog")
            .field("div_sel_override", &self.div_sel_override())
            .field("tx_d_cal_override", &self.tx_d_cal_override())
            .field("tx_cal45dp_override", &self.tx_cal45dp_override())
            .field("tx_cal45dm_override", &self.tx_cal45dm_override())
            .field("pll_ctrl0_div_sel", &self.pll_ctrl0_div_sel())
            .field("usbphy_tx_d_cal", &self.usbphy_tx_d_cal())
            .field("usbphy_tx_cal45dp", &self.usbphy_tx_cal45dp())
            .field("usbphy_tx_cal45dn", &self.usbphy_tx_cal45dn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrimOverrideEnTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TrimOverrideEnTog {{ div_sel_override: {=bool:?}, tx_d_cal_override: {=bool:?}, tx_cal45dp_override: {=bool:?}, tx_cal45dm_override: {=bool:?}, pll_ctrl0_div_sel: {=u8:?}, usbphy_tx_d_cal: {=u8:?}, usbphy_tx_cal45dp: {=u8:?}, usbphy_tx_cal45dn: {=u8:?} }}",
            self.div_sel_override(),
            self.tx_d_cal_override(),
            self.tx_cal45dp_override(),
            self.tx_cal45dm_override(),
            self.pll_ctrl0_div_sel(),
            self.usbphy_tx_d_cal(),
            self.usbphy_tx_cal45dp(),
            self.usbphy_tx_cal45dn()
        )
    }
}
#[doc = "TX Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tx(pub u32);
impl Tx {
    #[doc = "HS TX Output Current Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn d_cal(&self) -> DCal {
        let val = (self.0 >> 0usize) & 0x0f;
        DCal::from_bits(val as u8)
    }
    #[doc = "HS TX Output Current Trim."]
    #[inline(always)]
    pub const fn set_d_cal(&mut self, val: DCal) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "DM Series Termination Resistance Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn txcal45dn(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "DM Series Termination Resistance Trim."]
    #[inline(always)]
    pub const fn set_txcal45dn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "DP Series Termination Resistance Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn txcal45dp(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "DP Series Termination Resistance Trim."]
    #[inline(always)]
    pub const fn set_txcal45dp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Tx {
    #[inline(always)]
    fn default() -> Tx {
        Tx(0)
    }
}
impl core::fmt::Debug for Tx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tx")
            .field("d_cal", &self.d_cal())
            .field("txcal45dn", &self.txcal45dn())
            .field("txcal45dp", &self.txcal45dp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tx {{ d_cal: {:?}, txcal45dn: {=u8:?}, txcal45dp: {=u8:?} }}",
            self.d_cal(),
            self.txcal45dn(),
            self.txcal45dp()
        )
    }
}
#[doc = "TX Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxClr(pub u32);
impl TxClr {
    #[doc = "HS TX Output Current Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn d_cal(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "HS TX Output Current Trim."]
    #[inline(always)]
    pub const fn set_d_cal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "DM Series Termination Resistance Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn txcal45dn(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "DM Series Termination Resistance Trim."]
    #[inline(always)]
    pub const fn set_txcal45dn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "DP Series Termination Resistance Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn txcal45dp(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "DP Series Termination Resistance Trim."]
    #[inline(always)]
    pub const fn set_txcal45dp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for TxClr {
    #[inline(always)]
    fn default() -> TxClr {
        TxClr(0)
    }
}
impl core::fmt::Debug for TxClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxClr")
            .field("d_cal", &self.d_cal())
            .field("txcal45dn", &self.txcal45dn())
            .field("txcal45dp", &self.txcal45dp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TxClr {{ d_cal: {=u8:?}, txcal45dn: {=u8:?}, txcal45dp: {=u8:?} }}",
            self.d_cal(),
            self.txcal45dn(),
            self.txcal45dp()
        )
    }
}
#[doc = "TX Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxSet(pub u32);
impl TxSet {
    #[doc = "HS TX Output Current Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn d_cal(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "HS TX Output Current Trim."]
    #[inline(always)]
    pub const fn set_d_cal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "DM Series Termination Resistance Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn txcal45dn(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "DM Series Termination Resistance Trim."]
    #[inline(always)]
    pub const fn set_txcal45dn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "DP Series Termination Resistance Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn txcal45dp(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "DP Series Termination Resistance Trim."]
    #[inline(always)]
    pub const fn set_txcal45dp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for TxSet {
    #[inline(always)]
    fn default() -> TxSet {
        TxSet(0)
    }
}
impl core::fmt::Debug for TxSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxSet")
            .field("d_cal", &self.d_cal())
            .field("txcal45dn", &self.txcal45dn())
            .field("txcal45dp", &self.txcal45dp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TxSet {{ d_cal: {=u8:?}, txcal45dn: {=u8:?}, txcal45dp: {=u8:?} }}",
            self.d_cal(),
            self.txcal45dn(),
            self.txcal45dp()
        )
    }
}
#[doc = "TX Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxTog(pub u32);
impl TxTog {
    #[doc = "HS TX Output Current Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn d_cal(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "HS TX Output Current Trim."]
    #[inline(always)]
    pub const fn set_d_cal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "DM Series Termination Resistance Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn txcal45dn(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "DM Series Termination Resistance Trim."]
    #[inline(always)]
    pub const fn set_txcal45dn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "DP Series Termination Resistance Trim."]
    #[must_use]
    #[inline(always)]
    pub const fn txcal45dp(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "DP Series Termination Resistance Trim."]
    #[inline(always)]
    pub const fn set_txcal45dp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for TxTog {
    #[inline(always)]
    fn default() -> TxTog {
        TxTog(0)
    }
}
impl core::fmt::Debug for TxTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxTog")
            .field("d_cal", &self.d_cal())
            .field("txcal45dn", &self.txcal45dn())
            .field("txcal45dp", &self.txcal45dp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TxTog {{ d_cal: {=u8:?}, txcal45dn: {=u8:?}, txcal45dp: {=u8:?} }}",
            self.d_cal(),
            self.txcal45dn(),
            self.txcal45dp()
        )
    }
}
#[doc = "Charger Detect Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1ChrgDetStat(pub u32);
impl Usb1ChrgDetStat {
    #[doc = "Battery Charging Data Contact Detection Phase Output."]
    #[must_use]
    #[inline(always)]
    pub const fn plug_contact(&self) -> PlugContact {
        let val = (self.0 >> 0usize) & 0x01;
        PlugContact::from_bits(val as u8)
    }
    #[doc = "Battery Charging Data Contact Detection Phase Output."]
    #[inline(always)]
    pub const fn set_plug_contact(&mut self, val: PlugContact) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Battery Charging Primary Detection Phase Output."]
    #[must_use]
    #[inline(always)]
    pub const fn chrg_detected(&self) -> ChrgDetected {
        let val = (self.0 >> 1usize) & 0x01;
        ChrgDetected::from_bits(val as u8)
    }
    #[doc = "Battery Charging Primary Detection Phase Output."]
    #[inline(always)]
    pub const fn set_chrg_detected(&mut self, val: ChrgDetected) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "DM Voltage."]
    #[must_use]
    #[inline(always)]
    pub const fn dm_state(&self) -> DmState {
        let val = (self.0 >> 2usize) & 0x01;
        DmState::from_bits(val as u8)
    }
    #[doc = "DM Voltage."]
    #[inline(always)]
    pub const fn set_dm_state(&mut self, val: DmState) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "DP Voltage."]
    #[must_use]
    #[inline(always)]
    pub const fn dp_state(&self) -> DpState {
        let val = (self.0 >> 3usize) & 0x01;
        DpState::from_bits(val as u8)
    }
    #[doc = "DP Voltage."]
    #[inline(always)]
    pub const fn set_dp_state(&mut self, val: DpState) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Battery Charging Secondary Detection Phase Output."]
    #[must_use]
    #[inline(always)]
    pub const fn secdet_dcp(&self) -> SecdetDcp {
        let val = (self.0 >> 4usize) & 0x01;
        SecdetDcp::from_bits(val as u8)
    }
    #[doc = "Battery Charging Secondary Detection Phase Output."]
    #[inline(always)]
    pub const fn set_secdet_dcp(&mut self, val: SecdetDcp) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
}
impl Default for Usb1ChrgDetStat {
    #[inline(always)]
    fn default() -> Usb1ChrgDetStat {
        Usb1ChrgDetStat(0)
    }
}
impl core::fmt::Debug for Usb1ChrgDetStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1ChrgDetStat")
            .field("plug_contact", &self.plug_contact())
            .field("chrg_detected", &self.chrg_detected())
            .field("dm_state", &self.dm_state())
            .field("dp_state", &self.dp_state())
            .field("secdet_dcp", &self.secdet_dcp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1ChrgDetStat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1ChrgDetStat {{ plug_contact: {:?}, chrg_detected: {:?}, dm_state: {:?}, dp_state: {:?}, secdet_dcp: {:?} }}",
            self.plug_contact(),
            self.chrg_detected(),
            self.dm_state(),
            self.dp_state(),
            self.secdet_dcp()
        )
    }
}
#[doc = "Charger Detect Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1ChrgDetStatClr(pub u32);
impl Usb1ChrgDetStatClr {
    #[doc = "Battery Charging Data Contact Detection Phase Output."]
    #[must_use]
    #[inline(always)]
    pub const fn plug_contact(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Battery Charging Data Contact Detection Phase Output."]
    #[inline(always)]
    pub const fn set_plug_contact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Battery Charging Primary Detection Phase Output."]
    #[must_use]
    #[inline(always)]
    pub const fn chrg_detected(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Battery Charging Primary Detection Phase Output."]
    #[inline(always)]
    pub const fn set_chrg_detected(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DM Voltage."]
    #[must_use]
    #[inline(always)]
    pub const fn dm_state(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DM Voltage."]
    #[inline(always)]
    pub const fn set_dm_state(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DP Voltage."]
    #[must_use]
    #[inline(always)]
    pub const fn dp_state(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DP Voltage."]
    #[inline(always)]
    pub const fn set_dp_state(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Battery Charging Secondary Detection Phase Output."]
    #[must_use]
    #[inline(always)]
    pub const fn secdet_dcp(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Battery Charging Secondary Detection Phase Output."]
    #[inline(always)]
    pub const fn set_secdet_dcp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Usb1ChrgDetStatClr {
    #[inline(always)]
    fn default() -> Usb1ChrgDetStatClr {
        Usb1ChrgDetStatClr(0)
    }
}
impl core::fmt::Debug for Usb1ChrgDetStatClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1ChrgDetStatClr")
            .field("plug_contact", &self.plug_contact())
            .field("chrg_detected", &self.chrg_detected())
            .field("dm_state", &self.dm_state())
            .field("dp_state", &self.dp_state())
            .field("secdet_dcp", &self.secdet_dcp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1ChrgDetStatClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1ChrgDetStatClr {{ plug_contact: {=bool:?}, chrg_detected: {=bool:?}, dm_state: {=bool:?}, dp_state: {=bool:?}, secdet_dcp: {=bool:?} }}",
            self.plug_contact(),
            self.chrg_detected(),
            self.dm_state(),
            self.dp_state(),
            self.secdet_dcp()
        )
    }
}
#[doc = "Charger Detect Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1ChrgDetStatSet(pub u32);
impl Usb1ChrgDetStatSet {
    #[doc = "Battery Charging Data Contact Detection Phase Output."]
    #[must_use]
    #[inline(always)]
    pub const fn plug_contact(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Battery Charging Data Contact Detection Phase Output."]
    #[inline(always)]
    pub const fn set_plug_contact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Battery Charging Primary Detection Phase Output."]
    #[must_use]
    #[inline(always)]
    pub const fn chrg_detected(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Battery Charging Primary Detection Phase Output."]
    #[inline(always)]
    pub const fn set_chrg_detected(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DM Voltage."]
    #[must_use]
    #[inline(always)]
    pub const fn dm_state(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DM Voltage."]
    #[inline(always)]
    pub const fn set_dm_state(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DP Voltage."]
    #[must_use]
    #[inline(always)]
    pub const fn dp_state(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DP Voltage."]
    #[inline(always)]
    pub const fn set_dp_state(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Battery Charging Secondary Detection Phase Output."]
    #[must_use]
    #[inline(always)]
    pub const fn secdet_dcp(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Battery Charging Secondary Detection Phase Output."]
    #[inline(always)]
    pub const fn set_secdet_dcp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Usb1ChrgDetStatSet {
    #[inline(always)]
    fn default() -> Usb1ChrgDetStatSet {
        Usb1ChrgDetStatSet(0)
    }
}
impl core::fmt::Debug for Usb1ChrgDetStatSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1ChrgDetStatSet")
            .field("plug_contact", &self.plug_contact())
            .field("chrg_detected", &self.chrg_detected())
            .field("dm_state", &self.dm_state())
            .field("dp_state", &self.dp_state())
            .field("secdet_dcp", &self.secdet_dcp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1ChrgDetStatSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1ChrgDetStatSet {{ plug_contact: {=bool:?}, chrg_detected: {=bool:?}, dm_state: {=bool:?}, dp_state: {=bool:?}, secdet_dcp: {=bool:?} }}",
            self.plug_contact(),
            self.chrg_detected(),
            self.dm_state(),
            self.dp_state(),
            self.secdet_dcp()
        )
    }
}
#[doc = "Charger Detect Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1ChrgDetStatTog(pub u32);
impl Usb1ChrgDetStatTog {
    #[doc = "Battery Charging Data Contact Detection Phase Output."]
    #[must_use]
    #[inline(always)]
    pub const fn plug_contact(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Battery Charging Data Contact Detection Phase Output."]
    #[inline(always)]
    pub const fn set_plug_contact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Battery Charging Primary Detection Phase Output."]
    #[must_use]
    #[inline(always)]
    pub const fn chrg_detected(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Battery Charging Primary Detection Phase Output."]
    #[inline(always)]
    pub const fn set_chrg_detected(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DM Voltage."]
    #[must_use]
    #[inline(always)]
    pub const fn dm_state(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DM Voltage."]
    #[inline(always)]
    pub const fn set_dm_state(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DP Voltage."]
    #[must_use]
    #[inline(always)]
    pub const fn dp_state(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DP Voltage."]
    #[inline(always)]
    pub const fn set_dp_state(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Battery Charging Secondary Detection Phase Output."]
    #[must_use]
    #[inline(always)]
    pub const fn secdet_dcp(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Battery Charging Secondary Detection Phase Output."]
    #[inline(always)]
    pub const fn set_secdet_dcp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Usb1ChrgDetStatTog {
    #[inline(always)]
    fn default() -> Usb1ChrgDetStatTog {
        Usb1ChrgDetStatTog(0)
    }
}
impl core::fmt::Debug for Usb1ChrgDetStatTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1ChrgDetStatTog")
            .field("plug_contact", &self.plug_contact())
            .field("chrg_detected", &self.chrg_detected())
            .field("dm_state", &self.dm_state())
            .field("dp_state", &self.dp_state())
            .field("secdet_dcp", &self.secdet_dcp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1ChrgDetStatTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1ChrgDetStatTog {{ plug_contact: {=bool:?}, chrg_detected: {=bool:?}, dm_state: {=bool:?}, dp_state: {=bool:?}, secdet_dcp: {=bool:?} }}",
            self.plug_contact(),
            self.chrg_detected(),
            self.dm_state(),
            self.dp_state(),
            self.secdet_dcp()
        )
    }
}
#[doc = "Charger Detect."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1ChrgDetect(pub u32);
impl Usb1ChrgDetect {
    #[doc = "Secondary Detection Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn detect_sec(&self) -> DetectSec {
        let val = (self.0 >> 1usize) & 0x01;
        DetectSec::from_bits(val as u8)
    }
    #[doc = "Secondary Detection Function Enable."]
    #[inline(always)]
    pub const fn set_detect_sec(&mut self, val: DetectSec) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "DP Pullup Resistor Enable Override Control."]
    #[must_use]
    #[inline(always)]
    pub const fn pullup_dp(&self) -> PullupDp {
        let val = (self.0 >> 2usize) & 0x01;
        PullupDp::from_bits(val as u8)
    }
    #[doc = "DP Pullup Resistor Enable Override Control."]
    #[inline(always)]
    pub const fn set_pullup_dp(&mut self, val: PullupDp) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "VDM_SRC Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn vdm_src_enable(&self) -> VdmSrcEnable {
        let val = (self.0 >> 4usize) & 0x01;
        VdmSrcEnable::from_bits(val as u8)
    }
    #[doc = "VDM_SRC Function Enable."]
    #[inline(always)]
    pub const fn set_vdm_src_enable(&mut self, val: VdmSrcEnable) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "BC Data Contact Detect Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn chk_contact(&self) -> ChkContact {
        let val = (self.0 >> 18usize) & 0x01;
        ChkContact::from_bits(val as u8)
    }
    #[doc = "BC Data Contact Detect Function Enable."]
    #[inline(always)]
    pub const fn set_chk_contact(&mut self, val: ChkContact) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "BC Charger Detection Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn chk_chrg_b(&self) -> ChkChrgB {
        let val = (self.0 >> 19usize) & 0x01;
        ChkChrgB::from_bits(val as u8)
    }
    #[doc = "BC Charger Detection Function Enable."]
    #[inline(always)]
    pub const fn set_chk_chrg_b(&mut self, val: ChkChrgB) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Selection of BC v1.2 Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en_b(&self) -> EnB {
        let val = (self.0 >> 20usize) & 0x01;
        EnB::from_bits(val as u8)
    }
    #[doc = "Selection of BC v1.2 Function Enable."]
    #[inline(always)]
    pub const fn set_en_b(&mut self, val: EnB) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "DCD Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn dcdsel(&self) -> Dcdsel {
        let val = (self.0 >> 31usize) & 0x01;
        Dcdsel::from_bits(val as u8)
    }
    #[doc = "DCD Selection."]
    #[inline(always)]
    pub const fn set_dcdsel(&mut self, val: Dcdsel) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Usb1ChrgDetect {
    #[inline(always)]
    fn default() -> Usb1ChrgDetect {
        Usb1ChrgDetect(0)
    }
}
impl core::fmt::Debug for Usb1ChrgDetect {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1ChrgDetect")
            .field("detect_sec", &self.detect_sec())
            .field("pullup_dp", &self.pullup_dp())
            .field("vdm_src_enable", &self.vdm_src_enable())
            .field("chk_contact", &self.chk_contact())
            .field("chk_chrg_b", &self.chk_chrg_b())
            .field("en_b", &self.en_b())
            .field("dcdsel", &self.dcdsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1ChrgDetect {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1ChrgDetect {{ detect_sec: {:?}, pullup_dp: {:?}, vdm_src_enable: {:?}, chk_contact: {:?}, chk_chrg_b: {:?}, en_b: {:?}, dcdsel: {:?} }}",
            self.detect_sec(),
            self.pullup_dp(),
            self.vdm_src_enable(),
            self.chk_contact(),
            self.chk_chrg_b(),
            self.en_b(),
            self.dcdsel()
        )
    }
}
#[doc = "Charger Detect."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1ChrgDetectClr(pub u32);
impl Usb1ChrgDetectClr {
    #[doc = "Secondary Detection Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn detect_sec(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Secondary Detection Function Enable."]
    #[inline(always)]
    pub const fn set_detect_sec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DP Pullup Resistor Enable Override Control."]
    #[must_use]
    #[inline(always)]
    pub const fn pullup_dp(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DP Pullup Resistor Enable Override Control."]
    #[inline(always)]
    pub const fn set_pullup_dp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "VDM_SRC Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn vdm_src_enable(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "VDM_SRC Function Enable."]
    #[inline(always)]
    pub const fn set_vdm_src_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "BC Data Contact Detect Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn chk_contact(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "BC Data Contact Detect Function Enable."]
    #[inline(always)]
    pub const fn set_chk_contact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "BC Charger Detection Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn chk_chrg_b(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "BC Charger Detection Function Enable."]
    #[inline(always)]
    pub const fn set_chk_chrg_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Selection of BC v1.2 Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en_b(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Selection of BC v1.2 Function Enable."]
    #[inline(always)]
    pub const fn set_en_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DCD Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn dcdsel(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DCD Selection."]
    #[inline(always)]
    pub const fn set_dcdsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Usb1ChrgDetectClr {
    #[inline(always)]
    fn default() -> Usb1ChrgDetectClr {
        Usb1ChrgDetectClr(0)
    }
}
impl core::fmt::Debug for Usb1ChrgDetectClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1ChrgDetectClr")
            .field("detect_sec", &self.detect_sec())
            .field("pullup_dp", &self.pullup_dp())
            .field("vdm_src_enable", &self.vdm_src_enable())
            .field("chk_contact", &self.chk_contact())
            .field("chk_chrg_b", &self.chk_chrg_b())
            .field("en_b", &self.en_b())
            .field("dcdsel", &self.dcdsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1ChrgDetectClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1ChrgDetectClr {{ detect_sec: {=bool:?}, pullup_dp: {=bool:?}, vdm_src_enable: {=bool:?}, chk_contact: {=bool:?}, chk_chrg_b: {=bool:?}, en_b: {=bool:?}, dcdsel: {=bool:?} }}",
            self.detect_sec(),
            self.pullup_dp(),
            self.vdm_src_enable(),
            self.chk_contact(),
            self.chk_chrg_b(),
            self.en_b(),
            self.dcdsel()
        )
    }
}
#[doc = "Charger Detect."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1ChrgDetectSet(pub u32);
impl Usb1ChrgDetectSet {
    #[doc = "Secondary Detection Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn detect_sec(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Secondary Detection Function Enable."]
    #[inline(always)]
    pub const fn set_detect_sec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DP Pullup Resistor Enable Override Control."]
    #[must_use]
    #[inline(always)]
    pub const fn pullup_dp(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DP Pullup Resistor Enable Override Control."]
    #[inline(always)]
    pub const fn set_pullup_dp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "VDM_SRC Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn vdm_src_enable(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "VDM_SRC Function Enable."]
    #[inline(always)]
    pub const fn set_vdm_src_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "BC Data Contact Detect Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn chk_contact(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "BC Data Contact Detect Function Enable."]
    #[inline(always)]
    pub const fn set_chk_contact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "BC Charger Detection Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn chk_chrg_b(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "BC Charger Detection Function Enable."]
    #[inline(always)]
    pub const fn set_chk_chrg_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Selection of BC v1.2 Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en_b(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Selection of BC v1.2 Function Enable."]
    #[inline(always)]
    pub const fn set_en_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DCD Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn dcdsel(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DCD Selection."]
    #[inline(always)]
    pub const fn set_dcdsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Usb1ChrgDetectSet {
    #[inline(always)]
    fn default() -> Usb1ChrgDetectSet {
        Usb1ChrgDetectSet(0)
    }
}
impl core::fmt::Debug for Usb1ChrgDetectSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1ChrgDetectSet")
            .field("detect_sec", &self.detect_sec())
            .field("pullup_dp", &self.pullup_dp())
            .field("vdm_src_enable", &self.vdm_src_enable())
            .field("chk_contact", &self.chk_contact())
            .field("chk_chrg_b", &self.chk_chrg_b())
            .field("en_b", &self.en_b())
            .field("dcdsel", &self.dcdsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1ChrgDetectSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1ChrgDetectSet {{ detect_sec: {=bool:?}, pullup_dp: {=bool:?}, vdm_src_enable: {=bool:?}, chk_contact: {=bool:?}, chk_chrg_b: {=bool:?}, en_b: {=bool:?}, dcdsel: {=bool:?} }}",
            self.detect_sec(),
            self.pullup_dp(),
            self.vdm_src_enable(),
            self.chk_contact(),
            self.chk_chrg_b(),
            self.en_b(),
            self.dcdsel()
        )
    }
}
#[doc = "Charger Detect."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1ChrgDetectTog(pub u32);
impl Usb1ChrgDetectTog {
    #[doc = "Secondary Detection Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn detect_sec(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Secondary Detection Function Enable."]
    #[inline(always)]
    pub const fn set_detect_sec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DP Pullup Resistor Enable Override Control."]
    #[must_use]
    #[inline(always)]
    pub const fn pullup_dp(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DP Pullup Resistor Enable Override Control."]
    #[inline(always)]
    pub const fn set_pullup_dp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "VDM_SRC Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn vdm_src_enable(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "VDM_SRC Function Enable."]
    #[inline(always)]
    pub const fn set_vdm_src_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "BC Data Contact Detect Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn chk_contact(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "BC Data Contact Detect Function Enable."]
    #[inline(always)]
    pub const fn set_chk_contact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "BC Charger Detection Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn chk_chrg_b(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "BC Charger Detection Function Enable."]
    #[inline(always)]
    pub const fn set_chk_chrg_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Selection of BC v1.2 Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en_b(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Selection of BC v1.2 Function Enable."]
    #[inline(always)]
    pub const fn set_en_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DCD Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn dcdsel(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DCD Selection."]
    #[inline(always)]
    pub const fn set_dcdsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Usb1ChrgDetectTog {
    #[inline(always)]
    fn default() -> Usb1ChrgDetectTog {
        Usb1ChrgDetectTog(0)
    }
}
impl core::fmt::Debug for Usb1ChrgDetectTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1ChrgDetectTog")
            .field("detect_sec", &self.detect_sec())
            .field("pullup_dp", &self.pullup_dp())
            .field("vdm_src_enable", &self.vdm_src_enable())
            .field("chk_contact", &self.chk_contact())
            .field("chk_chrg_b", &self.chk_chrg_b())
            .field("en_b", &self.en_b())
            .field("dcdsel", &self.dcdsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1ChrgDetectTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1ChrgDetectTog {{ detect_sec: {=bool:?}, pullup_dp: {=bool:?}, vdm_src_enable: {=bool:?}, chk_contact: {=bool:?}, chk_chrg_b: {=bool:?}, en_b: {=bool:?}, dcdsel: {=bool:?} }}",
            self.detect_sec(),
            self.pullup_dp(),
            self.vdm_src_enable(),
            self.chk_contact(),
            self.chk_chrg_b(),
            self.en_b(),
            self.dcdsel()
        )
    }
}
#[doc = "VBUS Detect Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1VbusDetStat(pub u32);
impl Usb1VbusDetStat {
    #[doc = "Session End Indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn sessend(&self) -> Sessend {
        let val = (self.0 >> 0usize) & 0x01;
        Sessend::from_bits(val as u8)
    }
    #[doc = "Session End Indicator."]
    #[inline(always)]
    pub const fn set_sessend(&mut self, val: Sessend) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "B-Device Session Valid Status."]
    #[must_use]
    #[inline(always)]
    pub const fn bvalid(&self) -> Bvalid {
        let val = (self.0 >> 1usize) & 0x01;
        Bvalid::from_bits(val as u8)
    }
    #[doc = "B-Device Session Valid Status."]
    #[inline(always)]
    pub const fn set_bvalid(&mut self, val: Bvalid) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "A-Device Session Valid Status."]
    #[must_use]
    #[inline(always)]
    pub const fn avalid(&self) -> Avalid {
        let val = (self.0 >> 2usize) & 0x01;
        Avalid::from_bits(val as u8)
    }
    #[doc = "A-Device Session Valid Status."]
    #[inline(always)]
    pub const fn set_avalid(&mut self, val: Avalid) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "VBUS Voltage Status."]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_valid(&self) -> VbusValid {
        let val = (self.0 >> 3usize) & 0x01;
        VbusValid::from_bits(val as u8)
    }
    #[doc = "VBUS Voltage Status."]
    #[inline(always)]
    pub const fn set_vbus_valid(&mut self, val: VbusValid) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "VBUS_VALID_3V Detector Status."]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_valid_3v(&self) -> VbusValid3v {
        let val = (self.0 >> 4usize) & 0x01;
        VbusValid3v::from_bits(val as u8)
    }
    #[doc = "VBUS_VALID_3V Detector Status."]
    #[inline(always)]
    pub const fn set_vbus_valid_3v(&mut self, val: VbusValid3v) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "OTG ID External Override Status."]
    #[must_use]
    #[inline(always)]
    pub const fn ext_id(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID External Override Status."]
    #[inline(always)]
    pub const fn set_ext_id(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Usb1VbusDetStat {
    #[inline(always)]
    fn default() -> Usb1VbusDetStat {
        Usb1VbusDetStat(0)
    }
}
impl core::fmt::Debug for Usb1VbusDetStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1VbusDetStat")
            .field("sessend", &self.sessend())
            .field("bvalid", &self.bvalid())
            .field("avalid", &self.avalid())
            .field("vbus_valid", &self.vbus_valid())
            .field("vbus_valid_3v", &self.vbus_valid_3v())
            .field("ext_id", &self.ext_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1VbusDetStat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1VbusDetStat {{ sessend: {:?}, bvalid: {:?}, avalid: {:?}, vbus_valid: {:?}, vbus_valid_3v: {:?}, ext_id: {=bool:?} }}",
            self.sessend(),
            self.bvalid(),
            self.avalid(),
            self.vbus_valid(),
            self.vbus_valid_3v(),
            self.ext_id()
        )
    }
}
#[doc = "VBUS Detect Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1VbusDetStatClr(pub u32);
impl Usb1VbusDetStatClr {
    #[doc = "Session End Indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn sessend(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Session End Indicator."]
    #[inline(always)]
    pub const fn set_sessend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "B-Device Session Valid Status."]
    #[must_use]
    #[inline(always)]
    pub const fn bvalid(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "B-Device Session Valid Status."]
    #[inline(always)]
    pub const fn set_bvalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "A-Device Session Valid Status."]
    #[must_use]
    #[inline(always)]
    pub const fn avalid(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "A-Device Session Valid Status."]
    #[inline(always)]
    pub const fn set_avalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "VBUS Voltage Status."]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_valid(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Voltage Status."]
    #[inline(always)]
    pub const fn set_vbus_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "VBUS_VALID_3V Detector Status."]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_valid_3v(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS_VALID_3V Detector Status."]
    #[inline(always)]
    pub const fn set_vbus_valid_3v(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "OTG ID External Override Status."]
    #[must_use]
    #[inline(always)]
    pub const fn ext_id(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID External Override Status."]
    #[inline(always)]
    pub const fn set_ext_id(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Usb1VbusDetStatClr {
    #[inline(always)]
    fn default() -> Usb1VbusDetStatClr {
        Usb1VbusDetStatClr(0)
    }
}
impl core::fmt::Debug for Usb1VbusDetStatClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1VbusDetStatClr")
            .field("sessend", &self.sessend())
            .field("bvalid", &self.bvalid())
            .field("avalid", &self.avalid())
            .field("vbus_valid", &self.vbus_valid())
            .field("vbus_valid_3v", &self.vbus_valid_3v())
            .field("ext_id", &self.ext_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1VbusDetStatClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1VbusDetStatClr {{ sessend: {=bool:?}, bvalid: {=bool:?}, avalid: {=bool:?}, vbus_valid: {=bool:?}, vbus_valid_3v: {=bool:?}, ext_id: {=bool:?} }}",
            self.sessend(),
            self.bvalid(),
            self.avalid(),
            self.vbus_valid(),
            self.vbus_valid_3v(),
            self.ext_id()
        )
    }
}
#[doc = "VBUS Detect Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1VbusDetStatSet(pub u32);
impl Usb1VbusDetStatSet {
    #[doc = "Session End Indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn sessend(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Session End Indicator."]
    #[inline(always)]
    pub const fn set_sessend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "B-Device Session Valid Status."]
    #[must_use]
    #[inline(always)]
    pub const fn bvalid(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "B-Device Session Valid Status."]
    #[inline(always)]
    pub const fn set_bvalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "A-Device Session Valid Status."]
    #[must_use]
    #[inline(always)]
    pub const fn avalid(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "A-Device Session Valid Status."]
    #[inline(always)]
    pub const fn set_avalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "VBUS Voltage Status."]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_valid(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Voltage Status."]
    #[inline(always)]
    pub const fn set_vbus_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "VBUS_VALID_3V Detector Status."]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_valid_3v(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS_VALID_3V Detector Status."]
    #[inline(always)]
    pub const fn set_vbus_valid_3v(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "OTG ID External Override Status."]
    #[must_use]
    #[inline(always)]
    pub const fn ext_id(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID External Override Status."]
    #[inline(always)]
    pub const fn set_ext_id(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Usb1VbusDetStatSet {
    #[inline(always)]
    fn default() -> Usb1VbusDetStatSet {
        Usb1VbusDetStatSet(0)
    }
}
impl core::fmt::Debug for Usb1VbusDetStatSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1VbusDetStatSet")
            .field("sessend", &self.sessend())
            .field("bvalid", &self.bvalid())
            .field("avalid", &self.avalid())
            .field("vbus_valid", &self.vbus_valid())
            .field("vbus_valid_3v", &self.vbus_valid_3v())
            .field("ext_id", &self.ext_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1VbusDetStatSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1VbusDetStatSet {{ sessend: {=bool:?}, bvalid: {=bool:?}, avalid: {=bool:?}, vbus_valid: {=bool:?}, vbus_valid_3v: {=bool:?}, ext_id: {=bool:?} }}",
            self.sessend(),
            self.bvalid(),
            self.avalid(),
            self.vbus_valid(),
            self.vbus_valid_3v(),
            self.ext_id()
        )
    }
}
#[doc = "VBUS Detect Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1VbusDetStatTog(pub u32);
impl Usb1VbusDetStatTog {
    #[doc = "Session End Indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn sessend(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Session End Indicator."]
    #[inline(always)]
    pub const fn set_sessend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "B-Device Session Valid Status."]
    #[must_use]
    #[inline(always)]
    pub const fn bvalid(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "B-Device Session Valid Status."]
    #[inline(always)]
    pub const fn set_bvalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "A-Device Session Valid Status."]
    #[must_use]
    #[inline(always)]
    pub const fn avalid(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "A-Device Session Valid Status."]
    #[inline(always)]
    pub const fn set_avalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "VBUS Voltage Status."]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_valid(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Voltage Status."]
    #[inline(always)]
    pub const fn set_vbus_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "VBUS_VALID_3V Detector Status."]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_valid_3v(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS_VALID_3V Detector Status."]
    #[inline(always)]
    pub const fn set_vbus_valid_3v(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "OTG ID External Override Status."]
    #[must_use]
    #[inline(always)]
    pub const fn ext_id(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "OTG ID External Override Status."]
    #[inline(always)]
    pub const fn set_ext_id(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Usb1VbusDetStatTog {
    #[inline(always)]
    fn default() -> Usb1VbusDetStatTog {
        Usb1VbusDetStatTog(0)
    }
}
impl core::fmt::Debug for Usb1VbusDetStatTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1VbusDetStatTog")
            .field("sessend", &self.sessend())
            .field("bvalid", &self.bvalid())
            .field("avalid", &self.avalid())
            .field("vbus_valid", &self.vbus_valid())
            .field("vbus_valid_3v", &self.vbus_valid_3v())
            .field("ext_id", &self.ext_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1VbusDetStatTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1VbusDetStatTog {{ sessend: {=bool:?}, bvalid: {=bool:?}, avalid: {=bool:?}, vbus_valid: {=bool:?}, vbus_valid_3v: {=bool:?}, ext_id: {=bool:?} }}",
            self.sessend(),
            self.bvalid(),
            self.avalid(),
            self.vbus_valid(),
            self.vbus_valid_3v(),
            self.ext_id()
        )
    }
}
#[doc = "VBUS Detect."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1VbusDetect(pub u32);
impl Usb1VbusDetect {
    #[doc = "VBUS Comparator Threshold."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_thresh(&self) -> VbusvalidThresh {
        let val = (self.0 >> 0usize) & 0x07;
        VbusvalidThresh::from_bits(val as u8)
    }
    #[doc = "VBUS Comparator Threshold."]
    #[inline(always)]
    pub const fn set_vbusvalid_thresh(&mut self, val: VbusvalidThresh) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "VBUS Detect Signal Local Override Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_override_en(&self) -> VbusOverrideEn {
        let val = (self.0 >> 3usize) & 0x01;
        VbusOverrideEn::from_bits(val as u8)
    }
    #[doc = "VBUS Detect Signal Local Override Enable."]
    #[inline(always)]
    pub const fn set_vbus_override_en(&mut self, val: VbusOverrideEn) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Override Value for SESSEND."]
    #[must_use]
    #[inline(always)]
    pub const fn sessend_override(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for SESSEND."]
    #[inline(always)]
    pub const fn set_sessend_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Override Value for B-Device Session Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn bvalid_override(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for B-Device Session Valid."]
    #[inline(always)]
    pub const fn set_bvalid_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Override Value for A-Device Session Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn avalid_override(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for A-Device Session Valid."]
    #[inline(always)]
    pub const fn set_avalid_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Override Value for the VBUS_VALID Signal."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_override(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for the VBUS_VALID Signal."]
    #[inline(always)]
    pub const fn set_vbusvalid_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "VBUS_VALID Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_sel(&self) -> VbusvalidSel {
        let val = (self.0 >> 8usize) & 0x01;
        VbusvalidSel::from_bits(val as u8)
    }
    #[doc = "VBUS_VALID Selection."]
    #[inline(always)]
    pub const fn set_vbusvalid_sel(&mut self, val: VbusvalidSel) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "VBUS_VALID Source Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_source_sel(&self) -> VbusSourceSel {
        let val = (self.0 >> 9usize) & 0x03;
        VbusSourceSel::from_bits(val as u8)
    }
    #[doc = "VBUS_VALID Source Selection."]
    #[inline(always)]
    pub const fn set_vbus_source_sel(&mut self, val: VbusSourceSel) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
    }
    #[doc = "Enable Local ID Pin Status Override."]
    #[must_use]
    #[inline(always)]
    pub const fn id_override_en(&self) -> IdOverrideEn {
        let val = (self.0 >> 11usize) & 0x01;
        IdOverrideEn::from_bits(val as u8)
    }
    #[doc = "Enable Local ID Pin Status Override."]
    #[inline(always)]
    pub const fn set_id_override_en(&mut self, val: IdOverrideEn) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "ID Pin Status Local Override."]
    #[must_use]
    #[inline(always)]
    pub const fn id_override(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "ID Pin Status Local Override."]
    #[inline(always)]
    pub const fn set_id_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "External ID Override Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ext_id_override_en(&self) -> ExtIdOverrideEn {
        let val = (self.0 >> 13usize) & 0x01;
        ExtIdOverrideEn::from_bits(val as u8)
    }
    #[doc = "External ID Override Enable."]
    #[inline(always)]
    pub const fn set_ext_id_override_en(&mut self, val: ExtIdOverrideEn) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "External VBUS Override Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ext_vbus_override_en(&self) -> ExtVbusOverrideEn {
        let val = (self.0 >> 14usize) & 0x01;
        ExtVbusOverrideEn::from_bits(val as u8)
    }
    #[doc = "External VBUS Override Enable."]
    #[inline(always)]
    pub const fn set_ext_vbus_override_en(&mut self, val: ExtVbusOverrideEn) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "VBUS_VALID Comparator Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_to_b(&self) -> VbusvalidToB {
        let val = (self.0 >> 18usize) & 0x01;
        VbusvalidToB::from_bits(val as u8)
    }
    #[doc = "VBUS_VALID Comparator Selection."]
    #[inline(always)]
    pub const fn set_vbusvalid_to_b(&mut self, val: VbusvalidToB) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "VBUS_VALID Comparator Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_pwrup_cmps(&self) -> VbusvalidPwrupCmps {
        let val = (self.0 >> 20usize) & 0x07;
        VbusvalidPwrupCmps::from_bits(val as u8)
    }
    #[doc = "VBUS_VALID Comparator Enable."]
    #[inline(always)]
    pub const fn set_vbusvalid_pwrup_cmps(&mut self, val: VbusvalidPwrupCmps) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "VBUS Discharge Resistor."]
    #[must_use]
    #[inline(always)]
    pub const fn discharge_vbus(&self) -> DischargeVbus {
        let val = (self.0 >> 26usize) & 0x01;
        DischargeVbus::from_bits(val as u8)
    }
    #[doc = "VBUS Discharge Resistor."]
    #[inline(always)]
    pub const fn set_discharge_vbus(&mut self, val: DischargeVbus) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
}
impl Default for Usb1VbusDetect {
    #[inline(always)]
    fn default() -> Usb1VbusDetect {
        Usb1VbusDetect(0)
    }
}
impl core::fmt::Debug for Usb1VbusDetect {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1VbusDetect")
            .field("vbusvalid_thresh", &self.vbusvalid_thresh())
            .field("vbus_override_en", &self.vbus_override_en())
            .field("sessend_override", &self.sessend_override())
            .field("bvalid_override", &self.bvalid_override())
            .field("avalid_override", &self.avalid_override())
            .field("vbusvalid_override", &self.vbusvalid_override())
            .field("vbusvalid_sel", &self.vbusvalid_sel())
            .field("vbus_source_sel", &self.vbus_source_sel())
            .field("id_override_en", &self.id_override_en())
            .field("id_override", &self.id_override())
            .field("ext_id_override_en", &self.ext_id_override_en())
            .field("ext_vbus_override_en", &self.ext_vbus_override_en())
            .field("vbusvalid_to_b", &self.vbusvalid_to_b())
            .field("vbusvalid_pwrup_cmps", &self.vbusvalid_pwrup_cmps())
            .field("discharge_vbus", &self.discharge_vbus())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1VbusDetect {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1VbusDetect {{ vbusvalid_thresh: {:?}, vbus_override_en: {:?}, sessend_override: {=bool:?}, bvalid_override: {=bool:?}, avalid_override: {=bool:?}, vbusvalid_override: {=bool:?}, vbusvalid_sel: {:?}, vbus_source_sel: {:?}, id_override_en: {:?}, id_override: {=bool:?}, ext_id_override_en: {:?}, ext_vbus_override_en: {:?}, vbusvalid_to_b: {:?}, vbusvalid_pwrup_cmps: {:?}, discharge_vbus: {:?} }}",
            self.vbusvalid_thresh(),
            self.vbus_override_en(),
            self.sessend_override(),
            self.bvalid_override(),
            self.avalid_override(),
            self.vbusvalid_override(),
            self.vbusvalid_sel(),
            self.vbus_source_sel(),
            self.id_override_en(),
            self.id_override(),
            self.ext_id_override_en(),
            self.ext_vbus_override_en(),
            self.vbusvalid_to_b(),
            self.vbusvalid_pwrup_cmps(),
            self.discharge_vbus()
        )
    }
}
#[doc = "VBUS Detect."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1VbusDetectClr(pub u32);
impl Usb1VbusDetectClr {
    #[doc = "VBUS Comparator Threshold."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_thresh(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "VBUS Comparator Threshold."]
    #[inline(always)]
    pub const fn set_vbusvalid_thresh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "VBUS Detect Signal Local Override Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_override_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Detect Signal Local Override Enable."]
    #[inline(always)]
    pub const fn set_vbus_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Override Value for SESSEND."]
    #[must_use]
    #[inline(always)]
    pub const fn sessend_override(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for SESSEND."]
    #[inline(always)]
    pub const fn set_sessend_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Override Value for B-Device Session Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn bvalid_override(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for B-Device Session Valid."]
    #[inline(always)]
    pub const fn set_bvalid_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Override Value for A-Device Session Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn avalid_override(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for A-Device Session Valid."]
    #[inline(always)]
    pub const fn set_avalid_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Override Value for the VBUS_VALID Signal."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_override(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for the VBUS_VALID Signal."]
    #[inline(always)]
    pub const fn set_vbusvalid_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "VBUS_VALID Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_sel(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS_VALID Selection."]
    #[inline(always)]
    pub const fn set_vbusvalid_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "VBUS_VALID Source Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_source_sel(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "VBUS_VALID Source Selection."]
    #[inline(always)]
    pub const fn set_vbus_source_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[doc = "Enable Local ID Pin Status Override."]
    #[must_use]
    #[inline(always)]
    pub const fn id_override_en(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Local ID Pin Status Override."]
    #[inline(always)]
    pub const fn set_id_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "ID Pin Status Local Override."]
    #[must_use]
    #[inline(always)]
    pub const fn id_override(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "ID Pin Status Local Override."]
    #[inline(always)]
    pub const fn set_id_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "External ID Override Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ext_id_override_en(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "External ID Override Enable."]
    #[inline(always)]
    pub const fn set_ext_id_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "External VBUS Override Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ext_vbus_override_en(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "External VBUS Override Enable."]
    #[inline(always)]
    pub const fn set_ext_vbus_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "VBUS_VALID Comparator Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_to_b(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS_VALID Comparator Selection."]
    #[inline(always)]
    pub const fn set_vbusvalid_to_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "VBUS_VALID Comparator Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_pwrup_cmps(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "VBUS_VALID Comparator Enable."]
    #[inline(always)]
    pub const fn set_vbusvalid_pwrup_cmps(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[doc = "VBUS Discharge Resistor."]
    #[must_use]
    #[inline(always)]
    pub const fn discharge_vbus(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Discharge Resistor."]
    #[inline(always)]
    pub const fn set_discharge_vbus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for Usb1VbusDetectClr {
    #[inline(always)]
    fn default() -> Usb1VbusDetectClr {
        Usb1VbusDetectClr(0)
    }
}
impl core::fmt::Debug for Usb1VbusDetectClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1VbusDetectClr")
            .field("vbusvalid_thresh", &self.vbusvalid_thresh())
            .field("vbus_override_en", &self.vbus_override_en())
            .field("sessend_override", &self.sessend_override())
            .field("bvalid_override", &self.bvalid_override())
            .field("avalid_override", &self.avalid_override())
            .field("vbusvalid_override", &self.vbusvalid_override())
            .field("vbusvalid_sel", &self.vbusvalid_sel())
            .field("vbus_source_sel", &self.vbus_source_sel())
            .field("id_override_en", &self.id_override_en())
            .field("id_override", &self.id_override())
            .field("ext_id_override_en", &self.ext_id_override_en())
            .field("ext_vbus_override_en", &self.ext_vbus_override_en())
            .field("vbusvalid_to_b", &self.vbusvalid_to_b())
            .field("vbusvalid_pwrup_cmps", &self.vbusvalid_pwrup_cmps())
            .field("discharge_vbus", &self.discharge_vbus())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1VbusDetectClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1VbusDetectClr {{ vbusvalid_thresh: {=u8:?}, vbus_override_en: {=bool:?}, sessend_override: {=bool:?}, bvalid_override: {=bool:?}, avalid_override: {=bool:?}, vbusvalid_override: {=bool:?}, vbusvalid_sel: {=bool:?}, vbus_source_sel: {=u8:?}, id_override_en: {=bool:?}, id_override: {=bool:?}, ext_id_override_en: {=bool:?}, ext_vbus_override_en: {=bool:?}, vbusvalid_to_b: {=bool:?}, vbusvalid_pwrup_cmps: {=u8:?}, discharge_vbus: {=bool:?} }}",
            self.vbusvalid_thresh(),
            self.vbus_override_en(),
            self.sessend_override(),
            self.bvalid_override(),
            self.avalid_override(),
            self.vbusvalid_override(),
            self.vbusvalid_sel(),
            self.vbus_source_sel(),
            self.id_override_en(),
            self.id_override(),
            self.ext_id_override_en(),
            self.ext_vbus_override_en(),
            self.vbusvalid_to_b(),
            self.vbusvalid_pwrup_cmps(),
            self.discharge_vbus()
        )
    }
}
#[doc = "VBUS Detect."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1VbusDetectSet(pub u32);
impl Usb1VbusDetectSet {
    #[doc = "VBUS Comparator Threshold."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_thresh(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "VBUS Comparator Threshold."]
    #[inline(always)]
    pub const fn set_vbusvalid_thresh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "VBUS Detect Signal Local Override Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_override_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Detect Signal Local Override Enable."]
    #[inline(always)]
    pub const fn set_vbus_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Override Value for SESSEND."]
    #[must_use]
    #[inline(always)]
    pub const fn sessend_override(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for SESSEND."]
    #[inline(always)]
    pub const fn set_sessend_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Override Value for B-Device Session Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn bvalid_override(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for B-Device Session Valid."]
    #[inline(always)]
    pub const fn set_bvalid_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Override Value for A-Device Session Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn avalid_override(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for A-Device Session Valid."]
    #[inline(always)]
    pub const fn set_avalid_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Override Value for the VBUS_VALID Signal."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_override(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for the VBUS_VALID Signal."]
    #[inline(always)]
    pub const fn set_vbusvalid_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "VBUS_VALID Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_sel(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS_VALID Selection."]
    #[inline(always)]
    pub const fn set_vbusvalid_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "VBUS_VALID Source Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_source_sel(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "VBUS_VALID Source Selection."]
    #[inline(always)]
    pub const fn set_vbus_source_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[doc = "Enable Local ID Pin Status Override."]
    #[must_use]
    #[inline(always)]
    pub const fn id_override_en(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Local ID Pin Status Override."]
    #[inline(always)]
    pub const fn set_id_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "ID Pin Status Local Override."]
    #[must_use]
    #[inline(always)]
    pub const fn id_override(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "ID Pin Status Local Override."]
    #[inline(always)]
    pub const fn set_id_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "External ID Override Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ext_id_override_en(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "External ID Override Enable."]
    #[inline(always)]
    pub const fn set_ext_id_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "External VBUS Override Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ext_vbus_override_en(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "External VBUS Override Enable."]
    #[inline(always)]
    pub const fn set_ext_vbus_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "VBUS_VALID Comparator Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_to_b(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS_VALID Comparator Selection."]
    #[inline(always)]
    pub const fn set_vbusvalid_to_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "VBUS_VALID Comparator Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_pwrup_cmps(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "VBUS_VALID Comparator Enable."]
    #[inline(always)]
    pub const fn set_vbusvalid_pwrup_cmps(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[doc = "VBUS Discharge Resistor."]
    #[must_use]
    #[inline(always)]
    pub const fn discharge_vbus(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Discharge Resistor."]
    #[inline(always)]
    pub const fn set_discharge_vbus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for Usb1VbusDetectSet {
    #[inline(always)]
    fn default() -> Usb1VbusDetectSet {
        Usb1VbusDetectSet(0)
    }
}
impl core::fmt::Debug for Usb1VbusDetectSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1VbusDetectSet")
            .field("vbusvalid_thresh", &self.vbusvalid_thresh())
            .field("vbus_override_en", &self.vbus_override_en())
            .field("sessend_override", &self.sessend_override())
            .field("bvalid_override", &self.bvalid_override())
            .field("avalid_override", &self.avalid_override())
            .field("vbusvalid_override", &self.vbusvalid_override())
            .field("vbusvalid_sel", &self.vbusvalid_sel())
            .field("vbus_source_sel", &self.vbus_source_sel())
            .field("id_override_en", &self.id_override_en())
            .field("id_override", &self.id_override())
            .field("ext_id_override_en", &self.ext_id_override_en())
            .field("ext_vbus_override_en", &self.ext_vbus_override_en())
            .field("vbusvalid_to_b", &self.vbusvalid_to_b())
            .field("vbusvalid_pwrup_cmps", &self.vbusvalid_pwrup_cmps())
            .field("discharge_vbus", &self.discharge_vbus())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1VbusDetectSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1VbusDetectSet {{ vbusvalid_thresh: {=u8:?}, vbus_override_en: {=bool:?}, sessend_override: {=bool:?}, bvalid_override: {=bool:?}, avalid_override: {=bool:?}, vbusvalid_override: {=bool:?}, vbusvalid_sel: {=bool:?}, vbus_source_sel: {=u8:?}, id_override_en: {=bool:?}, id_override: {=bool:?}, ext_id_override_en: {=bool:?}, ext_vbus_override_en: {=bool:?}, vbusvalid_to_b: {=bool:?}, vbusvalid_pwrup_cmps: {=u8:?}, discharge_vbus: {=bool:?} }}",
            self.vbusvalid_thresh(),
            self.vbus_override_en(),
            self.sessend_override(),
            self.bvalid_override(),
            self.avalid_override(),
            self.vbusvalid_override(),
            self.vbusvalid_sel(),
            self.vbus_source_sel(),
            self.id_override_en(),
            self.id_override(),
            self.ext_id_override_en(),
            self.ext_vbus_override_en(),
            self.vbusvalid_to_b(),
            self.vbusvalid_pwrup_cmps(),
            self.discharge_vbus()
        )
    }
}
#[doc = "VBUS Detect."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb1VbusDetectTog(pub u32);
impl Usb1VbusDetectTog {
    #[doc = "VBUS Comparator Threshold."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_thresh(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "VBUS Comparator Threshold."]
    #[inline(always)]
    pub const fn set_vbusvalid_thresh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "VBUS Detect Signal Local Override Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_override_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Detect Signal Local Override Enable."]
    #[inline(always)]
    pub const fn set_vbus_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Override Value for SESSEND."]
    #[must_use]
    #[inline(always)]
    pub const fn sessend_override(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for SESSEND."]
    #[inline(always)]
    pub const fn set_sessend_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Override Value for B-Device Session Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn bvalid_override(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for B-Device Session Valid."]
    #[inline(always)]
    pub const fn set_bvalid_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Override Value for A-Device Session Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn avalid_override(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for A-Device Session Valid."]
    #[inline(always)]
    pub const fn set_avalid_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Override Value for the VBUS_VALID Signal."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_override(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Override Value for the VBUS_VALID Signal."]
    #[inline(always)]
    pub const fn set_vbusvalid_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "VBUS_VALID Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_sel(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS_VALID Selection."]
    #[inline(always)]
    pub const fn set_vbusvalid_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "VBUS_VALID Source Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_source_sel(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "VBUS_VALID Source Selection."]
    #[inline(always)]
    pub const fn set_vbus_source_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[doc = "Enable Local ID Pin Status Override."]
    #[must_use]
    #[inline(always)]
    pub const fn id_override_en(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Local ID Pin Status Override."]
    #[inline(always)]
    pub const fn set_id_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "ID Pin Status Local Override."]
    #[must_use]
    #[inline(always)]
    pub const fn id_override(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "ID Pin Status Local Override."]
    #[inline(always)]
    pub const fn set_id_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "External ID Override Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ext_id_override_en(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "External ID Override Enable."]
    #[inline(always)]
    pub const fn set_ext_id_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "External VBUS Override Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ext_vbus_override_en(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "External VBUS Override Enable."]
    #[inline(always)]
    pub const fn set_ext_vbus_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "VBUS_VALID Comparator Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_to_b(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS_VALID Comparator Selection."]
    #[inline(always)]
    pub const fn set_vbusvalid_to_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "VBUS_VALID Comparator Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn vbusvalid_pwrup_cmps(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "VBUS_VALID Comparator Enable."]
    #[inline(always)]
    pub const fn set_vbusvalid_pwrup_cmps(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[doc = "VBUS Discharge Resistor."]
    #[must_use]
    #[inline(always)]
    pub const fn discharge_vbus(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Discharge Resistor."]
    #[inline(always)]
    pub const fn set_discharge_vbus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for Usb1VbusDetectTog {
    #[inline(always)]
    fn default() -> Usb1VbusDetectTog {
        Usb1VbusDetectTog(0)
    }
}
impl core::fmt::Debug for Usb1VbusDetectTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb1VbusDetectTog")
            .field("vbusvalid_thresh", &self.vbusvalid_thresh())
            .field("vbus_override_en", &self.vbus_override_en())
            .field("sessend_override", &self.sessend_override())
            .field("bvalid_override", &self.bvalid_override())
            .field("avalid_override", &self.avalid_override())
            .field("vbusvalid_override", &self.vbusvalid_override())
            .field("vbusvalid_sel", &self.vbusvalid_sel())
            .field("vbus_source_sel", &self.vbus_source_sel())
            .field("id_override_en", &self.id_override_en())
            .field("id_override", &self.id_override())
            .field("ext_id_override_en", &self.ext_id_override_en())
            .field("ext_vbus_override_en", &self.ext_vbus_override_en())
            .field("vbusvalid_to_b", &self.vbusvalid_to_b())
            .field("vbusvalid_pwrup_cmps", &self.vbusvalid_pwrup_cmps())
            .field("discharge_vbus", &self.discharge_vbus())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb1VbusDetectTog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb1VbusDetectTog {{ vbusvalid_thresh: {=u8:?}, vbus_override_en: {=bool:?}, sessend_override: {=bool:?}, bvalid_override: {=bool:?}, avalid_override: {=bool:?}, vbusvalid_override: {=bool:?}, vbusvalid_sel: {=bool:?}, vbus_source_sel: {=u8:?}, id_override_en: {=bool:?}, id_override: {=bool:?}, ext_id_override_en: {=bool:?}, ext_vbus_override_en: {=bool:?}, vbusvalid_to_b: {=bool:?}, vbusvalid_pwrup_cmps: {=u8:?}, discharge_vbus: {=bool:?} }}",
            self.vbusvalid_thresh(),
            self.vbus_override_en(),
            self.sessend_override(),
            self.bvalid_override(),
            self.avalid_override(),
            self.vbusvalid_override(),
            self.vbusvalid_sel(),
            self.vbus_source_sel(),
            self.id_override_en(),
            self.id_override(),
            self.ext_id_override_en(),
            self.ext_vbus_override_en(),
            self.vbusvalid_to_b(),
            self.vbusvalid_pwrup_cmps(),
            self.discharge_vbus()
        )
    }
}
#[doc = "Version."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Version(pub u32);
impl Version {
    #[doc = "Step."]
    #[must_use]
    #[inline(always)]
    pub const fn step(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Step."]
    #[inline(always)]
    pub const fn set_step(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Minor."]
    #[must_use]
    #[inline(always)]
    pub const fn minor(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Minor."]
    #[inline(always)]
    pub const fn set_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Major."]
    #[must_use]
    #[inline(always)]
    pub const fn major(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Major."]
    #[inline(always)]
    pub const fn set_major(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Version {
    #[inline(always)]
    fn default() -> Version {
        Version(0)
    }
}
impl core::fmt::Debug for Version {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Version")
            .field("step", &self.step())
            .field("minor", &self.minor())
            .field("major", &self.major())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Version {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Version {{ step: {=u16:?}, minor: {=u8:?}, major: {=u8:?} }}",
            self.step(),
            self.minor(),
            self.major()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Avalid {
    #[doc = "Below threshold."]
    AvalidLo = 0x0,
    #[doc = "Above threshold."]
    AvalidHi = 0x01,
}
impl Avalid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Avalid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Avalid {
    #[inline(always)]
    fn from(val: u8) -> Avalid {
        Avalid::from_bits(val)
    }
}
impl From<Avalid> for u8 {
    #[inline(always)]
    fn from(val: Avalid) -> u8 {
        Avalid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bvalid {
    #[doc = "Below threshold."]
    BvalidLo = 0x0,
    #[doc = "Above threshold."]
    BvalidHi = 0x01,
}
impl Bvalid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bvalid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bvalid {
    #[inline(always)]
    fn from(val: u8) -> Bvalid {
        Bvalid::from_bits(val)
    }
}
impl From<Bvalid> for u8 {
    #[inline(always)]
    fn from(val: Bvalid) -> u8 {
        Bvalid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ChkChrgB {
    #[doc = "Enable."]
    BcChrgdetEnable = 0x0,
    #[doc = "Disable."]
    BcChrgdetDisable = 0x01,
}
impl ChkChrgB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChkChrgB {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChkChrgB {
    #[inline(always)]
    fn from(val: u8) -> ChkChrgB {
        ChkChrgB::from_bits(val)
    }
}
impl From<ChkChrgB> for u8 {
    #[inline(always)]
    fn from(val: ChkChrgB) -> u8 {
        ChkChrgB::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ChkContact {
    #[doc = "Disable."]
    BcDcdDisable = 0x0,
    #[doc = "Enable."]
    BcDcdEnable = 0x01,
}
impl ChkContact {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChkContact {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChkContact {
    #[inline(always)]
    fn from(val: u8) -> ChkContact {
        ChkContact::from_bits(val)
    }
}
impl From<ChkContact> for u8 {
    #[inline(always)]
    fn from(val: ChkContact) -> u8 {
        ChkContact::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ChrgDetected {
    #[doc = "SDP detected."]
    SdpDetect = 0x0,
    #[doc = "Charging port detected."]
    ChrgPortDetect = 0x01,
}
impl ChrgDetected {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ChrgDetected {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ChrgDetected {
    #[inline(always)]
    fn from(val: u8) -> ChrgDetected {
        ChrgDetected::from_bits(val)
    }
}
impl From<ChrgDetected> for u8 {
    #[inline(always)]
    fn from(val: ChrgDetected) -> u8 {
        ChrgDetected::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clkgate {
    #[doc = "Run clocks."]
    RunClocks = 0x0,
    #[doc = "Gate clocks."]
    GateClocks = 0x01,
}
impl Clkgate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clkgate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clkgate {
    #[inline(always)]
    fn from(val: u8) -> Clkgate {
        Clkgate::from_bits(val)
    }
}
impl From<Clkgate> for u8 {
    #[inline(always)]
    fn from(val: Clkgate) -> u8 {
        Clkgate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DCal {
    #[doc = "Maximum current, approximately 19% above nominal."]
    MaxCurrent = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Nominal."]
    Nominal = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Minimum current, approximately 19% below nominal."]
    MinCurrent = 0x0f,
}
impl DCal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DCal {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DCal {
    #[inline(always)]
    fn from(val: u8) -> DCal {
        DCal::from_bits(val)
    }
}
impl From<DCal> for u8 {
    #[inline(always)]
    fn from(val: DCal) -> u8 {
        DCal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dcdsel {
    #[doc = "Fields in USB1_CHRG_DETECT."]
    ChrgdetCtrl = 0x0,
    #[doc = "Fields and state machines in the USBHSDCD module."]
    UsbhsdcdCtrl = 0x01,
}
impl Dcdsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dcdsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dcdsel {
    #[inline(always)]
    fn from(val: u8) -> Dcdsel {
        Dcdsel::from_bits(val)
    }
}
impl From<Dcdsel> for u8 {
    #[inline(always)]
    fn from(val: Dcdsel) -> u8 {
        Dcdsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DetectSec {
    #[doc = "Disable."]
    BcSecdetDisable = 0x0,
    #[doc = "Enable."]
    BcSecdetEnable = 0x01,
}
impl DetectSec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DetectSec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DetectSec {
    #[inline(always)]
    fn from(val: u8) -> DetectSec {
        DetectSec::from_bits(val)
    }
}
impl From<DetectSec> for u8 {
    #[inline(always)]
    fn from(val: DetectSec) -> u8 {
        DetectSec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DevPulldown {
    #[doc = "Disable."]
    DevPulldownDis = 0x0,
    #[doc = "Enable."]
    DevPulldownEn = 0x01,
}
impl DevPulldown {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DevPulldown {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DevPulldown {
    #[inline(always)]
    fn from(val: u8) -> DevPulldown {
        DevPulldown::from_bits(val)
    }
}
impl From<DevPulldown> for u8 {
    #[inline(always)]
    fn from(val: DevPulldown) -> u8 {
        DevPulldown::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DevpluginPolarity {
    #[doc = "Plugged in."]
    PluggedIn = 0x0,
    #[doc = "Unplugged."]
    Unplugged = 0x01,
}
impl DevpluginPolarity {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DevpluginPolarity {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DevpluginPolarity {
    #[inline(always)]
    fn from(val: u8) -> DevpluginPolarity {
        DevpluginPolarity::from_bits(val)
    }
}
impl From<DevpluginPolarity> for u8 {
    #[inline(always)]
    fn from(val: DevpluginPolarity) -> u8 {
        DevpluginPolarity::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DevpluginStatus {
    #[doc = "No attachment detected."]
    NoCable = 0x0,
    #[doc = "Cable attachment detected."]
    CableAttach = 0x01,
}
impl DevpluginStatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DevpluginStatus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DevpluginStatus {
    #[inline(always)]
    fn from(val: u8) -> DevpluginStatus {
        DevpluginStatus::from_bits(val)
    }
}
impl From<DevpluginStatus> for u8 {
    #[inline(always)]
    fn from(val: DevpluginStatus) -> u8 {
        DevpluginStatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DischargeVbus {
    #[doc = "Disable."]
    VbusDchgOff = 0x0,
    #[doc = "Enable."]
    VbusDchgOn = 0x01,
}
impl DischargeVbus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DischargeVbus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DischargeVbus {
    #[inline(always)]
    fn from(val: u8) -> DischargeVbus {
        DischargeVbus::from_bits(val)
    }
}
impl From<DischargeVbus> for u8 {
    #[inline(always)]
    fn from(val: DischargeVbus) -> u8 {
        DischargeVbus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Disconadj {
    #[doc = "0.56875 V."]
    DisconTrimNom = 0x0,
    #[doc = "0.55000 V."]
    DisconTrimLo = 0x01,
    #[doc = "0.58125 V."]
    DisconTrimMedhi = 0x02,
    #[doc = "0.60000 V."]
    DisconTrimHi = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Disconadj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Disconadj {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Disconadj {
    #[inline(always)]
    fn from(val: u8) -> Disconadj {
        Disconadj::from_bits(val)
    }
}
impl From<Disconadj> for u8 {
    #[inline(always)]
    fn from(val: Disconadj) -> u8 {
        Disconadj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DivSelOverride {
    #[doc = "TRIM_OVERRIDE_EN."]
    UseTrim0Plldiv = 0x0,
    #[doc = "PLL_SIC."]
    UsePllSicPlldiv = 0x01,
}
impl DivSelOverride {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DivSelOverride {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DivSelOverride {
    #[inline(always)]
    fn from(val: u8) -> DivSelOverride {
        DivSelOverride::from_bits(val)
    }
}
impl From<DivSelOverride> for u8 {
    #[inline(always)]
    fn from(val: DivSelOverride) -> u8 {
        DivSelOverride::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DmState {
    #[doc = "USB_DM pin voltage is <= 0.8 V."]
    DmSerxLo = 0x0,
    #[doc = "USB_DM pin voltage is >= 2.0 V."]
    DmSerxHi = 0x01,
}
impl DmState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DmState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DmState {
    #[inline(always)]
    fn from(val: u8) -> DmState {
        DmState::from_bits(val)
    }
}
impl From<DmState> for u8 {
    #[inline(always)]
    fn from(val: DmState) -> u8 {
        DmState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DpState {
    #[doc = "USB_DP pin voltage is <= 0.8 V."]
    DpSerxLo = 0x0,
    #[doc = "USB_DP pin voltage is >= 2.0 V."]
    DpSerxHi = 0x01,
}
impl DpState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DpState {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DpState {
    #[inline(always)]
    fn from(val: u8) -> DpState {
        DpState::from_bits(val)
    }
}
impl From<DpState> for u8 {
    #[inline(always)]
    fn from(val: DpState) -> u8 {
        DpState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EnB {
    #[doc = "Enable."]
    BcEnable = 0x0,
    #[doc = "Disable."]
    BcDisable = 0x01,
}
impl EnB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EnB {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EnB {
    #[inline(always)]
    fn from(val: u8) -> EnB {
        EnB::from_bits(val)
    }
}
impl From<EnB> for u8 {
    #[inline(always)]
    fn from(val: EnB) -> u8 {
        EnB::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endevplugindetect {
    #[doc = "Disable."]
    PluginDisable = 0x0,
    #[doc = "Enable."]
    PluginEnable = 0x01,
}
impl Endevplugindetect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endevplugindetect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endevplugindetect {
    #[inline(always)]
    fn from(val: u8) -> Endevplugindetect {
        Endevplugindetect::from_bits(val)
    }
}
impl From<Endevplugindetect> for u8 {
    #[inline(always)]
    fn from(val: Endevplugindetect) -> u8 {
        Endevplugindetect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enhstpulldown {
    #[doc = "Disable."]
    Disable = 0x0,
    #[doc = "Enable."]
    Enable = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Enhstpulldown {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enhstpulldown {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enhstpulldown {
    #[inline(always)]
    fn from(val: u8) -> Enhstpulldown {
        Enhstpulldown::from_bits(val)
    }
}
impl From<Enhstpulldown> for u8 {
    #[inline(always)]
    fn from(val: Enhstpulldown) -> u8 {
        Enhstpulldown::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enotgiddetect {
    #[doc = "Disable."]
    IdDetDisable = 0x0,
    #[doc = "Enable."]
    IdDetEnable = 0x01,
}
impl Enotgiddetect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enotgiddetect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enotgiddetect {
    #[inline(always)]
    fn from(val: u8) -> Enotgiddetect {
        Enotgiddetect::from_bits(val)
    }
}
impl From<Enotgiddetect> for u8 {
    #[inline(always)]
    fn from(val: Enotgiddetect) -> u8 {
        Enotgiddetect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Envadj {
    #[doc = "0.1000 V."]
    EnvTrimNom = 0x0,
    #[doc = "0.1125 V."]
    EnvTrimMedhi = 0x01,
    #[doc = "0.1250 V."]
    EnvTrimHi = 0x02,
    #[doc = "0.0875 V."]
    EnvTrimLo = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Envadj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Envadj {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Envadj {
    #[inline(always)]
    fn from(val: u8) -> Envadj {
        Envadj::from_bits(val)
    }
}
impl From<Envadj> for u8 {
    #[inline(always)]
    fn from(val: Envadj) -> u8 {
        Envadj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ExtIdOverrideEn {
    #[doc = "Internal detector or local override."]
    UsePhyId = 0x0,
    #[doc = "External ID signal value."]
    UseExtId = 0x01,
}
impl ExtIdOverrideEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ExtIdOverrideEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ExtIdOverrideEn {
    #[inline(always)]
    fn from(val: u8) -> ExtIdOverrideEn {
        ExtIdOverrideEn::from_bits(val)
    }
}
impl From<ExtIdOverrideEn> for u8 {
    #[inline(always)]
    fn from(val: ExtIdOverrideEn) -> u8 {
        ExtIdOverrideEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ExtVbusOverrideEn {
    #[doc = "Internal detector or local override."]
    UsePhyVbus = 0x0,
    #[doc = "External VBUS_VALID value."]
    UsbExtVbus = 0x01,
}
impl ExtVbusOverrideEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ExtVbusOverrideEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ExtVbusOverrideEn {
    #[inline(always)]
    fn from(val: u8) -> ExtVbusOverrideEn {
        ExtVbusOverrideEn::from_bits(val)
    }
}
impl From<ExtVbusOverrideEn> for u8 {
    #[inline(always)]
    fn from(val: ExtVbusOverrideEn) -> u8 {
        ExtVbusOverrideEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HostdiscondetectIrq {
    #[doc = "Connected."]
    Connected = 0x0,
    #[doc = "Disconnected."]
    Disconnected = 0x01,
}
impl HostdiscondetectIrq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HostdiscondetectIrq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HostdiscondetectIrq {
    #[inline(always)]
    fn from(val: u8) -> HostdiscondetectIrq {
        HostdiscondetectIrq::from_bits(val)
    }
}
impl From<HostdiscondetectIrq> for u8 {
    #[inline(always)]
    fn from(val: HostdiscondetectIrq) -> u8 {
        HostdiscondetectIrq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hstpulldown {
    #[doc = "Disconnect."]
    Disconnect = 0x0,
    #[doc = "Connect."]
    Connect = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Hstpulldown {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hstpulldown {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hstpulldown {
    #[inline(always)]
    fn from(val: u8) -> Hstpulldown {
        Hstpulldown::from_bits(val)
    }
}
impl From<Hstpulldown> for u8 {
    #[inline(always)]
    fn from(val: Hstpulldown) -> u8 {
        Hstpulldown::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IdOverrideEn {
    #[doc = "Use ID pin detector or external override."]
    NoPhyIdOverride = 0x0,
    #[doc = "Allow local override of ID pin detection status."]
    UsePhyIdOverride = 0x01,
}
impl IdOverrideEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IdOverrideEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IdOverrideEn {
    #[inline(always)]
    fn from(val: u8) -> IdOverrideEn {
        IdOverrideEn::from_bits(val)
    }
}
impl From<IdOverrideEn> for u8 {
    #[inline(always)]
    fn from(val: IdOverrideEn) -> u8 {
        IdOverrideEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LviEn {
    #[doc = "Disable."]
    Lvi3vDisable = 0x0,
    #[doc = "Enable."]
    Lvi3vEnable = 0x01,
}
impl LviEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LviEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LviEn {
    #[inline(always)]
    fn from(val: u8) -> LviEn {
        LviEn::from_bits(val)
    }
}
impl From<LviEn> for u8 {
    #[inline(always)]
    fn from(val: LviEn) -> u8 {
        LviEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Misc2Control0 {
    #[doc = "Power up PLL."]
    PllOnSuspend = 0x0,
    #[doc = "Power down PLL."]
    PllOffSuspend = 0x01,
}
impl Misc2Control0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Misc2Control0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Misc2Control0 {
    #[inline(always)]
    fn from(val: u8) -> Misc2Control0 {
        Misc2Control0::from_bits(val)
    }
}
impl From<Misc2Control0> for u8 {
    #[inline(always)]
    fn from(val: Misc2Control0) -> u8 {
        Misc2Control0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OkStatus3v {
    #[doc = "Not powered."]
    Power31p8Ok = 0x0,
    #[doc = "Powered."]
    Power31p8Bad = 0x01,
}
impl OkStatus3v {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OkStatus3v {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OkStatus3v {
    #[inline(always)]
    fn from(val: u8) -> OkStatus3v {
        OkStatus3v::from_bits(val)
    }
}
impl From<OkStatus3v> for u8 {
    #[inline(always)]
    fn from(val: OkStatus3v) -> u8 {
        OkStatus3v::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OtgIdValue {
    #[doc = "Host."]
    IdHost = 0x0,
    #[doc = "Device."]
    IdDevice = 0x01,
}
impl OtgIdValue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OtgIdValue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OtgIdValue {
    #[inline(always)]
    fn from(val: u8) -> OtgIdValue {
        OtgIdValue::from_bits(val)
    }
}
impl From<OtgIdValue> for u8 {
    #[inline(always)]
    fn from(val: OtgIdValue) -> u8 {
        OtgIdValue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OtgidStatus {
    #[doc = "Host."]
    IdHost = 0x0,
    #[doc = "Device."]
    IdDevice = 0x01,
}
impl OtgidStatus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OtgidStatus {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OtgidStatus {
    #[inline(always)]
    fn from(val: u8) -> OtgidStatus {
        OtgidStatus::from_bits(val)
    }
}
impl From<OtgidStatus> for u8 {
    #[inline(always)]
    fn from(val: OtgidStatus) -> u8 {
        OtgidStatus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pfd0Clkgate {
    #[doc = "Enable."]
    Pfd0ClkEn = 0x0,
    #[doc = "Disable."]
    Pfd0ClkDis = 0x01,
}
impl Pfd0Clkgate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pfd0Clkgate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pfd0Clkgate {
    #[inline(always)]
    fn from(val: u8) -> Pfd0Clkgate {
        Pfd0Clkgate::from_bits(val)
    }
}
impl From<Pfd0Clkgate> for u8 {
    #[inline(always)]
    fn from(val: Pfd0Clkgate) -> u8 {
        Pfd0Clkgate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PfdClkSel {
    #[doc = "USB1PFDCLK = USB PLL reference clock."]
    PfdClkBypass = 0x0,
    #[doc = "USB1PFDCLK = pfd_clk / 4."]
    PfdClkDiv4 = 0x01,
    #[doc = "USB1PFDCLK frequency = pfd_clk / 2."]
    PfdClkDiv2 = 0x02,
    #[doc = "USB1PFDCLK = pfd_clk."]
    PfdClkDiv1 = 0x03,
}
impl PfdClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PfdClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PfdClkSel {
    #[inline(always)]
    fn from(val: u8) -> PfdClkSel {
        PfdClkSel::from_bits(val)
    }
}
impl From<PfdClkSel> for u8 {
    #[inline(always)]
    fn from(val: PfdClkSel) -> u8 {
        PfdClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllBypass {
    #[doc = "480 MHz output clock."]
    PllNoBypass = 0x0,
    #[doc = "Input reference clock."]
    PllBypass = 0x01,
}
impl PllBypass {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllBypass {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllBypass {
    #[inline(always)]
    fn from(val: u8) -> PllBypass {
        PllBypass::from_bits(val)
    }
}
impl From<PllBypass> for u8 {
    #[inline(always)]
    fn from(val: PllBypass) -> u8 {
        PllBypass::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllDivSel {
    #[doc = "Configure for a 32 MHz input clock (divide by 15)."]
    PllDiv15 = 0x0,
    #[doc = "Configure for a 30 MHz input clock (divide by 16)."]
    PllDiv16 = 0x01,
    #[doc = "Configure for a 24 MHz input clock (divide by 20)."]
    PllDiv20 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Configure for a 20 MHz input clock (divide by 24)."]
    PllDiv24 = 0x04,
    #[doc = "Configure for a 19.2 MHz input clock (divide by 25)."]
    PllDiv25 = 0x05,
    #[doc = "Configure for a 16 MHz input clock (divide by 30)."]
    PllDiv30 = 0x06,
    #[doc = "Configure for a 12 MHz input clock (divide by 40)."]
    PllDiv32 = 0x07,
}
impl PllDivSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllDivSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllDivSel {
    #[inline(always)]
    fn from(val: u8) -> PllDivSel {
        PllDivSel::from_bits(val)
    }
}
impl From<PllDivSel> for u8 {
    #[inline(always)]
    fn from(val: PllDivSel) -> u8 {
        PllDivSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllEnUsbClks {
    #[doc = "Disable."]
    PllMpDisable = 0x0,
    #[doc = "Enable."]
    PllMpEnable = 0x01,
}
impl PllEnUsbClks {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllEnUsbClks {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllEnUsbClks {
    #[inline(always)]
    fn from(val: u8) -> PllEnUsbClks {
        PllEnUsbClks::from_bits(val)
    }
}
impl From<PllEnUsbClks> for u8 {
    #[inline(always)]
    fn from(val: PllEnUsbClks) -> u8 {
        PllEnUsbClks::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllEnable {
    #[doc = "Disable."]
    PllOutDisable = 0x0,
    #[doc = "Enable."]
    PllOutEnable = 0x01,
}
impl PllEnable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllEnable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllEnable {
    #[inline(always)]
    fn from(val: u8) -> PllEnable {
        PllEnable::from_bits(val)
    }
}
impl From<PllEnable> for u8 {
    #[inline(always)]
    fn from(val: PllEnable) -> u8 {
        PllEnable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllLock {
    #[doc = "Not locked."]
    PllNotLocked = 0x0,
    #[doc = "Locked."]
    PllLocked = 0x01,
}
impl PllLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllLock {
    #[inline(always)]
    fn from(val: u8) -> PllLock {
        PllLock::from_bits(val)
    }
}
impl From<PllLock> for u8 {
    #[inline(always)]
    fn from(val: PllLock) -> u8 {
        PllLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllPower {
    #[doc = "Power down."]
    PllForcePwd = 0x0,
    #[doc = "Allow powerup."]
    PllAllowPowerup = 0x01,
}
impl PllPower {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllPower {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllPower {
    #[inline(always)]
    fn from(val: u8) -> PllPower {
        PllPower::from_bits(val)
    }
}
impl From<PllPower> for u8 {
    #[inline(always)]
    fn from(val: PllPower) -> u8 {
        PllPower::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllRegEnable {
    #[doc = "Disable."]
    PllRegDisable = 0x0,
    #[doc = "Enable."]
    PllRegEnable = 0x01,
}
impl PllRegEnable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllRegEnable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllRegEnable {
    #[inline(always)]
    fn from(val: u8) -> PllRegEnable {
        PllRegEnable::from_bits(val)
    }
}
impl From<PllRegEnable> for u8 {
    #[inline(always)]
    fn from(val: PllRegEnable) -> u8 {
        PllRegEnable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PlugContact {
    #[doc = "Not detected."]
    NoDcDetected = 0x0,
    #[doc = "Detected."]
    DcDeteced = 0x01,
}
impl PlugContact {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PlugContact {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PlugContact {
    #[inline(always)]
    fn from(val: u8) -> PlugContact {
        PlugContact::from_bits(val)
    }
}
impl From<PlugContact> for u8 {
    #[inline(always)]
    fn from(val: PlugContact) -> u8 {
        PlugContact::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PullupDp {
    #[doc = "Disable."]
    DpPueNormal = 0x0,
    #[doc = "Enable."]
    DpPueOverride = 0x01,
}
impl PullupDp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PullupDp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PullupDp {
    #[inline(always)]
    fn from(val: u8) -> PullupDp {
        PullupDp::from_bits(val)
    }
}
impl From<PullupDp> for u8 {
    #[inline(always)]
    fn from(val: PullupDp) -> u8 {
        PullupDp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RefbiasPwd {
    #[doc = "Enable."]
    RefbiasEnabled = 0x0,
    #[doc = "Disable or power down."]
    RefbiasPwd = 0x01,
}
impl RefbiasPwd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RefbiasPwd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RefbiasPwd {
    #[inline(always)]
    fn from(val: u8) -> RefbiasPwd {
        RefbiasPwd::from_bits(val)
    }
}
impl From<RefbiasPwd> for u8 {
    #[inline(always)]
    fn from(val: RefbiasPwd) -> u8 {
        RefbiasPwd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RefbiasPwdSel {
    #[doc = "PLL_POWER internal state signal."]
    BiasPllpower = 0x0,
    #[doc = "REFBIAS_PWD."]
    BiasRefbiasPwd = 0x01,
}
impl RefbiasPwdSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RefbiasPwdSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RefbiasPwdSel {
    #[inline(always)]
    fn from(val: u8) -> RefbiasPwdSel {
        RefbiasPwdSel::from_bits(val)
    }
}
impl From<RefbiasPwdSel> for u8 {
    #[inline(always)]
    fn from(val: RefbiasPwdSel) -> u8 {
        RefbiasPwdSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxpwd1pt1 {
    #[doc = "Enable."]
    FsRxdiffEnable = 0x0,
    #[doc = "Disable or power down."]
    FsRxdiffPwd = 0x01,
}
impl Rxpwd1pt1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxpwd1pt1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxpwd1pt1 {
    #[inline(always)]
    fn from(val: u8) -> Rxpwd1pt1 {
        Rxpwd1pt1::from_bits(val)
    }
}
impl From<Rxpwd1pt1> for u8 {
    #[inline(always)]
    fn from(val: Rxpwd1pt1) -> u8 {
        Rxpwd1pt1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxpwddiff {
    #[doc = "Enable."]
    HsRxdiffEnable = 0x0,
    #[doc = "Disable or power down."]
    HsRxdiffPwd = 0x01,
}
impl Rxpwddiff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxpwddiff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxpwddiff {
    #[inline(always)]
    fn from(val: u8) -> Rxpwddiff {
        Rxpwddiff::from_bits(val)
    }
}
impl From<Rxpwddiff> for u8 {
    #[inline(always)]
    fn from(val: Rxpwddiff) -> u8 {
        Rxpwddiff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxpwdenv {
    #[doc = "Enable."]
    RxEnvhdEnable = 0x0,
    #[doc = "Disable or power down."]
    RxEnvhdPwd = 0x01,
}
impl Rxpwdenv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxpwdenv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxpwdenv {
    #[inline(always)]
    fn from(val: u8) -> Rxpwdenv {
        Rxpwdenv::from_bits(val)
    }
}
impl From<Rxpwdenv> for u8 {
    #[inline(always)]
    fn from(val: Rxpwdenv) -> u8 {
        Rxpwdenv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxpwdrx {
    #[doc = "Enable."]
    RxBiasEnable = 0x0,
    #[doc = "Disable or power down."]
    RxBiasPwd = 0x01,
}
impl Rxpwdrx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxpwdrx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxpwdrx {
    #[inline(always)]
    fn from(val: u8) -> Rxpwdrx {
        Rxpwdrx::from_bits(val)
    }
}
impl From<Rxpwdrx> for u8 {
    #[inline(always)]
    fn from(val: Rxpwdrx) -> u8 {
        Rxpwdrx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SecdetDcp {
    #[doc = "CDP detected."]
    SecdetCdp = 0x0,
    #[doc = "DCP detected."]
    SecdetDcp = 0x01,
}
impl SecdetDcp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SecdetDcp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SecdetDcp {
    #[inline(always)]
    fn from(val: u8) -> SecdetDcp {
        SecdetDcp::from_bits(val)
    }
}
impl From<SecdetDcp> for u8 {
    #[inline(always)]
    fn from(val: SecdetDcp) -> u8 {
        SecdetDcp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sessend {
    #[doc = "Above threshold."]
    SessendLo = 0x0,
    #[doc = "Below threshold."]
    SessendHi = 0x01,
}
impl Sessend {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sessend {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sessend {
    #[inline(always)]
    fn from(val: u8) -> Sessend {
        Sessend::from_bits(val)
    }
}
impl From<Sessend> for u8 {
    #[inline(always)]
    fn from(val: Sessend) -> u8 {
        Sessend::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sftrst {
    #[doc = "Release from reset."]
    ReleaseReset = 0x0,
    #[doc = "Soft-reset."]
    SoftReset = 0x01,
}
impl Sftrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sftrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sftrst {
    #[inline(always)]
    fn from(val: u8) -> Sftrst {
        Sftrst::from_bits(val)
    }
}
impl From<Sftrst> for u8 {
    #[inline(always)]
    fn from(val: Sftrst) -> u8 {
        Sftrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TxCal45dmOverride {
    #[doc = "TRIM_OVERRIDE_EN."]
    UseTrim0Cal45dn = 0x0,
    #[doc = "TX."]
    UseTxCal45dn = 0x01,
}
impl TxCal45dmOverride {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TxCal45dmOverride {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TxCal45dmOverride {
    #[inline(always)]
    fn from(val: u8) -> TxCal45dmOverride {
        TxCal45dmOverride::from_bits(val)
    }
}
impl From<TxCal45dmOverride> for u8 {
    #[inline(always)]
    fn from(val: TxCal45dmOverride) -> u8 {
        TxCal45dmOverride::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TxCal45dpOverride {
    #[doc = "TRIM_OVERRIDE_EN."]
    UseTrim0Cal45dp = 0x0,
    #[doc = "TX."]
    UseTxCal45dp = 0x01,
}
impl TxCal45dpOverride {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TxCal45dpOverride {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TxCal45dpOverride {
    #[inline(always)]
    fn from(val: u8) -> TxCal45dpOverride {
        TxCal45dpOverride::from_bits(val)
    }
}
impl From<TxCal45dpOverride> for u8 {
    #[inline(always)]
    fn from(val: TxCal45dpOverride) -> u8 {
        TxCal45dpOverride::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TxDCalOverride {
    #[doc = "TRIM_OVERRIDE_EN."]
    UseTrim0Dcal = 0x0,
    #[doc = "TX."]
    UseTxDcal = 0x01,
}
impl TxDCalOverride {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TxDCalOverride {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TxDCalOverride {
    #[inline(always)]
    fn from(val: u8) -> TxDCalOverride {
        TxDCalOverride::from_bits(val)
    }
}
impl From<TxDCalOverride> for u8 {
    #[inline(always)]
    fn from(val: TxDCalOverride) -> u8 {
        TxDCalOverride::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txpwdfs {
    #[doc = "Provide bias to enable."]
    FstxBiasEnable = 0x0,
    #[doc = "Disable or power down."]
    FstxBiasPwd = 0x01,
}
impl Txpwdfs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txpwdfs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txpwdfs {
    #[inline(always)]
    fn from(val: u8) -> Txpwdfs {
        Txpwdfs::from_bits(val)
    }
}
impl From<Txpwdfs> for u8 {
    #[inline(always)]
    fn from(val: Txpwdfs) -> u8 {
        Txpwdfs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txpwdibias {
    #[doc = "Enable."]
    IbiasEnable = 0x0,
    #[doc = "Disable or power down."]
    IbiasPwd = 0x01,
}
impl Txpwdibias {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txpwdibias {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txpwdibias {
    #[inline(always)]
    fn from(val: u8) -> Txpwdibias {
        Txpwdibias::from_bits(val)
    }
}
impl From<Txpwdibias> for u8 {
    #[inline(always)]
    fn from(val: Txpwdibias) -> u8 {
        Txpwdibias::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txpwdv2i {
    #[doc = "Enable."]
    V2iBiasEnable = 0x0,
    #[doc = "Disable or power down."]
    V2iBiasPwd = 0x01,
}
impl Txpwdv2i {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txpwdv2i {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txpwdv2i {
    #[inline(always)]
    fn from(val: u8) -> Txpwdv2i {
        Txpwdv2i::from_bits(val)
    }
}
impl From<Txpwdv2i> for u8 {
    #[inline(always)]
    fn from(val: Txpwdv2i) -> u8 {
        Txpwdv2i::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbphyTxDCal {
    #[doc = "Maximum current, approximately 19% above nominal."]
    MaxCurrent = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Nominal."]
    Nominal = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Minimum current, approximately 19% below nominal."]
    MinCurrent = 0x0f,
}
impl UsbphyTxDCal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UsbphyTxDCal {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UsbphyTxDCal {
    #[inline(always)]
    fn from(val: u8) -> UsbphyTxDCal {
        UsbphyTxDCal::from_bits(val)
    }
}
impl From<UsbphyTxDCal> for u8 {
    #[inline(always)]
    fn from(val: UsbphyTxDCal) -> u8 {
        UsbphyTxDCal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VbusOverrideEn {
    #[doc = "Results of VBUS_VALID and session valid comparators for VBUS_VALID, AVALID, BVALID, and SESSEND."]
    VbusNoOverride = 0x0,
    #[doc = "Override values for VBUS_VALID, AVALID, BVALID, and SESSEND."]
    VbusOverride = 0x01,
}
impl VbusOverrideEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VbusOverrideEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VbusOverrideEn {
    #[inline(always)]
    fn from(val: u8) -> VbusOverrideEn {
        VbusOverrideEn::from_bits(val)
    }
}
impl From<VbusOverrideEn> for u8 {
    #[inline(always)]
    fn from(val: VbusOverrideEn) -> u8 {
        VbusOverrideEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VbusSourceSel {
    #[doc = "VBUS_VALID comparator result."]
    UseVbusVld = 0x0,
    #[doc = "Session valid comparator result."]
    UseAsessVld = 0x01,
    #[doc = "Session valid comparator result."]
    UseBsessVld = 0x02,
    _RESERVED_3 = 0x03,
}
impl VbusSourceSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VbusSourceSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VbusSourceSel {
    #[inline(always)]
    fn from(val: u8) -> VbusSourceSel {
        VbusSourceSel::from_bits(val)
    }
}
impl From<VbusSourceSel> for u8 {
    #[inline(always)]
    fn from(val: VbusSourceSel) -> u8 {
        VbusSourceSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VbusValid {
    #[doc = "Below threshold."]
    VbusLo = 0x0,
    #[doc = "Above threshold."]
    VbusHi = 0x01,
}
impl VbusValid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VbusValid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VbusValid {
    #[inline(always)]
    fn from(val: u8) -> VbusValid {
        VbusValid::from_bits(val)
    }
}
impl From<VbusValid> for u8 {
    #[inline(always)]
    fn from(val: VbusValid) -> u8 {
        VbusValid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VbusValid3v {
    #[doc = "Below threshold."]
    VbusVld3vLo = 0x0,
    #[doc = "Above threshold."]
    VbusVld3vHi = 0x01,
}
impl VbusValid3v {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VbusValid3v {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VbusValid3v {
    #[inline(always)]
    fn from(val: u8) -> VbusValid3v {
        VbusValid3v::from_bits(val)
    }
}
impl From<VbusValid3v> for u8 {
    #[inline(always)]
    fn from(val: VbusValid3v) -> u8 {
        VbusValid3v::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VbusvalidPwrupCmps {
    #[doc = "Disable or power down the VBUS_VALID comparator."]
    VbusValidDisable = 0x0,
    #[doc = "Enable the VBUS_VALID comparator."]
    VbusValidEnable = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl VbusvalidPwrupCmps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VbusvalidPwrupCmps {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VbusvalidPwrupCmps {
    #[inline(always)]
    fn from(val: u8) -> VbusvalidPwrupCmps {
        VbusvalidPwrupCmps::from_bits(val)
    }
}
impl From<VbusvalidPwrupCmps> for u8 {
    #[inline(always)]
    fn from(val: VbusvalidPwrupCmps) -> u8 {
        VbusvalidPwrupCmps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VbusvalidSel {
    #[doc = "VBUS_VALID comparator result."]
    VbusVldOut = 0x0,
    #[doc = "VBUS_VALID_3V comparator result."]
    VbusVld3vOut = 0x01,
}
impl VbusvalidSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VbusvalidSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VbusvalidSel {
    #[inline(always)]
    fn from(val: u8) -> VbusvalidSel {
        VbusvalidSel::from_bits(val)
    }
}
impl From<VbusvalidSel> for u8 {
    #[inline(always)]
    fn from(val: VbusvalidSel) -> u8 {
        VbusvalidSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VbusvalidThresh {
    #[doc = "4.0 V."]
    VbusVld4p0 = 0x0,
    #[doc = "4.1 V."]
    VbusVld4p1 = 0x01,
    #[doc = "4.2 V."]
    VbusVld4p2 = 0x02,
    #[doc = "4.3 V."]
    VbusVld4p3 = 0x03,
    #[doc = "4.4 V."]
    VbusVld4p4 = 0x04,
    #[doc = "4.5 V."]
    VbusVld4p5 = 0x05,
    #[doc = "4.6 V."]
    VbusVld4p6 = 0x06,
    #[doc = "4.7 V."]
    VbusVld4p7 = 0x07,
}
impl VbusvalidThresh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VbusvalidThresh {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VbusvalidThresh {
    #[inline(always)]
    fn from(val: u8) -> VbusvalidThresh {
        VbusvalidThresh::from_bits(val)
    }
}
impl From<VbusvalidThresh> for u8 {
    #[inline(always)]
    fn from(val: VbusvalidThresh) -> u8 {
        VbusvalidThresh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VbusvalidToB {
    #[doc = "VBUS_VALID comparator."]
    UseVbusVld = 0x0,
    #[doc = "Session valid detector."]
    UseSessVld = 0x01,
}
impl VbusvalidToB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VbusvalidToB {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VbusvalidToB {
    #[inline(always)]
    fn from(val: u8) -> VbusvalidToB {
        VbusvalidToB::from_bits(val)
    }
}
impl From<VbusvalidToB> for u8 {
    #[inline(always)]
    fn from(val: VbusvalidToB) -> u8 {
        VbusvalidToB::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VdmSrcEnable {
    #[doc = "Disable."]
    DcdVdmSrcDisable = 0x0,
    #[doc = "Enable."]
    DcdVdmSrcEnable = 0x01,
}
impl VdmSrcEnable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VdmSrcEnable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VdmSrcEnable {
    #[inline(always)]
    fn from(val: u8) -> VdmSrcEnable {
        VdmSrcEnable::from_bits(val)
    }
}
impl From<VdmSrcEnable> for u8 {
    #[inline(always)]
    fn from(val: VdmSrcEnable) -> u8 {
        VdmSrcEnable::to_bits(val)
    }
}
