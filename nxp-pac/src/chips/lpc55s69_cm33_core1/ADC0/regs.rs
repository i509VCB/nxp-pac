#[doc = "Calibration General A-Side Registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAL_GAR(pub u32);
impl CAL_GAR {
    #[doc = "Calibration General A Side Register Element."]
    #[must_use]
    #[inline(always)]
    pub const fn CAL_GAR_VAL(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Calibration General A Side Register Element."]
    #[inline(always)]
    pub const fn set_CAL_GAR_VAL(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for CAL_GAR {
    #[inline(always)]
    fn default() -> CAL_GAR {
        CAL_GAR(0)
    }
}
impl core::fmt::Debug for CAL_GAR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAL_GAR")
            .field("CAL_GAR_VAL", &self.CAL_GAR_VAL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAL_GAR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CAL_GAR {{ CAL_GAR_VAL: {=u16:?} }}", self.CAL_GAR_VAL())
    }
}
#[doc = "Calibration General B-Side Registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAL_GBR(pub u32);
impl CAL_GBR {
    #[doc = "Calibration General B Side Register Element."]
    #[must_use]
    #[inline(always)]
    pub const fn CAL_GBR_VAL(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Calibration General B Side Register Element."]
    #[inline(always)]
    pub const fn set_CAL_GBR_VAL(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for CAL_GBR {
    #[inline(always)]
    fn default() -> CAL_GBR {
        CAL_GBR(0)
    }
}
impl core::fmt::Debug for CAL_GBR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAL_GBR")
            .field("CAL_GBR_VAL", &self.CAL_GBR_VAL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAL_GBR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CAL_GBR {{ CAL_GBR_VAL: {=u16:?} }}", self.CAL_GBR_VAL())
    }
}
#[doc = "ADC Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CFG(pub u32);
impl CFG {
    #[doc = "ADC trigger priority control."]
    #[must_use]
    #[inline(always)]
    pub const fn TPRICTRL(&self) -> super::vals::TPRICTRL {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::TPRICTRL::from_bits(val as u8)
    }
    #[doc = "ADC trigger priority control."]
    #[inline(always)]
    pub const fn set_TPRICTRL(&mut self, val: super::vals::TPRICTRL) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Power Configuration Select."]
    #[must_use]
    #[inline(always)]
    pub const fn PWRSEL(&self) -> super::vals::PWRSEL {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PWRSEL::from_bits(val as u8)
    }
    #[doc = "Power Configuration Select."]
    #[inline(always)]
    pub const fn set_PWRSEL(&mut self, val: super::vals::PWRSEL) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Voltage Reference Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn REFSEL(&self) -> super::vals::REFSEL {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::REFSEL::from_bits(val as u8)
    }
    #[doc = "Voltage Reference Selection."]
    #[inline(always)]
    pub const fn set_REFSEL(&mut self, val: super::vals::REFSEL) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Trigger Resume Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn TRES(&self) -> super::vals::TRES {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::TRES::from_bits(val as u8)
    }
    #[doc = "Trigger Resume Enable."]
    #[inline(always)]
    pub const fn set_TRES(&mut self, val: super::vals::TRES) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Trigger Command Resume."]
    #[must_use]
    #[inline(always)]
    pub const fn TCMDRES(&self) -> super::vals::TCMDRES {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::TCMDRES::from_bits(val as u8)
    }
    #[doc = "Trigger Command Resume."]
    #[inline(always)]
    pub const fn set_TCMDRES(&mut self, val: super::vals::TCMDRES) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "High Priority Trigger Exception Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn HPT_EXDI(&self) -> super::vals::HPT_EXDI {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::HPT_EXDI::from_bits(val as u8)
    }
    #[doc = "High Priority Trigger Exception Disable."]
    #[inline(always)]
    pub const fn set_HPT_EXDI(&mut self, val: super::vals::HPT_EXDI) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Power Up Delay."]
    #[must_use]
    #[inline(always)]
    pub const fn PUDLY(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Power Up Delay."]
    #[inline(always)]
    pub const fn set_PUDLY(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "ADC Analog Pre-Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn PWREN(&self) -> super::vals::PWREN {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::PWREN::from_bits(val as u8)
    }
    #[doc = "ADC Analog Pre-Enable."]
    #[inline(always)]
    pub const fn set_PWREN(&mut self, val: super::vals::PWREN) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
}
impl Default for CFG {
    #[inline(always)]
    fn default() -> CFG {
        CFG(0)
    }
}
impl core::fmt::Debug for CFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG")
            .field("TPRICTRL", &self.TPRICTRL())
            .field("PWRSEL", &self.PWRSEL())
            .field("REFSEL", &self.REFSEL())
            .field("TRES", &self.TRES())
            .field("TCMDRES", &self.TCMDRES())
            .field("HPT_EXDI", &self.HPT_EXDI())
            .field("PUDLY", &self.PUDLY())
            .field("PWREN", &self.PWREN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CFG {{ TPRICTRL: {:?}, PWRSEL: {:?}, REFSEL: {:?}, TRES: {:?}, TCMDRES: {:?}, HPT_EXDI: {:?}, PUDLY: {=u8:?}, PWREN: {:?} }}",
            self.TPRICTRL(),
            self.PWRSEL(),
            self.REFSEL(),
            self.TRES(),
            self.TCMDRES(),
            self.HPT_EXDI(),
            self.PUDLY(),
            self.PWREN()
        )
    }
}
#[doc = "ADC Command High Buffer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CMDH1(pub u32);
impl CMDH1 {
    #[doc = "Compare Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn CMPEN(&self) -> super::vals::CMDH1_CMPEN {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::CMDH1_CMPEN::from_bits(val as u8)
    }
    #[doc = "Compare Function Enable."]
    #[inline(always)]
    pub const fn set_CMPEN(&mut self, val: super::vals::CMDH1_CMPEN) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Wait for trigger assertion before execution."]
    #[must_use]
    #[inline(always)]
    pub const fn WAIT_TRIG(&self) -> super::vals::CMDH1_WAIT_TRIG {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::CMDH1_WAIT_TRIG::from_bits(val as u8)
    }
    #[doc = "Wait for trigger assertion before execution."]
    #[inline(always)]
    pub const fn set_WAIT_TRIG(&mut self, val: super::vals::CMDH1_WAIT_TRIG) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Loop with Increment."]
    #[must_use]
    #[inline(always)]
    pub const fn LWI(&self) -> super::vals::CMDH1_LWI {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CMDH1_LWI::from_bits(val as u8)
    }
    #[doc = "Loop with Increment."]
    #[inline(always)]
    pub const fn set_LWI(&mut self, val: super::vals::CMDH1_LWI) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select."]
    #[must_use]
    #[inline(always)]
    pub const fn STS(&self) -> super::vals::CMDH1_STS {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::CMDH1_STS::from_bits(val as u8)
    }
    #[doc = "Sample Time Select."]
    #[inline(always)]
    pub const fn set_STS(&mut self, val: super::vals::CMDH1_STS) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select."]
    #[must_use]
    #[inline(always)]
    pub const fn AVGS(&self) -> super::vals::CMDH1_AVGS {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::CMDH1_AVGS::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select."]
    #[inline(always)]
    pub const fn set_AVGS(&mut self, val: super::vals::CMDH1_AVGS) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Loop Count Select."]
    #[must_use]
    #[inline(always)]
    pub const fn LOOP(&self) -> super::vals::CMDH1_LOOP {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::CMDH1_LOOP::from_bits(val as u8)
    }
    #[doc = "Loop Count Select."]
    #[inline(always)]
    pub const fn set_LOOP(&mut self, val: super::vals::CMDH1_LOOP) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select."]
    #[must_use]
    #[inline(always)]
    pub const fn NEXT(&self) -> super::vals::CMDH1_NEXT {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::CMDH1_NEXT::from_bits(val as u8)
    }
    #[doc = "Next Command Select."]
    #[inline(always)]
    pub const fn set_NEXT(&mut self, val: super::vals::CMDH1_NEXT) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for CMDH1 {
    #[inline(always)]
    fn default() -> CMDH1 {
        CMDH1(0)
    }
}
impl core::fmt::Debug for CMDH1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDH1")
            .field("CMPEN", &self.CMPEN())
            .field("WAIT_TRIG", &self.WAIT_TRIG())
            .field("LWI", &self.LWI())
            .field("STS", &self.STS())
            .field("AVGS", &self.AVGS())
            .field("LOOP", &self.LOOP())
            .field("NEXT", &self.NEXT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CMDH1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CMDH1 {{ CMPEN: {:?}, WAIT_TRIG: {:?}, LWI: {:?}, STS: {:?}, AVGS: {:?}, LOOP: {:?}, NEXT: {:?} }}",
            self.CMPEN(),
            self.WAIT_TRIG(),
            self.LWI(),
            self.STS(),
            self.AVGS(),
            self.LOOP(),
            self.NEXT()
        )
    }
}
#[doc = "ADC Command High Buffer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CMDH10(pub u32);
impl CMDH10 {
    #[doc = "Wait for trigger assertion before execution."]
    #[must_use]
    #[inline(always)]
    pub const fn WAIT_TRIG(&self) -> super::vals::CMDH10_WAIT_TRIG {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::CMDH10_WAIT_TRIG::from_bits(val as u8)
    }
    #[doc = "Wait for trigger assertion before execution."]
    #[inline(always)]
    pub const fn set_WAIT_TRIG(&mut self, val: super::vals::CMDH10_WAIT_TRIG) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Loop with Increment."]
    #[must_use]
    #[inline(always)]
    pub const fn LWI(&self) -> super::vals::CMDH10_LWI {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CMDH10_LWI::from_bits(val as u8)
    }
    #[doc = "Loop with Increment."]
    #[inline(always)]
    pub const fn set_LWI(&mut self, val: super::vals::CMDH10_LWI) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select."]
    #[must_use]
    #[inline(always)]
    pub const fn STS(&self) -> super::vals::CMDH10_STS {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::CMDH10_STS::from_bits(val as u8)
    }
    #[doc = "Sample Time Select."]
    #[inline(always)]
    pub const fn set_STS(&mut self, val: super::vals::CMDH10_STS) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select."]
    #[must_use]
    #[inline(always)]
    pub const fn AVGS(&self) -> super::vals::CMDH10_AVGS {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::CMDH10_AVGS::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select."]
    #[inline(always)]
    pub const fn set_AVGS(&mut self, val: super::vals::CMDH10_AVGS) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Loop Count Select."]
    #[must_use]
    #[inline(always)]
    pub const fn LOOP(&self) -> super::vals::CMDH10_LOOP {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::CMDH10_LOOP::from_bits(val as u8)
    }
    #[doc = "Loop Count Select."]
    #[inline(always)]
    pub const fn set_LOOP(&mut self, val: super::vals::CMDH10_LOOP) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select."]
    #[must_use]
    #[inline(always)]
    pub const fn NEXT(&self) -> super::vals::CMDH10_NEXT {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::CMDH10_NEXT::from_bits(val as u8)
    }
    #[doc = "Next Command Select."]
    #[inline(always)]
    pub const fn set_NEXT(&mut self, val: super::vals::CMDH10_NEXT) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for CMDH10 {
    #[inline(always)]
    fn default() -> CMDH10 {
        CMDH10(0)
    }
}
impl core::fmt::Debug for CMDH10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDH10")
            .field("WAIT_TRIG", &self.WAIT_TRIG())
            .field("LWI", &self.LWI())
            .field("STS", &self.STS())
            .field("AVGS", &self.AVGS())
            .field("LOOP", &self.LOOP())
            .field("NEXT", &self.NEXT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CMDH10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CMDH10 {{ WAIT_TRIG: {:?}, LWI: {:?}, STS: {:?}, AVGS: {:?}, LOOP: {:?}, NEXT: {:?} }}",
            self.WAIT_TRIG(),
            self.LWI(),
            self.STS(),
            self.AVGS(),
            self.LOOP(),
            self.NEXT()
        )
    }
}
#[doc = "ADC Command High Buffer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CMDH11(pub u32);
impl CMDH11 {
    #[doc = "Wait for trigger assertion before execution."]
    #[must_use]
    #[inline(always)]
    pub const fn WAIT_TRIG(&self) -> super::vals::CMDH11_WAIT_TRIG {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::CMDH11_WAIT_TRIG::from_bits(val as u8)
    }
    #[doc = "Wait for trigger assertion before execution."]
    #[inline(always)]
    pub const fn set_WAIT_TRIG(&mut self, val: super::vals::CMDH11_WAIT_TRIG) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Loop with Increment."]
    #[must_use]
    #[inline(always)]
    pub const fn LWI(&self) -> super::vals::CMDH11_LWI {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CMDH11_LWI::from_bits(val as u8)
    }
    #[doc = "Loop with Increment."]
    #[inline(always)]
    pub const fn set_LWI(&mut self, val: super::vals::CMDH11_LWI) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select."]
    #[must_use]
    #[inline(always)]
    pub const fn STS(&self) -> super::vals::CMDH11_STS {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::CMDH11_STS::from_bits(val as u8)
    }
    #[doc = "Sample Time Select."]
    #[inline(always)]
    pub const fn set_STS(&mut self, val: super::vals::CMDH11_STS) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select."]
    #[must_use]
    #[inline(always)]
    pub const fn AVGS(&self) -> super::vals::CMDH11_AVGS {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::CMDH11_AVGS::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select."]
    #[inline(always)]
    pub const fn set_AVGS(&mut self, val: super::vals::CMDH11_AVGS) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Loop Count Select."]
    #[must_use]
    #[inline(always)]
    pub const fn LOOP(&self) -> super::vals::CMDH11_LOOP {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::CMDH11_LOOP::from_bits(val as u8)
    }
    #[doc = "Loop Count Select."]
    #[inline(always)]
    pub const fn set_LOOP(&mut self, val: super::vals::CMDH11_LOOP) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select."]
    #[must_use]
    #[inline(always)]
    pub const fn NEXT(&self) -> super::vals::CMDH11_NEXT {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::CMDH11_NEXT::from_bits(val as u8)
    }
    #[doc = "Next Command Select."]
    #[inline(always)]
    pub const fn set_NEXT(&mut self, val: super::vals::CMDH11_NEXT) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for CMDH11 {
    #[inline(always)]
    fn default() -> CMDH11 {
        CMDH11(0)
    }
}
impl core::fmt::Debug for CMDH11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDH11")
            .field("WAIT_TRIG", &self.WAIT_TRIG())
            .field("LWI", &self.LWI())
            .field("STS", &self.STS())
            .field("AVGS", &self.AVGS())
            .field("LOOP", &self.LOOP())
            .field("NEXT", &self.NEXT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CMDH11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CMDH11 {{ WAIT_TRIG: {:?}, LWI: {:?}, STS: {:?}, AVGS: {:?}, LOOP: {:?}, NEXT: {:?} }}",
            self.WAIT_TRIG(),
            self.LWI(),
            self.STS(),
            self.AVGS(),
            self.LOOP(),
            self.NEXT()
        )
    }
}
#[doc = "ADC Command High Buffer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CMDH12(pub u32);
impl CMDH12 {
    #[doc = "Wait for trigger assertion before execution."]
    #[must_use]
    #[inline(always)]
    pub const fn WAIT_TRIG(&self) -> super::vals::CMDH12_WAIT_TRIG {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::CMDH12_WAIT_TRIG::from_bits(val as u8)
    }
    #[doc = "Wait for trigger assertion before execution."]
    #[inline(always)]
    pub const fn set_WAIT_TRIG(&mut self, val: super::vals::CMDH12_WAIT_TRIG) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Loop with Increment."]
    #[must_use]
    #[inline(always)]
    pub const fn LWI(&self) -> super::vals::CMDH12_LWI {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CMDH12_LWI::from_bits(val as u8)
    }
    #[doc = "Loop with Increment."]
    #[inline(always)]
    pub const fn set_LWI(&mut self, val: super::vals::CMDH12_LWI) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select."]
    #[must_use]
    #[inline(always)]
    pub const fn STS(&self) -> super::vals::CMDH12_STS {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::CMDH12_STS::from_bits(val as u8)
    }
    #[doc = "Sample Time Select."]
    #[inline(always)]
    pub const fn set_STS(&mut self, val: super::vals::CMDH12_STS) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select."]
    #[must_use]
    #[inline(always)]
    pub const fn AVGS(&self) -> super::vals::CMDH12_AVGS {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::CMDH12_AVGS::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select."]
    #[inline(always)]
    pub const fn set_AVGS(&mut self, val: super::vals::CMDH12_AVGS) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Loop Count Select."]
    #[must_use]
    #[inline(always)]
    pub const fn LOOP(&self) -> super::vals::CMDH12_LOOP {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::CMDH12_LOOP::from_bits(val as u8)
    }
    #[doc = "Loop Count Select."]
    #[inline(always)]
    pub const fn set_LOOP(&mut self, val: super::vals::CMDH12_LOOP) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select."]
    #[must_use]
    #[inline(always)]
    pub const fn NEXT(&self) -> super::vals::CMDH12_NEXT {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::CMDH12_NEXT::from_bits(val as u8)
    }
    #[doc = "Next Command Select."]
    #[inline(always)]
    pub const fn set_NEXT(&mut self, val: super::vals::CMDH12_NEXT) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for CMDH12 {
    #[inline(always)]
    fn default() -> CMDH12 {
        CMDH12(0)
    }
}
impl core::fmt::Debug for CMDH12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDH12")
            .field("WAIT_TRIG", &self.WAIT_TRIG())
            .field("LWI", &self.LWI())
            .field("STS", &self.STS())
            .field("AVGS", &self.AVGS())
            .field("LOOP", &self.LOOP())
            .field("NEXT", &self.NEXT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CMDH12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CMDH12 {{ WAIT_TRIG: {:?}, LWI: {:?}, STS: {:?}, AVGS: {:?}, LOOP: {:?}, NEXT: {:?} }}",
            self.WAIT_TRIG(),
            self.LWI(),
            self.STS(),
            self.AVGS(),
            self.LOOP(),
            self.NEXT()
        )
    }
}
#[doc = "ADC Command High Buffer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CMDH13(pub u32);
impl CMDH13 {
    #[doc = "Wait for trigger assertion before execution."]
    #[must_use]
    #[inline(always)]
    pub const fn WAIT_TRIG(&self) -> super::vals::CMDH13_WAIT_TRIG {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::CMDH13_WAIT_TRIG::from_bits(val as u8)
    }
    #[doc = "Wait for trigger assertion before execution."]
    #[inline(always)]
    pub const fn set_WAIT_TRIG(&mut self, val: super::vals::CMDH13_WAIT_TRIG) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Loop with Increment."]
    #[must_use]
    #[inline(always)]
    pub const fn LWI(&self) -> super::vals::CMDH13_LWI {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CMDH13_LWI::from_bits(val as u8)
    }
    #[doc = "Loop with Increment."]
    #[inline(always)]
    pub const fn set_LWI(&mut self, val: super::vals::CMDH13_LWI) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select."]
    #[must_use]
    #[inline(always)]
    pub const fn STS(&self) -> super::vals::CMDH13_STS {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::CMDH13_STS::from_bits(val as u8)
    }
    #[doc = "Sample Time Select."]
    #[inline(always)]
    pub const fn set_STS(&mut self, val: super::vals::CMDH13_STS) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select."]
    #[must_use]
    #[inline(always)]
    pub const fn AVGS(&self) -> super::vals::CMDH13_AVGS {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::CMDH13_AVGS::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select."]
    #[inline(always)]
    pub const fn set_AVGS(&mut self, val: super::vals::CMDH13_AVGS) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Loop Count Select."]
    #[must_use]
    #[inline(always)]
    pub const fn LOOP(&self) -> super::vals::CMDH13_LOOP {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::CMDH13_LOOP::from_bits(val as u8)
    }
    #[doc = "Loop Count Select."]
    #[inline(always)]
    pub const fn set_LOOP(&mut self, val: super::vals::CMDH13_LOOP) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select."]
    #[must_use]
    #[inline(always)]
    pub const fn NEXT(&self) -> super::vals::CMDH13_NEXT {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::CMDH13_NEXT::from_bits(val as u8)
    }
    #[doc = "Next Command Select."]
    #[inline(always)]
    pub const fn set_NEXT(&mut self, val: super::vals::CMDH13_NEXT) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for CMDH13 {
    #[inline(always)]
    fn default() -> CMDH13 {
        CMDH13(0)
    }
}
impl core::fmt::Debug for CMDH13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDH13")
            .field("WAIT_TRIG", &self.WAIT_TRIG())
            .field("LWI", &self.LWI())
            .field("STS", &self.STS())
            .field("AVGS", &self.AVGS())
            .field("LOOP", &self.LOOP())
            .field("NEXT", &self.NEXT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CMDH13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CMDH13 {{ WAIT_TRIG: {:?}, LWI: {:?}, STS: {:?}, AVGS: {:?}, LOOP: {:?}, NEXT: {:?} }}",
            self.WAIT_TRIG(),
            self.LWI(),
            self.STS(),
            self.AVGS(),
            self.LOOP(),
            self.NEXT()
        )
    }
}
#[doc = "ADC Command High Buffer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CMDH14(pub u32);
impl CMDH14 {
    #[doc = "Wait for trigger assertion before execution."]
    #[must_use]
    #[inline(always)]
    pub const fn WAIT_TRIG(&self) -> super::vals::CMDH14_WAIT_TRIG {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::CMDH14_WAIT_TRIG::from_bits(val as u8)
    }
    #[doc = "Wait for trigger assertion before execution."]
    #[inline(always)]
    pub const fn set_WAIT_TRIG(&mut self, val: super::vals::CMDH14_WAIT_TRIG) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Loop with Increment."]
    #[must_use]
    #[inline(always)]
    pub const fn LWI(&self) -> super::vals::CMDH14_LWI {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CMDH14_LWI::from_bits(val as u8)
    }
    #[doc = "Loop with Increment."]
    #[inline(always)]
    pub const fn set_LWI(&mut self, val: super::vals::CMDH14_LWI) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select."]
    #[must_use]
    #[inline(always)]
    pub const fn STS(&self) -> super::vals::CMDH14_STS {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::CMDH14_STS::from_bits(val as u8)
    }
    #[doc = "Sample Time Select."]
    #[inline(always)]
    pub const fn set_STS(&mut self, val: super::vals::CMDH14_STS) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select."]
    #[must_use]
    #[inline(always)]
    pub const fn AVGS(&self) -> super::vals::CMDH14_AVGS {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::CMDH14_AVGS::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select."]
    #[inline(always)]
    pub const fn set_AVGS(&mut self, val: super::vals::CMDH14_AVGS) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Loop Count Select."]
    #[must_use]
    #[inline(always)]
    pub const fn LOOP(&self) -> super::vals::CMDH14_LOOP {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::CMDH14_LOOP::from_bits(val as u8)
    }
    #[doc = "Loop Count Select."]
    #[inline(always)]
    pub const fn set_LOOP(&mut self, val: super::vals::CMDH14_LOOP) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select."]
    #[must_use]
    #[inline(always)]
    pub const fn NEXT(&self) -> super::vals::CMDH14_NEXT {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::CMDH14_NEXT::from_bits(val as u8)
    }
    #[doc = "Next Command Select."]
    #[inline(always)]
    pub const fn set_NEXT(&mut self, val: super::vals::CMDH14_NEXT) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for CMDH14 {
    #[inline(always)]
    fn default() -> CMDH14 {
        CMDH14(0)
    }
}
impl core::fmt::Debug for CMDH14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDH14")
            .field("WAIT_TRIG", &self.WAIT_TRIG())
            .field("LWI", &self.LWI())
            .field("STS", &self.STS())
            .field("AVGS", &self.AVGS())
            .field("LOOP", &self.LOOP())
            .field("NEXT", &self.NEXT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CMDH14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CMDH14 {{ WAIT_TRIG: {:?}, LWI: {:?}, STS: {:?}, AVGS: {:?}, LOOP: {:?}, NEXT: {:?} }}",
            self.WAIT_TRIG(),
            self.LWI(),
            self.STS(),
            self.AVGS(),
            self.LOOP(),
            self.NEXT()
        )
    }
}
#[doc = "ADC Command High Buffer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CMDH15(pub u32);
impl CMDH15 {
    #[doc = "Wait for trigger assertion before execution."]
    #[must_use]
    #[inline(always)]
    pub const fn WAIT_TRIG(&self) -> super::vals::CMDH15_WAIT_TRIG {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::CMDH15_WAIT_TRIG::from_bits(val as u8)
    }
    #[doc = "Wait for trigger assertion before execution."]
    #[inline(always)]
    pub const fn set_WAIT_TRIG(&mut self, val: super::vals::CMDH15_WAIT_TRIG) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Loop with Increment."]
    #[must_use]
    #[inline(always)]
    pub const fn LWI(&self) -> super::vals::CMDH15_LWI {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CMDH15_LWI::from_bits(val as u8)
    }
    #[doc = "Loop with Increment."]
    #[inline(always)]
    pub const fn set_LWI(&mut self, val: super::vals::CMDH15_LWI) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select."]
    #[must_use]
    #[inline(always)]
    pub const fn STS(&self) -> super::vals::CMDH15_STS {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::CMDH15_STS::from_bits(val as u8)
    }
    #[doc = "Sample Time Select."]
    #[inline(always)]
    pub const fn set_STS(&mut self, val: super::vals::CMDH15_STS) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select."]
    #[must_use]
    #[inline(always)]
    pub const fn AVGS(&self) -> super::vals::CMDH15_AVGS {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::CMDH15_AVGS::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select."]
    #[inline(always)]
    pub const fn set_AVGS(&mut self, val: super::vals::CMDH15_AVGS) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Loop Count Select."]
    #[must_use]
    #[inline(always)]
    pub const fn LOOP(&self) -> super::vals::CMDH15_LOOP {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::CMDH15_LOOP::from_bits(val as u8)
    }
    #[doc = "Loop Count Select."]
    #[inline(always)]
    pub const fn set_LOOP(&mut self, val: super::vals::CMDH15_LOOP) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select."]
    #[must_use]
    #[inline(always)]
    pub const fn NEXT(&self) -> super::vals::CMDH15_NEXT {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::CMDH15_NEXT::from_bits(val as u8)
    }
    #[doc = "Next Command Select."]
    #[inline(always)]
    pub const fn set_NEXT(&mut self, val: super::vals::CMDH15_NEXT) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for CMDH15 {
    #[inline(always)]
    fn default() -> CMDH15 {
        CMDH15(0)
    }
}
impl core::fmt::Debug for CMDH15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDH15")
            .field("WAIT_TRIG", &self.WAIT_TRIG())
            .field("LWI", &self.LWI())
            .field("STS", &self.STS())
            .field("AVGS", &self.AVGS())
            .field("LOOP", &self.LOOP())
            .field("NEXT", &self.NEXT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CMDH15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CMDH15 {{ WAIT_TRIG: {:?}, LWI: {:?}, STS: {:?}, AVGS: {:?}, LOOP: {:?}, NEXT: {:?} }}",
            self.WAIT_TRIG(),
            self.LWI(),
            self.STS(),
            self.AVGS(),
            self.LOOP(),
            self.NEXT()
        )
    }
}
#[doc = "ADC Command High Buffer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CMDH2(pub u32);
impl CMDH2 {
    #[doc = "Compare Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn CMPEN(&self) -> super::vals::CMDH2_CMPEN {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::CMDH2_CMPEN::from_bits(val as u8)
    }
    #[doc = "Compare Function Enable."]
    #[inline(always)]
    pub const fn set_CMPEN(&mut self, val: super::vals::CMDH2_CMPEN) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Wait for trigger assertion before execution."]
    #[must_use]
    #[inline(always)]
    pub const fn WAIT_TRIG(&self) -> super::vals::CMDH2_WAIT_TRIG {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::CMDH2_WAIT_TRIG::from_bits(val as u8)
    }
    #[doc = "Wait for trigger assertion before execution."]
    #[inline(always)]
    pub const fn set_WAIT_TRIG(&mut self, val: super::vals::CMDH2_WAIT_TRIG) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Loop with Increment."]
    #[must_use]
    #[inline(always)]
    pub const fn LWI(&self) -> super::vals::CMDH2_LWI {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CMDH2_LWI::from_bits(val as u8)
    }
    #[doc = "Loop with Increment."]
    #[inline(always)]
    pub const fn set_LWI(&mut self, val: super::vals::CMDH2_LWI) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select."]
    #[must_use]
    #[inline(always)]
    pub const fn STS(&self) -> super::vals::CMDH2_STS {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::CMDH2_STS::from_bits(val as u8)
    }
    #[doc = "Sample Time Select."]
    #[inline(always)]
    pub const fn set_STS(&mut self, val: super::vals::CMDH2_STS) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select."]
    #[must_use]
    #[inline(always)]
    pub const fn AVGS(&self) -> super::vals::CMDH2_AVGS {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::CMDH2_AVGS::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select."]
    #[inline(always)]
    pub const fn set_AVGS(&mut self, val: super::vals::CMDH2_AVGS) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Loop Count Select."]
    #[must_use]
    #[inline(always)]
    pub const fn LOOP(&self) -> super::vals::CMDH2_LOOP {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::CMDH2_LOOP::from_bits(val as u8)
    }
    #[doc = "Loop Count Select."]
    #[inline(always)]
    pub const fn set_LOOP(&mut self, val: super::vals::CMDH2_LOOP) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select."]
    #[must_use]
    #[inline(always)]
    pub const fn NEXT(&self) -> super::vals::CMDH2_NEXT {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::CMDH2_NEXT::from_bits(val as u8)
    }
    #[doc = "Next Command Select."]
    #[inline(always)]
    pub const fn set_NEXT(&mut self, val: super::vals::CMDH2_NEXT) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for CMDH2 {
    #[inline(always)]
    fn default() -> CMDH2 {
        CMDH2(0)
    }
}
impl core::fmt::Debug for CMDH2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDH2")
            .field("CMPEN", &self.CMPEN())
            .field("WAIT_TRIG", &self.WAIT_TRIG())
            .field("LWI", &self.LWI())
            .field("STS", &self.STS())
            .field("AVGS", &self.AVGS())
            .field("LOOP", &self.LOOP())
            .field("NEXT", &self.NEXT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CMDH2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CMDH2 {{ CMPEN: {:?}, WAIT_TRIG: {:?}, LWI: {:?}, STS: {:?}, AVGS: {:?}, LOOP: {:?}, NEXT: {:?} }}",
            self.CMPEN(),
            self.WAIT_TRIG(),
            self.LWI(),
            self.STS(),
            self.AVGS(),
            self.LOOP(),
            self.NEXT()
        )
    }
}
#[doc = "ADC Command High Buffer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CMDH3(pub u32);
impl CMDH3 {
    #[doc = "Compare Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn CMPEN(&self) -> super::vals::CMDH3_CMPEN {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::CMDH3_CMPEN::from_bits(val as u8)
    }
    #[doc = "Compare Function Enable."]
    #[inline(always)]
    pub const fn set_CMPEN(&mut self, val: super::vals::CMDH3_CMPEN) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Wait for trigger assertion before execution."]
    #[must_use]
    #[inline(always)]
    pub const fn WAIT_TRIG(&self) -> super::vals::CMDH3_WAIT_TRIG {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::CMDH3_WAIT_TRIG::from_bits(val as u8)
    }
    #[doc = "Wait for trigger assertion before execution."]
    #[inline(always)]
    pub const fn set_WAIT_TRIG(&mut self, val: super::vals::CMDH3_WAIT_TRIG) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Loop with Increment."]
    #[must_use]
    #[inline(always)]
    pub const fn LWI(&self) -> super::vals::CMDH3_LWI {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CMDH3_LWI::from_bits(val as u8)
    }
    #[doc = "Loop with Increment."]
    #[inline(always)]
    pub const fn set_LWI(&mut self, val: super::vals::CMDH3_LWI) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select."]
    #[must_use]
    #[inline(always)]
    pub const fn STS(&self) -> super::vals::CMDH3_STS {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::CMDH3_STS::from_bits(val as u8)
    }
    #[doc = "Sample Time Select."]
    #[inline(always)]
    pub const fn set_STS(&mut self, val: super::vals::CMDH3_STS) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select."]
    #[must_use]
    #[inline(always)]
    pub const fn AVGS(&self) -> super::vals::CMDH3_AVGS {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::CMDH3_AVGS::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select."]
    #[inline(always)]
    pub const fn set_AVGS(&mut self, val: super::vals::CMDH3_AVGS) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Loop Count Select."]
    #[must_use]
    #[inline(always)]
    pub const fn LOOP(&self) -> super::vals::CMDH3_LOOP {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::CMDH3_LOOP::from_bits(val as u8)
    }
    #[doc = "Loop Count Select."]
    #[inline(always)]
    pub const fn set_LOOP(&mut self, val: super::vals::CMDH3_LOOP) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select."]
    #[must_use]
    #[inline(always)]
    pub const fn NEXT(&self) -> super::vals::CMDH3_NEXT {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::CMDH3_NEXT::from_bits(val as u8)
    }
    #[doc = "Next Command Select."]
    #[inline(always)]
    pub const fn set_NEXT(&mut self, val: super::vals::CMDH3_NEXT) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for CMDH3 {
    #[inline(always)]
    fn default() -> CMDH3 {
        CMDH3(0)
    }
}
impl core::fmt::Debug for CMDH3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDH3")
            .field("CMPEN", &self.CMPEN())
            .field("WAIT_TRIG", &self.WAIT_TRIG())
            .field("LWI", &self.LWI())
            .field("STS", &self.STS())
            .field("AVGS", &self.AVGS())
            .field("LOOP", &self.LOOP())
            .field("NEXT", &self.NEXT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CMDH3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CMDH3 {{ CMPEN: {:?}, WAIT_TRIG: {:?}, LWI: {:?}, STS: {:?}, AVGS: {:?}, LOOP: {:?}, NEXT: {:?} }}",
            self.CMPEN(),
            self.WAIT_TRIG(),
            self.LWI(),
            self.STS(),
            self.AVGS(),
            self.LOOP(),
            self.NEXT()
        )
    }
}
#[doc = "ADC Command High Buffer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CMDH4(pub u32);
impl CMDH4 {
    #[doc = "Compare Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn CMPEN(&self) -> super::vals::CMDH4_CMPEN {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::CMDH4_CMPEN::from_bits(val as u8)
    }
    #[doc = "Compare Function Enable."]
    #[inline(always)]
    pub const fn set_CMPEN(&mut self, val: super::vals::CMDH4_CMPEN) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Wait for trigger assertion before execution."]
    #[must_use]
    #[inline(always)]
    pub const fn WAIT_TRIG(&self) -> super::vals::CMDH4_WAIT_TRIG {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::CMDH4_WAIT_TRIG::from_bits(val as u8)
    }
    #[doc = "Wait for trigger assertion before execution."]
    #[inline(always)]
    pub const fn set_WAIT_TRIG(&mut self, val: super::vals::CMDH4_WAIT_TRIG) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Loop with Increment."]
    #[must_use]
    #[inline(always)]
    pub const fn LWI(&self) -> super::vals::CMDH4_LWI {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CMDH4_LWI::from_bits(val as u8)
    }
    #[doc = "Loop with Increment."]
    #[inline(always)]
    pub const fn set_LWI(&mut self, val: super::vals::CMDH4_LWI) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select."]
    #[must_use]
    #[inline(always)]
    pub const fn STS(&self) -> super::vals::CMDH4_STS {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::CMDH4_STS::from_bits(val as u8)
    }
    #[doc = "Sample Time Select."]
    #[inline(always)]
    pub const fn set_STS(&mut self, val: super::vals::CMDH4_STS) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select."]
    #[must_use]
    #[inline(always)]
    pub const fn AVGS(&self) -> super::vals::CMDH4_AVGS {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::CMDH4_AVGS::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select."]
    #[inline(always)]
    pub const fn set_AVGS(&mut self, val: super::vals::CMDH4_AVGS) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Loop Count Select."]
    #[must_use]
    #[inline(always)]
    pub const fn LOOP(&self) -> super::vals::CMDH4_LOOP {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::CMDH4_LOOP::from_bits(val as u8)
    }
    #[doc = "Loop Count Select."]
    #[inline(always)]
    pub const fn set_LOOP(&mut self, val: super::vals::CMDH4_LOOP) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select."]
    #[must_use]
    #[inline(always)]
    pub const fn NEXT(&self) -> super::vals::CMDH4_NEXT {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::CMDH4_NEXT::from_bits(val as u8)
    }
    #[doc = "Next Command Select."]
    #[inline(always)]
    pub const fn set_NEXT(&mut self, val: super::vals::CMDH4_NEXT) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for CMDH4 {
    #[inline(always)]
    fn default() -> CMDH4 {
        CMDH4(0)
    }
}
impl core::fmt::Debug for CMDH4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDH4")
            .field("CMPEN", &self.CMPEN())
            .field("WAIT_TRIG", &self.WAIT_TRIG())
            .field("LWI", &self.LWI())
            .field("STS", &self.STS())
            .field("AVGS", &self.AVGS())
            .field("LOOP", &self.LOOP())
            .field("NEXT", &self.NEXT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CMDH4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CMDH4 {{ CMPEN: {:?}, WAIT_TRIG: {:?}, LWI: {:?}, STS: {:?}, AVGS: {:?}, LOOP: {:?}, NEXT: {:?} }}",
            self.CMPEN(),
            self.WAIT_TRIG(),
            self.LWI(),
            self.STS(),
            self.AVGS(),
            self.LOOP(),
            self.NEXT()
        )
    }
}
#[doc = "ADC Command High Buffer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CMDH5(pub u32);
impl CMDH5 {
    #[doc = "Wait for trigger assertion before execution."]
    #[must_use]
    #[inline(always)]
    pub const fn WAIT_TRIG(&self) -> super::vals::CMDH5_WAIT_TRIG {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::CMDH5_WAIT_TRIG::from_bits(val as u8)
    }
    #[doc = "Wait for trigger assertion before execution."]
    #[inline(always)]
    pub const fn set_WAIT_TRIG(&mut self, val: super::vals::CMDH5_WAIT_TRIG) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Loop with Increment."]
    #[must_use]
    #[inline(always)]
    pub const fn LWI(&self) -> super::vals::CMDH5_LWI {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CMDH5_LWI::from_bits(val as u8)
    }
    #[doc = "Loop with Increment."]
    #[inline(always)]
    pub const fn set_LWI(&mut self, val: super::vals::CMDH5_LWI) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select."]
    #[must_use]
    #[inline(always)]
    pub const fn STS(&self) -> super::vals::CMDH5_STS {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::CMDH5_STS::from_bits(val as u8)
    }
    #[doc = "Sample Time Select."]
    #[inline(always)]
    pub const fn set_STS(&mut self, val: super::vals::CMDH5_STS) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select."]
    #[must_use]
    #[inline(always)]
    pub const fn AVGS(&self) -> super::vals::CMDH5_AVGS {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::CMDH5_AVGS::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select."]
    #[inline(always)]
    pub const fn set_AVGS(&mut self, val: super::vals::CMDH5_AVGS) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Loop Count Select."]
    #[must_use]
    #[inline(always)]
    pub const fn LOOP(&self) -> super::vals::CMDH5_LOOP {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::CMDH5_LOOP::from_bits(val as u8)
    }
    #[doc = "Loop Count Select."]
    #[inline(always)]
    pub const fn set_LOOP(&mut self, val: super::vals::CMDH5_LOOP) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select."]
    #[must_use]
    #[inline(always)]
    pub const fn NEXT(&self) -> super::vals::CMDH5_NEXT {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::CMDH5_NEXT::from_bits(val as u8)
    }
    #[doc = "Next Command Select."]
    #[inline(always)]
    pub const fn set_NEXT(&mut self, val: super::vals::CMDH5_NEXT) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for CMDH5 {
    #[inline(always)]
    fn default() -> CMDH5 {
        CMDH5(0)
    }
}
impl core::fmt::Debug for CMDH5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDH5")
            .field("WAIT_TRIG", &self.WAIT_TRIG())
            .field("LWI", &self.LWI())
            .field("STS", &self.STS())
            .field("AVGS", &self.AVGS())
            .field("LOOP", &self.LOOP())
            .field("NEXT", &self.NEXT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CMDH5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CMDH5 {{ WAIT_TRIG: {:?}, LWI: {:?}, STS: {:?}, AVGS: {:?}, LOOP: {:?}, NEXT: {:?} }}",
            self.WAIT_TRIG(),
            self.LWI(),
            self.STS(),
            self.AVGS(),
            self.LOOP(),
            self.NEXT()
        )
    }
}
#[doc = "ADC Command High Buffer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CMDH6(pub u32);
impl CMDH6 {
    #[doc = "Wait for trigger assertion before execution."]
    #[must_use]
    #[inline(always)]
    pub const fn WAIT_TRIG(&self) -> super::vals::CMDH6_WAIT_TRIG {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::CMDH6_WAIT_TRIG::from_bits(val as u8)
    }
    #[doc = "Wait for trigger assertion before execution."]
    #[inline(always)]
    pub const fn set_WAIT_TRIG(&mut self, val: super::vals::CMDH6_WAIT_TRIG) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Loop with Increment."]
    #[must_use]
    #[inline(always)]
    pub const fn LWI(&self) -> super::vals::CMDH6_LWI {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CMDH6_LWI::from_bits(val as u8)
    }
    #[doc = "Loop with Increment."]
    #[inline(always)]
    pub const fn set_LWI(&mut self, val: super::vals::CMDH6_LWI) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select."]
    #[must_use]
    #[inline(always)]
    pub const fn STS(&self) -> super::vals::CMDH6_STS {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::CMDH6_STS::from_bits(val as u8)
    }
    #[doc = "Sample Time Select."]
    #[inline(always)]
    pub const fn set_STS(&mut self, val: super::vals::CMDH6_STS) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select."]
    #[must_use]
    #[inline(always)]
    pub const fn AVGS(&self) -> super::vals::CMDH6_AVGS {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::CMDH6_AVGS::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select."]
    #[inline(always)]
    pub const fn set_AVGS(&mut self, val: super::vals::CMDH6_AVGS) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Loop Count Select."]
    #[must_use]
    #[inline(always)]
    pub const fn LOOP(&self) -> super::vals::CMDH6_LOOP {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::CMDH6_LOOP::from_bits(val as u8)
    }
    #[doc = "Loop Count Select."]
    #[inline(always)]
    pub const fn set_LOOP(&mut self, val: super::vals::CMDH6_LOOP) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select."]
    #[must_use]
    #[inline(always)]
    pub const fn NEXT(&self) -> super::vals::CMDH6_NEXT {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::CMDH6_NEXT::from_bits(val as u8)
    }
    #[doc = "Next Command Select."]
    #[inline(always)]
    pub const fn set_NEXT(&mut self, val: super::vals::CMDH6_NEXT) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for CMDH6 {
    #[inline(always)]
    fn default() -> CMDH6 {
        CMDH6(0)
    }
}
impl core::fmt::Debug for CMDH6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDH6")
            .field("WAIT_TRIG", &self.WAIT_TRIG())
            .field("LWI", &self.LWI())
            .field("STS", &self.STS())
            .field("AVGS", &self.AVGS())
            .field("LOOP", &self.LOOP())
            .field("NEXT", &self.NEXT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CMDH6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CMDH6 {{ WAIT_TRIG: {:?}, LWI: {:?}, STS: {:?}, AVGS: {:?}, LOOP: {:?}, NEXT: {:?} }}",
            self.WAIT_TRIG(),
            self.LWI(),
            self.STS(),
            self.AVGS(),
            self.LOOP(),
            self.NEXT()
        )
    }
}
#[doc = "ADC Command High Buffer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CMDH7(pub u32);
impl CMDH7 {
    #[doc = "Wait for trigger assertion before execution."]
    #[must_use]
    #[inline(always)]
    pub const fn WAIT_TRIG(&self) -> super::vals::CMDH7_WAIT_TRIG {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::CMDH7_WAIT_TRIG::from_bits(val as u8)
    }
    #[doc = "Wait for trigger assertion before execution."]
    #[inline(always)]
    pub const fn set_WAIT_TRIG(&mut self, val: super::vals::CMDH7_WAIT_TRIG) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Loop with Increment."]
    #[must_use]
    #[inline(always)]
    pub const fn LWI(&self) -> super::vals::CMDH7_LWI {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CMDH7_LWI::from_bits(val as u8)
    }
    #[doc = "Loop with Increment."]
    #[inline(always)]
    pub const fn set_LWI(&mut self, val: super::vals::CMDH7_LWI) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select."]
    #[must_use]
    #[inline(always)]
    pub const fn STS(&self) -> super::vals::CMDH7_STS {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::CMDH7_STS::from_bits(val as u8)
    }
    #[doc = "Sample Time Select."]
    #[inline(always)]
    pub const fn set_STS(&mut self, val: super::vals::CMDH7_STS) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select."]
    #[must_use]
    #[inline(always)]
    pub const fn AVGS(&self) -> super::vals::CMDH7_AVGS {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::CMDH7_AVGS::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select."]
    #[inline(always)]
    pub const fn set_AVGS(&mut self, val: super::vals::CMDH7_AVGS) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Loop Count Select."]
    #[must_use]
    #[inline(always)]
    pub const fn LOOP(&self) -> super::vals::CMDH7_LOOP {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::CMDH7_LOOP::from_bits(val as u8)
    }
    #[doc = "Loop Count Select."]
    #[inline(always)]
    pub const fn set_LOOP(&mut self, val: super::vals::CMDH7_LOOP) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select."]
    #[must_use]
    #[inline(always)]
    pub const fn NEXT(&self) -> super::vals::CMDH7_NEXT {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::CMDH7_NEXT::from_bits(val as u8)
    }
    #[doc = "Next Command Select."]
    #[inline(always)]
    pub const fn set_NEXT(&mut self, val: super::vals::CMDH7_NEXT) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for CMDH7 {
    #[inline(always)]
    fn default() -> CMDH7 {
        CMDH7(0)
    }
}
impl core::fmt::Debug for CMDH7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDH7")
            .field("WAIT_TRIG", &self.WAIT_TRIG())
            .field("LWI", &self.LWI())
            .field("STS", &self.STS())
            .field("AVGS", &self.AVGS())
            .field("LOOP", &self.LOOP())
            .field("NEXT", &self.NEXT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CMDH7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CMDH7 {{ WAIT_TRIG: {:?}, LWI: {:?}, STS: {:?}, AVGS: {:?}, LOOP: {:?}, NEXT: {:?} }}",
            self.WAIT_TRIG(),
            self.LWI(),
            self.STS(),
            self.AVGS(),
            self.LOOP(),
            self.NEXT()
        )
    }
}
#[doc = "ADC Command High Buffer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CMDH8(pub u32);
impl CMDH8 {
    #[doc = "Wait for trigger assertion before execution."]
    #[must_use]
    #[inline(always)]
    pub const fn WAIT_TRIG(&self) -> super::vals::CMDH8_WAIT_TRIG {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::CMDH8_WAIT_TRIG::from_bits(val as u8)
    }
    #[doc = "Wait for trigger assertion before execution."]
    #[inline(always)]
    pub const fn set_WAIT_TRIG(&mut self, val: super::vals::CMDH8_WAIT_TRIG) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Loop with Increment."]
    #[must_use]
    #[inline(always)]
    pub const fn LWI(&self) -> super::vals::CMDH8_LWI {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CMDH8_LWI::from_bits(val as u8)
    }
    #[doc = "Loop with Increment."]
    #[inline(always)]
    pub const fn set_LWI(&mut self, val: super::vals::CMDH8_LWI) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select."]
    #[must_use]
    #[inline(always)]
    pub const fn STS(&self) -> super::vals::CMDH8_STS {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::CMDH8_STS::from_bits(val as u8)
    }
    #[doc = "Sample Time Select."]
    #[inline(always)]
    pub const fn set_STS(&mut self, val: super::vals::CMDH8_STS) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select."]
    #[must_use]
    #[inline(always)]
    pub const fn AVGS(&self) -> super::vals::CMDH8_AVGS {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::CMDH8_AVGS::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select."]
    #[inline(always)]
    pub const fn set_AVGS(&mut self, val: super::vals::CMDH8_AVGS) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Loop Count Select."]
    #[must_use]
    #[inline(always)]
    pub const fn LOOP(&self) -> super::vals::CMDH8_LOOP {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::CMDH8_LOOP::from_bits(val as u8)
    }
    #[doc = "Loop Count Select."]
    #[inline(always)]
    pub const fn set_LOOP(&mut self, val: super::vals::CMDH8_LOOP) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select."]
    #[must_use]
    #[inline(always)]
    pub const fn NEXT(&self) -> super::vals::CMDH8_NEXT {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::CMDH8_NEXT::from_bits(val as u8)
    }
    #[doc = "Next Command Select."]
    #[inline(always)]
    pub const fn set_NEXT(&mut self, val: super::vals::CMDH8_NEXT) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for CMDH8 {
    #[inline(always)]
    fn default() -> CMDH8 {
        CMDH8(0)
    }
}
impl core::fmt::Debug for CMDH8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDH8")
            .field("WAIT_TRIG", &self.WAIT_TRIG())
            .field("LWI", &self.LWI())
            .field("STS", &self.STS())
            .field("AVGS", &self.AVGS())
            .field("LOOP", &self.LOOP())
            .field("NEXT", &self.NEXT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CMDH8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CMDH8 {{ WAIT_TRIG: {:?}, LWI: {:?}, STS: {:?}, AVGS: {:?}, LOOP: {:?}, NEXT: {:?} }}",
            self.WAIT_TRIG(),
            self.LWI(),
            self.STS(),
            self.AVGS(),
            self.LOOP(),
            self.NEXT()
        )
    }
}
#[doc = "ADC Command High Buffer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CMDH9(pub u32);
impl CMDH9 {
    #[doc = "Wait for trigger assertion before execution."]
    #[must_use]
    #[inline(always)]
    pub const fn WAIT_TRIG(&self) -> super::vals::CMDH9_WAIT_TRIG {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::CMDH9_WAIT_TRIG::from_bits(val as u8)
    }
    #[doc = "Wait for trigger assertion before execution."]
    #[inline(always)]
    pub const fn set_WAIT_TRIG(&mut self, val: super::vals::CMDH9_WAIT_TRIG) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Loop with Increment."]
    #[must_use]
    #[inline(always)]
    pub const fn LWI(&self) -> super::vals::CMDH9_LWI {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CMDH9_LWI::from_bits(val as u8)
    }
    #[doc = "Loop with Increment."]
    #[inline(always)]
    pub const fn set_LWI(&mut self, val: super::vals::CMDH9_LWI) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select."]
    #[must_use]
    #[inline(always)]
    pub const fn STS(&self) -> super::vals::CMDH9_STS {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::CMDH9_STS::from_bits(val as u8)
    }
    #[doc = "Sample Time Select."]
    #[inline(always)]
    pub const fn set_STS(&mut self, val: super::vals::CMDH9_STS) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select."]
    #[must_use]
    #[inline(always)]
    pub const fn AVGS(&self) -> super::vals::CMDH9_AVGS {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::CMDH9_AVGS::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select."]
    #[inline(always)]
    pub const fn set_AVGS(&mut self, val: super::vals::CMDH9_AVGS) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Loop Count Select."]
    #[must_use]
    #[inline(always)]
    pub const fn LOOP(&self) -> super::vals::CMDH9_LOOP {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::CMDH9_LOOP::from_bits(val as u8)
    }
    #[doc = "Loop Count Select."]
    #[inline(always)]
    pub const fn set_LOOP(&mut self, val: super::vals::CMDH9_LOOP) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select."]
    #[must_use]
    #[inline(always)]
    pub const fn NEXT(&self) -> super::vals::CMDH9_NEXT {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::CMDH9_NEXT::from_bits(val as u8)
    }
    #[doc = "Next Command Select."]
    #[inline(always)]
    pub const fn set_NEXT(&mut self, val: super::vals::CMDH9_NEXT) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for CMDH9 {
    #[inline(always)]
    fn default() -> CMDH9 {
        CMDH9(0)
    }
}
impl core::fmt::Debug for CMDH9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDH9")
            .field("WAIT_TRIG", &self.WAIT_TRIG())
            .field("LWI", &self.LWI())
            .field("STS", &self.STS())
            .field("AVGS", &self.AVGS())
            .field("LOOP", &self.LOOP())
            .field("NEXT", &self.NEXT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CMDH9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CMDH9 {{ WAIT_TRIG: {:?}, LWI: {:?}, STS: {:?}, AVGS: {:?}, LOOP: {:?}, NEXT: {:?} }}",
            self.WAIT_TRIG(),
            self.LWI(),
            self.STS(),
            self.AVGS(),
            self.LOOP(),
            self.NEXT()
        )
    }
}
#[doc = "ADC Command Low Buffer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CMDL1(pub u32);
impl CMDL1 {
    #[doc = "Input channel select."]
    #[must_use]
    #[inline(always)]
    pub const fn ADCH(&self) -> super::vals::CMDL1_ADCH {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::CMDL1_ADCH::from_bits(val as u8)
    }
    #[doc = "Input channel select."]
    #[inline(always)]
    pub const fn set_ADCH(&mut self, val: super::vals::CMDL1_ADCH) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Conversion Type."]
    #[must_use]
    #[inline(always)]
    pub const fn CTYPE(&self) -> super::vals::CMDL1_CTYPE {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::CMDL1_CTYPE::from_bits(val as u8)
    }
    #[doc = "Conversion Type."]
    #[inline(always)]
    pub const fn set_CTYPE(&mut self, val: super::vals::CMDL1_CTYPE) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Select resolution of conversions."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::CMDL1_MODE {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CMDL1_MODE::from_bits(val as u8)
    }
    #[doc = "Select resolution of conversions."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::CMDL1_MODE) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for CMDL1 {
    #[inline(always)]
    fn default() -> CMDL1 {
        CMDL1(0)
    }
}
impl core::fmt::Debug for CMDL1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDL1")
            .field("ADCH", &self.ADCH())
            .field("CTYPE", &self.CTYPE())
            .field("MODE", &self.MODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CMDL1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CMDL1 {{ ADCH: {:?}, CTYPE: {:?}, MODE: {:?} }}",
            self.ADCH(),
            self.CTYPE(),
            self.MODE()
        )
    }
}
#[doc = "ADC Command Low Buffer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CMDL10(pub u32);
impl CMDL10 {
    #[doc = "Input channel select."]
    #[must_use]
    #[inline(always)]
    pub const fn ADCH(&self) -> super::vals::CMDL10_ADCH {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::CMDL10_ADCH::from_bits(val as u8)
    }
    #[doc = "Input channel select."]
    #[inline(always)]
    pub const fn set_ADCH(&mut self, val: super::vals::CMDL10_ADCH) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Conversion Type."]
    #[must_use]
    #[inline(always)]
    pub const fn CTYPE(&self) -> super::vals::CMDL10_CTYPE {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::CMDL10_CTYPE::from_bits(val as u8)
    }
    #[doc = "Conversion Type."]
    #[inline(always)]
    pub const fn set_CTYPE(&mut self, val: super::vals::CMDL10_CTYPE) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Select resolution of conversions."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::CMDL10_MODE {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CMDL10_MODE::from_bits(val as u8)
    }
    #[doc = "Select resolution of conversions."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::CMDL10_MODE) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for CMDL10 {
    #[inline(always)]
    fn default() -> CMDL10 {
        CMDL10(0)
    }
}
impl core::fmt::Debug for CMDL10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDL10")
            .field("ADCH", &self.ADCH())
            .field("CTYPE", &self.CTYPE())
            .field("MODE", &self.MODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CMDL10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CMDL10 {{ ADCH: {:?}, CTYPE: {:?}, MODE: {:?} }}",
            self.ADCH(),
            self.CTYPE(),
            self.MODE()
        )
    }
}
#[doc = "ADC Command Low Buffer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CMDL11(pub u32);
impl CMDL11 {
    #[doc = "Input channel select."]
    #[must_use]
    #[inline(always)]
    pub const fn ADCH(&self) -> super::vals::CMDL11_ADCH {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::CMDL11_ADCH::from_bits(val as u8)
    }
    #[doc = "Input channel select."]
    #[inline(always)]
    pub const fn set_ADCH(&mut self, val: super::vals::CMDL11_ADCH) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Conversion Type."]
    #[must_use]
    #[inline(always)]
    pub const fn CTYPE(&self) -> super::vals::CMDL11_CTYPE {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::CMDL11_CTYPE::from_bits(val as u8)
    }
    #[doc = "Conversion Type."]
    #[inline(always)]
    pub const fn set_CTYPE(&mut self, val: super::vals::CMDL11_CTYPE) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Select resolution of conversions."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::CMDL11_MODE {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CMDL11_MODE::from_bits(val as u8)
    }
    #[doc = "Select resolution of conversions."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::CMDL11_MODE) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for CMDL11 {
    #[inline(always)]
    fn default() -> CMDL11 {
        CMDL11(0)
    }
}
impl core::fmt::Debug for CMDL11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDL11")
            .field("ADCH", &self.ADCH())
            .field("CTYPE", &self.CTYPE())
            .field("MODE", &self.MODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CMDL11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CMDL11 {{ ADCH: {:?}, CTYPE: {:?}, MODE: {:?} }}",
            self.ADCH(),
            self.CTYPE(),
            self.MODE()
        )
    }
}
#[doc = "ADC Command Low Buffer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CMDL12(pub u32);
impl CMDL12 {
    #[doc = "Input channel select."]
    #[must_use]
    #[inline(always)]
    pub const fn ADCH(&self) -> super::vals::CMDL12_ADCH {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::CMDL12_ADCH::from_bits(val as u8)
    }
    #[doc = "Input channel select."]
    #[inline(always)]
    pub const fn set_ADCH(&mut self, val: super::vals::CMDL12_ADCH) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Conversion Type."]
    #[must_use]
    #[inline(always)]
    pub const fn CTYPE(&self) -> super::vals::CMDL12_CTYPE {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::CMDL12_CTYPE::from_bits(val as u8)
    }
    #[doc = "Conversion Type."]
    #[inline(always)]
    pub const fn set_CTYPE(&mut self, val: super::vals::CMDL12_CTYPE) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Select resolution of conversions."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::CMDL12_MODE {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CMDL12_MODE::from_bits(val as u8)
    }
    #[doc = "Select resolution of conversions."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::CMDL12_MODE) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for CMDL12 {
    #[inline(always)]
    fn default() -> CMDL12 {
        CMDL12(0)
    }
}
impl core::fmt::Debug for CMDL12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDL12")
            .field("ADCH", &self.ADCH())
            .field("CTYPE", &self.CTYPE())
            .field("MODE", &self.MODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CMDL12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CMDL12 {{ ADCH: {:?}, CTYPE: {:?}, MODE: {:?} }}",
            self.ADCH(),
            self.CTYPE(),
            self.MODE()
        )
    }
}
#[doc = "ADC Command Low Buffer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CMDL13(pub u32);
impl CMDL13 {
    #[doc = "Input channel select."]
    #[must_use]
    #[inline(always)]
    pub const fn ADCH(&self) -> super::vals::CMDL13_ADCH {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::CMDL13_ADCH::from_bits(val as u8)
    }
    #[doc = "Input channel select."]
    #[inline(always)]
    pub const fn set_ADCH(&mut self, val: super::vals::CMDL13_ADCH) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Conversion Type."]
    #[must_use]
    #[inline(always)]
    pub const fn CTYPE(&self) -> super::vals::CMDL13_CTYPE {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::CMDL13_CTYPE::from_bits(val as u8)
    }
    #[doc = "Conversion Type."]
    #[inline(always)]
    pub const fn set_CTYPE(&mut self, val: super::vals::CMDL13_CTYPE) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Select resolution of conversions."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::CMDL13_MODE {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CMDL13_MODE::from_bits(val as u8)
    }
    #[doc = "Select resolution of conversions."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::CMDL13_MODE) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for CMDL13 {
    #[inline(always)]
    fn default() -> CMDL13 {
        CMDL13(0)
    }
}
impl core::fmt::Debug for CMDL13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDL13")
            .field("ADCH", &self.ADCH())
            .field("CTYPE", &self.CTYPE())
            .field("MODE", &self.MODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CMDL13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CMDL13 {{ ADCH: {:?}, CTYPE: {:?}, MODE: {:?} }}",
            self.ADCH(),
            self.CTYPE(),
            self.MODE()
        )
    }
}
#[doc = "ADC Command Low Buffer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CMDL14(pub u32);
impl CMDL14 {
    #[doc = "Input channel select."]
    #[must_use]
    #[inline(always)]
    pub const fn ADCH(&self) -> super::vals::CMDL14_ADCH {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::CMDL14_ADCH::from_bits(val as u8)
    }
    #[doc = "Input channel select."]
    #[inline(always)]
    pub const fn set_ADCH(&mut self, val: super::vals::CMDL14_ADCH) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Conversion Type."]
    #[must_use]
    #[inline(always)]
    pub const fn CTYPE(&self) -> super::vals::CMDL14_CTYPE {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::CMDL14_CTYPE::from_bits(val as u8)
    }
    #[doc = "Conversion Type."]
    #[inline(always)]
    pub const fn set_CTYPE(&mut self, val: super::vals::CMDL14_CTYPE) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Select resolution of conversions."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::CMDL14_MODE {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CMDL14_MODE::from_bits(val as u8)
    }
    #[doc = "Select resolution of conversions."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::CMDL14_MODE) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for CMDL14 {
    #[inline(always)]
    fn default() -> CMDL14 {
        CMDL14(0)
    }
}
impl core::fmt::Debug for CMDL14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDL14")
            .field("ADCH", &self.ADCH())
            .field("CTYPE", &self.CTYPE())
            .field("MODE", &self.MODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CMDL14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CMDL14 {{ ADCH: {:?}, CTYPE: {:?}, MODE: {:?} }}",
            self.ADCH(),
            self.CTYPE(),
            self.MODE()
        )
    }
}
#[doc = "ADC Command Low Buffer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CMDL15(pub u32);
impl CMDL15 {
    #[doc = "Input channel select."]
    #[must_use]
    #[inline(always)]
    pub const fn ADCH(&self) -> super::vals::CMDL15_ADCH {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::CMDL15_ADCH::from_bits(val as u8)
    }
    #[doc = "Input channel select."]
    #[inline(always)]
    pub const fn set_ADCH(&mut self, val: super::vals::CMDL15_ADCH) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Conversion Type."]
    #[must_use]
    #[inline(always)]
    pub const fn CTYPE(&self) -> super::vals::CMDL15_CTYPE {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::CMDL15_CTYPE::from_bits(val as u8)
    }
    #[doc = "Conversion Type."]
    #[inline(always)]
    pub const fn set_CTYPE(&mut self, val: super::vals::CMDL15_CTYPE) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Select resolution of conversions."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::CMDL15_MODE {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CMDL15_MODE::from_bits(val as u8)
    }
    #[doc = "Select resolution of conversions."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::CMDL15_MODE) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for CMDL15 {
    #[inline(always)]
    fn default() -> CMDL15 {
        CMDL15(0)
    }
}
impl core::fmt::Debug for CMDL15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDL15")
            .field("ADCH", &self.ADCH())
            .field("CTYPE", &self.CTYPE())
            .field("MODE", &self.MODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CMDL15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CMDL15 {{ ADCH: {:?}, CTYPE: {:?}, MODE: {:?} }}",
            self.ADCH(),
            self.CTYPE(),
            self.MODE()
        )
    }
}
#[doc = "ADC Command Low Buffer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CMDL2(pub u32);
impl CMDL2 {
    #[doc = "Input channel select."]
    #[must_use]
    #[inline(always)]
    pub const fn ADCH(&self) -> super::vals::CMDL2_ADCH {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::CMDL2_ADCH::from_bits(val as u8)
    }
    #[doc = "Input channel select."]
    #[inline(always)]
    pub const fn set_ADCH(&mut self, val: super::vals::CMDL2_ADCH) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Conversion Type."]
    #[must_use]
    #[inline(always)]
    pub const fn CTYPE(&self) -> super::vals::CMDL2_CTYPE {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::CMDL2_CTYPE::from_bits(val as u8)
    }
    #[doc = "Conversion Type."]
    #[inline(always)]
    pub const fn set_CTYPE(&mut self, val: super::vals::CMDL2_CTYPE) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Select resolution of conversions."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::CMDL2_MODE {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CMDL2_MODE::from_bits(val as u8)
    }
    #[doc = "Select resolution of conversions."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::CMDL2_MODE) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for CMDL2 {
    #[inline(always)]
    fn default() -> CMDL2 {
        CMDL2(0)
    }
}
impl core::fmt::Debug for CMDL2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDL2")
            .field("ADCH", &self.ADCH())
            .field("CTYPE", &self.CTYPE())
            .field("MODE", &self.MODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CMDL2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CMDL2 {{ ADCH: {:?}, CTYPE: {:?}, MODE: {:?} }}",
            self.ADCH(),
            self.CTYPE(),
            self.MODE()
        )
    }
}
#[doc = "ADC Command Low Buffer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CMDL3(pub u32);
impl CMDL3 {
    #[doc = "Input channel select."]
    #[must_use]
    #[inline(always)]
    pub const fn ADCH(&self) -> super::vals::CMDL3_ADCH {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::CMDL3_ADCH::from_bits(val as u8)
    }
    #[doc = "Input channel select."]
    #[inline(always)]
    pub const fn set_ADCH(&mut self, val: super::vals::CMDL3_ADCH) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Conversion Type."]
    #[must_use]
    #[inline(always)]
    pub const fn CTYPE(&self) -> super::vals::CMDL3_CTYPE {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::CMDL3_CTYPE::from_bits(val as u8)
    }
    #[doc = "Conversion Type."]
    #[inline(always)]
    pub const fn set_CTYPE(&mut self, val: super::vals::CMDL3_CTYPE) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Select resolution of conversions."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::CMDL3_MODE {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CMDL3_MODE::from_bits(val as u8)
    }
    #[doc = "Select resolution of conversions."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::CMDL3_MODE) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for CMDL3 {
    #[inline(always)]
    fn default() -> CMDL3 {
        CMDL3(0)
    }
}
impl core::fmt::Debug for CMDL3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDL3")
            .field("ADCH", &self.ADCH())
            .field("CTYPE", &self.CTYPE())
            .field("MODE", &self.MODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CMDL3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CMDL3 {{ ADCH: {:?}, CTYPE: {:?}, MODE: {:?} }}",
            self.ADCH(),
            self.CTYPE(),
            self.MODE()
        )
    }
}
#[doc = "ADC Command Low Buffer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CMDL4(pub u32);
impl CMDL4 {
    #[doc = "Input channel select."]
    #[must_use]
    #[inline(always)]
    pub const fn ADCH(&self) -> super::vals::CMDL4_ADCH {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::CMDL4_ADCH::from_bits(val as u8)
    }
    #[doc = "Input channel select."]
    #[inline(always)]
    pub const fn set_ADCH(&mut self, val: super::vals::CMDL4_ADCH) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Conversion Type."]
    #[must_use]
    #[inline(always)]
    pub const fn CTYPE(&self) -> super::vals::CMDL4_CTYPE {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::CMDL4_CTYPE::from_bits(val as u8)
    }
    #[doc = "Conversion Type."]
    #[inline(always)]
    pub const fn set_CTYPE(&mut self, val: super::vals::CMDL4_CTYPE) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Select resolution of conversions."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::CMDL4_MODE {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CMDL4_MODE::from_bits(val as u8)
    }
    #[doc = "Select resolution of conversions."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::CMDL4_MODE) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for CMDL4 {
    #[inline(always)]
    fn default() -> CMDL4 {
        CMDL4(0)
    }
}
impl core::fmt::Debug for CMDL4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDL4")
            .field("ADCH", &self.ADCH())
            .field("CTYPE", &self.CTYPE())
            .field("MODE", &self.MODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CMDL4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CMDL4 {{ ADCH: {:?}, CTYPE: {:?}, MODE: {:?} }}",
            self.ADCH(),
            self.CTYPE(),
            self.MODE()
        )
    }
}
#[doc = "ADC Command Low Buffer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CMDL5(pub u32);
impl CMDL5 {
    #[doc = "Input channel select."]
    #[must_use]
    #[inline(always)]
    pub const fn ADCH(&self) -> super::vals::CMDL5_ADCH {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::CMDL5_ADCH::from_bits(val as u8)
    }
    #[doc = "Input channel select."]
    #[inline(always)]
    pub const fn set_ADCH(&mut self, val: super::vals::CMDL5_ADCH) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Conversion Type."]
    #[must_use]
    #[inline(always)]
    pub const fn CTYPE(&self) -> super::vals::CMDL5_CTYPE {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::CMDL5_CTYPE::from_bits(val as u8)
    }
    #[doc = "Conversion Type."]
    #[inline(always)]
    pub const fn set_CTYPE(&mut self, val: super::vals::CMDL5_CTYPE) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Select resolution of conversions."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::CMDL5_MODE {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CMDL5_MODE::from_bits(val as u8)
    }
    #[doc = "Select resolution of conversions."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::CMDL5_MODE) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for CMDL5 {
    #[inline(always)]
    fn default() -> CMDL5 {
        CMDL5(0)
    }
}
impl core::fmt::Debug for CMDL5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDL5")
            .field("ADCH", &self.ADCH())
            .field("CTYPE", &self.CTYPE())
            .field("MODE", &self.MODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CMDL5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CMDL5 {{ ADCH: {:?}, CTYPE: {:?}, MODE: {:?} }}",
            self.ADCH(),
            self.CTYPE(),
            self.MODE()
        )
    }
}
#[doc = "ADC Command Low Buffer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CMDL6(pub u32);
impl CMDL6 {
    #[doc = "Input channel select."]
    #[must_use]
    #[inline(always)]
    pub const fn ADCH(&self) -> super::vals::CMDL6_ADCH {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::CMDL6_ADCH::from_bits(val as u8)
    }
    #[doc = "Input channel select."]
    #[inline(always)]
    pub const fn set_ADCH(&mut self, val: super::vals::CMDL6_ADCH) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Conversion Type."]
    #[must_use]
    #[inline(always)]
    pub const fn CTYPE(&self) -> super::vals::CMDL6_CTYPE {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::CMDL6_CTYPE::from_bits(val as u8)
    }
    #[doc = "Conversion Type."]
    #[inline(always)]
    pub const fn set_CTYPE(&mut self, val: super::vals::CMDL6_CTYPE) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Select resolution of conversions."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::CMDL6_MODE {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CMDL6_MODE::from_bits(val as u8)
    }
    #[doc = "Select resolution of conversions."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::CMDL6_MODE) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for CMDL6 {
    #[inline(always)]
    fn default() -> CMDL6 {
        CMDL6(0)
    }
}
impl core::fmt::Debug for CMDL6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDL6")
            .field("ADCH", &self.ADCH())
            .field("CTYPE", &self.CTYPE())
            .field("MODE", &self.MODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CMDL6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CMDL6 {{ ADCH: {:?}, CTYPE: {:?}, MODE: {:?} }}",
            self.ADCH(),
            self.CTYPE(),
            self.MODE()
        )
    }
}
#[doc = "ADC Command Low Buffer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CMDL7(pub u32);
impl CMDL7 {
    #[doc = "Input channel select."]
    #[must_use]
    #[inline(always)]
    pub const fn ADCH(&self) -> super::vals::CMDL7_ADCH {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::CMDL7_ADCH::from_bits(val as u8)
    }
    #[doc = "Input channel select."]
    #[inline(always)]
    pub const fn set_ADCH(&mut self, val: super::vals::CMDL7_ADCH) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Conversion Type."]
    #[must_use]
    #[inline(always)]
    pub const fn CTYPE(&self) -> super::vals::CMDL7_CTYPE {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::CMDL7_CTYPE::from_bits(val as u8)
    }
    #[doc = "Conversion Type."]
    #[inline(always)]
    pub const fn set_CTYPE(&mut self, val: super::vals::CMDL7_CTYPE) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Select resolution of conversions."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::CMDL7_MODE {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CMDL7_MODE::from_bits(val as u8)
    }
    #[doc = "Select resolution of conversions."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::CMDL7_MODE) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for CMDL7 {
    #[inline(always)]
    fn default() -> CMDL7 {
        CMDL7(0)
    }
}
impl core::fmt::Debug for CMDL7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDL7")
            .field("ADCH", &self.ADCH())
            .field("CTYPE", &self.CTYPE())
            .field("MODE", &self.MODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CMDL7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CMDL7 {{ ADCH: {:?}, CTYPE: {:?}, MODE: {:?} }}",
            self.ADCH(),
            self.CTYPE(),
            self.MODE()
        )
    }
}
#[doc = "ADC Command Low Buffer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CMDL8(pub u32);
impl CMDL8 {
    #[doc = "Input channel select."]
    #[must_use]
    #[inline(always)]
    pub const fn ADCH(&self) -> super::vals::CMDL8_ADCH {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::CMDL8_ADCH::from_bits(val as u8)
    }
    #[doc = "Input channel select."]
    #[inline(always)]
    pub const fn set_ADCH(&mut self, val: super::vals::CMDL8_ADCH) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Conversion Type."]
    #[must_use]
    #[inline(always)]
    pub const fn CTYPE(&self) -> super::vals::CMDL8_CTYPE {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::CMDL8_CTYPE::from_bits(val as u8)
    }
    #[doc = "Conversion Type."]
    #[inline(always)]
    pub const fn set_CTYPE(&mut self, val: super::vals::CMDL8_CTYPE) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Select resolution of conversions."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::CMDL8_MODE {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CMDL8_MODE::from_bits(val as u8)
    }
    #[doc = "Select resolution of conversions."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::CMDL8_MODE) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for CMDL8 {
    #[inline(always)]
    fn default() -> CMDL8 {
        CMDL8(0)
    }
}
impl core::fmt::Debug for CMDL8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDL8")
            .field("ADCH", &self.ADCH())
            .field("CTYPE", &self.CTYPE())
            .field("MODE", &self.MODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CMDL8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CMDL8 {{ ADCH: {:?}, CTYPE: {:?}, MODE: {:?} }}",
            self.ADCH(),
            self.CTYPE(),
            self.MODE()
        )
    }
}
#[doc = "ADC Command Low Buffer Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CMDL9(pub u32);
impl CMDL9 {
    #[doc = "Input channel select."]
    #[must_use]
    #[inline(always)]
    pub const fn ADCH(&self) -> super::vals::CMDL9_ADCH {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::CMDL9_ADCH::from_bits(val as u8)
    }
    #[doc = "Input channel select."]
    #[inline(always)]
    pub const fn set_ADCH(&mut self, val: super::vals::CMDL9_ADCH) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Conversion Type."]
    #[must_use]
    #[inline(always)]
    pub const fn CTYPE(&self) -> super::vals::CMDL9_CTYPE {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::CMDL9_CTYPE::from_bits(val as u8)
    }
    #[doc = "Conversion Type."]
    #[inline(always)]
    pub const fn set_CTYPE(&mut self, val: super::vals::CMDL9_CTYPE) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Select resolution of conversions."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::CMDL9_MODE {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CMDL9_MODE::from_bits(val as u8)
    }
    #[doc = "Select resolution of conversions."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::CMDL9_MODE) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for CMDL9 {
    #[inline(always)]
    fn default() -> CMDL9 {
        CMDL9(0)
    }
}
impl core::fmt::Debug for CMDL9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDL9")
            .field("ADCH", &self.ADCH())
            .field("CTYPE", &self.CTYPE())
            .field("MODE", &self.MODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CMDL9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CMDL9 {{ ADCH: {:?}, CTYPE: {:?}, MODE: {:?} }}",
            self.ADCH(),
            self.CTYPE(),
            self.MODE()
        )
    }
}
#[doc = "ADC Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CTRL(pub u32);
impl CTRL {
    #[doc = "ADC Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ADCEN(&self) -> super::vals::ADCEN {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ADCEN::from_bits(val as u8)
    }
    #[doc = "ADC Enable."]
    #[inline(always)]
    pub const fn set_ADCEN(&mut self, val: super::vals::ADCEN) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Software Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn RST(&self) -> super::vals::RST {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::RST::from_bits(val as u8)
    }
    #[doc = "Software Reset."]
    #[inline(always)]
    pub const fn set_RST(&mut self, val: super::vals::RST) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Doze Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn DOZEN(&self) -> super::vals::DOZEN {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::DOZEN::from_bits(val as u8)
    }
    #[doc = "Doze Enable."]
    #[inline(always)]
    pub const fn set_DOZEN(&mut self, val: super::vals::DOZEN) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Auto-Calibration Request."]
    #[must_use]
    #[inline(always)]
    pub const fn CAL_REQ(&self) -> super::vals::CAL_REQ {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::CAL_REQ::from_bits(val as u8)
    }
    #[doc = "Auto-Calibration Request."]
    #[inline(always)]
    pub const fn set_CAL_REQ(&mut self, val: super::vals::CAL_REQ) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Configure for offset calibration function."]
    #[must_use]
    #[inline(always)]
    pub const fn CALOFS(&self) -> super::vals::CALOFS {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::CALOFS::from_bits(val as u8)
    }
    #[doc = "Configure for offset calibration function."]
    #[inline(always)]
    pub const fn set_CALOFS(&mut self, val: super::vals::CALOFS) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Reset FIFO 0."]
    #[must_use]
    #[inline(always)]
    pub const fn RSTFIFO0(&self) -> super::vals::RSTFIFO0 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::RSTFIFO0::from_bits(val as u8)
    }
    #[doc = "Reset FIFO 0."]
    #[inline(always)]
    pub const fn set_RSTFIFO0(&mut self, val: super::vals::RSTFIFO0) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Reset FIFO 1."]
    #[must_use]
    #[inline(always)]
    pub const fn RSTFIFO1(&self) -> super::vals::RSTFIFO1 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::RSTFIFO1::from_bits(val as u8)
    }
    #[doc = "Reset FIFO 1."]
    #[inline(always)]
    pub const fn set_RSTFIFO1(&mut self, val: super::vals::RSTFIFO1) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Auto-Calibration Averages."]
    #[must_use]
    #[inline(always)]
    pub const fn CAL_AVGS(&self) -> super::vals::CAL_AVGS {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::CAL_AVGS::from_bits(val as u8)
    }
    #[doc = "Auto-Calibration Averages."]
    #[inline(always)]
    pub const fn set_CAL_AVGS(&mut self, val: super::vals::CAL_AVGS) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
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
            .field("ADCEN", &self.ADCEN())
            .field("RST", &self.RST())
            .field("DOZEN", &self.DOZEN())
            .field("CAL_REQ", &self.CAL_REQ())
            .field("CALOFS", &self.CALOFS())
            .field("RSTFIFO0", &self.RSTFIFO0())
            .field("RSTFIFO1", &self.RSTFIFO1())
            .field("CAL_AVGS", &self.CAL_AVGS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CTRL {{ ADCEN: {:?}, RST: {:?}, DOZEN: {:?}, CAL_REQ: {:?}, CALOFS: {:?}, RSTFIFO0: {:?}, RSTFIFO1: {:?}, CAL_AVGS: {:?} }}",
            self.ADCEN(),
            self.RST(),
            self.DOZEN(),
            self.CAL_REQ(),
            self.CALOFS(),
            self.RSTFIFO0(),
            self.RSTFIFO1(),
            self.CAL_AVGS()
        )
    }
}
#[doc = "Compare Value Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CV(pub u32);
impl CV {
    #[doc = "Compare Value Low."]
    #[must_use]
    #[inline(always)]
    pub const fn CVL(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Compare Value Low."]
    #[inline(always)]
    pub const fn set_CVL(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Compare Value High."]
    #[must_use]
    #[inline(always)]
    pub const fn CVH(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Compare Value High."]
    #[inline(always)]
    pub const fn set_CVH(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CV {
    #[inline(always)]
    fn default() -> CV {
        CV(0)
    }
}
impl core::fmt::Debug for CV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CV")
            .field("CVL", &self.CVL())
            .field("CVH", &self.CVH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CV {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CV {{ CVL: {=u16:?}, CVH: {=u16:?} }}",
            self.CVL(),
            self.CVH()
        )
    }
}
#[doc = "DMA Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DE(pub u32);
impl DE {
    #[doc = "FIFO 0 Watermark DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn FWMDE0(&self) -> super::vals::FWMDE0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::FWMDE0::from_bits(val as u8)
    }
    #[doc = "FIFO 0 Watermark DMA Enable."]
    #[inline(always)]
    pub const fn set_FWMDE0(&mut self, val: super::vals::FWMDE0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "FIFO1 Watermark DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn FWMDE1(&self) -> super::vals::FWMDE1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::FWMDE1::from_bits(val as u8)
    }
    #[doc = "FIFO1 Watermark DMA Enable."]
    #[inline(always)]
    pub const fn set_FWMDE1(&mut self, val: super::vals::FWMDE1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for DE {
    #[inline(always)]
    fn default() -> DE {
        DE(0)
    }
}
impl core::fmt::Debug for DE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DE")
            .field("FWMDE0", &self.FWMDE0())
            .field("FWMDE1", &self.FWMDE1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DE {{ FWMDE0: {:?}, FWMDE1: {:?} }}",
            self.FWMDE0(),
            self.FWMDE1()
        )
    }
}
#[doc = "FIFO Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FCTRL(pub u32);
impl FCTRL {
    #[doc = "Result FIFO counter."]
    #[must_use]
    #[inline(always)]
    pub const fn FCOUNT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Result FIFO counter."]
    #[inline(always)]
    pub const fn set_FCOUNT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Watermark level selection."]
    #[must_use]
    #[inline(always)]
    pub const fn FWMARK(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Watermark level selection."]
    #[inline(always)]
    pub const fn set_FWMARK(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for FCTRL {
    #[inline(always)]
    fn default() -> FCTRL {
        FCTRL(0)
    }
}
impl core::fmt::Debug for FCTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCTRL")
            .field("FCOUNT", &self.FCOUNT())
            .field("FWMARK", &self.FWMARK())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FCTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FCTRL {{ FCOUNT: {=u8:?}, FWMARK: {=u8:?} }}",
            self.FCOUNT(),
            self.FWMARK()
        )
    }
}
#[doc = "Gain Calibration Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GCC(pub u32);
impl GCC {
    #[doc = "Gain Calibration Value."]
    #[must_use]
    #[inline(always)]
    pub const fn GAIN_CAL(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Gain Calibration Value."]
    #[inline(always)]
    pub const fn set_GAIN_CAL(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Gain Calibration Value Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn RDY(&self) -> super::vals::GCC_RDY {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::GCC_RDY::from_bits(val as u8)
    }
    #[doc = "Gain Calibration Value Valid."]
    #[inline(always)]
    pub const fn set_RDY(&mut self, val: super::vals::GCC_RDY) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for GCC {
    #[inline(always)]
    fn default() -> GCC {
        GCC(0)
    }
}
impl core::fmt::Debug for GCC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GCC")
            .field("GAIN_CAL", &self.GAIN_CAL())
            .field("RDY", &self.RDY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GCC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GCC {{ GAIN_CAL: {=u16:?}, RDY: {:?} }}",
            self.GAIN_CAL(),
            self.RDY()
        )
    }
}
#[doc = "Gain Calculation Result."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GCR(pub u32);
impl GCR {
    #[doc = "Gain Calculation Result."]
    #[must_use]
    #[inline(always)]
    pub const fn GCALR(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Gain Calculation Result."]
    #[inline(always)]
    pub const fn set_GCALR(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Gain Calculation Ready."]
    #[must_use]
    #[inline(always)]
    pub const fn RDY(&self) -> super::vals::GCR_RDY {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::GCR_RDY::from_bits(val as u8)
    }
    #[doc = "Gain Calculation Ready."]
    #[inline(always)]
    pub const fn set_RDY(&mut self, val: super::vals::GCR_RDY) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for GCR {
    #[inline(always)]
    fn default() -> GCR {
        GCR(0)
    }
}
impl core::fmt::Debug for GCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GCR")
            .field("GCALR", &self.GCALR())
            .field("RDY", &self.RDY())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GCR {{ GCALR: {=u16:?}, RDY: {:?} }}",
            self.GCALR(),
            self.RDY()
        )
    }
}
#[doc = "Interrupt Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IE(pub u32);
impl IE {
    #[doc = "FIFO 0 Watermark Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn FWMIE0(&self) -> super::vals::FWMIE0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::FWMIE0::from_bits(val as u8)
    }
    #[doc = "FIFO 0 Watermark Interrupt Enable."]
    #[inline(always)]
    pub const fn set_FWMIE0(&mut self, val: super::vals::FWMIE0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Result FIFO 0 Overflow Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn FOFIE0(&self) -> super::vals::FOFIE0 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::FOFIE0::from_bits(val as u8)
    }
    #[doc = "Result FIFO 0 Overflow Interrupt Enable."]
    #[inline(always)]
    pub const fn set_FOFIE0(&mut self, val: super::vals::FOFIE0) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "FIFO1 Watermark Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn FWMIE1(&self) -> super::vals::FWMIE1 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::FWMIE1::from_bits(val as u8)
    }
    #[doc = "FIFO1 Watermark Interrupt Enable."]
    #[inline(always)]
    pub const fn set_FWMIE1(&mut self, val: super::vals::FWMIE1) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Result FIFO1 Overflow Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn FOFIE1(&self) -> super::vals::FOFIE1 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::FOFIE1::from_bits(val as u8)
    }
    #[doc = "Result FIFO1 Overflow Interrupt Enable."]
    #[inline(always)]
    pub const fn set_FOFIE1(&mut self, val: super::vals::FOFIE1) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Trigger Exception Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn TEXC_IE(&self) -> super::vals::TEXC_IE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::TEXC_IE::from_bits(val as u8)
    }
    #[doc = "Trigger Exception Interrupt Enable."]
    #[inline(always)]
    pub const fn set_TEXC_IE(&mut self, val: super::vals::TEXC_IE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Trigger Completion Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn TCOMP_IE(&self) -> super::vals::TCOMP_IE {
        let val = (self.0 >> 16usize) & 0xffff;
        super::vals::TCOMP_IE::from_bits(val as u16)
    }
    #[doc = "Trigger Completion Interrupt Enable."]
    #[inline(always)]
    pub const fn set_TCOMP_IE(&mut self, val: super::vals::TCOMP_IE) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val.to_bits() as u32) & 0xffff) << 16usize);
    }
}
impl Default for IE {
    #[inline(always)]
    fn default() -> IE {
        IE(0)
    }
}
impl core::fmt::Debug for IE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IE")
            .field("FWMIE0", &self.FWMIE0())
            .field("FOFIE0", &self.FOFIE0())
            .field("FWMIE1", &self.FWMIE1())
            .field("FOFIE1", &self.FOFIE1())
            .field("TEXC_IE", &self.TEXC_IE())
            .field("TCOMP_IE", &self.TCOMP_IE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IE {{ FWMIE0: {:?}, FOFIE0: {:?}, FWMIE1: {:?}, FOFIE1: {:?}, TEXC_IE: {:?}, TCOMP_IE: {:?} }}",
            self.FWMIE0(),
            self.FOFIE0(),
            self.FWMIE1(),
            self.FOFIE1(),
            self.TEXC_IE(),
            self.TCOMP_IE()
        )
    }
}
#[doc = "ADC Offset Trim Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OFSTRIM(pub u32);
impl OFSTRIM {
    #[doc = "Trim for offset."]
    #[must_use]
    #[inline(always)]
    pub const fn OFSTRIM_A(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Trim for offset."]
    #[inline(always)]
    pub const fn set_OFSTRIM_A(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Trim for offset."]
    #[must_use]
    #[inline(always)]
    pub const fn OFSTRIM_B(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Trim for offset."]
    #[inline(always)]
    pub const fn set_OFSTRIM_B(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for OFSTRIM {
    #[inline(always)]
    fn default() -> OFSTRIM {
        OFSTRIM(0)
    }
}
impl core::fmt::Debug for OFSTRIM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OFSTRIM")
            .field("OFSTRIM_A", &self.OFSTRIM_A())
            .field("OFSTRIM_B", &self.OFSTRIM_B())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OFSTRIM {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OFSTRIM {{ OFSTRIM_A: {=u8:?}, OFSTRIM_B: {=u8:?} }}",
            self.OFSTRIM_A(),
            self.OFSTRIM_B()
        )
    }
}
#[doc = "Parameter Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PARAM(pub u32);
impl PARAM {
    #[doc = "Trigger Number."]
    #[must_use]
    #[inline(always)]
    pub const fn TRIG_NUM(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Trigger Number."]
    #[inline(always)]
    pub const fn set_TRIG_NUM(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Result FIFO Depth."]
    #[must_use]
    #[inline(always)]
    pub const fn FIFOSIZE(&self) -> super::vals::FIFOSIZE {
        let val = (self.0 >> 8usize) & 0xff;
        super::vals::FIFOSIZE::from_bits(val as u8)
    }
    #[doc = "Result FIFO Depth."]
    #[inline(always)]
    pub const fn set_FIFOSIZE(&mut self, val: super::vals::FIFOSIZE) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u32) & 0xff) << 8usize);
    }
    #[doc = "Compare Value Number."]
    #[must_use]
    #[inline(always)]
    pub const fn CV_NUM(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Compare Value Number."]
    #[inline(always)]
    pub const fn set_CV_NUM(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Command Buffer Number."]
    #[must_use]
    #[inline(always)]
    pub const fn CMD_NUM(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Command Buffer Number."]
    #[inline(always)]
    pub const fn set_CMD_NUM(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for PARAM {
    #[inline(always)]
    fn default() -> PARAM {
        PARAM(0)
    }
}
impl core::fmt::Debug for PARAM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PARAM")
            .field("TRIG_NUM", &self.TRIG_NUM())
            .field("FIFOSIZE", &self.FIFOSIZE())
            .field("CV_NUM", &self.CV_NUM())
            .field("CMD_NUM", &self.CMD_NUM())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PARAM {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PARAM {{ TRIG_NUM: {=u8:?}, FIFOSIZE: {:?}, CV_NUM: {=u8:?}, CMD_NUM: {=u8:?} }}",
            self.TRIG_NUM(),
            self.FIFOSIZE(),
            self.CV_NUM(),
            self.CMD_NUM()
        )
    }
}
#[doc = "ADC Pause Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PAUSE(pub u32);
impl PAUSE {
    #[doc = "Pause Delay."]
    #[must_use]
    #[inline(always)]
    pub const fn PAUSEDLY(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Pause Delay."]
    #[inline(always)]
    pub const fn set_PAUSEDLY(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "PAUSE Option Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn PAUSEEN(&self) -> super::vals::PAUSEEN {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PAUSEEN::from_bits(val as u8)
    }
    #[doc = "PAUSE Option Enable."]
    #[inline(always)]
    pub const fn set_PAUSEEN(&mut self, val: super::vals::PAUSEEN) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PAUSE {
    #[inline(always)]
    fn default() -> PAUSE {
        PAUSE(0)
    }
}
impl core::fmt::Debug for PAUSE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAUSE")
            .field("PAUSEDLY", &self.PAUSEDLY())
            .field("PAUSEEN", &self.PAUSEEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PAUSE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PAUSE {{ PAUSEDLY: {=u16:?}, PAUSEEN: {:?} }}",
            self.PAUSEDLY(),
            self.PAUSEEN()
        )
    }
}
#[doc = "ADC Data Result FIFO Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RESFIFO(pub u32);
impl RESFIFO {
    #[doc = "Data result."]
    #[must_use]
    #[inline(always)]
    pub const fn D(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Data result."]
    #[inline(always)]
    pub const fn set_D(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Trigger Source."]
    #[must_use]
    #[inline(always)]
    pub const fn TSRC(&self) -> super::vals::TSRC {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::TSRC::from_bits(val as u8)
    }
    #[doc = "Trigger Source."]
    #[inline(always)]
    pub const fn set_TSRC(&mut self, val: super::vals::TSRC) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Loop count value."]
    #[must_use]
    #[inline(always)]
    pub const fn LOOPCNT(&self) -> super::vals::LOOPCNT {
        let val = (self.0 >> 20usize) & 0x0f;
        super::vals::LOOPCNT::from_bits(val as u8)
    }
    #[doc = "Loop count value."]
    #[inline(always)]
    pub const fn set_LOOPCNT(&mut self, val: super::vals::LOOPCNT) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
    #[doc = "Command Buffer Source."]
    #[must_use]
    #[inline(always)]
    pub const fn CMDSRC(&self) -> super::vals::CMDSRC {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::CMDSRC::from_bits(val as u8)
    }
    #[doc = "Command Buffer Source."]
    #[inline(always)]
    pub const fn set_CMDSRC(&mut self, val: super::vals::CMDSRC) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
    #[doc = "FIFO entry is valid."]
    #[must_use]
    #[inline(always)]
    pub const fn VALID(&self) -> super::vals::VALID {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::VALID::from_bits(val as u8)
    }
    #[doc = "FIFO entry is valid."]
    #[inline(always)]
    pub const fn set_VALID(&mut self, val: super::vals::VALID) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for RESFIFO {
    #[inline(always)]
    fn default() -> RESFIFO {
        RESFIFO(0)
    }
}
impl core::fmt::Debug for RESFIFO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESFIFO")
            .field("D", &self.D())
            .field("TSRC", &self.TSRC())
            .field("LOOPCNT", &self.LOOPCNT())
            .field("CMDSRC", &self.CMDSRC())
            .field("VALID", &self.VALID())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RESFIFO {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RESFIFO {{ D: {=u16:?}, TSRC: {:?}, LOOPCNT: {:?}, CMDSRC: {:?}, VALID: {:?} }}",
            self.D(),
            self.TSRC(),
            self.LOOPCNT(),
            self.CMDSRC(),
            self.VALID()
        )
    }
}
#[doc = "ADC Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct STAT(pub u32);
impl STAT {
    #[doc = "Result FIFO 0 Ready Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn RDY0(&self) -> super::vals::RDY0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RDY0::from_bits(val as u8)
    }
    #[doc = "Result FIFO 0 Ready Flag."]
    #[inline(always)]
    pub const fn set_RDY0(&mut self, val: super::vals::RDY0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Result FIFO 0 Overflow Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn FOF0(&self) -> super::vals::FOF0 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::FOF0::from_bits(val as u8)
    }
    #[doc = "Result FIFO 0 Overflow Flag."]
    #[inline(always)]
    pub const fn set_FOF0(&mut self, val: super::vals::FOF0) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Result FIFO1 Ready Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn RDY1(&self) -> super::vals::RDY1 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::RDY1::from_bits(val as u8)
    }
    #[doc = "Result FIFO1 Ready Flag."]
    #[inline(always)]
    pub const fn set_RDY1(&mut self, val: super::vals::RDY1) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Result FIFO1 Overflow Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn FOF1(&self) -> super::vals::FOF1 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::FOF1::from_bits(val as u8)
    }
    #[doc = "Result FIFO1 Overflow Flag."]
    #[inline(always)]
    pub const fn set_FOF1(&mut self, val: super::vals::FOF1) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Interrupt Flag For High Priority Trigger Exception."]
    #[must_use]
    #[inline(always)]
    pub const fn TEXC_INT(&self) -> super::vals::TEXC_INT {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::TEXC_INT::from_bits(val as u8)
    }
    #[doc = "Interrupt Flag For High Priority Trigger Exception."]
    #[inline(always)]
    pub const fn set_TEXC_INT(&mut self, val: super::vals::TEXC_INT) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt Flag For Trigger Completion."]
    #[must_use]
    #[inline(always)]
    pub const fn TCOMP_INT(&self) -> super::vals::TCOMP_INT {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::TCOMP_INT::from_bits(val as u8)
    }
    #[doc = "Interrupt Flag For Trigger Completion."]
    #[inline(always)]
    pub const fn set_TCOMP_INT(&mut self, val: super::vals::TCOMP_INT) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Calibration Ready."]
    #[must_use]
    #[inline(always)]
    pub const fn CAL_RDY(&self) -> super::vals::CAL_RDY {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::CAL_RDY::from_bits(val as u8)
    }
    #[doc = "Calibration Ready."]
    #[inline(always)]
    pub const fn set_CAL_RDY(&mut self, val: super::vals::CAL_RDY) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "ADC Active."]
    #[must_use]
    #[inline(always)]
    pub const fn ADC_ACTIVE(&self) -> super::vals::ADC_ACTIVE {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::ADC_ACTIVE::from_bits(val as u8)
    }
    #[doc = "ADC Active."]
    #[inline(always)]
    pub const fn set_ADC_ACTIVE(&mut self, val: super::vals::ADC_ACTIVE) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Trigger Active."]
    #[must_use]
    #[inline(always)]
    pub const fn TRGACT(&self) -> super::vals::TRGACT {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::TRGACT::from_bits(val as u8)
    }
    #[doc = "Trigger Active."]
    #[inline(always)]
    pub const fn set_TRGACT(&mut self, val: super::vals::TRGACT) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Command Active."]
    #[must_use]
    #[inline(always)]
    pub const fn CMDACT(&self) -> super::vals::CMDACT {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::CMDACT::from_bits(val as u8)
    }
    #[doc = "Command Active."]
    #[inline(always)]
    pub const fn set_CMDACT(&mut self, val: super::vals::CMDACT) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for STAT {
    #[inline(always)]
    fn default() -> STAT {
        STAT(0)
    }
}
impl core::fmt::Debug for STAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STAT")
            .field("RDY0", &self.RDY0())
            .field("FOF0", &self.FOF0())
            .field("RDY1", &self.RDY1())
            .field("FOF1", &self.FOF1())
            .field("TEXC_INT", &self.TEXC_INT())
            .field("TCOMP_INT", &self.TCOMP_INT())
            .field("CAL_RDY", &self.CAL_RDY())
            .field("ADC_ACTIVE", &self.ADC_ACTIVE())
            .field("TRGACT", &self.TRGACT())
            .field("CMDACT", &self.CMDACT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for STAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "STAT {{ RDY0: {:?}, FOF0: {:?}, RDY1: {:?}, FOF1: {:?}, TEXC_INT: {:?}, TCOMP_INT: {:?}, CAL_RDY: {:?}, ADC_ACTIVE: {:?}, TRGACT: {:?}, CMDACT: {:?} }}",
            self.RDY0(),
            self.FOF0(),
            self.RDY1(),
            self.FOF1(),
            self.TEXC_INT(),
            self.TCOMP_INT(),
            self.CAL_RDY(),
            self.ADC_ACTIVE(),
            self.TRGACT(),
            self.CMDACT()
        )
    }
}
#[doc = "Software Trigger Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SWTRIG(pub u32);
impl SWTRIG {
    #[doc = "Software trigger 0 event."]
    #[must_use]
    #[inline(always)]
    pub const fn SWT0(&self) -> super::vals::SWT0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SWT0::from_bits(val as u8)
    }
    #[doc = "Software trigger 0 event."]
    #[inline(always)]
    pub const fn set_SWT0(&mut self, val: super::vals::SWT0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Software trigger 1 event."]
    #[must_use]
    #[inline(always)]
    pub const fn SWT1(&self) -> super::vals::SWT1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SWT1::from_bits(val as u8)
    }
    #[doc = "Software trigger 1 event."]
    #[inline(always)]
    pub const fn set_SWT1(&mut self, val: super::vals::SWT1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Software trigger 2 event."]
    #[must_use]
    #[inline(always)]
    pub const fn SWT2(&self) -> super::vals::SWT2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SWT2::from_bits(val as u8)
    }
    #[doc = "Software trigger 2 event."]
    #[inline(always)]
    pub const fn set_SWT2(&mut self, val: super::vals::SWT2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Software trigger 3 event."]
    #[must_use]
    #[inline(always)]
    pub const fn SWT3(&self) -> super::vals::SWT3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SWT3::from_bits(val as u8)
    }
    #[doc = "Software trigger 3 event."]
    #[inline(always)]
    pub const fn set_SWT3(&mut self, val: super::vals::SWT3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Software trigger 4 event."]
    #[must_use]
    #[inline(always)]
    pub const fn SWT4(&self) -> super::vals::SWT4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SWT4::from_bits(val as u8)
    }
    #[doc = "Software trigger 4 event."]
    #[inline(always)]
    pub const fn set_SWT4(&mut self, val: super::vals::SWT4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Software trigger 5 event."]
    #[must_use]
    #[inline(always)]
    pub const fn SWT5(&self) -> super::vals::SWT5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::SWT5::from_bits(val as u8)
    }
    #[doc = "Software trigger 5 event."]
    #[inline(always)]
    pub const fn set_SWT5(&mut self, val: super::vals::SWT5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Software trigger 6 event."]
    #[must_use]
    #[inline(always)]
    pub const fn SWT6(&self) -> super::vals::SWT6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::SWT6::from_bits(val as u8)
    }
    #[doc = "Software trigger 6 event."]
    #[inline(always)]
    pub const fn set_SWT6(&mut self, val: super::vals::SWT6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Software trigger 7 event."]
    #[must_use]
    #[inline(always)]
    pub const fn SWT7(&self) -> super::vals::SWT7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::SWT7::from_bits(val as u8)
    }
    #[doc = "Software trigger 7 event."]
    #[inline(always)]
    pub const fn set_SWT7(&mut self, val: super::vals::SWT7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Software trigger 8 event."]
    #[must_use]
    #[inline(always)]
    pub const fn SWT8(&self) -> super::vals::SWT8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::SWT8::from_bits(val as u8)
    }
    #[doc = "Software trigger 8 event."]
    #[inline(always)]
    pub const fn set_SWT8(&mut self, val: super::vals::SWT8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Software trigger 9 event."]
    #[must_use]
    #[inline(always)]
    pub const fn SWT9(&self) -> super::vals::SWT9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::SWT9::from_bits(val as u8)
    }
    #[doc = "Software trigger 9 event."]
    #[inline(always)]
    pub const fn set_SWT9(&mut self, val: super::vals::SWT9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Software trigger 10 event."]
    #[must_use]
    #[inline(always)]
    pub const fn SWT10(&self) -> super::vals::SWT10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::SWT10::from_bits(val as u8)
    }
    #[doc = "Software trigger 10 event."]
    #[inline(always)]
    pub const fn set_SWT10(&mut self, val: super::vals::SWT10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Software trigger 11 event."]
    #[must_use]
    #[inline(always)]
    pub const fn SWT11(&self) -> super::vals::SWT11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::SWT11::from_bits(val as u8)
    }
    #[doc = "Software trigger 11 event."]
    #[inline(always)]
    pub const fn set_SWT11(&mut self, val: super::vals::SWT11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Software trigger 12 event."]
    #[must_use]
    #[inline(always)]
    pub const fn SWT12(&self) -> super::vals::SWT12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::SWT12::from_bits(val as u8)
    }
    #[doc = "Software trigger 12 event."]
    #[inline(always)]
    pub const fn set_SWT12(&mut self, val: super::vals::SWT12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Software trigger 13 event."]
    #[must_use]
    #[inline(always)]
    pub const fn SWT13(&self) -> super::vals::SWT13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::SWT13::from_bits(val as u8)
    }
    #[doc = "Software trigger 13 event."]
    #[inline(always)]
    pub const fn set_SWT13(&mut self, val: super::vals::SWT13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Software trigger 14 event."]
    #[must_use]
    #[inline(always)]
    pub const fn SWT14(&self) -> super::vals::SWT14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::SWT14::from_bits(val as u8)
    }
    #[doc = "Software trigger 14 event."]
    #[inline(always)]
    pub const fn set_SWT14(&mut self, val: super::vals::SWT14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Software trigger 15 event."]
    #[must_use]
    #[inline(always)]
    pub const fn SWT15(&self) -> super::vals::SWT15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::SWT15::from_bits(val as u8)
    }
    #[doc = "Software trigger 15 event."]
    #[inline(always)]
    pub const fn set_SWT15(&mut self, val: super::vals::SWT15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for SWTRIG {
    #[inline(always)]
    fn default() -> SWTRIG {
        SWTRIG(0)
    }
}
impl core::fmt::Debug for SWTRIG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWTRIG")
            .field("SWT0", &self.SWT0())
            .field("SWT1", &self.SWT1())
            .field("SWT2", &self.SWT2())
            .field("SWT3", &self.SWT3())
            .field("SWT4", &self.SWT4())
            .field("SWT5", &self.SWT5())
            .field("SWT6", &self.SWT6())
            .field("SWT7", &self.SWT7())
            .field("SWT8", &self.SWT8())
            .field("SWT9", &self.SWT9())
            .field("SWT10", &self.SWT10())
            .field("SWT11", &self.SWT11())
            .field("SWT12", &self.SWT12())
            .field("SWT13", &self.SWT13())
            .field("SWT14", &self.SWT14())
            .field("SWT15", &self.SWT15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SWTRIG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SWTRIG {{ SWT0: {:?}, SWT1: {:?}, SWT2: {:?}, SWT3: {:?}, SWT4: {:?}, SWT5: {:?}, SWT6: {:?}, SWT7: {:?}, SWT8: {:?}, SWT9: {:?}, SWT10: {:?}, SWT11: {:?}, SWT12: {:?}, SWT13: {:?}, SWT14: {:?}, SWT15: {:?} }}",
            self.SWT0(),
            self.SWT1(),
            self.SWT2(),
            self.SWT3(),
            self.SWT4(),
            self.SWT5(),
            self.SWT6(),
            self.SWT7(),
            self.SWT8(),
            self.SWT9(),
            self.SWT10(),
            self.SWT11(),
            self.SWT12(),
            self.SWT13(),
            self.SWT14(),
            self.SWT15()
        )
    }
}
#[doc = "Trigger Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TCTRL(pub u32);
impl TCTRL {
    #[doc = "Trigger enable."]
    #[must_use]
    #[inline(always)]
    pub const fn HTEN(&self) -> super::vals::HTEN {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::HTEN::from_bits(val as u8)
    }
    #[doc = "Trigger enable."]
    #[inline(always)]
    pub const fn set_HTEN(&mut self, val: super::vals::HTEN) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "SAR Result Destination For Channel A."]
    #[must_use]
    #[inline(always)]
    pub const fn FIFO_SEL_A(&self) -> super::vals::FIFO_SEL_A {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::FIFO_SEL_A::from_bits(val as u8)
    }
    #[doc = "SAR Result Destination For Channel A."]
    #[inline(always)]
    pub const fn set_FIFO_SEL_A(&mut self, val: super::vals::FIFO_SEL_A) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "SAR Result Destination For Channel B."]
    #[must_use]
    #[inline(always)]
    pub const fn FIFO_SEL_B(&self) -> super::vals::FIFO_SEL_B {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::FIFO_SEL_B::from_bits(val as u8)
    }
    #[doc = "SAR Result Destination For Channel B."]
    #[inline(always)]
    pub const fn set_FIFO_SEL_B(&mut self, val: super::vals::FIFO_SEL_B) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Trigger priority setting."]
    #[must_use]
    #[inline(always)]
    pub const fn TPRI(&self) -> super::vals::TPRI {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::TPRI::from_bits(val as u8)
    }
    #[doc = "Trigger priority setting."]
    #[inline(always)]
    pub const fn set_TPRI(&mut self, val: super::vals::TPRI) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Trigger Resync."]
    #[must_use]
    #[inline(always)]
    pub const fn RSYNC(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger Resync."]
    #[inline(always)]
    pub const fn set_RSYNC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Trigger delay select."]
    #[must_use]
    #[inline(always)]
    pub const fn TDLY(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Trigger delay select."]
    #[inline(always)]
    pub const fn set_TDLY(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Trigger command select."]
    #[must_use]
    #[inline(always)]
    pub const fn TCMD(&self) -> super::vals::TCMD {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::TCMD::from_bits(val as u8)
    }
    #[doc = "Trigger command select."]
    #[inline(always)]
    pub const fn set_TCMD(&mut self, val: super::vals::TCMD) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for TCTRL {
    #[inline(always)]
    fn default() -> TCTRL {
        TCTRL(0)
    }
}
impl core::fmt::Debug for TCTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCTRL")
            .field("HTEN", &self.HTEN())
            .field("FIFO_SEL_A", &self.FIFO_SEL_A())
            .field("FIFO_SEL_B", &self.FIFO_SEL_B())
            .field("TPRI", &self.TPRI())
            .field("RSYNC", &self.RSYNC())
            .field("TDLY", &self.TDLY())
            .field("TCMD", &self.TCMD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TCTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TCTRL {{ HTEN: {:?}, FIFO_SEL_A: {:?}, FIFO_SEL_B: {:?}, TPRI: {:?}, RSYNC: {=bool:?}, TDLY: {=u8:?}, TCMD: {:?} }}",
            self.HTEN(),
            self.FIFO_SEL_A(),
            self.FIFO_SEL_B(),
            self.TPRI(),
            self.RSYNC(),
            self.TDLY(),
            self.TCMD()
        )
    }
}
#[doc = "ADC Test Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TST(pub u32);
impl TST {
    #[doc = "Calibration Sample Time Long."]
    #[must_use]
    #[inline(always)]
    pub const fn CST_LONG(&self) -> super::vals::CST_LONG {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::CST_LONG::from_bits(val as u8)
    }
    #[doc = "Calibration Sample Time Long."]
    #[inline(always)]
    pub const fn set_CST_LONG(&mut self, val: super::vals::CST_LONG) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Force M-side positive offset."]
    #[must_use]
    #[inline(always)]
    pub const fn FOFFM(&self) -> super::vals::FOFFM {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::FOFFM::from_bits(val as u8)
    }
    #[doc = "Force M-side positive offset."]
    #[inline(always)]
    pub const fn set_FOFFM(&mut self, val: super::vals::FOFFM) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Force P-side positive offset."]
    #[must_use]
    #[inline(always)]
    pub const fn FOFFP(&self) -> super::vals::FOFFP {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::FOFFP::from_bits(val as u8)
    }
    #[doc = "Force P-side positive offset."]
    #[inline(always)]
    pub const fn set_FOFFP(&mut self, val: super::vals::FOFFP) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Force M-side negative offset."]
    #[must_use]
    #[inline(always)]
    pub const fn FOFFM2(&self) -> super::vals::FOFFM2 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::FOFFM2::from_bits(val as u8)
    }
    #[doc = "Force M-side negative offset."]
    #[inline(always)]
    pub const fn set_FOFFM2(&mut self, val: super::vals::FOFFM2) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Force P-side negative offset."]
    #[must_use]
    #[inline(always)]
    pub const fn FOFFP2(&self) -> super::vals::FOFFP2 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::FOFFP2::from_bits(val as u8)
    }
    #[doc = "Force P-side negative offset."]
    #[inline(always)]
    pub const fn set_FOFFP2(&mut self, val: super::vals::FOFFP2) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Enable test configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn TESTEN(&self) -> super::vals::TESTEN {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::TESTEN::from_bits(val as u8)
    }
    #[doc = "Enable test configuration."]
    #[inline(always)]
    pub const fn set_TESTEN(&mut self, val: super::vals::TESTEN) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
}
impl Default for TST {
    #[inline(always)]
    fn default() -> TST {
        TST(0)
    }
}
impl core::fmt::Debug for TST {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TST")
            .field("CST_LONG", &self.CST_LONG())
            .field("FOFFM", &self.FOFFM())
            .field("FOFFP", &self.FOFFP())
            .field("FOFFM2", &self.FOFFM2())
            .field("FOFFP2", &self.FOFFP2())
            .field("TESTEN", &self.TESTEN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TST {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TST {{ CST_LONG: {:?}, FOFFM: {:?}, FOFFP: {:?}, FOFFM2: {:?}, FOFFP2: {:?}, TESTEN: {:?} }}",
            self.CST_LONG(),
            self.FOFFM(),
            self.FOFFP(),
            self.FOFFM2(),
            self.FOFFP2(),
            self.TESTEN()
        )
    }
}
#[doc = "Trigger Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TSTAT(pub u32);
impl TSTAT {
    #[doc = "Trigger Exception Number."]
    #[must_use]
    #[inline(always)]
    pub const fn TEXC_NUM(&self) -> super::vals::TEXC_NUM {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::TEXC_NUM::from_bits(val as u16)
    }
    #[doc = "Trigger Exception Number."]
    #[inline(always)]
    pub const fn set_TEXC_NUM(&mut self, val: super::vals::TEXC_NUM) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
    #[doc = "Trigger Completion Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn TCOMP_FLAG(&self) -> super::vals::TCOMP_FLAG {
        let val = (self.0 >> 16usize) & 0xffff;
        super::vals::TCOMP_FLAG::from_bits(val as u16)
    }
    #[doc = "Trigger Completion Flag."]
    #[inline(always)]
    pub const fn set_TCOMP_FLAG(&mut self, val: super::vals::TCOMP_FLAG) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val.to_bits() as u32) & 0xffff) << 16usize);
    }
}
impl Default for TSTAT {
    #[inline(always)]
    fn default() -> TSTAT {
        TSTAT(0)
    }
}
impl core::fmt::Debug for TSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSTAT")
            .field("TEXC_NUM", &self.TEXC_NUM())
            .field("TCOMP_FLAG", &self.TCOMP_FLAG())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TSTAT {{ TEXC_NUM: {:?}, TCOMP_FLAG: {:?} }}",
            self.TEXC_NUM(),
            self.TCOMP_FLAG()
        )
    }
}
#[doc = "Version ID Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VERID(pub u32);
impl VERID {
    #[doc = "Resolution."]
    #[must_use]
    #[inline(always)]
    pub const fn RES(&self) -> super::vals::RES {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RES::from_bits(val as u8)
    }
    #[doc = "Resolution."]
    #[inline(always)]
    pub const fn set_RES(&mut self, val: super::vals::RES) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Differential Supported."]
    #[must_use]
    #[inline(always)]
    pub const fn DIFFEN(&self) -> super::vals::DIFFEN {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::DIFFEN::from_bits(val as u8)
    }
    #[doc = "Differential Supported."]
    #[inline(always)]
    pub const fn set_DIFFEN(&mut self, val: super::vals::DIFFEN) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Multi Vref Implemented."]
    #[must_use]
    #[inline(always)]
    pub const fn MVI(&self) -> super::vals::MVI {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::MVI::from_bits(val as u8)
    }
    #[doc = "Multi Vref Implemented."]
    #[inline(always)]
    pub const fn set_MVI(&mut self, val: super::vals::MVI) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Channel Scale Width."]
    #[must_use]
    #[inline(always)]
    pub const fn CSW(&self) -> super::vals::CSW {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::CSW::from_bits(val as u8)
    }
    #[doc = "Channel Scale Width."]
    #[inline(always)]
    pub const fn set_CSW(&mut self, val: super::vals::CSW) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Voltage Reference 1 Range Control Bit Implemented."]
    #[must_use]
    #[inline(always)]
    pub const fn VR1RNGI(&self) -> super::vals::VR1RNGI {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::VR1RNGI::from_bits(val as u8)
    }
    #[doc = "Voltage Reference 1 Range Control Bit Implemented."]
    #[inline(always)]
    pub const fn set_VR1RNGI(&mut self, val: super::vals::VR1RNGI) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Internal ADC Clock implemented."]
    #[must_use]
    #[inline(always)]
    pub const fn IADCKI(&self) -> super::vals::IADCKI {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::IADCKI::from_bits(val as u8)
    }
    #[doc = "Internal ADC Clock implemented."]
    #[inline(always)]
    pub const fn set_IADCKI(&mut self, val: super::vals::IADCKI) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Calibration Function Implemented."]
    #[must_use]
    #[inline(always)]
    pub const fn CALOFSI(&self) -> super::vals::CALOFSI {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::CALOFSI::from_bits(val as u8)
    }
    #[doc = "Calibration Function Implemented."]
    #[inline(always)]
    pub const fn set_CALOFSI(&mut self, val: super::vals::CALOFSI) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Number of Single Ended Outputs Supported."]
    #[must_use]
    #[inline(always)]
    pub const fn NUM_SEC(&self) -> super::vals::NUM_SEC {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::NUM_SEC::from_bits(val as u8)
    }
    #[doc = "Number of Single Ended Outputs Supported."]
    #[inline(always)]
    pub const fn set_NUM_SEC(&mut self, val: super::vals::NUM_SEC) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Number of FIFOs."]
    #[must_use]
    #[inline(always)]
    pub const fn NUM_FIFO(&self) -> super::vals::NUM_FIFO {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::NUM_FIFO::from_bits(val as u8)
    }
    #[doc = "Number of FIFOs."]
    #[inline(always)]
    pub const fn set_NUM_FIFO(&mut self, val: super::vals::NUM_FIFO) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Minor Version Number."]
    #[must_use]
    #[inline(always)]
    pub const fn MINOR(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Minor Version Number."]
    #[inline(always)]
    pub const fn set_MINOR(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Major Version Number."]
    #[must_use]
    #[inline(always)]
    pub const fn MAJOR(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Major Version Number."]
    #[inline(always)]
    pub const fn set_MAJOR(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for VERID {
    #[inline(always)]
    fn default() -> VERID {
        VERID(0)
    }
}
impl core::fmt::Debug for VERID {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VERID")
            .field("RES", &self.RES())
            .field("DIFFEN", &self.DIFFEN())
            .field("MVI", &self.MVI())
            .field("CSW", &self.CSW())
            .field("VR1RNGI", &self.VR1RNGI())
            .field("IADCKI", &self.IADCKI())
            .field("CALOFSI", &self.CALOFSI())
            .field("NUM_SEC", &self.NUM_SEC())
            .field("NUM_FIFO", &self.NUM_FIFO())
            .field("MINOR", &self.MINOR())
            .field("MAJOR", &self.MAJOR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VERID {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VERID {{ RES: {:?}, DIFFEN: {:?}, MVI: {:?}, CSW: {:?}, VR1RNGI: {:?}, IADCKI: {:?}, CALOFSI: {:?}, NUM_SEC: {:?}, NUM_FIFO: {:?}, MINOR: {=u8:?}, MAJOR: {=u8:?} }}",
            self.RES(),
            self.DIFFEN(),
            self.MVI(),
            self.CSW(),
            self.VR1RNGI(),
            self.IADCKI(),
            self.CALOFSI(),
            self.NUM_SEC(),
            self.NUM_FIFO(),
            self.MINOR(),
            self.MAJOR()
        )
    }
}
