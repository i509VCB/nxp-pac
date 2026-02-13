#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg0 {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH = 0x0,
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGE = 0x01,
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE = 0x02,
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_FALLING_EDGE = 0x03,
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL = 0x04,
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL = 0x05,
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_ZERO = 0x06,
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    EVENT = 0x07,
}
impl Cfg0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg0 {
    #[inline(always)]
    fn from(val: u8) -> Cfg0 {
        Cfg0::from_bits(val)
    }
}
impl From<Cfg0> for u8 {
    #[inline(always)]
    fn from(val: Cfg0) -> u8 {
        Cfg0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg1 {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH = 0x0,
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGE = 0x01,
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE = 0x02,
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_FALLING_EDGE = 0x03,
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL = 0x04,
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL = 0x05,
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_ZERO = 0x06,
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    EVENT = 0x07,
}
impl Cfg1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg1 {
    #[inline(always)]
    fn from(val: u8) -> Cfg1 {
        Cfg1::from_bits(val)
    }
}
impl From<Cfg1> for u8 {
    #[inline(always)]
    fn from(val: Cfg1) -> u8 {
        Cfg1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg2 {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH = 0x0,
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGE = 0x01,
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE = 0x02,
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_FALLING_EDGE = 0x03,
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL = 0x04,
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL = 0x05,
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_ZERO = 0x06,
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    EVENT = 0x07,
}
impl Cfg2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg2 {
    #[inline(always)]
    fn from(val: u8) -> Cfg2 {
        Cfg2::from_bits(val)
    }
}
impl From<Cfg2> for u8 {
    #[inline(always)]
    fn from(val: Cfg2) -> u8 {
        Cfg2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg3 {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH = 0x0,
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGE = 0x01,
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE = 0x02,
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_FALLING_EDGE = 0x03,
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL = 0x04,
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL = 0x05,
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_ZERO = 0x06,
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    EVENT = 0x07,
}
impl Cfg3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg3 {
    #[inline(always)]
    fn from(val: u8) -> Cfg3 {
        Cfg3::from_bits(val)
    }
}
impl From<Cfg3> for u8 {
    #[inline(always)]
    fn from(val: Cfg3) -> u8 {
        Cfg3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg4 {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH = 0x0,
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGE = 0x01,
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE = 0x02,
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_FALLING_EDGE = 0x03,
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL = 0x04,
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL = 0x05,
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_ZERO = 0x06,
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    EVENT = 0x07,
}
impl Cfg4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg4 {
    #[inline(always)]
    fn from(val: u8) -> Cfg4 {
        Cfg4::from_bits(val)
    }
}
impl From<Cfg4> for u8 {
    #[inline(always)]
    fn from(val: Cfg4) -> u8 {
        Cfg4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg5 {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH = 0x0,
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGE = 0x01,
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE = 0x02,
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_FALLING_EDGE = 0x03,
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL = 0x04,
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL = 0x05,
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_ZERO = 0x06,
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    EVENT = 0x07,
}
impl Cfg5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg5 {
    #[inline(always)]
    fn from(val: u8) -> Cfg5 {
        Cfg5::from_bits(val)
    }
}
impl From<Cfg5> for u8 {
    #[inline(always)]
    fn from(val: Cfg5) -> u8 {
        Cfg5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg6 {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH = 0x0,
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGE = 0x01,
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE = 0x02,
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_FALLING_EDGE = 0x03,
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL = 0x04,
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL = 0x05,
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_ZERO = 0x06,
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    EVENT = 0x07,
}
impl Cfg6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg6 {
    #[inline(always)]
    fn from(val: u8) -> Cfg6 {
        Cfg6::from_bits(val)
    }
}
impl From<Cfg6> for u8 {
    #[inline(always)]
    fn from(val: Cfg6) -> u8 {
        Cfg6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfg7 {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH = 0x0,
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGE = 0x01,
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE = 0x02,
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_FALLING_EDGE = 0x03,
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL = 0x04,
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL = 0x05,
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_ZERO = 0x06,
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    EVENT = 0x07,
}
impl Cfg7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfg7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfg7 {
    #[inline(always)]
    fn from(val: u8) -> Cfg7 {
        Cfg7::from_bits(val)
    }
}
impl From<Cfg7> for u8 {
    #[inline(always)]
    fn from(val: Cfg7) -> u8 {
        Cfg7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ProdEndpts0 {
    #[doc = "No effect. Slice 0 is not an endpoint."]
    NO_EFFECT = 0x0,
    #[doc = "endpoint. Slice 0 is the endpoint of a product term (minterm). Pin interrupt 0 in the NVIC is raised if the minterm evaluates as true."]
    ENDPOINT = 0x01,
}
impl ProdEndpts0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ProdEndpts0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ProdEndpts0 {
    #[inline(always)]
    fn from(val: u8) -> ProdEndpts0 {
        ProdEndpts0::from_bits(val)
    }
}
impl From<ProdEndpts0> for u8 {
    #[inline(always)]
    fn from(val: ProdEndpts0) -> u8 {
        ProdEndpts0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ProdEndpts1 {
    #[doc = "No effect. Slice 1 is not an endpoint."]
    NO_EFFECT = 0x0,
    #[doc = "endpoint. Slice 1 is the endpoint of a product term (minterm). Pin interrupt 1 in the NVIC is raised if the minterm evaluates as true."]
    ENDPOINT = 0x01,
}
impl ProdEndpts1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ProdEndpts1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ProdEndpts1 {
    #[inline(always)]
    fn from(val: u8) -> ProdEndpts1 {
        ProdEndpts1::from_bits(val)
    }
}
impl From<ProdEndpts1> for u8 {
    #[inline(always)]
    fn from(val: ProdEndpts1) -> u8 {
        ProdEndpts1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ProdEndpts2 {
    #[doc = "No effect. Slice 2 is not an endpoint."]
    NO_EFFECT = 0x0,
    #[doc = "endpoint. Slice 2 is the endpoint of a product term (minterm). Pin interrupt 2 in the NVIC is raised if the minterm evaluates as true."]
    ENDPOINT = 0x01,
}
impl ProdEndpts2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ProdEndpts2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ProdEndpts2 {
    #[inline(always)]
    fn from(val: u8) -> ProdEndpts2 {
        ProdEndpts2::from_bits(val)
    }
}
impl From<ProdEndpts2> for u8 {
    #[inline(always)]
    fn from(val: ProdEndpts2) -> u8 {
        ProdEndpts2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ProdEndpts3 {
    #[doc = "No effect. Slice 3 is not an endpoint."]
    NO_EFFECT = 0x0,
    #[doc = "endpoint. Slice 3 is the endpoint of a product term (minterm). Pin interrupt 3 in the NVIC is raised if the minterm evaluates as true."]
    ENDPOINT = 0x01,
}
impl ProdEndpts3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ProdEndpts3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ProdEndpts3 {
    #[inline(always)]
    fn from(val: u8) -> ProdEndpts3 {
        ProdEndpts3::from_bits(val)
    }
}
impl From<ProdEndpts3> for u8 {
    #[inline(always)]
    fn from(val: ProdEndpts3) -> u8 {
        ProdEndpts3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ProdEndpts4 {
    #[doc = "No effect. Slice 4 is not an endpoint."]
    NO_EFFECT = 0x0,
    #[doc = "endpoint. Slice 4 is the endpoint of a product term (minterm). Pin interrupt 4 in the NVIC is raised if the minterm evaluates as true."]
    ENDPOINT = 0x01,
}
impl ProdEndpts4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ProdEndpts4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ProdEndpts4 {
    #[inline(always)]
    fn from(val: u8) -> ProdEndpts4 {
        ProdEndpts4::from_bits(val)
    }
}
impl From<ProdEndpts4> for u8 {
    #[inline(always)]
    fn from(val: ProdEndpts4) -> u8 {
        ProdEndpts4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ProdEndpts5 {
    #[doc = "No effect. Slice 5 is not an endpoint."]
    NO_EFFECT = 0x0,
    #[doc = "endpoint. Slice 5 is the endpoint of a product term (minterm). Pin interrupt 5 in the NVIC is raised if the minterm evaluates as true."]
    ENDPOINT = 0x01,
}
impl ProdEndpts5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ProdEndpts5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ProdEndpts5 {
    #[inline(always)]
    fn from(val: u8) -> ProdEndpts5 {
        ProdEndpts5::from_bits(val)
    }
}
impl From<ProdEndpts5> for u8 {
    #[inline(always)]
    fn from(val: ProdEndpts5) -> u8 {
        ProdEndpts5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ProdEndpts6 {
    #[doc = "No effect. Slice 6 is not an endpoint."]
    NO_EFFECT = 0x0,
    #[doc = "endpoint. Slice 6 is the endpoint of a product term (minterm). Pin interrupt 6 in the NVIC is raised if the minterm evaluates as true."]
    ENDPOINT = 0x01,
}
impl ProdEndpts6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ProdEndpts6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ProdEndpts6 {
    #[inline(always)]
    fn from(val: u8) -> ProdEndpts6 {
        ProdEndpts6::from_bits(val)
    }
}
impl From<ProdEndpts6> for u8 {
    #[inline(always)]
    fn from(val: ProdEndpts6) -> u8 {
        ProdEndpts6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SelPmatch {
    #[doc = "Pin interrupt. Interrupts are driven in response to the standard pin interrupt function."]
    PIN_INTERRUPT = 0x0,
    #[doc = "Pattern match. Interrupts are driven in response to pattern matches."]
    PATTERN_MATCH = 0x01,
}
impl SelPmatch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SelPmatch {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SelPmatch {
    #[inline(always)]
    fn from(val: u8) -> SelPmatch {
        SelPmatch::from_bits(val)
    }
}
impl From<SelPmatch> for u8 {
    #[inline(always)]
    fn from(val: SelPmatch) -> u8 {
        SelPmatch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Src0 {
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 0."]
    INPUT0 = 0x0,
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 0."]
    INPUT1 = 0x01,
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 0."]
    INPUT2 = 0x02,
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 0."]
    INPUT3 = 0x03,
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 0."]
    INPUT4 = 0x04,
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 0."]
    INPUT5 = 0x05,
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 0."]
    INPUT6 = 0x06,
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 0."]
    INPUT7 = 0x07,
}
impl Src0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Src0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Src0 {
    #[inline(always)]
    fn from(val: u8) -> Src0 {
        Src0::from_bits(val)
    }
}
impl From<Src0> for u8 {
    #[inline(always)]
    fn from(val: Src0) -> u8 {
        Src0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Src1 {
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 1."]
    INPUT0 = 0x0,
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 1."]
    INPUT1 = 0x01,
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 1."]
    INPUT2 = 0x02,
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 1."]
    INPUT3 = 0x03,
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 1."]
    INPUT4 = 0x04,
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 1."]
    INPUT5 = 0x05,
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 1."]
    INPUT6 = 0x06,
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 1."]
    INPUT7 = 0x07,
}
impl Src1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Src1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Src1 {
    #[inline(always)]
    fn from(val: u8) -> Src1 {
        Src1::from_bits(val)
    }
}
impl From<Src1> for u8 {
    #[inline(always)]
    fn from(val: Src1) -> u8 {
        Src1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Src2 {
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 2."]
    INPUT0 = 0x0,
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 2."]
    INPUT1 = 0x01,
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 2."]
    INPUT2 = 0x02,
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 2."]
    INPUT3 = 0x03,
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 2."]
    INPUT4 = 0x04,
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 2."]
    INPUT5 = 0x05,
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 2."]
    INPUT6 = 0x06,
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 2."]
    INPUT7 = 0x07,
}
impl Src2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Src2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Src2 {
    #[inline(always)]
    fn from(val: u8) -> Src2 {
        Src2::from_bits(val)
    }
}
impl From<Src2> for u8 {
    #[inline(always)]
    fn from(val: Src2) -> u8 {
        Src2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Src3 {
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 3."]
    INPUT0 = 0x0,
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 3."]
    INPUT1 = 0x01,
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 3."]
    INPUT2 = 0x02,
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 3."]
    INPUT3 = 0x03,
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 3."]
    INPUT4 = 0x04,
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 3."]
    INPUT5 = 0x05,
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 3."]
    INPUT6 = 0x06,
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 3."]
    INPUT7 = 0x07,
}
impl Src3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Src3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Src3 {
    #[inline(always)]
    fn from(val: u8) -> Src3 {
        Src3::from_bits(val)
    }
}
impl From<Src3> for u8 {
    #[inline(always)]
    fn from(val: Src3) -> u8 {
        Src3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Src4 {
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 4."]
    INPUT0 = 0x0,
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 4."]
    INPUT1 = 0x01,
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 4."]
    INPUT2 = 0x02,
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 4."]
    INPUT3 = 0x03,
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 4."]
    INPUT4 = 0x04,
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 4."]
    INPUT5 = 0x05,
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 4."]
    INPUT6 = 0x06,
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 4."]
    INPUT7 = 0x07,
}
impl Src4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Src4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Src4 {
    #[inline(always)]
    fn from(val: u8) -> Src4 {
        Src4::from_bits(val)
    }
}
impl From<Src4> for u8 {
    #[inline(always)]
    fn from(val: Src4) -> u8 {
        Src4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Src5 {
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 5."]
    INPUT0 = 0x0,
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 5."]
    INPUT1 = 0x01,
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 5."]
    INPUT2 = 0x02,
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 5."]
    INPUT3 = 0x03,
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 5."]
    INPUT4 = 0x04,
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 5."]
    INPUT5 = 0x05,
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 5."]
    INPUT6 = 0x06,
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 5."]
    INPUT7 = 0x07,
}
impl Src5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Src5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Src5 {
    #[inline(always)]
    fn from(val: u8) -> Src5 {
        Src5::from_bits(val)
    }
}
impl From<Src5> for u8 {
    #[inline(always)]
    fn from(val: Src5) -> u8 {
        Src5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Src6 {
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 6."]
    INPUT0 = 0x0,
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 6."]
    INPUT1 = 0x01,
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 6."]
    INPUT2 = 0x02,
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 6."]
    INPUT3 = 0x03,
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 6."]
    INPUT4 = 0x04,
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 6."]
    INPUT5 = 0x05,
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 6."]
    INPUT6 = 0x06,
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 6."]
    INPUT7 = 0x07,
}
impl Src6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Src6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Src6 {
    #[inline(always)]
    fn from(val: u8) -> Src6 {
        Src6::from_bits(val)
    }
}
impl From<Src6> for u8 {
    #[inline(always)]
    fn from(val: Src6) -> u8 {
        Src6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Src7 {
    #[doc = "Input 0. Selects the pin selected in the PINTSEL0 register as the source to bit slice 7."]
    INPUT0 = 0x0,
    #[doc = "Input 1. Selects the pin selected in the PINTSEL1 register as the source to bit slice 7."]
    INPUT1 = 0x01,
    #[doc = "Input 2. Selects the pin selected in the PINTSEL2 register as the source to bit slice 7."]
    INPUT2 = 0x02,
    #[doc = "Input 3. Selects the pin selected in the PINTSEL3 register as the source to bit slice 7."]
    INPUT3 = 0x03,
    #[doc = "Input 4. Selects the pin selected in the PINTSEL4 register as the source to bit slice 7."]
    INPUT4 = 0x04,
    #[doc = "Input 5. Selects the pin selected in the PINTSEL5 register as the source to bit slice 7."]
    INPUT5 = 0x05,
    #[doc = "Input 6. Selects the pin selected in the PINTSEL6 register as the source to bit slice 7."]
    INPUT6 = 0x06,
    #[doc = "Input 7. Selects the pin selected in the PINTSEL7 register as the source to bit slice 7."]
    INPUT7 = 0x07,
}
impl Src7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Src7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Src7 {
    #[inline(always)]
    fn from(val: u8) -> Src7 {
        Src7::from_bits(val)
    }
}
impl From<Src7> for u8 {
    #[inline(always)]
    fn from(val: Src7) -> u8 {
        Src7::to_bits(val)
    }
}
