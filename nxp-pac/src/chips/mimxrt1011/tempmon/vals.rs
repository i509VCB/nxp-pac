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
