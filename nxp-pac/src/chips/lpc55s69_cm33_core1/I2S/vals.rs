#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BUSY {
    #[doc = "The transmitter/receiver for channel pair is currently idle."]
    IDLE = 0x0,
    #[doc = "The transmitter/receiver for channel pair is currently processing data."]
    BUSY = 0x01,
}
impl BUSY {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BUSY {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BUSY {
    #[inline(always)]
    fn from(val: u8) -> BUSY {
        BUSY::from_bits(val)
    }
}
impl From<BUSY> for u8 {
    #[inline(always)]
    fn from(val: BUSY) -> u8 {
        BUSY::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DATAPAUSE {
    #[doc = "Normal operation, or resuming normal operation at the next frame if the I2S has already been paused."]
    NORMAL = 0x0,
    #[doc = "A pause in the data flow is being requested. It is in effect when DATAPAUSED in STAT = 1."]
    PAUSE = 0x01,
}
impl DATAPAUSE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DATAPAUSE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DATAPAUSE {
    #[inline(always)]
    fn from(val: u8) -> DATAPAUSE {
        DATAPAUSE::from_bits(val)
    }
}
impl From<DATAPAUSE> for u8 {
    #[inline(always)]
    fn from(val: DATAPAUSE) -> u8 {
        DATAPAUSE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LEFTJUST {
    #[doc = "Data is transferred between the FIFO and the I2S serializer/deserializer right justified, i.e. starting from bit 0 and continuing to the position defined by DATALEN. This would correspond to right justified data in the stream on the data bus."]
    RIGHT_JUSTIFIED = 0x0,
    #[doc = "Data is transferred between the FIFO and the I2S serializer/deserializer left justified, i.e. starting from the MSB of the FIFO entry and continuing for the number of bits defined by DATALEN. This would correspond to left justified data in the stream on the data bus."]
    LEFT_JUSTIFIED = 0x01,
}
impl LEFTJUST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LEFTJUST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LEFTJUST {
    #[inline(always)]
    fn from(val: u8) -> LEFTJUST {
        LEFTJUST::from_bits(val)
    }
}
impl From<LEFTJUST> for u8 {
    #[inline(always)]
    fn from(val: LEFTJUST) -> u8 {
        LEFTJUST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LR {
    #[doc = "Left channel."]
    LEFT_CHANNEL = 0x0,
    #[doc = "Right channel."]
    RIGHT_CHANNEL = 0x01,
}
impl LR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LR {
    #[inline(always)]
    fn from(val: u8) -> LR {
        LR::from_bits(val)
    }
}
impl From<LR> for u8 {
    #[inline(always)]
    fn from(val: LR) -> u8 {
        LR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MODE {
    #[doc = "I2S mode a.k.a. 'classic' mode. WS has a 50% duty cycle, with (for each enabled channel pair) one piece of left channel data occurring during the first phase, and one pieces of right channel data occurring during the second phase. In this mode, the data region begins one clock after the leading WS edge for the frame. For a 50% WS duty cycle, FRAMELEN must define an even number of I2S clocks for the frame. If FRAMELEN defines an odd number of clocks per frame, the extra clock will occur on the right."]
    CLASSIC_MODE = 0x0,
    #[doc = "DSP mode where WS has a 50% duty cycle. See remark for mode 0."]
    DSP_MODE_WS_50_DUTYCYCLE = 0x01,
    #[doc = "DSP mode where WS has a one clock long pulse at the beginning of each data frame."]
    DSP_MODE_WS_1_CLOCK = 0x02,
    #[doc = "DSP mode where WS has a one data slot long pulse at the beginning of each data frame."]
    DSP_MODE_WS_1_DATA = 0x03,
}
impl MODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MODE {
    #[inline(always)]
    fn from(val: u8) -> MODE {
        MODE::from_bits(val)
    }
}
impl From<MODE> for u8 {
    #[inline(always)]
    fn from(val: MODE) -> u8 {
        MODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MSTSLVCFG {
    #[doc = "Normal slave mode, the default mode. SCK and WS are received from a master and used to transmit or receive data."]
    NORMAL_SLAVE_MODE = 0x0,
    #[doc = "WS synchronized master. WS is received from another master and used to synchronize the generation of SCK, when divided from the Flexcomm function clock."]
    WS_SYNC_MASTER = 0x01,
    #[doc = "Master using an existing SCK. SCK is received and used directly to generate WS, as well as transmitting or receiving data."]
    MASTER_USING_SCK = 0x02,
    #[doc = "Normal master mode. SCK and WS are generated so they can be sent to one or more slave devices."]
    NORMAL_MASTER = 0x03,
}
impl MSTSLVCFG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MSTSLVCFG {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MSTSLVCFG {
    #[inline(always)]
    fn from(val: u8) -> MSTSLVCFG {
        MSTSLVCFG::from_bits(val)
    }
}
impl From<MSTSLVCFG> for u8 {
    #[inline(always)]
    fn from(val: MSTSLVCFG) -> u8 {
        MSTSLVCFG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ONECHANNEL {
    #[doc = "I2S data for this channel pair is treated as left and right channels."]
    DUAL_CHANNEL = 0x0,
    #[doc = "I2S data for this channel pair is treated as a single channel, functionally the left channel for this pair. In mode 0 only, the right side of the frame begins at POSITION = 0x100. This is because mode 0 makes a clear distinction between the left and right sides of the frame. When ONECHANNEL = 1, the single channel of data may be placed on the right by setting POSITION to 0x100 + the data position within the right side (e.g. 0x108 would place data starting at the 8th clock after the middle of the frame). In other modes, data for the single channel of data is placed at the clock defined by POSITION."]
    SINGLE_CHANNEL = 0x01,
}
impl ONECHANNEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ONECHANNEL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ONECHANNEL {
    #[inline(always)]
    fn from(val: u8) -> ONECHANNEL {
        ONECHANNEL::from_bits(val)
    }
}
impl From<ONECHANNEL> for u8 {
    #[inline(always)]
    fn from(val: ONECHANNEL) -> u8 {
        ONECHANNEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PACK48 {
    #[doc = "48-bit I2S FIFO entries are handled as all 24-bit values."]
    BIT_24 = 0x0,
    #[doc = "48-bit I2S FIFO entries are handled as alternating 32-bit and 16-bit values."]
    BIT_32_16 = 0x01,
}
impl PACK48 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PACK48 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PACK48 {
    #[inline(always)]
    fn from(val: u8) -> PACK48 {
        PACK48::from_bits(val)
    }
}
impl From<PACK48> for u8 {
    #[inline(always)]
    fn from(val: PACK48) -> u8 {
        PACK48::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PAIRCOUNT {
    #[doc = "1 I2S channel pairs in this flexcomm."]
    PAIRS_1 = 0x0,
    #[doc = "2 I2S channel pairs in this flexcomm."]
    PAIRS_2 = 0x01,
    #[doc = "3 I2S channel pairs in this flexcomm."]
    PAIRS_3 = 0x02,
    #[doc = "4 I2S channel pairs in this flexcomm."]
    PAIRS_4 = 0x03,
}
impl PAIRCOUNT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PAIRCOUNT {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PAIRCOUNT {
    #[inline(always)]
    fn from(val: u8) -> PAIRCOUNT {
        PAIRCOUNT::from_bits(val)
    }
}
impl From<PAIRCOUNT> for u8 {
    #[inline(always)]
    fn from(val: PAIRCOUNT) -> u8 {
        PAIRCOUNT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RIGHTLOW {
    #[doc = "The right channel is taken from the high part of the FIFO data. For example, when data is 16 bits, FIFO bits 31:16 are used for the right channel."]
    RIGHT_HIGH = 0x0,
    #[doc = "The right channel is taken from the low part of the FIFO data. For example, when data is 16 bits, FIFO bits 15:0 are used for the right channel."]
    RIGHT_LOW = 0x01,
}
impl RIGHTLOW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RIGHTLOW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RIGHTLOW {
    #[inline(always)]
    fn from(val: u8) -> RIGHTLOW {
        RIGHTLOW::from_bits(val)
    }
}
impl From<RIGHTLOW> for u8 {
    #[inline(always)]
    fn from(val: RIGHTLOW) -> u8 {
        RIGHTLOW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SCK_POL {
    #[doc = "Data is launched on SCK falling edges and sampled on SCK rising edges (standard for I2S)."]
    FALLING_EDGE = 0x0,
    #[doc = "Data is launched on SCK rising edges and sampled on SCK falling edges."]
    RISING_EDGE = 0x01,
}
impl SCK_POL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SCK_POL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SCK_POL {
    #[inline(always)]
    fn from(val: u8) -> SCK_POL {
        SCK_POL::from_bits(val)
    }
}
impl From<SCK_POL> for u8 {
    #[inline(always)]
    fn from(val: SCK_POL) -> u8 {
        SCK_POL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TXI2SE0 {
    #[doc = "If the TX FIFO becomes empty, the last value is sent. This setting may be used when the data length is 24 bits or less, or when MONO = 1 for this channel pair."]
    LAST_VALUE = 0x0,
    #[doc = "If the TX FIFO becomes empty, 0 is sent. Use if the data length is greater than 24 bits or if zero fill is preferred."]
    ZERO = 0x01,
}
impl TXI2SE0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TXI2SE0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TXI2SE0 {
    #[inline(always)]
    fn from(val: u8) -> TXI2SE0 {
        TXI2SE0::from_bits(val)
    }
}
impl From<TXI2SE0> for u8 {
    #[inline(always)]
    fn from(val: TXI2SE0) -> u8 {
        TXI2SE0::to_bits(val)
    }
}
