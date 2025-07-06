#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aasa {
    #[doc = "SETAASA not supported"]
    NOTSUPPORTED = 0x0,
    #[doc = "SETAASA supported"]
    SUPPORTED = 0x01,
}
impl Aasa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aasa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aasa {
    #[inline(always)]
    fn from(val: u8) -> Aasa {
        Aasa::from_bits(val)
    }
}
impl From<Aasa> for u8 {
    #[inline(always)]
    fn from(val: Aasa) -> u8 {
        Aasa::to_bits(val)
    }
}
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
pub enum Between {
    #[doc = "Inactive (for other cases)"]
    INACTIVE = 0x0,
    #[doc = "Active"]
    ACTIVE = 0x01,
}
impl Between {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Between {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Between {
    #[inline(always)]
    fn from(val: u8) -> Between {
        Between::from_bits(val)
    }
}
impl From<Between> for u8 {
    #[inline(always)]
    fn from(val: Between) -> u8 {
        Between::to_bits(val)
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
pub enum Ddrmatched {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Ddrmatched {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ddrmatched {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ddrmatched {
    #[inline(always)]
    fn from(val: u8) -> Ddrmatched {
        Ddrmatched::from_bits(val)
    }
}
impl From<Ddrmatched> for u8 {
    #[inline(always)]
    fn from(val: Ddrmatched) -> u8 {
        Ddrmatched::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ddrok {
    #[doc = "Do not allow HDR-DDR messaging"]
    DISABLE = 0x0,
    #[doc = "Allow HDR-DDR messaging"]
    ENABLE = 0x01,
}
impl Ddrok {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ddrok {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ddrok {
    #[inline(always)]
    fn from(val: u8) -> Ddrok {
        Ddrok::from_bits(val)
    }
}
impl From<Ddrok> for u8 {
    #[inline(always)]
    fn from(val: Ddrok) -> u8 {
        Ddrok::to_bits(val)
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
pub enum Ena {
    #[doc = "Disabled"]
    DISABLE = 0x0,
    #[doc = "Enabled"]
    ENABLE = 0x01,
}
impl Ena {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ena {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ena {
    #[inline(always)]
    fn from(val: u8) -> Ena {
        Ena::from_bits(val)
    }
}
impl From<Ena> for u8 {
    #[inline(always)]
    fn from(val: Ena) -> u8 {
        Ena::to_bits(val)
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
pub enum Extdata {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Extdata {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Extdata {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Extdata {
    #[inline(always)]
    fn from(val: u8) -> Extdata {
        Extdata::from_bits(val)
    }
}
impl From<Extdata> for u8 {
    #[inline(always)]
    fn from(val: Extdata) -> u8 {
        Extdata::to_bits(val)
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
pub enum Flushfb {
    #[doc = "No action"]
    NO_ACTION = 0x0,
    #[doc = "Flush the buffer"]
    FLUSH = 0x01,
}
impl Flushfb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flushfb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flushfb {
    #[inline(always)]
    fn from(val: u8) -> Flushfb {
        Flushfb::from_bits(val)
    }
}
impl From<Flushfb> for u8 {
    #[inline(always)]
    fn from(val: Flushfb) -> u8 {
        Flushfb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flushtb {
    #[doc = "No action"]
    NO_ACTION = 0x0,
    #[doc = "Flush the buffer"]
    FLUSH = 0x01,
}
impl Flushtb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flushtb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flushtb {
    #[inline(always)]
    fn from(val: u8) -> Flushtb {
        Flushtb::from_bits(val)
    }
}
impl From<Flushtb> for u8 {
    #[inline(always)]
    fn from(val: Flushtb) -> u8 {
        Flushtb::to_bits(val)
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
pub enum Hdrmatch {
    #[doc = "Did not match"]
    NO_MATCH = 0x0,
    #[doc = "Matched the I3C dynamic address"]
    MATCH = 0x01,
}
impl Hdrmatch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hdrmatch {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hdrmatch {
    #[inline(always)]
    fn from(val: u8) -> Hdrmatch {
        Hdrmatch::from_bits(val)
    }
}
impl From<Hdrmatch> for u8 {
    #[inline(always)]
    fn from(val: Hdrmatch) -> u8 {
        Hdrmatch::to_bits(val)
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
    #[doc = "PASSIVE_SDA"]
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
pub enum I2c10b {
    #[doc = "Not supported"]
    DISABLE = 0x0,
    #[doc = "Supported"]
    ENABLE = 0x01,
}
impl I2c10b {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I2c10b {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I2c10b {
    #[inline(always)]
    fn from(val: u8) -> I2c10b {
        I2c10b::from_bits(val)
    }
}
impl From<I2c10b> for u8 {
    #[inline(always)]
    fn from(val: I2c10b) -> u8 {
        I2c10b::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I2cdevid {
    #[doc = "Not supported"]
    DISABLE = 0x0,
    #[doc = "Supported"]
    ENABLE = 0x01,
}
impl I2cdevid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I2cdevid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I2cdevid {
    #[inline(always)]
    fn from(val: u8) -> I2cdevid {
        I2cdevid::from_bits(val)
    }
}
impl From<I2cdevid> for u8 {
    #[inline(always)]
    fn from(val: I2cdevid) -> u8 {
        I2cdevid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I2crst {
    #[doc = "Not supported"]
    DISABLE = 0x0,
    #[doc = "Supported"]
    ENABLE = 0x01,
}
impl I2crst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I2crst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I2crst {
    #[inline(always)]
    fn from(val: u8) -> I2crst {
        I2crst::from_bits(val)
    }
}
impl From<I2crst> for u8 {
    #[inline(always)]
    fn from(val: I2crst) -> u8 {
        I2crst::to_bits(val)
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
pub enum Ibiext {
    #[doc = "Not supported"]
    DISABLE = 0x0,
    #[doc = "Supported"]
    ENABLE = 0x01,
}
impl Ibiext {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ibiext {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ibiext {
    #[inline(always)]
    fn from(val: u8) -> Ibiext {
        Ibiext::from_bits(val)
    }
}
impl From<Ibiext> for u8 {
    #[inline(always)]
    fn from(val: Ibiext) -> u8 {
        Ibiext::to_bits(val)
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
pub enum Ibixreg {
    #[doc = "Not supported"]
    DISABLE = 0x0,
    #[doc = "Supported"]
    ENABLE = 0x01,
}
impl Ibixreg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ibixreg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ibixreg {
    #[inline(always)]
    fn from(val: u8) -> Ibixreg {
        Ibixreg::from_bits(val)
    }
}
impl From<Ibixreg> for u8 {
    #[inline(always)]
    fn from(val: Ibixreg) -> u8 {
        Ibixreg::to_bits(val)
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
pub enum Idrand {
    #[doc = "Part number and an instance"]
    DISABLE = 0x0,
    #[doc = "Random value"]
    ENABLE = 0x01,
}
impl Idrand {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Idrand {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Idrand {
    #[inline(always)]
    fn from(val: u8) -> Idrand {
        Idrand::from_bits(val)
    }
}
impl From<Idrand> for u8 {
    #[inline(always)]
    fn from(val: Idrand) -> u8 {
        Idrand::to_bits(val)
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
pub enum Invreq {
    #[doc = "No error"]
    NO_ERROR = 0x0,
    #[doc = "Error"]
    ERROR = 0x01,
}
impl Invreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Invreq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Invreq {
    #[inline(always)]
    fn from(val: u8) -> Invreq {
        Invreq::from_bits(val)
    }
}
impl From<Invreq> for u8 {
    #[inline(always)]
    fn from(val: Invreq) -> u8 {
        Invreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Invstart {
    #[doc = "No invalid start error"]
    NO_ERROR = 0x0,
    #[doc = "Invalid start error"]
    ERROR = 0x01,
}
impl Invstart {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Invstart {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Invstart {
    #[inline(always)]
    fn from(val: u8) -> Invstart {
        Invstart::from_bits(val)
    }
}
impl From<Invstart> for u8 {
    #[inline(always)]
    fn from(val: Invstart) -> u8 {
        Invstart::to_bits(val)
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
pub enum Matchss {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Matchss {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Matchss {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Matchss {
    #[inline(always)]
    fn from(val: u8) -> Matchss {
        Matchss::from_bits(val)
    }
}
impl From<Matchss> for u8 {
    #[inline(always)]
    fn from(val: Matchss) -> u8 {
        Matchss::to_bits(val)
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
pub enum MdatactrlRxempty {
    #[doc = "Not empty"]
    NOT_EMPTY = 0x0,
    #[doc = "Empty"]
    EMPTY = 0x01,
}
impl MdatactrlRxempty {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MdatactrlRxempty {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MdatactrlRxempty {
    #[inline(always)]
    fn from(val: u8) -> MdatactrlRxempty {
        MdatactrlRxempty::from_bits(val)
    }
}
impl From<MdatactrlRxempty> for u8 {
    #[inline(always)]
    fn from(val: MdatactrlRxempty) -> u8 {
        MdatactrlRxempty::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MdatactrlRxtrig {
    #[doc = "Trigger when not empty"]
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
pub enum MdatactrlTxfull {
    #[doc = "Not full"]
    NOT_FULL = 0x0,
    #[doc = "Full"]
    FULL = 0x01,
}
impl MdatactrlTxfull {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MdatactrlTxfull {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MdatactrlTxfull {
    #[inline(always)]
    fn from(val: u8) -> MdatactrlTxfull {
        MdatactrlTxfull::from_bits(val)
    }
}
impl From<MdatactrlTxfull> for u8 {
    #[inline(always)]
    fn from(val: MdatactrlTxfull) -> u8 {
        MdatactrlTxfull::to_bits(val)
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
pub enum MdatactrlUnlock {
    #[doc = "Locked"]
    DISABLED = 0x0,
    #[doc = "Unlocked"]
    ENABLED = 0x01,
}
impl MdatactrlUnlock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MdatactrlUnlock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MdatactrlUnlock {
    #[inline(always)]
    fn from(val: u8) -> MdatactrlUnlock {
        MdatactrlUnlock::from_bits(val)
    }
}
impl From<MdatactrlUnlock> for u8 {
    #[inline(always)]
    fn from(val: MdatactrlUnlock) -> u8 {
        MdatactrlUnlock::to_bits(val)
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
pub enum MdynaddrDavalid {
    #[doc = "No valid DA assigned"]
    NO_VALID = 0x0,
    #[doc = "Valid DA assigned"]
    VALID = 0x01,
}
impl MdynaddrDavalid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MdynaddrDavalid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MdynaddrDavalid {
    #[inline(always)]
    fn from(val: u8) -> MdynaddrDavalid {
        MdynaddrDavalid::from_bits(val)
    }
}
impl From<MdynaddrDavalid> for u8 {
    #[inline(always)]
    fn from(val: MdynaddrDavalid) -> u8 {
        MdynaddrDavalid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MerrwarnHcrc {
    #[doc = "No error"]
    NO_ERROR = 0x0,
    #[doc = "Error"]
    ERROR = 0x01,
}
impl MerrwarnHcrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MerrwarnHcrc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MerrwarnHcrc {
    #[inline(always)]
    fn from(val: u8) -> MerrwarnHcrc {
        MerrwarnHcrc::from_bits(val)
    }
}
impl From<MerrwarnHcrc> for u8 {
    #[inline(always)]
    fn from(val: MerrwarnHcrc) -> u8 {
        MerrwarnHcrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MerrwarnHpar {
    #[doc = "No error"]
    NO_ERROR = 0x0,
    #[doc = "Error"]
    ERROR = 0x01,
}
impl MerrwarnHpar {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MerrwarnHpar {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MerrwarnHpar {
    #[inline(always)]
    fn from(val: u8) -> MerrwarnHpar {
        MerrwarnHpar::from_bits(val)
    }
}
impl From<MerrwarnHpar> for u8 {
    #[inline(always)]
    fn from(val: MerrwarnHpar) -> u8 {
        MerrwarnHpar::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MerrwarnNack {
    #[doc = "No error"]
    NO_ERROR = 0x0,
    #[doc = "Error"]
    ERROR = 0x01,
}
impl MerrwarnNack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MerrwarnNack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MerrwarnNack {
    #[inline(always)]
    fn from(val: u8) -> MerrwarnNack {
        MerrwarnNack::from_bits(val)
    }
}
impl From<MerrwarnNack> for u8 {
    #[inline(always)]
    fn from(val: MerrwarnNack) -> u8 {
        MerrwarnNack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MerrwarnOread {
    #[doc = "No error"]
    NO_ERROR = 0x0,
    #[doc = "Error"]
    ERROR = 0x01,
}
impl MerrwarnOread {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MerrwarnOread {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MerrwarnOread {
    #[inline(always)]
    fn from(val: u8) -> MerrwarnOread {
        MerrwarnOread::from_bits(val)
    }
}
impl From<MerrwarnOread> for u8 {
    #[inline(always)]
    fn from(val: MerrwarnOread) -> u8 {
        MerrwarnOread::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MerrwarnOwrite {
    #[doc = "No error"]
    NO_ERROR = 0x0,
    #[doc = "Error"]
    ERROR = 0x01,
}
impl MerrwarnOwrite {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MerrwarnOwrite {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MerrwarnOwrite {
    #[inline(always)]
    fn from(val: u8) -> MerrwarnOwrite {
        MerrwarnOwrite::from_bits(val)
    }
}
impl From<MerrwarnOwrite> for u8 {
    #[inline(always)]
    fn from(val: MerrwarnOwrite) -> u8 {
        MerrwarnOwrite::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MintclrComplete {
    #[doc = "No effect"]
    NONE = 0x0,
    #[doc = "Interrupt enable cleared"]
    CLEAR = 0x01,
}
impl MintclrComplete {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MintclrComplete {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MintclrComplete {
    #[inline(always)]
    fn from(val: u8) -> MintclrComplete {
        MintclrComplete::from_bits(val)
    }
}
impl From<MintclrComplete> for u8 {
    #[inline(always)]
    fn from(val: MintclrComplete) -> u8 {
        MintclrComplete::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MintclrErrwarn {
    #[doc = "No effect"]
    NONE = 0x0,
    #[doc = "Interrupt enable cleared"]
    CLEAR = 0x01,
}
impl MintclrErrwarn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MintclrErrwarn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MintclrErrwarn {
    #[inline(always)]
    fn from(val: u8) -> MintclrErrwarn {
        MintclrErrwarn::from_bits(val)
    }
}
impl From<MintclrErrwarn> for u8 {
    #[inline(always)]
    fn from(val: MintclrErrwarn) -> u8 {
        MintclrErrwarn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MintclrIbiwon {
    #[doc = "No effect"]
    NONE = 0x0,
    #[doc = "Interrupt enable cleared"]
    CLEAR = 0x01,
}
impl MintclrIbiwon {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MintclrIbiwon {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MintclrIbiwon {
    #[inline(always)]
    fn from(val: u8) -> MintclrIbiwon {
        MintclrIbiwon::from_bits(val)
    }
}
impl From<MintclrIbiwon> for u8 {
    #[inline(always)]
    fn from(val: MintclrIbiwon) -> u8 {
        MintclrIbiwon::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MintclrMctrldone {
    #[doc = "No effect"]
    NONE = 0x0,
    #[doc = "Interrupt enable cleared"]
    CLEAR = 0x01,
}
impl MintclrMctrldone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MintclrMctrldone {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MintclrMctrldone {
    #[inline(always)]
    fn from(val: u8) -> MintclrMctrldone {
        MintclrMctrldone::from_bits(val)
    }
}
impl From<MintclrMctrldone> for u8 {
    #[inline(always)]
    fn from(val: MintclrMctrldone) -> u8 {
        MintclrMctrldone::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MintclrNowmaster {
    #[doc = "No effect"]
    NONE = 0x0,
    #[doc = "Interrupt enable cleared"]
    CLEAR = 0x01,
}
impl MintclrNowmaster {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MintclrNowmaster {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MintclrNowmaster {
    #[inline(always)]
    fn from(val: u8) -> MintclrNowmaster {
        MintclrNowmaster::from_bits(val)
    }
}
impl From<MintclrNowmaster> for u8 {
    #[inline(always)]
    fn from(val: MintclrNowmaster) -> u8 {
        MintclrNowmaster::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MintclrRxpend {
    #[doc = "No effect"]
    NONE = 0x0,
    #[doc = "Interrupt enable cleared"]
    CLEAR = 0x01,
}
impl MintclrRxpend {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MintclrRxpend {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MintclrRxpend {
    #[inline(always)]
    fn from(val: u8) -> MintclrRxpend {
        MintclrRxpend::from_bits(val)
    }
}
impl From<MintclrRxpend> for u8 {
    #[inline(always)]
    fn from(val: MintclrRxpend) -> u8 {
        MintclrRxpend::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MintclrSlvstart {
    #[doc = "No effect"]
    NONE = 0x0,
    #[doc = "Interrupt enable cleared"]
    CLEAR = 0x01,
}
impl MintclrSlvstart {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MintclrSlvstart {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MintclrSlvstart {
    #[inline(always)]
    fn from(val: u8) -> MintclrSlvstart {
        MintclrSlvstart::from_bits(val)
    }
}
impl From<MintclrSlvstart> for u8 {
    #[inline(always)]
    fn from(val: MintclrSlvstart) -> u8 {
        MintclrSlvstart::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MintclrTxnotfull {
    #[doc = "No effect"]
    NONE = 0x0,
    #[doc = "Interrupt enable cleared"]
    CLEAR = 0x01,
}
impl MintclrTxnotfull {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MintclrTxnotfull {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MintclrTxnotfull {
    #[inline(always)]
    fn from(val: u8) -> MintclrTxnotfull {
        MintclrTxnotfull::from_bits(val)
    }
}
impl From<MintclrTxnotfull> for u8 {
    #[inline(always)]
    fn from(val: MintclrTxnotfull) -> u8 {
        MintclrTxnotfull::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MintmaskedComplete {
    #[doc = "Disabled"]
    NOT_ENABLED = 0x0,
    #[doc = "Enabled"]
    ENABLED = 0x01,
}
impl MintmaskedComplete {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MintmaskedComplete {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MintmaskedComplete {
    #[inline(always)]
    fn from(val: u8) -> MintmaskedComplete {
        MintmaskedComplete::from_bits(val)
    }
}
impl From<MintmaskedComplete> for u8 {
    #[inline(always)]
    fn from(val: MintmaskedComplete) -> u8 {
        MintmaskedComplete::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MintmaskedErrwarn {
    #[doc = "Disabled"]
    NOT_ENABLED = 0x0,
    #[doc = "Enabled"]
    ENABLED = 0x01,
}
impl MintmaskedErrwarn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MintmaskedErrwarn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MintmaskedErrwarn {
    #[inline(always)]
    fn from(val: u8) -> MintmaskedErrwarn {
        MintmaskedErrwarn::from_bits(val)
    }
}
impl From<MintmaskedErrwarn> for u8 {
    #[inline(always)]
    fn from(val: MintmaskedErrwarn) -> u8 {
        MintmaskedErrwarn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MintmaskedIbiwon {
    #[doc = "Disabled"]
    NOT_ENABLED = 0x0,
    #[doc = "Enabled"]
    ENABLED = 0x01,
}
impl MintmaskedIbiwon {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MintmaskedIbiwon {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MintmaskedIbiwon {
    #[inline(always)]
    fn from(val: u8) -> MintmaskedIbiwon {
        MintmaskedIbiwon::from_bits(val)
    }
}
impl From<MintmaskedIbiwon> for u8 {
    #[inline(always)]
    fn from(val: MintmaskedIbiwon) -> u8 {
        MintmaskedIbiwon::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MintmaskedMctrldone {
    #[doc = "Disabled"]
    NOT_ENABLED = 0x0,
    #[doc = "Enabled"]
    ENABLED = 0x01,
}
impl MintmaskedMctrldone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MintmaskedMctrldone {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MintmaskedMctrldone {
    #[inline(always)]
    fn from(val: u8) -> MintmaskedMctrldone {
        MintmaskedMctrldone::from_bits(val)
    }
}
impl From<MintmaskedMctrldone> for u8 {
    #[inline(always)]
    fn from(val: MintmaskedMctrldone) -> u8 {
        MintmaskedMctrldone::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MintmaskedNowmaster {
    #[doc = "Disabled"]
    NOT_ENABLED = 0x0,
    #[doc = "Enabled"]
    ENABLED = 0x01,
}
impl MintmaskedNowmaster {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MintmaskedNowmaster {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MintmaskedNowmaster {
    #[inline(always)]
    fn from(val: u8) -> MintmaskedNowmaster {
        MintmaskedNowmaster::from_bits(val)
    }
}
impl From<MintmaskedNowmaster> for u8 {
    #[inline(always)]
    fn from(val: MintmaskedNowmaster) -> u8 {
        MintmaskedNowmaster::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MintmaskedSlvstart {
    #[doc = "Disabled"]
    NOT_ENABLED = 0x0,
    #[doc = "Enabled"]
    ENABLED = 0x01,
}
impl MintmaskedSlvstart {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MintmaskedSlvstart {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MintmaskedSlvstart {
    #[inline(always)]
    fn from(val: u8) -> MintmaskedSlvstart {
        MintmaskedSlvstart::from_bits(val)
    }
}
impl From<MintmaskedSlvstart> for u8 {
    #[inline(always)]
    fn from(val: MintmaskedSlvstart) -> u8 {
        MintmaskedSlvstart::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MintmaskedTxnotfull {
    #[doc = "Disabled"]
    NOT_ENABLED = 0x0,
    #[doc = "Enabled"]
    ENABLED = 0x01,
}
impl MintmaskedTxnotfull {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MintmaskedTxnotfull {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MintmaskedTxnotfull {
    #[inline(always)]
    fn from(val: u8) -> MintmaskedTxnotfull {
        MintmaskedTxnotfull::from_bits(val)
    }
}
impl From<MintmaskedTxnotfull> for u8 {
    #[inline(always)]
    fn from(val: MintmaskedTxnotfull) -> u8 {
        MintmaskedTxnotfull::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MintsetComplete {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl MintsetComplete {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MintsetComplete {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MintsetComplete {
    #[inline(always)]
    fn from(val: u8) -> MintsetComplete {
        MintsetComplete::from_bits(val)
    }
}
impl From<MintsetComplete> for u8 {
    #[inline(always)]
    fn from(val: MintsetComplete) -> u8 {
        MintsetComplete::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MintsetErrwarn {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl MintsetErrwarn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MintsetErrwarn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MintsetErrwarn {
    #[inline(always)]
    fn from(val: u8) -> MintsetErrwarn {
        MintsetErrwarn::from_bits(val)
    }
}
impl From<MintsetErrwarn> for u8 {
    #[inline(always)]
    fn from(val: MintsetErrwarn) -> u8 {
        MintsetErrwarn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MintsetIbiwon {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl MintsetIbiwon {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MintsetIbiwon {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MintsetIbiwon {
    #[inline(always)]
    fn from(val: u8) -> MintsetIbiwon {
        MintsetIbiwon::from_bits(val)
    }
}
impl From<MintsetIbiwon> for u8 {
    #[inline(always)]
    fn from(val: MintsetIbiwon) -> u8 {
        MintsetIbiwon::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MintsetMctrldone {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl MintsetMctrldone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MintsetMctrldone {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MintsetMctrldone {
    #[inline(always)]
    fn from(val: u8) -> MintsetMctrldone {
        MintsetMctrldone::from_bits(val)
    }
}
impl From<MintsetMctrldone> for u8 {
    #[inline(always)]
    fn from(val: MintsetMctrldone) -> u8 {
        MintsetMctrldone::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MintsetNowmaster {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl MintsetNowmaster {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MintsetNowmaster {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MintsetNowmaster {
    #[inline(always)]
    fn from(val: u8) -> MintsetNowmaster {
        MintsetNowmaster::from_bits(val)
    }
}
impl From<MintsetNowmaster> for u8 {
    #[inline(always)]
    fn from(val: MintsetNowmaster) -> u8 {
        MintsetNowmaster::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MintsetSlvstart {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl MintsetSlvstart {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MintsetSlvstart {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MintsetSlvstart {
    #[inline(always)]
    fn from(val: u8) -> MintsetSlvstart {
        MintsetSlvstart::from_bits(val)
    }
}
impl From<MintsetSlvstart> for u8 {
    #[inline(always)]
    fn from(val: MintsetSlvstart) -> u8 {
        MintsetSlvstart::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MintsetTxnotfull {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl MintsetTxnotfull {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MintsetTxnotfull {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MintsetTxnotfull {
    #[inline(always)]
    fn from(val: u8) -> MintsetTxnotfull {
        MintsetTxnotfull::from_bits(val)
    }
}
impl From<MintsetTxnotfull> for u8 {
    #[inline(always)]
    fn from(val: MintsetTxnotfull) -> u8 {
        MintsetTxnotfull::to_bits(val)
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
pub enum Msb0 {
    #[doc = "MSB is not 0"]
    DISABLE = 0x0,
    #[doc = "MSB is 0"]
    ENABLE = 0x01,
}
impl Msb0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Msb0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Msb0 {
    #[inline(always)]
    fn from(val: u8) -> Msb0 {
        Msb0::from_bits(val)
    }
}
impl From<Msb0> for u8 {
    #[inline(always)]
    fn from(val: Msb0) -> u8 {
        Msb0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Msgerr {
    #[doc = "No error"]
    NO_ERROR = 0x0,
    #[doc = "Error"]
    ERROR = 0x01,
}
impl Msgerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Msgerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Msgerr {
    #[inline(always)]
    fn from(val: u8) -> Msgerr {
        Msgerr::from_bits(val)
    }
}
impl From<Msgerr> for u8 {
    #[inline(always)]
    fn from(val: Msgerr) -> u8 {
        Msgerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MstatusComplete {
    #[doc = "Not complete"]
    NOT_COMPLETE = 0x0,
    #[doc = "Complete"]
    COMPLETE = 0x01,
}
impl MstatusComplete {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MstatusComplete {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MstatusComplete {
    #[inline(always)]
    fn from(val: u8) -> MstatusComplete {
        MstatusComplete::from_bits(val)
    }
}
impl From<MstatusComplete> for u8 {
    #[inline(always)]
    fn from(val: MstatusComplete) -> u8 {
        MstatusComplete::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MstatusErrwarn {
    #[doc = "No error or warning"]
    NO_ERROR = 0x0,
    #[doc = "Error or warning"]
    ERROR = 0x01,
}
impl MstatusErrwarn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MstatusErrwarn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MstatusErrwarn {
    #[inline(always)]
    fn from(val: u8) -> MstatusErrwarn {
        MstatusErrwarn::from_bits(val)
    }
}
impl From<MstatusErrwarn> for u8 {
    #[inline(always)]
    fn from(val: MstatusErrwarn) -> u8 {
        MstatusErrwarn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MstatusIbiwon {
    #[doc = "No IBI arbitration won"]
    NOT_WON = 0x0,
    #[doc = "IBI arbitration won"]
    WON = 0x01,
}
impl MstatusIbiwon {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MstatusIbiwon {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MstatusIbiwon {
    #[inline(always)]
    fn from(val: u8) -> MstatusIbiwon {
        MstatusIbiwon::from_bits(val)
    }
}
impl From<MstatusIbiwon> for u8 {
    #[inline(always)]
    fn from(val: MstatusIbiwon) -> u8 {
        MstatusIbiwon::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MstatusMctrldone {
    #[doc = "Not done"]
    NOT_DONE = 0x0,
    #[doc = "Done"]
    DONE = 0x01,
}
impl MstatusMctrldone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MstatusMctrldone {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MstatusMctrldone {
    #[inline(always)]
    fn from(val: u8) -> MstatusMctrldone {
        MstatusMctrldone::from_bits(val)
    }
}
impl From<MstatusMctrldone> for u8 {
    #[inline(always)]
    fn from(val: MstatusMctrldone) -> u8 {
        MstatusMctrldone::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MstatusNowmaster {
    #[doc = "Not a controller"]
    NOT_MASTER = 0x0,
    #[doc = "Controller"]
    MASTER = 0x01,
}
impl MstatusNowmaster {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MstatusNowmaster {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MstatusNowmaster {
    #[inline(always)]
    fn from(val: u8) -> MstatusNowmaster {
        MstatusNowmaster::from_bits(val)
    }
}
impl From<MstatusNowmaster> for u8 {
    #[inline(always)]
    fn from(val: MstatusNowmaster) -> u8 {
        MstatusNowmaster::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MstatusRxpend {
    #[doc = "No receive message pending"]
    IDLE = 0x0,
    #[doc = "Receive message pending"]
    PENDING = 0x01,
}
impl MstatusRxpend {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MstatusRxpend {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MstatusRxpend {
    #[inline(always)]
    fn from(val: u8) -> MstatusRxpend {
        MstatusRxpend::from_bits(val)
    }
}
impl From<MstatusRxpend> for u8 {
    #[inline(always)]
    fn from(val: MstatusRxpend) -> u8 {
        MstatusRxpend::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MstatusSlvstart {
    #[doc = "Target not requesting START"]
    NOT_START = 0x0,
    #[doc = "Target requesting START"]
    START = 0x01,
}
impl MstatusSlvstart {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MstatusSlvstart {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MstatusSlvstart {
    #[inline(always)]
    fn from(val: u8) -> MstatusSlvstart {
        MstatusSlvstart::from_bits(val)
    }
}
impl From<MstatusSlvstart> for u8 {
    #[inline(always)]
    fn from(val: MstatusSlvstart) -> u8 {
        MstatusSlvstart::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MstatusTxnotfull {
    #[doc = "Receive buffer or FIFO full"]
    FULL = 0x0,
    #[doc = "Receive buffer or FIFO not full"]
    NOTFULL = 0x01,
}
impl MstatusTxnotfull {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MstatusTxnotfull {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MstatusTxnotfull {
    #[inline(always)]
    fn from(val: u8) -> MstatusTxnotfull {
        MstatusTxnotfull::from_bits(val)
    }
}
impl From<MstatusTxnotfull> for u8 {
    #[inline(always)]
    fn from(val: MstatusTxnotfull) -> u8 {
        MstatusTxnotfull::to_bits(val)
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
pub enum MwdatabEnd {
    #[doc = "Not the end"]
    NOT_END = 0x0,
    #[doc = "End"]
    END = 0x01,
}
impl MwdatabEnd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MwdatabEnd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MwdatabEnd {
    #[inline(always)]
    fn from(val: u8) -> MwdatabEnd {
        MwdatabEnd::from_bits(val)
    }
}
impl From<MwdatabEnd> for u8 {
    #[inline(always)]
    fn from(val: MwdatabEnd) -> u8 {
        MwdatabEnd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MwdatabEndAlso {
    #[doc = "Not the end"]
    NOT_END = 0x0,
    #[doc = "End"]
    END = 0x01,
}
impl MwdatabEndAlso {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MwdatabEndAlso {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MwdatabEndAlso {
    #[inline(always)]
    fn from(val: u8) -> MwdatabEndAlso {
        MwdatabEndAlso::from_bits(val)
    }
}
impl From<MwdatabEndAlso> for u8 {
    #[inline(always)]
    fn from(val: MwdatabEndAlso) -> u8 {
        MwdatabEndAlso::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MwdatahEnd {
    #[doc = "Not the end"]
    NOT_END = 0x0,
    #[doc = "End"]
    END = 0x01,
}
impl MwdatahEnd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MwdatahEnd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MwdatahEnd {
    #[inline(always)]
    fn from(val: u8) -> MwdatahEnd {
        MwdatahEnd::from_bits(val)
    }
}
impl From<MwdatahEnd> for u8 {
    #[inline(always)]
    fn from(val: MwdatahEnd) -> u8 {
        MwdatahEnd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MwmsgDdrControl2End {
    #[doc = "Not the end"]
    NOT_END = 0x0,
    #[doc = "End"]
    END = 0x01,
}
impl MwmsgDdrControl2End {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MwmsgDdrControl2End {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MwmsgDdrControl2End {
    #[inline(always)]
    fn from(val: u8) -> MwmsgDdrControl2End {
        MwmsgDdrControl2End::from_bits(val)
    }
}
impl From<MwmsgDdrControl2End> for u8 {
    #[inline(always)]
    fn from(val: MwmsgDdrControl2End) -> u8 {
        MwmsgDdrControl2End::to_bits(val)
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
pub enum MwmsgSdrControlEnd {
    #[doc = "Not the end"]
    NOT_END = 0x0,
    #[doc = "End"]
    END = 0x01,
}
impl MwmsgSdrControlEnd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MwmsgSdrControlEnd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MwmsgSdrControlEnd {
    #[inline(always)]
    fn from(val: u8) -> MwmsgSdrControlEnd {
        MwmsgSdrControlEnd::from_bits(val)
    }
}
impl From<MwmsgSdrControlEnd> for u8 {
    #[inline(always)]
    fn from(val: MwmsgSdrControlEnd) -> u8 {
        MwmsgSdrControlEnd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nacked {
    #[doc = "Not NACKed"]
    NOT_NACKED = 0x0,
    #[doc = "NACKed (not acknowledged)"]
    NACKED = 0x01,
}
impl Nacked {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nacked {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nacked {
    #[inline(always)]
    fn from(val: u8) -> Nacked {
        Nacked::from_bits(val)
    }
}
impl From<Nacked> for u8 {
    #[inline(always)]
    fn from(val: Nacked) -> u8 {
        Nacked::to_bits(val)
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
pub enum Odhpp {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Odhpp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Odhpp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Odhpp {
    #[inline(always)]
    fn from(val: u8) -> Odhpp {
        Odhpp::from_bits(val)
    }
}
impl From<Odhpp> for u8 {
    #[inline(always)]
    fn from(val: Odhpp) -> u8 {
        Odhpp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Odstop {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Odstop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Odstop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Odstop {
    #[inline(always)]
    fn from(val: u8) -> Odstop {
        Odstop::from_bits(val)
    }
}
impl From<Odstop> for u8 {
    #[inline(always)]
    fn from(val: Odstop) -> u8 {
        Odstop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Offline {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Offline {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Offline {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Offline {
    #[inline(always)]
    fn from(val: u8) -> Offline {
        Offline::from_bits(val)
    }
}
impl From<Offline> for u8 {
    #[inline(always)]
    fn from(val: Offline) -> u8 {
        Offline::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Orun {
    #[doc = "No overrun error"]
    NO_ERROR = 0x0,
    #[doc = "Overrun error"]
    ERROR = 0x01,
}
impl Orun {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Orun {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Orun {
    #[inline(always)]
    fn from(val: u8) -> Orun {
        Orun::from_bits(val)
    }
}
impl From<Orun> for u8 {
    #[inline(always)]
    fn from(val: Orun) -> u8 {
        Orun::to_bits(val)
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
pub enum RxPend {
    #[doc = "No received message pending"]
    NO_MSG_PENDING = 0x0,
    #[doc = "Received message pending"]
    MSG_PENDING = 0x01,
}
impl RxPend {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxPend {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxPend {
    #[inline(always)]
    fn from(val: u8) -> RxPend {
        RxPend::from_bits(val)
    }
}
impl From<RxPend> for u8 {
    #[inline(always)]
    fn from(val: RxPend) -> u8 {
        RxPend::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum S0ignore {
    #[doc = "Do not ignore TE0 or TE1 errors"]
    DISABLE = 0x0,
    #[doc = "Ignore TE0 or TE1 errors"]
    ENABLE = 0x01,
}
impl S0ignore {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> S0ignore {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for S0ignore {
    #[inline(always)]
    fn from(val: u8) -> S0ignore {
        S0ignore::from_bits(val)
    }
}
impl From<S0ignore> for u8 {
    #[inline(always)]
    fn from(val: S0ignore) -> u8 {
        S0ignore::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum S0s1 {
    #[doc = "No TE0 or TE1 error occurred"]
    NO_ERROR = 0x0,
    #[doc = "TE0 or TE1 error occurred"]
    ERROR = 0x01,
}
impl S0s1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> S0s1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for S0s1 {
    #[inline(always)]
    fn from(val: u8) -> S0s1 {
        S0s1::from_bits(val)
    }
}
impl From<S0s1> for u8 {
    #[inline(always)]
    fn from(val: S0s1) -> u8 {
        S0s1::to_bits(val)
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
pub enum SconfigNack {
    #[doc = "Always disable NACK mode"]
    DISABLE = 0x0,
    #[doc = "Always enable NACK mode (works normally)"]
    ENABLE = 0x01,
}
impl SconfigNack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SconfigNack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SconfigNack {
    #[inline(always)]
    fn from(val: u8) -> SconfigNack {
        SconfigNack::from_bits(val)
    }
}
impl From<SconfigNack> for u8 {
    #[inline(always)]
    fn from(val: SconfigNack) -> u8 {
        SconfigNack::to_bits(val)
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
    #[doc = "Trigger when not empty"]
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
pub enum SdatactrlUnlock {
    #[doc = "Cannot be changed"]
    DISABLED = 0x0,
    #[doc = "Can be changed"]
    ENABLED = 0x01,
}
impl SdatactrlUnlock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SdatactrlUnlock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SdatactrlUnlock {
    #[inline(always)]
    fn from(val: u8) -> SdatactrlUnlock {
        SdatactrlUnlock::from_bits(val)
    }
}
impl From<SdatactrlUnlock> for u8 {
    #[inline(always)]
    fn from(val: SdatactrlUnlock) -> u8 {
        SdatactrlUnlock::to_bits(val)
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
pub enum SerrwarnHcrc {
    #[doc = "No HDR-DDR CRC error occurred"]
    NO_ERROR = 0x0,
    #[doc = "HDR-DDR CRC error occurred"]
    ERROR = 0x01,
}
impl SerrwarnHcrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SerrwarnHcrc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SerrwarnHcrc {
    #[inline(always)]
    fn from(val: u8) -> SerrwarnHcrc {
        SerrwarnHcrc::from_bits(val)
    }
}
impl From<SerrwarnHcrc> for u8 {
    #[inline(always)]
    fn from(val: SerrwarnHcrc) -> u8 {
        SerrwarnHcrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SerrwarnHpar {
    #[doc = "No HDR parity error"]
    NO_ERROR = 0x0,
    #[doc = "HDR parity error"]
    ERROR = 0x01,
}
impl SerrwarnHpar {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SerrwarnHpar {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SerrwarnHpar {
    #[inline(always)]
    fn from(val: u8) -> SerrwarnHpar {
        SerrwarnHpar::from_bits(val)
    }
}
impl From<SerrwarnHpar> for u8 {
    #[inline(always)]
    fn from(val: SerrwarnHpar) -> u8 {
        SerrwarnHpar::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SerrwarnOread {
    #[doc = "No over-read error"]
    NO_ERROR = 0x0,
    #[doc = "Over-read error"]
    ERROR = 0x01,
}
impl SerrwarnOread {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SerrwarnOread {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SerrwarnOread {
    #[inline(always)]
    fn from(val: u8) -> SerrwarnOread {
        SerrwarnOread::from_bits(val)
    }
}
impl From<SerrwarnOread> for u8 {
    #[inline(always)]
    fn from(val: SerrwarnOread) -> u8 {
        SerrwarnOread::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SerrwarnOwrite {
    #[doc = "No overwrite error"]
    NO_ERROR = 0x0,
    #[doc = "Overwrite error"]
    ERROR = 0x01,
}
impl SerrwarnOwrite {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SerrwarnOwrite {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SerrwarnOwrite {
    #[inline(always)]
    fn from(val: u8) -> SerrwarnOwrite {
        SerrwarnOwrite::from_bits(val)
    }
}
impl From<SerrwarnOwrite> for u8 {
    #[inline(always)]
    fn from(val: SerrwarnOwrite) -> u8 {
        SerrwarnOwrite::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SintsetCcc {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl SintsetCcc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SintsetCcc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SintsetCcc {
    #[inline(always)]
    fn from(val: u8) -> SintsetCcc {
        SintsetCcc::from_bits(val)
    }
}
impl From<SintsetCcc> for u8 {
    #[inline(always)]
    fn from(val: SintsetCcc) -> u8 {
        SintsetCcc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SintsetChandled {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl SintsetChandled {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SintsetChandled {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SintsetChandled {
    #[inline(always)]
    fn from(val: u8) -> SintsetChandled {
        SintsetChandled::from_bits(val)
    }
}
impl From<SintsetChandled> for u8 {
    #[inline(always)]
    fn from(val: SintsetChandled) -> u8 {
        SintsetChandled::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SintsetDachg {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl SintsetDachg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SintsetDachg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SintsetDachg {
    #[inline(always)]
    fn from(val: u8) -> SintsetDachg {
        SintsetDachg::from_bits(val)
    }
}
impl From<SintsetDachg> for u8 {
    #[inline(always)]
    fn from(val: SintsetDachg) -> u8 {
        SintsetDachg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SintsetErrwarn {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl SintsetErrwarn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SintsetErrwarn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SintsetErrwarn {
    #[inline(always)]
    fn from(val: u8) -> SintsetErrwarn {
        SintsetErrwarn::from_bits(val)
    }
}
impl From<SintsetErrwarn> for u8 {
    #[inline(always)]
    fn from(val: SintsetErrwarn) -> u8 {
        SintsetErrwarn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SintsetEvent {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl SintsetEvent {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SintsetEvent {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SintsetEvent {
    #[inline(always)]
    fn from(val: u8) -> SintsetEvent {
        SintsetEvent::from_bits(val)
    }
}
impl From<SintsetEvent> for u8 {
    #[inline(always)]
    fn from(val: SintsetEvent) -> u8 {
        SintsetEvent::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SintsetMatched {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl SintsetMatched {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SintsetMatched {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SintsetMatched {
    #[inline(always)]
    fn from(val: u8) -> SintsetMatched {
        SintsetMatched::from_bits(val)
    }
}
impl From<SintsetMatched> for u8 {
    #[inline(always)]
    fn from(val: SintsetMatched) -> u8 {
        SintsetMatched::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SintsetRxpend {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl SintsetRxpend {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SintsetRxpend {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SintsetRxpend {
    #[inline(always)]
    fn from(val: u8) -> SintsetRxpend {
        SintsetRxpend::from_bits(val)
    }
}
impl From<SintsetRxpend> for u8 {
    #[inline(always)]
    fn from(val: SintsetRxpend) -> u8 {
        SintsetRxpend::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SintsetStart {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl SintsetStart {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SintsetStart {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SintsetStart {
    #[inline(always)]
    fn from(val: u8) -> SintsetStart {
        SintsetStart::from_bits(val)
    }
}
impl From<SintsetStart> for u8 {
    #[inline(always)]
    fn from(val: SintsetStart) -> u8 {
        SintsetStart::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SintsetStop {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl SintsetStop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SintsetStop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SintsetStop {
    #[inline(always)]
    fn from(val: u8) -> SintsetStop {
        SintsetStop::from_bits(val)
    }
}
impl From<SintsetStop> for u8 {
    #[inline(always)]
    fn from(val: SintsetStop) -> u8 {
        SintsetStop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Slvena {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Slvena {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Slvena {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Slvena {
    #[inline(always)]
    fn from(val: u8) -> Slvena {
        Slvena::from_bits(val)
    }
}
impl From<Slvena> for u8 {
    #[inline(always)]
    fn from(val: Slvena) -> u8 {
        Slvena::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Slvrst {
    #[doc = "Not supported"]
    DISABLE = 0x0,
    #[doc = "Supported"]
    ENABLE = 0x01,
}
impl Slvrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Slvrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Slvrst {
    #[inline(always)]
    fn from(val: u8) -> Slvrst {
        Slvrst::from_bits(val)
    }
}
impl From<Slvrst> for u8 {
    #[inline(always)]
    fn from(val: Slvrst) -> u8 {
        Slvrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spar {
    #[doc = "No SDR parity error"]
    NO_ERROR = 0x0,
    #[doc = "SDR parity error"]
    ERROR = 0x01,
}
impl Spar {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spar {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spar {
    #[inline(always)]
    fn from(val: u8) -> Spar {
        Spar::from_bits(val)
    }
}
impl From<Spar> for u8 {
    #[inline(always)]
    fn from(val: Spar) -> u8 {
        Spar::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SstatusCcc {
    #[doc = "CCC not received"]
    NO_CCC_RECEIVED = 0x0,
    #[doc = "CCC received"]
    CCC_RECEIVED = 0x01,
}
impl SstatusCcc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SstatusCcc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SstatusCcc {
    #[inline(always)]
    fn from(val: u8) -> SstatusCcc {
        SstatusCcc::from_bits(val)
    }
}
impl From<SstatusCcc> for u8 {
    #[inline(always)]
    fn from(val: SstatusCcc) -> u8 {
        SstatusCcc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SstatusChandled {
    #[doc = "CCC handling not in progress"]
    NOT_HANDLED = 0x0,
    #[doc = "CCC handling in progress"]
    HANDLED = 0x01,
}
impl SstatusChandled {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SstatusChandled {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SstatusChandled {
    #[inline(always)]
    fn from(val: u8) -> SstatusChandled {
        SstatusChandled::from_bits(val)
    }
}
impl From<SstatusChandled> for u8 {
    #[inline(always)]
    fn from(val: SstatusChandled) -> u8 {
        SstatusChandled::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SstatusDachg {
    #[doc = "No DA change detected"]
    NO_CHANGE_DETECTED = 0x0,
    #[doc = "DA change detected"]
    CHANGE_DETECTED = 0x01,
}
impl SstatusDachg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SstatusDachg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SstatusDachg {
    #[inline(always)]
    fn from(val: u8) -> SstatusDachg {
        SstatusDachg::from_bits(val)
    }
}
impl From<SstatusDachg> for u8 {
    #[inline(always)]
    fn from(val: SstatusDachg) -> u8 {
        SstatusDachg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SstatusEvent {
    #[doc = "No event occurred"]
    NO_EVENT = 0x0,
    #[doc = "IBI, CR, or HJ occurred"]
    EVENT = 0x01,
}
impl SstatusEvent {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SstatusEvent {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SstatusEvent {
    #[inline(always)]
    fn from(val: u8) -> SstatusEvent {
        SstatusEvent::from_bits(val)
    }
}
impl From<SstatusEvent> for u8 {
    #[inline(always)]
    fn from(val: SstatusEvent) -> u8 {
        SstatusEvent::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SstatusMatched {
    #[doc = "Header not matched"]
    NOT_MATCHED = 0x0,
    #[doc = "Header matched"]
    MATCHED = 0x01,
}
impl SstatusMatched {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SstatusMatched {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SstatusMatched {
    #[inline(always)]
    fn from(val: u8) -> SstatusMatched {
        SstatusMatched::from_bits(val)
    }
}
impl From<SstatusMatched> for u8 {
    #[inline(always)]
    fn from(val: SstatusMatched) -> u8 {
        SstatusMatched::to_bits(val)
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
pub enum SstatusStop {
    #[doc = "No Stopped state detected"]
    NO_STOP_DETECTED = 0x0,
    #[doc = "Stopped state detected"]
    STOP_DETECTED = 0x01,
}
impl SstatusStop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SstatusStop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SstatusStop {
    #[inline(always)]
    fn from(val: u8) -> SstatusStop {
        SstatusStop::from_bits(val)
    }
}
impl From<SstatusStop> for u8 {
    #[inline(always)]
    fn from(val: SstatusStop) -> u8 {
        SstatusStop::to_bits(val)
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
pub enum Sstsub {
    #[doc = "Not subscriber capable"]
    NOTSUPPORTED = 0x0,
    #[doc = "Subscriber capable"]
    SUPPORTED = 0x01,
}
impl Sstsub {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sstsub {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sstsub {
    #[inline(always)]
    fn from(val: u8) -> Sstsub {
        Sstsub::from_bits(val)
    }
}
impl From<Sstsub> for u8 {
    #[inline(always)]
    fn from(val: Sstsub) -> u8 {
        Sstsub::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sstwr {
    #[doc = "Not write capable"]
    NOTSUPPORTED = 0x0,
    #[doc = "Write capable"]
    SUPPORTED = 0x01,
}
impl Sstwr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sstwr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sstwr {
    #[inline(always)]
    fn from(val: u8) -> Sstwr {
        Sstwr::from_bits(val)
    }
}
impl From<Sstwr> for u8 {
    #[inline(always)]
    fn from(val: Sstwr) -> u8 {
        Sstwr::to_bits(val)
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
pub enum Stdaa {
    #[doc = "Not in ENTDAA mode"]
    NOT_IN_ENTDAA = 0x0,
    #[doc = "In ENTDAA mode"]
    IN_ENTDAA = 0x01,
}
impl Stdaa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stdaa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stdaa {
    #[inline(always)]
    fn from(val: u8) -> Stdaa {
        Stdaa::from_bits(val)
    }
}
impl From<Stdaa> for u8 {
    #[inline(always)]
    fn from(val: Stdaa) -> u8 {
        Stdaa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sthdr {
    #[doc = "I3C bus not in HDR-DDR mode"]
    NOT_IN_HDR_DDR = 0x0,
    #[doc = "I3C bus in HDR-DDR mode"]
    IN_HDR_DDR = 0x01,
}
impl Sthdr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sthdr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sthdr {
    #[inline(always)]
    fn from(val: u8) -> Sthdr {
        Sthdr::from_bits(val)
    }
}
impl From<Sthdr> for u8 {
    #[inline(always)]
    fn from(val: Sthdr) -> u8 {
        Sthdr::to_bits(val)
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
pub enum SwdatabEnd {
    #[doc = "Not the end"]
    NOT_END = 0x0,
    #[doc = "End"]
    END = 0x01,
}
impl SwdatabEnd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwdatabEnd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwdatabEnd {
    #[inline(always)]
    fn from(val: u8) -> SwdatabEnd {
        SwdatabEnd::from_bits(val)
    }
}
impl From<SwdatabEnd> for u8 {
    #[inline(always)]
    fn from(val: SwdatabEnd) -> u8 {
        SwdatabEnd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwdatabEndAlso {
    #[doc = "Not the end"]
    NOT_END = 0x0,
    #[doc = "End"]
    END = 0x01,
}
impl SwdatabEndAlso {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwdatabEndAlso {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwdatabEndAlso {
    #[inline(always)]
    fn from(val: u8) -> SwdatabEndAlso {
        SwdatabEndAlso::from_bits(val)
    }
}
impl From<SwdatabEndAlso> for u8 {
    #[inline(always)]
    fn from(val: SwdatabEndAlso) -> u8 {
        SwdatabEndAlso::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwdatahEnd {
    #[doc = "Not the end"]
    NOT_END = 0x0,
    #[doc = "End"]
    END = 0x01,
}
impl SwdatahEnd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwdatahEnd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwdatahEnd {
    #[inline(always)]
    fn from(val: u8) -> SwdatahEnd {
        SwdatahEnd::from_bits(val)
    }
}
impl From<SwdatahEnd> for u8 {
    #[inline(always)]
    fn from(val: SwdatahEnd) -> u8 {
        SwdatahEnd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Term {
    #[doc = "No terminated error"]
    NO_ERROR = 0x0,
    #[doc = "Terminated error"]
    ERROR = 0x01,
}
impl Term {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Term {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Term {
    #[inline(always)]
    fn from(val: u8) -> Term {
        Term::from_bits(val)
    }
}
impl From<Term> for u8 {
    #[inline(always)]
    fn from(val: Term) -> u8 {
        Term::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timeout {
    #[doc = "No error"]
    NO_ERROR = 0x0,
    #[doc = "Error"]
    ERROR = 0x01,
}
impl Timeout {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timeout {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timeout {
    #[inline(always)]
    fn from(val: u8) -> Timeout {
        Timeout::from_bits(val)
    }
}
impl From<Timeout> for u8 {
    #[inline(always)]
    fn from(val: Timeout) -> u8 {
        Timeout::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txsend {
    #[doc = "Disable"]
    DISABLE = 0x0,
    #[doc = "Enable"]
    ENABLE = 0x01,
}
impl Txsend {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txsend {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txsend {
    #[inline(always)]
    fn from(val: u8) -> Txsend {
        Txsend::from_bits(val)
    }
}
impl From<Txsend> for u8 {
    #[inline(always)]
    fn from(val: Txsend) -> u8 {
        Txsend::to_bits(val)
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Urun {
    #[doc = "No underrun error"]
    NO_ERROR = 0x0,
    #[doc = "Underrun error"]
    ERROR = 0x01,
}
impl Urun {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Urun {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Urun {
    #[inline(always)]
    fn from(val: u8) -> Urun {
        Urun::from_bits(val)
    }
}
impl From<Urun> for u8 {
    #[inline(always)]
    fn from(val: Urun) -> u8 {
        Urun::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Urunnack {
    #[doc = "No underrun; not acknowledged error"]
    NO_ERROR = 0x0,
    #[doc = "Underrun; not acknowledged error"]
    ERROR = 0x01,
}
impl Urunnack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Urunnack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Urunnack {
    #[inline(always)]
    fn from(val: u8) -> Urunnack {
        Urunnack::from_bits(val)
    }
}
impl From<Urunnack> for u8 {
    #[inline(always)]
    fn from(val: Urunnack) -> u8 {
        Urunnack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wrabt {
    #[doc = "No error"]
    NO_ERROR = 0x0,
    #[doc = "Error"]
    ERROR = 0x01,
}
impl Wrabt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wrabt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wrabt {
    #[inline(always)]
    fn from(val: u8) -> Wrabt {
        Wrabt::from_bits(val)
    }
}
impl From<Wrabt> for u8 {
    #[inline(always)]
    fn from(val: Wrabt) -> u8 {
        Wrabt::to_bits(val)
    }
}
