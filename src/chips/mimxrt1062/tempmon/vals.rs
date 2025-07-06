#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tempsense0ClrMeasureTemp {
    #[doc = "Do not start the measurement process."]
    STOP = 0x0,
    #[doc = "Start the measurement process."]
    START = 0x01,
}
impl Tempsense0ClrMeasureTemp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tempsense0ClrMeasureTemp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tempsense0ClrMeasureTemp {
    #[inline(always)]
    fn from(val: u8) -> Tempsense0ClrMeasureTemp {
        Tempsense0ClrMeasureTemp::from_bits(val)
    }
}
impl From<Tempsense0ClrMeasureTemp> for u8 {
    #[inline(always)]
    fn from(val: Tempsense0ClrMeasureTemp) -> u8 {
        Tempsense0ClrMeasureTemp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tempsense0ClrPowerDown {
    #[doc = "Enable power to the temperature sensor."]
    POWER_UP = 0x0,
    #[doc = "Power down the temperature sensor."]
    POWER_DOWN = 0x01,
}
impl Tempsense0ClrPowerDown {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tempsense0ClrPowerDown {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tempsense0ClrPowerDown {
    #[inline(always)]
    fn from(val: u8) -> Tempsense0ClrPowerDown {
        Tempsense0ClrPowerDown::from_bits(val)
    }
}
impl From<Tempsense0ClrPowerDown> for u8 {
    #[inline(always)]
    fn from(val: Tempsense0ClrPowerDown) -> u8 {
        Tempsense0ClrPowerDown::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tempsense0MeasureTemp {
    #[doc = "Do not start the measurement process."]
    STOP = 0x0,
    #[doc = "Start the measurement process."]
    START = 0x01,
}
impl Tempsense0MeasureTemp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tempsense0MeasureTemp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tempsense0MeasureTemp {
    #[inline(always)]
    fn from(val: u8) -> Tempsense0MeasureTemp {
        Tempsense0MeasureTemp::from_bits(val)
    }
}
impl From<Tempsense0MeasureTemp> for u8 {
    #[inline(always)]
    fn from(val: Tempsense0MeasureTemp) -> u8 {
        Tempsense0MeasureTemp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tempsense0PowerDown {
    #[doc = "Enable power to the temperature sensor."]
    POWER_UP = 0x0,
    #[doc = "Power down the temperature sensor."]
    POWER_DOWN = 0x01,
}
impl Tempsense0PowerDown {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tempsense0PowerDown {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tempsense0PowerDown {
    #[inline(always)]
    fn from(val: u8) -> Tempsense0PowerDown {
        Tempsense0PowerDown::from_bits(val)
    }
}
impl From<Tempsense0PowerDown> for u8 {
    #[inline(always)]
    fn from(val: Tempsense0PowerDown) -> u8 {
        Tempsense0PowerDown::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tempsense0SetMeasureTemp {
    #[doc = "Do not start the measurement process."]
    STOP = 0x0,
    #[doc = "Start the measurement process."]
    START = 0x01,
}
impl Tempsense0SetMeasureTemp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tempsense0SetMeasureTemp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tempsense0SetMeasureTemp {
    #[inline(always)]
    fn from(val: u8) -> Tempsense0SetMeasureTemp {
        Tempsense0SetMeasureTemp::from_bits(val)
    }
}
impl From<Tempsense0SetMeasureTemp> for u8 {
    #[inline(always)]
    fn from(val: Tempsense0SetMeasureTemp) -> u8 {
        Tempsense0SetMeasureTemp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tempsense0SetPowerDown {
    #[doc = "Enable power to the temperature sensor."]
    POWER_UP = 0x0,
    #[doc = "Power down the temperature sensor."]
    POWER_DOWN = 0x01,
}
impl Tempsense0SetPowerDown {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tempsense0SetPowerDown {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tempsense0SetPowerDown {
    #[inline(always)]
    fn from(val: u8) -> Tempsense0SetPowerDown {
        Tempsense0SetPowerDown::from_bits(val)
    }
}
impl From<Tempsense0SetPowerDown> for u8 {
    #[inline(always)]
    fn from(val: Tempsense0SetPowerDown) -> u8 {
        Tempsense0SetPowerDown::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tempsense0TogMeasureTemp {
    #[doc = "Do not start the measurement process."]
    STOP = 0x0,
    #[doc = "Start the measurement process."]
    START = 0x01,
}
impl Tempsense0TogMeasureTemp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tempsense0TogMeasureTemp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tempsense0TogMeasureTemp {
    #[inline(always)]
    fn from(val: u8) -> Tempsense0TogMeasureTemp {
        Tempsense0TogMeasureTemp::from_bits(val)
    }
}
impl From<Tempsense0TogMeasureTemp> for u8 {
    #[inline(always)]
    fn from(val: Tempsense0TogMeasureTemp) -> u8 {
        Tempsense0TogMeasureTemp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tempsense0TogPowerDown {
    #[doc = "Enable power to the temperature sensor."]
    POWER_UP = 0x0,
    #[doc = "Power down the temperature sensor."]
    POWER_DOWN = 0x01,
}
impl Tempsense0TogPowerDown {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tempsense0TogPowerDown {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tempsense0TogPowerDown {
    #[inline(always)]
    fn from(val: u8) -> Tempsense0TogPowerDown {
        Tempsense0TogPowerDown::from_bits(val)
    }
}
impl From<Tempsense0TogPowerDown> for u8 {
    #[inline(always)]
    fn from(val: Tempsense0TogPowerDown) -> u8 {
        Tempsense0TogPowerDown::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Tempsense1ClrMeasureFreq(u16);
impl Tempsense1ClrMeasureFreq {
    #[doc = "Defines a single measurement with no repeat."]
    pub const MEASURE_FREQ_0: Self = Self(0x0);
    #[doc = "Updates the temperature value at a RTC clock rate."]
    pub const MEASURE_FREQ_1: Self = Self(0x01);
    #[doc = "Updates the temperature value at a RTC/2 clock rate."]
    pub const MEASURE_FREQ_2: Self = Self(0x02);
    #[doc = "Determines a two second sample period with a 32.768KHz RTC clock. Exact timings depend on the accuracy of the RTC clock."]
    pub const MEASURE_FREQ_65535: Self = Self(0xffff);
}
impl Tempsense1ClrMeasureFreq {
    pub const fn from_bits(val: u16) -> Tempsense1ClrMeasureFreq {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Tempsense1ClrMeasureFreq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("MEASURE_FREQ_0"),
            0x01 => f.write_str("MEASURE_FREQ_1"),
            0x02 => f.write_str("MEASURE_FREQ_2"),
            0xffff => f.write_str("MEASURE_FREQ_65535"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tempsense1ClrMeasureFreq {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "MEASURE_FREQ_0"),
            0x01 => defmt::write!(f, "MEASURE_FREQ_1"),
            0x02 => defmt::write!(f, "MEASURE_FREQ_2"),
            0xffff => defmt::write!(f, "MEASURE_FREQ_65535"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Tempsense1ClrMeasureFreq {
    #[inline(always)]
    fn from(val: u16) -> Tempsense1ClrMeasureFreq {
        Tempsense1ClrMeasureFreq::from_bits(val)
    }
}
impl From<Tempsense1ClrMeasureFreq> for u16 {
    #[inline(always)]
    fn from(val: Tempsense1ClrMeasureFreq) -> u16 {
        Tempsense1ClrMeasureFreq::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Tempsense1MeasureFreq(u16);
impl Tempsense1MeasureFreq {
    #[doc = "Defines a single measurement with no repeat."]
    pub const MEASURE_FREQ_0: Self = Self(0x0);
    #[doc = "Updates the temperature value at a RTC clock rate."]
    pub const MEASURE_FREQ_1: Self = Self(0x01);
    #[doc = "Updates the temperature value at a RTC/2 clock rate."]
    pub const MEASURE_FREQ_2: Self = Self(0x02);
    #[doc = "Determines a two second sample period with a 32.768KHz RTC clock. Exact timings depend on the accuracy of the RTC clock."]
    pub const MEASURE_FREQ_65535: Self = Self(0xffff);
}
impl Tempsense1MeasureFreq {
    pub const fn from_bits(val: u16) -> Tempsense1MeasureFreq {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Tempsense1MeasureFreq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("MEASURE_FREQ_0"),
            0x01 => f.write_str("MEASURE_FREQ_1"),
            0x02 => f.write_str("MEASURE_FREQ_2"),
            0xffff => f.write_str("MEASURE_FREQ_65535"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tempsense1MeasureFreq {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "MEASURE_FREQ_0"),
            0x01 => defmt::write!(f, "MEASURE_FREQ_1"),
            0x02 => defmt::write!(f, "MEASURE_FREQ_2"),
            0xffff => defmt::write!(f, "MEASURE_FREQ_65535"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Tempsense1MeasureFreq {
    #[inline(always)]
    fn from(val: u16) -> Tempsense1MeasureFreq {
        Tempsense1MeasureFreq::from_bits(val)
    }
}
impl From<Tempsense1MeasureFreq> for u16 {
    #[inline(always)]
    fn from(val: Tempsense1MeasureFreq) -> u16 {
        Tempsense1MeasureFreq::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Tempsense1SetMeasureFreq(u16);
impl Tempsense1SetMeasureFreq {
    #[doc = "Defines a single measurement with no repeat."]
    pub const MEASURE_FREQ_0: Self = Self(0x0);
    #[doc = "Updates the temperature value at a RTC clock rate."]
    pub const MEASURE_FREQ_1: Self = Self(0x01);
    #[doc = "Updates the temperature value at a RTC/2 clock rate."]
    pub const MEASURE_FREQ_2: Self = Self(0x02);
    #[doc = "Determines a two second sample period with a 32.768KHz RTC clock. Exact timings depend on the accuracy of the RTC clock."]
    pub const MEASURE_FREQ_65535: Self = Self(0xffff);
}
impl Tempsense1SetMeasureFreq {
    pub const fn from_bits(val: u16) -> Tempsense1SetMeasureFreq {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Tempsense1SetMeasureFreq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("MEASURE_FREQ_0"),
            0x01 => f.write_str("MEASURE_FREQ_1"),
            0x02 => f.write_str("MEASURE_FREQ_2"),
            0xffff => f.write_str("MEASURE_FREQ_65535"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tempsense1SetMeasureFreq {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "MEASURE_FREQ_0"),
            0x01 => defmt::write!(f, "MEASURE_FREQ_1"),
            0x02 => defmt::write!(f, "MEASURE_FREQ_2"),
            0xffff => defmt::write!(f, "MEASURE_FREQ_65535"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Tempsense1SetMeasureFreq {
    #[inline(always)]
    fn from(val: u16) -> Tempsense1SetMeasureFreq {
        Tempsense1SetMeasureFreq::from_bits(val)
    }
}
impl From<Tempsense1SetMeasureFreq> for u16 {
    #[inline(always)]
    fn from(val: Tempsense1SetMeasureFreq) -> u16 {
        Tempsense1SetMeasureFreq::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Tempsense1TogMeasureFreq(u16);
impl Tempsense1TogMeasureFreq {
    #[doc = "Defines a single measurement with no repeat."]
    pub const MEASURE_FREQ_0: Self = Self(0x0);
    #[doc = "Updates the temperature value at a RTC clock rate."]
    pub const MEASURE_FREQ_1: Self = Self(0x01);
    #[doc = "Updates the temperature value at a RTC/2 clock rate."]
    pub const MEASURE_FREQ_2: Self = Self(0x02);
    #[doc = "Determines a two second sample period with a 32.768KHz RTC clock. Exact timings depend on the accuracy of the RTC clock."]
    pub const MEASURE_FREQ_65535: Self = Self(0xffff);
}
impl Tempsense1TogMeasureFreq {
    pub const fn from_bits(val: u16) -> Tempsense1TogMeasureFreq {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Tempsense1TogMeasureFreq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("MEASURE_FREQ_0"),
            0x01 => f.write_str("MEASURE_FREQ_1"),
            0x02 => f.write_str("MEASURE_FREQ_2"),
            0xffff => f.write_str("MEASURE_FREQ_65535"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tempsense1TogMeasureFreq {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "MEASURE_FREQ_0"),
            0x01 => defmt::write!(f, "MEASURE_FREQ_1"),
            0x02 => defmt::write!(f, "MEASURE_FREQ_2"),
            0xffff => defmt::write!(f, "MEASURE_FREQ_65535"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Tempsense1TogMeasureFreq {
    #[inline(always)]
    fn from(val: u16) -> Tempsense1TogMeasureFreq {
        Tempsense1TogMeasureFreq::from_bits(val)
    }
}
impl From<Tempsense1TogMeasureFreq> for u16 {
    #[inline(always)]
    fn from(val: Tempsense1TogMeasureFreq) -> u16 {
        Tempsense1TogMeasureFreq::to_bits(val)
    }
}
