#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Brk13 {
    #[doc = "Break character is transmitted with length of 9 to 13 bit times."]
    Short = 0x0,
    #[doc = "Break character is transmitted with length of 12 to 15 bit times."]
    Long = 0x01,
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
    Enabled = 0x0,
    #[doc = "LPUART is disabled in Doze mode."]
    Disabled = 0x01,
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
    pub const Standard: Self = Self(0x01);
    #[doc = "Standard feature set with MODEM/IrDA support."]
    pub const Modem: Self = Self(0x03);
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
            0x01 => f.write_str("Standard"),
            0x03 => f.write_str("Modem"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Feature {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "Standard"),
            0x03 => defmt::write!(f, "Modem"),
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
    #[doc = "1 idle character."]
    Idle1 = 0x0,
    #[doc = "2 idle characters."]
    Idle2 = 0x01,
    #[doc = "4 idle characters."]
    Idle4 = 0x02,
    #[doc = "8 idle characters."]
    Idle8 = 0x03,
    #[doc = "16 idle characters."]
    Idle16 = 0x04,
    #[doc = "32 idle characters."]
    Idle32 = 0x05,
    #[doc = "64 idle characters."]
    Idle64 = 0x06,
    #[doc = "128 idle characters."]
    Idle128 = 0x07,
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
    FromStart = 0x0,
    #[doc = "Idle character bit count starts after stop bit."]
    FromStop = 0x01,
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
    Data8 = 0x0,
    #[doc = "Receiver and transmitter use 9-bit data characters."]
    Data9 = 0x01,
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
    NoEffect = 0x0,
    #[doc = "Receiver and transmitter use 7-bit data characters."]
    Data7 = 0x01,
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
    #[doc = "Address Match Wakeup."]
    AddrMatch = 0x0,
    #[doc = "Idle Match Wakeup."]
    IdleMatch = 0x01,
    #[doc = "Match On and Match Off."]
    OnoffMatch = 0x02,
    #[doc = "Enables RWU on Data Match and Match On/Off for transmitter CTS input."]
    RwuMatch = 0x03,
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
    LsbFirst = 0x0,
    #[doc = "MSB (identified as bit9, bit8, bit7 or bit6) is the first bit that is transmitted following the start bit depending on the setting of CTRL\\[M\\], CTRL\\[PE\\] and BAUD\\[M10\\].."]
    MsbFirst = 0x01,
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
    #[doc = "Writing 0 to this field results in an oversampling ratio of 16."]
    Default = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Oversampling ratio of 4, requires BOTHEDGE to be set."]
    Osr4 = 0x03,
    #[doc = "Oversampling ratio of 5, requires BOTHEDGE to be set."]
    Osr5 = 0x04,
    #[doc = "Oversampling ratio of 6, requires BOTHEDGE to be set."]
    Osr6 = 0x05,
    #[doc = "Oversampling ratio of 7, requires BOTHEDGE to be set."]
    Osr7 = 0x06,
    #[doc = "Oversampling ratio of 8."]
    Osr8 = 0x07,
    #[doc = "Oversampling ratio of 9."]
    Osr9 = 0x08,
    #[doc = "Oversampling ratio of 10."]
    Osr10 = 0x09,
    #[doc = "Oversampling ratio of 11."]
    Osr11 = 0x0a,
    #[doc = "Oversampling ratio of 12."]
    Osr12 = 0x0b,
    #[doc = "Oversampling ratio of 13."]
    Osr13 = 0x0c,
    #[doc = "Oversampling ratio of 14."]
    Osr14 = 0x0d,
    #[doc = "Oversampling ratio of 15."]
    Osr15 = 0x0e,
    #[doc = "Oversampling ratio of 16."]
    Osr16 = 0x0f,
    #[doc = "Oversampling ratio of 17."]
    Osr17 = 0x10,
    #[doc = "Oversampling ratio of 18."]
    Osr18 = 0x11,
    #[doc = "Oversampling ratio of 19."]
    Osr19 = 0x12,
    #[doc = "Oversampling ratio of 20."]
    Osr20 = 0x13,
    #[doc = "Oversampling ratio of 21."]
    Osr21 = 0x14,
    #[doc = "Oversampling ratio of 22."]
    Osr22 = 0x15,
    #[doc = "Oversampling ratio of 23."]
    Osr23 = 0x16,
    #[doc = "Oversampling ratio of 24."]
    Osr24 = 0x17,
    #[doc = "Oversampling ratio of 25."]
    Osr25 = 0x18,
    #[doc = "Oversampling ratio of 26."]
    Osr26 = 0x19,
    #[doc = "Oversampling ratio of 27."]
    Osr27 = 0x1a,
    #[doc = "Oversampling ratio of 28."]
    Osr28 = 0x1b,
    #[doc = "Oversampling ratio of 29."]
    Osr29 = 0x1c,
    #[doc = "Oversampling ratio of 30."]
    Osr30 = 0x1d,
    #[doc = "Oversampling ratio of 31."]
    Osr31 = 0x1e,
    #[doc = "Oversampling ratio of 32."]
    Osr32 = 0x1f,
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
    Even = 0x0,
    #[doc = "Odd parity."]
    Odd = 0x01,
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
    Idle = 0x0,
    #[doc = "LPUART receiver active (RXD input not idle)."]
    Active = 0x01,
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
    Resync = 0x0,
    #[doc = "Resynchronization during received data word is disabled."]
    NoResync = 0x01,
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
    NoEffect = 0x0,
    #[doc = "Single-wire LPUART mode where the TXD pin is connected to the transmitter output and receiver input."]
    Onewire = 0x01,
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
    NoEffect = 0x0,
    #[doc = "LPUART receiver in standby waiting for wakeup condition."]
    RxWakeup = 0x01,
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
    IdleNotset = 0x0,
    #[doc = "During receive standby state (RWU = 1), the IDLE bit gets set upon detection of an idle character. During address match wakeup, the IDLE bit does set when an address does not match."]
    IdleSet = 0x01,
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
    Fifo1 = 0x0,
    #[doc = "Receive FIFO/Buffer depth = 4 datawords."]
    Fifo4 = 0x01,
    #[doc = "Receive FIFO/Buffer depth = 8 datawords."]
    Fifo8 = 0x02,
    #[doc = "Receive FIFO/Buffer depth = 16 datawords."]
    Fifo16 = 0x03,
    #[doc = "Receive FIFO/Buffer depth = 32 datawords."]
    Fifo32 = 0x04,
    #[doc = "Receive FIFO/Buffer depth = 64 datawords."]
    Fifo64 = 0x05,
    #[doc = "Receive FIFO/Buffer depth = 128 datawords."]
    Fifo128 = 0x06,
    #[doc = "Receive FIFO/Buffer depth = 256 datawords."]
    Fifo256 = 0x07,
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
    Disabled = 0x0,
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 1 character."]
    Idle1 = 0x01,
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 2 characters."]
    Idle2 = 0x02,
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 4 characters."]
    Idle4 = 0x03,
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 8 characters."]
    Idle8 = 0x04,
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 16 characters."]
    Idle16 = 0x05,
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 32 characters."]
    Idle32 = 0x06,
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 64 characters."]
    Idle64 = 0x07,
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
    One = 0x0,
    #[doc = "Two stop bits."]
    Two = 0x01,
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
    Active = 0x0,
    #[doc = "Transmitter idle (transmission activity complete)."]
    Complete = 0x01,
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
    Txdata = 0x0,
    #[doc = "Transmit FIFO level is equal or less than watermark."]
    NoTxdata = 0x01,
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
    OneSample = 0x0,
    #[doc = "2/OSR."]
    TwoSample = 0x01,
    #[doc = "3/OSR."]
    ThreeSample = 0x02,
    #[doc = "4/OSR."]
    FourSample = 0x03,
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
    Disabled = 0x0,
    #[doc = "Input trigger is used instead of RXD pin input."]
    TrgRxd = 0x01,
    #[doc = "Input trigger is used instead of CTS_B pin input."]
    TrgCts = 0x02,
    #[doc = "Input trigger is used to modulate the TXD pin output. The TXD pin output (after TXINV configuration) is internally ANDed with the input trigger."]
    TrgTxd = 0x03,
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
    Start = 0x0,
    #[doc = "CTS input is sampled when the transmitter is idle."]
    Idle = 0x01,
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
    Cts = 0x0,
    #[doc = "CTS input is an internal connection to the receiver address match result."]
    Match = 0x01,
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
    TxInput = 0x0,
    #[doc = "TXD pin is an output in single-wire mode."]
    TxOutput = 0x01,
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
    Fifo1 = 0x0,
    #[doc = "Transmit FIFO/Buffer depth = 4 datawords."]
    Fifo4 = 0x01,
    #[doc = "Transmit FIFO/Buffer depth = 8 datawords."]
    Fifo8 = 0x02,
    #[doc = "Transmit FIFO/Buffer depth = 16 datawords."]
    Fifo16 = 0x03,
    #[doc = "Transmit FIFO/Buffer depth = 32 datawords."]
    Fifo32 = 0x04,
    #[doc = "Transmit FIFO/Buffer depth = 64 datawords."]
    Fifo64 = 0x05,
    #[doc = "Transmit FIFO/Buffer depth = 128 datawords."]
    Fifo128 = 0x06,
    #[doc = "Transmit FIFO/Buffer depth = 256 datawords."]
    Fifo256 = 0x07,
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
    NoEffect = 0x0,
    #[doc = "All data in the transmit FIFO is cleared out."]
    TxfifoRst = 0x01,
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
    Low = 0x0,
    #[doc = "Transmitter RTS is active high."]
    High = 0x01,
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
    Idle = 0x0,
    #[doc = "Configures RWU with address-mark wakeup."]
    Mark = 0x01,
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
