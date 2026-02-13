#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Actstate {
    #[doc = "NO_LATENCY: normal bus operations"]
    NO_LATENCY = 0x0,
    #[doc = "LATENCY_1MS: 1 ms of latency"]
    LATENCY_1MS = 0x01,
    #[doc = "LATENCY_100MS: 100 ms of latency"]
    LATENCY_100MS = 0x02,
    #[doc = "LATENCY_10S: 10 seconds of latency"]
    LATENCY_10S = 0x03,
}
impl Actstate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Actstate {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Actstate {
    #[inline(always)]
    fn from(val: u8) -> Actstate {
        Actstate::from_bits(val)
    }
}
impl From<Actstate> for u8 {
    #[inline(always)]
    fn from(val: Actstate) -> u8 {
        Actstate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Davalid {
    #[doc = "DANOTASSIGNED: a Dynamic Address is not assigned"]
    DANOTASSIGNED = 0x0,
    #[doc = "DAASSIGNED: a Dynamic Address is assigned"]
    DAASSIGNED = 0x01,
}
impl Davalid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Davalid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Davalid {
    #[inline(always)]
    fn from(val: u8) -> Davalid {
        Davalid::from_bits(val)
    }
}
impl From<Davalid> for u8 {
    #[inline(always)]
    fn from(val: Davalid) -> u8 {
        Davalid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma {
    #[doc = "DMA is not supported"]
    DMANO = 0x0,
    #[doc = "DMA is supported"]
    DMAYES = 0x01,
}
impl Dma {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma {
    #[inline(always)]
    fn from(val: u8) -> Dma {
        Dma::from_bits(val)
    }
}
impl From<Dma> for u8 {
    #[inline(always)]
    fn from(val: Dma) -> u8 {
        Dma::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Evdet {
    #[doc = "NONE: no event or no pending event"]
    NONE = 0x0,
    #[doc = "NO_REQUEST: Request not sent yet. Either there was no START yet, or is waiting for Bus-Available or Bus-Idle (HJ)."]
    NO_REQUEST = 0x01,
    #[doc = "NACKED: Not acknowledged(Request sent and NACKed); the module will try again."]
    NACKED = 0x02,
    #[doc = "ACKED: Acknowledged (Request sent and ACKed), so Done (unless the time control data is still being sent)."]
    ACKED = 0x03,
}
impl Evdet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Evdet {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Evdet {
    #[inline(always)]
    fn from(val: u8) -> Evdet {
        Evdet::from_bits(val)
    }
}
impl From<Evdet> for u8 {
    #[inline(always)]
    fn from(val: Evdet) -> u8 {
        Evdet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Event {
    #[doc = "NORMAL_MODE: If EVENT is set to 0 after was a non-0 value, event processing will cancel if the event processing has not yet started; if event processing has already been started, then event processing will not be be cancelled."]
    NORMAL_MODE = 0x0,
    #[doc = "IBI: Start an In-Band Interrupt. This will try to push an IBI interrupt onto the I3C bus. If data is associated with the IBI, then the data will be read from the SCTRL.IBIDATA field. If time control is enabled, then this data will also include any time control-related bytes; additionally, the IBIDATA byte will have bit 7 set to 1 automatically (as is required for time control). The IBI interrupt will occur after the 1st (mandatory) IBIDATA, if any."]
    IBI = 0x01,
    #[doc = "MASTER_REQUEST: Start a Master-Request."]
    MASTER_REQUEST = 0x02,
    #[doc = "HOT_JOIN_REQUEST: Start a Hot-Join request. A Hot-Join Request should only be used when the device is powered on after the I3C bus is already powered up, or when the device is connected using hot insertion methods (the device is powered up when it is physically inserted onto the powered-up I3C bus). The hot join will wait for Bus Idle, and SCTRL.EVENT=HOT_JOIN_REQUEST must be set before the slave enable (SCONFIG.SLVENA)."]
    HOT_JOIN_REQUEST = 0x03,
}
impl Event {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Event {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Event {
    #[inline(always)]
    fn from(val: u8) -> Event {
        Event::from_bits(val)
    }
}
impl From<Event> for u8 {
    #[inline(always)]
    fn from(val: Event) -> u8 {
        Event::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Extfifo {
    _RESERVED_0 = 0x0,
    #[doc = "STD_EXT_FIFO: standard available/free external FIFO"]
    STD_EXT_FIFO = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Extfifo {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extfifo {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extfifo {
    #[inline(always)]
    fn from(val: u8) -> Extfifo {
        Extfifo::from_bits(val)
    }
}
impl From<Extfifo> for u8 {
    #[inline(always)]
    fn from(val: Extfifo) -> u8 {
        Extfifo::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fiforx {
    #[doc = "FIFO_2BYTE: 2 (or 3)-byte RX FIFO, the default FIFO receive value (FIFORX)"]
    FIFO_2BYTE = 0x0,
    #[doc = "FIFO_4BYTE: 4-byte RX FIFO"]
    FIFO_4BYTE = 0x01,
    #[doc = "FIFO_8BYTE: 8-byte RX FIFO"]
    FIFO_8BYTE = 0x02,
    #[doc = "FIFO_16BYTE: 16-byte RX FIFO"]
    FIFO_16BYTE = 0x03,
}
impl Fiforx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fiforx {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fiforx {
    #[inline(always)]
    fn from(val: u8) -> Fiforx {
        Fiforx::from_bits(val)
    }
}
impl From<Fiforx> for u8 {
    #[inline(always)]
    fn from(val: Fiforx) -> u8 {
        Fiforx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fifotx {
    #[doc = "FIFO_2BYTE: 2-byte TX FIFO, the default FIFO transmit value (FIFOTX)"]
    FIFO_2BYTE = 0x0,
    #[doc = "FIFO_4BYTE: 4-byte TX FIFO"]
    FIFO_4BYTE = 0x01,
    #[doc = "FIFO_8BYTE: 8-byte TX FIFO"]
    FIFO_8BYTE = 0x02,
    #[doc = "FIFO_16BYTE: 16-byte TX FIFO"]
    FIFO_16BYTE = 0x03,
}
impl Fifotx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fifotx {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fifotx {
    #[inline(always)]
    fn from(val: u8) -> Fifotx {
        Fifotx::from_bits(val)
    }
}
impl From<Fifotx> for u8 {
    #[inline(always)]
    fn from(val: Fifotx) -> u8 {
        Fifotx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hkeep {
    #[doc = "NONE: Use PUR (Pull-Up Resistor). Hold SCL High."]
    NONE = 0x0,
    #[doc = "WIRED_IN: Wired-in High Keeper controls; use pin_HK (High Keeper) controls."]
    WIRED_IN = 0x01,
    #[doc = "PASSIVE_SDA: Passive on SDA; can Hi-Z (high impedance) for Bus Free (IDLE) and hold."]
    PASSIVE_SDA = 0x02,
    #[doc = "PASSIVE_ON_SDA_SCL: Passive on SDA and SCL; can Hi-Z (high impedance) both for Bus Free (IDLE), and can Hi-Z SDA for hold."]
    PASSIVE_ON_SDA_SCL = 0x03,
}
impl Hkeep {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hkeep {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hkeep {
    #[inline(always)]
    fn from(val: u8) -> Hkeep {
        Hkeep::from_bits(val)
    }
}
impl From<Hkeep> for u8 {
    #[inline(always)]
    fn from(val: Hkeep) -> u8 {
        Hkeep::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I2c {
    #[doc = "I3C message"]
    I3CMESSAGE = 0x0,
    #[doc = "I2C message"]
    I2CMESSAGE = 0x01,
}
impl I2c {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I2c {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I2c {
    #[inline(always)]
    fn from(val: u8) -> I2c {
        I2c::from_bits(val)
    }
}
impl From<I2c> for u8 {
    #[inline(always)]
    fn from(val: I2c) -> u8 {
        I2c::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ibiresp {
    #[doc = "ACK: Acknowledge. A mandatory byte (or not) is decided by the Master In-band Interrupt Registry and Rules Register (MIBIRULES). To limit the maximum number of IBI bytes, configure the Read Termination field (MCTRL.RDTERM)."]
    ACK = 0x0,
    #[doc = "NACK: Not acknowledge"]
    NACK = 0x01,
    #[doc = "ACK_WITH_MANDATORY: Acknowledge with mandatory byte (ignores the MIBIRULES register). Acknowledge with mandatory byte should not be used, unless only slaves with a mandatory byte can cause an In-Band Interrupt."]
    ACK_WITH_MANDATORY = 0x02,
    #[doc = "MANUAL: stop and wait for a decision using the IBIAckNack request"]
    MANUAL = 0x03,
}
impl Ibiresp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ibiresp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ibiresp {
    #[inline(always)]
    fn from(val: u8) -> Ibiresp {
        Ibiresp::from_bits(val)
    }
}
impl From<Ibiresp> for u8 {
    #[inline(always)]
    fn from(val: Ibiresp) -> u8 {
        Ibiresp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ibitype {
    #[doc = "NONE: cleared when IBI Won bit (MSTATUS.IBIWON) is cleared"]
    NONE = 0x0,
    #[doc = "IBI: In-Band Interrupt"]
    IBI = 0x01,
    #[doc = "MR: Master Request"]
    MR = 0x02,
    #[doc = "HJ: Hot-Join"]
    HJ = 0x03,
}
impl Ibitype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ibitype {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ibitype {
    #[inline(always)]
    fn from(val: u8) -> Ibitype {
        Ibitype::from_bits(val)
    }
}
impl From<Ibitype> for u8 {
    #[inline(always)]
    fn from(val: Ibitype) -> u8 {
        Ibitype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Idena {
    #[doc = "APPLICATION: Application handles ID 48b"]
    APPLICATION = 0x0,
    #[doc = "HW: Hardware handles ID 48b"]
    HW = 0x01,
    #[doc = "HW_BUT: in hardware but the I3C module instance handles ID 48b."]
    HW_BUT = 0x02,
    #[doc = "PARTNO: a part number register (PARTNO) handles ID 48b"]
    PARTNO = 0x03,
}
impl Idena {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Idena {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Idena {
    #[inline(always)]
    fn from(val: u8) -> Idena {
        Idena::from_bits(val)
    }
}
impl From<Idena> for u8 {
    #[inline(always)]
    fn from(val: Idena) -> u8 {
        Idena::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Int {
    #[doc = "Interrupts are not supported"]
    INTERRUPTSNO = 0x0,
    #[doc = "Interrupts are supported"]
    INTERRUPTSYES = 0x01,
}
impl Int {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Int {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Int {
    #[inline(always)]
    fn from(val: u8) -> Int {
        Int::from_bits(val)
    }
}
impl From<Int> for u8 {
    #[inline(always)]
    fn from(val: Int) -> u8 {
        Int::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Master {
    #[doc = "MASTERNOTSUPPORTED: master capability is not supported."]
    MASTERNOTSUPPORTED = 0x0,
    #[doc = "MASTERSUPPORTED: master capability is supported."]
    MASTERSUPPORTED = 0x01,
}
impl Master {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Master {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Master {
    #[inline(always)]
    fn from(val: u8) -> Master {
        Master::from_bits(val)
    }
}
impl From<Master> for u8 {
    #[inline(always)]
    fn from(val: Master) -> u8 {
        Master::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MctrlDir {
    #[doc = "DIRWRITE: Write"]
    DIRWRITE = 0x0,
    #[doc = "DIRREAD: Read"]
    DIRREAD = 0x01,
}
impl MctrlDir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MctrlDir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MctrlDir {
    #[inline(always)]
    fn from(val: u8) -> MctrlDir {
        MctrlDir::from_bits(val)
    }
}
impl From<MctrlDir> for u8 {
    #[inline(always)]
    fn from(val: MctrlDir) -> u8 {
        MctrlDir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MdmactrlDmafb {
    #[doc = "NOT_USED: DMA is not used"]
    NOT_USED = 0x0,
    #[doc = "ENABLE_ONE_FRAME: DMA is enabled for 1 frame. DMAFB auto-clears on STOP or repeated START. See MCONFIG.MATCHSS."]
    ENABLE_ONE_FRAME = 0x01,
    #[doc = "ENABLE: DMA is enabled until the DMA is turned off."]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl MdmactrlDmafb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MdmactrlDmafb {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MdmactrlDmafb {
    #[inline(always)]
    fn from(val: u8) -> MdmactrlDmafb {
        MdmactrlDmafb::from_bits(val)
    }
}
impl From<MdmactrlDmafb> for u8 {
    #[inline(always)]
    fn from(val: MdmactrlDmafb) -> u8 {
        MdmactrlDmafb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MdmactrlDmatb {
    #[doc = "NOT_USED: DMA is not used"]
    NOT_USED = 0x0,
    #[doc = "ENABLE_ONE_FRAME: DMA is enabled for 1 frame (ended by DMA or Terminated). DMATB auto-clears on STOP or START. See MCONFIG.MATCHSS."]
    ENABLE_ONE_FRAME = 0x01,
    #[doc = "ENABLE: DMA is enabled until DMA is turned off. Normally DMA ENABLE should only be used in Master Message mode."]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl MdmactrlDmatb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MdmactrlDmatb {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MdmactrlDmatb {
    #[inline(always)]
    fn from(val: u8) -> MdmactrlDmatb {
        MdmactrlDmatb::from_bits(val)
    }
}
impl From<MdmactrlDmatb> for u8 {
    #[inline(always)]
    fn from(val: MdmactrlDmatb) -> u8 {
        MdmactrlDmatb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MdmactrlDmawidth {
    #[doc = "BYTE"]
    BYTE = 0x0,
    #[doc = "BYTE_AGAIN"]
    BYTE_AGAIN = 0x01,
    #[doc = "HALF_WORD: Half-word (16 bits). This will make sure that 2 bytes are free/available in FIFO."]
    HALF_WORD = 0x02,
    _RESERVED_3 = 0x03,
}
impl MdmactrlDmawidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MdmactrlDmawidth {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MdmactrlDmawidth {
    #[inline(always)]
    fn from(val: u8) -> MdmactrlDmawidth {
        MdmactrlDmawidth::from_bits(val)
    }
}
impl From<MdmactrlDmawidth> for u8 {
    #[inline(always)]
    fn from(val: MdmactrlDmawidth) -> u8 {
        MdmactrlDmawidth::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstena {
    #[doc = "MASTER_OFF: Master is off (is not enabled). If MASTER_OFF is enabled, then the I3C module can only use slave mode."]
    MASTER_OFF = 0x0,
    #[doc = "MASTER_ON: Master is on (is enabled). When used from start-up, this I3C module is master by default (the main master). The module will control the bus unless the master is handed off. If the master is handed off, then MSTENA must move to 2 after that happens. The handoff means emitting GETACCMST and if accepted, the module will emit a STOP and set the MSTENA bit to 2 (or 0)."]
    MASTER_ON = 0x01,
    #[doc = "MASTER_CAPABLE: The I3C module is master-capable; however the module is operating as a slave now. When used from the start, the I3C module will start as a slave, but will be prepared to switch to master mode. To switch to master mode, the slave emits an Master Request (MR), or gets a GETACCMST CCC command and accepts it (to switch on the STOP)."]
    MASTER_CAPABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl Mstena {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mstena {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mstena {
    #[inline(always)]
    fn from(val: u8) -> Mstena {
        Mstena::from_bits(val)
    }
}
impl From<Mstena> for u8 {
    #[inline(always)]
    fn from(val: Mstena) -> u8 {
        Mstena::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MwmsgSdrControlDir {
    #[doc = "Write"]
    WRITE = 0x0,
    #[doc = "Read"]
    READ = 0x01,
}
impl MwmsgSdrControlDir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MwmsgSdrControlDir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MwmsgSdrControlDir {
    #[inline(always)]
    fn from(val: u8) -> MwmsgSdrControlDir {
        MwmsgSdrControlDir::from_bits(val)
    }
}
impl From<MwmsgSdrControlDir> for u8 {
    #[inline(always)]
    fn from(val: MwmsgSdrControlDir) -> u8 {
        MwmsgSdrControlDir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Request {
    #[doc = "NONE: Returns to this when finished with any request. The MSTATUS register indicates the master's state. See also AutoIBI mode. NONE is only written as 0: when setting RDTERM to 1 (to stop a read in progress) or when setting IBI reponse field (IBIRESP) for MSG use"]
    NONE = 0x0,
    #[doc = "EMITSTARTADDR: Emit START with address and direction from a stopped state or in the middle of a Single Data Rate (SDR) message. If from a stopped state (IDLE), then emit start may be prevented by an event (like IBI, MR, HJ), in which case the appropriate interrupt is signaled; note that Emit START can be resubmitted."]
    EMITSTARTADDR = 0x01,
    #[doc = "EMITSTOP: Emit a STOP on bus. Must be in Single Data Rate (SDR) mode. If in Dynamic Address Assignment (DAA) mode, Emit stop will exit DAA mode."]
    EMITSTOP = 0x02,
    #[doc = "IBIACKNACK: Manual In-Band Interrupt (IBI) Acknowledge (ACK) or Not Acknowledge (NACK). When IBIRESP has indicated a hold on an In-Band Interrupt to allow a manual decision, this request completes it. Uses IBIRESP to provide the information."]
    IBIACKNACK = 0x03,
    #[doc = "PROCESSDAA: If not in Dynamic Address Assignment (DAA) mode now, will issue START, 7E, ENTDAA, and then will emit 7E/R to process each slave. Will stop just before the new Dynamic Address (DA) is to be emitted. The next Process DAA request will use the Addr field as the new DA to assign. If NACKed on the 7E/R, then the interrupt will indicate this situation, and a STOP will be emitted."]
    PROCESSDAA = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "FORCEEXIT and IBHR: Emit an Exit Pattern from any state, but end Double Data Rate (DDR) (including MSGDDR), if in DDR mode now. Includes a STOP afterward. If TYPE != 0, then it will perform an IBHR (In-Band Hardware Reset). If TYPE=2, then it does a normal reset (DEFRST can prevent the reset). If TYPE=3, it does a forced reset (will always reset)."]
    FORCEEXIT = 0x06,
    #[doc = "AUTOIBI: Hold in a stopped state, but auto-emit START,7E when the slave is holding down SDA to get an In-Band Interrupt (IBI). Actual In-Band Interrupt handling is defined by IBIRESP."]
    AUTOIBI = 0x07,
}
impl Request {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Request {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Request {
    #[inline(always)]
    fn from(val: u8) -> Request {
        Request::from_bits(val)
    }
}
impl From<Request> for u8 {
    #[inline(always)]
    fn from(val: Request) -> u8 {
        Request::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxempty {
    #[doc = "RX is not empty"]
    RXISNOTEMPTY = 0x0,
    #[doc = "RX is empty"]
    RXISEMPTY = 0x01,
}
impl Rxempty {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxempty {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxempty {
    #[inline(always)]
    fn from(val: u8) -> Rxempty {
        Rxempty::from_bits(val)
    }
}
impl From<Rxempty> for u8 {
    #[inline(always)]
    fn from(val: Rxempty) -> u8 {
        Rxempty::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxtrig {
    #[doc = "Trigger on not empty"]
    TRIGGRNOTEMPTY = 0x0,
    #[doc = "Trigger on or more full"]
    TRIGGRONEFOURTH = 0x01,
    #[doc = "Trigger on .5 or more full"]
    TRIGGRONEHALF = 0x02,
    #[doc = "Trigger on 3/4 or more full"]
    TRIGGRTHREEFOURTHS = 0x03,
}
impl Rxtrig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxtrig {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxtrig {
    #[inline(always)]
    fn from(val: u8) -> Rxtrig {
        Rxtrig::from_bits(val)
    }
}
impl From<Rxtrig> for u8 {
    #[inline(always)]
    fn from(val: Rxtrig) -> u8 {
        Rxtrig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Saddr {
    #[doc = "NO_STATIC: No static address"]
    NO_STATIC = 0x0,
    #[doc = "STATIC: Static address is fixed in hardware"]
    STATIC = 0x01,
    #[doc = "HW_CONTROL: Hardware controls the static address dynamically (for example, from the pin strap)"]
    HW_CONTROL = 0x02,
    #[doc = "CONFIG: SCONFIG register supplies the static address"]
    CONFIG = 0x03,
}
impl Saddr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Saddr {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Saddr {
    #[inline(always)]
    fn from(val: u8) -> Saddr {
        Saddr::from_bits(val)
    }
}
impl From<Saddr> for u8 {
    #[inline(always)]
    fn from(val: Saddr) -> u8 {
        Saddr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ScapabilitiesTimectrl {
    #[doc = "NO_TIME_CONTROL_TYPE: No time control is enabled"]
    NO_TIME_CONTROL_TYPE = 0x0,
    #[doc = "ATLEAST1_TIME_CONTROL: at least one time-control type is supported"]
    ATLEAST1_TIME_CONTROL = 0x01,
}
impl ScapabilitiesTimectrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ScapabilitiesTimectrl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ScapabilitiesTimectrl {
    #[inline(always)]
    fn from(val: u8) -> ScapabilitiesTimectrl {
        ScapabilitiesTimectrl::from_bits(val)
    }
}
impl From<ScapabilitiesTimectrl> for u8 {
    #[inline(always)]
    fn from(val: ScapabilitiesTimectrl) -> u8 {
        ScapabilitiesTimectrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SdmactrlDmafb {
    #[doc = "DMA not used"]
    NOT_USED = 0x0,
    #[doc = "DMA is enabled for 1 frame"]
    ENABLE_ONE_FRAME = 0x01,
    #[doc = "DMA enable"]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl SdmactrlDmafb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SdmactrlDmafb {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SdmactrlDmafb {
    #[inline(always)]
    fn from(val: u8) -> SdmactrlDmafb {
        SdmactrlDmafb::from_bits(val)
    }
}
impl From<SdmactrlDmafb> for u8 {
    #[inline(always)]
    fn from(val: SdmactrlDmafb) -> u8 {
        SdmactrlDmafb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SdmactrlDmatb {
    #[doc = "NOT_USED: DMA is not used"]
    NOT_USED = 0x0,
    #[doc = "ENABLE_ONE_FRAME: DMA is enabled for 1 Frame (ended by DMA or terminated). DMATB auto-clears on a STOP or START (see the Match START or STOP bit (SCONFIG.MATCHSS)."]
    ENABLE_ONE_FRAME = 0x01,
    #[doc = "ENABLE: DMA is enabled until turned off. Normally, ENABLE should only be used with Master Message mode."]
    ENABLE = 0x02,
    _RESERVED_3 = 0x03,
}
impl SdmactrlDmatb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SdmactrlDmatb {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SdmactrlDmatb {
    #[inline(always)]
    fn from(val: u8) -> SdmactrlDmatb {
        SdmactrlDmatb::from_bits(val)
    }
}
impl From<SdmactrlDmatb> for u8 {
    #[inline(always)]
    fn from(val: SdmactrlDmatb) -> u8 {
        SdmactrlDmatb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SdmactrlDmawidth {
    #[doc = "BYTE"]
    BYTE = 0x0,
    #[doc = "BYTE_AGAIN"]
    BYTE_AGAIN = 0x01,
    #[doc = "HALF_WORD: Half word (16 bits). This will make sure that 2 bytes are free/available in the FIFO."]
    HALF_WORD = 0x02,
    _RESERVED_3 = 0x03,
}
impl SdmactrlDmawidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SdmactrlDmawidth {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SdmactrlDmawidth {
    #[inline(always)]
    fn from(val: u8) -> SdmactrlDmawidth {
        SdmactrlDmawidth::from_bits(val)
    }
}
impl From<SdmactrlDmawidth> for u8 {
    #[inline(always)]
    fn from(val: SdmactrlDmawidth) -> u8 {
        SdmactrlDmawidth::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SstatusTimectrl {
    #[doc = "NO_TIME_CONTROL: No time control is enabled"]
    NO_TIME_CONTROL = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "ASYNC_MODE: Asynchronous standard mode (0) is enabled"]
    ASYNC_MODE = 0x02,
    _RESERVED_3 = 0x03,
}
impl SstatusTimectrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SstatusTimectrl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SstatusTimectrl {
    #[inline(always)]
    fn from(val: u8) -> SstatusTimectrl {
        SstatusTimectrl::from_bits(val)
    }
}
impl From<SstatusTimectrl> for u8 {
    #[inline(always)]
    fn from(val: SstatusTimectrl) -> u8 {
        SstatusTimectrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum State {
    #[doc = "IDLE: the bus has STOPped."]
    IDLE = 0x0,
    #[doc = "SLVREQ: (Slave Request state) the bus has STOPped but a slave is holding SDA low. If using auto-emit IBI (MCTRL.AutoIBI), then the master will not stay in the Slave Request state."]
    SLVREQ = 0x01,
    #[doc = "MSGSDR: in Single Data Rate (SDR) Message state (from using MWMSG_SDR)"]
    MSGSDR = 0x02,
    #[doc = "NORMACT: normal active Single Data Rate (SDR) state (from using MCTRL and MWDATAn and MRDATAn registers). The master will stay in the NORMACT state until a STOP is issued."]
    NORMACT = 0x03,
    #[doc = "MSGDDR: in Double Data Rate (DDR) Message mode (from using MWMSG_DDR or using the normal method with DDR). The master will stay in the DDR state, until the master exits using EXIT (emits the Exit pattern)."]
    DDR = 0x04,
    #[doc = "DAA: in Enter Dynamic Address Assignment (ENTDAA) mode"]
    DAA = 0x05,
    #[doc = "IBIACK: waiting for an In-Band Interrupt (IBI) ACK/NACK decision"]
    IBIACK = 0x06,
    #[doc = "IBIRCV: Receiving an In-Band Interrupt (IBI); this IBIRCV state is used after IBI/MR/HJ has won the arbitration, and IBIRCV state is also used for IBI mandatory byte (if any) and any bytes that follow."]
    IBIRCV = 0x07,
}
impl State {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> State {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for State {
    #[inline(always)]
    fn from(val: u8) -> State {
        State::from_bits(val)
    }
}
impl From<State> for u8 {
    #[inline(always)]
    fn from(val: State) -> u8 {
        State::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txfull {
    #[doc = "TX is not full"]
    TXISNOTFULL = 0x0,
    #[doc = "TX is full"]
    TXISFULL = 0x01,
}
impl Txfull {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txfull {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txfull {
    #[inline(always)]
    fn from(val: u8) -> Txfull {
        Txfull::from_bits(val)
    }
}
impl From<Txfull> for u8 {
    #[inline(always)]
    fn from(val: Txfull) -> u8 {
        Txfull::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txtrig {
    #[doc = "Trigger on empty"]
    TRIGGREMPTY = 0x0,
    #[doc = "Trigger on full or less"]
    TRIGGRONEFOURTH = 0x01,
    #[doc = "Trigger on .5 full or less"]
    TRIGGRONEHALF = 0x02,
    #[doc = "Trigger on 1 less than full or less (Default)"]
    TRIGGRONELESS = 0x03,
}
impl Txtrig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txtrig {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txtrig {
    #[inline(always)]
    fn from(val: u8) -> Txtrig {
        Txtrig::from_bits(val)
    }
}
impl From<Txtrig> for u8 {
    #[inline(always)]
    fn from(val: Txtrig) -> u8 {
        Txtrig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Type {
    #[doc = "I3C: Normally the SDR mode of I3C. For ForceExit, the Exit pattern."]
    I3C = 0x0,
    #[doc = "I2C: Normally the Standard I2C protocol."]
    I2C = 0x01,
    #[doc = "DDR: (Double Data Rate): Normally the HDR-DDR mode of I3C. Enter DDR mode (7E and then ENTHDR0), if the module is not already in DDR mode. The 1st byte written to the TX FIFO should be a command, and should already be in the FIFO. To end DDR mode, use ForceExit. For ForceExit, the normal IBHR (In-Band Hardware Reset)."]
    DDR = 0x02,
    #[doc = "For ForcedExit, this is forced IBHR."]
    FORCEDIBHR = 0x03,
}
impl Type {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Type {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Type {
    #[inline(always)]
    fn from(val: u8) -> Type {
        Type::from_bits(val)
    }
}
impl From<Type> for u8 {
    #[inline(always)]
    fn from(val: Type) -> u8 {
        Type::to_bits(val)
    }
}
