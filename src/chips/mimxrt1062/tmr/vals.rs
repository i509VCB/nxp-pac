#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csctrl0Cl1 {
    #[doc = "Never preload"]
    NEVER = 0x0,
    #[doc = "Load upon successful compare with the value in COMP1"]
    COMP1 = 0x01,
    #[doc = "Load upon successful compare with the value in COMP2"]
    COMP2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Csctrl0Cl1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csctrl0Cl1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csctrl0Cl1 {
    #[inline(always)]
    fn from(val: u8) -> Csctrl0Cl1 {
        Csctrl0Cl1::from_bits(val)
    }
}
impl From<Csctrl0Cl1> for u8 {
    #[inline(always)]
    fn from(val: Csctrl0Cl1) -> u8 {
        Csctrl0Cl1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csctrl0Cl2 {
    #[doc = "Never preload"]
    NEVER = 0x0,
    #[doc = "Load upon successful compare with the value in COMP1"]
    COMP1 = 0x01,
    #[doc = "Load upon successful compare with the value in COMP2"]
    COMP2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Csctrl0Cl2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csctrl0Cl2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csctrl0Cl2 {
    #[inline(always)]
    fn from(val: u8) -> Csctrl0Cl2 {
        Csctrl0Cl2::from_bits(val)
    }
}
impl From<Csctrl0Cl2> for u8 {
    #[inline(always)]
    fn from(val: Csctrl0Cl2) -> u8 {
        Csctrl0Cl2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csctrl0DbgEn {
    #[doc = "Continue with normal operation during debug mode. (default)"]
    NORMAL = 0x0,
    #[doc = "Halt TMR counter during debug mode."]
    HALT_TMR = 0x01,
    #[doc = "Force TMR output to logic 0 (prior to consideration of SCTRL\\[OPS\\])."]
    FORCE_0 = 0x02,
    #[doc = "Both halt counter and force output to 0 during debug mode."]
    HALT_AND_FORCE_0 = 0x03,
}
impl Csctrl0DbgEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csctrl0DbgEn {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csctrl0DbgEn {
    #[inline(always)]
    fn from(val: u8) -> Csctrl0DbgEn {
        Csctrl0DbgEn::from_bits(val)
    }
}
impl From<Csctrl0DbgEn> for u8 {
    #[inline(always)]
    fn from(val: Csctrl0DbgEn) -> u8 {
        Csctrl0DbgEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csctrl0Tci {
    #[doc = "Stop counter upon receiving a second trigger event while still counting from the first trigger event."]
    STOP = 0x0,
    #[doc = "Reload the counter upon receiving a second trigger event while still counting from the first trigger event."]
    RELOAD = 0x01,
}
impl Csctrl0Tci {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csctrl0Tci {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csctrl0Tci {
    #[inline(always)]
    fn from(val: u8) -> Csctrl0Tci {
        Csctrl0Tci::from_bits(val)
    }
}
impl From<Csctrl0Tci> for u8 {
    #[inline(always)]
    fn from(val: Csctrl0Tci) -> u8 {
        Csctrl0Tci::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csctrl0Up {
    #[doc = "The last count was in the DOWN direction."]
    DOWN = 0x0,
    #[doc = "The last count was in the UP direction."]
    UP = 0x01,
}
impl Csctrl0Up {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csctrl0Up {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csctrl0Up {
    #[inline(always)]
    fn from(val: u8) -> Csctrl0Up {
        Csctrl0Up::from_bits(val)
    }
}
impl From<Csctrl0Up> for u8 {
    #[inline(always)]
    fn from(val: Csctrl0Up) -> u8 {
        Csctrl0Up::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csctrl1Cl1 {
    #[doc = "Never preload"]
    NEVER = 0x0,
    #[doc = "Load upon successful compare with the value in COMP1"]
    COMP1 = 0x01,
    #[doc = "Load upon successful compare with the value in COMP2"]
    COMP2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Csctrl1Cl1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csctrl1Cl1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csctrl1Cl1 {
    #[inline(always)]
    fn from(val: u8) -> Csctrl1Cl1 {
        Csctrl1Cl1::from_bits(val)
    }
}
impl From<Csctrl1Cl1> for u8 {
    #[inline(always)]
    fn from(val: Csctrl1Cl1) -> u8 {
        Csctrl1Cl1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csctrl1Cl2 {
    #[doc = "Never preload"]
    NEVER = 0x0,
    #[doc = "Load upon successful compare with the value in COMP1"]
    COMP1 = 0x01,
    #[doc = "Load upon successful compare with the value in COMP2"]
    COMP2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Csctrl1Cl2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csctrl1Cl2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csctrl1Cl2 {
    #[inline(always)]
    fn from(val: u8) -> Csctrl1Cl2 {
        Csctrl1Cl2::from_bits(val)
    }
}
impl From<Csctrl1Cl2> for u8 {
    #[inline(always)]
    fn from(val: Csctrl1Cl2) -> u8 {
        Csctrl1Cl2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csctrl1DbgEn {
    #[doc = "Continue with normal operation during debug mode. (default)"]
    NORMAL = 0x0,
    #[doc = "Halt TMR counter during debug mode."]
    HALT_TMR = 0x01,
    #[doc = "Force TMR output to logic 0 (prior to consideration of SCTRL\\[OPS\\])."]
    FORCE_0 = 0x02,
    #[doc = "Both halt counter and force output to 0 during debug mode."]
    HALT_AND_FORCE_0 = 0x03,
}
impl Csctrl1DbgEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csctrl1DbgEn {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csctrl1DbgEn {
    #[inline(always)]
    fn from(val: u8) -> Csctrl1DbgEn {
        Csctrl1DbgEn::from_bits(val)
    }
}
impl From<Csctrl1DbgEn> for u8 {
    #[inline(always)]
    fn from(val: Csctrl1DbgEn) -> u8 {
        Csctrl1DbgEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csctrl1Tci {
    #[doc = "Stop counter upon receiving a second trigger event while still counting from the first trigger event."]
    STOP = 0x0,
    #[doc = "Reload the counter upon receiving a second trigger event while still counting from the first trigger event."]
    RELOAD = 0x01,
}
impl Csctrl1Tci {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csctrl1Tci {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csctrl1Tci {
    #[inline(always)]
    fn from(val: u8) -> Csctrl1Tci {
        Csctrl1Tci::from_bits(val)
    }
}
impl From<Csctrl1Tci> for u8 {
    #[inline(always)]
    fn from(val: Csctrl1Tci) -> u8 {
        Csctrl1Tci::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csctrl1Up {
    #[doc = "The last count was in the DOWN direction."]
    DOWN = 0x0,
    #[doc = "The last count was in the UP direction."]
    UP = 0x01,
}
impl Csctrl1Up {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csctrl1Up {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csctrl1Up {
    #[inline(always)]
    fn from(val: u8) -> Csctrl1Up {
        Csctrl1Up::from_bits(val)
    }
}
impl From<Csctrl1Up> for u8 {
    #[inline(always)]
    fn from(val: Csctrl1Up) -> u8 {
        Csctrl1Up::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csctrl2Cl1 {
    #[doc = "Never preload"]
    NEVER = 0x0,
    #[doc = "Load upon successful compare with the value in COMP1"]
    COMP1 = 0x01,
    #[doc = "Load upon successful compare with the value in COMP2"]
    COMP2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Csctrl2Cl1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csctrl2Cl1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csctrl2Cl1 {
    #[inline(always)]
    fn from(val: u8) -> Csctrl2Cl1 {
        Csctrl2Cl1::from_bits(val)
    }
}
impl From<Csctrl2Cl1> for u8 {
    #[inline(always)]
    fn from(val: Csctrl2Cl1) -> u8 {
        Csctrl2Cl1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csctrl2Cl2 {
    #[doc = "Never preload"]
    NEVER = 0x0,
    #[doc = "Load upon successful compare with the value in COMP1"]
    COMP1 = 0x01,
    #[doc = "Load upon successful compare with the value in COMP2"]
    COMP2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Csctrl2Cl2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csctrl2Cl2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csctrl2Cl2 {
    #[inline(always)]
    fn from(val: u8) -> Csctrl2Cl2 {
        Csctrl2Cl2::from_bits(val)
    }
}
impl From<Csctrl2Cl2> for u8 {
    #[inline(always)]
    fn from(val: Csctrl2Cl2) -> u8 {
        Csctrl2Cl2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csctrl2DbgEn {
    #[doc = "Continue with normal operation during debug mode. (default)"]
    NORMAL = 0x0,
    #[doc = "Halt TMR counter during debug mode."]
    HALT_TMR = 0x01,
    #[doc = "Force TMR output to logic 0 (prior to consideration of SCTRL\\[OPS\\])."]
    FORCE_0 = 0x02,
    #[doc = "Both halt counter and force output to 0 during debug mode."]
    HALT_AND_FORCE_0 = 0x03,
}
impl Csctrl2DbgEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csctrl2DbgEn {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csctrl2DbgEn {
    #[inline(always)]
    fn from(val: u8) -> Csctrl2DbgEn {
        Csctrl2DbgEn::from_bits(val)
    }
}
impl From<Csctrl2DbgEn> for u8 {
    #[inline(always)]
    fn from(val: Csctrl2DbgEn) -> u8 {
        Csctrl2DbgEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csctrl2Tci {
    #[doc = "Stop counter upon receiving a second trigger event while still counting from the first trigger event."]
    STOP = 0x0,
    #[doc = "Reload the counter upon receiving a second trigger event while still counting from the first trigger event."]
    RELOAD = 0x01,
}
impl Csctrl2Tci {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csctrl2Tci {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csctrl2Tci {
    #[inline(always)]
    fn from(val: u8) -> Csctrl2Tci {
        Csctrl2Tci::from_bits(val)
    }
}
impl From<Csctrl2Tci> for u8 {
    #[inline(always)]
    fn from(val: Csctrl2Tci) -> u8 {
        Csctrl2Tci::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csctrl2Up {
    #[doc = "The last count was in the DOWN direction."]
    DOWN = 0x0,
    #[doc = "The last count was in the UP direction."]
    UP = 0x01,
}
impl Csctrl2Up {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csctrl2Up {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csctrl2Up {
    #[inline(always)]
    fn from(val: u8) -> Csctrl2Up {
        Csctrl2Up::from_bits(val)
    }
}
impl From<Csctrl2Up> for u8 {
    #[inline(always)]
    fn from(val: Csctrl2Up) -> u8 {
        Csctrl2Up::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csctrl3Cl1 {
    #[doc = "Never preload"]
    NEVER = 0x0,
    #[doc = "Load upon successful compare with the value in COMP1"]
    COMP1 = 0x01,
    #[doc = "Load upon successful compare with the value in COMP2"]
    COMP2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Csctrl3Cl1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csctrl3Cl1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csctrl3Cl1 {
    #[inline(always)]
    fn from(val: u8) -> Csctrl3Cl1 {
        Csctrl3Cl1::from_bits(val)
    }
}
impl From<Csctrl3Cl1> for u8 {
    #[inline(always)]
    fn from(val: Csctrl3Cl1) -> u8 {
        Csctrl3Cl1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csctrl3Cl2 {
    #[doc = "Never preload"]
    NEVER = 0x0,
    #[doc = "Load upon successful compare with the value in COMP1"]
    COMP1 = 0x01,
    #[doc = "Load upon successful compare with the value in COMP2"]
    COMP2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Csctrl3Cl2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csctrl3Cl2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csctrl3Cl2 {
    #[inline(always)]
    fn from(val: u8) -> Csctrl3Cl2 {
        Csctrl3Cl2::from_bits(val)
    }
}
impl From<Csctrl3Cl2> for u8 {
    #[inline(always)]
    fn from(val: Csctrl3Cl2) -> u8 {
        Csctrl3Cl2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csctrl3DbgEn {
    #[doc = "Continue with normal operation during debug mode. (default)"]
    NORMAL = 0x0,
    #[doc = "Halt TMR counter during debug mode."]
    HALT_TMR = 0x01,
    #[doc = "Force TMR output to logic 0 (prior to consideration of SCTRL\\[OPS\\])."]
    FORCE_0 = 0x02,
    #[doc = "Both halt counter and force output to 0 during debug mode."]
    HALT_AND_FORCE_0 = 0x03,
}
impl Csctrl3DbgEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csctrl3DbgEn {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csctrl3DbgEn {
    #[inline(always)]
    fn from(val: u8) -> Csctrl3DbgEn {
        Csctrl3DbgEn::from_bits(val)
    }
}
impl From<Csctrl3DbgEn> for u8 {
    #[inline(always)]
    fn from(val: Csctrl3DbgEn) -> u8 {
        Csctrl3DbgEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csctrl3Tci {
    #[doc = "Stop counter upon receiving a second trigger event while still counting from the first trigger event."]
    STOP = 0x0,
    #[doc = "Reload the counter upon receiving a second trigger event while still counting from the first trigger event."]
    RELOAD = 0x01,
}
impl Csctrl3Tci {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csctrl3Tci {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csctrl3Tci {
    #[inline(always)]
    fn from(val: u8) -> Csctrl3Tci {
        Csctrl3Tci::from_bits(val)
    }
}
impl From<Csctrl3Tci> for u8 {
    #[inline(always)]
    fn from(val: Csctrl3Tci) -> u8 {
        Csctrl3Tci::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csctrl3Up {
    #[doc = "The last count was in the DOWN direction."]
    DOWN = 0x0,
    #[doc = "The last count was in the UP direction."]
    UP = 0x01,
}
impl Csctrl3Up {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csctrl3Up {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csctrl3Up {
    #[inline(always)]
    fn from(val: u8) -> Csctrl3Up {
        Csctrl3Up::from_bits(val)
    }
}
impl From<Csctrl3Up> for u8 {
    #[inline(always)]
    fn from(val: Csctrl3Up) -> u8 {
        Csctrl3Up::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl0Cm {
    #[doc = "No operation"]
    NOOP = 0x0,
    #[doc = "Count rising edges of primary sourceRising edges are counted only when SCTRL\\[IPS\\] = 0. Falling edges are counted when SCTRL\\[IPS\\] = 1. If the primary count source is IP bus clock divide by 1, only rising edges are counted regardless of the value of SCTRL\\[IPS\\]."]
    RISING_ONLY = 0x01,
    #[doc = "Count rising and falling edges of primary sourceIP bus clock divide by 1 cannot be used as a primary count source in edge count mode."]
    RISING_AND_FALLING = 0x02,
    #[doc = "Count rising edges of primary source while secondary input high active"]
    RISING_WHILE_SEC_HIGH = 0x03,
    #[doc = "Quadrature count mode, uses primary and secondary sources"]
    QUADRATURE = 0x04,
    #[doc = "Count rising edges of primary source; secondary source specifies directionRising edges are counted only when SCTRL\\[IPS\\] = 0. Falling edges are counted when SCTRL\\[IPS\\] = 1."]
    RISING_SEC_DIR = 0x05,
    #[doc = "Edge of secondary source triggers primary count until compare"]
    SECONDARY = 0x06,
    #[doc = "Cascaded counter mode (up/down)The primary count source must be set to one of the counter outputs."]
    CASCADE = 0x07,
}
impl Ctrl0Cm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl0Cm {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl0Cm {
    #[inline(always)]
    fn from(val: u8) -> Ctrl0Cm {
        Ctrl0Cm::from_bits(val)
    }
}
impl From<Ctrl0Cm> for u8 {
    #[inline(always)]
    fn from(val: Ctrl0Cm) -> u8 {
        Ctrl0Cm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl0Dir {
    #[doc = "Count up."]
    COUNTUP = 0x0,
    #[doc = "Count down."]
    COUNTDOWN = 0x01,
}
impl Ctrl0Dir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl0Dir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl0Dir {
    #[inline(always)]
    fn from(val: u8) -> Ctrl0Dir {
        Ctrl0Dir::from_bits(val)
    }
}
impl From<Ctrl0Dir> for u8 {
    #[inline(always)]
    fn from(val: Ctrl0Dir) -> u8 {
        Ctrl0Dir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl0Length {
    #[doc = "Count until roll over at $FFFF and continue from $0000."]
    UNTIL_ROLLOVER = 0x0,
    #[doc = "Count until compare, then re-initialize. If counting up, a successful compare occurs when the counter reaches a COMP1 value. If counting down, a successful compare occurs when the counter reaches a COMP2 value. When output mode $4 is used, alternating values of COMP1 and COMP2 are used to generate successful comparisons. For example, the counter counts until a COMP1 value is reached, re-initializes, counts until COMP2 value is reached, re-initializes, counts until COMP1 value is reached, and so on."]
    UNTIL_COMPARE = 0x01,
}
impl Ctrl0Length {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl0Length {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl0Length {
    #[inline(always)]
    fn from(val: u8) -> Ctrl0Length {
        Ctrl0Length::from_bits(val)
    }
}
impl From<Ctrl0Length> for u8 {
    #[inline(always)]
    fn from(val: Ctrl0Length) -> u8 {
        Ctrl0Length::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl0Once {
    #[doc = "Count repeatedly."]
    REPEAT = 0x0,
    #[doc = "Count until compare and then stop. If counting up, a successful compare occurs when the counter reaches a COMP1 value. If counting down, a successful compare occurs when the counter reaches a COMP2 value. When output mode $4 is used, the counter re-initializes after reaching the COMP1 value, continues to count to the COMP2 value, and then stops."]
    UNTIL_COMPARE = 0x01,
}
impl Ctrl0Once {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl0Once {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl0Once {
    #[inline(always)]
    fn from(val: u8) -> Ctrl0Once {
        Ctrl0Once::from_bits(val)
    }
}
impl From<Ctrl0Once> for u8 {
    #[inline(always)]
    fn from(val: Ctrl0Once) -> u8 {
        Ctrl0Once::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl0Outmode {
    #[doc = "Asserted while counter is active"]
    COUNTER_ACTIVE = 0x0,
    #[doc = "Clear OFLAG output on successful compare"]
    CLEAR_OFLAG = 0x01,
    #[doc = "Set OFLAG output on successful compare"]
    SET_OFLAG = 0x02,
    #[doc = "Toggle OFLAG output on successful compare"]
    TOGGLE_OFLAG_SUCCESS = 0x03,
    #[doc = "Toggle OFLAG output using alternating compare registers"]
    TOGGLE_OFLAG_ALT = 0x04,
    #[doc = "Set on compare, cleared on secondary source input edge"]
    CLEAR_ON_SECONDARY = 0x05,
    #[doc = "Set on compare, cleared on counter rollover"]
    CLEAR_ON_ROLLOVER = 0x06,
    #[doc = "Enable gated clock output while counter is active"]
    ENABLE_GATED_OUT = 0x07,
}
impl Ctrl0Outmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl0Outmode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl0Outmode {
    #[inline(always)]
    fn from(val: u8) -> Ctrl0Outmode {
        Ctrl0Outmode::from_bits(val)
    }
}
impl From<Ctrl0Outmode> for u8 {
    #[inline(always)]
    fn from(val: Ctrl0Outmode) -> u8 {
        Ctrl0Outmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl0Pcs {
    #[doc = "Counter 0 input pin"]
    COUNTER0_IN = 0x0,
    #[doc = "Counter 1 input pin"]
    COUNTER1_IN = 0x01,
    #[doc = "Counter 2 input pin"]
    COUNTER2_IN = 0x02,
    #[doc = "Counter 3 input pin"]
    COUNTER3_IN = 0x03,
    #[doc = "Counter 0 output"]
    COUNTER0_OUT = 0x04,
    #[doc = "Counter 1 output"]
    COUNTER1_OUT = 0x05,
    #[doc = "Counter 2 output"]
    COUNTER2_OUT = 0x06,
    #[doc = "Counter 3 output"]
    COUNTER3_OUT = 0x07,
    #[doc = "IP bus clock divide by 1 prescaler"]
    BUS_DIVBY1 = 0x08,
    #[doc = "IP bus clock divide by 2 prescaler"]
    BUS_DIVBY2 = 0x09,
    #[doc = "IP bus clock divide by 4 prescaler"]
    BUS_DIVBY4 = 0x0a,
    #[doc = "IP bus clock divide by 8 prescaler"]
    BUS_DIVBY8 = 0x0b,
    #[doc = "IP bus clock divide by 16 prescaler"]
    BUS_DIVBY16 = 0x0c,
    #[doc = "IP bus clock divide by 32 prescaler"]
    BUS_DIVBY32 = 0x0d,
    #[doc = "IP bus clock divide by 64 prescaler"]
    BUS_DIVBY64 = 0x0e,
    #[doc = "IP bus clock divide by 128 prescaler"]
    BUS_DIVBY128 = 0x0f,
}
impl Ctrl0Pcs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl0Pcs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl0Pcs {
    #[inline(always)]
    fn from(val: u8) -> Ctrl0Pcs {
        Ctrl0Pcs::from_bits(val)
    }
}
impl From<Ctrl0Pcs> for u8 {
    #[inline(always)]
    fn from(val: Ctrl0Pcs) -> u8 {
        Ctrl0Pcs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl0Scs {
    #[doc = "Counter 0 input pin"]
    COUNTER0_IN = 0x0,
    #[doc = "Counter 1 input pin"]
    COUNTER1_IN = 0x01,
    #[doc = "Counter 2 input pin"]
    COUNTER2_IN = 0x02,
    #[doc = "Counter 3 input pin"]
    COUNTER3_IN = 0x03,
}
impl Ctrl0Scs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl0Scs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl0Scs {
    #[inline(always)]
    fn from(val: u8) -> Ctrl0Scs {
        Ctrl0Scs::from_bits(val)
    }
}
impl From<Ctrl0Scs> for u8 {
    #[inline(always)]
    fn from(val: Ctrl0Scs) -> u8 {
        Ctrl0Scs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl1Cm {
    #[doc = "No operation"]
    NOOP = 0x0,
    #[doc = "Count rising edges of primary sourceRising edges are counted only when SCTRL\\[IPS\\] = 0. Falling edges are counted when SCTRL\\[IPS\\] = 1. If the primary count source is IP bus clock divide by 1, only rising edges are counted regardless of the value of SCTRL\\[IPS\\]."]
    RISING_ONLY = 0x01,
    #[doc = "Count rising and falling edges of primary sourceIP bus clock divide by 1 cannot be used as a primary count source in edge count mode."]
    RISING_AND_FALLING = 0x02,
    #[doc = "Count rising edges of primary source while secondary input high active"]
    RISING_WHILE_SEC_HIGH = 0x03,
    #[doc = "Quadrature count mode, uses primary and secondary sources"]
    QUADRATURE = 0x04,
    #[doc = "Count rising edges of primary source; secondary source specifies directionRising edges are counted only when SCTRL\\[IPS\\] = 0. Falling edges are counted when SCTRL\\[IPS\\] = 1."]
    RISING_SEC_DIR = 0x05,
    #[doc = "Edge of secondary source triggers primary count until compare"]
    SECONDARY = 0x06,
    #[doc = "Cascaded counter mode (up/down)The primary count source must be set to one of the counter outputs."]
    CASCADE = 0x07,
}
impl Ctrl1Cm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl1Cm {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl1Cm {
    #[inline(always)]
    fn from(val: u8) -> Ctrl1Cm {
        Ctrl1Cm::from_bits(val)
    }
}
impl From<Ctrl1Cm> for u8 {
    #[inline(always)]
    fn from(val: Ctrl1Cm) -> u8 {
        Ctrl1Cm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl1Dir {
    #[doc = "Count up."]
    COUNTUP = 0x0,
    #[doc = "Count down."]
    COUNTDOWN = 0x01,
}
impl Ctrl1Dir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl1Dir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl1Dir {
    #[inline(always)]
    fn from(val: u8) -> Ctrl1Dir {
        Ctrl1Dir::from_bits(val)
    }
}
impl From<Ctrl1Dir> for u8 {
    #[inline(always)]
    fn from(val: Ctrl1Dir) -> u8 {
        Ctrl1Dir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl1Length {
    #[doc = "Count until roll over at $FFFF and continue from $0000."]
    UNTIL_ROLLOVER = 0x0,
    #[doc = "Count until compare, then re-initialize. If counting up, a successful compare occurs when the counter reaches a COMP1 value. If counting down, a successful compare occurs when the counter reaches a COMP2 value. When output mode $4 is used, alternating values of COMP1 and COMP2 are used to generate successful comparisons. For example, the counter counts until a COMP1 value is reached, re-initializes, counts until COMP2 value is reached, re-initializes, counts until COMP1 value is reached, and so on."]
    UNTIL_COMPARE = 0x01,
}
impl Ctrl1Length {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl1Length {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl1Length {
    #[inline(always)]
    fn from(val: u8) -> Ctrl1Length {
        Ctrl1Length::from_bits(val)
    }
}
impl From<Ctrl1Length> for u8 {
    #[inline(always)]
    fn from(val: Ctrl1Length) -> u8 {
        Ctrl1Length::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl1Once {
    #[doc = "Count repeatedly."]
    REPEAT = 0x0,
    #[doc = "Count until compare and then stop. If counting up, a successful compare occurs when the counter reaches a COMP1 value. If counting down, a successful compare occurs when the counter reaches a COMP2 value. When output mode $4 is used, the counter re-initializes after reaching the COMP1 value, continues to count to the COMP2 value, and then stops."]
    UNTIL_COMPARE = 0x01,
}
impl Ctrl1Once {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl1Once {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl1Once {
    #[inline(always)]
    fn from(val: u8) -> Ctrl1Once {
        Ctrl1Once::from_bits(val)
    }
}
impl From<Ctrl1Once> for u8 {
    #[inline(always)]
    fn from(val: Ctrl1Once) -> u8 {
        Ctrl1Once::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl1Outmode {
    #[doc = "Asserted while counter is active"]
    COUNTER_ACTIVE = 0x0,
    #[doc = "Clear OFLAG output on successful compare"]
    CLEAR_OFLAG = 0x01,
    #[doc = "Set OFLAG output on successful compare"]
    SET_OFLAG = 0x02,
    #[doc = "Toggle OFLAG output on successful compare"]
    TOGGLE_OFLAG_SUCCESS = 0x03,
    #[doc = "Toggle OFLAG output using alternating compare registers"]
    TOGGLE_OFLAG_ALT = 0x04,
    #[doc = "Set on compare, cleared on secondary source input edge"]
    CLEAR_ON_SECONDARY = 0x05,
    #[doc = "Set on compare, cleared on counter rollover"]
    CLEAR_ON_ROLLOVER = 0x06,
    #[doc = "Enable gated clock output while counter is active"]
    ENABLE_GATED_OUT = 0x07,
}
impl Ctrl1Outmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl1Outmode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl1Outmode {
    #[inline(always)]
    fn from(val: u8) -> Ctrl1Outmode {
        Ctrl1Outmode::from_bits(val)
    }
}
impl From<Ctrl1Outmode> for u8 {
    #[inline(always)]
    fn from(val: Ctrl1Outmode) -> u8 {
        Ctrl1Outmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl1Pcs {
    #[doc = "Counter 0 input pin"]
    COUNTER0_IN = 0x0,
    #[doc = "Counter 1 input pin"]
    COUNTER1_IN = 0x01,
    #[doc = "Counter 2 input pin"]
    COUNTER2_IN = 0x02,
    #[doc = "Counter 3 input pin"]
    COUNTER3_IN = 0x03,
    #[doc = "Counter 0 output"]
    COUNTER0_OUT = 0x04,
    #[doc = "Counter 1 output"]
    COUNTER1_OUT = 0x05,
    #[doc = "Counter 2 output"]
    COUNTER2_OUT = 0x06,
    #[doc = "Counter 3 output"]
    COUNTER3_OUT = 0x07,
    #[doc = "IP bus clock divide by 1 prescaler"]
    BUS_DIVBY1 = 0x08,
    #[doc = "IP bus clock divide by 2 prescaler"]
    BUS_DIVBY2 = 0x09,
    #[doc = "IP bus clock divide by 4 prescaler"]
    BUS_DIVBY4 = 0x0a,
    #[doc = "IP bus clock divide by 8 prescaler"]
    BUS_DIVBY8 = 0x0b,
    #[doc = "IP bus clock divide by 16 prescaler"]
    BUS_DIVBY16 = 0x0c,
    #[doc = "IP bus clock divide by 32 prescaler"]
    BUS_DIVBY32 = 0x0d,
    #[doc = "IP bus clock divide by 64 prescaler"]
    BUS_DIVBY64 = 0x0e,
    #[doc = "IP bus clock divide by 128 prescaler"]
    BUS_DIVBY128 = 0x0f,
}
impl Ctrl1Pcs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl1Pcs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl1Pcs {
    #[inline(always)]
    fn from(val: u8) -> Ctrl1Pcs {
        Ctrl1Pcs::from_bits(val)
    }
}
impl From<Ctrl1Pcs> for u8 {
    #[inline(always)]
    fn from(val: Ctrl1Pcs) -> u8 {
        Ctrl1Pcs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl1Scs {
    #[doc = "Counter 0 input pin"]
    COUNTER0_IN = 0x0,
    #[doc = "Counter 1 input pin"]
    COUNTER1_IN = 0x01,
    #[doc = "Counter 2 input pin"]
    COUNTER2_IN = 0x02,
    #[doc = "Counter 3 input pin"]
    COUNTER3_IN = 0x03,
}
impl Ctrl1Scs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl1Scs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl1Scs {
    #[inline(always)]
    fn from(val: u8) -> Ctrl1Scs {
        Ctrl1Scs::from_bits(val)
    }
}
impl From<Ctrl1Scs> for u8 {
    #[inline(always)]
    fn from(val: Ctrl1Scs) -> u8 {
        Ctrl1Scs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl2Cm {
    #[doc = "No operation"]
    NOOP = 0x0,
    #[doc = "Count rising edges of primary sourceRising edges are counted only when SCTRL\\[IPS\\] = 0. Falling edges are counted when SCTRL\\[IPS\\] = 1. If the primary count source is IP bus clock divide by 1, only rising edges are counted regardless of the value of SCTRL\\[IPS\\]."]
    RISING_ONLY = 0x01,
    #[doc = "Count rising and falling edges of primary sourceIP bus clock divide by 1 cannot be used as a primary count source in edge count mode."]
    RISING_AND_FALLING = 0x02,
    #[doc = "Count rising edges of primary source while secondary input high active"]
    RISING_WHILE_SEC_HIGH = 0x03,
    #[doc = "Quadrature count mode, uses primary and secondary sources"]
    QUADRATURE = 0x04,
    #[doc = "Count rising edges of primary source; secondary source specifies directionRising edges are counted only when SCTRL\\[IPS\\] = 0. Falling edges are counted when SCTRL\\[IPS\\] = 1."]
    RISING_SEC_DIR = 0x05,
    #[doc = "Edge of secondary source triggers primary count until compare"]
    SECONDARY = 0x06,
    #[doc = "Cascaded counter mode (up/down)The primary count source must be set to one of the counter outputs."]
    CASCADE = 0x07,
}
impl Ctrl2Cm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl2Cm {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl2Cm {
    #[inline(always)]
    fn from(val: u8) -> Ctrl2Cm {
        Ctrl2Cm::from_bits(val)
    }
}
impl From<Ctrl2Cm> for u8 {
    #[inline(always)]
    fn from(val: Ctrl2Cm) -> u8 {
        Ctrl2Cm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl2Dir {
    #[doc = "Count up."]
    COUNTUP = 0x0,
    #[doc = "Count down."]
    COUNTDOWN = 0x01,
}
impl Ctrl2Dir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl2Dir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl2Dir {
    #[inline(always)]
    fn from(val: u8) -> Ctrl2Dir {
        Ctrl2Dir::from_bits(val)
    }
}
impl From<Ctrl2Dir> for u8 {
    #[inline(always)]
    fn from(val: Ctrl2Dir) -> u8 {
        Ctrl2Dir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl2Length {
    #[doc = "Count until roll over at $FFFF and continue from $0000."]
    UNTIL_ROLLOVER = 0x0,
    #[doc = "Count until compare, then re-initialize. If counting up, a successful compare occurs when the counter reaches a COMP1 value. If counting down, a successful compare occurs when the counter reaches a COMP2 value. When output mode $4 is used, alternating values of COMP1 and COMP2 are used to generate successful comparisons. For example, the counter counts until a COMP1 value is reached, re-initializes, counts until COMP2 value is reached, re-initializes, counts until COMP1 value is reached, and so on."]
    UNTIL_COMPARE = 0x01,
}
impl Ctrl2Length {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl2Length {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl2Length {
    #[inline(always)]
    fn from(val: u8) -> Ctrl2Length {
        Ctrl2Length::from_bits(val)
    }
}
impl From<Ctrl2Length> for u8 {
    #[inline(always)]
    fn from(val: Ctrl2Length) -> u8 {
        Ctrl2Length::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl2Once {
    #[doc = "Count repeatedly."]
    REPEAT = 0x0,
    #[doc = "Count until compare and then stop. If counting up, a successful compare occurs when the counter reaches a COMP1 value. If counting down, a successful compare occurs when the counter reaches a COMP2 value. When output mode $4 is used, the counter re-initializes after reaching the COMP1 value, continues to count to the COMP2 value, and then stops."]
    UNTIL_COMPARE = 0x01,
}
impl Ctrl2Once {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl2Once {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl2Once {
    #[inline(always)]
    fn from(val: u8) -> Ctrl2Once {
        Ctrl2Once::from_bits(val)
    }
}
impl From<Ctrl2Once> for u8 {
    #[inline(always)]
    fn from(val: Ctrl2Once) -> u8 {
        Ctrl2Once::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl2Outmode {
    #[doc = "Asserted while counter is active"]
    COUNTER_ACTIVE = 0x0,
    #[doc = "Clear OFLAG output on successful compare"]
    CLEAR_OFLAG = 0x01,
    #[doc = "Set OFLAG output on successful compare"]
    SET_OFLAG = 0x02,
    #[doc = "Toggle OFLAG output on successful compare"]
    TOGGLE_OFLAG_SUCCESS = 0x03,
    #[doc = "Toggle OFLAG output using alternating compare registers"]
    TOGGLE_OFLAG_ALT = 0x04,
    #[doc = "Set on compare, cleared on secondary source input edge"]
    CLEAR_ON_SECONDARY = 0x05,
    #[doc = "Set on compare, cleared on counter rollover"]
    CLEAR_ON_ROLLOVER = 0x06,
    #[doc = "Enable gated clock output while counter is active"]
    ENABLE_GATED_OUT = 0x07,
}
impl Ctrl2Outmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl2Outmode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl2Outmode {
    #[inline(always)]
    fn from(val: u8) -> Ctrl2Outmode {
        Ctrl2Outmode::from_bits(val)
    }
}
impl From<Ctrl2Outmode> for u8 {
    #[inline(always)]
    fn from(val: Ctrl2Outmode) -> u8 {
        Ctrl2Outmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl2Pcs {
    #[doc = "Counter 0 input pin"]
    COUNTER0_IN = 0x0,
    #[doc = "Counter 1 input pin"]
    COUNTER1_IN = 0x01,
    #[doc = "Counter 2 input pin"]
    COUNTER2_IN = 0x02,
    #[doc = "Counter 3 input pin"]
    COUNTER3_IN = 0x03,
    #[doc = "Counter 0 output"]
    COUNTER0_OUT = 0x04,
    #[doc = "Counter 1 output"]
    COUNTER1_OUT = 0x05,
    #[doc = "Counter 2 output"]
    COUNTER2_OUT = 0x06,
    #[doc = "Counter 3 output"]
    COUNTER3_OUT = 0x07,
    #[doc = "IP bus clock divide by 1 prescaler"]
    BUS_DIVBY1 = 0x08,
    #[doc = "IP bus clock divide by 2 prescaler"]
    BUS_DIVBY2 = 0x09,
    #[doc = "IP bus clock divide by 4 prescaler"]
    BUS_DIVBY4 = 0x0a,
    #[doc = "IP bus clock divide by 8 prescaler"]
    BUS_DIVBY8 = 0x0b,
    #[doc = "IP bus clock divide by 16 prescaler"]
    BUS_DIVBY16 = 0x0c,
    #[doc = "IP bus clock divide by 32 prescaler"]
    BUS_DIVBY32 = 0x0d,
    #[doc = "IP bus clock divide by 64 prescaler"]
    BUS_DIVBY64 = 0x0e,
    #[doc = "IP bus clock divide by 128 prescaler"]
    BUS_DIVBY128 = 0x0f,
}
impl Ctrl2Pcs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl2Pcs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl2Pcs {
    #[inline(always)]
    fn from(val: u8) -> Ctrl2Pcs {
        Ctrl2Pcs::from_bits(val)
    }
}
impl From<Ctrl2Pcs> for u8 {
    #[inline(always)]
    fn from(val: Ctrl2Pcs) -> u8 {
        Ctrl2Pcs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl2Scs {
    #[doc = "Counter 0 input pin"]
    COUNTER0_IN = 0x0,
    #[doc = "Counter 1 input pin"]
    COUNTER1_IN = 0x01,
    #[doc = "Counter 2 input pin"]
    COUNTER2_IN = 0x02,
    #[doc = "Counter 3 input pin"]
    COUNTER3_IN = 0x03,
}
impl Ctrl2Scs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl2Scs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl2Scs {
    #[inline(always)]
    fn from(val: u8) -> Ctrl2Scs {
        Ctrl2Scs::from_bits(val)
    }
}
impl From<Ctrl2Scs> for u8 {
    #[inline(always)]
    fn from(val: Ctrl2Scs) -> u8 {
        Ctrl2Scs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl3Cm {
    #[doc = "No operation"]
    NOOP = 0x0,
    #[doc = "Count rising edges of primary sourceRising edges are counted only when SCTRL\\[IPS\\] = 0. Falling edges are counted when SCTRL\\[IPS\\] = 1. If the primary count source is IP bus clock divide by 1, only rising edges are counted regardless of the value of SCTRL\\[IPS\\]."]
    RISING_ONLY = 0x01,
    #[doc = "Count rising and falling edges of primary sourceIP bus clock divide by 1 cannot be used as a primary count source in edge count mode."]
    RISING_AND_FALLING = 0x02,
    #[doc = "Count rising edges of primary source while secondary input high active"]
    RISING_WHILE_SEC_HIGH = 0x03,
    #[doc = "Quadrature count mode, uses primary and secondary sources"]
    QUADRATURE = 0x04,
    #[doc = "Count rising edges of primary source; secondary source specifies directionRising edges are counted only when SCTRL\\[IPS\\] = 0. Falling edges are counted when SCTRL\\[IPS\\] = 1."]
    RISING_SEC_DIR = 0x05,
    #[doc = "Edge of secondary source triggers primary count until compare"]
    SECONDARY = 0x06,
    #[doc = "Cascaded counter mode (up/down)The primary count source must be set to one of the counter outputs."]
    CASCADE = 0x07,
}
impl Ctrl3Cm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl3Cm {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl3Cm {
    #[inline(always)]
    fn from(val: u8) -> Ctrl3Cm {
        Ctrl3Cm::from_bits(val)
    }
}
impl From<Ctrl3Cm> for u8 {
    #[inline(always)]
    fn from(val: Ctrl3Cm) -> u8 {
        Ctrl3Cm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl3Dir {
    #[doc = "Count up."]
    COUNTUP = 0x0,
    #[doc = "Count down."]
    COUNTDOWN = 0x01,
}
impl Ctrl3Dir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl3Dir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl3Dir {
    #[inline(always)]
    fn from(val: u8) -> Ctrl3Dir {
        Ctrl3Dir::from_bits(val)
    }
}
impl From<Ctrl3Dir> for u8 {
    #[inline(always)]
    fn from(val: Ctrl3Dir) -> u8 {
        Ctrl3Dir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl3Length {
    #[doc = "Count until roll over at $FFFF and continue from $0000."]
    UNTIL_ROLLOVER = 0x0,
    #[doc = "Count until compare, then re-initialize. If counting up, a successful compare occurs when the counter reaches a COMP1 value. If counting down, a successful compare occurs when the counter reaches a COMP2 value. When output mode $4 is used, alternating values of COMP1 and COMP2 are used to generate successful comparisons. For example, the counter counts until a COMP1 value is reached, re-initializes, counts until COMP2 value is reached, re-initializes, counts until COMP1 value is reached, and so on."]
    UNTIL_COMPARE = 0x01,
}
impl Ctrl3Length {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl3Length {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl3Length {
    #[inline(always)]
    fn from(val: u8) -> Ctrl3Length {
        Ctrl3Length::from_bits(val)
    }
}
impl From<Ctrl3Length> for u8 {
    #[inline(always)]
    fn from(val: Ctrl3Length) -> u8 {
        Ctrl3Length::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl3Once {
    #[doc = "Count repeatedly."]
    REPEAT = 0x0,
    #[doc = "Count until compare and then stop. If counting up, a successful compare occurs when the counter reaches a COMP1 value. If counting down, a successful compare occurs when the counter reaches a COMP2 value. When output mode $4 is used, the counter re-initializes after reaching the COMP1 value, continues to count to the COMP2 value, and then stops."]
    UNTIL_COMPARE = 0x01,
}
impl Ctrl3Once {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl3Once {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl3Once {
    #[inline(always)]
    fn from(val: u8) -> Ctrl3Once {
        Ctrl3Once::from_bits(val)
    }
}
impl From<Ctrl3Once> for u8 {
    #[inline(always)]
    fn from(val: Ctrl3Once) -> u8 {
        Ctrl3Once::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl3Outmode {
    #[doc = "Asserted while counter is active"]
    COUNTER_ACTIVE = 0x0,
    #[doc = "Clear OFLAG output on successful compare"]
    CLEAR_OFLAG = 0x01,
    #[doc = "Set OFLAG output on successful compare"]
    SET_OFLAG = 0x02,
    #[doc = "Toggle OFLAG output on successful compare"]
    TOGGLE_OFLAG_SUCCESS = 0x03,
    #[doc = "Toggle OFLAG output using alternating compare registers"]
    TOGGLE_OFLAG_ALT = 0x04,
    #[doc = "Set on compare, cleared on secondary source input edge"]
    CLEAR_ON_SECONDARY = 0x05,
    #[doc = "Set on compare, cleared on counter rollover"]
    CLEAR_ON_ROLLOVER = 0x06,
    #[doc = "Enable gated clock output while counter is active"]
    ENABLE_GATED_OUT = 0x07,
}
impl Ctrl3Outmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl3Outmode {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl3Outmode {
    #[inline(always)]
    fn from(val: u8) -> Ctrl3Outmode {
        Ctrl3Outmode::from_bits(val)
    }
}
impl From<Ctrl3Outmode> for u8 {
    #[inline(always)]
    fn from(val: Ctrl3Outmode) -> u8 {
        Ctrl3Outmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl3Pcs {
    #[doc = "Counter 0 input pin"]
    COUNTER0_IN = 0x0,
    #[doc = "Counter 1 input pin"]
    COUNTER1_IN = 0x01,
    #[doc = "Counter 2 input pin"]
    COUNTER2_IN = 0x02,
    #[doc = "Counter 3 input pin"]
    COUNTER3_IN = 0x03,
    #[doc = "Counter 0 output"]
    COUNTER0_OUT = 0x04,
    #[doc = "Counter 1 output"]
    COUNTER1_OUT = 0x05,
    #[doc = "Counter 2 output"]
    COUNTER2_OUT = 0x06,
    #[doc = "Counter 3 output"]
    COUNTER3_OUT = 0x07,
    #[doc = "IP bus clock divide by 1 prescaler"]
    BUS_DIVBY1 = 0x08,
    #[doc = "IP bus clock divide by 2 prescaler"]
    BUS_DIVBY2 = 0x09,
    #[doc = "IP bus clock divide by 4 prescaler"]
    BUS_DIVBY4 = 0x0a,
    #[doc = "IP bus clock divide by 8 prescaler"]
    BUS_DIVBY8 = 0x0b,
    #[doc = "IP bus clock divide by 16 prescaler"]
    BUS_DIVBY16 = 0x0c,
    #[doc = "IP bus clock divide by 32 prescaler"]
    BUS_DIVBY32 = 0x0d,
    #[doc = "IP bus clock divide by 64 prescaler"]
    BUS_DIVBY64 = 0x0e,
    #[doc = "IP bus clock divide by 128 prescaler"]
    BUS_DIVBY128 = 0x0f,
}
impl Ctrl3Pcs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl3Pcs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl3Pcs {
    #[inline(always)]
    fn from(val: u8) -> Ctrl3Pcs {
        Ctrl3Pcs::from_bits(val)
    }
}
impl From<Ctrl3Pcs> for u8 {
    #[inline(always)]
    fn from(val: Ctrl3Pcs) -> u8 {
        Ctrl3Pcs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl3Scs {
    #[doc = "Counter 0 input pin"]
    COUNTER0_IN = 0x0,
    #[doc = "Counter 1 input pin"]
    COUNTER1_IN = 0x01,
    #[doc = "Counter 2 input pin"]
    COUNTER2_IN = 0x02,
    #[doc = "Counter 3 input pin"]
    COUNTER3_IN = 0x03,
}
impl Ctrl3Scs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl3Scs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl3Scs {
    #[inline(always)]
    fn from(val: u8) -> Ctrl3Scs {
        Ctrl3Scs::from_bits(val)
    }
}
impl From<Ctrl3Scs> for u8 {
    #[inline(always)]
    fn from(val: Ctrl3Scs) -> u8 {
        Ctrl3Scs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enbl {
    #[doc = "Timer channel is disabled."]
    DISABLE = 0x0,
    #[doc = "Timer channel is enabled. (default)"]
    ENABLE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Enbl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enbl {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enbl {
    #[inline(always)]
    fn from(val: u8) -> Enbl {
        Enbl::from_bits(val)
    }
}
impl From<Enbl> for u8 {
    #[inline(always)]
    fn from(val: Enbl) -> u8 {
        Enbl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sctrl0CaptureMode {
    #[doc = "Capture function is disabled"]
    DISABLED = 0x0,
    #[doc = "Load capture register on rising edge (when IPS=0) or falling edge (when IPS=1) of input"]
    ENABLE_RISING = 0x01,
    #[doc = "Load capture register on falling edge (when IPS=0) or rising edge (when IPS=1) of input"]
    ENABLE_FALLING = 0x02,
    #[doc = "Load capture register on both edges of input"]
    ENABLE_BOTH = 0x03,
}
impl Sctrl0CaptureMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sctrl0CaptureMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sctrl0CaptureMode {
    #[inline(always)]
    fn from(val: u8) -> Sctrl0CaptureMode {
        Sctrl0CaptureMode::from_bits(val)
    }
}
impl From<Sctrl0CaptureMode> for u8 {
    #[inline(always)]
    fn from(val: Sctrl0CaptureMode) -> u8 {
        Sctrl0CaptureMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sctrl0Oen {
    #[doc = "The external pin is configured as an input."]
    INPUT = 0x0,
    #[doc = "The OFLAG output signal is driven on the external pin. Other timer groups using this external pin as their input see the driven value. The polarity of the signal is determined by OPS."]
    OFLAG_OUT = 0x01,
}
impl Sctrl0Oen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sctrl0Oen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sctrl0Oen {
    #[inline(always)]
    fn from(val: u8) -> Sctrl0Oen {
        Sctrl0Oen::from_bits(val)
    }
}
impl From<Sctrl0Oen> for u8 {
    #[inline(always)]
    fn from(val: Sctrl0Oen) -> u8 {
        Sctrl0Oen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sctrl0Ops {
    #[doc = "True polarity."]
    TRUE = 0x0,
    #[doc = "Inverted polarity."]
    INVERTED = 0x01,
}
impl Sctrl0Ops {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sctrl0Ops {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sctrl0Ops {
    #[inline(always)]
    fn from(val: u8) -> Sctrl0Ops {
        Sctrl0Ops::from_bits(val)
    }
}
impl From<Sctrl0Ops> for u8 {
    #[inline(always)]
    fn from(val: Sctrl0Ops) -> u8 {
        Sctrl0Ops::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sctrl1CaptureMode {
    #[doc = "Capture function is disabled"]
    DISABLED = 0x0,
    #[doc = "Load capture register on rising edge (when IPS=0) or falling edge (when IPS=1) of input"]
    ENABLE_RISING = 0x01,
    #[doc = "Load capture register on falling edge (when IPS=0) or rising edge (when IPS=1) of input"]
    ENABLE_FALLING = 0x02,
    #[doc = "Load capture register on both edges of input"]
    ENABLE_BOTH = 0x03,
}
impl Sctrl1CaptureMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sctrl1CaptureMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sctrl1CaptureMode {
    #[inline(always)]
    fn from(val: u8) -> Sctrl1CaptureMode {
        Sctrl1CaptureMode::from_bits(val)
    }
}
impl From<Sctrl1CaptureMode> for u8 {
    #[inline(always)]
    fn from(val: Sctrl1CaptureMode) -> u8 {
        Sctrl1CaptureMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sctrl1Oen {
    #[doc = "The external pin is configured as an input."]
    INPUT = 0x0,
    #[doc = "The OFLAG output signal is driven on the external pin. Other timer groups using this external pin as their input see the driven value. The polarity of the signal is determined by OPS."]
    OFLAG_OUT = 0x01,
}
impl Sctrl1Oen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sctrl1Oen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sctrl1Oen {
    #[inline(always)]
    fn from(val: u8) -> Sctrl1Oen {
        Sctrl1Oen::from_bits(val)
    }
}
impl From<Sctrl1Oen> for u8 {
    #[inline(always)]
    fn from(val: Sctrl1Oen) -> u8 {
        Sctrl1Oen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sctrl1Ops {
    #[doc = "True polarity."]
    TRUE = 0x0,
    #[doc = "Inverted polarity."]
    INVERTED = 0x01,
}
impl Sctrl1Ops {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sctrl1Ops {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sctrl1Ops {
    #[inline(always)]
    fn from(val: u8) -> Sctrl1Ops {
        Sctrl1Ops::from_bits(val)
    }
}
impl From<Sctrl1Ops> for u8 {
    #[inline(always)]
    fn from(val: Sctrl1Ops) -> u8 {
        Sctrl1Ops::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sctrl2CaptureMode {
    #[doc = "Capture function is disabled"]
    DISABLED = 0x0,
    #[doc = "Load capture register on rising edge (when IPS=0) or falling edge (when IPS=1) of input"]
    ENABLE_RISING = 0x01,
    #[doc = "Load capture register on falling edge (when IPS=0) or rising edge (when IPS=1) of input"]
    ENABLE_FALLING = 0x02,
    #[doc = "Load capture register on both edges of input"]
    ENABLE_BOTH = 0x03,
}
impl Sctrl2CaptureMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sctrl2CaptureMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sctrl2CaptureMode {
    #[inline(always)]
    fn from(val: u8) -> Sctrl2CaptureMode {
        Sctrl2CaptureMode::from_bits(val)
    }
}
impl From<Sctrl2CaptureMode> for u8 {
    #[inline(always)]
    fn from(val: Sctrl2CaptureMode) -> u8 {
        Sctrl2CaptureMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sctrl2Oen {
    #[doc = "The external pin is configured as an input."]
    INPUT = 0x0,
    #[doc = "The OFLAG output signal is driven on the external pin. Other timer groups using this external pin as their input see the driven value. The polarity of the signal is determined by OPS."]
    OFLAG_OUT = 0x01,
}
impl Sctrl2Oen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sctrl2Oen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sctrl2Oen {
    #[inline(always)]
    fn from(val: u8) -> Sctrl2Oen {
        Sctrl2Oen::from_bits(val)
    }
}
impl From<Sctrl2Oen> for u8 {
    #[inline(always)]
    fn from(val: Sctrl2Oen) -> u8 {
        Sctrl2Oen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sctrl2Ops {
    #[doc = "True polarity."]
    TRUE = 0x0,
    #[doc = "Inverted polarity."]
    INVERTED = 0x01,
}
impl Sctrl2Ops {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sctrl2Ops {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sctrl2Ops {
    #[inline(always)]
    fn from(val: u8) -> Sctrl2Ops {
        Sctrl2Ops::from_bits(val)
    }
}
impl From<Sctrl2Ops> for u8 {
    #[inline(always)]
    fn from(val: Sctrl2Ops) -> u8 {
        Sctrl2Ops::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sctrl3CaptureMode {
    #[doc = "Capture function is disabled"]
    DISABLED = 0x0,
    #[doc = "Load capture register on rising edge (when IPS=0) or falling edge (when IPS=1) of input"]
    ENABLE_RISING = 0x01,
    #[doc = "Load capture register on falling edge (when IPS=0) or rising edge (when IPS=1) of input"]
    ENABLE_FALLING = 0x02,
    #[doc = "Load capture register on both edges of input"]
    ENABLE_BOTH = 0x03,
}
impl Sctrl3CaptureMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sctrl3CaptureMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sctrl3CaptureMode {
    #[inline(always)]
    fn from(val: u8) -> Sctrl3CaptureMode {
        Sctrl3CaptureMode::from_bits(val)
    }
}
impl From<Sctrl3CaptureMode> for u8 {
    #[inline(always)]
    fn from(val: Sctrl3CaptureMode) -> u8 {
        Sctrl3CaptureMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sctrl3Oen {
    #[doc = "The external pin is configured as an input."]
    INPUT = 0x0,
    #[doc = "The OFLAG output signal is driven on the external pin. Other timer groups using this external pin as their input see the driven value. The polarity of the signal is determined by OPS."]
    OFLAG_OUT = 0x01,
}
impl Sctrl3Oen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sctrl3Oen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sctrl3Oen {
    #[inline(always)]
    fn from(val: u8) -> Sctrl3Oen {
        Sctrl3Oen::from_bits(val)
    }
}
impl From<Sctrl3Oen> for u8 {
    #[inline(always)]
    fn from(val: Sctrl3Oen) -> u8 {
        Sctrl3Oen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sctrl3Ops {
    #[doc = "True polarity."]
    TRUE = 0x0,
    #[doc = "Inverted polarity."]
    INVERTED = 0x01,
}
impl Sctrl3Ops {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sctrl3Ops {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sctrl3Ops {
    #[inline(always)]
    fn from(val: u8) -> Sctrl3Ops {
        Sctrl3Ops::from_bits(val)
    }
}
impl From<Sctrl3Ops> for u8 {
    #[inline(always)]
    fn from(val: Sctrl3Ops) -> u8 {
        Sctrl3Ops::to_bits(val)
    }
}
