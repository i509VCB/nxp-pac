#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Brk13 {
    #[doc = "Break character is transmitted with length of 9 to 13 bit times."]
    SHORT = 0x0,
    #[doc = "Break character is transmitted with length of 12 to 15 bit times."]
    LONG = 0x01,
}
impl Brk13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Brk13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Brk13 {
    #[inline(always)]
    fn from(val: u8) -> Brk13 {
        Brk13::from_bits(val)
    }
}
impl From<Brk13> for u8 {
    #[inline(always)]
    fn from(val: Brk13) -> u8 {
        Brk13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dozeen {
    #[doc = "LPUART is enabled in Doze mode."]
    ENABLED = 0x0,
    #[doc = "LPUART is disabled in Doze mode ."]
    DISABLED = 0x01,
}
impl Dozeen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dozeen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dozeen {
    #[inline(always)]
    fn from(val: u8) -> Dozeen {
        Dozeen::from_bits(val)
    }
}
impl From<Dozeen> for u8 {
    #[inline(always)]
    fn from(val: Dozeen) -> u8 {
        Dozeen::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Feature(u16);
impl Feature {
    #[doc = "Standard feature set."]
    pub const STANDARD: Self = Self(0x01);
    #[doc = "Standard feature set with MODEM/IrDA support."]
    pub const MODEM: Self = Self(0x03);
}
impl Feature {
    pub const fn from_bits(val: u16) -> Feature {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Feature {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("STANDARD"),
            0x03 => f.write_str("MODEM"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Feature {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "STANDARD"),
            0x03 => defmt::write!(f, "MODEM"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Feature {
    #[inline(always)]
    fn from(val: u16) -> Feature {
        Feature::from_bits(val)
    }
}
impl From<Feature> for u16 {
    #[inline(always)]
    fn from(val: Feature) -> u16 {
        Feature::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Idlecfg {
    #[doc = "1 idle character"]
    IDLE_1 = 0x0,
    #[doc = "2 idle characters"]
    IDLE_2 = 0x01,
    #[doc = "4 idle characters"]
    IDLE_4 = 0x02,
    #[doc = "8 idle characters"]
    IDLE_8 = 0x03,
    #[doc = "16 idle characters"]
    IDLE_16 = 0x04,
    #[doc = "32 idle characters"]
    IDLE_32 = 0x05,
    #[doc = "64 idle characters"]
    IDLE_64 = 0x06,
    #[doc = "128 idle characters"]
    IDLE_128 = 0x07,
}
impl Idlecfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Idlecfg {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Idlecfg {
    #[inline(always)]
    fn from(val: u8) -> Idlecfg {
        Idlecfg::from_bits(val)
    }
}
impl From<Idlecfg> for u8 {
    #[inline(always)]
    fn from(val: Idlecfg) -> u8 {
        Idlecfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ilt {
    #[doc = "Idle character bit count starts after start bit."]
    FROM_START = 0x0,
    #[doc = "Idle character bit count starts after stop bit."]
    FROM_STOP = 0x01,
}
impl Ilt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ilt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ilt {
    #[inline(always)]
    fn from(val: u8) -> Ilt {
        Ilt::from_bits(val)
    }
}
impl From<Ilt> for u8 {
    #[inline(always)]
    fn from(val: Ilt) -> u8 {
        Ilt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M {
    #[doc = "Receiver and transmitter use 8-bit data characters."]
    DATA8 = 0x0,
    #[doc = "Receiver and transmitter use 9-bit data characters."]
    DATA9 = 0x01,
}
impl M {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M {
    #[inline(always)]
    fn from(val: u8) -> M {
        M::from_bits(val)
    }
}
impl From<M> for u8 {
    #[inline(always)]
    fn from(val: M) -> u8 {
        M::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7 {
    #[doc = "Receiver and transmitter use 8-bit to 10-bit data characters."]
    NO_EFFECT = 0x0,
    #[doc = "Receiver and transmitter use 7-bit data characters."]
    DATA7 = 0x01,
}
impl M7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7 {
    #[inline(always)]
    fn from(val: u8) -> M7 {
        M7::from_bits(val)
    }
}
impl From<M7> for u8 {
    #[inline(always)]
    fn from(val: M7) -> u8 {
        M7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Matcfg {
    #[doc = "Address Match Wakeup"]
    ADDR_MATCH = 0x0,
    #[doc = "Idle Match Wakeup"]
    IDLE_MATCH = 0x01,
    #[doc = "Match On and Match Off"]
    ONOFF_MATCH = 0x02,
    #[doc = "Enables RWU on Data Match and Match On/Off for transmitter CTS input"]
    RWU_MATCH = 0x03,
}
impl Matcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Matcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Matcfg {
    #[inline(always)]
    fn from(val: u8) -> Matcfg {
        Matcfg::from_bits(val)
    }
}
impl From<Matcfg> for u8 {
    #[inline(always)]
    fn from(val: Matcfg) -> u8 {
        Matcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Msbf {
    #[doc = "LSB (bit0) is the first bit that is transmitted following the start bit. Further, the first bit received after the start bit is identified as bit0."]
    LSB_FIRST = 0x0,
    #[doc = "MSB (identified as bit9, bit8, bit7 or bit6) is the first bit that is transmitted following the start bit depending on the setting of CTRL\\[M\\], CTRL\\[PE\\] and BAUD\\[M10\\]. ."]
    MSB_FIRST = 0x01,
}
impl Msbf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Msbf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Msbf {
    #[inline(always)]
    fn from(val: u8) -> Msbf {
        Msbf::from_bits(val)
    }
}
impl From<Msbf> for u8 {
    #[inline(always)]
    fn from(val: Msbf) -> u8 {
        Msbf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Osr {
    #[doc = "Writing 0 to this field results in an oversampling ratio of 16"]
    DEFAULT = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Oversampling ratio of 4, requires BOTHEDGE to be set."]
    OSR_4 = 0x03,
    #[doc = "Oversampling ratio of 5, requires BOTHEDGE to be set."]
    OSR_5 = 0x04,
    #[doc = "Oversampling ratio of 6, requires BOTHEDGE to be set."]
    OSR_6 = 0x05,
    #[doc = "Oversampling ratio of 7, requires BOTHEDGE to be set."]
    OSR_7 = 0x06,
    #[doc = "Oversampling ratio of 8."]
    OSR_8 = 0x07,
    #[doc = "Oversampling ratio of 9."]
    OSR_9 = 0x08,
    #[doc = "Oversampling ratio of 10."]
    OSR_10 = 0x09,
    #[doc = "Oversampling ratio of 11."]
    OSR_11 = 0x0a,
    #[doc = "Oversampling ratio of 12."]
    OSR_12 = 0x0b,
    #[doc = "Oversampling ratio of 13."]
    OSR_13 = 0x0c,
    #[doc = "Oversampling ratio of 14."]
    OSR_14 = 0x0d,
    #[doc = "Oversampling ratio of 15."]
    OSR_15 = 0x0e,
    #[doc = "Oversampling ratio of 16."]
    OSR_16 = 0x0f,
    #[doc = "Oversampling ratio of 17."]
    OSR_17 = 0x10,
    #[doc = "Oversampling ratio of 18."]
    OSR_18 = 0x11,
    #[doc = "Oversampling ratio of 19."]
    OSR_19 = 0x12,
    #[doc = "Oversampling ratio of 20."]
    OSR_20 = 0x13,
    #[doc = "Oversampling ratio of 21."]
    OSR_21 = 0x14,
    #[doc = "Oversampling ratio of 22."]
    OSR_22 = 0x15,
    #[doc = "Oversampling ratio of 23."]
    OSR_23 = 0x16,
    #[doc = "Oversampling ratio of 24."]
    OSR_24 = 0x17,
    #[doc = "Oversampling ratio of 25."]
    OSR_25 = 0x18,
    #[doc = "Oversampling ratio of 26."]
    OSR_26 = 0x19,
    #[doc = "Oversampling ratio of 27."]
    OSR_27 = 0x1a,
    #[doc = "Oversampling ratio of 28."]
    OSR_28 = 0x1b,
    #[doc = "Oversampling ratio of 29."]
    OSR_29 = 0x1c,
    #[doc = "Oversampling ratio of 30."]
    OSR_30 = 0x1d,
    #[doc = "Oversampling ratio of 31."]
    OSR_31 = 0x1e,
    #[doc = "Oversampling ratio of 32."]
    OSR_32 = 0x1f,
}
impl Osr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Osr {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Osr {
    #[inline(always)]
    fn from(val: u8) -> Osr {
        Osr::from_bits(val)
    }
}
impl From<Osr> for u8 {
    #[inline(always)]
    fn from(val: Osr) -> u8 {
        Osr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pt {
    #[doc = "Even parity."]
    EVEN = 0x0,
    #[doc = "Odd parity."]
    ODD = 0x01,
}
impl Pt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pt {
    #[inline(always)]
    fn from(val: u8) -> Pt {
        Pt::from_bits(val)
    }
}
impl From<Pt> for u8 {
    #[inline(always)]
    fn from(val: Pt) -> u8 {
        Pt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Raf {
    #[doc = "LPUART receiver idle waiting for a start bit."]
    IDLE = 0x0,
    #[doc = "LPUART receiver active (RXD input not idle)."]
    ACTIVE = 0x01,
}
impl Raf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Raf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Raf {
    #[inline(always)]
    fn from(val: u8) -> Raf {
        Raf::from_bits(val)
    }
}
impl From<Raf> for u8 {
    #[inline(always)]
    fn from(val: Raf) -> u8 {
        Raf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Resyncdis {
    #[doc = "Resynchronization during received data word is supported."]
    RESYNC = 0x0,
    #[doc = "Resynchronization during received data word is disabled."]
    NO_RESYNC = 0x01,
}
impl Resyncdis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Resyncdis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Resyncdis {
    #[inline(always)]
    fn from(val: u8) -> Resyncdis {
        Resyncdis::from_bits(val)
    }
}
impl From<Resyncdis> for u8 {
    #[inline(always)]
    fn from(val: Resyncdis) -> u8 {
        Resyncdis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rsrc {
    #[doc = "Provided LOOPS is set, RSRC is cleared, selects internal loop back mode and the LPUART does not use the RXD pin."]
    NO_EFFECT = 0x0,
    #[doc = "Single-wire LPUART mode where the TXD pin is connected to the transmitter output and receiver input."]
    ONEWIRE = 0x01,
}
impl Rsrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rsrc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rsrc {
    #[inline(always)]
    fn from(val: u8) -> Rsrc {
        Rsrc::from_bits(val)
    }
}
impl From<Rsrc> for u8 {
    #[inline(always)]
    fn from(val: Rsrc) -> u8 {
        Rsrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rwu {
    #[doc = "Normal receiver operation."]
    NO_EFFECT = 0x0,
    #[doc = "LPUART receiver in standby waiting for wakeup condition."]
    RX_WAKEUP = 0x01,
}
impl Rwu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rwu {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rwu {
    #[inline(always)]
    fn from(val: u8) -> Rwu {
        Rwu::from_bits(val)
    }
}
impl From<Rwu> for u8 {
    #[inline(always)]
    fn from(val: Rwu) -> u8 {
        Rwu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rwuid {
    #[doc = "During receive standby state (RWU = 1), the IDLE bit does not get set upon detection of an idle character. During address match wakeup, the IDLE bit does not set when an address does not match."]
    IDLE_NOTSET = 0x0,
    #[doc = "During receive standby state (RWU = 1), the IDLE bit gets set upon detection of an idle character. During address match wakeup, the IDLE bit does set when an address does not match."]
    IDLE_SET = 0x01,
}
impl Rwuid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rwuid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rwuid {
    #[inline(always)]
    fn from(val: u8) -> Rwuid {
        Rwuid::from_bits(val)
    }
}
impl From<Rwuid> for u8 {
    #[inline(always)]
    fn from(val: Rwuid) -> u8 {
        Rwuid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxfifosize {
    #[doc = "Receive FIFO/Buffer depth = 1 dataword."]
    FIFO_1 = 0x0,
    #[doc = "Receive FIFO/Buffer depth = 4 datawords."]
    FIFO_4 = 0x01,
    #[doc = "Receive FIFO/Buffer depth = 8 datawords."]
    FIFO_8 = 0x02,
    #[doc = "Receive FIFO/Buffer depth = 16 datawords."]
    FIFO_16 = 0x03,
    #[doc = "Receive FIFO/Buffer depth = 32 datawords."]
    FIFO_32 = 0x04,
    #[doc = "Receive FIFO/Buffer depth = 64 datawords."]
    FIFO_64 = 0x05,
    #[doc = "Receive FIFO/Buffer depth = 128 datawords."]
    FIFO_128 = 0x06,
    #[doc = "Receive FIFO/Buffer depth = 256 datawords."]
    FIFO_256 = 0x07,
}
impl Rxfifosize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxfifosize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxfifosize {
    #[inline(always)]
    fn from(val: u8) -> Rxfifosize {
        Rxfifosize::from_bits(val)
    }
}
impl From<Rxfifosize> for u8 {
    #[inline(always)]
    fn from(val: Rxfifosize) -> u8 {
        Rxfifosize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxiden {
    #[doc = "Disable RDRF assertion due to partially filled FIFO when receiver is idle."]
    DISABLED = 0x0,
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 1 character."]
    IDLE_1 = 0x01,
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 2 characters."]
    IDLE_2 = 0x02,
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 4 characters."]
    IDLE_4 = 0x03,
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 8 characters."]
    IDLE_8 = 0x04,
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 16 characters."]
    IDLE_16 = 0x05,
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 32 characters."]
    IDLE_32 = 0x06,
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 64 characters."]
    IDLE_64 = 0x07,
}
impl Rxiden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxiden {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxiden {
    #[inline(always)]
    fn from(val: u8) -> Rxiden {
        Rxiden::from_bits(val)
    }
}
impl From<Rxiden> for u8 {
    #[inline(always)]
    fn from(val: Rxiden) -> u8 {
        Rxiden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sbns {
    #[doc = "One stop bit."]
    ONE = 0x0,
    #[doc = "Two stop bits."]
    TWO = 0x01,
}
impl Sbns {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sbns {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sbns {
    #[inline(always)]
    fn from(val: u8) -> Sbns {
        Sbns::from_bits(val)
    }
}
impl From<Sbns> for u8 {
    #[inline(always)]
    fn from(val: Sbns) -> u8 {
        Sbns::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tc {
    #[doc = "Transmitter active (sending data, a preamble, or a break)."]
    ACTIVE = 0x0,
    #[doc = "Transmitter idle (transmission activity complete)."]
    COMPLETE = 0x01,
}
impl Tc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tc {
    #[inline(always)]
    fn from(val: u8) -> Tc {
        Tc::from_bits(val)
    }
}
impl From<Tc> for u8 {
    #[inline(always)]
    fn from(val: Tc) -> u8 {
        Tc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tdre {
    #[doc = "Transmit FIFO level is greater than watermark."]
    TXDATA = 0x0,
    #[doc = "Transmit FIFO level is equal or less than watermark."]
    NO_TXDATA = 0x01,
}
impl Tdre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tdre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tdre {
    #[inline(always)]
    fn from(val: u8) -> Tdre {
        Tdre::from_bits(val)
    }
}
impl From<Tdre> for u8 {
    #[inline(always)]
    fn from(val: Tdre) -> u8 {
        Tdre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tnp {
    #[doc = "1/OSR."]
    ONE_SAMPLE = 0x0,
    #[doc = "2/OSR."]
    TWO_SAMPLE = 0x01,
    #[doc = "3/OSR."]
    THREE_SAMPLE = 0x02,
    #[doc = "4/OSR."]
    FOUR_SAMPLE = 0x03,
}
impl Tnp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tnp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tnp {
    #[inline(always)]
    fn from(val: u8) -> Tnp {
        Tnp::from_bits(val)
    }
}
impl From<Tnp> for u8 {
    #[inline(always)]
    fn from(val: Tnp) -> u8 {
        Tnp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trgsel {
    #[doc = "Input trigger is disabled."]
    DISABLED = 0x0,
    #[doc = "Input trigger is used instead of RXD pin input."]
    TRG_RXD = 0x01,
    #[doc = "Input trigger is used instead of CTS_B pin input."]
    TRG_CTS = 0x02,
    #[doc = "Input trigger is used to modulate the TXD pin output. The TXD pin output (after TXINV configuration) is internally ANDed with the input trigger."]
    TRG_TXD = 0x03,
}
impl Trgsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trgsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trgsel {
    #[inline(always)]
    fn from(val: u8) -> Trgsel {
        Trgsel::from_bits(val)
    }
}
impl From<Trgsel> for u8 {
    #[inline(always)]
    fn from(val: Trgsel) -> u8 {
        Trgsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txctsc {
    #[doc = "CTS input is sampled at the start of each character."]
    START = 0x0,
    #[doc = "CTS input is sampled when the transmitter is idle."]
    IDLE = 0x01,
}
impl Txctsc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txctsc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txctsc {
    #[inline(always)]
    fn from(val: u8) -> Txctsc {
        Txctsc::from_bits(val)
    }
}
impl From<Txctsc> for u8 {
    #[inline(always)]
    fn from(val: Txctsc) -> u8 {
        Txctsc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txctssrc {
    #[doc = "CTS input is the CTS_B pin."]
    CTS = 0x0,
    #[doc = "CTS input is an internal connection to the receiver address match result."]
    MATCH = 0x01,
}
impl Txctssrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txctssrc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txctssrc {
    #[inline(always)]
    fn from(val: u8) -> Txctssrc {
        Txctssrc::from_bits(val)
    }
}
impl From<Txctssrc> for u8 {
    #[inline(always)]
    fn from(val: Txctssrc) -> u8 {
        Txctssrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txdir {
    #[doc = "TXD pin is an input in single-wire mode."]
    TX_INPUT = 0x0,
    #[doc = "TXD pin is an output in single-wire mode."]
    TX_OUTPUT = 0x01,
}
impl Txdir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txdir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txdir {
    #[inline(always)]
    fn from(val: u8) -> Txdir {
        Txdir::from_bits(val)
    }
}
impl From<Txdir> for u8 {
    #[inline(always)]
    fn from(val: Txdir) -> u8 {
        Txdir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txfifosize {
    #[doc = "Transmit FIFO/Buffer depth = 1 dataword."]
    FIFO_1 = 0x0,
    #[doc = "Transmit FIFO/Buffer depth = 4 datawords."]
    FIFO_4 = 0x01,
    #[doc = "Transmit FIFO/Buffer depth = 8 datawords."]
    FIFO_8 = 0x02,
    #[doc = "Transmit FIFO/Buffer depth = 16 datawords."]
    FIFO_16 = 0x03,
    #[doc = "Transmit FIFO/Buffer depth = 32 datawords."]
    FIFO_32 = 0x04,
    #[doc = "Transmit FIFO/Buffer depth = 64 datawords."]
    FIFO_64 = 0x05,
    #[doc = "Transmit FIFO/Buffer depth = 128 datawords."]
    FIFO_128 = 0x06,
    #[doc = "Transmit FIFO/Buffer depth = 256 datawords"]
    FIFO_256 = 0x07,
}
impl Txfifosize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txfifosize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txfifosize {
    #[inline(always)]
    fn from(val: u8) -> Txfifosize {
        Txfifosize::from_bits(val)
    }
}
impl From<Txfifosize> for u8 {
    #[inline(always)]
    fn from(val: Txfifosize) -> u8 {
        Txfifosize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txflush {
    #[doc = "No flush operation occurs."]
    NO_EFFECT = 0x0,
    #[doc = "All data in the transmit FIFO is cleared out."]
    TXFIFO_RST = 0x01,
}
impl Txflush {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txflush {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txflush {
    #[inline(always)]
    fn from(val: u8) -> Txflush {
        Txflush::from_bits(val)
    }
}
impl From<Txflush> for u8 {
    #[inline(always)]
    fn from(val: Txflush) -> u8 {
        Txflush::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txrtspol {
    #[doc = "Transmitter RTS is active low."]
    LOW = 0x0,
    #[doc = "Transmitter RTS is active high."]
    HIGH = 0x01,
}
impl Txrtspol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txrtspol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txrtspol {
    #[inline(always)]
    fn from(val: u8) -> Txrtspol {
        Txrtspol::from_bits(val)
    }
}
impl From<Txrtspol> for u8 {
    #[inline(always)]
    fn from(val: Txrtspol) -> u8 {
        Txrtspol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wake {
    #[doc = "Configures RWU for idle-line wakeup."]
    IDLE = 0x0,
    #[doc = "Configures RWU with address-mark wakeup."]
    MARK = 0x01,
}
impl Wake {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wake {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wake {
    #[inline(always)]
    fn from(val: u8) -> Wake {
        Wake::from_bits(val)
    }
}
impl From<Wake> for u8 {
    #[inline(always)]
    fn from(val: Wake) -> u8 {
        Wake::to_bits(val)
    }
}
