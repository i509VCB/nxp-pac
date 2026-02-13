#[doc = "General Purpose ADC VBAT Divider branch control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdcCtrl(pub u32);
impl AdcCtrl {
    #[doc = "Switch On/Off VBAT divider branch."]
    #[must_use]
    #[inline(always)]
    pub const fn vbatdivenable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Switch On/Off VBAT divider branch."]
    #[inline(always)]
    pub const fn set_vbatdivenable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for AdcCtrl {
    #[inline(always)]
    fn default() -> AdcCtrl {
        AdcCtrl(0)
    }
}
impl core::fmt::Debug for AdcCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AdcCtrl")
            .field("vbatdivenable", &self.vbatdivenable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AdcCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AdcCtrl {{ vbatdivenable: {=bool:?} }}",
            self.vbatdivenable()
        )
    }
}
#[doc = "Various Analog blocks configuration (like FRO 192MHz trimmings source ...)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnalogCtrlCfg(pub u32);
impl AnalogCtrlCfg {
    #[doc = "FRO192M trimming and 'Enable' source."]
    #[must_use]
    #[inline(always)]
    pub const fn fro192m_trim_src(&self) -> super::vals::Fro192mTrimSrc {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Fro192mTrimSrc::from_bits(val as u8)
    }
    #[doc = "FRO192M trimming and 'Enable' source."]
    #[inline(always)]
    pub const fn set_fro192m_trim_src(&mut self, val: super::vals::Fro192mTrimSrc) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for AnalogCtrlCfg {
    #[inline(always)]
    fn default() -> AnalogCtrlCfg {
        AnalogCtrlCfg(0)
    }
}
impl core::fmt::Debug for AnalogCtrlCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AnalogCtrlCfg")
            .field("fro192m_trim_src", &self.fro192m_trim_src())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AnalogCtrlCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AnalogCtrlCfg {{ fro192m_trim_src: {:?} }}",
            self.fro192m_trim_src()
        )
    }
}
#[doc = "Analog Macroblock Identity registers, Flash Status registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnalogCtrlStatus(pub u32);
impl AnalogCtrlStatus {
    #[doc = "Flash Power Down status."]
    #[must_use]
    #[inline(always)]
    pub const fn flash_pwrdwn(&self) -> super::vals::FlashPwrdwn {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::FlashPwrdwn::from_bits(val as u8)
    }
    #[doc = "Flash Power Down status."]
    #[inline(always)]
    pub const fn set_flash_pwrdwn(&mut self, val: super::vals::FlashPwrdwn) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Flash initialization error status."]
    #[must_use]
    #[inline(always)]
    pub const fn flash_init_error(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Flash initialization error status."]
    #[inline(always)]
    pub const fn set_flash_init_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for AnalogCtrlStatus {
    #[inline(always)]
    fn default() -> AnalogCtrlStatus {
        AnalogCtrlStatus(0)
    }
}
impl core::fmt::Debug for AnalogCtrlStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AnalogCtrlStatus")
            .field("flash_pwrdwn", &self.flash_pwrdwn())
            .field("flash_init_error", &self.flash_init_error())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AnalogCtrlStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AnalogCtrlStatus {{ flash_pwrdwn: {:?}, flash_init_error: {=bool:?} }}",
            self.flash_pwrdwn(),
            self.flash_init_error()
        )
    }
}
#[doc = "AUX_BIAS"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AuxBias(pub u32);
impl AuxBias {
    #[doc = "Control output of 1V reference voltage."]
    #[must_use]
    #[inline(always)]
    pub const fn vref1venable(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Control output of 1V reference voltage."]
    #[inline(always)]
    pub const fn set_vref1venable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "current trimming control word."]
    #[must_use]
    #[inline(always)]
    pub const fn itrim(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x1f;
        val as u8
    }
    #[doc = "current trimming control word."]
    #[inline(always)]
    pub const fn set_itrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 2usize)) | (((val as u32) & 0x1f) << 2usize);
    }
    #[doc = "current trimming control word for ptat current."]
    #[must_use]
    #[inline(always)]
    pub const fn ptatitrim(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x1f;
        val as u8
    }
    #[doc = "current trimming control word for ptat current."]
    #[inline(always)]
    pub const fn set_ptatitrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 7usize)) | (((val as u32) & 0x1f) << 7usize);
    }
    #[doc = "voltage trimming control word."]
    #[must_use]
    #[inline(always)]
    pub const fn vref1vtrim(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x1f;
        val as u8
    }
    #[doc = "voltage trimming control word."]
    #[inline(always)]
    pub const fn set_vref1vtrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 12usize)) | (((val as u32) & 0x1f) << 12usize);
    }
    #[doc = "Control bit to configure trimming state of mirror."]
    #[must_use]
    #[inline(always)]
    pub const fn vref1vcurvetrim(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x07;
        val as u8
    }
    #[doc = "Control bit to configure trimming state of mirror."]
    #[inline(always)]
    pub const fn set_vref1vcurvetrim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
    }
    #[doc = "Control bit to configure trimming state of mirror."]
    #[must_use]
    #[inline(always)]
    pub const fn itrimctrl0(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to configure trimming state of mirror."]
    #[inline(always)]
    pub const fn set_itrimctrl0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Control bit to configure trimming state of mirror."]
    #[must_use]
    #[inline(always)]
    pub const fn itrimctrl1(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to configure trimming state of mirror."]
    #[inline(always)]
    pub const fn set_itrimctrl1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for AuxBias {
    #[inline(always)]
    fn default() -> AuxBias {
        AuxBias(0)
    }
}
impl core::fmt::Debug for AuxBias {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AuxBias")
            .field("vref1venable", &self.vref1venable())
            .field("itrim", &self.itrim())
            .field("ptatitrim", &self.ptatitrim())
            .field("vref1vtrim", &self.vref1vtrim())
            .field("vref1vcurvetrim", &self.vref1vcurvetrim())
            .field("itrimctrl0", &self.itrimctrl0())
            .field("itrimctrl1", &self.itrimctrl1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AuxBias {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AuxBias {{ vref1venable: {=bool:?}, itrim: {=u8:?}, ptatitrim: {=u8:?}, vref1vtrim: {=u8:?}, vref1vcurvetrim: {=u8:?}, itrimctrl0: {=bool:?}, itrimctrl1: {=bool:?} }}",
            self.vref1venable(),
            self.itrim(),
            self.ptatitrim(),
            self.vref1vtrim(),
            self.vref1vcurvetrim(),
            self.itrimctrl0(),
            self.itrimctrl1()
        )
    }
}
#[doc = "Brown Out Detectors (BoDs) & DCDC interrupts generation control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BodDcdcIntCtrl(pub u32);
impl BodDcdcIntCtrl {
    #[doc = "BOD VBAT interrupt control."]
    #[must_use]
    #[inline(always)]
    pub const fn bodvbat_int_enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "BOD VBAT interrupt control."]
    #[inline(always)]
    pub const fn set_bodvbat_int_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "BOD VBAT interrupt clear.1: Clear the interrupt. Self-cleared bit."]
    #[must_use]
    #[inline(always)]
    pub const fn bodvbat_int_clear(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "BOD VBAT interrupt clear.1: Clear the interrupt. Self-cleared bit."]
    #[inline(always)]
    pub const fn set_bodvbat_int_clear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "BOD CORE interrupt control."]
    #[must_use]
    #[inline(always)]
    pub const fn bodcore_int_enable(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "BOD CORE interrupt control."]
    #[inline(always)]
    pub const fn set_bodcore_int_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "BOD CORE interrupt clear.1: Clear the interrupt. Self-cleared bit."]
    #[must_use]
    #[inline(always)]
    pub const fn bodcore_int_clear(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "BOD CORE interrupt clear.1: Clear the interrupt. Self-cleared bit."]
    #[inline(always)]
    pub const fn set_bodcore_int_clear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "DCDC interrupt control."]
    #[must_use]
    #[inline(always)]
    pub const fn dcdc_int_enable(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "DCDC interrupt control."]
    #[inline(always)]
    pub const fn set_dcdc_int_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "DCDC interrupt clear.1: Clear the interrupt. Self-cleared bit."]
    #[must_use]
    #[inline(always)]
    pub const fn dcdc_int_clear(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "DCDC interrupt clear.1: Clear the interrupt. Self-cleared bit."]
    #[inline(always)]
    pub const fn set_dcdc_int_clear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for BodDcdcIntCtrl {
    #[inline(always)]
    fn default() -> BodDcdcIntCtrl {
        BodDcdcIntCtrl(0)
    }
}
impl core::fmt::Debug for BodDcdcIntCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BodDcdcIntCtrl")
            .field("bodvbat_int_enable", &self.bodvbat_int_enable())
            .field("bodvbat_int_clear", &self.bodvbat_int_clear())
            .field("bodcore_int_enable", &self.bodcore_int_enable())
            .field("bodcore_int_clear", &self.bodcore_int_clear())
            .field("dcdc_int_enable", &self.dcdc_int_enable())
            .field("dcdc_int_clear", &self.dcdc_int_clear())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BodDcdcIntCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BodDcdcIntCtrl {{ bodvbat_int_enable: {=bool:?}, bodvbat_int_clear: {=bool:?}, bodcore_int_enable: {=bool:?}, bodcore_int_clear: {=bool:?}, dcdc_int_enable: {=bool:?}, dcdc_int_clear: {=bool:?} }}",
            self.bodvbat_int_enable(),
            self.bodvbat_int_clear(),
            self.bodcore_int_enable(),
            self.bodcore_int_clear(),
            self.dcdc_int_enable(),
            self.dcdc_int_clear()
        )
    }
}
#[doc = "BoDs & DCDC interrupts status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BodDcdcIntStatus(pub u32);
impl BodDcdcIntStatus {
    #[doc = "BOD VBAT Interrupt status before Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bodvbat_status(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "BOD VBAT Interrupt status before Interrupt Enable."]
    #[inline(always)]
    pub const fn set_bodvbat_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "BOD VBAT Interrupt status after Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bodvbat_int_status(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "BOD VBAT Interrupt status after Interrupt Enable."]
    #[inline(always)]
    pub const fn set_bodvbat_int_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Current value of BOD VBAT power status output."]
    #[must_use]
    #[inline(always)]
    pub const fn bodvbat_val(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Current value of BOD VBAT power status output."]
    #[inline(always)]
    pub const fn set_bodvbat_val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "BOD CORE Interrupt status before Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bodcore_status(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "BOD CORE Interrupt status before Interrupt Enable."]
    #[inline(always)]
    pub const fn set_bodcore_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "BOD CORE Interrupt status after Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bodcore_int_status(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "BOD CORE Interrupt status after Interrupt Enable."]
    #[inline(always)]
    pub const fn set_bodcore_int_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Current value of BOD CORE power status output."]
    #[must_use]
    #[inline(always)]
    pub const fn bodcore_val(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Current value of BOD CORE power status output."]
    #[inline(always)]
    pub const fn set_bodcore_val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "DCDC Interrupt status before Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dcdc_status(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DCDC Interrupt status before Interrupt Enable."]
    #[inline(always)]
    pub const fn set_dcdc_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DCDC Interrupt status after Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dcdc_int_status(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DCDC Interrupt status after Interrupt Enable."]
    #[inline(always)]
    pub const fn set_dcdc_int_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Current value of DCDC power status output."]
    #[must_use]
    #[inline(always)]
    pub const fn dcdc_val(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Current value of DCDC power status output."]
    #[inline(always)]
    pub const fn set_dcdc_val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for BodDcdcIntStatus {
    #[inline(always)]
    fn default() -> BodDcdcIntStatus {
        BodDcdcIntStatus(0)
    }
}
impl core::fmt::Debug for BodDcdcIntStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BodDcdcIntStatus")
            .field("bodvbat_status", &self.bodvbat_status())
            .field("bodvbat_int_status", &self.bodvbat_int_status())
            .field("bodvbat_val", &self.bodvbat_val())
            .field("bodcore_status", &self.bodcore_status())
            .field("bodcore_int_status", &self.bodcore_int_status())
            .field("bodcore_val", &self.bodcore_val())
            .field("dcdc_status", &self.dcdc_status())
            .field("dcdc_int_status", &self.dcdc_int_status())
            .field("dcdc_val", &self.dcdc_val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BodDcdcIntStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BodDcdcIntStatus {{ bodvbat_status: {=bool:?}, bodvbat_int_status: {=bool:?}, bodvbat_val: {=bool:?}, bodcore_status: {=bool:?}, bodcore_int_status: {=bool:?}, bodcore_val: {=bool:?}, dcdc_status: {=bool:?}, dcdc_int_status: {=bool:?}, dcdc_val: {=bool:?} }}",
            self.bodvbat_status(),
            self.bodvbat_int_status(),
            self.bodvbat_val(),
            self.bodcore_status(),
            self.bodcore_int_status(),
            self.bodcore_val(),
            self.dcdc_status(),
            self.dcdc_int_status(),
            self.dcdc_val()
        )
    }
}
#[doc = "Frequency Measure function control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FreqMeCtrl(pub u32);
impl FreqMeCtrl {
    #[doc = "Frequency measure result /Frequency measur scale"]
    #[must_use]
    #[inline(always)]
    pub const fn capval_scale(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "Frequency measure result /Frequency measur scale"]
    #[inline(always)]
    pub const fn set_capval_scale(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
    }
    #[doc = "Set this bit to one to initiate a frequency measurement cycle. Hardware clears this bit when the measurement cycle has completed and there is valid capture data in the CAPVAL field (bits 30:0)."]
    #[must_use]
    #[inline(always)]
    pub const fn prog(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Set this bit to one to initiate a frequency measurement cycle. Hardware clears this bit when the measurement cycle has completed and there is valid capture data in the CAPVAL field (bits 30:0)."]
    #[inline(always)]
    pub const fn set_prog(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for FreqMeCtrl {
    #[inline(always)]
    fn default() -> FreqMeCtrl {
        FreqMeCtrl(0)
    }
}
impl core::fmt::Debug for FreqMeCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FreqMeCtrl")
            .field("capval_scale", &self.capval_scale())
            .field("prog", &self.prog())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FreqMeCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FreqMeCtrl {{ capval_scale: {=u32:?}, prog: {=bool:?} }}",
            self.capval_scale(),
            self.prog()
        )
    }
}
#[doc = "192MHz Free Running OScillator (FRO) Control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fro192mCtrl(pub u32);
impl Fro192mCtrl {
    #[doc = "12 MHz clock control."]
    #[must_use]
    #[inline(always)]
    pub const fn ena_12mhzclk(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "12 MHz clock control."]
    #[inline(always)]
    pub const fn set_ena_12mhzclk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "48 MHz clock control."]
    #[must_use]
    #[inline(always)]
    pub const fn ena_48mhzclk(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "48 MHz clock control."]
    #[inline(always)]
    pub const fn set_ena_48mhzclk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Frequency trim."]
    #[must_use]
    #[inline(always)]
    pub const fn dac_trim(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Frequency trim."]
    #[inline(always)]
    pub const fn set_dac_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "If this bit is set and the USB peripheral is enabled into full speed device mode, the USB block will provide FRO clock adjustments to lock it to the host clock using the SOF packets."]
    #[must_use]
    #[inline(always)]
    pub const fn usbclkadj(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "If this bit is set and the USB peripheral is enabled into full speed device mode, the USB block will provide FRO clock adjustments to lock it to the host clock using the SOF packets."]
    #[inline(always)]
    pub const fn set_usbclkadj(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "If it reads as 1 when reading the DAC_TRIM field and USBCLKADJ=1, it should be re-read until it is 0."]
    #[must_use]
    #[inline(always)]
    pub const fn usbmodchg(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "If it reads as 1 when reading the DAC_TRIM field and USBCLKADJ=1, it should be re-read until it is 0."]
    #[inline(always)]
    pub const fn set_usbmodchg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "96 MHz clock control."]
    #[must_use]
    #[inline(always)]
    pub const fn ena_96mhzclk(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "96 MHz clock control."]
    #[inline(always)]
    pub const fn set_ena_96mhzclk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "This must be written to 1 to modify the BIAS_TRIM and TEMP_TRIM fields."]
    #[must_use]
    #[inline(always)]
    pub const fn wrtrim(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "This must be written to 1 to modify the BIAS_TRIM and TEMP_TRIM fields."]
    #[inline(always)]
    pub const fn set_wrtrim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Fro192mCtrl {
    #[inline(always)]
    fn default() -> Fro192mCtrl {
        Fro192mCtrl(0)
    }
}
impl core::fmt::Debug for Fro192mCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fro192mCtrl")
            .field("ena_12mhzclk", &self.ena_12mhzclk())
            .field("ena_48mhzclk", &self.ena_48mhzclk())
            .field("dac_trim", &self.dac_trim())
            .field("usbclkadj", &self.usbclkadj())
            .field("usbmodchg", &self.usbmodchg())
            .field("ena_96mhzclk", &self.ena_96mhzclk())
            .field("wrtrim", &self.wrtrim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fro192mCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fro192mCtrl {{ ena_12mhzclk: {=bool:?}, ena_48mhzclk: {=bool:?}, dac_trim: {=u8:?}, usbclkadj: {=bool:?}, usbmodchg: {=bool:?}, ena_96mhzclk: {=bool:?}, wrtrim: {=bool:?} }}",
            self.ena_12mhzclk(),
            self.ena_48mhzclk(),
            self.dac_trim(),
            self.usbclkadj(),
            self.usbmodchg(),
            self.ena_96mhzclk(),
            self.wrtrim()
        )
    }
}
#[doc = "192MHz Free Running OScillator (FRO) Status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fro192mStatus(pub u32);
impl Fro192mStatus {
    #[doc = "Output clock valid signal. Indicates that CCO clock has settled."]
    #[must_use]
    #[inline(always)]
    pub const fn clk_valid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Output clock valid signal. Indicates that CCO clock has settled."]
    #[inline(always)]
    pub const fn set_clk_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "CCO threshold voltage detector output (signal vcco_ok). Once the CCO voltage crosses the threshold voltage of a SLVT transistor, this output signal will go high. It is also possible to observe the clk_valid signal."]
    #[must_use]
    #[inline(always)]
    pub const fn atb_vctrl(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "CCO threshold voltage detector output (signal vcco_ok). Once the CCO voltage crosses the threshold voltage of a SLVT transistor, this output signal will go high. It is also possible to observe the clk_valid signal."]
    #[inline(always)]
    pub const fn set_atb_vctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Fro192mStatus {
    #[inline(always)]
    fn default() -> Fro192mStatus {
        Fro192mStatus(0)
    }
}
impl core::fmt::Debug for Fro192mStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fro192mStatus")
            .field("clk_valid", &self.clk_valid())
            .field("atb_vctrl", &self.atb_vctrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fro192mStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fro192mStatus {{ clk_valid: {=bool:?}, atb_vctrl: {=bool:?} }}",
            self.clk_valid(),
            self.atb_vctrl()
        )
    }
}
#[doc = "High Speed Crystal Oscillator (12 MHz - 32 MHz) Voltage Source Supply Control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LdoXo32m(pub u32);
impl LdoXo32m {
    #[doc = "Activate LDO bypass."]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Activate LDO bypass."]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn highz(&self) -> super::vals::Highz {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Highz::from_bits(val as u8)
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_highz(&mut self, val: super::vals::Highz) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Sets the LDO output level."]
    #[must_use]
    #[inline(always)]
    pub const fn vout(&self) -> super::vals::Vout {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::Vout::from_bits(val as u8)
    }
    #[doc = "Sets the LDO output level."]
    #[inline(always)]
    pub const fn set_vout(&mut self, val: super::vals::Vout) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
    }
    #[doc = "Adjust the biasing current."]
    #[must_use]
    #[inline(always)]
    pub const fn ibias(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Adjust the biasing current."]
    #[inline(always)]
    pub const fn set_ibias(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Stability configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn stabmode(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Stability configuration."]
    #[inline(always)]
    pub const fn set_stabmode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
}
impl Default for LdoXo32m {
    #[inline(always)]
    fn default() -> LdoXo32m {
        LdoXo32m(0)
    }
}
impl core::fmt::Debug for LdoXo32m {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LdoXo32m")
            .field("bypass", &self.bypass())
            .field("highz", &self.highz())
            .field("vout", &self.vout())
            .field("ibias", &self.ibias())
            .field("stabmode", &self.stabmode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LdoXo32m {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LdoXo32m {{ bypass: {=bool:?}, highz: {:?}, vout: {:?}, ibias: {=u8:?}, stabmode: {=u8:?} }}",
            self.bypass(),
            self.highz(),
            self.vout(),
            self.ibias(),
            self.stabmode()
        )
    }
}
#[doc = "First Ring Oscillator module control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ringo0Ctrl(pub u32);
impl Ringo0Ctrl {
    #[doc = "Select short or long ringo (for all ringos types)."]
    #[must_use]
    #[inline(always)]
    pub const fn sl(&self) -> super::vals::Sl {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sl::from_bits(val as u8)
    }
    #[doc = "Select short or long ringo (for all ringos types)."]
    #[inline(always)]
    pub const fn set_sl(&mut self, val: super::vals::Sl) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Ringo frequency output divider."]
    #[must_use]
    #[inline(always)]
    pub const fn fs(&self) -> super::vals::Ringo0CtrlFs {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ringo0CtrlFs::from_bits(val as u8)
    }
    #[doc = "Ringo frequency output divider."]
    #[inline(always)]
    pub const fn set_fs(&mut self, val: super::vals::Ringo0CtrlFs) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "PN-Ringos (P-Transistor and N-Transistor processing) control."]
    #[must_use]
    #[inline(always)]
    pub const fn swn_swp(&self) -> super::vals::SwnSwp {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwnSwp::from_bits(val as u8)
    }
    #[doc = "PN-Ringos (P-Transistor and N-Transistor processing) control."]
    #[inline(always)]
    pub const fn set_swn_swp(&mut self, val: super::vals::SwnSwp) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Ringo module Power control."]
    #[must_use]
    #[inline(always)]
    pub const fn pd(&self) -> super::vals::Ringo0CtrlPd {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Ringo0CtrlPd::from_bits(val as u8)
    }
    #[doc = "Ringo module Power control."]
    #[inline(always)]
    pub const fn set_pd(&mut self, val: super::vals::Ringo0CtrlPd) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "First NAND2-based ringo control."]
    #[must_use]
    #[inline(always)]
    pub const fn e_nd0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "First NAND2-based ringo control."]
    #[inline(always)]
    pub const fn set_e_nd0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Second NAND2-based ringo control."]
    #[must_use]
    #[inline(always)]
    pub const fn e_nd1(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Second NAND2-based ringo control."]
    #[inline(always)]
    pub const fn set_e_nd1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "First NOR2-based ringo control."]
    #[must_use]
    #[inline(always)]
    pub const fn e_nr0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "First NOR2-based ringo control."]
    #[inline(always)]
    pub const fn set_e_nr0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Second NOR2-based ringo control."]
    #[must_use]
    #[inline(always)]
    pub const fn e_nr1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Second NOR2-based ringo control."]
    #[inline(always)]
    pub const fn set_e_nr1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "First Inverter-based ringo control."]
    #[must_use]
    #[inline(always)]
    pub const fn e_iv0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "First Inverter-based ringo control."]
    #[inline(always)]
    pub const fn set_e_iv0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Second Inverter-based ringo control."]
    #[must_use]
    #[inline(always)]
    pub const fn e_iv1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Second Inverter-based ringo control."]
    #[inline(always)]
    pub const fn set_e_iv1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "First PN (P-Transistor and N-Transistor processing) monitor control."]
    #[must_use]
    #[inline(always)]
    pub const fn e_pn0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "First PN (P-Transistor and N-Transistor processing) monitor control."]
    #[inline(always)]
    pub const fn set_e_pn0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Second PN (P-Transistor and N-Transistor processing) monitor control."]
    #[must_use]
    #[inline(always)]
    pub const fn e_pn1(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Second PN (P-Transistor and N-Transistor processing) monitor control."]
    #[inline(always)]
    pub const fn set_e_pn1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Ringo out Clock divider value. Frequency Output = Frequency input / (DIViSOR+1). (minimum = Frequency input / 16)"]
    #[must_use]
    #[inline(always)]
    pub const fn divisor(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Ringo out Clock divider value. Frequency Output = Frequency input / (DIViSOR+1). (minimum = Frequency input / 16)"]
    #[inline(always)]
    pub const fn set_divisor(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Ringo clock out Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[must_use]
    #[inline(always)]
    pub const fn div_update_req(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Ringo clock out Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub const fn set_div_update_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ringo0Ctrl {
    #[inline(always)]
    fn default() -> Ringo0Ctrl {
        Ringo0Ctrl(0)
    }
}
impl core::fmt::Debug for Ringo0Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ringo0Ctrl")
            .field("sl", &self.sl())
            .field("fs", &self.fs())
            .field("swn_swp", &self.swn_swp())
            .field("pd", &self.pd())
            .field("e_nd0", &self.e_nd0())
            .field("e_nd1", &self.e_nd1())
            .field("e_nr0", &self.e_nr0())
            .field("e_nr1", &self.e_nr1())
            .field("e_iv0", &self.e_iv0())
            .field("e_iv1", &self.e_iv1())
            .field("e_pn0", &self.e_pn0())
            .field("e_pn1", &self.e_pn1())
            .field("divisor", &self.divisor())
            .field("div_update_req", &self.div_update_req())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ringo0Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ringo0Ctrl {{ sl: {:?}, fs: {:?}, swn_swp: {:?}, pd: {:?}, e_nd0: {=bool:?}, e_nd1: {=bool:?}, e_nr0: {=bool:?}, e_nr1: {=bool:?}, e_iv0: {=bool:?}, e_iv1: {=bool:?}, e_pn0: {=bool:?}, e_pn1: {=bool:?}, divisor: {=u8:?}, div_update_req: {=bool:?} }}",
            self.sl(),
            self.fs(),
            self.swn_swp(),
            self.pd(),
            self.e_nd0(),
            self.e_nd1(),
            self.e_nr0(),
            self.e_nr1(),
            self.e_iv0(),
            self.e_iv1(),
            self.e_pn0(),
            self.e_pn1(),
            self.divisor(),
            self.div_update_req()
        )
    }
}
#[doc = "Second Ring Oscillator module control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ringo1Ctrl(pub u32);
impl Ringo1Ctrl {
    #[doc = "Select short or long ringo (for all ringos types)."]
    #[must_use]
    #[inline(always)]
    pub const fn s(&self) -> super::vals::Ringo1CtrlS {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ringo1CtrlS::from_bits(val as u8)
    }
    #[doc = "Select short or long ringo (for all ringos types)."]
    #[inline(always)]
    pub const fn set_s(&mut self, val: super::vals::Ringo1CtrlS) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Ringo frequency output divider."]
    #[must_use]
    #[inline(always)]
    pub const fn fs(&self) -> super::vals::Ringo1CtrlFs {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ringo1CtrlFs::from_bits(val as u8)
    }
    #[doc = "Ringo frequency output divider."]
    #[inline(always)]
    pub const fn set_fs(&mut self, val: super::vals::Ringo1CtrlFs) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Ringo module Power control."]
    #[must_use]
    #[inline(always)]
    pub const fn pd(&self) -> super::vals::Ringo1CtrlPd {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Ringo1CtrlPd::from_bits(val as u8)
    }
    #[doc = "Ringo module Power control."]
    #[inline(always)]
    pub const fn set_pd(&mut self, val: super::vals::Ringo1CtrlPd) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn e_r24(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_e_r24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn e_r35(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_e_r35(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Metal 2 (M2) monitor control."]
    #[must_use]
    #[inline(always)]
    pub const fn e_m2(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Metal 2 (M2) monitor control."]
    #[inline(always)]
    pub const fn set_e_m2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Metal 3 (M3) monitor control."]
    #[must_use]
    #[inline(always)]
    pub const fn e_m3(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Metal 3 (M3) monitor control."]
    #[inline(always)]
    pub const fn set_e_m3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Metal 4 (M4) monitor control."]
    #[must_use]
    #[inline(always)]
    pub const fn e_m4(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Metal 4 (M4) monitor control."]
    #[inline(always)]
    pub const fn set_e_m4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Metal 5 (M5) monitor control."]
    #[must_use]
    #[inline(always)]
    pub const fn e_m5(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Metal 5 (M5) monitor control."]
    #[inline(always)]
    pub const fn set_e_m5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Ringo out Clock divider value. Frequency Output = Frequency input / (DIViSOR+1). (minimum = Frequency input / 16)"]
    #[must_use]
    #[inline(always)]
    pub const fn divisor(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Ringo out Clock divider value. Frequency Output = Frequency input / (DIViSOR+1). (minimum = Frequency input / 16)"]
    #[inline(always)]
    pub const fn set_divisor(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Ringo clock out Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[must_use]
    #[inline(always)]
    pub const fn div_update_req(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Ringo clock out Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub const fn set_div_update_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ringo1Ctrl {
    #[inline(always)]
    fn default() -> Ringo1Ctrl {
        Ringo1Ctrl(0)
    }
}
impl core::fmt::Debug for Ringo1Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ringo1Ctrl")
            .field("s", &self.s())
            .field("fs", &self.fs())
            .field("pd", &self.pd())
            .field("e_r24", &self.e_r24())
            .field("e_r35", &self.e_r35())
            .field("e_m2", &self.e_m2())
            .field("e_m3", &self.e_m3())
            .field("e_m4", &self.e_m4())
            .field("e_m5", &self.e_m5())
            .field("divisor", &self.divisor())
            .field("div_update_req", &self.div_update_req())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ringo1Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ringo1Ctrl {{ s: {:?}, fs: {:?}, pd: {:?}, e_r24: {=bool:?}, e_r35: {=bool:?}, e_m2: {=bool:?}, e_m3: {=bool:?}, e_m4: {=bool:?}, e_m5: {=bool:?}, divisor: {=u8:?}, div_update_req: {=bool:?} }}",
            self.s(),
            self.fs(),
            self.pd(),
            self.e_r24(),
            self.e_r35(),
            self.e_m2(),
            self.e_m3(),
            self.e_m4(),
            self.e_m5(),
            self.divisor(),
            self.div_update_req()
        )
    }
}
#[doc = "Third Ring Oscillator module control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ringo2Ctrl(pub u32);
impl Ringo2Ctrl {
    #[doc = "Select short or long ringo (for all ringos types)."]
    #[must_use]
    #[inline(always)]
    pub const fn s(&self) -> super::vals::Ringo2CtrlS {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ringo2CtrlS::from_bits(val as u8)
    }
    #[doc = "Select short or long ringo (for all ringos types)."]
    #[inline(always)]
    pub const fn set_s(&mut self, val: super::vals::Ringo2CtrlS) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Ringo frequency output divider."]
    #[must_use]
    #[inline(always)]
    pub const fn fs(&self) -> super::vals::Ringo2CtrlFs {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ringo2CtrlFs::from_bits(val as u8)
    }
    #[doc = "Ringo frequency output divider."]
    #[inline(always)]
    pub const fn set_fs(&mut self, val: super::vals::Ringo2CtrlFs) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Ringo module Power control."]
    #[must_use]
    #[inline(always)]
    pub const fn pd(&self) -> super::vals::Ringo2CtrlPd {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Ringo2CtrlPd::from_bits(val as u8)
    }
    #[doc = "Ringo module Power control."]
    #[inline(always)]
    pub const fn set_pd(&mut self, val: super::vals::Ringo2CtrlPd) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn e_r24(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_e_r24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn e_r35(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_e_r35(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Metal 2 (M2) monitor control."]
    #[must_use]
    #[inline(always)]
    pub const fn e_m2(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Metal 2 (M2) monitor control."]
    #[inline(always)]
    pub const fn set_e_m2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Metal 3 (M3) monitor control."]
    #[must_use]
    #[inline(always)]
    pub const fn e_m3(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Metal 3 (M3) monitor control."]
    #[inline(always)]
    pub const fn set_e_m3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Metal 4 (M4) monitor control."]
    #[must_use]
    #[inline(always)]
    pub const fn e_m4(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Metal 4 (M4) monitor control."]
    #[inline(always)]
    pub const fn set_e_m4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Metal 5 (M5) monitor control."]
    #[must_use]
    #[inline(always)]
    pub const fn e_m5(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Metal 5 (M5) monitor control."]
    #[inline(always)]
    pub const fn set_e_m5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Ringo out Clock divider value. Frequency Output = Frequency input / (DIViSOR+1). (minimum = Frequency input / 16)"]
    #[must_use]
    #[inline(always)]
    pub const fn divisor(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Ringo out Clock divider value. Frequency Output = Frequency input / (DIViSOR+1). (minimum = Frequency input / 16)"]
    #[inline(always)]
    pub const fn set_divisor(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Ringo clock out Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[must_use]
    #[inline(always)]
    pub const fn div_update_req(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Ringo clock out Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub const fn set_div_update_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ringo2Ctrl {
    #[inline(always)]
    fn default() -> Ringo2Ctrl {
        Ringo2Ctrl(0)
    }
}
impl core::fmt::Debug for Ringo2Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ringo2Ctrl")
            .field("s", &self.s())
            .field("fs", &self.fs())
            .field("pd", &self.pd())
            .field("e_r24", &self.e_r24())
            .field("e_r35", &self.e_r35())
            .field("e_m2", &self.e_m2())
            .field("e_m3", &self.e_m3())
            .field("e_m4", &self.e_m4())
            .field("e_m5", &self.e_m5())
            .field("divisor", &self.divisor())
            .field("div_update_req", &self.div_update_req())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ringo2Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ringo2Ctrl {{ s: {:?}, fs: {:?}, pd: {:?}, e_r24: {=bool:?}, e_r35: {=bool:?}, e_m2: {=bool:?}, e_m3: {=bool:?}, e_m4: {=bool:?}, e_m5: {=bool:?}, divisor: {=u8:?}, div_update_req: {=bool:?} }}",
            self.s(),
            self.fs(),
            self.pd(),
            self.e_r24(),
            self.e_r35(),
            self.e_m2(),
            self.e_m3(),
            self.e_m4(),
            self.e_m5(),
            self.divisor(),
            self.div_update_req()
        )
    }
}
#[doc = "USB High Speed Phy Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbhsPhyCtrl(pub u32);
impl UsbhsPhyCtrl {
    #[doc = "Override value for Vbus if using external detectors."]
    #[must_use]
    #[inline(always)]
    pub const fn usb_vbusvalid_ext(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Override value for Vbus if using external detectors."]
    #[inline(always)]
    pub const fn set_usb_vbusvalid_ext(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Override value for ID if using external detectors."]
    #[must_use]
    #[inline(always)]
    pub const fn usb_id_ext(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Override value for ID if using external detectors."]
    #[inline(always)]
    pub const fn set_usb_id_ext(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for UsbhsPhyCtrl {
    #[inline(always)]
    fn default() -> UsbhsPhyCtrl {
        UsbhsPhyCtrl(0)
    }
}
impl core::fmt::Debug for UsbhsPhyCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UsbhsPhyCtrl")
            .field("usb_vbusvalid_ext", &self.usb_vbusvalid_ext())
            .field("usb_id_ext", &self.usb_id_ext())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UsbhsPhyCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UsbhsPhyCtrl {{ usb_vbusvalid_ext: {=bool:?}, usb_id_ext: {=bool:?} }}",
            self.usb_vbusvalid_ext(),
            self.usb_id_ext()
        )
    }
}
#[doc = "USB High Speed Phy Trim values"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbhsPhyTrim(pub u32);
impl UsbhsPhyTrim {
    #[doc = "Adjusts time constant of HS RX squelch (envelope) comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_usb_reg_env_tail_adj_vd(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Adjusts time constant of HS RX squelch (envelope) comparator."]
    #[inline(always)]
    pub const fn set_trim_usb_reg_env_tail_adj_vd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_usbphy_tx_d_cal(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x0f;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_trim_usbphy_tx_d_cal(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_usbphy_tx_cal45dp(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x1f;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_trim_usbphy_tx_cal45dp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_usbphy_tx_cal45dm(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_trim_usbphy_tx_cal45dm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_usb2_refbias_tst(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_trim_usb2_refbias_tst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_usb2_refbias_vbgadj(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x07;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_trim_usb2_refbias_vbgadj(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn trim_pll_ctrl0_div_sel(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x07;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_trim_pll_ctrl0_div_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
    }
}
impl Default for UsbhsPhyTrim {
    #[inline(always)]
    fn default() -> UsbhsPhyTrim {
        UsbhsPhyTrim(0)
    }
}
impl core::fmt::Debug for UsbhsPhyTrim {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UsbhsPhyTrim")
            .field(
                "trim_usb_reg_env_tail_adj_vd",
                &self.trim_usb_reg_env_tail_adj_vd(),
            )
            .field("trim_usbphy_tx_d_cal", &self.trim_usbphy_tx_d_cal())
            .field("trim_usbphy_tx_cal45dp", &self.trim_usbphy_tx_cal45dp())
            .field("trim_usbphy_tx_cal45dm", &self.trim_usbphy_tx_cal45dm())
            .field("trim_usb2_refbias_tst", &self.trim_usb2_refbias_tst())
            .field("trim_usb2_refbias_vbgadj", &self.trim_usb2_refbias_vbgadj())
            .field("trim_pll_ctrl0_div_sel", &self.trim_pll_ctrl0_div_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UsbhsPhyTrim {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UsbhsPhyTrim {{ trim_usb_reg_env_tail_adj_vd: {=u8:?}, trim_usbphy_tx_d_cal: {=u8:?}, trim_usbphy_tx_cal45dp: {=u8:?}, trim_usbphy_tx_cal45dm: {=u8:?}, trim_usb2_refbias_tst: {=u8:?}, trim_usb2_refbias_vbgadj: {=u8:?}, trim_pll_ctrl0_div_sel: {=u8:?} }}",
            self.trim_usb_reg_env_tail_adj_vd(),
            self.trim_usbphy_tx_d_cal(),
            self.trim_usbphy_tx_cal45dp(),
            self.trim_usbphy_tx_cal45dm(),
            self.trim_usb2_refbias_tst(),
            self.trim_usb2_refbias_vbgadj(),
            self.trim_pll_ctrl0_div_sel()
        )
    }
}
#[doc = "High speed Crystal Oscillator Control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xo32mCtrl(pub u32);
impl Xo32mCtrl {
    #[doc = "Xo in slave mode."]
    #[must_use]
    #[inline(always)]
    pub const fn slave(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Xo in slave mode."]
    #[inline(always)]
    pub const fn set_slave(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Tune capa banks of High speed Crystal Oscillator input pin"]
    #[must_use]
    #[inline(always)]
    pub const fn osc_cap_in(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Tune capa banks of High speed Crystal Oscillator input pin"]
    #[inline(always)]
    pub const fn set_osc_cap_in(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Tune capa banks of High speed Crystal Oscillator output pin"]
    #[must_use]
    #[inline(always)]
    pub const fn osc_cap_out(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x7f;
        val as u8
    }
    #[doc = "Tune capa banks of High speed Crystal Oscillator output pin"]
    #[inline(always)]
    pub const fn set_osc_cap_out(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 15usize)) | (((val as u32) & 0x7f) << 15usize);
    }
    #[doc = "Bypass enable of XO AC buffer enable in pll and top level."]
    #[must_use]
    #[inline(always)]
    pub const fn acbuf_pass_enable(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass enable of XO AC buffer enable in pll and top level."]
    #[inline(always)]
    pub const fn set_acbuf_pass_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Enable High speed Crystal oscillator output to USB HS PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_pll_usb_out(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enable High speed Crystal oscillator output to USB HS PLL."]
    #[inline(always)]
    pub const fn set_enable_pll_usb_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Enable High speed Crystal oscillator output to CPU system."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_system_clk_out(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Enable High speed Crystal oscillator output to CPU system."]
    #[inline(always)]
    pub const fn set_enable_system_clk_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Xo32mCtrl {
    #[inline(always)]
    fn default() -> Xo32mCtrl {
        Xo32mCtrl(0)
    }
}
impl core::fmt::Debug for Xo32mCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xo32mCtrl")
            .field("slave", &self.slave())
            .field("osc_cap_in", &self.osc_cap_in())
            .field("osc_cap_out", &self.osc_cap_out())
            .field("acbuf_pass_enable", &self.acbuf_pass_enable())
            .field("enable_pll_usb_out", &self.enable_pll_usb_out())
            .field("enable_system_clk_out", &self.enable_system_clk_out())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xo32mCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Xo32mCtrl {{ slave: {=bool:?}, osc_cap_in: {=u8:?}, osc_cap_out: {=u8:?}, acbuf_pass_enable: {=bool:?}, enable_pll_usb_out: {=bool:?}, enable_system_clk_out: {=bool:?} }}",
            self.slave(),
            self.osc_cap_in(),
            self.osc_cap_out(),
            self.acbuf_pass_enable(),
            self.enable_pll_usb_out(),
            self.enable_system_clk_out()
        )
    }
}
#[doc = "High speed Crystal Oscillator Status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xo32mStatus(pub u32);
impl Xo32mStatus {
    #[doc = "Indicates XO out frequency statibilty."]
    #[must_use]
    #[inline(always)]
    pub const fn xo_ready(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates XO out frequency statibilty."]
    #[inline(always)]
    pub const fn set_xo_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Xo32mStatus {
    #[inline(always)]
    fn default() -> Xo32mStatus {
        Xo32mStatus(0)
    }
}
impl core::fmt::Debug for Xo32mStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Xo32mStatus")
            .field("xo_ready", &self.xo_ready())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Xo32mStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Xo32mStatus {{ xo_ready: {=bool:?} }}", self.xo_ready())
    }
}
