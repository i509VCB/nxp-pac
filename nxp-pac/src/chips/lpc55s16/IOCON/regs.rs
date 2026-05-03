#[doc = "Digital I/O control for port 0 pins PIO0_0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO0_0(pub u32);
impl PIO0_0 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO0_0_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO0_0_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO0_0_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO0_0_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO0_0_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO0_0_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO0_0_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO0_0_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO0_0_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO0_0_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO0_0_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO0_0_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO0_0_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO0_0_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO0_0_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn ASW(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_ASW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for PIO0_0 {
    #[inline(always)]
    fn default() -> PIO0_0 {
        PIO0_0(0)
    }
}
impl core::fmt::Debug for PIO0_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO0_0")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .field("ASW", &self.ASW())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO0_0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO0_0 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?}, ASW: {=bool:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD(),
            self.ASW()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO0_1(pub u32);
impl PIO0_1 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO0_1_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO0_1_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO0_1_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO0_1_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO0_1_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO0_1_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO0_1_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO0_1_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO0_1_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO0_1_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO0_1_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO0_1_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO0_1_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO0_1_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO0_1_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO0_1 {
    #[inline(always)]
    fn default() -> PIO0_1 {
        PIO0_1(0)
    }
}
impl core::fmt::Debug for PIO0_1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO0_1")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO0_1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO0_1 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_10."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO0_10(pub u32);
impl PIO0_10 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO0_10_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO0_10_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO0_10_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO0_10_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO0_10_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO0_10_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO0_10_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO0_10_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO0_10_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO0_10_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO0_10_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO0_10_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO0_10_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO0_10_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO0_10_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn ASW(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_ASW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for PIO0_10 {
    #[inline(always)]
    fn default() -> PIO0_10 {
        PIO0_10(0)
    }
}
impl core::fmt::Debug for PIO0_10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO0_10")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .field("ASW", &self.ASW())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO0_10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO0_10 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?}, ASW: {=bool:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD(),
            self.ASW()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_11."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO0_11(pub u32);
impl PIO0_11 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO0_11_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO0_11_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO0_11_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO0_11_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO0_11_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO0_11_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO0_11_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO0_11_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO0_11_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO0_11_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO0_11_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO0_11_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO0_11_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO0_11_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO0_11_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn ASW(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_ASW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for PIO0_11 {
    #[inline(always)]
    fn default() -> PIO0_11 {
        PIO0_11(0)
    }
}
impl core::fmt::Debug for PIO0_11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO0_11")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .field("ASW", &self.ASW())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO0_11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO0_11 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?}, ASW: {=bool:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD(),
            self.ASW()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_12."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO0_12(pub u32);
impl PIO0_12 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO0_12_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO0_12_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO0_12_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO0_12_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO0_12_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO0_12_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO0_12_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO0_12_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO0_12_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO0_12_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO0_12_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO0_12_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO0_12_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO0_12_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO0_12_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn ASW(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_ASW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for PIO0_12 {
    #[inline(always)]
    fn default() -> PIO0_12 {
        PIO0_12(0)
    }
}
impl core::fmt::Debug for PIO0_12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO0_12")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .field("ASW", &self.ASW())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO0_12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO0_12 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?}, ASW: {=bool:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD(),
            self.ASW()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_13."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO0_13(pub u32);
impl PIO0_13 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO0_13_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO0_13_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO0_13_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO0_13_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO0_13_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO0_13_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO0_13_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO0_13_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO0_13_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO0_13_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO0_13_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO0_13_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode in standard GPIO mode (EGP = 1). This bit has no effect in I2C mode (EGP=0)."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO0_13_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO0_13_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode in standard GPIO mode (EGP = 1). This bit has no effect in I2C mode (EGP=0)."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO0_13_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Supply Selection bit."]
    #[must_use]
    #[inline(always)]
    pub const fn SSEL(&self) -> super::vals::PIO0_13_SSEL {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::PIO0_13_SSEL::from_bits(val as u8)
    }
    #[doc = "Supply Selection bit."]
    #[inline(always)]
    pub const fn set_SSEL(&mut self, val: super::vals::PIO0_13_SSEL) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Controls input glitch filter."]
    #[must_use]
    #[inline(always)]
    pub const fn FILTEROFF(&self) -> super::vals::PIO0_13_FILTEROFF {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::PIO0_13_FILTEROFF::from_bits(val as u8)
    }
    #[doc = "Controls input glitch filter."]
    #[inline(always)]
    pub const fn set_FILTEROFF(&mut self, val: super::vals::PIO0_13_FILTEROFF) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Pull-up current source enable in I2C mode."]
    #[must_use]
    #[inline(always)]
    pub const fn ECS(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Pull-up current source enable in I2C mode."]
    #[inline(always)]
    pub const fn set_ECS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Switch between GPIO mode and I2C mode."]
    #[must_use]
    #[inline(always)]
    pub const fn EGP(&self) -> super::vals::PIO0_13_EGP {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::PIO0_13_EGP::from_bits(val as u8)
    }
    #[doc = "Switch between GPIO mode and I2C mode."]
    #[inline(always)]
    pub const fn set_EGP(&mut self, val: super::vals::PIO0_13_EGP) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation and High-Speed mode operation."]
    #[must_use]
    #[inline(always)]
    pub const fn I2CFILTER(&self) -> super::vals::PIO0_13_I2CFILTER {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::PIO0_13_I2CFILTER::from_bits(val as u8)
    }
    #[doc = "Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation and High-Speed mode operation."]
    #[inline(always)]
    pub const fn set_I2CFILTER(&mut self, val: super::vals::PIO0_13_I2CFILTER) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for PIO0_13 {
    #[inline(always)]
    fn default() -> PIO0_13 {
        PIO0_13(0)
    }
}
impl core::fmt::Debug for PIO0_13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO0_13")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .field("SSEL", &self.SSEL())
            .field("FILTEROFF", &self.FILTEROFF())
            .field("ECS", &self.ECS())
            .field("EGP", &self.EGP())
            .field("I2CFILTER", &self.I2CFILTER())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO0_13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO0_13 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?}, SSEL: {:?}, FILTEROFF: {:?}, ECS: {=bool:?}, EGP: {:?}, I2CFILTER: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD(),
            self.SSEL(),
            self.FILTEROFF(),
            self.ECS(),
            self.EGP(),
            self.I2CFILTER()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_14."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO0_14(pub u32);
impl PIO0_14 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO0_14_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO0_14_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO0_14_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO0_14_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO0_14_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO0_14_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO0_14_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO0_14_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO0_14_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO0_14_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO0_14_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO0_14_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode in standard GPIO mode (EGP = 1). This bit has no effect in I2C mode (EGP=0)."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO0_14_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO0_14_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode in standard GPIO mode (EGP = 1). This bit has no effect in I2C mode (EGP=0)."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO0_14_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Supply Selection bit."]
    #[must_use]
    #[inline(always)]
    pub const fn SSEL(&self) -> super::vals::PIO0_14_SSEL {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::PIO0_14_SSEL::from_bits(val as u8)
    }
    #[doc = "Supply Selection bit."]
    #[inline(always)]
    pub const fn set_SSEL(&mut self, val: super::vals::PIO0_14_SSEL) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Controls input glitch filter."]
    #[must_use]
    #[inline(always)]
    pub const fn FILTEROFF(&self) -> super::vals::PIO0_14_FILTEROFF {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::PIO0_14_FILTEROFF::from_bits(val as u8)
    }
    #[doc = "Controls input glitch filter."]
    #[inline(always)]
    pub const fn set_FILTEROFF(&mut self, val: super::vals::PIO0_14_FILTEROFF) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Pull-up current source enable in I2C mode."]
    #[must_use]
    #[inline(always)]
    pub const fn ECS(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Pull-up current source enable in I2C mode."]
    #[inline(always)]
    pub const fn set_ECS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Switch between GPIO mode and I2C mode."]
    #[must_use]
    #[inline(always)]
    pub const fn EGP(&self) -> super::vals::PIO0_14_EGP {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::PIO0_14_EGP::from_bits(val as u8)
    }
    #[doc = "Switch between GPIO mode and I2C mode."]
    #[inline(always)]
    pub const fn set_EGP(&mut self, val: super::vals::PIO0_14_EGP) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation and High-Speed mode operation."]
    #[must_use]
    #[inline(always)]
    pub const fn I2CFILTER(&self) -> super::vals::PIO0_14_I2CFILTER {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::PIO0_14_I2CFILTER::from_bits(val as u8)
    }
    #[doc = "Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation and High-Speed mode operation."]
    #[inline(always)]
    pub const fn set_I2CFILTER(&mut self, val: super::vals::PIO0_14_I2CFILTER) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for PIO0_14 {
    #[inline(always)]
    fn default() -> PIO0_14 {
        PIO0_14(0)
    }
}
impl core::fmt::Debug for PIO0_14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO0_14")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .field("SSEL", &self.SSEL())
            .field("FILTEROFF", &self.FILTEROFF())
            .field("ECS", &self.ECS())
            .field("EGP", &self.EGP())
            .field("I2CFILTER", &self.I2CFILTER())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO0_14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO0_14 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?}, SSEL: {:?}, FILTEROFF: {:?}, ECS: {=bool:?}, EGP: {:?}, I2CFILTER: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD(),
            self.SSEL(),
            self.FILTEROFF(),
            self.ECS(),
            self.EGP(),
            self.I2CFILTER()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_15."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO0_15(pub u32);
impl PIO0_15 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO0_15_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO0_15_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO0_15_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO0_15_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO0_15_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO0_15_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO0_15_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO0_15_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO0_15_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO0_15_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO0_15_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO0_15_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO0_15_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO0_15_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO0_15_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn ASW(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_ASW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for PIO0_15 {
    #[inline(always)]
    fn default() -> PIO0_15 {
        PIO0_15(0)
    }
}
impl core::fmt::Debug for PIO0_15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO0_15")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .field("ASW", &self.ASW())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO0_15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO0_15 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?}, ASW: {=bool:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD(),
            self.ASW()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_16."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO0_16(pub u32);
impl PIO0_16 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO0_16_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO0_16_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO0_16_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO0_16_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO0_16_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO0_16_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO0_16_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO0_16_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO0_16_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO0_16_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO0_16_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO0_16_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO0_16_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO0_16_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO0_16_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn ASW(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_ASW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for PIO0_16 {
    #[inline(always)]
    fn default() -> PIO0_16 {
        PIO0_16(0)
    }
}
impl core::fmt::Debug for PIO0_16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO0_16")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .field("ASW", &self.ASW())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO0_16 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO0_16 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?}, ASW: {=bool:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD(),
            self.ASW()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_17."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO0_17(pub u32);
impl PIO0_17 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO0_17_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO0_17_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO0_17_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO0_17_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO0_17_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO0_17_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO0_17_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO0_17_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO0_17_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO0_17_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO0_17_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO0_17_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO0_17_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO0_17_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO0_17_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO0_17 {
    #[inline(always)]
    fn default() -> PIO0_17 {
        PIO0_17(0)
    }
}
impl core::fmt::Debug for PIO0_17 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO0_17")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO0_17 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO0_17 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_18."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO0_18(pub u32);
impl PIO0_18 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO0_18_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO0_18_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO0_18_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO0_18_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO0_18_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO0_18_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO0_18_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO0_18_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO0_18_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO0_18_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO0_18_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO0_18_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO0_18_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO0_18_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO0_18_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn ASW(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_ASW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for PIO0_18 {
    #[inline(always)]
    fn default() -> PIO0_18 {
        PIO0_18(0)
    }
}
impl core::fmt::Debug for PIO0_18 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO0_18")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .field("ASW", &self.ASW())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO0_18 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO0_18 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?}, ASW: {=bool:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD(),
            self.ASW()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_19."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO0_19(pub u32);
impl PIO0_19 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO0_19_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO0_19_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO0_19_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO0_19_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO0_19_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO0_19_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO0_19_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO0_19_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO0_19_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO0_19_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO0_19_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO0_19_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO0_19_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO0_19_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO0_19_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO0_19 {
    #[inline(always)]
    fn default() -> PIO0_19 {
        PIO0_19(0)
    }
}
impl core::fmt::Debug for PIO0_19 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO0_19")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO0_19 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO0_19 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO0_2(pub u32);
impl PIO0_2 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO0_2_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO0_2_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO0_2_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO0_2_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO0_2_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO0_2_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO0_2_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO0_2_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO0_2_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO0_2_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO0_2_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO0_2_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO0_2_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO0_2_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO0_2_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO0_2 {
    #[inline(always)]
    fn default() -> PIO0_2 {
        PIO0_2(0)
    }
}
impl core::fmt::Debug for PIO0_2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO0_2")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO0_2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO0_2 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_20."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO0_20(pub u32);
impl PIO0_20 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO0_20_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO0_20_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO0_20_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO0_20_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO0_20_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO0_20_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO0_20_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO0_20_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO0_20_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO0_20_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO0_20_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO0_20_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO0_20_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO0_20_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO0_20_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO0_20 {
    #[inline(always)]
    fn default() -> PIO0_20 {
        PIO0_20(0)
    }
}
impl core::fmt::Debug for PIO0_20 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO0_20")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO0_20 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO0_20 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_21."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO0_21(pub u32);
impl PIO0_21 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO0_21_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO0_21_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO0_21_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO0_21_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO0_21_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO0_21_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO0_21_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO0_21_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO0_21_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO0_21_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO0_21_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO0_21_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO0_21_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO0_21_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO0_21_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO0_21 {
    #[inline(always)]
    fn default() -> PIO0_21 {
        PIO0_21(0)
    }
}
impl core::fmt::Debug for PIO0_21 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO0_21")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO0_21 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO0_21 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_22."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO0_22(pub u32);
impl PIO0_22 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO0_22_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO0_22_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO0_22_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO0_22_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO0_22_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO0_22_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO0_22_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO0_22_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO0_22_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO0_22_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO0_22_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO0_22_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO0_22_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO0_22_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO0_22_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO0_22 {
    #[inline(always)]
    fn default() -> PIO0_22 {
        PIO0_22(0)
    }
}
impl core::fmt::Debug for PIO0_22 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO0_22")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO0_22 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO0_22 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_23."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO0_23(pub u32);
impl PIO0_23 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO0_23_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO0_23_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO0_23_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO0_23_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO0_23_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO0_23_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO0_23_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO0_23_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO0_23_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO0_23_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO0_23_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO0_23_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO0_23_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO0_23_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO0_23_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn ASW(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_ASW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for PIO0_23 {
    #[inline(always)]
    fn default() -> PIO0_23 {
        PIO0_23(0)
    }
}
impl core::fmt::Debug for PIO0_23 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO0_23")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .field("ASW", &self.ASW())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO0_23 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO0_23 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?}, ASW: {=bool:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD(),
            self.ASW()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_24."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO0_24(pub u32);
impl PIO0_24 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO0_24_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO0_24_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO0_24_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO0_24_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO0_24_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO0_24_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO0_24_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO0_24_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO0_24_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO0_24_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO0_24_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO0_24_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO0_24_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO0_24_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO0_24_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO0_24 {
    #[inline(always)]
    fn default() -> PIO0_24 {
        PIO0_24(0)
    }
}
impl core::fmt::Debug for PIO0_24 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO0_24")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO0_24 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO0_24 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_25."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO0_25(pub u32);
impl PIO0_25 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO0_25_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO0_25_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO0_25_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO0_25_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO0_25_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO0_25_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO0_25_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO0_25_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO0_25_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO0_25_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO0_25_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO0_25_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO0_25_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO0_25_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO0_25_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO0_25 {
    #[inline(always)]
    fn default() -> PIO0_25 {
        PIO0_25(0)
    }
}
impl core::fmt::Debug for PIO0_25 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO0_25")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO0_25 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO0_25 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_26."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO0_26(pub u32);
impl PIO0_26 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO0_26_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO0_26_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO0_26_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO0_26_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO0_26_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO0_26_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO0_26_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO0_26_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO0_26_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO0_26_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO0_26_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO0_26_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO0_26_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO0_26_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO0_26_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO0_26 {
    #[inline(always)]
    fn default() -> PIO0_26 {
        PIO0_26(0)
    }
}
impl core::fmt::Debug for PIO0_26 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO0_26")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO0_26 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO0_26 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_27."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO0_27(pub u32);
impl PIO0_27 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO0_27_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO0_27_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO0_27_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO0_27_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO0_27_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO0_27_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO0_27_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO0_27_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO0_27_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO0_27_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO0_27_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO0_27_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO0_27_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO0_27_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO0_27_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO0_27 {
    #[inline(always)]
    fn default() -> PIO0_27 {
        PIO0_27(0)
    }
}
impl core::fmt::Debug for PIO0_27 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO0_27")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO0_27 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO0_27 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_28."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO0_28(pub u32);
impl PIO0_28 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO0_28_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO0_28_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO0_28_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO0_28_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO0_28_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO0_28_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO0_28_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO0_28_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO0_28_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO0_28_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO0_28_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO0_28_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO0_28_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO0_28_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO0_28_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO0_28 {
    #[inline(always)]
    fn default() -> PIO0_28 {
        PIO0_28(0)
    }
}
impl core::fmt::Debug for PIO0_28 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO0_28")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO0_28 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO0_28 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_29."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO0_29(pub u32);
impl PIO0_29 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO0_29_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO0_29_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO0_29_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO0_29_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO0_29_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO0_29_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO0_29_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO0_29_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO0_29_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO0_29_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO0_29_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO0_29_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO0_29_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO0_29_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO0_29_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO0_29 {
    #[inline(always)]
    fn default() -> PIO0_29 {
        PIO0_29(0)
    }
}
impl core::fmt::Debug for PIO0_29 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO0_29")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO0_29 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO0_29 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO0_3(pub u32);
impl PIO0_3 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO0_3_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO0_3_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO0_3_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO0_3_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO0_3_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO0_3_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO0_3_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO0_3_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO0_3_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO0_3_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO0_3_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO0_3_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO0_3_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO0_3_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO0_3_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO0_3 {
    #[inline(always)]
    fn default() -> PIO0_3 {
        PIO0_3(0)
    }
}
impl core::fmt::Debug for PIO0_3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO0_3")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO0_3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO0_3 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_30."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO0_30(pub u32);
impl PIO0_30 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO0_30_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO0_30_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO0_30_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO0_30_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO0_30_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO0_30_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO0_30_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO0_30_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO0_30_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO0_30_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO0_30_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO0_30_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO0_30_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO0_30_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO0_30_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO0_30 {
    #[inline(always)]
    fn default() -> PIO0_30 {
        PIO0_30(0)
    }
}
impl core::fmt::Debug for PIO0_30 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO0_30")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO0_30 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO0_30 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_31."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO0_31(pub u32);
impl PIO0_31 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO0_31_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO0_31_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO0_31_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO0_31_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO0_31_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO0_31_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO0_31_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO0_31_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO0_31_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO0_31_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO0_31_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO0_31_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO0_31_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO0_31_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO0_31_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn ASW(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_ASW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for PIO0_31 {
    #[inline(always)]
    fn default() -> PIO0_31 {
        PIO0_31(0)
    }
}
impl core::fmt::Debug for PIO0_31 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO0_31")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .field("ASW", &self.ASW())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO0_31 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO0_31 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?}, ASW: {=bool:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD(),
            self.ASW()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_4."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO0_4(pub u32);
impl PIO0_4 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO0_4_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO0_4_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO0_4_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO0_4_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO0_4_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO0_4_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO0_4_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO0_4_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO0_4_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO0_4_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO0_4_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO0_4_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO0_4_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO0_4_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO0_4_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO0_4 {
    #[inline(always)]
    fn default() -> PIO0_4 {
        PIO0_4(0)
    }
}
impl core::fmt::Debug for PIO0_4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO0_4")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO0_4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO0_4 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_5."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO0_5(pub u32);
impl PIO0_5 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO0_5_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO0_5_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO0_5_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO0_5_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO0_5_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO0_5_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO0_5_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO0_5_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO0_5_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO0_5_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO0_5_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO0_5_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO0_5_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO0_5_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO0_5_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO0_5 {
    #[inline(always)]
    fn default() -> PIO0_5 {
        PIO0_5(0)
    }
}
impl core::fmt::Debug for PIO0_5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO0_5")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO0_5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO0_5 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_6."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO0_6(pub u32);
impl PIO0_6 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO0_6_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO0_6_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO0_6_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO0_6_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO0_6_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO0_6_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO0_6_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO0_6_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO0_6_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO0_6_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO0_6_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO0_6_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO0_6_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO0_6_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO0_6_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO0_6 {
    #[inline(always)]
    fn default() -> PIO0_6 {
        PIO0_6(0)
    }
}
impl core::fmt::Debug for PIO0_6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO0_6")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO0_6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO0_6 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_7."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO0_7(pub u32);
impl PIO0_7 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO0_7_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO0_7_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO0_7_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO0_7_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO0_7_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO0_7_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO0_7_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO0_7_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO0_7_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO0_7_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO0_7_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO0_7_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO0_7_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO0_7_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO0_7_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO0_7 {
    #[inline(always)]
    fn default() -> PIO0_7 {
        PIO0_7(0)
    }
}
impl core::fmt::Debug for PIO0_7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO0_7")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO0_7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO0_7 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_8."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO0_8(pub u32);
impl PIO0_8 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO0_8_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO0_8_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO0_8_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO0_8_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO0_8_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO0_8_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO0_8_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO0_8_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO0_8_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO0_8_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO0_8_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO0_8_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO0_8_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO0_8_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO0_8_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO0_8 {
    #[inline(always)]
    fn default() -> PIO0_8 {
        PIO0_8(0)
    }
}
impl core::fmt::Debug for PIO0_8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO0_8")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO0_8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO0_8 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 0 pins PIO0_9."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO0_9(pub u32);
impl PIO0_9 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO0_9_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO0_9_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO0_9_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO0_9_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO0_9_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO0_9_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO0_9_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO0_9_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO0_9_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO0_9_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO0_9_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO0_9_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO0_9_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO0_9_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO0_9_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn ASW(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_ASW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for PIO0_9 {
    #[inline(always)]
    fn default() -> PIO0_9 {
        PIO0_9(0)
    }
}
impl core::fmt::Debug for PIO0_9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO0_9")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .field("ASW", &self.ASW())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO0_9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO0_9 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?}, ASW: {=bool:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD(),
            self.ASW()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO1_0(pub u32);
impl PIO1_0 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO1_0_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO1_0_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO1_0_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO1_0_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO1_0_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO1_0_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO1_0_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO1_0_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO1_0_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO1_0_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO1_0_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO1_0_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO1_0_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO1_0_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO1_0_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn ASW(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_ASW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for PIO1_0 {
    #[inline(always)]
    fn default() -> PIO1_0 {
        PIO1_0(0)
    }
}
impl core::fmt::Debug for PIO1_0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO1_0")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .field("ASW", &self.ASW())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO1_0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO1_0 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?}, ASW: {=bool:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD(),
            self.ASW()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO1_1(pub u32);
impl PIO1_1 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO1_1_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO1_1_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO1_1_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO1_1_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO1_1_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO1_1_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO1_1_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO1_1_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO1_1_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO1_1_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO1_1_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO1_1_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO1_1_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO1_1_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO1_1_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO1_1 {
    #[inline(always)]
    fn default() -> PIO1_1 {
        PIO1_1(0)
    }
}
impl core::fmt::Debug for PIO1_1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO1_1")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO1_1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO1_1 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_10."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO1_10(pub u32);
impl PIO1_10 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO1_10_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO1_10_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO1_10_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO1_10_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO1_10_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO1_10_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO1_10_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO1_10_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO1_10_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO1_10_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO1_10_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO1_10_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO1_10_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO1_10_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO1_10_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO1_10 {
    #[inline(always)]
    fn default() -> PIO1_10 {
        PIO1_10(0)
    }
}
impl core::fmt::Debug for PIO1_10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO1_10")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO1_10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO1_10 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_11."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO1_11(pub u32);
impl PIO1_11 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO1_11_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO1_11_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO1_11_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO1_11_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO1_11_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO1_11_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO1_11_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO1_11_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO1_11_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO1_11_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO1_11_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO1_11_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO1_11_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO1_11_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO1_11_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO1_11 {
    #[inline(always)]
    fn default() -> PIO1_11 {
        PIO1_11(0)
    }
}
impl core::fmt::Debug for PIO1_11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO1_11")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO1_11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO1_11 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_12."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO1_12(pub u32);
impl PIO1_12 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO1_12_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO1_12_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO1_12_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO1_12_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO1_12_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO1_12_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO1_12_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO1_12_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO1_12_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO1_12_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO1_12_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO1_12_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO1_12_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO1_12_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO1_12_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO1_12 {
    #[inline(always)]
    fn default() -> PIO1_12 {
        PIO1_12(0)
    }
}
impl core::fmt::Debug for PIO1_12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO1_12")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO1_12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO1_12 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_13."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO1_13(pub u32);
impl PIO1_13 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO1_13_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO1_13_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO1_13_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO1_13_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO1_13_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO1_13_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO1_13_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO1_13_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO1_13_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO1_13_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO1_13_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO1_13_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO1_13_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO1_13_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO1_13_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO1_13 {
    #[inline(always)]
    fn default() -> PIO1_13 {
        PIO1_13(0)
    }
}
impl core::fmt::Debug for PIO1_13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO1_13")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO1_13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO1_13 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_14."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO1_14(pub u32);
impl PIO1_14 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO1_14_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO1_14_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO1_14_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO1_14_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO1_14_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO1_14_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO1_14_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO1_14_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO1_14_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO1_14_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO1_14_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO1_14_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO1_14_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO1_14_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO1_14_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn ASW(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_ASW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for PIO1_14 {
    #[inline(always)]
    fn default() -> PIO1_14 {
        PIO1_14(0)
    }
}
impl core::fmt::Debug for PIO1_14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO1_14")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .field("ASW", &self.ASW())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO1_14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO1_14 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?}, ASW: {=bool:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD(),
            self.ASW()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_15."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO1_15(pub u32);
impl PIO1_15 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO1_15_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO1_15_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO1_15_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO1_15_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO1_15_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO1_15_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO1_15_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO1_15_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO1_15_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO1_15_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO1_15_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO1_15_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO1_15_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO1_15_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO1_15_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO1_15 {
    #[inline(always)]
    fn default() -> PIO1_15 {
        PIO1_15(0)
    }
}
impl core::fmt::Debug for PIO1_15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO1_15")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO1_15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO1_15 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_16."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO1_16(pub u32);
impl PIO1_16 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO1_16_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO1_16_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO1_16_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO1_16_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO1_16_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO1_16_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO1_16_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO1_16_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO1_16_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO1_16_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO1_16_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO1_16_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO1_16_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO1_16_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO1_16_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO1_16 {
    #[inline(always)]
    fn default() -> PIO1_16 {
        PIO1_16(0)
    }
}
impl core::fmt::Debug for PIO1_16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO1_16")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO1_16 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO1_16 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_17."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO1_17(pub u32);
impl PIO1_17 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO1_17_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO1_17_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO1_17_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO1_17_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO1_17_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO1_17_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO1_17_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO1_17_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO1_17_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO1_17_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO1_17_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO1_17_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO1_17_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO1_17_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO1_17_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO1_17 {
    #[inline(always)]
    fn default() -> PIO1_17 {
        PIO1_17(0)
    }
}
impl core::fmt::Debug for PIO1_17 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO1_17")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO1_17 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO1_17 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_18."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO1_18(pub u32);
impl PIO1_18 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO1_18_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO1_18_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO1_18_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO1_18_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO1_18_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO1_18_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO1_18_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO1_18_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO1_18_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO1_18_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO1_18_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO1_18_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO1_18_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO1_18_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO1_18_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO1_18 {
    #[inline(always)]
    fn default() -> PIO1_18 {
        PIO1_18(0)
    }
}
impl core::fmt::Debug for PIO1_18 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO1_18")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO1_18 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO1_18 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_19."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO1_19(pub u32);
impl PIO1_19 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO1_19_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO1_19_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO1_19_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO1_19_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO1_19_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO1_19_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO1_19_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO1_19_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO1_19_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO1_19_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO1_19_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO1_19_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO1_19_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO1_19_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO1_19_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn ASW(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_ASW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for PIO1_19 {
    #[inline(always)]
    fn default() -> PIO1_19 {
        PIO1_19(0)
    }
}
impl core::fmt::Debug for PIO1_19 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO1_19")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .field("ASW", &self.ASW())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO1_19 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO1_19 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?}, ASW: {=bool:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD(),
            self.ASW()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO1_2(pub u32);
impl PIO1_2 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO1_2_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO1_2_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO1_2_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO1_2_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO1_2_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO1_2_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO1_2_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO1_2_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO1_2_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO1_2_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO1_2_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO1_2_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO1_2_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO1_2_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO1_2_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO1_2 {
    #[inline(always)]
    fn default() -> PIO1_2 {
        PIO1_2(0)
    }
}
impl core::fmt::Debug for PIO1_2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO1_2")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO1_2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO1_2 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_20."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO1_20(pub u32);
impl PIO1_20 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO1_20_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO1_20_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO1_20_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO1_20_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO1_20_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO1_20_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO1_20_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO1_20_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO1_20_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO1_20_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO1_20_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO1_20_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO1_20_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO1_20_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO1_20_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO1_20 {
    #[inline(always)]
    fn default() -> PIO1_20 {
        PIO1_20(0)
    }
}
impl core::fmt::Debug for PIO1_20 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO1_20")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO1_20 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO1_20 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_21."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO1_21(pub u32);
impl PIO1_21 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO1_21_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO1_21_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO1_21_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO1_21_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO1_21_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO1_21_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO1_21_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO1_21_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO1_21_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO1_21_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO1_21_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO1_21_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO1_21_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO1_21_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO1_21_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO1_21 {
    #[inline(always)]
    fn default() -> PIO1_21 {
        PIO1_21(0)
    }
}
impl core::fmt::Debug for PIO1_21 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO1_21")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO1_21 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO1_21 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_22."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO1_22(pub u32);
impl PIO1_22 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO1_22_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO1_22_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO1_22_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO1_22_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO1_22_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO1_22_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO1_22_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO1_22_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO1_22_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO1_22_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO1_22_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO1_22_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO1_22_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO1_22_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO1_22_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO1_22 {
    #[inline(always)]
    fn default() -> PIO1_22 {
        PIO1_22(0)
    }
}
impl core::fmt::Debug for PIO1_22 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO1_22")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO1_22 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO1_22 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_23."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO1_23(pub u32);
impl PIO1_23 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO1_23_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO1_23_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO1_23_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO1_23_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO1_23_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO1_23_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO1_23_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO1_23_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO1_23_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO1_23_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO1_23_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO1_23_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO1_23_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO1_23_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO1_23_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO1_23 {
    #[inline(always)]
    fn default() -> PIO1_23 {
        PIO1_23(0)
    }
}
impl core::fmt::Debug for PIO1_23 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO1_23")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO1_23 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO1_23 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_24."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO1_24(pub u32);
impl PIO1_24 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO1_24_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO1_24_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO1_24_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO1_24_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO1_24_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO1_24_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO1_24_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO1_24_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO1_24_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO1_24_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO1_24_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO1_24_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO1_24_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO1_24_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO1_24_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO1_24 {
    #[inline(always)]
    fn default() -> PIO1_24 {
        PIO1_24(0)
    }
}
impl core::fmt::Debug for PIO1_24 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO1_24")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO1_24 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO1_24 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_25."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO1_25(pub u32);
impl PIO1_25 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO1_25_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO1_25_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO1_25_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO1_25_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO1_25_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO1_25_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO1_25_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO1_25_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO1_25_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO1_25_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO1_25_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO1_25_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO1_25_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO1_25_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO1_25_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO1_25 {
    #[inline(always)]
    fn default() -> PIO1_25 {
        PIO1_25(0)
    }
}
impl core::fmt::Debug for PIO1_25 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO1_25")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO1_25 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO1_25 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_26."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO1_26(pub u32);
impl PIO1_26 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO1_26_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO1_26_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO1_26_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO1_26_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO1_26_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO1_26_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO1_26_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO1_26_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO1_26_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO1_26_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO1_26_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO1_26_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO1_26_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO1_26_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO1_26_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO1_26 {
    #[inline(always)]
    fn default() -> PIO1_26 {
        PIO1_26(0)
    }
}
impl core::fmt::Debug for PIO1_26 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO1_26")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO1_26 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO1_26 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_27."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO1_27(pub u32);
impl PIO1_27 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO1_27_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO1_27_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO1_27_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO1_27_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO1_27_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO1_27_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO1_27_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO1_27_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO1_27_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO1_27_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO1_27_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO1_27_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO1_27_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO1_27_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO1_27_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO1_27 {
    #[inline(always)]
    fn default() -> PIO1_27 {
        PIO1_27(0)
    }
}
impl core::fmt::Debug for PIO1_27 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO1_27")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO1_27 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO1_27 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_28."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO1_28(pub u32);
impl PIO1_28 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO1_28_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO1_28_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO1_28_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO1_28_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO1_28_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO1_28_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO1_28_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO1_28_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO1_28_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO1_28_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO1_28_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO1_28_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO1_28_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO1_28_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO1_28_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO1_28 {
    #[inline(always)]
    fn default() -> PIO1_28 {
        PIO1_28(0)
    }
}
impl core::fmt::Debug for PIO1_28 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO1_28")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO1_28 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO1_28 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_29."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO1_29(pub u32);
impl PIO1_29 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO1_29_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO1_29_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO1_29_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO1_29_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO1_29_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO1_29_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO1_29_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO1_29_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO1_29_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO1_29_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO1_29_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO1_29_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO1_29_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO1_29_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO1_29_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO1_29 {
    #[inline(always)]
    fn default() -> PIO1_29 {
        PIO1_29(0)
    }
}
impl core::fmt::Debug for PIO1_29 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO1_29")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO1_29 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO1_29 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO1_3(pub u32);
impl PIO1_3 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO1_3_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO1_3_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO1_3_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO1_3_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO1_3_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO1_3_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO1_3_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO1_3_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO1_3_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO1_3_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO1_3_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO1_3_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO1_3_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO1_3_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO1_3_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO1_3 {
    #[inline(always)]
    fn default() -> PIO1_3 {
        PIO1_3(0)
    }
}
impl core::fmt::Debug for PIO1_3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO1_3")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO1_3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO1_3 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_30."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO1_30(pub u32);
impl PIO1_30 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO1_30_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO1_30_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO1_30_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO1_30_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO1_30_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO1_30_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO1_30_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO1_30_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO1_30_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO1_30_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO1_30_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO1_30_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO1_30_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO1_30_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO1_30_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO1_30 {
    #[inline(always)]
    fn default() -> PIO1_30 {
        PIO1_30(0)
    }
}
impl core::fmt::Debug for PIO1_30 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO1_30")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO1_30 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO1_30 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_31."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO1_31(pub u32);
impl PIO1_31 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO1_31_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO1_31_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO1_31_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO1_31_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO1_31_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO1_31_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO1_31_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO1_31_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO1_31_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO1_31_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO1_31_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO1_31_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO1_31_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO1_31_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO1_31_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO1_31 {
    #[inline(always)]
    fn default() -> PIO1_31 {
        PIO1_31(0)
    }
}
impl core::fmt::Debug for PIO1_31 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO1_31")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO1_31 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO1_31 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_4."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO1_4(pub u32);
impl PIO1_4 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO1_4_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO1_4_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO1_4_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO1_4_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO1_4_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO1_4_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO1_4_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO1_4_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO1_4_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO1_4_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO1_4_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO1_4_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO1_4_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO1_4_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO1_4_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO1_4 {
    #[inline(always)]
    fn default() -> PIO1_4 {
        PIO1_4(0)
    }
}
impl core::fmt::Debug for PIO1_4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO1_4")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO1_4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO1_4 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_5."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO1_5(pub u32);
impl PIO1_5 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO1_5_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO1_5_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO1_5_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO1_5_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO1_5_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO1_5_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO1_5_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO1_5_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO1_5_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO1_5_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO1_5_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO1_5_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO1_5_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO1_5_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO1_5_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO1_5 {
    #[inline(always)]
    fn default() -> PIO1_5 {
        PIO1_5(0)
    }
}
impl core::fmt::Debug for PIO1_5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO1_5")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO1_5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO1_5 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_6."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO1_6(pub u32);
impl PIO1_6 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO1_6_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO1_6_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO1_6_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO1_6_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO1_6_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO1_6_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO1_6_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO1_6_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO1_6_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO1_6_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO1_6_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO1_6_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO1_6_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO1_6_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO1_6_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO1_6 {
    #[inline(always)]
    fn default() -> PIO1_6 {
        PIO1_6(0)
    }
}
impl core::fmt::Debug for PIO1_6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO1_6")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO1_6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO1_6 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_7."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO1_7(pub u32);
impl PIO1_7 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO1_7_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO1_7_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO1_7_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO1_7_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO1_7_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO1_7_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO1_7_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO1_7_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO1_7_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO1_7_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO1_7_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO1_7_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO1_7_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO1_7_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO1_7_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for PIO1_7 {
    #[inline(always)]
    fn default() -> PIO1_7 {
        PIO1_7(0)
    }
}
impl core::fmt::Debug for PIO1_7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO1_7")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO1_7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO1_7 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_8."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO1_8(pub u32);
impl PIO1_8 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO1_8_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO1_8_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO1_8_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO1_8_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO1_8_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO1_8_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO1_8_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO1_8_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO1_8_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO1_8_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO1_8_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO1_8_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO1_8_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO1_8_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO1_8_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn ASW(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_ASW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for PIO1_8 {
    #[inline(always)]
    fn default() -> PIO1_8 {
        PIO1_8(0)
    }
}
impl core::fmt::Debug for PIO1_8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO1_8")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .field("ASW", &self.ASW())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO1_8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO1_8 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?}, ASW: {=bool:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD(),
            self.ASW()
        )
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_9."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PIO1_9(pub u32);
impl PIO1_9 {
    #[doc = "Selects pin function."]
    #[must_use]
    #[inline(always)]
    pub const fn FUNC(&self) -> super::vals::PIO1_9_FUNC {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::PIO1_9_FUNC::from_bits(val as u8)
    }
    #[doc = "Selects pin function."]
    #[inline(always)]
    pub const fn set_FUNC(&mut self, val: super::vals::PIO1_9_FUNC) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[must_use]
    #[inline(always)]
    pub const fn MODE(&self) -> super::vals::PIO1_9_MODE {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PIO1_9_MODE::from_bits(val as u8)
    }
    #[doc = "Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub const fn set_MODE(&mut self, val: super::vals::PIO1_9_MODE) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Driver slew rate."]
    #[must_use]
    #[inline(always)]
    pub const fn SLEW(&self) -> super::vals::PIO1_9_SLEW {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PIO1_9_SLEW::from_bits(val as u8)
    }
    #[doc = "Driver slew rate."]
    #[inline(always)]
    pub const fn set_SLEW(&mut self, val: super::vals::PIO1_9_SLEW) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Input polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn INVERT(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Input polarity."]
    #[inline(always)]
    pub const fn set_INVERT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Select Digital mode."]
    #[must_use]
    #[inline(always)]
    pub const fn DIGIMODE(&self) -> super::vals::PIO1_9_DIGIMODE {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PIO1_9_DIGIMODE::from_bits(val as u8)
    }
    #[doc = "Select Digital mode."]
    #[inline(always)]
    pub const fn set_DIGIMODE(&mut self, val: super::vals::PIO1_9_DIGIMODE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Controls open-drain mode."]
    #[must_use]
    #[inline(always)]
    pub const fn OD(&self) -> super::vals::PIO1_9_OD {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PIO1_9_OD::from_bits(val as u8)
    }
    #[doc = "Controls open-drain mode."]
    #[inline(always)]
    pub const fn set_OD(&mut self, val: super::vals::PIO1_9_OD) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Analog switch input control."]
    #[must_use]
    #[inline(always)]
    pub const fn ASW(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Analog switch input control."]
    #[inline(always)]
    pub const fn set_ASW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for PIO1_9 {
    #[inline(always)]
    fn default() -> PIO1_9 {
        PIO1_9(0)
    }
}
impl core::fmt::Debug for PIO1_9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIO1_9")
            .field("FUNC", &self.FUNC())
            .field("MODE", &self.MODE())
            .field("SLEW", &self.SLEW())
            .field("INVERT", &self.INVERT())
            .field("DIGIMODE", &self.DIGIMODE())
            .field("OD", &self.OD())
            .field("ASW", &self.ASW())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PIO1_9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PIO1_9 {{ FUNC: {:?}, MODE: {:?}, SLEW: {:?}, INVERT: {=bool:?}, DIGIMODE: {:?}, OD: {:?}, ASW: {=bool:?} }}",
            self.FUNC(),
            self.MODE(),
            self.SLEW(),
            self.INVERT(),
            self.DIGIMODE(),
            self.OD(),
            self.ASW()
        )
    }
}
