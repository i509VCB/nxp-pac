#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Addsel {
    #[doc = "Node MAC address programmed on PADDR1/2 registers."]
    VAL_MAC = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Addsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Addsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Addsel {
    #[inline(always)]
    fn from(val: u8) -> Addsel {
        Addsel::from_bits(val)
    }
}
impl From<Addsel> for u8 {
    #[inline(always)]
    fn from(val: Addsel) -> u8 {
        Addsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Holdtime {
    #[doc = "1 internal module clock cycle"]
    VAL_1 = 0x0,
    #[doc = "2 internal module clock cycles"]
    VAL2 = 0x01,
    #[doc = "3 internal module clock cycles"]
    VAL3 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "8 internal module clock cycles"]
    VAL8 = 0x07,
}
impl Holdtime {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Holdtime {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Holdtime {
    #[inline(always)]
    fn from(val: u8) -> Holdtime {
        Holdtime::from_bits(val)
    }
}
impl From<Holdtime> for u8 {
    #[inline(always)]
    fn from(val: Holdtime) -> u8 {
        Holdtime::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcsr0Tmode {
    #[doc = "Timer Channel is disabled."]
    TMR_DIS = 0x0,
    #[doc = "Timer Channel is configured for Input Capture on rising edge."]
    TMR_RE = 0x01,
    #[doc = "Timer Channel is configured for Input Capture on falling edge."]
    TMR_FE = 0x02,
    #[doc = "Timer Channel is configured for Input Capture on both edges."]
    TMR_BE = 0x03,
    #[doc = "Timer Channel is configured for Output Compare - software only."]
    TMR_OUT = 0x04,
    #[doc = "Timer Channel is configured for Output Compare - toggle output on compare."]
    TMR_TOGGLE = 0x05,
    #[doc = "Timer Channel is configured for Output Compare - clear output on compare."]
    TMR_CLR = 0x06,
    #[doc = "Timer Channel is configured for Output Compare - set output on compare."]
    TMR_SET_OUT = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Timer Channel is configured for Output Compare - set output on compare, clear output on overflow."]
    TMR_CLR_SET1 = 0x09,
    #[doc = "Timer Channel is configured for Output Compare - clear output on compare, set output on overflow."]
    TMR_CLR_SET = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "Timer Channel is configured for Output Compare - pulse output low on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
    TMR_OUT_CMP_LOW = 0x0e,
    #[doc = "Timer Channel is configured for Output Compare - pulse output high on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
    TMR_OUT_CMP_HIGH = 0x0f,
}
impl Tcsr0Tmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcsr0Tmode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcsr0Tmode {
    #[inline(always)]
    fn from(val: u8) -> Tcsr0Tmode {
        Tcsr0Tmode::from_bits(val)
    }
}
impl From<Tcsr0Tmode> for u8 {
    #[inline(always)]
    fn from(val: Tcsr0Tmode) -> u8 {
        Tcsr0Tmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcsr0Tpwc {
    #[doc = "Pulse width is one 1588-clock cycle."]
    VALW1 = 0x0,
    #[doc = "Pulse width is two 1588-clock cycles."]
    VALW2 = 0x01,
    #[doc = "Pulse width is three 1588-clock cycles."]
    VALW3 = 0x02,
    #[doc = "Pulse width is four 1588-clock cycles."]
    VALW4 = 0x03,
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
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    #[doc = "Pulse width is 32 1588-clock cycles."]
    VALW32 = 0x1f,
}
impl Tcsr0Tpwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcsr0Tpwc {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcsr0Tpwc {
    #[inline(always)]
    fn from(val: u8) -> Tcsr0Tpwc {
        Tcsr0Tpwc::from_bits(val)
    }
}
impl From<Tcsr0Tpwc> for u8 {
    #[inline(always)]
    fn from(val: Tcsr0Tpwc) -> u8 {
        Tcsr0Tpwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcsr1Tmode {
    #[doc = "Timer Channel is disabled."]
    TMR_DIS = 0x0,
    #[doc = "Timer Channel is configured for Input Capture on rising edge."]
    TMR_RE = 0x01,
    #[doc = "Timer Channel is configured for Input Capture on falling edge."]
    TMR_FE = 0x02,
    #[doc = "Timer Channel is configured for Input Capture on both edges."]
    TMR_BE = 0x03,
    #[doc = "Timer Channel is configured for Output Compare - software only."]
    TMR_OUT = 0x04,
    #[doc = "Timer Channel is configured for Output Compare - toggle output on compare."]
    TMR_TOGGLE = 0x05,
    #[doc = "Timer Channel is configured for Output Compare - clear output on compare."]
    TMR_CLR = 0x06,
    #[doc = "Timer Channel is configured for Output Compare - set output on compare."]
    TMR_SET_OUT = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Timer Channel is configured for Output Compare - set output on compare, clear output on overflow."]
    TMR_CLR_SET1 = 0x09,
    #[doc = "Timer Channel is configured for Output Compare - clear output on compare, set output on overflow."]
    TMR_CLR_SET = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "Timer Channel is configured for Output Compare - pulse output low on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
    TMR_OUT_CMP_LOW = 0x0e,
    #[doc = "Timer Channel is configured for Output Compare - pulse output high on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
    TMR_OUT_CMP_HIGH = 0x0f,
}
impl Tcsr1Tmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcsr1Tmode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcsr1Tmode {
    #[inline(always)]
    fn from(val: u8) -> Tcsr1Tmode {
        Tcsr1Tmode::from_bits(val)
    }
}
impl From<Tcsr1Tmode> for u8 {
    #[inline(always)]
    fn from(val: Tcsr1Tmode) -> u8 {
        Tcsr1Tmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcsr1Tpwc {
    #[doc = "Pulse width is one 1588-clock cycle."]
    VALW1 = 0x0,
    #[doc = "Pulse width is two 1588-clock cycles."]
    VALW2 = 0x01,
    #[doc = "Pulse width is three 1588-clock cycles."]
    VALW3 = 0x02,
    #[doc = "Pulse width is four 1588-clock cycles."]
    VALW4 = 0x03,
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
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    #[doc = "Pulse width is 32 1588-clock cycles."]
    VALW32 = 0x1f,
}
impl Tcsr1Tpwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcsr1Tpwc {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcsr1Tpwc {
    #[inline(always)]
    fn from(val: u8) -> Tcsr1Tpwc {
        Tcsr1Tpwc::from_bits(val)
    }
}
impl From<Tcsr1Tpwc> for u8 {
    #[inline(always)]
    fn from(val: Tcsr1Tpwc) -> u8 {
        Tcsr1Tpwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcsr2Tmode {
    #[doc = "Timer Channel is disabled."]
    TMR_DIS = 0x0,
    #[doc = "Timer Channel is configured for Input Capture on rising edge."]
    TMR_RE = 0x01,
    #[doc = "Timer Channel is configured for Input Capture on falling edge."]
    TMR_FE = 0x02,
    #[doc = "Timer Channel is configured for Input Capture on both edges."]
    TMR_BE = 0x03,
    #[doc = "Timer Channel is configured for Output Compare - software only."]
    TMR_OUT = 0x04,
    #[doc = "Timer Channel is configured for Output Compare - toggle output on compare."]
    TMR_TOGGLE = 0x05,
    #[doc = "Timer Channel is configured for Output Compare - clear output on compare."]
    TMR_CLR = 0x06,
    #[doc = "Timer Channel is configured for Output Compare - set output on compare."]
    TMR_SET_OUT = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Timer Channel is configured for Output Compare - set output on compare, clear output on overflow."]
    TMR_CLR_SET1 = 0x09,
    #[doc = "Timer Channel is configured for Output Compare - clear output on compare, set output on overflow."]
    TMR_CLR_SET = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "Timer Channel is configured for Output Compare - pulse output low on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
    TMR_OUT_CMP_LOW = 0x0e,
    #[doc = "Timer Channel is configured for Output Compare - pulse output high on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
    TMR_OUT_CMP_HIGH = 0x0f,
}
impl Tcsr2Tmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcsr2Tmode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcsr2Tmode {
    #[inline(always)]
    fn from(val: u8) -> Tcsr2Tmode {
        Tcsr2Tmode::from_bits(val)
    }
}
impl From<Tcsr2Tmode> for u8 {
    #[inline(always)]
    fn from(val: Tcsr2Tmode) -> u8 {
        Tcsr2Tmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcsr2Tpwc {
    #[doc = "Pulse width is one 1588-clock cycle."]
    VALW1 = 0x0,
    #[doc = "Pulse width is two 1588-clock cycles."]
    VALW2 = 0x01,
    #[doc = "Pulse width is three 1588-clock cycles."]
    VALW3 = 0x02,
    #[doc = "Pulse width is four 1588-clock cycles."]
    VALW4 = 0x03,
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
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    #[doc = "Pulse width is 32 1588-clock cycles."]
    VALW32 = 0x1f,
}
impl Tcsr2Tpwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcsr2Tpwc {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcsr2Tpwc {
    #[inline(always)]
    fn from(val: u8) -> Tcsr2Tpwc {
        Tcsr2Tpwc::from_bits(val)
    }
}
impl From<Tcsr2Tpwc> for u8 {
    #[inline(always)]
    fn from(val: Tcsr2Tpwc) -> u8 {
        Tcsr2Tpwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcsr3Tmode {
    #[doc = "Timer Channel is disabled."]
    TMR_DIS = 0x0,
    #[doc = "Timer Channel is configured for Input Capture on rising edge."]
    TMR_RE = 0x01,
    #[doc = "Timer Channel is configured for Input Capture on falling edge."]
    TMR_FE = 0x02,
    #[doc = "Timer Channel is configured for Input Capture on both edges."]
    TMR_BE = 0x03,
    #[doc = "Timer Channel is configured for Output Compare - software only."]
    TMR_OUT = 0x04,
    #[doc = "Timer Channel is configured for Output Compare - toggle output on compare."]
    TMR_TOGGLE = 0x05,
    #[doc = "Timer Channel is configured for Output Compare - clear output on compare."]
    TMR_CLR = 0x06,
    #[doc = "Timer Channel is configured for Output Compare - set output on compare."]
    TMR_SET_OUT = 0x07,
    _RESERVED_8 = 0x08,
    #[doc = "Timer Channel is configured for Output Compare - set output on compare, clear output on overflow."]
    TMR_CLR_SET1 = 0x09,
    #[doc = "Timer Channel is configured for Output Compare - clear output on compare, set output on overflow."]
    TMR_CLR_SET = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "Timer Channel is configured for Output Compare - pulse output low on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
    TMR_OUT_CMP_LOW = 0x0e,
    #[doc = "Timer Channel is configured for Output Compare - pulse output high on compare for 1 to 32 1588-clock cycles as specified by TPWC."]
    TMR_OUT_CMP_HIGH = 0x0f,
}
impl Tcsr3Tmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcsr3Tmode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcsr3Tmode {
    #[inline(always)]
    fn from(val: u8) -> Tcsr3Tmode {
        Tcsr3Tmode::from_bits(val)
    }
}
impl From<Tcsr3Tmode> for u8 {
    #[inline(always)]
    fn from(val: Tcsr3Tmode) -> u8 {
        Tcsr3Tmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcsr3Tpwc {
    #[doc = "Pulse width is one 1588-clock cycle."]
    VALW1 = 0x0,
    #[doc = "Pulse width is two 1588-clock cycles."]
    VALW2 = 0x01,
    #[doc = "Pulse width is three 1588-clock cycles."]
    VALW3 = 0x02,
    #[doc = "Pulse width is four 1588-clock cycles."]
    VALW4 = 0x03,
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
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    #[doc = "Pulse width is 32 1588-clock cycles."]
    VALW32 = 0x1f,
}
impl Tcsr3Tpwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcsr3Tpwc {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcsr3Tpwc {
    #[inline(always)]
    fn from(val: u8) -> Tcsr3Tpwc {
        Tcsr3Tpwc::from_bits(val)
    }
}
impl From<Tcsr3Tpwc> for u8 {
    #[inline(always)]
    fn from(val: Tcsr3Tpwc) -> u8 {
        Tcsr3Tpwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tfwr {
    #[doc = "64 bytes written."]
    VAL64_0 = 0x0,
    #[doc = "64 bytes written."]
    VAL64_1 = 0x01,
    #[doc = "128 bytes written."]
    VAL128 = 0x02,
    #[doc = "192 bytes written."]
    VAL192 = 0x03,
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
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    #[doc = "1984 bytes written."]
    VAL1984 = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Tfwr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tfwr {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tfwr {
    #[inline(always)]
    fn from(val: u8) -> Tfwr {
        Tfwr::from_bits(val)
    }
}
impl From<Tfwr> for u8 {
    #[inline(always)]
    fn from(val: Tfwr) -> u8 {
        Tfwr::to_bits(val)
    }
}
