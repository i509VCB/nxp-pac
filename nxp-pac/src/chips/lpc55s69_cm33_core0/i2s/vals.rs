#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Busy {
    #[doc = "The transmitter/receiver for channel pair is currently idle."]
    IDLE = 0x0,
    #[doc = "The transmitter/receiver for channel pair is currently processing data."]
    BUSY = 0x01,
}
impl Busy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Busy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Busy {
    #[inline(always)]
    fn from(val: u8) -> Busy {
        Busy::from_bits(val)
    }
}
impl From<Busy> for u8 {
    #[inline(always)]
    fn from(val: Busy) -> u8 {
        Busy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Datapause {
    #[doc = "Normal operation, or resuming normal operation at the next frame if the I2S has already been paused."]
    NORMAL = 0x0,
    #[doc = "A pause in the data flow is being requested. It is in effect when DATAPAUSED in STAT = 1."]
    PAUSE = 0x01,
}
impl Datapause {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Datapause {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Datapause {
    #[inline(always)]
    fn from(val: u8) -> Datapause {
        Datapause::from_bits(val)
    }
}
impl From<Datapause> for u8 {
    #[inline(always)]
    fn from(val: Datapause) -> u8 {
        Datapause::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Leftjust {
    #[doc = "Data is transferred between the FIFO and the I2S serializer/deserializer right justified, i.e. starting from bit 0 and continuing to the position defined by DATALEN. This would correspond to right justified data in the stream on the data bus."]
    RIGHT_JUSTIFIED = 0x0,
    #[doc = "Data is transferred between the FIFO and the I2S serializer/deserializer left justified, i.e. starting from the MSB of the FIFO entry and continuing for the number of bits defined by DATALEN. This would correspond to left justified data in the stream on the data bus."]
    LEFT_JUSTIFIED = 0x01,
}
impl Leftjust {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Leftjust {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Leftjust {
    #[inline(always)]
    fn from(val: u8) -> Leftjust {
        Leftjust::from_bits(val)
    }
}
impl From<Leftjust> for u8 {
    #[inline(always)]
    fn from(val: Leftjust) -> u8 {
        Leftjust::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lr {
    #[doc = "Left channel."]
    LEFT_CHANNEL = 0x0,
    #[doc = "Right channel."]
    RIGHT_CHANNEL = 0x01,
}
impl Lr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lr {
    #[inline(always)]
    fn from(val: u8) -> Lr {
        Lr::from_bits(val)
    }
}
impl From<Lr> for u8 {
    #[inline(always)]
    fn from(val: Lr) -> u8 {
        Lr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mode {
    #[doc = "I2S mode a.k.a. 'classic' mode. WS has a 50% duty cycle, with (for each enabled channel pair) one piece of left channel data occurring during the first phase, and one pieces of right channel data occurring during the second phase. In this mode, the data region begins one clock after the leading WS edge for the frame. For a 50% WS duty cycle, FRAMELEN must define an even number of I2S clocks for the frame. If FRAMELEN defines an odd number of clocks per frame, the extra clock will occur on the right."]
    CLASSIC_MODE = 0x0,
    #[doc = "DSP mode where WS has a 50% duty cycle. See remark for mode 0."]
    DSP_MODE_WS_50_DUTYCYCLE = 0x01,
    #[doc = "DSP mode where WS has a one clock long pulse at the beginning of each data frame."]
    DSP_MODE_WS_1_CLOCK = 0x02,
    #[doc = "DSP mode where WS has a one data slot long pulse at the beginning of each data frame."]
    DSP_MODE_WS_1_DATA = 0x03,
}
impl Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mode {
    #[inline(always)]
    fn from(val: u8) -> Mode {
        Mode::from_bits(val)
    }
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(val: Mode) -> u8 {
        Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstslvcfg {
    #[doc = "Normal slave mode, the default mode. SCK and WS are received from a master and used to transmit or receive data."]
    NORMAL_SLAVE_MODE = 0x0,
    #[doc = "WS synchronized master. WS is received from another master and used to synchronize the generation of SCK, when divided from the Flexcomm function clock."]
    WS_SYNC_MASTER = 0x01,
    #[doc = "Master using an existing SCK. SCK is received and used directly to generate WS, as well as transmitting or receiving data."]
    MASTER_USING_SCK = 0x02,
    #[doc = "Normal master mode. SCK and WS are generated so they can be sent to one or more slave devices."]
    NORMAL_MASTER = 0x03,
}
impl Mstslvcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mstslvcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mstslvcfg {
    #[inline(always)]
    fn from(val: u8) -> Mstslvcfg {
        Mstslvcfg::from_bits(val)
    }
}
impl From<Mstslvcfg> for u8 {
    #[inline(always)]
    fn from(val: Mstslvcfg) -> u8 {
        Mstslvcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Onechannel {
    #[doc = "I2S data for this channel pair is treated as left and right channels."]
    DUAL_CHANNEL = 0x0,
    #[doc = "I2S data for this channel pair is treated as a single channel, functionally the left channel for this pair. In mode 0 only, the right side of the frame begins at POSITION = 0x100. This is because mode 0 makes a clear distinction between the left and right sides of the frame. When ONECHANNEL = 1, the single channel of data may be placed on the right by setting POSITION to 0x100 + the data position within the right side (e.g. 0x108 would place data starting at the 8th clock after the middle of the frame). In other modes, data for the single channel of data is placed at the clock defined by POSITION."]
    SINGLE_CHANNEL = 0x01,
}
impl Onechannel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Onechannel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Onechannel {
    #[inline(always)]
    fn from(val: u8) -> Onechannel {
        Onechannel::from_bits(val)
    }
}
impl From<Onechannel> for u8 {
    #[inline(always)]
    fn from(val: Onechannel) -> u8 {
        Onechannel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pack48 {
    #[doc = "48-bit I2S FIFO entries are handled as all 24-bit values."]
    BIT_24 = 0x0,
    #[doc = "48-bit I2S FIFO entries are handled as alternating 32-bit and 16-bit values."]
    BIT_32_16 = 0x01,
}
impl Pack48 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pack48 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pack48 {
    #[inline(always)]
    fn from(val: u8) -> Pack48 {
        Pack48::from_bits(val)
    }
}
impl From<Pack48> for u8 {
    #[inline(always)]
    fn from(val: Pack48) -> u8 {
        Pack48::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Paircount {
    #[doc = "1 I2S channel pairs in this flexcomm"]
    PAIRS_1 = 0x0,
    #[doc = "2 I2S channel pairs in this flexcomm"]
    PAIRS_2 = 0x01,
    #[doc = "3 I2S channel pairs in this flexcomm"]
    PAIRS_3 = 0x02,
    #[doc = "4 I2S channel pairs in this flexcomm"]
    PAIRS_4 = 0x03,
}
impl Paircount {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Paircount {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Paircount {
    #[inline(always)]
    fn from(val: u8) -> Paircount {
        Paircount::from_bits(val)
    }
}
impl From<Paircount> for u8 {
    #[inline(always)]
    fn from(val: Paircount) -> u8 {
        Paircount::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rightlow {
    #[doc = "The right channel is taken from the high part of the FIFO data. For example, when data is 16 bits, FIFO bits 31:16 are used for the right channel."]
    RIGHT_HIGH = 0x0,
    #[doc = "The right channel is taken from the low part of the FIFO data. For example, when data is 16 bits, FIFO bits 15:0 are used for the right channel."]
    RIGHT_LOW = 0x01,
}
impl Rightlow {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rightlow {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rightlow {
    #[inline(always)]
    fn from(val: u8) -> Rightlow {
        Rightlow::from_bits(val)
    }
}
impl From<Rightlow> for u8 {
    #[inline(always)]
    fn from(val: Rightlow) -> u8 {
        Rightlow::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SckPol {
    #[doc = "Data is launched on SCK falling edges and sampled on SCK rising edges (standard for I2S)."]
    FALLING_EDGE = 0x0,
    #[doc = "Data is launched on SCK rising edges and sampled on SCK falling edges."]
    RISING_EDGE = 0x01,
}
impl SckPol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SckPol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SckPol {
    #[inline(always)]
    fn from(val: u8) -> SckPol {
        SckPol::from_bits(val)
    }
}
impl From<SckPol> for u8 {
    #[inline(always)]
    fn from(val: SckPol) -> u8 {
        SckPol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txi2se0 {
    #[doc = "If the TX FIFO becomes empty, the last value is sent. This setting may be used when the data length is 24 bits or less, or when MONO = 1 for this channel pair."]
    LAST_VALUE = 0x0,
    #[doc = "If the TX FIFO becomes empty, 0 is sent. Use if the data length is greater than 24 bits or if zero fill is preferred."]
    ZERO = 0x01,
}
impl Txi2se0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txi2se0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txi2se0 {
    #[inline(always)]
    fn from(val: u8) -> Txi2se0 {
        Txi2se0::from_bits(val)
    }
}
impl From<Txi2se0> for u8 {
    #[inline(always)]
    fn from(val: Txi2se0) -> u8 {
        Txi2se0::to_bits(val)
    }
}
