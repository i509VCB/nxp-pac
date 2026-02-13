#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MuxMode {
    #[doc = "Select mux mode: ALT0 mux port: SNVS_LP_PMIC_ON_REQ of instance: snvs_lp"]
    ALT0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Select mux mode: ALT5 mux port: GPIO5_IO00 of instance: gpio5"]
    ALT5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MuxMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MuxMode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MuxMode {
    #[inline(always)]
    fn from(val: u8) -> MuxMode {
        MuxMode::from_bits(val)
    }
}
impl From<MuxMode> for u8 {
    #[inline(always)]
    fn from(val: MuxMode) -> u8 {
        MuxMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwPadCtlPadOnoffDse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED_ = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V = 0x01,
    #[doc = "R0/2"]
    DSE_2_R0_2 = 0x02,
    #[doc = "R0/3"]
    DSE_3_R0_3 = 0x03,
    #[doc = "R0/4"]
    DSE_4_R0_4 = 0x04,
    #[doc = "R0/5"]
    DSE_5_R0_5 = 0x05,
    #[doc = "R0/6"]
    DSE_6_R0_6 = 0x06,
    #[doc = "R0/7"]
    DSE_7_R0_7 = 0x07,
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
    #[doc = "Hysteresis Disabled"]
    HYS_0_HYSTERESIS_DISABLED = 0x0,
    #[doc = "Hysteresis Enabled"]
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
    #[doc = "Open Drain Disabled"]
    ODE_0_OPEN_DRAIN_DISABLED = 0x0,
    #[doc = "Open Drain Enabled"]
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
    #[doc = "Keeper"]
    PUE_0_KEEPER = 0x0,
    #[doc = "Pull"]
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
    #[doc = "medium(100MHz)"]
    SPEED = 0x02,
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
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED_ = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V = 0x01,
    #[doc = "R0/2"]
    DSE_2_R0_2 = 0x02,
    #[doc = "R0/3"]
    DSE_3_R0_3 = 0x03,
    #[doc = "R0/4"]
    DSE_4_R0_4 = 0x04,
    #[doc = "R0/5"]
    DSE_5_R0_5 = 0x05,
    #[doc = "R0/6"]
    DSE_6_R0_6 = 0x06,
    #[doc = "R0/7"]
    DSE_7_R0_7 = 0x07,
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
    #[doc = "Hysteresis Disabled"]
    HYS_0_HYSTERESIS_DISABLED = 0x0,
    #[doc = "Hysteresis Enabled"]
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
    #[doc = "Open Drain Disabled"]
    ODE_0_OPEN_DRAIN_DISABLED = 0x0,
    #[doc = "Open Drain Enabled"]
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
    #[doc = "Keeper"]
    PUE_0_KEEPER = 0x0,
    #[doc = "Pull"]
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
    #[doc = "medium(100MHz)"]
    SPEED = 0x02,
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
pub enum SwPadCtlPadPorBDse {
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED_ = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V = 0x01,
    #[doc = "R0/2"]
    DSE_2_R0_2 = 0x02,
    #[doc = "R0/3"]
    DSE_3_R0_3 = 0x03,
    #[doc = "R0/4"]
    DSE_4_R0_4 = 0x04,
    #[doc = "R0/5"]
    DSE_5_R0_5 = 0x05,
    #[doc = "R0/6"]
    DSE_6_R0_6 = 0x06,
    #[doc = "R0/7"]
    DSE_7_R0_7 = 0x07,
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
    #[doc = "Hysteresis Disabled"]
    HYS_0_HYSTERESIS_DISABLED = 0x0,
    #[doc = "Hysteresis Enabled"]
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
    #[doc = "Open Drain Disabled"]
    ODE_0_OPEN_DRAIN_DISABLED = 0x0,
    #[doc = "Open Drain Enabled"]
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
    #[doc = "Keeper"]
    PUE_0_KEEPER = 0x0,
    #[doc = "Pull"]
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
    #[doc = "medium(100MHz)"]
    SPEED = 0x02,
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
    #[doc = "output driver disabled;"]
    DSE_0_OUTPUT_DRIVER_DISABLED_ = 0x0,
    #[doc = "R0(150 Ohm @ 3.3V, 260 Ohm@1.8V)"]
    DSE_1_R0_150_OHM___3_3V__260_OHM_1_8V = 0x01,
    #[doc = "R0/2"]
    DSE_2_R0_2 = 0x02,
    #[doc = "R0/3"]
    DSE_3_R0_3 = 0x03,
    #[doc = "R0/4"]
    DSE_4_R0_4 = 0x04,
    #[doc = "R0/5"]
    DSE_5_R0_5 = 0x05,
    #[doc = "R0/6"]
    DSE_6_R0_6 = 0x06,
    #[doc = "R0/7"]
    DSE_7_R0_7 = 0x07,
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
    #[doc = "Hysteresis Disabled"]
    HYS_0_HYSTERESIS_DISABLED = 0x0,
    #[doc = "Hysteresis Enabled"]
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
    #[doc = "Open Drain Disabled"]
    ODE_0_OPEN_DRAIN_DISABLED = 0x0,
    #[doc = "Open Drain Enabled"]
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
    #[doc = "Keeper"]
    PUE_0_KEEPER = 0x0,
    #[doc = "Pull"]
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
    #[doc = "medium(100MHz)"]
    SPEED = 0x02,
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
