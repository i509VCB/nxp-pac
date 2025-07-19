#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ackerr {
    #[doc = "No such occurrence."]
    ACK_ERROR_NO = 0x0,
    #[doc = "An ACK error occurred since last read of this register."]
    ACK_ERROR_YES = 0x01,
}
impl Ackerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ackerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ackerr {
    #[inline(always)]
    fn from(val: u8) -> Ackerr {
        Ackerr::from_bits(val)
    }
}
impl From<Ackerr> for u8 {
    #[inline(always)]
    fn from(val: Ackerr) -> u8 {
        Ackerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bit0err {
    #[doc = "No such occurrence."]
    BIT0_ERROR_NO = 0x0,
    #[doc = "At least one bit sent as dominant is received as recessive."]
    BIT0_ERROR_YES = 0x01,
}
impl Bit0err {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bit0err {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bit0err {
    #[inline(always)]
    fn from(val: u8) -> Bit0err {
        Bit0err::from_bits(val)
    }
}
impl From<Bit0err> for u8 {
    #[inline(always)]
    fn from(val: Bit0err) -> u8 {
        Bit0err::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bit0errFast {
    #[doc = "No such occurrence."]
    BIT0_ERROR_NO = 0x0,
    #[doc = "At least one bit sent as dominant is received as recessive."]
    BIT0_ERROR_YES = 0x01,
}
impl Bit0errFast {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bit0errFast {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bit0errFast {
    #[inline(always)]
    fn from(val: u8) -> Bit0errFast {
        Bit0errFast::from_bits(val)
    }
}
impl From<Bit0errFast> for u8 {
    #[inline(always)]
    fn from(val: Bit0errFast) -> u8 {
        Bit0errFast::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bit1err {
    #[doc = "No such occurrence."]
    BIT1_ERROR_NO = 0x0,
    #[doc = "At least one bit sent as recessive is received as dominant."]
    BIT1_ERROR_YES = 0x01,
}
impl Bit1err {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bit1err {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bit1err {
    #[inline(always)]
    fn from(val: u8) -> Bit1err {
        Bit1err::from_bits(val)
    }
}
impl From<Bit1err> for u8 {
    #[inline(always)]
    fn from(val: Bit1err) -> u8 {
        Bit1err::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bit1errFast {
    #[doc = "No such occurrence."]
    BIT1_ERROR_NO = 0x0,
    #[doc = "At least one bit sent as recessive is received as dominant."]
    BIT1_ERROR_YES = 0x01,
}
impl Bit1errFast {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bit1errFast {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bit1errFast {
    #[inline(always)]
    fn from(val: u8) -> Bit1errFast {
        Bit1errFast::from_bits(val)
    }
}
impl From<Bit1errFast> for u8 {
    #[inline(always)]
    fn from(val: Bit1errFast) -> u8 {
        Bit1errFast::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Boffdoneint {
    #[doc = "No such occurrence."]
    BUS_OFF_NOT_DONE = 0x0,
    #[doc = "FlexCAN module has completed Bus Off process."]
    BUS_OFF_DONE = 0x01,
}
impl Boffdoneint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Boffdoneint {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Boffdoneint {
    #[inline(always)]
    fn from(val: u8) -> Boffdoneint {
        Boffdoneint::from_bits(val)
    }
}
impl From<Boffdoneint> for u8 {
    #[inline(always)]
    fn from(val: Boffdoneint) -> u8 {
        Boffdoneint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Boffrec {
    #[doc = "Automatic recovering from Bus Off state enabled."]
    AUTO_RECOVER_ENABLED = 0x0,
    #[doc = "Automatic recovering from Bus Off state disabled."]
    AUTO_RECOVER_DISABLED = 0x01,
}
impl Boffrec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Boffrec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Boffrec {
    #[inline(always)]
    fn from(val: u8) -> Boffrec {
        Boffrec::from_bits(val)
    }
}
impl From<Boffrec> for u8 {
    #[inline(always)]
    fn from(val: Boffrec) -> u8 {
        Boffrec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Buf0i {
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    BUFFER_TX_RX_NOT_COMPLETE = 0x0,
    #[doc = "The corresponding buffer has successfully completed transmission or reception when MCR\\[RFEN\\]=0."]
    BUFFER_TX_RX_COMPLETE = 0x01,
}
impl Buf0i {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Buf0i {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Buf0i {
    #[inline(always)]
    fn from(val: u8) -> Buf0i {
        Buf0i::from_bits(val)
    }
}
impl From<Buf0i> for u8 {
    #[inline(always)]
    fn from(val: Buf0i) -> u8 {
        Buf0i::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Buf5i {
    #[doc = "No occurrence of MB5 completing transmission/reception when MCR\\[RFEN\\]=0, or of frame(s) available in the FIFO, when MCR\\[RFEN\\]=1"]
    ID2 = 0x0,
    #[doc = "MB5 completed transmission/reception when MCR\\[RFEN\\]=0, or frame(s) available in the Rx FIFO when MCR\\[RFEN\\]=1. It generates a DMA request in case of MCR\\[RFEN\\] and MCR\\[DMA\\] are enabled."]
    ID4 = 0x01,
}
impl Buf5i {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Buf5i {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Buf5i {
    #[inline(always)]
    fn from(val: u8) -> Buf5i {
        Buf5i::from_bits(val)
    }
}
impl From<Buf5i> for u8 {
    #[inline(always)]
    fn from(val: Buf5i) -> u8 {
        Buf5i::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Buf6i {
    #[doc = "No occurrence of MB6 completing transmission/reception when MCR\\[RFEN\\]=0, or of Rx FIFO almost full when MCR\\[RFEN\\]=1"]
    ID2 = 0x0,
    #[doc = "MB6 completed transmission/reception when MCR\\[RFEN\\]=0, or Rx FIFO almost full when MCR\\[RFEN\\]=1"]
    ID4 = 0x01,
}
impl Buf6i {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Buf6i {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Buf6i {
    #[inline(always)]
    fn from(val: u8) -> Buf6i {
        Buf6i::from_bits(val)
    }
}
impl From<Buf6i> for u8 {
    #[inline(always)]
    fn from(val: Buf6i) -> u8 {
        Buf6i::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Buf7i {
    #[doc = "No occurrence of MB7 completing transmission/reception when MCR\\[RFEN\\]=0, or of Rx FIFO overflow when MCR\\[RFEN\\]=1"]
    ID2 = 0x0,
    #[doc = "MB7 completed transmission/reception when MCR\\[RFEN\\]=0, or Rx FIFO overflow when MCR\\[RFEN\\]=1"]
    ID4 = 0x01,
}
impl Buf7i {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Buf7i {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Buf7i {
    #[inline(always)]
    fn from(val: u8) -> Buf7i {
        Buf7i::from_bits(val)
    }
}
impl From<Buf7i> for u8 {
    #[inline(always)]
    fn from(val: Buf7i) -> u8 {
        Buf7i::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crcerr {
    #[doc = "No such occurrence."]
    CRC_ERROR_NO = 0x0,
    #[doc = "A CRC error occurred since last read of this register."]
    CRC_ERROR_YES = 0x01,
}
impl Crcerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crcerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crcerr {
    #[inline(always)]
    fn from(val: u8) -> Crcerr {
        Crcerr::from_bits(val)
    }
}
impl From<Crcerr> for u8 {
    #[inline(always)]
    fn from(val: Crcerr) -> u8 {
        Crcerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CrcerrFast {
    #[doc = "No such occurrence."]
    CRC_ERROR_NO = 0x0,
    #[doc = "A CRC error occurred since last read of this register."]
    CRC_ERROR_YES = 0x01,
}
impl CrcerrFast {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CrcerrFast {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CrcerrFast {
    #[inline(always)]
    fn from(val: u8) -> CrcerrFast {
        CrcerrFast::from_bits(val)
    }
}
impl From<CrcerrFast> for u8 {
    #[inline(always)]
    fn from(val: CrcerrFast) -> u8 {
        CrcerrFast::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ErrintFast {
    #[doc = "No such occurrence."]
    ERRORS_DATA_PHASE_NO = 0x0,
    #[doc = "Indicates setting of any error bit detected in the data phase of CAN FD frames with the BRS bit set."]
    ERRORS_DATA_PHASE_YES = 0x01,
}
impl ErrintFast {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ErrintFast {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ErrintFast {
    #[inline(always)]
    fn from(val: u8) -> ErrintFast {
        ErrintFast::from_bits(val)
    }
}
impl From<ErrintFast> for u8 {
    #[inline(always)]
    fn from(val: ErrintFast) -> u8 {
        ErrintFast::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Errovr {
    #[doc = "Overrun has not occurred."]
    OVERRUN_NOT_OCCURRED = 0x0,
    #[doc = "Overrun has occurred."]
    OVERRUN_OCCURRED = 0x01,
}
impl Errovr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Errovr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Errovr {
    #[inline(always)]
    fn from(val: u8) -> Errovr {
        Errovr::from_bits(val)
    }
}
impl From<Errovr> for u8 {
    #[inline(always)]
    fn from(val: Errovr) -> u8 {
        Errovr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fltconf {
    #[doc = "Error Active"]
    ERROR_ACTIVE = 0x0,
    #[doc = "Error Passive"]
    ERROR_PASSIVE = 0x01,
    #[doc = "Bus Off"]
    BUS_OFF = 0x02,
    _RESERVED_3 = 0x03,
}
impl Fltconf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fltconf {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fltconf {
    #[inline(always)]
    fn from(val: u8) -> Fltconf {
        Fltconf::from_bits(val)
    }
}
impl From<Fltconf> for u8 {
    #[inline(always)]
    fn from(val: Fltconf) -> u8 {
        Fltconf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Frmerr {
    #[doc = "No such occurrence."]
    FORM_ERROR_NO = 0x0,
    #[doc = "A Form Error occurred since last read of this register."]
    FORM_ERROR_YES = 0x01,
}
impl Frmerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Frmerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Frmerr {
    #[inline(always)]
    fn from(val: u8) -> Frmerr {
        Frmerr::from_bits(val)
    }
}
impl From<Frmerr> for u8 {
    #[inline(always)]
    fn from(val: Frmerr) -> u8 {
        Frmerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrmerrFast {
    #[doc = "No such occurrence."]
    FORM_ERROR_NO = 0x0,
    #[doc = "A form error occurred since last read of this register."]
    FORM_ERROR_YES = 0x01,
}
impl FrmerrFast {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrmerrFast {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrmerrFast {
    #[inline(always)]
    fn from(val: u8) -> FrmerrFast {
        FrmerrFast::from_bits(val)
    }
}
impl From<FrmerrFast> for u8 {
    #[inline(always)]
    fn from(val: FrmerrFast) -> u8 {
        FrmerrFast::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Idam {
    #[doc = "Format A: One full ID (standard and extended) per ID filter table element."]
    ONE_FULL_ID = 0x0,
    #[doc = "Format B: Two full standard IDs or two partial 14-bit (standard and extended) IDs per ID filter table element."]
    TWO_FULL_ID = 0x01,
    #[doc = "Format C: Four partial 8-bit standard IDs per ID filter table element."]
    FOUR_PARTIAL_ID = 0x02,
    #[doc = "Format D: All frames rejected."]
    ALL_FRAMES_REJECTED = 0x03,
}
impl Idam {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Idam {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Idam {
    #[inline(always)]
    fn from(val: u8) -> Idam {
        Idam::from_bits(val)
    }
}
impl From<Idam> for u8 {
    #[inline(always)]
    fn from(val: Idam) -> u8 {
        Idam::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Idle {
    #[doc = "No such occurrence."]
    CAN_BUS_NOT_IDLE = 0x0,
    #[doc = "CAN bus is now IDLE."]
    CAN_BUS_IDLE = 0x01,
}
impl Idle {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Idle {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Idle {
    #[inline(always)]
    fn from(val: u8) -> Idle {
        Idle::from_bits(val)
    }
}
impl From<Idle> for u8 {
    #[inline(always)]
    fn from(val: Idle) -> u8 {
        Idle::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbdsr0 {
    #[doc = "Selects 8 bytes per message buffer."]
    R0_8_BYTES = 0x0,
    #[doc = "Selects 16 bytes per message buffer."]
    R0_16_BYTES = 0x01,
    #[doc = "Selects 32 bytes per message buffer."]
    R0_32_BYTES = 0x02,
    #[doc = "Selects 64 bytes per message buffer."]
    R0_64_BYTES = 0x03,
}
impl Mbdsr0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbdsr0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbdsr0 {
    #[inline(always)]
    fn from(val: u8) -> Mbdsr0 {
        Mbdsr0::from_bits(val)
    }
}
impl From<Mbdsr0> for u8 {
    #[inline(always)]
    fn from(val: Mbdsr0) -> u8 {
        Mbdsr0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbdsr1 {
    #[doc = "Selects 8 bytes per message buffer."]
    R1_8_BYTES = 0x0,
    #[doc = "Selects 16 bytes per message buffer."]
    R1_16_BYTES = 0x01,
    #[doc = "Selects 32 bytes per message buffer."]
    R1_32_BYTES = 0x02,
    #[doc = "Selects 64 bytes per message buffer."]
    R1_64_BYTES = 0x03,
}
impl Mbdsr1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbdsr1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbdsr1 {
    #[inline(always)]
    fn from(val: u8) -> Mbdsr1 {
        Mbdsr1::from_bits(val)
    }
}
impl From<Mbdsr1> for u8 {
    #[inline(always)]
    fn from(val: Mbdsr1) -> u8 {
        Mbdsr1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mrp {
    #[doc = "Matching starts from Rx FIFO and continues on mailboxes."]
    ID2 = 0x0,
    #[doc = "Matching starts from mailboxes and continues on Rx FIFO."]
    ID4 = 0x01,
}
impl Mrp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mrp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mrp {
    #[inline(always)]
    fn from(val: u8) -> Mrp {
        Mrp::from_bits(val)
    }
}
impl From<Mrp> for u8 {
    #[inline(always)]
    fn from(val: Mrp) -> u8 {
        Mrp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Notrdy {
    #[doc = "FlexCAN module is either in Normal mode, Listen-Only mode, or Loop-Back mode."]
    ID1 = 0x0,
    #[doc = "FlexCAN module is either in Disable mode, Doze mode, Stop mode, or Freeze mode."]
    ID2 = 0x01,
}
impl Notrdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Notrdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Notrdy {
    #[inline(always)]
    fn from(val: u8) -> Notrdy {
        Notrdy::from_bits(val)
    }
}
impl From<Notrdy> for u8 {
    #[inline(always)]
    fn from(val: Notrdy) -> u8 {
        Notrdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rrs {
    #[doc = "Remote response frame is generated."]
    REMOTE_RESPONSE_FRAME_NOT_GENERATED = 0x0,
    #[doc = "Remote request frame is stored."]
    REMOTE_RESPONSE_FRAME_GENERATED = 0x01,
}
impl Rrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rrs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rrs {
    #[inline(always)]
    fn from(val: u8) -> Rrs {
        Rrs::from_bits(val)
    }
}
impl From<Rrs> for u8 {
    #[inline(always)]
    fn from(val: Rrs) -> u8 {
        Rrs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rwrnint {
    #[doc = "No such occurrence."]
    RX_WARNING_INT_NO = 0x0,
    #[doc = "The Rx error counter transitioned from less than 96 to greater than or equal to 96."]
    RX_WARNING_INT_YES = 0x01,
}
impl Rwrnint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rwrnint {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rwrnint {
    #[inline(always)]
    fn from(val: u8) -> Rwrnint {
        Rwrnint::from_bits(val)
    }
}
impl From<Rwrnint> for u8 {
    #[inline(always)]
    fn from(val: Rwrnint) -> u8 {
        Rwrnint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxwrn {
    #[doc = "No such occurrence."]
    RXERRCNT_LT_96 = 0x0,
    #[doc = "RXERRCNT is greater than or equal to 96."]
    RXERRCNT_GTE_96 = 0x01,
}
impl Rxwrn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxwrn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxwrn {
    #[inline(always)]
    fn from(val: u8) -> Rxwrn {
        Rxwrn::from_bits(val)
    }
}
impl From<Rxwrn> for u8 {
    #[inline(always)]
    fn from(val: Rxwrn) -> u8 {
        Rxwrn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Stferr {
    #[doc = "No such occurrence."]
    STUFFING_ERROR_NO = 0x0,
    #[doc = "A stuffing error occurred since last read of this register."]
    STUFFING_ERROR_YES = 0x01,
}
impl Stferr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stferr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stferr {
    #[inline(always)]
    fn from(val: u8) -> Stferr {
        Stferr::from_bits(val)
    }
}
impl From<Stferr> for u8 {
    #[inline(always)]
    fn from(val: Stferr) -> u8 {
        Stferr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StferrFast {
    #[doc = "No such occurrence."]
    STUFFING_ERROR_NO = 0x0,
    #[doc = "A stuffing error occurred since last read of this register."]
    STUFFING_ERROR_YES = 0x01,
}
impl StferrFast {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StferrFast {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StferrFast {
    #[inline(always)]
    fn from(val: u8) -> StferrFast {
        StferrFast::from_bits(val)
    }
}
impl From<StferrFast> for u8 {
    #[inline(always)]
    fn from(val: StferrFast) -> u8 {
        StferrFast::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Synch {
    #[doc = "FlexCAN is not synchronized to the CAN bus."]
    CAN_BUS_SYNC_NO = 0x0,
    #[doc = "FlexCAN is synchronized to the CAN bus."]
    CAN_BUS_SYNC_YES = 0x01,
}
impl Synch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Synch {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Synch {
    #[inline(always)]
    fn from(val: u8) -> Synch {
        Synch::from_bits(val)
    }
}
impl From<Synch> for u8 {
    #[inline(always)]
    fn from(val: Synch) -> u8 {
        Synch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tdcfail {
    #[doc = "Measured loop delay is in range."]
    IN_RANGE = 0x0,
    #[doc = "Measured loop delay is out of range."]
    OUT_OF_RANGE = 0x01,
}
impl Tdcfail {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tdcfail {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tdcfail {
    #[inline(always)]
    fn from(val: u8) -> Tdcfail {
        Tdcfail::from_bits(val)
    }
}
impl From<Tdcfail> for u8 {
    #[inline(always)]
    fn from(val: Tdcfail) -> u8 {
        Tdcfail::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TimerSrc {
    #[doc = "The free running timer is clocked by the CAN bit clock, which defines the baud rate on the CAN bus."]
    CAN_BIT_CLOCK = 0x0,
    #[doc = "The free running timer is clocked by an external time tick. The period can be either adjusted to be equal to the baud rate on the CAN bus, or a different value as required. See the device-specific section for details about the external time tick."]
    EXTERNAL_CLOCK = 0x01,
}
impl TimerSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TimerSrc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TimerSrc {
    #[inline(always)]
    fn from(val: u8) -> TimerSrc {
        TimerSrc::from_bits(val)
    }
}
impl From<TimerSrc> for u8 {
    #[inline(always)]
    fn from(val: TimerSrc) -> u8 {
        TimerSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Twrnint {
    #[doc = "No such occurrence."]
    TX_WARNING_INT_NO = 0x0,
    #[doc = "The Tx error counter transitioned from less than 96 to greater than or equal to 96."]
    TX_WARNING_INT_YES = 0x01,
}
impl Twrnint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Twrnint {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Twrnint {
    #[inline(always)]
    fn from(val: u8) -> Twrnint {
        Twrnint::from_bits(val)
    }
}
impl From<Twrnint> for u8 {
    #[inline(always)]
    fn from(val: Twrnint) -> u8 {
        Twrnint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tx {
    #[doc = "FlexCAN is not transmitting a message."]
    TRANSMIT_MESSAGE_NO = 0x0,
    #[doc = "FlexCAN is transmitting a message."]
    TRANSMIT_MESSAGE_YES = 0x01,
}
impl Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tx {
    #[inline(always)]
    fn from(val: u8) -> Tx {
        Tx::from_bits(val)
    }
}
impl From<Tx> for u8 {
    #[inline(always)]
    fn from(val: Tx) -> u8 {
        Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txwrn {
    #[doc = "No such occurrence."]
    TXERRCNT_LT_96 = 0x0,
    #[doc = "TXERRCNT is greater than or equal to 96."]
    TXERRCNT_GTE_96 = 0x01,
}
impl Txwrn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txwrn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txwrn {
    #[inline(always)]
    fn from(val: u8) -> Txwrn {
        Txwrn::from_bits(val)
    }
}
impl From<Txwrn> for u8 {
    #[inline(always)]
    fn from(val: Txwrn) -> u8 {
        Txwrn::to_bits(val)
    }
}
