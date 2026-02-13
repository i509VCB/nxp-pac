#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Actstate {
    #[doc = "NO_LATENCY (normal bus operations)"]
    NO_LATENCY = 0x0,
    #[doc = "LATENCY_1MS (1 ms of latency)"]
    LATENCY_1MS = 0x01,
    #[doc = "LATENCY_100MS (100 ms of latency)"]
    LATENCY_100MS = 0x02,
    #[doc = "LATENCY_10S (10 seconds of latency)"]
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
pub enum Cause {
    #[doc = "No information (this value occurs when not configured to write DA)"]
    NONE = 0x0,
    #[doc = "Set using ENTDAA"]
    ENTDAA = 0x01,
    #[doc = "Set using SETDASA, SETAASA, or SETNEWDA"]
    SETDASA = 0x02,
    #[doc = "Cleared using RSTDAA"]
    RSTDAA = 0x03,
    #[doc = "Auto MAP change happened last"]
    AUTOMAP = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Cause {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cause {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cause {
    #[inline(always)]
    fn from(val: u8) -> Cause {
        Cause::from_bits(val)
    }
}
impl From<Cause> for u8 {
    #[inline(always)]
    fn from(val: Cause) -> u8 {
        Cause::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ccchandle {
    #[doc = "All handling features disabled"]
    ALL_DISABLED = 0x0,
    #[doc = "The I3C module manages events, activities, status, HDR, and if enabled for it, ID and static-address-related items"]
    BLOCK_HANDLE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
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
}
impl Ccchandle {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ccchandle {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ccchandle {
    #[inline(always)]
    fn from(val: u8) -> Ccchandle {
        Ccchandle::from_bits(val)
    }
}
impl From<Ccchandle> for u8 {
    #[inline(always)]
    fn from(val: Ccchandle) -> u8 {
        Ccchandle::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Disto {
    #[doc = "Enabled"]
    ENABLE = 0x0,
    #[doc = "Disabled, if configured"]
    DISABLE = 0x01,
}
impl Disto {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Disto {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Disto {
    #[inline(always)]
    fn from(val: u8) -> Disto {
        Disto::from_bits(val)
    }
}
impl From<Disto> for u8 {
    #[inline(always)]
    fn from(val: Disto) -> u8 {
        Disto::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma {
    #[doc = "Not supported"]
    DMANO = 0x0,
    #[doc = "Supported"]
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
    #[doc = "NONE (no event or no pending event)"]
    NONE = 0x0,
    #[doc = "NO_REQUEST (request is not sent yet; either there is no START condition yet, or is waiting for Bus-Available or Bus-Idle (HJ))"]
    NO_REQUEST = 0x01,
    #[doc = "NACKed (not acknowledged, request sent and rejected); I3C tries again"]
    NACKED = 0x02,
    #[doc = "ACKed (acknowledged; request sent and accepted), so done (unless the time control data is still being sent)"]
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
pub enum Extfifo {
    #[doc = "No external FIFO available"]
    NO_EXT_FIFO = 0x0,
    #[doc = "Standard available or free external FIFO"]
    STD_EXT_FIFO = 0x01,
    #[doc = "Request track external FIFO"]
    REQUEST_EXT_FIFO = 0x02,
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
    #[doc = "Two or three"]
    FIFO_2BYTE = 0x0,
    #[doc = "Four"]
    FIFO_4BYTE = 0x01,
    #[doc = "Eight"]
    FIFO_8BYTE = 0x02,
    #[doc = "16 or larger"]
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
    #[doc = "Two"]
    FIFO_2BYTE = 0x0,
    #[doc = "Four"]
    FIFO_4BYTE = 0x01,
    #[doc = "Eight"]
    FIFO_8BYTE = 0x02,
    #[doc = "16 or larger"]
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
pub enum Group {
    #[doc = "v1.1 group addressing not supported"]
    NOTSUPPORTED = 0x0,
    #[doc = "One group supported"]
    ONE = 0x01,
    #[doc = "Two groups supported"]
    TWO = 0x02,
    #[doc = "Three groups supported"]
    THREE = 0x03,
}
impl Group {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Group {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Group {
    #[inline(always)]
    fn from(val: u8) -> Group {
        Group::from_bits(val)
    }
}
impl From<Group> for u8 {
    #[inline(always)]
    fn from(val: Group) -> u8 {
        Group::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hdrsupp {
    #[doc = "No HDR modes supported"]
    NO_HDR = 0x0,
    #[doc = "DDR mode supported"]
    DDR = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Hdrsupp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hdrsupp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hdrsupp {
    #[inline(always)]
    fn from(val: u8) -> Hdrsupp {
        Hdrsupp::from_bits(val)
    }
}
impl From<Hdrsupp> for u8 {
    #[inline(always)]
    fn from(val: Hdrsupp) -> u8 {
        Hdrsupp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hjdis {
    #[doc = "Enabled"]
    MR_ENABLED = 0x0,
    #[doc = "Disabled"]
    MR_DISABLED = 0x01,
}
impl Hjdis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hjdis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hjdis {
    #[inline(always)]
    fn from(val: u8) -> Hjdis {
        Hjdis::from_bits(val)
    }
}
impl From<Hjdis> for u8 {
    #[inline(always)]
    fn from(val: Hjdis) -> u8 {
        Hjdis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hkeep {
    #[doc = "None"]
    NONE = 0x0,
    #[doc = "WIRED_IN"]
    WIRED_IN = 0x01,
    #[doc = "PASSIVE_SDA (I2C mode, no clock stretches mode)"]
    PASSIVE_SDA = 0x02,
    #[doc = "PASSIVE_ON_SDA_SCL"]
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
pub enum I3cCasDel {
    #[doc = "No delay"]
    NO_DELAY = 0x0,
    #[doc = "Increases SCL clock period by 1/2"]
    ONE_HALF_CLK = 0x01,
    #[doc = "Increases SCL clock period by 1"]
    ONE_CLK = 0x02,
    #[doc = "Increases SCL clock period by 3/2"]
    ONE_AND_ONE_HALF_CLK = 0x03,
}
impl I3cCasDel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3cCasDel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3cCasDel {
    #[inline(always)]
    fn from(val: u8) -> I3cCasDel {
        I3cCasDel::from_bits(val)
    }
}
impl From<I3cCasDel> for u8 {
    #[inline(always)]
    fn from(val: I3cCasDel) -> u8 {
        I3cCasDel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3cCasrDel {
    #[doc = "No delay"]
    NO_DELAY = 0x0,
    #[doc = "Increases SCL clock period by 1/2"]
    ONE_HALF_CLK = 0x01,
    #[doc = "Increases SCL clock period by 1"]
    ONE_CLK = 0x02,
    #[doc = "Increases SCL clock period by 1 1/2"]
    ONE_AND_ONE_HALF_CLK = 0x03,
}
impl I3cCasrDel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3cCasrDel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3cCasrDel {
    #[inline(always)]
    fn from(val: u8) -> I3cCasrDel {
        I3cCasrDel::from_bits(val)
    }
}
impl From<I3cCasrDel> for u8 {
    #[inline(always)]
    fn from(val: I3cCasrDel) -> u8 {
        I3cCasrDel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IbiMrHj {
    #[doc = "Application cannot generate IBI, CR, or HJ"]
    ALL_DISABLED = 0x0,
    #[doc = "Application can generate an IBI"]
    IBI = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
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
    _RESERVED_1f = 0x1f,
}
impl IbiMrHj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IbiMrHj {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IbiMrHj {
    #[inline(always)]
    fn from(val: u8) -> IbiMrHj {
        IbiMrHj::from_bits(val)
    }
}
impl From<IbiMrHj> for u8 {
    #[inline(always)]
    fn from(val: IbiMrHj) -> u8 {
        IbiMrHj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ibidis {
    #[doc = "Enabled"]
    INTERRUPTS_ENABLED = 0x0,
    #[doc = "Disabled"]
    INTERRUPTS_DISABLED = 0x01,
}
impl Ibidis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ibidis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ibidis {
    #[inline(always)]
    fn from(val: u8) -> Ibidis {
        Ibidis::from_bits(val)
    }
}
impl From<Ibidis> for u8 {
    #[inline(always)]
    fn from(val: Ibidis) -> u8 {
        Ibidis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ibiresp {
    #[doc = "ACK (acknowledge)"]
    ACK = 0x0,
    #[doc = "NACK (reject)"]
    NACK = 0x01,
    #[doc = "Acknowledge with mandatory byte"]
    ACK_WITH_MANDATORY = 0x02,
    #[doc = "Manual"]
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
    #[doc = "NONE (no IBI: this status occurs when MSTATUS\\[IBIWON\\] becomes 0)"]
    NONE = 0x0,
    #[doc = "IBI"]
    IBI = 0x01,
    #[doc = "CR"]
    MR = 0x02,
    #[doc = "HJ"]
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
    #[doc = "Application"]
    APPLICATION = 0x0,
    #[doc = "Hardware"]
    HW = 0x01,
    #[doc = "Hardware, but the I3C module instance handles ID 48b"]
    HW_BUT = 0x02,
    #[doc = "A part number register (PARTNO)"]
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
pub enum Idreg {
    #[doc = "All ID register features disabled"]
    ALL_DISABLED = 0x0,
    #[doc = "ID Instance is a register; used if there is no PARTNO register"]
    ID_INSTANCE = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
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
}
impl Idreg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Idreg {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Idreg {
    #[inline(always)]
    fn from(val: u8) -> Idreg {
        Idreg::from_bits(val)
    }
}
impl From<Idreg> for u8 {
    #[inline(always)]
    fn from(val: Idreg) -> u8 {
        Idreg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Int {
    #[doc = "Not supported"]
    INTERRUPTSNO = 0x0,
    #[doc = "Supported"]
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
pub enum Laststatic {
    #[doc = "I3C dynamic address"]
    I3C = 0x0,
    #[doc = "I2C static address"]
    I2C = 0x01,
}
impl Laststatic {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Laststatic {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Laststatic {
    #[inline(always)]
    fn from(val: u8) -> Laststatic {
        Laststatic::from_bits(val)
    }
}
impl From<Laststatic> for u8 {
    #[inline(always)]
    fn from(val: Laststatic) -> u8 {
        Laststatic::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Master {
    #[doc = "Not supported"]
    MASTERNOTSUPPORTED = 0x0,
    #[doc = "Supported"]
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
    #[doc = "Write"]
    DIRWRITE = 0x0,
    #[doc = "Read"]
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
pub enum MdatactrlRxtrig {
    #[doc = "Trigger when not empty (default)"]
    NOT_EMPTY = 0x0,
    #[doc = "Trigger when 1/4 full or more"]
    QUARTER_OR_MORE = 0x01,
    #[doc = "Trigger when 1/2 full or more"]
    HALF_OR_MORE = 0x02,
    #[doc = "Trigger when 3/4 full or more"]
    THREE_QUARTER_OR_MORE = 0x03,
}
impl MdatactrlRxtrig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MdatactrlRxtrig {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MdatactrlRxtrig {
    #[inline(always)]
    fn from(val: u8) -> MdatactrlRxtrig {
        MdatactrlRxtrig::from_bits(val)
    }
}
impl From<MdatactrlRxtrig> for u8 {
    #[inline(always)]
    fn from(val: MdatactrlRxtrig) -> u8 {
        MdatactrlRxtrig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MdatactrlTxtrig {
    #[doc = "Trigger when empty"]
    EMPTY = 0x0,
    #[doc = "Trigger when 1/4 full or less"]
    QUARTER_OR_LESS = 0x01,
    #[doc = "Trigger when 1/2 full or less"]
    HALF_OR_LESS = 0x02,
    #[doc = "Trigger when 1 less than full or less (default)"]
    FULL_OR_LESS = 0x03,
}
impl MdatactrlTxtrig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MdatactrlTxtrig {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MdatactrlTxtrig {
    #[inline(always)]
    fn from(val: u8) -> MdatactrlTxtrig {
        MdatactrlTxtrig::from_bits(val)
    }
}
impl From<MdatactrlTxtrig> for u8 {
    #[inline(always)]
    fn from(val: MdatactrlTxtrig) -> u8 {
        MdatactrlTxtrig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MdmactrlDmafb {
    #[doc = "DMA not used"]
    NOT_USED = 0x0,
    #[doc = "Enable DMA for one frame"]
    ENABLE_ONE_FRAME = 0x01,
    #[doc = "Enable DMA until DMA is turned off"]
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
    #[doc = "DMA not used"]
    NOT_USED = 0x0,
    #[doc = "Enable DMA for one frame (ended by DMA or terminated)"]
    ENABLE_ONE_FRAME = 0x01,
    #[doc = "Enable DMA until DMA is turned off"]
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
    #[doc = "Byte"]
    BYTE_0 = 0x0,
    #[doc = "Byte"]
    BYTE_1 = 0x01,
    #[doc = "Halfword (16 bits)"]
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
pub enum Mrdis {
    #[doc = "Enabled"]
    MR_ENABLED = 0x0,
    #[doc = "Disabled"]
    MR_DISABLED = 0x01,
}
impl Mrdis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mrdis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mrdis {
    #[inline(always)]
    fn from(val: u8) -> Mrdis {
        Mrdis::from_bits(val)
    }
}
impl From<Mrdis> for u8 {
    #[inline(always)]
    fn from(val: Mrdis) -> u8 {
        Mrdis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mstena {
    #[doc = "CONTROLLER_OFF"]
    MASTER_OFF = 0x0,
    #[doc = "CONTROLLER_ON"]
    MASTER_ON = 0x01,
    #[doc = "CONTROLLER_CAPABLE"]
    MASTER_CAPABLE = 0x02,
    #[doc = "I2C_CONTROLLER_MODE"]
    I2C_MASTER_MODE = 0x03,
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
pub enum Nobyte {
    #[doc = "With mandatory IBI byte"]
    IBIBYTE = 0x0,
    #[doc = "Without mandatory IBI byte"]
    NO_IBIBYTE = 0x01,
}
impl Nobyte {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nobyte {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nobyte {
    #[inline(always)]
    fn from(val: u8) -> Nobyte {
        Nobyte::from_bits(val)
    }
}
impl From<Nobyte> for u8 {
    #[inline(always)]
    fn from(val: Nobyte) -> u8 {
        Nobyte::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Request {
    #[doc = "NONE"]
    NONE = 0x0,
    #[doc = "EMITSTARTADDR"]
    EMITSTARTADDR = 0x01,
    #[doc = "EMITSTOP"]
    EMITSTOP = 0x02,
    #[doc = "IBIACKNACK"]
    IBIACKNACK = 0x03,
    #[doc = "PROCESSDAA"]
    PROCESSDAA = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "Force Exit and Target Reset"]
    FORCEEXIT = 0x06,
    #[doc = "AUTOIBI"]
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
pub enum Saddr {
    #[doc = "No static address"]
    NO_STATIC = 0x0,
    #[doc = "Static address is fixed in hardware"]
    STATIC = 0x01,
    #[doc = "Hardware controls the static address dynamically (for example, from the pin strap)"]
    HW_CONTROL = 0x02,
    #[doc = "SCONFIG register supplies the static address"]
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
    #[doc = "No time control supported"]
    NO_TIME_CONTROL_TYPE = 0x0,
    #[doc = "At least one time-control type supported"]
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
pub enum SctrlEvent {
    #[doc = "NORMAL_MODE"]
    NORMAL_MODE = 0x0,
    #[doc = "IBI"]
    IBI = 0x01,
    #[doc = "CONTROLLER_REQUEST"]
    MASTER_REQUEST = 0x02,
    #[doc = "HOT_JOIN_REQUEST"]
    HOT_JOIN_REQUEST = 0x03,
}
impl SctrlEvent {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SctrlEvent {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SctrlEvent {
    #[inline(always)]
    fn from(val: u8) -> SctrlEvent {
        SctrlEvent::from_bits(val)
    }
}
impl From<SctrlEvent> for u8 {
    #[inline(always)]
    fn from(val: SctrlEvent) -> u8 {
        SctrlEvent::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SdatactrlRxempty {
    #[doc = "Not empty"]
    RXISNOTEMPTY = 0x0,
    #[doc = "Empty"]
    RXISEMPTY = 0x01,
}
impl SdatactrlRxempty {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SdatactrlRxempty {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SdatactrlRxempty {
    #[inline(always)]
    fn from(val: u8) -> SdatactrlRxempty {
        SdatactrlRxempty::from_bits(val)
    }
}
impl From<SdatactrlRxempty> for u8 {
    #[inline(always)]
    fn from(val: SdatactrlRxempty) -> u8 {
        SdatactrlRxempty::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SdatactrlRxtrig {
    #[doc = "Trigger when not empty (default)"]
    TRIGGRNOTEMPTY = 0x0,
    #[doc = "Trigger when 1/4 or more full"]
    TRIGGRONEFOURTH = 0x01,
    #[doc = "Trigger when 1/2 or more full"]
    TRIGGRONEHALF = 0x02,
    #[doc = "Trigger when 3/4 or more full"]
    TRIGGRTHREEFOURTHS = 0x03,
}
impl SdatactrlRxtrig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SdatactrlRxtrig {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SdatactrlRxtrig {
    #[inline(always)]
    fn from(val: u8) -> SdatactrlRxtrig {
        SdatactrlRxtrig::from_bits(val)
    }
}
impl From<SdatactrlRxtrig> for u8 {
    #[inline(always)]
    fn from(val: SdatactrlRxtrig) -> u8 {
        SdatactrlRxtrig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SdatactrlTxfull {
    #[doc = "Not full"]
    TXISNOTFULL = 0x0,
    #[doc = "Full"]
    TXISFULL = 0x01,
}
impl SdatactrlTxfull {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SdatactrlTxfull {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SdatactrlTxfull {
    #[inline(always)]
    fn from(val: u8) -> SdatactrlTxfull {
        SdatactrlTxfull::from_bits(val)
    }
}
impl From<SdatactrlTxfull> for u8 {
    #[inline(always)]
    fn from(val: SdatactrlTxfull) -> u8 {
        SdatactrlTxfull::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SdatactrlTxtrig {
    #[doc = "Trigger when empty"]
    TRIGGREMPTY = 0x0,
    #[doc = "Trigger when 1/4 full or less"]
    TRIGGRONEFOURTH = 0x01,
    #[doc = "Trigger when 1/2 full or less"]
    TRIGGRONEHALF = 0x02,
    #[doc = "Default (trigger when 1 less than full or less)"]
    TRIGGRONELESS = 0x03,
}
impl SdatactrlTxtrig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SdatactrlTxtrig {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SdatactrlTxtrig {
    #[inline(always)]
    fn from(val: u8) -> SdatactrlTxtrig {
        SdatactrlTxtrig::from_bits(val)
    }
}
impl From<SdatactrlTxtrig> for u8 {
    #[inline(always)]
    fn from(val: SdatactrlTxtrig) -> u8 {
        SdatactrlTxtrig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SdmactrlDmafb {
    #[doc = "DMA not used"]
    NOT_USED = 0x0,
    #[doc = "DMA enabled for one frame"]
    ENABLE_ONE_FRAME = 0x01,
    #[doc = "DMA enabled until turned off"]
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
    #[doc = "DMA not used"]
    NOT_USED = 0x0,
    #[doc = "DMA enabled for one frame"]
    ENABLE_ONE_FRAME = 0x01,
    #[doc = "DMA enabled until turned off"]
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
    #[doc = "Byte"]
    BYTE_0 = 0x0,
    #[doc = "Byte"]
    BYTE_1 = 0x01,
    #[doc = "Halfword (16 bits) (this value ensures that two bytes are available in the FIFO)"]
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
pub enum SdynaddrDavalid {
    #[doc = "DANOTASSIGNED: a dynamic address is not assigned"]
    DANOTASSIGNED = 0x0,
    #[doc = "DAASSIGNED: a dynamic address is assigned"]
    DAASSIGNED = 0x01,
}
impl SdynaddrDavalid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SdynaddrDavalid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SdynaddrDavalid {
    #[inline(always)]
    fn from(val: u8) -> SdynaddrDavalid {
        SdynaddrDavalid::from_bits(val)
    }
}
impl From<SdynaddrDavalid> for u8 {
    #[inline(always)]
    fn from(val: SdynaddrDavalid) -> u8 {
        SdynaddrDavalid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SstatusStart {
    #[doc = "Not detected"]
    START_NOT_DETECTED = 0x0,
    #[doc = "Detected"]
    START_DETECTED = 0x01,
}
impl SstatusStart {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SstatusStart {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SstatusStart {
    #[inline(always)]
    fn from(val: u8) -> SstatusStart {
        SstatusStart::from_bits(val)
    }
}
impl From<SstatusStart> for u8 {
    #[inline(always)]
    fn from(val: SstatusStart) -> u8 {
        SstatusStart::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SstatusTimectrl {
    #[doc = "NO_TIME_CONTROL (no time control is enabled)"]
    NO_TIME_CONTROL = 0x0,
    #[doc = "SYNC_MODE (Synchronous mode is enabled)"]
    SYNC = 0x01,
    #[doc = "ASYNC_MODE (Asynchronous standard mode (0 or 1) is enabled)"]
    ASYNC_MODE = 0x02,
    #[doc = "BOTHSYNCASYNC (both Synchronous and Asynchronous modes are enabled)"]
    BOTHSYNCASYNC = 0x03,
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
pub enum SstatusTxnotfull {
    #[doc = "Transmit buffer full"]
    FULL = 0x0,
    #[doc = "Transmit buffer not full"]
    NOT_FULL = 0x01,
}
impl SstatusTxnotfull {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SstatusTxnotfull {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SstatusTxnotfull {
    #[inline(always)]
    fn from(val: u8) -> SstatusTxnotfull {
        SstatusTxnotfull::from_bits(val)
    }
}
impl From<SstatusTxnotfull> for u8 {
    #[inline(always)]
    fn from(val: SstatusTxnotfull) -> u8 {
        SstatusTxnotfull::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum State {
    #[doc = "IDLE (bus has stopped)"]
    IDLE = 0x0,
    #[doc = "SLVREQ (target request)"]
    SLVREQ = 0x01,
    #[doc = "MSGSDR"]
    MSGSDR = 0x02,
    #[doc = "NORMACT"]
    NORMACT = 0x03,
    #[doc = "MSGDDR"]
    DDR = 0x04,
    #[doc = "DAA"]
    DAA = 0x05,
    #[doc = "IBIACK"]
    IBIACK = 0x06,
    #[doc = "IBIRCV"]
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
pub enum Stccch {
    #[doc = "No CCC message handled"]
    IDLE = 0x0,
    #[doc = "Handled automatically"]
    BUSY = 0x01,
}
impl Stccch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stccch {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stccch {
    #[inline(always)]
    fn from(val: u8) -> Stccch {
        Stccch::from_bits(val)
    }
}
impl From<Stccch> for u8 {
    #[inline(always)]
    fn from(val: Stccch) -> u8 {
        Stccch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Stmsg {
    #[doc = "Idle"]
    IDLE = 0x0,
    #[doc = "Busy"]
    BUSY = 0x01,
}
impl Stmsg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stmsg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stmsg {
    #[inline(always)]
    fn from(val: u8) -> Stmsg {
        Stmsg::from_bits(val)
    }
}
impl From<Stmsg> for u8 {
    #[inline(always)]
    fn from(val: Stmsg) -> u8 {
        Stmsg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Stnotstop {
    #[doc = "In STOP condition"]
    STOPPED = 0x0,
    #[doc = "Busy"]
    BUSY = 0x01,
}
impl Stnotstop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stnotstop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stnotstop {
    #[inline(always)]
    fn from(val: u8) -> Stnotstop {
        Stnotstop::from_bits(val)
    }
}
impl From<Stnotstop> for u8 {
    #[inline(always)]
    fn from(val: Stnotstop) -> u8 {
        Stnotstop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Streqrd {
    #[doc = "Not an SDR read"]
    IDLE = 0x0,
    #[doc = "SDR read from this target or an IBI is being pushed out"]
    BUSY = 0x01,
}
impl Streqrd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Streqrd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Streqrd {
    #[inline(always)]
    fn from(val: u8) -> Streqrd {
        Streqrd::from_bits(val)
    }
}
impl From<Streqrd> for u8 {
    #[inline(always)]
    fn from(val: Streqrd) -> u8 {
        Streqrd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Streqwr {
    #[doc = "Not an SDR write"]
    IDLE = 0x0,
    #[doc = "SDR write data from the controller, but not in ENTDAA mode"]
    BUSY = 0x01,
}
impl Streqwr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Streqwr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Streqwr {
    #[inline(always)]
    fn from(val: u8) -> Streqwr {
        Streqwr::from_bits(val)
    }
}
impl From<Streqwr> for u8 {
    #[inline(always)]
    fn from(val: Streqwr) -> u8 {
        Streqwr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Type {
    #[doc = "I3C"]
    I3C = 0x0,
    #[doc = "I2C"]
    I2C = 0x01,
    #[doc = "DDR"]
    DDR = 0x02,
    _RESERVED_3 = 0x03,
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
