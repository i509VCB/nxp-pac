#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadOnoffDse {
    #[doc = "HI-Z"]
    DSE_0 = 0x0,
    #[doc = "Dual/Single voltage: 262/260 Ohm @ 1.8V, 247/157 Ohm @ 3.3V"]
    DSE_1 = 0x01,
    #[doc = "Dual/Single voltage: 134/130 Ohm @ 1.8V, 126/78 Ohm @ 3.3V"]
    DSE_2 = 0x02,
    #[doc = "Dual/Single voltage: 88/88 Ohm @ 1.8V, 84/53 Ohm @ 3.3V"]
    DSE_3 = 0x03,
    #[doc = "Dual/Single voltage: 62/65 Ohm @ 1.8V, 57/39 Ohm @ 3.3V"]
    DSE_4 = 0x04,
    #[doc = "Dual/Single voltage: 51/52 Ohm @ 1.8V, 47/32 Ohm @ 3.3V"]
    DSE_5 = 0x05,
    #[doc = "Dual/Single voltage: 43/43 Ohm @ 1.8V, 40/26 Ohm @ 3.3V"]
    DSE_6 = 0x06,
    #[doc = "Dual/Single voltage: 37/37 Ohm @ 1.8V, 34/23 Ohm @ 3.3V"]
    DSE_7 = 0x07,
}
impl SwPadCtlPadOnoffDse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadOnoffDse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadOnoffDse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadOnoffDse {
        SwPadCtlPadOnoffDse::from_bits(val)
    }
}
impl From<SwPadCtlPadOnoffDse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadOnoffDse) -> u8 {
        SwPadCtlPadOnoffDse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadOnoffHys {
    #[doc = "Hysteresis Disabled (CMOS input)"]
    HYS_0_HYSTERESIS_DISABLED = 0x0,
    #[doc = "Hysteresis Enabled (Schmitt Trigger input)"]
    HYS_1_HYSTERESIS_ENABLED = 0x01,
}
impl SwPadCtlPadOnoffHys {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadOnoffHys {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadOnoffHys {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadOnoffHys {
        SwPadCtlPadOnoffHys::from_bits(val)
    }
}
impl From<SwPadCtlPadOnoffHys> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadOnoffHys) -> u8 {
        SwPadCtlPadOnoffHys::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadOnoffOde {
    #[doc = "Open Drain Disabled (Output is CMOS)"]
    ODE_0_OPEN_DRAIN_DISABLED = 0x0,
    #[doc = "Open Drain Enabled (Output is Open Drain)"]
    ODE_1_OPEN_DRAIN_ENABLED = 0x01,
}
impl SwPadCtlPadOnoffOde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadOnoffOde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadOnoffOde {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadOnoffOde {
        SwPadCtlPadOnoffOde::from_bits(val)
    }
}
impl From<SwPadCtlPadOnoffOde> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadOnoffOde) -> u8 {
        SwPadCtlPadOnoffOde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadOnoffPke {
    #[doc = "Pull/Keeper Disabled"]
    PKE_0_PULL_KEEPER_DISABLED = 0x0,
    #[doc = "Pull/Keeper Enabled"]
    PKE_1_PULL_KEEPER_ENABLED = 0x01,
}
impl SwPadCtlPadOnoffPke {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadOnoffPke {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadOnoffPke {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadOnoffPke {
        SwPadCtlPadOnoffPke::from_bits(val)
    }
}
impl From<SwPadCtlPadOnoffPke> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadOnoffPke) -> u8 {
        SwPadCtlPadOnoffPke::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadOnoffPue {
    #[doc = "Keep the previous output value when the output driver is disabled."]
    PUE_0_KEEPER = 0x0,
    #[doc = "Pull-up or pull-down (determined by PUS field)."]
    PUE_1_PULL = 0x01,
}
impl SwPadCtlPadOnoffPue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadOnoffPue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadOnoffPue {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadOnoffPue {
        SwPadCtlPadOnoffPue::from_bits(val)
    }
}
impl From<SwPadCtlPadOnoffPue> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadOnoffPue) -> u8 {
        SwPadCtlPadOnoffPue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadOnoffPus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadOnoffPus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadOnoffPus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadOnoffPus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadOnoffPus {
        SwPadCtlPadOnoffPus::from_bits(val)
    }
}
impl From<SwPadCtlPadOnoffPus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadOnoffPus) -> u8 {
        SwPadCtlPadOnoffPus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadOnoffSpeed {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "100MHz"]
    MEDIUM = 0x02,
    _RESERVED_3 = 0x03,
}
impl SwPadCtlPadOnoffSpeed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadOnoffSpeed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadOnoffSpeed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadOnoffSpeed {
        SwPadCtlPadOnoffSpeed::from_bits(val)
    }
}
impl From<SwPadCtlPadOnoffSpeed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadOnoffSpeed) -> u8 {
        SwPadCtlPadOnoffSpeed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadOnoffSre {
    #[doc = "Slow Slew Rate"]
    SRE_0_SLOW_SLEW_RATE = 0x0,
    #[doc = "Fast Slew Rate"]
    SRE_1_FAST_SLEW_RATE = 0x01,
}
impl SwPadCtlPadOnoffSre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadOnoffSre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadOnoffSre {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadOnoffSre {
        SwPadCtlPadOnoffSre::from_bits(val)
    }
}
impl From<SwPadCtlPadOnoffSre> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadOnoffSre) -> u8 {
        SwPadCtlPadOnoffSre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadPmicOnReqDse {
    #[doc = "HI-Z"]
    DSE_0 = 0x0,
    #[doc = "Dual/Single voltage: 262/260 Ohm @ 1.8V, 247/157 Ohm @ 3.3V"]
    DSE_1 = 0x01,
    #[doc = "Dual/Single voltage: 134/130 Ohm @ 1.8V, 126/78 Ohm @ 3.3V"]
    DSE_2 = 0x02,
    #[doc = "Dual/Single voltage: 88/88 Ohm @ 1.8V, 84/53 Ohm @ 3.3V"]
    DSE_3 = 0x03,
    #[doc = "Dual/Single voltage: 62/65 Ohm @ 1.8V, 57/39 Ohm @ 3.3V"]
    DSE_4 = 0x04,
    #[doc = "Dual/Single voltage: 51/52 Ohm @ 1.8V, 47/32 Ohm @ 3.3V"]
    DSE_5 = 0x05,
    #[doc = "Dual/Single voltage: 43/43 Ohm @ 1.8V, 40/26 Ohm @ 3.3V"]
    DSE_6 = 0x06,
    #[doc = "Dual/Single voltage: 37/37 Ohm @ 1.8V, 34/23 Ohm @ 3.3V"]
    DSE_7 = 0x07,
}
impl SwPadCtlPadPmicOnReqDse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadPmicOnReqDse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadPmicOnReqDse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadPmicOnReqDse {
        SwPadCtlPadPmicOnReqDse::from_bits(val)
    }
}
impl From<SwPadCtlPadPmicOnReqDse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadPmicOnReqDse) -> u8 {
        SwPadCtlPadPmicOnReqDse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadPmicOnReqHys {
    #[doc = "Hysteresis Disabled (CMOS input)"]
    HYS_0_HYSTERESIS_DISABLED = 0x0,
    #[doc = "Hysteresis Enabled (Schmitt Trigger input)"]
    HYS_1_HYSTERESIS_ENABLED = 0x01,
}
impl SwPadCtlPadPmicOnReqHys {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadPmicOnReqHys {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadPmicOnReqHys {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadPmicOnReqHys {
        SwPadCtlPadPmicOnReqHys::from_bits(val)
    }
}
impl From<SwPadCtlPadPmicOnReqHys> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadPmicOnReqHys) -> u8 {
        SwPadCtlPadPmicOnReqHys::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadPmicOnReqOde {
    #[doc = "Open Drain Disabled (Output is CMOS)"]
    ODE_0_OPEN_DRAIN_DISABLED = 0x0,
    #[doc = "Open Drain Enabled (Output is Open Drain)"]
    ODE_1_OPEN_DRAIN_ENABLED = 0x01,
}
impl SwPadCtlPadPmicOnReqOde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadPmicOnReqOde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadPmicOnReqOde {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadPmicOnReqOde {
        SwPadCtlPadPmicOnReqOde::from_bits(val)
    }
}
impl From<SwPadCtlPadPmicOnReqOde> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadPmicOnReqOde) -> u8 {
        SwPadCtlPadPmicOnReqOde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadPmicOnReqPke {
    #[doc = "Pull/Keeper Disabled"]
    PKE_0_PULL_KEEPER_DISABLED = 0x0,
    #[doc = "Pull/Keeper Enabled"]
    PKE_1_PULL_KEEPER_ENABLED = 0x01,
}
impl SwPadCtlPadPmicOnReqPke {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadPmicOnReqPke {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadPmicOnReqPke {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadPmicOnReqPke {
        SwPadCtlPadPmicOnReqPke::from_bits(val)
    }
}
impl From<SwPadCtlPadPmicOnReqPke> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadPmicOnReqPke) -> u8 {
        SwPadCtlPadPmicOnReqPke::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadPmicOnReqPue {
    #[doc = "Keep the previous output value when the output driver is disabled."]
    PUE_0_KEEPER = 0x0,
    #[doc = "Pull-up or pull-down (determined by PUS field)."]
    PUE_1_PULL = 0x01,
}
impl SwPadCtlPadPmicOnReqPue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadPmicOnReqPue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadPmicOnReqPue {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadPmicOnReqPue {
        SwPadCtlPadPmicOnReqPue::from_bits(val)
    }
}
impl From<SwPadCtlPadPmicOnReqPue> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadPmicOnReqPue) -> u8 {
        SwPadCtlPadPmicOnReqPue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadPmicOnReqPus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadPmicOnReqPus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadPmicOnReqPus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadPmicOnReqPus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadPmicOnReqPus {
        SwPadCtlPadPmicOnReqPus::from_bits(val)
    }
}
impl From<SwPadCtlPadPmicOnReqPus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadPmicOnReqPus) -> u8 {
        SwPadCtlPadPmicOnReqPus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadPmicOnReqSpeed {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "100MHz"]
    MEDIUM = 0x02,
    _RESERVED_3 = 0x03,
}
impl SwPadCtlPadPmicOnReqSpeed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadPmicOnReqSpeed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadPmicOnReqSpeed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadPmicOnReqSpeed {
        SwPadCtlPadPmicOnReqSpeed::from_bits(val)
    }
}
impl From<SwPadCtlPadPmicOnReqSpeed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadPmicOnReqSpeed) -> u8 {
        SwPadCtlPadPmicOnReqSpeed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadPmicOnReqSre {
    #[doc = "Slow Slew Rate"]
    SRE_0_SLOW_SLEW_RATE = 0x0,
    #[doc = "Fast Slew Rate"]
    SRE_1_FAST_SLEW_RATE = 0x01,
}
impl SwPadCtlPadPmicOnReqSre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadPmicOnReqSre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadPmicOnReqSre {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadPmicOnReqSre {
        SwPadCtlPadPmicOnReqSre::from_bits(val)
    }
}
impl From<SwPadCtlPadPmicOnReqSre> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadPmicOnReqSre) -> u8 {
        SwPadCtlPadPmicOnReqSre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadPmicStbyReqDse {
    #[doc = "HI-Z"]
    DSE_0 = 0x0,
    #[doc = "Dual/Single voltage: 262/260 Ohm @ 1.8V, 247/157 Ohm @ 3.3V"]
    DSE_1 = 0x01,
    #[doc = "Dual/Single voltage: 134/130 Ohm @ 1.8V, 126/78 Ohm @ 3.3V"]
    DSE_2 = 0x02,
    #[doc = "Dual/Single voltage: 88/88 Ohm @ 1.8V, 84/53 Ohm @ 3.3V"]
    DSE_3 = 0x03,
    #[doc = "Dual/Single voltage: 62/65 Ohm @ 1.8V, 57/39 Ohm @ 3.3V"]
    DSE_4 = 0x04,
    #[doc = "Dual/Single voltage: 51/52 Ohm @ 1.8V, 47/32 Ohm @ 3.3V"]
    DSE_5 = 0x05,
    #[doc = "Dual/Single voltage: 43/43 Ohm @ 1.8V, 40/26 Ohm @ 3.3V"]
    DSE_6 = 0x06,
    #[doc = "Dual/Single voltage: 37/37 Ohm @ 1.8V, 34/23 Ohm @ 3.3V"]
    DSE_7 = 0x07,
}
impl SwPadCtlPadPmicStbyReqDse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadPmicStbyReqDse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadPmicStbyReqDse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadPmicStbyReqDse {
        SwPadCtlPadPmicStbyReqDse::from_bits(val)
    }
}
impl From<SwPadCtlPadPmicStbyReqDse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadPmicStbyReqDse) -> u8 {
        SwPadCtlPadPmicStbyReqDse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadPmicStbyReqHys {
    #[doc = "Hysteresis Disabled (CMOS input)"]
    HYS_0_HYSTERESIS_DISABLED = 0x0,
    #[doc = "Hysteresis Enabled (Schmitt Trigger input)"]
    HYS_1_HYSTERESIS_ENABLED = 0x01,
}
impl SwPadCtlPadPmicStbyReqHys {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadPmicStbyReqHys {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadPmicStbyReqHys {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadPmicStbyReqHys {
        SwPadCtlPadPmicStbyReqHys::from_bits(val)
    }
}
impl From<SwPadCtlPadPmicStbyReqHys> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadPmicStbyReqHys) -> u8 {
        SwPadCtlPadPmicStbyReqHys::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadPmicStbyReqOde {
    #[doc = "Open Drain Disabled (Output is CMOS)"]
    ODE_0_OPEN_DRAIN_DISABLED = 0x0,
    #[doc = "Open Drain Enabled (Output is Open Drain)"]
    ODE_1_OPEN_DRAIN_ENABLED = 0x01,
}
impl SwPadCtlPadPmicStbyReqOde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadPmicStbyReqOde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadPmicStbyReqOde {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadPmicStbyReqOde {
        SwPadCtlPadPmicStbyReqOde::from_bits(val)
    }
}
impl From<SwPadCtlPadPmicStbyReqOde> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadPmicStbyReqOde) -> u8 {
        SwPadCtlPadPmicStbyReqOde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadPmicStbyReqPke {
    #[doc = "Pull/Keeper Disabled"]
    PKE_0_PULL_KEEPER_DISABLED = 0x0,
    #[doc = "Pull/Keeper Enabled"]
    PKE_1_PULL_KEEPER_ENABLED = 0x01,
}
impl SwPadCtlPadPmicStbyReqPke {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadPmicStbyReqPke {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadPmicStbyReqPke {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadPmicStbyReqPke {
        SwPadCtlPadPmicStbyReqPke::from_bits(val)
    }
}
impl From<SwPadCtlPadPmicStbyReqPke> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadPmicStbyReqPke) -> u8 {
        SwPadCtlPadPmicStbyReqPke::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadPmicStbyReqPue {
    #[doc = "Keep the previous output value when the output driver is disabled."]
    PUE_0_KEEPER = 0x0,
    #[doc = "Pull-up or pull-down (determined by PUS field)."]
    PUE_1_PULL = 0x01,
}
impl SwPadCtlPadPmicStbyReqPue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadPmicStbyReqPue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadPmicStbyReqPue {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadPmicStbyReqPue {
        SwPadCtlPadPmicStbyReqPue::from_bits(val)
    }
}
impl From<SwPadCtlPadPmicStbyReqPue> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadPmicStbyReqPue) -> u8 {
        SwPadCtlPadPmicStbyReqPue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadPmicStbyReqPus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadPmicStbyReqPus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadPmicStbyReqPus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadPmicStbyReqPus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadPmicStbyReqPus {
        SwPadCtlPadPmicStbyReqPus::from_bits(val)
    }
}
impl From<SwPadCtlPadPmicStbyReqPus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadPmicStbyReqPus) -> u8 {
        SwPadCtlPadPmicStbyReqPus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadPmicStbyReqSpeed {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "100MHz"]
    MEDIUM = 0x02,
    _RESERVED_3 = 0x03,
}
impl SwPadCtlPadPmicStbyReqSpeed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadPmicStbyReqSpeed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadPmicStbyReqSpeed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadPmicStbyReqSpeed {
        SwPadCtlPadPmicStbyReqSpeed::from_bits(val)
    }
}
impl From<SwPadCtlPadPmicStbyReqSpeed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadPmicStbyReqSpeed) -> u8 {
        SwPadCtlPadPmicStbyReqSpeed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadPmicStbyReqSre {
    #[doc = "Slow Slew Rate"]
    SRE_0_SLOW_SLEW_RATE = 0x0,
    #[doc = "Fast Slew Rate"]
    SRE_1_FAST_SLEW_RATE = 0x01,
}
impl SwPadCtlPadPmicStbyReqSre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadPmicStbyReqSre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadPmicStbyReqSre {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadPmicStbyReqSre {
        SwPadCtlPadPmicStbyReqSre::from_bits(val)
    }
}
impl From<SwPadCtlPadPmicStbyReqSre> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadPmicStbyReqSre) -> u8 {
        SwPadCtlPadPmicStbyReqSre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadPorBDse {
    #[doc = "HI-Z"]
    DSE_0 = 0x0,
    #[doc = "Dual/Single voltage: 262/260 Ohm @ 1.8V, 247/157 Ohm @ 3.3V"]
    DSE_1 = 0x01,
    #[doc = "Dual/Single voltage: 134/130 Ohm @ 1.8V, 126/78 Ohm @ 3.3V"]
    DSE_2 = 0x02,
    #[doc = "Dual/Single voltage: 88/88 Ohm @ 1.8V, 84/53 Ohm @ 3.3V"]
    DSE_3 = 0x03,
    #[doc = "Dual/Single voltage: 62/65 Ohm @ 1.8V, 57/39 Ohm @ 3.3V"]
    DSE_4 = 0x04,
    #[doc = "Dual/Single voltage: 51/52 Ohm @ 1.8V, 47/32 Ohm @ 3.3V"]
    DSE_5 = 0x05,
    #[doc = "Dual/Single voltage: 43/43 Ohm @ 1.8V, 40/26 Ohm @ 3.3V"]
    DSE_6 = 0x06,
    #[doc = "Dual/Single voltage: 37/37 Ohm @ 1.8V, 34/23 Ohm @ 3.3V"]
    DSE_7 = 0x07,
}
impl SwPadCtlPadPorBDse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadPorBDse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadPorBDse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadPorBDse {
        SwPadCtlPadPorBDse::from_bits(val)
    }
}
impl From<SwPadCtlPadPorBDse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadPorBDse) -> u8 {
        SwPadCtlPadPorBDse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadPorBHys {
    #[doc = "Hysteresis Disabled (CMOS input)"]
    HYS_0_HYSTERESIS_DISABLED = 0x0,
    #[doc = "Hysteresis Enabled (Schmitt Trigger input)"]
    HYS_1_HYSTERESIS_ENABLED = 0x01,
}
impl SwPadCtlPadPorBHys {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadPorBHys {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadPorBHys {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadPorBHys {
        SwPadCtlPadPorBHys::from_bits(val)
    }
}
impl From<SwPadCtlPadPorBHys> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadPorBHys) -> u8 {
        SwPadCtlPadPorBHys::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadPorBOde {
    #[doc = "Open Drain Disabled (Output is CMOS)"]
    ODE_0_OPEN_DRAIN_DISABLED = 0x0,
    #[doc = "Open Drain Enabled (Output is Open Drain)"]
    ODE_1_OPEN_DRAIN_ENABLED = 0x01,
}
impl SwPadCtlPadPorBOde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadPorBOde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadPorBOde {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadPorBOde {
        SwPadCtlPadPorBOde::from_bits(val)
    }
}
impl From<SwPadCtlPadPorBOde> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadPorBOde) -> u8 {
        SwPadCtlPadPorBOde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadPorBPke {
    #[doc = "Pull/Keeper Disabled"]
    PKE_0_PULL_KEEPER_DISABLED = 0x0,
    #[doc = "Pull/Keeper Enabled"]
    PKE_1_PULL_KEEPER_ENABLED = 0x01,
}
impl SwPadCtlPadPorBPke {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadPorBPke {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadPorBPke {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadPorBPke {
        SwPadCtlPadPorBPke::from_bits(val)
    }
}
impl From<SwPadCtlPadPorBPke> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadPorBPke) -> u8 {
        SwPadCtlPadPorBPke::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadPorBPue {
    #[doc = "Keep the previous output value when the output driver is disabled."]
    PUE_0_KEEPER = 0x0,
    #[doc = "Pull-up or pull-down (determined by PUS field)."]
    PUE_1_PULL = 0x01,
}
impl SwPadCtlPadPorBPue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadPorBPue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadPorBPue {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadPorBPue {
        SwPadCtlPadPorBPue::from_bits(val)
    }
}
impl From<SwPadCtlPadPorBPue> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadPorBPue) -> u8 {
        SwPadCtlPadPorBPue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadPorBPus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadPorBPus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadPorBPus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadPorBPus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadPorBPus {
        SwPadCtlPadPorBPus::from_bits(val)
    }
}
impl From<SwPadCtlPadPorBPus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadPorBPus) -> u8 {
        SwPadCtlPadPorBPus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadPorBSpeed {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "100MHz"]
    MEDIUM = 0x02,
    _RESERVED_3 = 0x03,
}
impl SwPadCtlPadPorBSpeed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadPorBSpeed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadPorBSpeed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadPorBSpeed {
        SwPadCtlPadPorBSpeed::from_bits(val)
    }
}
impl From<SwPadCtlPadPorBSpeed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadPorBSpeed) -> u8 {
        SwPadCtlPadPorBSpeed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadPorBSre {
    #[doc = "Slow Slew Rate"]
    SRE_0_SLOW_SLEW_RATE = 0x0,
    #[doc = "Fast Slew Rate"]
    SRE_1_FAST_SLEW_RATE = 0x01,
}
impl SwPadCtlPadPorBSre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadPorBSre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadPorBSre {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadPorBSre {
        SwPadCtlPadPorBSre::from_bits(val)
    }
}
impl From<SwPadCtlPadPorBSre> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadPorBSre) -> u8 {
        SwPadCtlPadPorBSre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadTestModeDse {
    #[doc = "HI-Z"]
    DSE_0 = 0x0,
    #[doc = "Dual/Single voltage: 262/260 Ohm @ 1.8V, 247/157 Ohm @ 3.3V"]
    DSE_1 = 0x01,
    #[doc = "Dual/Single voltage: 134/130 Ohm @ 1.8V, 126/78 Ohm @ 3.3V"]
    DSE_2 = 0x02,
    #[doc = "Dual/Single voltage: 88/88 Ohm @ 1.8V, 84/53 Ohm @ 3.3V"]
    DSE_3 = 0x03,
    #[doc = "Dual/Single voltage: 62/65 Ohm @ 1.8V, 57/39 Ohm @ 3.3V"]
    DSE_4 = 0x04,
    #[doc = "Dual/Single voltage: 51/52 Ohm @ 1.8V, 47/32 Ohm @ 3.3V"]
    DSE_5 = 0x05,
    #[doc = "Dual/Single voltage: 43/43 Ohm @ 1.8V, 40/26 Ohm @ 3.3V"]
    DSE_6 = 0x06,
    #[doc = "Dual/Single voltage: 37/37 Ohm @ 1.8V, 34/23 Ohm @ 3.3V"]
    DSE_7 = 0x07,
}
impl SwPadCtlPadTestModeDse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadTestModeDse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadTestModeDse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadTestModeDse {
        SwPadCtlPadTestModeDse::from_bits(val)
    }
}
impl From<SwPadCtlPadTestModeDse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadTestModeDse) -> u8 {
        SwPadCtlPadTestModeDse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadTestModeHys {
    #[doc = "Hysteresis Disabled (CMOS input)"]
    HYS_0_HYSTERESIS_DISABLED = 0x0,
    #[doc = "Hysteresis Enabled (Schmitt Trigger input)"]
    HYS_1_HYSTERESIS_ENABLED = 0x01,
}
impl SwPadCtlPadTestModeHys {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadTestModeHys {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadTestModeHys {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadTestModeHys {
        SwPadCtlPadTestModeHys::from_bits(val)
    }
}
impl From<SwPadCtlPadTestModeHys> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadTestModeHys) -> u8 {
        SwPadCtlPadTestModeHys::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadTestModeOde {
    #[doc = "Open Drain Disabled (Output is CMOS)"]
    ODE_0_OPEN_DRAIN_DISABLED = 0x0,
    #[doc = "Open Drain Enabled (Output is Open Drain)"]
    ODE_1_OPEN_DRAIN_ENABLED = 0x01,
}
impl SwPadCtlPadTestModeOde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadTestModeOde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadTestModeOde {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadTestModeOde {
        SwPadCtlPadTestModeOde::from_bits(val)
    }
}
impl From<SwPadCtlPadTestModeOde> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadTestModeOde) -> u8 {
        SwPadCtlPadTestModeOde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadTestModePke {
    #[doc = "Pull/Keeper Disabled"]
    PKE_0_PULL_KEEPER_DISABLED = 0x0,
    #[doc = "Pull/Keeper Enabled"]
    PKE_1_PULL_KEEPER_ENABLED = 0x01,
}
impl SwPadCtlPadTestModePke {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadTestModePke {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadTestModePke {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadTestModePke {
        SwPadCtlPadTestModePke::from_bits(val)
    }
}
impl From<SwPadCtlPadTestModePke> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadTestModePke) -> u8 {
        SwPadCtlPadTestModePke::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadTestModePue {
    #[doc = "Keep the previous output value when the output driver is disabled."]
    PUE_0_KEEPER = 0x0,
    #[doc = "Pull-up or pull-down (determined by PUS field)."]
    PUE_1_PULL = 0x01,
}
impl SwPadCtlPadTestModePue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadTestModePue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadTestModePue {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadTestModePue {
        SwPadCtlPadTestModePue::from_bits(val)
    }
}
impl From<SwPadCtlPadTestModePue> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadTestModePue) -> u8 {
        SwPadCtlPadTestModePue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadTestModePus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadTestModePus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadTestModePus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadTestModePus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadTestModePus {
        SwPadCtlPadTestModePus::from_bits(val)
    }
}
impl From<SwPadCtlPadTestModePus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadTestModePus) -> u8 {
        SwPadCtlPadTestModePus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadTestModeSpeed {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "100MHz"]
    MEDIUM = 0x02,
    _RESERVED_3 = 0x03,
}
impl SwPadCtlPadTestModeSpeed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadTestModeSpeed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadTestModeSpeed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadTestModeSpeed {
        SwPadCtlPadTestModeSpeed::from_bits(val)
    }
}
impl From<SwPadCtlPadTestModeSpeed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadTestModeSpeed) -> u8 {
        SwPadCtlPadTestModeSpeed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadTestModeSre {
    #[doc = "Slow Slew Rate"]
    SRE_0_SLOW_SLEW_RATE = 0x0,
    #[doc = "Fast Slew Rate"]
    SRE_1_FAST_SLEW_RATE = 0x01,
}
impl SwPadCtlPadTestModeSre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadTestModeSre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadTestModeSre {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadTestModeSre {
        SwPadCtlPadTestModeSre::from_bits(val)
    }
}
impl From<SwPadCtlPadTestModeSre> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadTestModeSre) -> u8 {
        SwPadCtlPadTestModeSre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadWakeupDse {
    #[doc = "HI-Z"]
    DSE_0 = 0x0,
    #[doc = "Dual/Single voltage: 262/260 Ohm @ 1.8V, 247/157 Ohm @ 3.3V"]
    DSE_1 = 0x01,
    #[doc = "Dual/Single voltage: 134/130 Ohm @ 1.8V, 126/78 Ohm @ 3.3V"]
    DSE_2 = 0x02,
    #[doc = "Dual/Single voltage: 88/88 Ohm @ 1.8V, 84/53 Ohm @ 3.3V"]
    DSE_3 = 0x03,
    #[doc = "Dual/Single voltage: 62/65 Ohm @ 1.8V, 57/39 Ohm @ 3.3V"]
    DSE_4 = 0x04,
    #[doc = "Dual/Single voltage: 51/52 Ohm @ 1.8V, 47/32 Ohm @ 3.3V"]
    DSE_5 = 0x05,
    #[doc = "Dual/Single voltage: 43/43 Ohm @ 1.8V, 40/26 Ohm @ 3.3V"]
    DSE_6 = 0x06,
    #[doc = "Dual/Single voltage: 37/37 Ohm @ 1.8V, 34/23 Ohm @ 3.3V"]
    DSE_7 = 0x07,
}
impl SwPadCtlPadWakeupDse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadWakeupDse {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadWakeupDse {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadWakeupDse {
        SwPadCtlPadWakeupDse::from_bits(val)
    }
}
impl From<SwPadCtlPadWakeupDse> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadWakeupDse) -> u8 {
        SwPadCtlPadWakeupDse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadWakeupHys {
    #[doc = "Hysteresis Disabled (CMOS input)"]
    HYS_0_HYSTERESIS_DISABLED = 0x0,
    #[doc = "Hysteresis Enabled (Schmitt Trigger input)"]
    HYS_1_HYSTERESIS_ENABLED = 0x01,
}
impl SwPadCtlPadWakeupHys {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadWakeupHys {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadWakeupHys {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadWakeupHys {
        SwPadCtlPadWakeupHys::from_bits(val)
    }
}
impl From<SwPadCtlPadWakeupHys> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadWakeupHys) -> u8 {
        SwPadCtlPadWakeupHys::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadWakeupOde {
    #[doc = "Open Drain Disabled (Output is CMOS)"]
    ODE_0_OPEN_DRAIN_DISABLED = 0x0,
    #[doc = "Open Drain Enabled (Output is Open Drain)"]
    ODE_1_OPEN_DRAIN_ENABLED = 0x01,
}
impl SwPadCtlPadWakeupOde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadWakeupOde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadWakeupOde {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadWakeupOde {
        SwPadCtlPadWakeupOde::from_bits(val)
    }
}
impl From<SwPadCtlPadWakeupOde> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadWakeupOde) -> u8 {
        SwPadCtlPadWakeupOde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadWakeupPke {
    #[doc = "Pull/Keeper Disabled"]
    PKE_0_PULL_KEEPER_DISABLED = 0x0,
    #[doc = "Pull/Keeper Enabled"]
    PKE_1_PULL_KEEPER_ENABLED = 0x01,
}
impl SwPadCtlPadWakeupPke {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadWakeupPke {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadWakeupPke {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadWakeupPke {
        SwPadCtlPadWakeupPke::from_bits(val)
    }
}
impl From<SwPadCtlPadWakeupPke> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadWakeupPke) -> u8 {
        SwPadCtlPadWakeupPke::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadWakeupPue {
    #[doc = "Keep the previous output value when the output driver is disabled."]
    PUE_0_KEEPER = 0x0,
    #[doc = "Pull-up or pull-down (determined by PUS field)."]
    PUE_1_PULL = 0x01,
}
impl SwPadCtlPadWakeupPue {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadWakeupPue {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadWakeupPue {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadWakeupPue {
        SwPadCtlPadWakeupPue::from_bits(val)
    }
}
impl From<SwPadCtlPadWakeupPue> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadWakeupPue) -> u8 {
        SwPadCtlPadWakeupPue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadWakeupPus {
    #[doc = "100K Ohm Pull Down"]
    PUS_0_100K_OHM_PULL_DOWN = 0x0,
    #[doc = "47K Ohm Pull Up"]
    PUS_1_47K_OHM_PULL_UP = 0x01,
    #[doc = "100K Ohm Pull Up"]
    PUS_2_100K_OHM_PULL_UP = 0x02,
    #[doc = "22K Ohm Pull Up"]
    PUS_3_22K_OHM_PULL_UP = 0x03,
}
impl SwPadCtlPadWakeupPus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadWakeupPus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadWakeupPus {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadWakeupPus {
        SwPadCtlPadWakeupPus::from_bits(val)
    }
}
impl From<SwPadCtlPadWakeupPus> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadWakeupPus) -> u8 {
        SwPadCtlPadWakeupPus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadWakeupSpeed {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "100MHz"]
    MEDIUM = 0x02,
    _RESERVED_3 = 0x03,
}
impl SwPadCtlPadWakeupSpeed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadWakeupSpeed {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadWakeupSpeed {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadWakeupSpeed {
        SwPadCtlPadWakeupSpeed::from_bits(val)
    }
}
impl From<SwPadCtlPadWakeupSpeed> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadWakeupSpeed) -> u8 {
        SwPadCtlPadWakeupSpeed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadWakeupSre {
    #[doc = "Slow Slew Rate"]
    SRE_0_SLOW_SLEW_RATE = 0x0,
    #[doc = "Fast Slew Rate"]
    SRE_1_FAST_SLEW_RATE = 0x01,
}
impl SwPadCtlPadWakeupSre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwPadCtlPadWakeupSre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwPadCtlPadWakeupSre {
    #[inline(always)]
    fn from(val: u8) -> SwPadCtlPadWakeupSre {
        SwPadCtlPadWakeupSre::from_bits(val)
    }
}
impl From<SwPadCtlPadWakeupSre> for u8 {
    #[inline(always)]
    fn from(val: SwPadCtlPadWakeupSre) -> u8 {
        SwPadCtlPadWakeupSre::to_bits(val)
    }
}
