#[doc = "SW_PAD_CTL_PAD_GPIO_00 SW PAD Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctl(pub u32);
impl Ctl {
    #[doc = "Slew Rate Field"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Slew Rate Field"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Drive Strength Field"]
    #[must_use]
    #[inline(always)]
    pub const fn dse(&self) -> super::vals::Dse {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::Dse::from_bits(val as u8)
    }
    #[doc = "Drive Strength Field"]
    #[inline(always)]
    pub const fn set_dse(&mut self, val: super::vals::Dse) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
    }
    #[doc = "Speed Field"]
    #[must_use]
    #[inline(always)]
    pub const fn speed(&self) -> super::vals::Speed {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Speed::from_bits(val as u8)
    }
    #[doc = "Speed Field"]
    #[inline(always)]
    pub const fn set_speed(&mut self, val: super::vals::Speed) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Open Drain Enable Field"]
    #[must_use]
    #[inline(always)]
    pub const fn ode(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Open Drain Enable Field"]
    #[inline(always)]
    pub const fn set_ode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Pull / Keep Enable Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pke(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Pull / Keep Enable Field"]
    #[inline(always)]
    pub const fn set_pke(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Pull / Keep Select Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Pull / Keep Select Field"]
    #[inline(always)]
    pub const fn set_pue(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[must_use]
    #[inline(always)]
    pub const fn pus(&self) -> super::vals::Pus {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Pus::from_bits(val as u8)
    }
    #[doc = "Pull Up / Down Config. Field"]
    #[inline(always)]
    pub const fn set_pus(&mut self, val: super::vals::Pus) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Hyst. Enable Field"]
    #[must_use]
    #[inline(always)]
    pub const fn hys(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Hyst. Enable Field"]
    #[inline(always)]
    pub const fn set_hys(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Ctl {
    #[inline(always)]
    fn default() -> Ctl {
        Ctl(0)
    }
}
impl core::fmt::Debug for Ctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctl")
            .field("sre", &self.sre())
            .field("dse", &self.dse())
            .field("speed", &self.speed())
            .field("ode", &self.ode())
            .field("pke", &self.pke())
            .field("pue", &self.pue())
            .field("pus", &self.pus())
            .field("hys", &self.hys())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctl {{ sre: {=bool:?}, dse: {:?}, speed: {:?}, ode: {=bool:?}, pke: {=bool:?}, pue: {=bool:?}, pus: {:?}, hys: {=bool:?} }}",
            self.sre(),
            self.dse(),
            self.speed(),
            self.ode(),
            self.pke(),
            self.pue(),
            self.pus(),
            self.hys()
        )
    }
}
#[doc = "FLEXPWM1_PWMA_SELECT_INPUT_0 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm1PwmaSelectInput0(pub u32);
impl Flexpwm1PwmaSelectInput0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm1PwmaSelectInput0Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexpwm1PwmaSelectInput0Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm1PwmaSelectInput0Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm1PwmaSelectInput0 {
    #[inline(always)]
    fn default() -> Flexpwm1PwmaSelectInput0 {
        Flexpwm1PwmaSelectInput0(0)
    }
}
impl core::fmt::Debug for Flexpwm1PwmaSelectInput0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm1PwmaSelectInput0")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm1PwmaSelectInput0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexpwm1PwmaSelectInput0 {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "FLEXPWM1_PWMA_SELECT_INPUT_1 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm1PwmaSelectInput1(pub u32);
impl Flexpwm1PwmaSelectInput1 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm1PwmaSelectInput1Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexpwm1PwmaSelectInput1Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm1PwmaSelectInput1Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm1PwmaSelectInput1 {
    #[inline(always)]
    fn default() -> Flexpwm1PwmaSelectInput1 {
        Flexpwm1PwmaSelectInput1(0)
    }
}
impl core::fmt::Debug for Flexpwm1PwmaSelectInput1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm1PwmaSelectInput1")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm1PwmaSelectInput1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexpwm1PwmaSelectInput1 {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "FLEXPWM1_PWMA_SELECT_INPUT_2 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm1PwmaSelectInput2(pub u32);
impl Flexpwm1PwmaSelectInput2 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm1PwmaSelectInput2Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexpwm1PwmaSelectInput2Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm1PwmaSelectInput2Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm1PwmaSelectInput2 {
    #[inline(always)]
    fn default() -> Flexpwm1PwmaSelectInput2 {
        Flexpwm1PwmaSelectInput2(0)
    }
}
impl core::fmt::Debug for Flexpwm1PwmaSelectInput2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm1PwmaSelectInput2")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm1PwmaSelectInput2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexpwm1PwmaSelectInput2 {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "FLEXPWM1_PWMA_SELECT_INPUT_3 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm1PwmaSelectInput3(pub u32);
impl Flexpwm1PwmaSelectInput3 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm1PwmaSelectInput3Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexpwm1PwmaSelectInput3Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm1PwmaSelectInput3Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm1PwmaSelectInput3 {
    #[inline(always)]
    fn default() -> Flexpwm1PwmaSelectInput3 {
        Flexpwm1PwmaSelectInput3(0)
    }
}
impl core::fmt::Debug for Flexpwm1PwmaSelectInput3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm1PwmaSelectInput3")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm1PwmaSelectInput3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexpwm1PwmaSelectInput3 {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "FLEXPWM1_PWMB_SELECT_INPUT_0 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm1PwmbSelectInput0(pub u32);
impl Flexpwm1PwmbSelectInput0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm1PwmbSelectInput0Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexpwm1PwmbSelectInput0Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm1PwmbSelectInput0Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm1PwmbSelectInput0 {
    #[inline(always)]
    fn default() -> Flexpwm1PwmbSelectInput0 {
        Flexpwm1PwmbSelectInput0(0)
    }
}
impl core::fmt::Debug for Flexpwm1PwmbSelectInput0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm1PwmbSelectInput0")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm1PwmbSelectInput0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexpwm1PwmbSelectInput0 {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "FLEXPWM1_PWMB_SELECT_INPUT_1 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm1PwmbSelectInput1(pub u32);
impl Flexpwm1PwmbSelectInput1 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm1PwmbSelectInput1Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexpwm1PwmbSelectInput1Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm1PwmbSelectInput1Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm1PwmbSelectInput1 {
    #[inline(always)]
    fn default() -> Flexpwm1PwmbSelectInput1 {
        Flexpwm1PwmbSelectInput1(0)
    }
}
impl core::fmt::Debug for Flexpwm1PwmbSelectInput1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm1PwmbSelectInput1")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm1PwmbSelectInput1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexpwm1PwmbSelectInput1 {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "FLEXPWM1_PWMB_SELECT_INPUT_2 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm1PwmbSelectInput2(pub u32);
impl Flexpwm1PwmbSelectInput2 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm1PwmbSelectInput2Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexpwm1PwmbSelectInput2Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm1PwmbSelectInput2Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm1PwmbSelectInput2 {
    #[inline(always)]
    fn default() -> Flexpwm1PwmbSelectInput2 {
        Flexpwm1PwmbSelectInput2(0)
    }
}
impl core::fmt::Debug for Flexpwm1PwmbSelectInput2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm1PwmbSelectInput2")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm1PwmbSelectInput2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexpwm1PwmbSelectInput2 {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "FLEXPWM1_PWMB_SELECT_INPUT_3 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexpwm1PwmbSelectInput3(pub u32);
impl Flexpwm1PwmbSelectInput3 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Flexpwm1PwmbSelectInput3Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexpwm1PwmbSelectInput3Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Flexpwm1PwmbSelectInput3Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Flexpwm1PwmbSelectInput3 {
    #[inline(always)]
    fn default() -> Flexpwm1PwmbSelectInput3 {
        Flexpwm1PwmbSelectInput3(0)
    }
}
impl core::fmt::Debug for Flexpwm1PwmbSelectInput3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexpwm1PwmbSelectInput3")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexpwm1PwmbSelectInput3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexpwm1PwmbSelectInput3 {{ daisy: {:?} }}",
            self.daisy()
        )
    }
}
#[doc = "FLEXSPI_DQS_FA_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexspiDqsFaSelectInput(pub u32);
impl FlexspiDqsFaSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::FlexspiDqsFaSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::FlexspiDqsFaSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::FlexspiDqsFaSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for FlexspiDqsFaSelectInput {
    #[inline(always)]
    fn default() -> FlexspiDqsFaSelectInput {
        FlexspiDqsFaSelectInput(0)
    }
}
impl core::fmt::Debug for FlexspiDqsFaSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexspiDqsFaSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexspiDqsFaSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FlexspiDqsFaSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "FLEXSPI_DQS_FB_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexspiDqsFbSelectInput(pub u32);
impl FlexspiDqsFbSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::FlexspiDqsFbSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::FlexspiDqsFbSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::FlexspiDqsFbSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for FlexspiDqsFbSelectInput {
    #[inline(always)]
    fn default() -> FlexspiDqsFbSelectInput {
        FlexspiDqsFbSelectInput(0)
    }
}
impl core::fmt::Debug for FlexspiDqsFbSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexspiDqsFbSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexspiDqsFbSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FlexspiDqsFbSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "KPP_COL_SELECT_INPUT_0 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KppColSelectInput0(pub u32);
impl KppColSelectInput0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::KppColSelectInput0Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::KppColSelectInput0Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::KppColSelectInput0Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for KppColSelectInput0 {
    #[inline(always)]
    fn default() -> KppColSelectInput0 {
        KppColSelectInput0(0)
    }
}
impl core::fmt::Debug for KppColSelectInput0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KppColSelectInput0")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KppColSelectInput0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "KppColSelectInput0 {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "KPP_COL_SELECT_INPUT_1 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KppColSelectInput1(pub u32);
impl KppColSelectInput1 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::KppColSelectInput1Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::KppColSelectInput1Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::KppColSelectInput1Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for KppColSelectInput1 {
    #[inline(always)]
    fn default() -> KppColSelectInput1 {
        KppColSelectInput1(0)
    }
}
impl core::fmt::Debug for KppColSelectInput1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KppColSelectInput1")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KppColSelectInput1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "KppColSelectInput1 {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "KPP_COL_SELECT_INPUT_2 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KppColSelectInput2(pub u32);
impl KppColSelectInput2 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::KppColSelectInput2Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::KppColSelectInput2Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::KppColSelectInput2Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for KppColSelectInput2 {
    #[inline(always)]
    fn default() -> KppColSelectInput2 {
        KppColSelectInput2(0)
    }
}
impl core::fmt::Debug for KppColSelectInput2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KppColSelectInput2")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KppColSelectInput2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "KppColSelectInput2 {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "KPP_COL_SELECT_INPUT_3 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KppColSelectInput3(pub u32);
impl KppColSelectInput3 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::KppColSelectInput3Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::KppColSelectInput3Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::KppColSelectInput3Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for KppColSelectInput3 {
    #[inline(always)]
    fn default() -> KppColSelectInput3 {
        KppColSelectInput3(0)
    }
}
impl core::fmt::Debug for KppColSelectInput3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KppColSelectInput3")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KppColSelectInput3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "KppColSelectInput3 {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "KPP_ROW_SELECT_INPUT_0 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KppRowSelectInput0(pub u32);
impl KppRowSelectInput0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::KppRowSelectInput0Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::KppRowSelectInput0Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::KppRowSelectInput0Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for KppRowSelectInput0 {
    #[inline(always)]
    fn default() -> KppRowSelectInput0 {
        KppRowSelectInput0(0)
    }
}
impl core::fmt::Debug for KppRowSelectInput0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KppRowSelectInput0")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KppRowSelectInput0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "KppRowSelectInput0 {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "KPP_ROW_SELECT_INPUT_1 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KppRowSelectInput1(pub u32);
impl KppRowSelectInput1 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::KppRowSelectInput1Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::KppRowSelectInput1Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::KppRowSelectInput1Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for KppRowSelectInput1 {
    #[inline(always)]
    fn default() -> KppRowSelectInput1 {
        KppRowSelectInput1(0)
    }
}
impl core::fmt::Debug for KppRowSelectInput1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KppRowSelectInput1")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KppRowSelectInput1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "KppRowSelectInput1 {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "KPP_ROW_SELECT_INPUT_2 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KppRowSelectInput2(pub u32);
impl KppRowSelectInput2 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::KppRowSelectInput2Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::KppRowSelectInput2Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::KppRowSelectInput2Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for KppRowSelectInput2 {
    #[inline(always)]
    fn default() -> KppRowSelectInput2 {
        KppRowSelectInput2(0)
    }
}
impl core::fmt::Debug for KppRowSelectInput2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KppRowSelectInput2")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KppRowSelectInput2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "KppRowSelectInput2 {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "KPP_ROW_SELECT_INPUT_3 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KppRowSelectInput3(pub u32);
impl KppRowSelectInput3 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::KppRowSelectInput3Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::KppRowSelectInput3Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::KppRowSelectInput3Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for KppRowSelectInput3 {
    #[inline(always)]
    fn default() -> KppRowSelectInput3 {
        KppRowSelectInput3(0)
    }
}
impl core::fmt::Debug for KppRowSelectInput3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KppRowSelectInput3")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KppRowSelectInput3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "KppRowSelectInput3 {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPI2C1_HREQ_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpi2c1HreqSelectInput(pub u32);
impl Lpi2c1HreqSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpi2c1HreqSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpi2c1HreqSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpi2c1HreqSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpi2c1HreqSelectInput {
    #[inline(always)]
    fn default() -> Lpi2c1HreqSelectInput {
        Lpi2c1HreqSelectInput(0)
    }
}
impl core::fmt::Debug for Lpi2c1HreqSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpi2c1HreqSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpi2c1HreqSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpi2c1HreqSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPI2C1_SCL_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpi2c1SclSelectInput(pub u32);
impl Lpi2c1SclSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpi2c1SclSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpi2c1SclSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpi2c1SclSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpi2c1SclSelectInput {
    #[inline(always)]
    fn default() -> Lpi2c1SclSelectInput {
        Lpi2c1SclSelectInput(0)
    }
}
impl core::fmt::Debug for Lpi2c1SclSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpi2c1SclSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpi2c1SclSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpi2c1SclSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPI2C1_SDA_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpi2c1SdaSelectInput(pub u32);
impl Lpi2c1SdaSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpi2c1SdaSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpi2c1SdaSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpi2c1SdaSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpi2c1SdaSelectInput {
    #[inline(always)]
    fn default() -> Lpi2c1SdaSelectInput {
        Lpi2c1SdaSelectInput(0)
    }
}
impl core::fmt::Debug for Lpi2c1SdaSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpi2c1SdaSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpi2c1SdaSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpi2c1SdaSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPI2C2_SCL_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpi2c2SclSelectInput(pub u32);
impl Lpi2c2SclSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpi2c2SclSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpi2c2SclSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpi2c2SclSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpi2c2SclSelectInput {
    #[inline(always)]
    fn default() -> Lpi2c2SclSelectInput {
        Lpi2c2SclSelectInput(0)
    }
}
impl core::fmt::Debug for Lpi2c2SclSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpi2c2SclSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpi2c2SclSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpi2c2SclSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPI2C2_SDA_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpi2c2SdaSelectInput(pub u32);
impl Lpi2c2SdaSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpi2c2SdaSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpi2c2SdaSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpi2c2SdaSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpi2c2SdaSelectInput {
    #[inline(always)]
    fn default() -> Lpi2c2SdaSelectInput {
        Lpi2c2SdaSelectInput(0)
    }
}
impl core::fmt::Debug for Lpi2c2SdaSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpi2c2SdaSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpi2c2SdaSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpi2c2SdaSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPSPI1_PCS_SELECT_INPUT_0 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi1PcsSelectInput0(pub u32);
impl Lpspi1PcsSelectInput0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi1PcsSelectInput0Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpspi1PcsSelectInput0Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi1PcsSelectInput0Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi1PcsSelectInput0 {
    #[inline(always)]
    fn default() -> Lpspi1PcsSelectInput0 {
        Lpspi1PcsSelectInput0(0)
    }
}
impl core::fmt::Debug for Lpspi1PcsSelectInput0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi1PcsSelectInput0")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi1PcsSelectInput0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpspi1PcsSelectInput0 {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPSPI1_SCK_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi1SckSelectInput(pub u32);
impl Lpspi1SckSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi1SckSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpspi1SckSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi1SckSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi1SckSelectInput {
    #[inline(always)]
    fn default() -> Lpspi1SckSelectInput {
        Lpspi1SckSelectInput(0)
    }
}
impl core::fmt::Debug for Lpspi1SckSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi1SckSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi1SckSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpspi1SckSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPSPI1_SDI_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi1SdiSelectInput(pub u32);
impl Lpspi1SdiSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi1SdiSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpspi1SdiSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi1SdiSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi1SdiSelectInput {
    #[inline(always)]
    fn default() -> Lpspi1SdiSelectInput {
        Lpspi1SdiSelectInput(0)
    }
}
impl core::fmt::Debug for Lpspi1SdiSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi1SdiSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi1SdiSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpspi1SdiSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPSPI1_SDO_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi1SdoSelectInput(pub u32);
impl Lpspi1SdoSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi1SdoSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpspi1SdoSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi1SdoSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi1SdoSelectInput {
    #[inline(always)]
    fn default() -> Lpspi1SdoSelectInput {
        Lpspi1SdoSelectInput(0)
    }
}
impl core::fmt::Debug for Lpspi1SdoSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi1SdoSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi1SdoSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpspi1SdoSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPSPI2_PCS_SELECT_INPUT_0 DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi2PcsSelectInput0(pub u32);
impl Lpspi2PcsSelectInput0 {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi2PcsSelectInput0Daisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpspi2PcsSelectInput0Daisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi2PcsSelectInput0Daisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi2PcsSelectInput0 {
    #[inline(always)]
    fn default() -> Lpspi2PcsSelectInput0 {
        Lpspi2PcsSelectInput0(0)
    }
}
impl core::fmt::Debug for Lpspi2PcsSelectInput0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi2PcsSelectInput0")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi2PcsSelectInput0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpspi2PcsSelectInput0 {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPSPI2_SCK_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi2SckSelectInput(pub u32);
impl Lpspi2SckSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi2SckSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpspi2SckSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi2SckSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi2SckSelectInput {
    #[inline(always)]
    fn default() -> Lpspi2SckSelectInput {
        Lpspi2SckSelectInput(0)
    }
}
impl core::fmt::Debug for Lpspi2SckSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi2SckSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi2SckSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpspi2SckSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPSPI2_SDI_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi2SdiSelectInput(pub u32);
impl Lpspi2SdiSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi2SdiSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpspi2SdiSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi2SdiSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi2SdiSelectInput {
    #[inline(always)]
    fn default() -> Lpspi2SdiSelectInput {
        Lpspi2SdiSelectInput(0)
    }
}
impl core::fmt::Debug for Lpspi2SdiSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi2SdiSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi2SdiSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpspi2SdiSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPSPI2_SDO_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi2SdoSelectInput(pub u32);
impl Lpspi2SdoSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpspi2SdoSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpspi2SdoSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpspi2SdoSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpspi2SdoSelectInput {
    #[inline(always)]
    fn default() -> Lpspi2SdoSelectInput {
        Lpspi2SdoSelectInput(0)
    }
}
impl core::fmt::Debug for Lpspi2SdoSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpspi2SdoSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpspi2SdoSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpspi2SdoSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPUART1_RXD_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart1RxdSelectInput(pub u32);
impl Lpuart1RxdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart1RxdSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart1RxdSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart1RxdSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart1RxdSelectInput {
    #[inline(always)]
    fn default() -> Lpuart1RxdSelectInput {
        Lpuart1RxdSelectInput(0)
    }
}
impl core::fmt::Debug for Lpuart1RxdSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart1RxdSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart1RxdSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpuart1RxdSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPUART1_TXD_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart1TxdSelectInput(pub u32);
impl Lpuart1TxdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart1TxdSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart1TxdSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart1TxdSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart1TxdSelectInput {
    #[inline(always)]
    fn default() -> Lpuart1TxdSelectInput {
        Lpuart1TxdSelectInput(0)
    }
}
impl core::fmt::Debug for Lpuart1TxdSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart1TxdSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart1TxdSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpuart1TxdSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPUART2_RXD_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart2RxdSelectInput(pub u32);
impl Lpuart2RxdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart2RxdSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart2RxdSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart2RxdSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart2RxdSelectInput {
    #[inline(always)]
    fn default() -> Lpuart2RxdSelectInput {
        Lpuart2RxdSelectInput(0)
    }
}
impl core::fmt::Debug for Lpuart2RxdSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart2RxdSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart2RxdSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpuart2RxdSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPUART2_TXD_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart2TxdSelectInput(pub u32);
impl Lpuart2TxdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart2TxdSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart2TxdSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart2TxdSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart2TxdSelectInput {
    #[inline(always)]
    fn default() -> Lpuart2TxdSelectInput {
        Lpuart2TxdSelectInput(0)
    }
}
impl core::fmt::Debug for Lpuart2TxdSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart2TxdSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart2TxdSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpuart2TxdSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPUART3_RXD_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart3RxdSelectInput(pub u32);
impl Lpuart3RxdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart3RxdSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpuart3RxdSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart3RxdSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpuart3RxdSelectInput {
    #[inline(always)]
    fn default() -> Lpuart3RxdSelectInput {
        Lpuart3RxdSelectInput(0)
    }
}
impl core::fmt::Debug for Lpuart3RxdSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart3RxdSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart3RxdSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpuart3RxdSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPUART3_TXD_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart3TxdSelectInput(pub u32);
impl Lpuart3TxdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart3TxdSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Lpuart3TxdSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart3TxdSelectInputDaisy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Lpuart3TxdSelectInput {
    #[inline(always)]
    fn default() -> Lpuart3TxdSelectInput {
        Lpuart3TxdSelectInput(0)
    }
}
impl core::fmt::Debug for Lpuart3TxdSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart3TxdSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart3TxdSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpuart3TxdSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPUART4_RXD_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart4RxdSelectInput(pub u32);
impl Lpuart4RxdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart4RxdSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart4RxdSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart4RxdSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart4RxdSelectInput {
    #[inline(always)]
    fn default() -> Lpuart4RxdSelectInput {
        Lpuart4RxdSelectInput(0)
    }
}
impl core::fmt::Debug for Lpuart4RxdSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart4RxdSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart4RxdSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpuart4RxdSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "LPUART4_TXD_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart4TxdSelectInput(pub u32);
impl Lpuart4TxdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::Lpuart4TxdSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpuart4TxdSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::Lpuart4TxdSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Lpuart4TxdSelectInput {
    #[inline(always)]
    fn default() -> Lpuart4TxdSelectInput {
        Lpuart4TxdSelectInput(0)
    }
}
impl core::fmt::Debug for Lpuart4TxdSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpuart4TxdSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpuart4TxdSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lpuart4TxdSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "SW_MUX_CTL_PAD_GPIO_00 SW MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MuxCtl(pub u32);
impl MuxCtl {
    #[doc = "MUX Mode Select Field."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_mode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "MUX Mode Select Field."]
    #[inline(always)]
    pub const fn set_mux_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Software Input On Field."]
    #[must_use]
    #[inline(always)]
    pub const fn sion(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Software Input On Field."]
    #[inline(always)]
    pub const fn set_sion(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for MuxCtl {
    #[inline(always)]
    fn default() -> MuxCtl {
        MuxCtl(0)
    }
}
impl core::fmt::Debug for MuxCtl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MuxCtl")
            .field("mux_mode", &self.mux_mode())
            .field("sion", &self.sion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MuxCtl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MuxCtl {{ mux_mode: {=u8:?}, sion: {=bool:?} }}",
            self.mux_mode(),
            self.sion()
        )
    }
}
#[doc = "NMI_GLUE_NMI_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NmiGlueNmiSelectInput(pub u32);
impl NmiGlueNmiSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::NmiGlueNmiSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::NmiGlueNmiSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::NmiGlueNmiSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for NmiGlueNmiSelectInput {
    #[inline(always)]
    fn default() -> NmiGlueNmiSelectInput {
        NmiGlueNmiSelectInput(0)
    }
}
impl core::fmt::Debug for NmiGlueNmiSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NmiGlueNmiSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NmiGlueNmiSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "NmiGlueNmiSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "SPDIF_IN1_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpdifIn1SelectInput(pub u32);
impl SpdifIn1SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::SpdifIn1SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SpdifIn1SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::SpdifIn1SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for SpdifIn1SelectInput {
    #[inline(always)]
    fn default() -> SpdifIn1SelectInput {
        SpdifIn1SelectInput(0)
    }
}
impl core::fmt::Debug for SpdifIn1SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SpdifIn1SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SpdifIn1SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SpdifIn1SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "SPDIF_TX_CLK2_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpdifTxClk2SelectInput(pub u32);
impl SpdifTxClk2SelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::SpdifTxClk2SelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SpdifTxClk2SelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::SpdifTxClk2SelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for SpdifTxClk2SelectInput {
    #[inline(always)]
    fn default() -> SpdifTxClk2SelectInput {
        SpdifTxClk2SelectInput(0)
    }
}
impl core::fmt::Debug for SpdifTxClk2SelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SpdifTxClk2SelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SpdifTxClk2SelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SpdifTxClk2SelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "USB_OTG_ID_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbOtgIdSelectInput(pub u32);
impl UsbOtgIdSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::UsbOtgIdSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::UsbOtgIdSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::UsbOtgIdSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for UsbOtgIdSelectInput {
    #[inline(always)]
    fn default() -> UsbOtgIdSelectInput {
        UsbOtgIdSelectInput(0)
    }
}
impl core::fmt::Debug for UsbOtgIdSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UsbOtgIdSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UsbOtgIdSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UsbOtgIdSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "USB_OTG_OC_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbOtgOcSelectInput(pub u32);
impl UsbOtgOcSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::UsbOtgOcSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::UsbOtgOcSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::UsbOtgOcSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for UsbOtgOcSelectInput {
    #[inline(always)]
    fn default() -> UsbOtgOcSelectInput {
        UsbOtgOcSelectInput(0)
    }
}
impl core::fmt::Debug for UsbOtgOcSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UsbOtgOcSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UsbOtgOcSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "UsbOtgOcSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
#[doc = "XEV_GLUE_RXEV_SELECT_INPUT DAISY Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XevGlueRxevSelectInput(pub u32);
impl XevGlueRxevSelectInput {
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[must_use]
    #[inline(always)]
    pub const fn daisy(&self) -> super::vals::XevGlueRxevSelectInputDaisy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::XevGlueRxevSelectInputDaisy::from_bits(val as u8)
    }
    #[doc = "Selecting Pads Involved in Daisy Chain."]
    #[inline(always)]
    pub const fn set_daisy(&mut self, val: super::vals::XevGlueRxevSelectInputDaisy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for XevGlueRxevSelectInput {
    #[inline(always)]
    fn default() -> XevGlueRxevSelectInput {
        XevGlueRxevSelectInput(0)
    }
}
impl core::fmt::Debug for XevGlueRxevSelectInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XevGlueRxevSelectInput")
            .field("daisy", &self.daisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for XevGlueRxevSelectInput {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "XevGlueRxevSelectInput {{ daisy: {:?} }}", self.daisy())
    }
}
