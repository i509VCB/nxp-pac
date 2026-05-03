#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CFG0 {
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
impl CFG0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CFG0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CFG0 {
    #[inline(always)]
    fn from(val: u8) -> CFG0 {
        CFG0::from_bits(val)
    }
}
impl From<CFG0> for u8 {
    #[inline(always)]
    fn from(val: CFG0) -> u8 {
        CFG0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CFG1 {
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
impl CFG1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CFG1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CFG1 {
    #[inline(always)]
    fn from(val: u8) -> CFG1 {
        CFG1::from_bits(val)
    }
}
impl From<CFG1> for u8 {
    #[inline(always)]
    fn from(val: CFG1) -> u8 {
        CFG1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CFG2 {
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
impl CFG2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CFG2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CFG2 {
    #[inline(always)]
    fn from(val: u8) -> CFG2 {
        CFG2::from_bits(val)
    }
}
impl From<CFG2> for u8 {
    #[inline(always)]
    fn from(val: CFG2) -> u8 {
        CFG2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CFG3 {
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
impl CFG3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CFG3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CFG3 {
    #[inline(always)]
    fn from(val: u8) -> CFG3 {
        CFG3::from_bits(val)
    }
}
impl From<CFG3> for u8 {
    #[inline(always)]
    fn from(val: CFG3) -> u8 {
        CFG3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CFG4 {
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
impl CFG4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CFG4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CFG4 {
    #[inline(always)]
    fn from(val: u8) -> CFG4 {
        CFG4::from_bits(val)
    }
}
impl From<CFG4> for u8 {
    #[inline(always)]
    fn from(val: CFG4) -> u8 {
        CFG4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CFG5 {
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
impl CFG5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CFG5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CFG5 {
    #[inline(always)]
    fn from(val: u8) -> CFG5 {
        CFG5::from_bits(val)
    }
}
impl From<CFG5> for u8 {
    #[inline(always)]
    fn from(val: CFG5) -> u8 {
        CFG5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CFG6 {
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
impl CFG6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CFG6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CFG6 {
    #[inline(always)]
    fn from(val: u8) -> CFG6 {
        CFG6::from_bits(val)
    }
}
impl From<CFG6> for u8 {
    #[inline(always)]
    fn from(val: CFG6) -> u8 {
        CFG6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CFG7 {
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
impl CFG7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CFG7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CFG7 {
    #[inline(always)]
    fn from(val: u8) -> CFG7 {
        CFG7::from_bits(val)
    }
}
impl From<CFG7> for u8 {
    #[inline(always)]
    fn from(val: CFG7) -> u8 {
        CFG7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PROD_ENDPTS0 {
    #[doc = "No effect. Slice 0 is not an endpoint."]
    NO_EFFECT = 0x0,
    #[doc = "endpoint. Slice 0 is the endpoint of a product term (minterm). Pin interrupt 0 in the NVIC is raised if the minterm evaluates as true."]
    ENDPOINT = 0x01,
}
impl PROD_ENDPTS0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PROD_ENDPTS0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PROD_ENDPTS0 {
    #[inline(always)]
    fn from(val: u8) -> PROD_ENDPTS0 {
        PROD_ENDPTS0::from_bits(val)
    }
}
impl From<PROD_ENDPTS0> for u8 {
    #[inline(always)]
    fn from(val: PROD_ENDPTS0) -> u8 {
        PROD_ENDPTS0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PROD_ENDPTS1 {
    #[doc = "No effect. Slice 1 is not an endpoint."]
    NO_EFFECT = 0x0,
    #[doc = "endpoint. Slice 1 is the endpoint of a product term (minterm). Pin interrupt 1 in the NVIC is raised if the minterm evaluates as true."]
    ENDPOINT = 0x01,
}
impl PROD_ENDPTS1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PROD_ENDPTS1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PROD_ENDPTS1 {
    #[inline(always)]
    fn from(val: u8) -> PROD_ENDPTS1 {
        PROD_ENDPTS1::from_bits(val)
    }
}
impl From<PROD_ENDPTS1> for u8 {
    #[inline(always)]
    fn from(val: PROD_ENDPTS1) -> u8 {
        PROD_ENDPTS1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PROD_ENDPTS2 {
    #[doc = "No effect. Slice 2 is not an endpoint."]
    NO_EFFECT = 0x0,
    #[doc = "endpoint. Slice 2 is the endpoint of a product term (minterm). Pin interrupt 2 in the NVIC is raised if the minterm evaluates as true."]
    ENDPOINT = 0x01,
}
impl PROD_ENDPTS2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PROD_ENDPTS2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PROD_ENDPTS2 {
    #[inline(always)]
    fn from(val: u8) -> PROD_ENDPTS2 {
        PROD_ENDPTS2::from_bits(val)
    }
}
impl From<PROD_ENDPTS2> for u8 {
    #[inline(always)]
    fn from(val: PROD_ENDPTS2) -> u8 {
        PROD_ENDPTS2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PROD_ENDPTS3 {
    #[doc = "No effect. Slice 3 is not an endpoint."]
    NO_EFFECT = 0x0,
    #[doc = "endpoint. Slice 3 is the endpoint of a product term (minterm). Pin interrupt 3 in the NVIC is raised if the minterm evaluates as true."]
    ENDPOINT = 0x01,
}
impl PROD_ENDPTS3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PROD_ENDPTS3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PROD_ENDPTS3 {
    #[inline(always)]
    fn from(val: u8) -> PROD_ENDPTS3 {
        PROD_ENDPTS3::from_bits(val)
    }
}
impl From<PROD_ENDPTS3> for u8 {
    #[inline(always)]
    fn from(val: PROD_ENDPTS3) -> u8 {
        PROD_ENDPTS3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PROD_ENDPTS4 {
    #[doc = "No effect. Slice 4 is not an endpoint."]
    NO_EFFECT = 0x0,
    #[doc = "endpoint. Slice 4 is the endpoint of a product term (minterm). Pin interrupt 4 in the NVIC is raised if the minterm evaluates as true."]
    ENDPOINT = 0x01,
}
impl PROD_ENDPTS4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PROD_ENDPTS4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PROD_ENDPTS4 {
    #[inline(always)]
    fn from(val: u8) -> PROD_ENDPTS4 {
        PROD_ENDPTS4::from_bits(val)
    }
}
impl From<PROD_ENDPTS4> for u8 {
    #[inline(always)]
    fn from(val: PROD_ENDPTS4) -> u8 {
        PROD_ENDPTS4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PROD_ENDPTS5 {
    #[doc = "No effect. Slice 5 is not an endpoint."]
    NO_EFFECT = 0x0,
    #[doc = "endpoint. Slice 5 is the endpoint of a product term (minterm). Pin interrupt 5 in the NVIC is raised if the minterm evaluates as true."]
    ENDPOINT = 0x01,
}
impl PROD_ENDPTS5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PROD_ENDPTS5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PROD_ENDPTS5 {
    #[inline(always)]
    fn from(val: u8) -> PROD_ENDPTS5 {
        PROD_ENDPTS5::from_bits(val)
    }
}
impl From<PROD_ENDPTS5> for u8 {
    #[inline(always)]
    fn from(val: PROD_ENDPTS5) -> u8 {
        PROD_ENDPTS5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PROD_ENDPTS6 {
    #[doc = "No effect. Slice 6 is not an endpoint."]
    NO_EFFECT = 0x0,
    #[doc = "endpoint. Slice 6 is the endpoint of a product term (minterm). Pin interrupt 6 in the NVIC is raised if the minterm evaluates as true."]
    ENDPOINT = 0x01,
}
impl PROD_ENDPTS6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PROD_ENDPTS6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PROD_ENDPTS6 {
    #[inline(always)]
    fn from(val: u8) -> PROD_ENDPTS6 {
        PROD_ENDPTS6::from_bits(val)
    }
}
impl From<PROD_ENDPTS6> for u8 {
    #[inline(always)]
    fn from(val: PROD_ENDPTS6) -> u8 {
        PROD_ENDPTS6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEL_PMATCH {
    #[doc = "Pin interrupt. Interrupts are driven in response to the standard pin interrupt function."]
    PIN_INTERRUPT = 0x0,
    #[doc = "Pattern match. Interrupts are driven in response to pattern matches."]
    PATTERN_MATCH = 0x01,
}
impl SEL_PMATCH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEL_PMATCH {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEL_PMATCH {
    #[inline(always)]
    fn from(val: u8) -> SEL_PMATCH {
        SEL_PMATCH::from_bits(val)
    }
}
impl From<SEL_PMATCH> for u8 {
    #[inline(always)]
    fn from(val: SEL_PMATCH) -> u8 {
        SEL_PMATCH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SRC0 {
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
impl SRC0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SRC0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SRC0 {
    #[inline(always)]
    fn from(val: u8) -> SRC0 {
        SRC0::from_bits(val)
    }
}
impl From<SRC0> for u8 {
    #[inline(always)]
    fn from(val: SRC0) -> u8 {
        SRC0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SRC1 {
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
impl SRC1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SRC1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SRC1 {
    #[inline(always)]
    fn from(val: u8) -> SRC1 {
        SRC1::from_bits(val)
    }
}
impl From<SRC1> for u8 {
    #[inline(always)]
    fn from(val: SRC1) -> u8 {
        SRC1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SRC2 {
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
impl SRC2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SRC2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SRC2 {
    #[inline(always)]
    fn from(val: u8) -> SRC2 {
        SRC2::from_bits(val)
    }
}
impl From<SRC2> for u8 {
    #[inline(always)]
    fn from(val: SRC2) -> u8 {
        SRC2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SRC3 {
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
impl SRC3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SRC3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SRC3 {
    #[inline(always)]
    fn from(val: u8) -> SRC3 {
        SRC3::from_bits(val)
    }
}
impl From<SRC3> for u8 {
    #[inline(always)]
    fn from(val: SRC3) -> u8 {
        SRC3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SRC4 {
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
impl SRC4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SRC4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SRC4 {
    #[inline(always)]
    fn from(val: u8) -> SRC4 {
        SRC4::from_bits(val)
    }
}
impl From<SRC4> for u8 {
    #[inline(always)]
    fn from(val: SRC4) -> u8 {
        SRC4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SRC5 {
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
impl SRC5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SRC5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SRC5 {
    #[inline(always)]
    fn from(val: u8) -> SRC5 {
        SRC5::from_bits(val)
    }
}
impl From<SRC5> for u8 {
    #[inline(always)]
    fn from(val: SRC5) -> u8 {
        SRC5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SRC6 {
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
impl SRC6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SRC6 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SRC6 {
    #[inline(always)]
    fn from(val: u8) -> SRC6 {
        SRC6::from_bits(val)
    }
}
impl From<SRC6> for u8 {
    #[inline(always)]
    fn from(val: SRC6) -> u8 {
        SRC6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SRC7 {
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
impl SRC7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SRC7 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SRC7 {
    #[inline(always)]
    fn from(val: u8) -> SRC7 {
        SRC7::from_bits(val)
    }
}
impl From<SRC7> for u8 {
    #[inline(always)]
    fn from(val: SRC7) -> u8 {
        SRC7::to_bits(val)
    }
}
