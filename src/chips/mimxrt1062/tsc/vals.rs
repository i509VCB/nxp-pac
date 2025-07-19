#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DeGlitch {
    #[doc = "Normal function: 0x1fff ipg clock cycles; Low power mode: 0x9 low power clock cycles"]
    DE_GLITCH_0 = 0x0,
    #[doc = "Normal function: 0xfff ipg clock cycles; Low power mode: :0x7 low power clock cycles"]
    DE_GLITCH_1 = 0x01,
    #[doc = "Normal function: 0x7ff ipg clock cycles; Low power mode:0x5 low power clock cycles"]
    DE_GLITCH_2 = 0x02,
    #[doc = "Normal function: 0x3 ipg clock cycles; Low power mode:0x3 low power clock cycles"]
    DE_GLITCH_3 = 0x03,
}
impl DeGlitch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DeGlitch {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DeGlitch {
    #[inline(always)]
    fn from(val: u8) -> DeGlitch {
        DeGlitch::from_bits(val)
    }
}
impl From<DeGlitch> for u8 {
    #[inline(always)]
    fn from(val: DeGlitch) -> u8 {
        DeGlitch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DetectEnableFiveWire {
    #[doc = "Do not read five wire detect value, read default value from analogue"]
    DETECT_ENABLE_FIVE_WIRE_0 = 0x0,
    #[doc = "Read five wire detect status from analogue"]
    DETECT_ENABLE_FIVE_WIRE_1 = 0x01,
}
impl DetectEnableFiveWire {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DetectEnableFiveWire {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DetectEnableFiveWire {
    #[inline(always)]
    fn from(val: u8) -> DetectEnableFiveWire {
        DetectEnableFiveWire::from_bits(val)
    }
}
impl From<DetectEnableFiveWire> for u8 {
    #[inline(always)]
    fn from(val: DetectEnableFiveWire) -> u8 {
        DetectEnableFiveWire::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DetectEnableFourWire {
    #[doc = "Do not read four wire detect value, read default value from analogue"]
    DETECT_ENABLE_FOUR_WIRE_0 = 0x0,
    #[doc = "Read four wire detect status from analogue"]
    DETECT_ENABLE_FOUR_WIRE_1 = 0x01,
}
impl DetectEnableFourWire {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DetectEnableFourWire {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DetectEnableFourWire {
    #[inline(always)]
    fn from(val: u8) -> DetectEnableFourWire {
        DetectEnableFourWire::from_bits(val)
    }
}
impl From<DetectEnableFourWire> for u8 {
    #[inline(always)]
    fn from(val: DetectEnableFourWire) -> u8 {
        DetectEnableFourWire::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DetectFiveWire {
    #[doc = "No detect signal"]
    DETECT_FIVE_WIRE_0 = 0x0,
    #[doc = "Yes, there is a detect on the touch screen."]
    DETECT_FIVE_WIRE_1 = 0x01,
}
impl DetectFiveWire {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DetectFiveWire {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DetectFiveWire {
    #[inline(always)]
    fn from(val: u8) -> DetectFiveWire {
        DetectFiveWire::from_bits(val)
    }
}
impl From<DetectFiveWire> for u8 {
    #[inline(always)]
    fn from(val: DetectFiveWire) -> u8 {
        DetectFiveWire::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DetectFourWire {
    #[doc = "No detect signal"]
    DETECT_FOUR_WIRE_0 = 0x0,
    #[doc = "Yes, there is a detect on the touch screen."]
    DETECT_FOUR_WIRE_1 = 0x01,
}
impl DetectFourWire {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DetectFourWire {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DetectFourWire {
    #[inline(always)]
    fn from(val: u8) -> DetectFourWire {
        DetectFourWire::from_bits(val)
    }
}
impl From<DetectFourWire> for u8 {
    #[inline(always)]
    fn from(val: DetectFourWire) -> u8 {
        DetectFourWire::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StateMachine {
    #[doc = "Idle"]
    STATE_MACHINE_0 = 0x0,
    #[doc = "Pre-charge"]
    STATE_MACHINE_1 = 0x01,
    #[doc = "Detect"]
    STATE_MACHINE_2 = 0x02,
    #[doc = "X-measure"]
    STATE_MACHINE_3 = 0x03,
    #[doc = "Y-measure"]
    STATE_MACHINE_4 = 0x04,
    #[doc = "Pre-charge"]
    STATE_MACHINE_5 = 0x05,
    #[doc = "Detect"]
    STATE_MACHINE_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl StateMachine {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StateMachine {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StateMachine {
    #[inline(always)]
    fn from(val: u8) -> StateMachine {
        StateMachine::from_bits(val)
    }
}
impl From<StateMachine> for u8 {
    #[inline(always)]
    fn from(val: StateMachine) -> u8 {
        StateMachine::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wire45 {
    #[doc = "4-Wire Detection Mode"]
    WIRE_4_5_0 = 0x0,
    #[doc = "5-Wire Detection Mode"]
    WIRE_4_5_1 = 0x01,
}
impl Wire45 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wire45 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wire45 {
    #[inline(always)]
    fn from(val: u8) -> Wire45 {
        Wire45::from_bits(val)
    }
}
impl From<Wire45> for u8 {
    #[inline(always)]
    fn from(val: Wire45) -> u8 {
        Wire45::to_bits(val)
    }
}
