#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cap0fe {
    #[doc = "Does not load."]
    Cap0fe0 = 0x0,
    #[doc = "Loads."]
    Capofe1 = 0x01,
}
impl Cap0fe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cap0fe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cap0fe {
    #[inline(always)]
    fn from(val: u8) -> Cap0fe {
        Cap0fe::from_bits(val)
    }
}
impl From<Cap0fe> for u8 {
    #[inline(always)]
    fn from(val: Cap0fe) -> u8 {
        Cap0fe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cap0i {
    #[doc = "Does not generate."]
    Cap0i0 = 0x0,
    #[doc = "Generates."]
    Capoi1 = 0x01,
}
impl Cap0i {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cap0i {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cap0i {
    #[inline(always)]
    fn from(val: u8) -> Cap0i {
        Cap0i::from_bits(val)
    }
}
impl From<Cap0i> for u8 {
    #[inline(always)]
    fn from(val: Cap0i) -> u8 {
        Cap0i::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cap0re {
    #[doc = "Does not load."]
    Cap0re0 = 0x0,
    #[doc = "Loads."]
    Capore1 = 0x01,
}
impl Cap0re {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cap0re {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cap0re {
    #[inline(always)]
    fn from(val: u8) -> Cap0re {
        Cap0re::from_bits(val)
    }
}
impl From<Cap0re> for u8 {
    #[inline(always)]
    fn from(val: Cap0re) -> u8 {
        Cap0re::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cap1fe {
    #[doc = "Does not load."]
    Cap1fe0 = 0x0,
    #[doc = "Loads."]
    Cap1fe1 = 0x01,
}
impl Cap1fe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cap1fe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cap1fe {
    #[inline(always)]
    fn from(val: u8) -> Cap1fe {
        Cap1fe::from_bits(val)
    }
}
impl From<Cap1fe> for u8 {
    #[inline(always)]
    fn from(val: Cap1fe) -> u8 {
        Cap1fe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cap1i {
    #[doc = "Does not generates."]
    Cap1i0 = 0x0,
    #[doc = "Generates."]
    Cap1i1 = 0x01,
}
impl Cap1i {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cap1i {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cap1i {
    #[inline(always)]
    fn from(val: u8) -> Cap1i {
        Cap1i::from_bits(val)
    }
}
impl From<Cap1i> for u8 {
    #[inline(always)]
    fn from(val: Cap1i) -> u8 {
        Cap1i::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cap1re {
    #[doc = "Does not load."]
    Cap1re0 = 0x0,
    #[doc = "Loads."]
    Cap1re1 = 0x01,
}
impl Cap1re {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cap1re {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cap1re {
    #[inline(always)]
    fn from(val: u8) -> Cap1re {
        Cap1re::from_bits(val)
    }
}
impl From<Cap1re> for u8 {
    #[inline(always)]
    fn from(val: Cap1re) -> u8 {
        Cap1re::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cap2fe {
    #[doc = "Does not load."]
    Cap2fe0 = 0x0,
    #[doc = "Loads."]
    Cap2fe1 = 0x01,
}
impl Cap2fe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cap2fe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cap2fe {
    #[inline(always)]
    fn from(val: u8) -> Cap2fe {
        Cap2fe::from_bits(val)
    }
}
impl From<Cap2fe> for u8 {
    #[inline(always)]
    fn from(val: Cap2fe) -> u8 {
        Cap2fe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cap2i {
    #[doc = "Does not generate."]
    Cap2i0 = 0x0,
    #[doc = "Generates."]
    Cap2i1 = 0x01,
}
impl Cap2i {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cap2i {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cap2i {
    #[inline(always)]
    fn from(val: u8) -> Cap2i {
        Cap2i::from_bits(val)
    }
}
impl From<Cap2i> for u8 {
    #[inline(always)]
    fn from(val: Cap2i) -> u8 {
        Cap2i::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cap2re {
    #[doc = "Does not load."]
    Cap2re0 = 0x0,
    #[doc = "Loads."]
    Cap2re1 = 0x01,
}
impl Cap2re {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cap2re {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cap2re {
    #[inline(always)]
    fn from(val: u8) -> Cap2re {
        Cap2re::from_bits(val)
    }
}
impl From<Cap2re> for u8 {
    #[inline(always)]
    fn from(val: Cap2re) -> u8 {
        Cap2re::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cap3fe {
    #[doc = "Does not load."]
    Cap3fe0 = 0x0,
    #[doc = "Loads."]
    Cap3fe1 = 0x01,
}
impl Cap3fe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cap3fe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cap3fe {
    #[inline(always)]
    fn from(val: u8) -> Cap3fe {
        Cap3fe::from_bits(val)
    }
}
impl From<Cap3fe> for u8 {
    #[inline(always)]
    fn from(val: Cap3fe) -> u8 {
        Cap3fe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cap3i {
    #[doc = "Does not generate."]
    Cap3i0 = 0x0,
    #[doc = "Generates."]
    Cap3i1 = 0x01,
}
impl Cap3i {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cap3i {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cap3i {
    #[inline(always)]
    fn from(val: u8) -> Cap3i {
        Cap3i::from_bits(val)
    }
}
impl From<Cap3i> for u8 {
    #[inline(always)]
    fn from(val: Cap3i) -> u8 {
        Cap3i::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cap3re {
    #[doc = "Does not load."]
    Cap3re0 = 0x0,
    #[doc = "Loads."]
    Cap3re1 = 0x01,
}
impl Cap3re {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cap3re {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cap3re {
    #[inline(always)]
    fn from(val: u8) -> Cap3re {
        Cap3re::from_bits(val)
    }
}
impl From<Cap3re> for u8 {
    #[inline(always)]
    fn from(val: Cap3re) -> u8 {
        Cap3re::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cinsel {
    #[doc = "Channel 0, CAPn\\[0\\] for CTIMERn."]
    Channel0 = 0x0,
    #[doc = "Channel 1, CAPn\\[1\\] for CTIMERn."]
    Channel1 = 0x01,
    #[doc = "Channel 2, CAPn\\[2\\] for CTIMERn."]
    Channel2 = 0x02,
    #[doc = "Channel 3, CAPn\\[3\\] for CTIMERn."]
    Channel3 = 0x03,
}
impl Cinsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cinsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cinsel {
    #[inline(always)]
    fn from(val: u8) -> Cinsel {
        Cinsel::from_bits(val)
    }
}
impl From<Cinsel> for u8 {
    #[inline(always)]
    fn from(val: Cinsel) -> u8 {
        Cinsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctmode {
    #[doc = "Timer mode."]
    Timer = 0x0,
    #[doc = "Counter mode rising edge."]
    CounterRisingEdge = 0x01,
    #[doc = "Counter mode falling edge."]
    CounterFallingEdge = 0x02,
    #[doc = "Counter mode dual edge."]
    CounterDualEdge = 0x03,
}
impl Ctmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctmode {
    #[inline(always)]
    fn from(val: u8) -> Ctmode {
        Ctmode::from_bits(val)
    }
}
impl From<Ctmode> for u8 {
    #[inline(always)]
    fn from(val: Ctmode) -> u8 {
        Ctmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Em0 {
    #[doc = "Low."]
    Clear = 0x0,
    #[doc = "High."]
    Set = 0x01,
}
impl Em0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Em0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Em0 {
    #[inline(always)]
    fn from(val: u8) -> Em0 {
        Em0::from_bits(val)
    }
}
impl From<Em0> for u8 {
    #[inline(always)]
    fn from(val: Em0) -> u8 {
        Em0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Em1 {
    #[doc = "Low."]
    Clear = 0x0,
    #[doc = "High."]
    Set = 0x01,
}
impl Em1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Em1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Em1 {
    #[inline(always)]
    fn from(val: u8) -> Em1 {
        Em1::from_bits(val)
    }
}
impl From<Em1> for u8 {
    #[inline(always)]
    fn from(val: Em1) -> u8 {
        Em1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Em2 {
    #[doc = "Low."]
    Clear = 0x0,
    #[doc = "High."]
    Set = 0x01,
}
impl Em2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Em2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Em2 {
    #[inline(always)]
    fn from(val: u8) -> Em2 {
        Em2::from_bits(val)
    }
}
impl From<Em2> for u8 {
    #[inline(always)]
    fn from(val: Em2) -> u8 {
        Em2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Em3 {
    #[doc = "Low."]
    Clear = 0x0,
    #[doc = "High."]
    Set = 0x01,
}
impl Em3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Em3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Em3 {
    #[inline(always)]
    fn from(val: u8) -> Em3 {
        Em3::from_bits(val)
    }
}
impl From<Em3> for u8 {
    #[inline(always)]
    fn from(val: Em3) -> u8 {
        Em3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Emc0 {
    #[doc = "Does nothing."]
    DoNothing = 0x0,
    #[doc = "Goes low."]
    Clear = 0x01,
    #[doc = "Goes high."]
    Set = 0x02,
    #[doc = "Toggles."]
    Toggle = 0x03,
}
impl Emc0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emc0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emc0 {
    #[inline(always)]
    fn from(val: u8) -> Emc0 {
        Emc0::from_bits(val)
    }
}
impl From<Emc0> for u8 {
    #[inline(always)]
    fn from(val: Emc0) -> u8 {
        Emc0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Emc1 {
    #[doc = "Does nothing."]
    DoNothing = 0x0,
    #[doc = "Goes low."]
    Clear = 0x01,
    #[doc = "Goes high."]
    Set = 0x02,
    #[doc = "Toggles."]
    Toggle = 0x03,
}
impl Emc1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emc1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emc1 {
    #[inline(always)]
    fn from(val: u8) -> Emc1 {
        Emc1::from_bits(val)
    }
}
impl From<Emc1> for u8 {
    #[inline(always)]
    fn from(val: Emc1) -> u8 {
        Emc1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Emc2 {
    #[doc = "Does nothing."]
    DoNothing = 0x0,
    #[doc = "Goes low."]
    Clear = 0x01,
    #[doc = "Goes high."]
    Set = 0x02,
    #[doc = "Toggles."]
    Toggle = 0x03,
}
impl Emc2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emc2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emc2 {
    #[inline(always)]
    fn from(val: u8) -> Emc2 {
        Emc2::from_bits(val)
    }
}
impl From<Emc2> for u8 {
    #[inline(always)]
    fn from(val: Emc2) -> u8 {
        Emc2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Emc3 {
    #[doc = "Does nothing."]
    DoNothing = 0x0,
    #[doc = "Goes low."]
    Clear = 0x01,
    #[doc = "Goes high."]
    Set = 0x02,
    #[doc = "Toggles."]
    Toggle = 0x03,
}
impl Emc3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emc3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emc3 {
    #[inline(always)]
    fn from(val: u8) -> Emc3 {
        Emc3::from_bits(val)
    }
}
impl From<Emc3> for u8 {
    #[inline(always)]
    fn from(val: Emc3) -> u8 {
        Emc3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mr0i {
    #[doc = "Does not generate."]
    Mr0i0 = 0x0,
    #[doc = "Generates."]
    Mr0i1 = 0x01,
}
impl Mr0i {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mr0i {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mr0i {
    #[inline(always)]
    fn from(val: u8) -> Mr0i {
        Mr0i::from_bits(val)
    }
}
impl From<Mr0i> for u8 {
    #[inline(always)]
    fn from(val: Mr0i) -> u8 {
        Mr0i::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mr0r {
    #[doc = "Does not reset."]
    Mr0r0 = 0x0,
    #[doc = "Resets."]
    Mr0r1 = 0x01,
}
impl Mr0r {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mr0r {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mr0r {
    #[inline(always)]
    fn from(val: u8) -> Mr0r {
        Mr0r::from_bits(val)
    }
}
impl From<Mr0r> for u8 {
    #[inline(always)]
    fn from(val: Mr0r) -> u8 {
        Mr0r::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mr0rl {
    #[doc = "Does not reload."]
    Mr0rl0 = 0x0,
    #[doc = "Reloads."]
    Mr0rl1 = 0x01,
}
impl Mr0rl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mr0rl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mr0rl {
    #[inline(always)]
    fn from(val: u8) -> Mr0rl {
        Mr0rl::from_bits(val)
    }
}
impl From<Mr0rl> for u8 {
    #[inline(always)]
    fn from(val: Mr0rl) -> u8 {
        Mr0rl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mr0s {
    #[doc = "Does not stop."]
    Mr0s0 = 0x0,
    #[doc = "Stops."]
    Mr0s1 = 0x01,
}
impl Mr0s {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mr0s {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mr0s {
    #[inline(always)]
    fn from(val: u8) -> Mr0s {
        Mr0s::from_bits(val)
    }
}
impl From<Mr0s> for u8 {
    #[inline(always)]
    fn from(val: Mr0s) -> u8 {
        Mr0s::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mr1i {
    #[doc = "Does not generate."]
    Mr1i0 = 0x0,
    #[doc = "Generates."]
    Mr1i1 = 0x01,
}
impl Mr1i {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mr1i {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mr1i {
    #[inline(always)]
    fn from(val: u8) -> Mr1i {
        Mr1i::from_bits(val)
    }
}
impl From<Mr1i> for u8 {
    #[inline(always)]
    fn from(val: Mr1i) -> u8 {
        Mr1i::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mr1r {
    #[doc = "Does not reset."]
    Mr1r0 = 0x0,
    #[doc = "Resets."]
    Mr1r1 = 0x01,
}
impl Mr1r {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mr1r {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mr1r {
    #[inline(always)]
    fn from(val: u8) -> Mr1r {
        Mr1r::from_bits(val)
    }
}
impl From<Mr1r> for u8 {
    #[inline(always)]
    fn from(val: Mr1r) -> u8 {
        Mr1r::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mr1rl {
    #[doc = "Does not reload."]
    Mr1rl0 = 0x0,
    #[doc = "Reloads."]
    Mr1rl1 = 0x01,
}
impl Mr1rl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mr1rl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mr1rl {
    #[inline(always)]
    fn from(val: u8) -> Mr1rl {
        Mr1rl::from_bits(val)
    }
}
impl From<Mr1rl> for u8 {
    #[inline(always)]
    fn from(val: Mr1rl) -> u8 {
        Mr1rl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mr1s {
    #[doc = "Does not stop."]
    Mris0 = 0x0,
    #[doc = "Stops."]
    Mris1 = 0x01,
}
impl Mr1s {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mr1s {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mr1s {
    #[inline(always)]
    fn from(val: u8) -> Mr1s {
        Mr1s::from_bits(val)
    }
}
impl From<Mr1s> for u8 {
    #[inline(always)]
    fn from(val: Mr1s) -> u8 {
        Mr1s::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mr2i {
    #[doc = "Does not generate."]
    Mr2i0 = 0x0,
    #[doc = "Generates."]
    Mr2i1 = 0x01,
}
impl Mr2i {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mr2i {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mr2i {
    #[inline(always)]
    fn from(val: u8) -> Mr2i {
        Mr2i::from_bits(val)
    }
}
impl From<Mr2i> for u8 {
    #[inline(always)]
    fn from(val: Mr2i) -> u8 {
        Mr2i::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mr2r {
    #[doc = "Does not reset."]
    Mr2r0 = 0x0,
    #[doc = "Resets."]
    Mr2r1 = 0x01,
}
impl Mr2r {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mr2r {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mr2r {
    #[inline(always)]
    fn from(val: u8) -> Mr2r {
        Mr2r::from_bits(val)
    }
}
impl From<Mr2r> for u8 {
    #[inline(always)]
    fn from(val: Mr2r) -> u8 {
        Mr2r::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mr2rl {
    #[doc = "Does not reload."]
    Mr2rl0 = 0x0,
    #[doc = "Reloads."]
    Mr2rl1 = 0x01,
}
impl Mr2rl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mr2rl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mr2rl {
    #[inline(always)]
    fn from(val: u8) -> Mr2rl {
        Mr2rl::from_bits(val)
    }
}
impl From<Mr2rl> for u8 {
    #[inline(always)]
    fn from(val: Mr2rl) -> u8 {
        Mr2rl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mr2s {
    #[doc = "Does not stop."]
    Mr2s0 = 0x0,
    #[doc = "Stops."]
    Mr2s1 = 0x01,
}
impl Mr2s {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mr2s {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mr2s {
    #[inline(always)]
    fn from(val: u8) -> Mr2s {
        Mr2s::from_bits(val)
    }
}
impl From<Mr2s> for u8 {
    #[inline(always)]
    fn from(val: Mr2s) -> u8 {
        Mr2s::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mr3i {
    #[doc = "Does not generate."]
    Mr3i0 = 0x0,
    #[doc = "Generates."]
    Mr3i1 = 0x01,
}
impl Mr3i {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mr3i {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mr3i {
    #[inline(always)]
    fn from(val: u8) -> Mr3i {
        Mr3i::from_bits(val)
    }
}
impl From<Mr3i> for u8 {
    #[inline(always)]
    fn from(val: Mr3i) -> u8 {
        Mr3i::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mr3r {
    #[doc = "Does not reset."]
    Mr3r0 = 0x0,
    #[doc = "Resets."]
    Mr3r1 = 0x01,
}
impl Mr3r {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mr3r {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mr3r {
    #[inline(always)]
    fn from(val: u8) -> Mr3r {
        Mr3r::from_bits(val)
    }
}
impl From<Mr3r> for u8 {
    #[inline(always)]
    fn from(val: Mr3r) -> u8 {
        Mr3r::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mr3rl {
    #[doc = "Does not reload."]
    Mr3rl0 = 0x0,
    #[doc = "Reloads."]
    Mr3rl1 = 0x01,
}
impl Mr3rl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mr3rl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mr3rl {
    #[inline(always)]
    fn from(val: u8) -> Mr3rl {
        Mr3rl::from_bits(val)
    }
}
impl From<Mr3rl> for u8 {
    #[inline(always)]
    fn from(val: Mr3rl) -> u8 {
        Mr3rl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mr3s {
    #[doc = "Does not stop."]
    Mr3s0 = 0x0,
    #[doc = "Stops."]
    Mr3s1 = 0x01,
}
impl Mr3s {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mr3s {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mr3s {
    #[inline(always)]
    fn from(val: u8) -> Mr3s {
        Mr3s::from_bits(val)
    }
}
impl From<Mr3s> for u8 {
    #[inline(always)]
    fn from(val: Mr3s) -> u8 {
        Mr3s::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwmen0 {
    #[doc = "Disable."]
    Match = 0x0,
    #[doc = "Enable."]
    Pwm = 0x01,
}
impl Pwmen0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwmen0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwmen0 {
    #[inline(always)]
    fn from(val: u8) -> Pwmen0 {
        Pwmen0::from_bits(val)
    }
}
impl From<Pwmen0> for u8 {
    #[inline(always)]
    fn from(val: Pwmen0) -> u8 {
        Pwmen0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwmen1 {
    #[doc = "Disable."]
    Match = 0x0,
    #[doc = "Enable."]
    Pwm = 0x01,
}
impl Pwmen1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwmen1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwmen1 {
    #[inline(always)]
    fn from(val: u8) -> Pwmen1 {
        Pwmen1::from_bits(val)
    }
}
impl From<Pwmen1> for u8 {
    #[inline(always)]
    fn from(val: Pwmen1) -> u8 {
        Pwmen1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwmen2 {
    #[doc = "Disable."]
    Match = 0x0,
    #[doc = "Enable."]
    Pwm = 0x01,
}
impl Pwmen2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwmen2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwmen2 {
    #[inline(always)]
    fn from(val: u8) -> Pwmen2 {
        Pwmen2::from_bits(val)
    }
}
impl From<Pwmen2> for u8 {
    #[inline(always)]
    fn from(val: Pwmen2) -> u8 {
        Pwmen2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwmen3 {
    #[doc = "Disable."]
    Match = 0x0,
    #[doc = "Enable."]
    Pwm = 0x01,
}
impl Pwmen3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwmen3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwmen3 {
    #[inline(always)]
    fn from(val: u8) -> Pwmen3 {
        Pwmen3::from_bits(val)
    }
}
impl From<Pwmen3> for u8 {
    #[inline(always)]
    fn from(val: Pwmen3) -> u8 {
        Pwmen3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Selcc {
    #[doc = "Capture channel 0 rising edge."]
    Channel0Rising = 0x0,
    #[doc = "Capture channel 0 falling edge."]
    Channel0Falling = 0x01,
    #[doc = "Capture channel 1 rising edge."]
    Channel1Rising = 0x02,
    #[doc = "Capture channel 1 falling edge."]
    Channel1Falling = 0x03,
    #[doc = "Capture channel 2 rising edge."]
    Channel2Rising = 0x04,
    #[doc = "Capture channel 2 falling edge."]
    Channel2Falling = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Selcc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Selcc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Selcc {
    #[inline(always)]
    fn from(val: u8) -> Selcc {
        Selcc::from_bits(val)
    }
}
impl From<Selcc> for u8 {
    #[inline(always)]
    fn from(val: Selcc) -> u8 {
        Selcc::to_bits(val)
    }
}
