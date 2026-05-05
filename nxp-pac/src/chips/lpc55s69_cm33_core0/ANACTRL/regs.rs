#[doc = "General Purpose ADC VBAT Divider branch control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC_CTRL(pub u32);
impl ADC_CTRL {
    #[doc = "Switch On/Off VBAT divider branch."]
    #[must_use]
    #[inline(always)]
    pub const fn VBATDIVENABLE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Switch On/Off VBAT divider branch."]
    #[inline(always)]
    pub const fn set_VBATDIVENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for ADC_CTRL {
    #[inline(always)]
    fn default() -> ADC_CTRL {
        ADC_CTRL(0)
    }
}
impl core::fmt::Debug for ADC_CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_CTRL")
            .field("VBATDIVENABLE", &self.VBATDIVENABLE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ADC_CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ADC_CTRL {{ VBATDIVENABLE: {=bool:?} }}",
            self.VBATDIVENABLE()
        )
    }
}
#[doc = "Various Analog blocks configuration (like FRO 192MHz trimmings source ...)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ANALOG_CTRL_CFG(pub u32);
impl ANALOG_CTRL_CFG {
    #[doc = "FRO192M trimming and 'Enable' source."]
    #[must_use]
    #[inline(always)]
    pub const fn FRO192M_TRIM_SRC(&self) -> super::vals::FRO192M_TRIM_SRC {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::FRO192M_TRIM_SRC::from_bits(val as u8)
    }
    #[doc = "FRO192M trimming and 'Enable' source."]
    #[inline(always)]
    pub const fn set_FRO192M_TRIM_SRC(&mut self, val: super::vals::FRO192M_TRIM_SRC) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for ANALOG_CTRL_CFG {
    #[inline(always)]
    fn default() -> ANALOG_CTRL_CFG {
        ANALOG_CTRL_CFG(0)
    }
}
impl core::fmt::Debug for ANALOG_CTRL_CFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ANALOG_CTRL_CFG")
            .field("FRO192M_TRIM_SRC", &self.FRO192M_TRIM_SRC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ANALOG_CTRL_CFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ANALOG_CTRL_CFG {{ FRO192M_TRIM_SRC: {:?} }}",
            self.FRO192M_TRIM_SRC()
        )
    }
}
#[doc = "Analog Macroblock Identity registers, Flash Status registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ANALOG_CTRL_STATUS(pub u32);
impl ANALOG_CTRL_STATUS {
    #[doc = "Flash Power Down status."]
    #[must_use]
    #[inline(always)]
    pub const fn FLASH_PWRDWN(&self) -> super::vals::FLASH_PWRDWN {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::FLASH_PWRDWN::from_bits(val as u8)
    }
    #[doc = "Flash Power Down status."]
    #[inline(always)]
    pub const fn set_FLASH_PWRDWN(&mut self, val: super::vals::FLASH_PWRDWN) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Flash initialization error status."]
    #[must_use]
    #[inline(always)]
    pub const fn FLASH_INIT_ERROR(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Flash initialization error status."]
    #[inline(always)]
    pub const fn set_FLASH_INIT_ERROR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for ANALOG_CTRL_STATUS {
    #[inline(always)]
    fn default() -> ANALOG_CTRL_STATUS {
        ANALOG_CTRL_STATUS(0)
    }
}
impl core::fmt::Debug for ANALOG_CTRL_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ANALOG_CTRL_STATUS")
            .field("FLASH_PWRDWN", &self.FLASH_PWRDWN())
            .field("FLASH_INIT_ERROR", &self.FLASH_INIT_ERROR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ANALOG_CTRL_STATUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ANALOG_CTRL_STATUS {{ FLASH_PWRDWN: {:?}, FLASH_INIT_ERROR: {=bool:?} }}",
            self.FLASH_PWRDWN(),
            self.FLASH_INIT_ERROR()
        )
    }
}
#[doc = "AUX_BIAS."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AUX_BIAS(pub u32);
impl AUX_BIAS {
    #[doc = "Control output of 1V reference voltage."]
    #[must_use]
    #[inline(always)]
    pub const fn VREF1VENABLE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Control output of 1V reference voltage."]
    #[inline(always)]
    pub const fn set_VREF1VENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "current trimming control word."]
    #[must_use]
    #[inline(always)]
    pub const fn ITRIM(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x1f;
        val as u8
    }
    #[doc = "current trimming control word."]
    #[inline(always)]
    pub const fn set_ITRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 2usize)) | (((val as u32) & 0x1f) << 2usize);
    }
    #[doc = "current trimming control word for ptat current."]
    #[must_use]
    #[inline(always)]
    pub const fn PTATITRIM(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x1f;
        val as u8
    }
    #[doc = "current trimming control word for ptat current."]
    #[inline(always)]
    pub const fn set_PTATITRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 7usize)) | (((val as u32) & 0x1f) << 7usize);
    }
    #[doc = "voltage trimming control word."]
    #[must_use]
    #[inline(always)]
    pub const fn VREF1VTRIM(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x1f;
        val as u8
    }
    #[doc = "voltage trimming control word."]
    #[inline(always)]
    pub const fn set_VREF1VTRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 12usize)) | (((val as u32) & 0x1f) << 12usize);
    }
    #[doc = "Control bit to configure trimming state of mirror."]
    #[must_use]
    #[inline(always)]
    pub const fn VREF1VCURVETRIM(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x07;
        val as u8
    }
    #[doc = "Control bit to configure trimming state of mirror."]
    #[inline(always)]
    pub const fn set_VREF1VCURVETRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
    }
    #[doc = "Control bit to configure trimming state of mirror."]
    #[must_use]
    #[inline(always)]
    pub const fn ITRIMCTRL0(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to configure trimming state of mirror."]
    #[inline(always)]
    pub const fn set_ITRIMCTRL0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Control bit to configure trimming state of mirror."]
    #[must_use]
    #[inline(always)]
    pub const fn ITRIMCTRL1(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to configure trimming state of mirror."]
    #[inline(always)]
    pub const fn set_ITRIMCTRL1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for AUX_BIAS {
    #[inline(always)]
    fn default() -> AUX_BIAS {
        AUX_BIAS(0)
    }
}
impl core::fmt::Debug for AUX_BIAS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AUX_BIAS")
            .field("VREF1VENABLE", &self.VREF1VENABLE())
            .field("ITRIM", &self.ITRIM())
            .field("PTATITRIM", &self.PTATITRIM())
            .field("VREF1VTRIM", &self.VREF1VTRIM())
            .field("VREF1VCURVETRIM", &self.VREF1VCURVETRIM())
            .field("ITRIMCTRL0", &self.ITRIMCTRL0())
            .field("ITRIMCTRL1", &self.ITRIMCTRL1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AUX_BIAS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AUX_BIAS {{ VREF1VENABLE: {=bool:?}, ITRIM: {=u8:?}, PTATITRIM: {=u8:?}, VREF1VTRIM: {=u8:?}, VREF1VCURVETRIM: {=u8:?}, ITRIMCTRL0: {=bool:?}, ITRIMCTRL1: {=bool:?} }}",
            self.VREF1VENABLE(),
            self.ITRIM(),
            self.PTATITRIM(),
            self.VREF1VTRIM(),
            self.VREF1VCURVETRIM(),
            self.ITRIMCTRL0(),
            self.ITRIMCTRL1()
        )
    }
}
#[doc = "Brown Out Detectors (BoDs) & DCDC interrupts generation control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BOD_DCDC_INT_CTRL(pub u32);
impl BOD_DCDC_INT_CTRL {
    #[doc = "BOD VBAT interrupt control."]
    #[must_use]
    #[inline(always)]
    pub const fn BODVBAT_INT_ENABLE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "BOD VBAT interrupt control."]
    #[inline(always)]
    pub const fn set_BODVBAT_INT_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "BOD VBAT interrupt clear.1: Clear the interrupt. Self-cleared bit."]
    #[must_use]
    #[inline(always)]
    pub const fn BODVBAT_INT_CLEAR(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "BOD VBAT interrupt clear.1: Clear the interrupt. Self-cleared bit."]
    #[inline(always)]
    pub const fn set_BODVBAT_INT_CLEAR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "BOD CORE interrupt control."]
    #[must_use]
    #[inline(always)]
    pub const fn BODCORE_INT_ENABLE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "BOD CORE interrupt control."]
    #[inline(always)]
    pub const fn set_BODCORE_INT_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "BOD CORE interrupt clear.1: Clear the interrupt. Self-cleared bit."]
    #[must_use]
    #[inline(always)]
    pub const fn BODCORE_INT_CLEAR(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "BOD CORE interrupt clear.1: Clear the interrupt. Self-cleared bit."]
    #[inline(always)]
    pub const fn set_BODCORE_INT_CLEAR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "DCDC interrupt control."]
    #[must_use]
    #[inline(always)]
    pub const fn DCDC_INT_ENABLE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "DCDC interrupt control."]
    #[inline(always)]
    pub const fn set_DCDC_INT_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "DCDC interrupt clear.1: Clear the interrupt. Self-cleared bit."]
    #[must_use]
    #[inline(always)]
    pub const fn DCDC_INT_CLEAR(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "DCDC interrupt clear.1: Clear the interrupt. Self-cleared bit."]
    #[inline(always)]
    pub const fn set_DCDC_INT_CLEAR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for BOD_DCDC_INT_CTRL {
    #[inline(always)]
    fn default() -> BOD_DCDC_INT_CTRL {
        BOD_DCDC_INT_CTRL(0)
    }
}
impl core::fmt::Debug for BOD_DCDC_INT_CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BOD_DCDC_INT_CTRL")
            .field("BODVBAT_INT_ENABLE", &self.BODVBAT_INT_ENABLE())
            .field("BODVBAT_INT_CLEAR", &self.BODVBAT_INT_CLEAR())
            .field("BODCORE_INT_ENABLE", &self.BODCORE_INT_ENABLE())
            .field("BODCORE_INT_CLEAR", &self.BODCORE_INT_CLEAR())
            .field("DCDC_INT_ENABLE", &self.DCDC_INT_ENABLE())
            .field("DCDC_INT_CLEAR", &self.DCDC_INT_CLEAR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BOD_DCDC_INT_CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BOD_DCDC_INT_CTRL {{ BODVBAT_INT_ENABLE: {=bool:?}, BODVBAT_INT_CLEAR: {=bool:?}, BODCORE_INT_ENABLE: {=bool:?}, BODCORE_INT_CLEAR: {=bool:?}, DCDC_INT_ENABLE: {=bool:?}, DCDC_INT_CLEAR: {=bool:?} }}",
            self.BODVBAT_INT_ENABLE(),
            self.BODVBAT_INT_CLEAR(),
            self.BODCORE_INT_ENABLE(),
            self.BODCORE_INT_CLEAR(),
            self.DCDC_INT_ENABLE(),
            self.DCDC_INT_CLEAR()
        )
    }
}
#[doc = "BoDs & DCDC interrupts status register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BOD_DCDC_INT_STATUS(pub u32);
impl BOD_DCDC_INT_STATUS {
    #[doc = "BOD VBAT Interrupt status before Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn BODVBAT_STATUS(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "BOD VBAT Interrupt status before Interrupt Enable."]
    #[inline(always)]
    pub const fn set_BODVBAT_STATUS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "BOD VBAT Interrupt status after Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn BODVBAT_INT_STATUS(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "BOD VBAT Interrupt status after Interrupt Enable."]
    #[inline(always)]
    pub const fn set_BODVBAT_INT_STATUS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Current value of BOD VBAT power status output."]
    #[must_use]
    #[inline(always)]
    pub const fn BODVBAT_VAL(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Current value of BOD VBAT power status output."]
    #[inline(always)]
    pub const fn set_BODVBAT_VAL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "BOD CORE Interrupt status before Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn BODCORE_STATUS(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "BOD CORE Interrupt status before Interrupt Enable."]
    #[inline(always)]
    pub const fn set_BODCORE_STATUS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "BOD CORE Interrupt status after Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn BODCORE_INT_STATUS(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "BOD CORE Interrupt status after Interrupt Enable."]
    #[inline(always)]
    pub const fn set_BODCORE_INT_STATUS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Current value of BOD CORE power status output."]
    #[must_use]
    #[inline(always)]
    pub const fn BODCORE_VAL(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Current value of BOD CORE power status output."]
    #[inline(always)]
    pub const fn set_BODCORE_VAL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "DCDC Interrupt status before Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn DCDC_STATUS(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DCDC Interrupt status before Interrupt Enable."]
    #[inline(always)]
    pub const fn set_DCDC_STATUS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DCDC Interrupt status after Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn DCDC_INT_STATUS(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DCDC Interrupt status after Interrupt Enable."]
    #[inline(always)]
    pub const fn set_DCDC_INT_STATUS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Current value of DCDC power status output."]
    #[must_use]
    #[inline(always)]
    pub const fn DCDC_VAL(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Current value of DCDC power status output."]
    #[inline(always)]
    pub const fn set_DCDC_VAL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for BOD_DCDC_INT_STATUS {
    #[inline(always)]
    fn default() -> BOD_DCDC_INT_STATUS {
        BOD_DCDC_INT_STATUS(0)
    }
}
impl core::fmt::Debug for BOD_DCDC_INT_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BOD_DCDC_INT_STATUS")
            .field("BODVBAT_STATUS", &self.BODVBAT_STATUS())
            .field("BODVBAT_INT_STATUS", &self.BODVBAT_INT_STATUS())
            .field("BODVBAT_VAL", &self.BODVBAT_VAL())
            .field("BODCORE_STATUS", &self.BODCORE_STATUS())
            .field("BODCORE_INT_STATUS", &self.BODCORE_INT_STATUS())
            .field("BODCORE_VAL", &self.BODCORE_VAL())
            .field("DCDC_STATUS", &self.DCDC_STATUS())
            .field("DCDC_INT_STATUS", &self.DCDC_INT_STATUS())
            .field("DCDC_VAL", &self.DCDC_VAL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BOD_DCDC_INT_STATUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BOD_DCDC_INT_STATUS {{ BODVBAT_STATUS: {=bool:?}, BODVBAT_INT_STATUS: {=bool:?}, BODVBAT_VAL: {=bool:?}, BODCORE_STATUS: {=bool:?}, BODCORE_INT_STATUS: {=bool:?}, BODCORE_VAL: {=bool:?}, DCDC_STATUS: {=bool:?}, DCDC_INT_STATUS: {=bool:?}, DCDC_VAL: {=bool:?} }}",
            self.BODVBAT_STATUS(),
            self.BODVBAT_INT_STATUS(),
            self.BODVBAT_VAL(),
            self.BODCORE_STATUS(),
            self.BODCORE_INT_STATUS(),
            self.BODCORE_VAL(),
            self.DCDC_STATUS(),
            self.DCDC_INT_STATUS(),
            self.DCDC_VAL()
        )
    }
}
#[doc = "Frequency Measure function control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FREQ_ME_CTRL(pub u32);
impl FREQ_ME_CTRL {
    #[doc = "Frequency measure result /Frequency measur scale."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPVAL_SCALE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "Frequency measure result /Frequency measur scale."]
    #[inline(always)]
    pub const fn set_CAPVAL_SCALE(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
    }
    #[doc = "Set this bit to one to initiate a frequency measurement cycle. Hardware clears this bit when the measurement cycle has completed and there is valid capture data in the CAPVAL field (bits 30:0)."]
    #[must_use]
    #[inline(always)]
    pub const fn PROG(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Set this bit to one to initiate a frequency measurement cycle. Hardware clears this bit when the measurement cycle has completed and there is valid capture data in the CAPVAL field (bits 30:0)."]
    #[inline(always)]
    pub const fn set_PROG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for FREQ_ME_CTRL {
    #[inline(always)]
    fn default() -> FREQ_ME_CTRL {
        FREQ_ME_CTRL(0)
    }
}
impl core::fmt::Debug for FREQ_ME_CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FREQ_ME_CTRL")
            .field("CAPVAL_SCALE", &self.CAPVAL_SCALE())
            .field("PROG", &self.PROG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FREQ_ME_CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FREQ_ME_CTRL {{ CAPVAL_SCALE: {=u32:?}, PROG: {=bool:?} }}",
            self.CAPVAL_SCALE(),
            self.PROG()
        )
    }
}
#[doc = "192MHz Free Running OScillator (FRO) Control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FRO192M_CTRL(pub u32);
impl FRO192M_CTRL {
    #[doc = "12 MHz clock control."]
    #[must_use]
    #[inline(always)]
    pub const fn ENA_12MHZCLK(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "12 MHz clock control."]
    #[inline(always)]
    pub const fn set_ENA_12MHZCLK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "48 MHz clock control."]
    #[must_use]
    #[inline(always)]
    pub const fn ENA_48MHZCLK(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "48 MHz clock control."]
    #[inline(always)]
    pub const fn set_ENA_48MHZCLK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Frequency trim."]
    #[must_use]
    #[inline(always)]
    pub const fn DAC_TRIM(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Frequency trim."]
    #[inline(always)]
    pub const fn set_DAC_TRIM(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "If this bit is set and the USB peripheral is enabled into full speed device mode, the USB block will provide FRO clock adjustments to lock it to the host clock using the SOF packets."]
    #[must_use]
    #[inline(always)]
    pub const fn USBCLKADJ(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "If this bit is set and the USB peripheral is enabled into full speed device mode, the USB block will provide FRO clock adjustments to lock it to the host clock using the SOF packets."]
    #[inline(always)]
    pub const fn set_USBCLKADJ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "If it reads as 1 when reading the DAC_TRIM field and USBCLKADJ=1, it should be re-read until it is 0."]
    #[must_use]
    #[inline(always)]
    pub const fn USBMODCHG(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "If it reads as 1 when reading the DAC_TRIM field and USBCLKADJ=1, it should be re-read until it is 0."]
    #[inline(always)]
    pub const fn set_USBMODCHG(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "96 MHz clock control."]
    #[must_use]
    #[inline(always)]
    pub const fn ENA_96MHZCLK(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "96 MHz clock control."]
    #[inline(always)]
    pub const fn set_ENA_96MHZCLK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "This must be written to 1 to modify the BIAS_TRIM and TEMP_TRIM fields."]
    #[must_use]
    #[inline(always)]
    pub const fn WRTRIM(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "This must be written to 1 to modify the BIAS_TRIM and TEMP_TRIM fields."]
    #[inline(always)]
    pub const fn set_WRTRIM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for FRO192M_CTRL {
    #[inline(always)]
    fn default() -> FRO192M_CTRL {
        FRO192M_CTRL(0)
    }
}
impl core::fmt::Debug for FRO192M_CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRO192M_CTRL")
            .field("ENA_12MHZCLK", &self.ENA_12MHZCLK())
            .field("ENA_48MHZCLK", &self.ENA_48MHZCLK())
            .field("DAC_TRIM", &self.DAC_TRIM())
            .field("USBCLKADJ", &self.USBCLKADJ())
            .field("USBMODCHG", &self.USBMODCHG())
            .field("ENA_96MHZCLK", &self.ENA_96MHZCLK())
            .field("WRTRIM", &self.WRTRIM())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FRO192M_CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FRO192M_CTRL {{ ENA_12MHZCLK: {=bool:?}, ENA_48MHZCLK: {=bool:?}, DAC_TRIM: {=u8:?}, USBCLKADJ: {=bool:?}, USBMODCHG: {=bool:?}, ENA_96MHZCLK: {=bool:?}, WRTRIM: {=bool:?} }}",
            self.ENA_12MHZCLK(),
            self.ENA_48MHZCLK(),
            self.DAC_TRIM(),
            self.USBCLKADJ(),
            self.USBMODCHG(),
            self.ENA_96MHZCLK(),
            self.WRTRIM()
        )
    }
}
#[doc = "192MHz Free Running OScillator (FRO) Status register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FRO192M_STATUS(pub u32);
impl FRO192M_STATUS {
    #[doc = "Output clock valid signal. Indicates that CCO clock has settled."]
    #[must_use]
    #[inline(always)]
    pub const fn CLK_VALID(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Output clock valid signal. Indicates that CCO clock has settled."]
    #[inline(always)]
    pub const fn set_CLK_VALID(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "CCO threshold voltage detector output (signal vcco_ok). Once the CCO voltage crosses the threshold voltage of a SLVT transistor, this output signal will go high. It is also possible to observe the clk_valid signal."]
    #[must_use]
    #[inline(always)]
    pub const fn ATB_VCTRL(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "CCO threshold voltage detector output (signal vcco_ok). Once the CCO voltage crosses the threshold voltage of a SLVT transistor, this output signal will go high. It is also possible to observe the clk_valid signal."]
    #[inline(always)]
    pub const fn set_ATB_VCTRL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for FRO192M_STATUS {
    #[inline(always)]
    fn default() -> FRO192M_STATUS {
        FRO192M_STATUS(0)
    }
}
impl core::fmt::Debug for FRO192M_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRO192M_STATUS")
            .field("CLK_VALID", &self.CLK_VALID())
            .field("ATB_VCTRL", &self.ATB_VCTRL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FRO192M_STATUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FRO192M_STATUS {{ CLK_VALID: {=bool:?}, ATB_VCTRL: {=bool:?} }}",
            self.CLK_VALID(),
            self.ATB_VCTRL()
        )
    }
}
#[doc = "High Speed Crystal Oscillator (12 MHz - 32 MHz) Voltage Source Supply Control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LDO_XO32M(pub u32);
impl LDO_XO32M {
    #[doc = "Activate LDO bypass."]
    #[must_use]
    #[inline(always)]
    pub const fn BYPASS(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Activate LDO bypass."]
    #[inline(always)]
    pub const fn set_BYPASS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn HIGHZ(&self) -> super::vals::HIGHZ {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::HIGHZ::from_bits(val as u8)
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_HIGHZ(&mut self, val: super::vals::HIGHZ) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Sets the LDO output level."]
    #[must_use]
    #[inline(always)]
    pub const fn VOUT(&self) -> super::vals::VOUT {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::VOUT::from_bits(val as u8)
    }
    #[doc = "Sets the LDO output level."]
    #[inline(always)]
    pub const fn set_VOUT(&mut self, val: super::vals::VOUT) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
    }
    #[doc = "Adjust the biasing current."]
    #[must_use]
    #[inline(always)]
    pub const fn IBIAS(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Adjust the biasing current."]
    #[inline(always)]
    pub const fn set_IBIAS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Stability configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn STABMODE(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Stability configuration."]
    #[inline(always)]
    pub const fn set_STABMODE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
}
impl Default for LDO_XO32M {
    #[inline(always)]
    fn default() -> LDO_XO32M {
        LDO_XO32M(0)
    }
}
impl core::fmt::Debug for LDO_XO32M {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LDO_XO32M")
            .field("BYPASS", &self.BYPASS())
            .field("HIGHZ", &self.HIGHZ())
            .field("VOUT", &self.VOUT())
            .field("IBIAS", &self.IBIAS())
            .field("STABMODE", &self.STABMODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LDO_XO32M {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LDO_XO32M {{ BYPASS: {=bool:?}, HIGHZ: {:?}, VOUT: {:?}, IBIAS: {=u8:?}, STABMODE: {=u8:?} }}",
            self.BYPASS(),
            self.HIGHZ(),
            self.VOUT(),
            self.IBIAS(),
            self.STABMODE()
        )
    }
}
#[doc = "First Ring Oscillator module control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RINGO0_CTRL(pub u32);
impl RINGO0_CTRL {
    #[doc = "Select short or long ringo (for all ringos types)."]
    #[must_use]
    #[inline(always)]
    pub const fn SL(&self) -> super::vals::SL {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SL::from_bits(val as u8)
    }
    #[doc = "Select short or long ringo (for all ringos types)."]
    #[inline(always)]
    pub const fn set_SL(&mut self, val: super::vals::SL) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Ringo frequency output divider."]
    #[must_use]
    #[inline(always)]
    pub const fn FS(&self) -> super::vals::RINGO0_CTRL_FS {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::RINGO0_CTRL_FS::from_bits(val as u8)
    }
    #[doc = "Ringo frequency output divider."]
    #[inline(always)]
    pub const fn set_FS(&mut self, val: super::vals::RINGO0_CTRL_FS) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "PN-Ringos (P-Transistor and N-Transistor processing) control."]
    #[must_use]
    #[inline(always)]
    pub const fn SWN_SWP(&self) -> super::vals::SWN_SWP {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SWN_SWP::from_bits(val as u8)
    }
    #[doc = "PN-Ringos (P-Transistor and N-Transistor processing) control."]
    #[inline(always)]
    pub const fn set_SWN_SWP(&mut self, val: super::vals::SWN_SWP) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Ringo module Power control."]
    #[must_use]
    #[inline(always)]
    pub const fn PD(&self) -> super::vals::RINGO0_CTRL_PD {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::RINGO0_CTRL_PD::from_bits(val as u8)
    }
    #[doc = "Ringo module Power control."]
    #[inline(always)]
    pub const fn set_PD(&mut self, val: super::vals::RINGO0_CTRL_PD) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "First NAND2-based ringo control."]
    #[must_use]
    #[inline(always)]
    pub const fn E_ND0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "First NAND2-based ringo control."]
    #[inline(always)]
    pub const fn set_E_ND0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Second NAND2-based ringo control."]
    #[must_use]
    #[inline(always)]
    pub const fn E_ND1(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Second NAND2-based ringo control."]
    #[inline(always)]
    pub const fn set_E_ND1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "First NOR2-based ringo control."]
    #[must_use]
    #[inline(always)]
    pub const fn E_NR0(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "First NOR2-based ringo control."]
    #[inline(always)]
    pub const fn set_E_NR0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Second NOR2-based ringo control."]
    #[must_use]
    #[inline(always)]
    pub const fn E_NR1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Second NOR2-based ringo control."]
    #[inline(always)]
    pub const fn set_E_NR1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "First Inverter-based ringo control."]
    #[must_use]
    #[inline(always)]
    pub const fn E_IV0(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "First Inverter-based ringo control."]
    #[inline(always)]
    pub const fn set_E_IV0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Second Inverter-based ringo control."]
    #[must_use]
    #[inline(always)]
    pub const fn E_IV1(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Second Inverter-based ringo control."]
    #[inline(always)]
    pub const fn set_E_IV1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "First PN (P-Transistor and N-Transistor processing) monitor control."]
    #[must_use]
    #[inline(always)]
    pub const fn E_PN0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "First PN (P-Transistor and N-Transistor processing) monitor control."]
    #[inline(always)]
    pub const fn set_E_PN0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Second PN (P-Transistor and N-Transistor processing) monitor control."]
    #[must_use]
    #[inline(always)]
    pub const fn E_PN1(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Second PN (P-Transistor and N-Transistor processing) monitor control."]
    #[inline(always)]
    pub const fn set_E_PN1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Ringo out Clock divider value. Frequency Output = Frequency input / (DIViSOR+1). (minimum = Frequency input / 16)."]
    #[must_use]
    #[inline(always)]
    pub const fn DIVISOR(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Ringo out Clock divider value. Frequency Output = Frequency input / (DIViSOR+1). (minimum = Frequency input / 16)."]
    #[inline(always)]
    pub const fn set_DIVISOR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Ringo clock out Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[must_use]
    #[inline(always)]
    pub const fn DIV_UPDATE_REQ(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Ringo clock out Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub const fn set_DIV_UPDATE_REQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RINGO0_CTRL {
    #[inline(always)]
    fn default() -> RINGO0_CTRL {
        RINGO0_CTRL(0)
    }
}
impl core::fmt::Debug for RINGO0_CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RINGO0_CTRL")
            .field("SL", &self.SL())
            .field("FS", &self.FS())
            .field("SWN_SWP", &self.SWN_SWP())
            .field("PD", &self.PD())
            .field("E_ND0", &self.E_ND0())
            .field("E_ND1", &self.E_ND1())
            .field("E_NR0", &self.E_NR0())
            .field("E_NR1", &self.E_NR1())
            .field("E_IV0", &self.E_IV0())
            .field("E_IV1", &self.E_IV1())
            .field("E_PN0", &self.E_PN0())
            .field("E_PN1", &self.E_PN1())
            .field("DIVISOR", &self.DIVISOR())
            .field("DIV_UPDATE_REQ", &self.DIV_UPDATE_REQ())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RINGO0_CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RINGO0_CTRL {{ SL: {:?}, FS: {:?}, SWN_SWP: {:?}, PD: {:?}, E_ND0: {=bool:?}, E_ND1: {=bool:?}, E_NR0: {=bool:?}, E_NR1: {=bool:?}, E_IV0: {=bool:?}, E_IV1: {=bool:?}, E_PN0: {=bool:?}, E_PN1: {=bool:?}, DIVISOR: {=u8:?}, DIV_UPDATE_REQ: {=bool:?} }}",
            self.SL(),
            self.FS(),
            self.SWN_SWP(),
            self.PD(),
            self.E_ND0(),
            self.E_ND1(),
            self.E_NR0(),
            self.E_NR1(),
            self.E_IV0(),
            self.E_IV1(),
            self.E_PN0(),
            self.E_PN1(),
            self.DIVISOR(),
            self.DIV_UPDATE_REQ()
        )
    }
}
#[doc = "Second Ring Oscillator module control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RINGO1_CTRL(pub u32);
impl RINGO1_CTRL {
    #[doc = "Select short or long ringo (for all ringos types)."]
    #[must_use]
    #[inline(always)]
    pub const fn S(&self) -> super::vals::RINGO1_CTRL_S {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RINGO1_CTRL_S::from_bits(val as u8)
    }
    #[doc = "Select short or long ringo (for all ringos types)."]
    #[inline(always)]
    pub const fn set_S(&mut self, val: super::vals::RINGO1_CTRL_S) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Ringo frequency output divider."]
    #[must_use]
    #[inline(always)]
    pub const fn FS(&self) -> super::vals::RINGO1_CTRL_FS {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::RINGO1_CTRL_FS::from_bits(val as u8)
    }
    #[doc = "Ringo frequency output divider."]
    #[inline(always)]
    pub const fn set_FS(&mut self, val: super::vals::RINGO1_CTRL_FS) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Ringo module Power control."]
    #[must_use]
    #[inline(always)]
    pub const fn PD(&self) -> super::vals::RINGO1_CTRL_PD {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::RINGO1_CTRL_PD::from_bits(val as u8)
    }
    #[doc = "Ringo module Power control."]
    #[inline(always)]
    pub const fn set_PD(&mut self, val: super::vals::RINGO1_CTRL_PD) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn E_R24(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_E_R24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn E_R35(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_E_R35(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Metal 2 (M2) monitor control."]
    #[must_use]
    #[inline(always)]
    pub const fn E_M2(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Metal 2 (M2) monitor control."]
    #[inline(always)]
    pub const fn set_E_M2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Metal 3 (M3) monitor control."]
    #[must_use]
    #[inline(always)]
    pub const fn E_M3(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Metal 3 (M3) monitor control."]
    #[inline(always)]
    pub const fn set_E_M3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Metal 4 (M4) monitor control."]
    #[must_use]
    #[inline(always)]
    pub const fn E_M4(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Metal 4 (M4) monitor control."]
    #[inline(always)]
    pub const fn set_E_M4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Metal 5 (M5) monitor control."]
    #[must_use]
    #[inline(always)]
    pub const fn E_M5(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Metal 5 (M5) monitor control."]
    #[inline(always)]
    pub const fn set_E_M5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Ringo out Clock divider value. Frequency Output = Frequency input / (DIViSOR+1). (minimum = Frequency input / 16)."]
    #[must_use]
    #[inline(always)]
    pub const fn DIVISOR(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Ringo out Clock divider value. Frequency Output = Frequency input / (DIViSOR+1). (minimum = Frequency input / 16)."]
    #[inline(always)]
    pub const fn set_DIVISOR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Ringo clock out Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[must_use]
    #[inline(always)]
    pub const fn DIV_UPDATE_REQ(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Ringo clock out Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub const fn set_DIV_UPDATE_REQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RINGO1_CTRL {
    #[inline(always)]
    fn default() -> RINGO1_CTRL {
        RINGO1_CTRL(0)
    }
}
impl core::fmt::Debug for RINGO1_CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RINGO1_CTRL")
            .field("S", &self.S())
            .field("FS", &self.FS())
            .field("PD", &self.PD())
            .field("E_R24", &self.E_R24())
            .field("E_R35", &self.E_R35())
            .field("E_M2", &self.E_M2())
            .field("E_M3", &self.E_M3())
            .field("E_M4", &self.E_M4())
            .field("E_M5", &self.E_M5())
            .field("DIVISOR", &self.DIVISOR())
            .field("DIV_UPDATE_REQ", &self.DIV_UPDATE_REQ())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RINGO1_CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RINGO1_CTRL {{ S: {:?}, FS: {:?}, PD: {:?}, E_R24: {=bool:?}, E_R35: {=bool:?}, E_M2: {=bool:?}, E_M3: {=bool:?}, E_M4: {=bool:?}, E_M5: {=bool:?}, DIVISOR: {=u8:?}, DIV_UPDATE_REQ: {=bool:?} }}",
            self.S(),
            self.FS(),
            self.PD(),
            self.E_R24(),
            self.E_R35(),
            self.E_M2(),
            self.E_M3(),
            self.E_M4(),
            self.E_M5(),
            self.DIVISOR(),
            self.DIV_UPDATE_REQ()
        )
    }
}
#[doc = "Third Ring Oscillator module control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RINGO2_CTRL(pub u32);
impl RINGO2_CTRL {
    #[doc = "Select short or long ringo (for all ringos types)."]
    #[must_use]
    #[inline(always)]
    pub const fn S(&self) -> super::vals::RINGO2_CTRL_S {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RINGO2_CTRL_S::from_bits(val as u8)
    }
    #[doc = "Select short or long ringo (for all ringos types)."]
    #[inline(always)]
    pub const fn set_S(&mut self, val: super::vals::RINGO2_CTRL_S) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Ringo frequency output divider."]
    #[must_use]
    #[inline(always)]
    pub const fn FS(&self) -> super::vals::RINGO2_CTRL_FS {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::RINGO2_CTRL_FS::from_bits(val as u8)
    }
    #[doc = "Ringo frequency output divider."]
    #[inline(always)]
    pub const fn set_FS(&mut self, val: super::vals::RINGO2_CTRL_FS) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Ringo module Power control."]
    #[must_use]
    #[inline(always)]
    pub const fn PD(&self) -> super::vals::RINGO2_CTRL_PD {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::RINGO2_CTRL_PD::from_bits(val as u8)
    }
    #[doc = "Ringo module Power control."]
    #[inline(always)]
    pub const fn set_PD(&mut self, val: super::vals::RINGO2_CTRL_PD) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn E_R24(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_E_R24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn E_R35(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_E_R35(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Metal 2 (M2) monitor control."]
    #[must_use]
    #[inline(always)]
    pub const fn E_M2(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Metal 2 (M2) monitor control."]
    #[inline(always)]
    pub const fn set_E_M2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Metal 3 (M3) monitor control."]
    #[must_use]
    #[inline(always)]
    pub const fn E_M3(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Metal 3 (M3) monitor control."]
    #[inline(always)]
    pub const fn set_E_M3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Metal 4 (M4) monitor control."]
    #[must_use]
    #[inline(always)]
    pub const fn E_M4(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Metal 4 (M4) monitor control."]
    #[inline(always)]
    pub const fn set_E_M4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Metal 5 (M5) monitor control."]
    #[must_use]
    #[inline(always)]
    pub const fn E_M5(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Metal 5 (M5) monitor control."]
    #[inline(always)]
    pub const fn set_E_M5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Ringo out Clock divider value. Frequency Output = Frequency input / (DIViSOR+1). (minimum = Frequency input / 16)."]
    #[must_use]
    #[inline(always)]
    pub const fn DIVISOR(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Ringo out Clock divider value. Frequency Output = Frequency input / (DIViSOR+1). (minimum = Frequency input / 16)."]
    #[inline(always)]
    pub const fn set_DIVISOR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Ringo clock out Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[must_use]
    #[inline(always)]
    pub const fn DIV_UPDATE_REQ(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Ringo clock out Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub const fn set_DIV_UPDATE_REQ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RINGO2_CTRL {
    #[inline(always)]
    fn default() -> RINGO2_CTRL {
        RINGO2_CTRL(0)
    }
}
impl core::fmt::Debug for RINGO2_CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RINGO2_CTRL")
            .field("S", &self.S())
            .field("FS", &self.FS())
            .field("PD", &self.PD())
            .field("E_R24", &self.E_R24())
            .field("E_R35", &self.E_R35())
            .field("E_M2", &self.E_M2())
            .field("E_M3", &self.E_M3())
            .field("E_M4", &self.E_M4())
            .field("E_M5", &self.E_M5())
            .field("DIVISOR", &self.DIVISOR())
            .field("DIV_UPDATE_REQ", &self.DIV_UPDATE_REQ())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RINGO2_CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RINGO2_CTRL {{ S: {:?}, FS: {:?}, PD: {:?}, E_R24: {=bool:?}, E_R35: {=bool:?}, E_M2: {=bool:?}, E_M3: {=bool:?}, E_M4: {=bool:?}, E_M5: {=bool:?}, DIVISOR: {=u8:?}, DIV_UPDATE_REQ: {=bool:?} }}",
            self.S(),
            self.FS(),
            self.PD(),
            self.E_R24(),
            self.E_R35(),
            self.E_M2(),
            self.E_M3(),
            self.E_M4(),
            self.E_M5(),
            self.DIVISOR(),
            self.DIV_UPDATE_REQ()
        )
    }
}
#[doc = "USB High Speed Phy Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USBHS_PHY_CTRL(pub u32);
impl USBHS_PHY_CTRL {
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
impl Default for USBHS_PHY_CTRL {
    #[inline(always)]
    fn default() -> USBHS_PHY_CTRL {
        USBHS_PHY_CTRL(0)
    }
}
impl core::fmt::Debug for USBHS_PHY_CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USBHS_PHY_CTRL")
            .field("usb_vbusvalid_ext", &self.usb_vbusvalid_ext())
            .field("usb_id_ext", &self.usb_id_ext())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USBHS_PHY_CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USBHS_PHY_CTRL {{ usb_vbusvalid_ext: {=bool:?}, usb_id_ext: {=bool:?} }}",
            self.usb_vbusvalid_ext(),
            self.usb_id_ext()
        )
    }
}
#[doc = "USB High Speed Phy Trim values."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USBHS_PHY_TRIM(pub u32);
impl USBHS_PHY_TRIM {
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
impl Default for USBHS_PHY_TRIM {
    #[inline(always)]
    fn default() -> USBHS_PHY_TRIM {
        USBHS_PHY_TRIM(0)
    }
}
impl core::fmt::Debug for USBHS_PHY_TRIM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USBHS_PHY_TRIM")
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
impl defmt::Format for USBHS_PHY_TRIM {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USBHS_PHY_TRIM {{ trim_usb_reg_env_tail_adj_vd: {=u8:?}, trim_usbphy_tx_d_cal: {=u8:?}, trim_usbphy_tx_cal45dp: {=u8:?}, trim_usbphy_tx_cal45dm: {=u8:?}, trim_usb2_refbias_tst: {=u8:?}, trim_usb2_refbias_vbgadj: {=u8:?}, trim_pll_ctrl0_div_sel: {=u8:?} }}",
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
#[doc = "High speed Crystal Oscillator Control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XO32M_CTRL(pub u32);
impl XO32M_CTRL {
    #[doc = "Xo in slave mode."]
    #[must_use]
    #[inline(always)]
    pub const fn SLAVE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Xo in slave mode."]
    #[inline(always)]
    pub const fn set_SLAVE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Tune capa banks of High speed Crystal Oscillator input pin."]
    #[must_use]
    #[inline(always)]
    pub const fn OSC_CAP_IN(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Tune capa banks of High speed Crystal Oscillator input pin."]
    #[inline(always)]
    pub const fn set_OSC_CAP_IN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Tune capa banks of High speed Crystal Oscillator output pin."]
    #[must_use]
    #[inline(always)]
    pub const fn OSC_CAP_OUT(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x7f;
        val as u8
    }
    #[doc = "Tune capa banks of High speed Crystal Oscillator output pin."]
    #[inline(always)]
    pub const fn set_OSC_CAP_OUT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 15usize)) | (((val as u32) & 0x7f) << 15usize);
    }
    #[doc = "Bypass enable of XO AC buffer enable in pll and top level."]
    #[must_use]
    #[inline(always)]
    pub const fn ACBUF_PASS_ENABLE(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass enable of XO AC buffer enable in pll and top level."]
    #[inline(always)]
    pub const fn set_ACBUF_PASS_ENABLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Enable High speed Crystal oscillator output to USB HS PLL."]
    #[must_use]
    #[inline(always)]
    pub const fn ENABLE_PLL_USB_OUT(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enable High speed Crystal oscillator output to USB HS PLL."]
    #[inline(always)]
    pub const fn set_ENABLE_PLL_USB_OUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Enable High speed Crystal oscillator output to CPU system."]
    #[must_use]
    #[inline(always)]
    pub const fn ENABLE_SYSTEM_CLK_OUT(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Enable High speed Crystal oscillator output to CPU system."]
    #[inline(always)]
    pub const fn set_ENABLE_SYSTEM_CLK_OUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for XO32M_CTRL {
    #[inline(always)]
    fn default() -> XO32M_CTRL {
        XO32M_CTRL(0)
    }
}
impl core::fmt::Debug for XO32M_CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XO32M_CTRL")
            .field("SLAVE", &self.SLAVE())
            .field("OSC_CAP_IN", &self.OSC_CAP_IN())
            .field("OSC_CAP_OUT", &self.OSC_CAP_OUT())
            .field("ACBUF_PASS_ENABLE", &self.ACBUF_PASS_ENABLE())
            .field("ENABLE_PLL_USB_OUT", &self.ENABLE_PLL_USB_OUT())
            .field("ENABLE_SYSTEM_CLK_OUT", &self.ENABLE_SYSTEM_CLK_OUT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for XO32M_CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "XO32M_CTRL {{ SLAVE: {=bool:?}, OSC_CAP_IN: {=u8:?}, OSC_CAP_OUT: {=u8:?}, ACBUF_PASS_ENABLE: {=bool:?}, ENABLE_PLL_USB_OUT: {=bool:?}, ENABLE_SYSTEM_CLK_OUT: {=bool:?} }}",
            self.SLAVE(),
            self.OSC_CAP_IN(),
            self.OSC_CAP_OUT(),
            self.ACBUF_PASS_ENABLE(),
            self.ENABLE_PLL_USB_OUT(),
            self.ENABLE_SYSTEM_CLK_OUT()
        )
    }
}
#[doc = "High speed Crystal Oscillator Status register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XO32M_STATUS(pub u32);
impl XO32M_STATUS {
    #[doc = "Indicates XO out frequency statibilty."]
    #[must_use]
    #[inline(always)]
    pub const fn XO_READY(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates XO out frequency statibilty."]
    #[inline(always)]
    pub const fn set_XO_READY(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for XO32M_STATUS {
    #[inline(always)]
    fn default() -> XO32M_STATUS {
        XO32M_STATUS(0)
    }
}
impl core::fmt::Debug for XO32M_STATUS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XO32M_STATUS")
            .field("XO_READY", &self.XO_READY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for XO32M_STATUS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "XO32M_STATUS {{ XO_READY: {=bool:?} }}", self.XO_READY())
    }
}
