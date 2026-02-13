#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Capfe {
    #[doc = "Does not load"]
    CAPFE0 = 0x0,
    #[doc = "Loads"]
    CAPFE1 = 0x01,
}
impl Capfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Capfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Capfe {
    #[inline(always)]
    fn from(val: u8) -> Capfe {
        Capfe::from_bits(val)
    }
}
impl From<Capfe> for u8 {
    #[inline(always)]
    fn from(val: Capfe) -> u8 {
        Capfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Capi {
    #[doc = "Does not generate"]
    CAPI0 = 0x0,
    #[doc = "Generates"]
    CAPI1 = 0x01,
}
impl Capi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Capi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Capi {
    #[inline(always)]
    fn from(val: u8) -> Capi {
        Capi::from_bits(val)
    }
}
impl From<Capi> for u8 {
    #[inline(always)]
    fn from(val: Capi) -> u8 {
        Capi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Capre {
    #[doc = "Does not load"]
    CAPRE0 = 0x0,
    #[doc = "Loads"]
    CAPRE1 = 0x01,
}
impl Capre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Capre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Capre {
    #[inline(always)]
    fn from(val: u8) -> Capre {
        Capre::from_bits(val)
    }
}
impl From<Capre> for u8 {
    #[inline(always)]
    fn from(val: Capre) -> u8 {
        Capre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cinsel {
    #[doc = "Channel 0, CAPn\\[0\\] for CTIMERn"]
    CHANNEL_0 = 0x0,
    #[doc = "Channel 1, CAPn\\[1\\] for CTIMERn"]
    CHANNEL_1 = 0x01,
    #[doc = "Channel 2, CAPn\\[2\\] for CTIMERn"]
    CHANNEL_2 = 0x02,
    #[doc = "Channel 3, CAPn\\[3\\] for CTIMERn"]
    CHANNEL_3 = 0x03,
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
    #[doc = "Timer mode"]
    TIMER = 0x0,
    #[doc = "Counter mode rising edge"]
    COUNTER_RISING_EDGE = 0x01,
    #[doc = "Counter mode falling edge"]
    COUNTER_FALLING_EDGE = 0x02,
    #[doc = "Counter mode dual edge"]
    COUNTER_DUAL_EDGE = 0x03,
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
pub enum Em {
    #[doc = "Low"]
    CLEAR = 0x0,
    #[doc = "High"]
    SET = 0x01,
}
impl Em {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Em {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Em {
    #[inline(always)]
    fn from(val: u8) -> Em {
        Em::from_bits(val)
    }
}
impl From<Em> for u8 {
    #[inline(always)]
    fn from(val: Em) -> u8 {
        Em::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Emc {
    #[doc = "Does nothing"]
    DO_NOTHING = 0x0,
    #[doc = "Goes low"]
    CLEAR = 0x01,
    #[doc = "Goes high"]
    SET = 0x02,
    #[doc = "Toggles"]
    TOGGLE = 0x03,
}
impl Emc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emc {
    #[inline(always)]
    fn from(val: u8) -> Emc {
        Emc::from_bits(val)
    }
}
impl From<Emc> for u8 {
    #[inline(always)]
    fn from(val: Emc) -> u8 {
        Emc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mri {
    #[doc = "Does not generate"]
    MRI0 = 0x0,
    #[doc = "Generates"]
    MRI1 = 0x01,
}
impl Mri {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mri {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mri {
    #[inline(always)]
    fn from(val: u8) -> Mri {
        Mri::from_bits(val)
    }
}
impl From<Mri> for u8 {
    #[inline(always)]
    fn from(val: Mri) -> u8 {
        Mri::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mrr {
    #[doc = "Does not reset"]
    MRR0 = 0x0,
    #[doc = "Resets"]
    MRR1 = 0x01,
}
impl Mrr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mrr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mrr {
    #[inline(always)]
    fn from(val: u8) -> Mrr {
        Mrr::from_bits(val)
    }
}
impl From<Mrr> for u8 {
    #[inline(always)]
    fn from(val: Mrr) -> u8 {
        Mrr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mrrl {
    #[doc = "Does not reload"]
    MRRL0 = 0x0,
    #[doc = "Reloads"]
    MRRL1 = 0x01,
}
impl Mrrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mrrl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mrrl {
    #[inline(always)]
    fn from(val: u8) -> Mrrl {
        Mrrl::from_bits(val)
    }
}
impl From<Mrrl> for u8 {
    #[inline(always)]
    fn from(val: Mrrl) -> u8 {
        Mrrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mrs {
    #[doc = "Does not stop"]
    MRS0 = 0x0,
    #[doc = "Stops"]
    MRS1 = 0x01,
}
impl Mrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mrs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mrs {
    #[inline(always)]
    fn from(val: u8) -> Mrs {
        Mrs::from_bits(val)
    }
}
impl From<Mrs> for u8 {
    #[inline(always)]
    fn from(val: Mrs) -> u8 {
        Mrs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwmen {
    #[doc = "Disable"]
    MATCH = 0x0,
    #[doc = "Enable"]
    PWM = 0x01,
}
impl Pwmen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwmen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwmen {
    #[inline(always)]
    fn from(val: u8) -> Pwmen {
        Pwmen::from_bits(val)
    }
}
impl From<Pwmen> for u8 {
    #[inline(always)]
    fn from(val: Pwmen) -> u8 {
        Pwmen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Selcc {
    #[doc = "Capture channel 0 rising edge"]
    CHANNEL_0_RISING = 0x0,
    #[doc = "Capture channel 0 falling edge"]
    CHANNEL_0_FALLING = 0x01,
    #[doc = "Capture channel 1 rising edge"]
    CHANNEL_1_RISING = 0x02,
    #[doc = "Capture channel 1 falling edge"]
    CHANNEL_1_FALLING = 0x03,
    #[doc = "Capture channel 2 rising edge"]
    CHANNEL_2_RISING = 0x04,
    #[doc = "Capture channel 2 falling edge"]
    CHANNEL_2_FALLING = 0x05,
    #[doc = "Capture channel 3 rising edge"]
    CHANNEL_3_RISING = 0x06,
    #[doc = "Capture channel 3 falling edge"]
    CHANNEL_3_FALLING = 0x07,
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
