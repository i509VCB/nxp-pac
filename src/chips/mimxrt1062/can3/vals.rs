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
pub enum Aen {
    #[doc = "Abort disabled."]
    ABORT_DISABLED = 0x0,
    #[doc = "Abort enabled."]
    ABORT_ENABLED = 0x01,
}
impl Aen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aen {
    #[inline(always)]
    fn from(val: u8) -> Aen {
        Aen::from_bits(val)
    }
}
impl From<Aen> for u8 {
    #[inline(always)]
    fn from(val: Aen) -> u8 {
        Aen::to_bits(val)
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
pub enum Boffmsk {
    #[doc = "Bus Off interrupt disabled."]
    BUS_OFF_INT_DISABLED = 0x0,
    #[doc = "Bus Off interrupt enabled."]
    BUS_OFF_INT_ENABLED = 0x01,
}
impl Boffmsk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Boffmsk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Boffmsk {
    #[inline(always)]
    fn from(val: u8) -> Boffmsk {
        Boffmsk::from_bits(val)
    }
}
impl From<Boffmsk> for u8 {
    #[inline(always)]
    fn from(val: Boffmsk) -> u8 {
        Boffmsk::to_bits(val)
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
pub enum Dma {
    #[doc = "DMA feature for RX FIFO disabled."]
    ID2 = 0x0,
    #[doc = "DMA feature for RX FIFO enabled."]
    ID4 = 0x01,
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
pub enum Doze {
    #[doc = "FlexCAN is not enabled to enter low-power mode when Doze mode is requested."]
    LOW_POWER_DOZE_DISABLED = 0x0,
    #[doc = "FlexCAN is enabled to enter low-power mode when Doze mode is requested."]
    LOW_POWER_DOZE_ENABLED = 0x01,
}
impl Doze {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Doze {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Doze {
    #[inline(always)]
    fn from(val: u8) -> Doze {
        Doze::from_bits(val)
    }
}
impl From<Doze> for u8 {
    #[inline(always)]
    fn from(val: Doze) -> u8 {
        Doze::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Eacen {
    #[doc = "Rx mailbox filter's IDE bit is always compared and RTR is never compared despite mask bits."]
    RTR_COMPARE_NO = 0x0,
    #[doc = "Enables the comparison of both Rx mailbox filter's IDE and RTR bit with their corresponding bits within the incoming frame. Mask bits do apply."]
    RTR_COMPARE_YES = 0x01,
}
impl Eacen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eacen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eacen {
    #[inline(always)]
    fn from(val: u8) -> Eacen {
        Eacen::from_bits(val)
    }
}
impl From<Eacen> for u8 {
    #[inline(always)]
    fn from(val: Eacen) -> u8 {
        Eacen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edfltdis {
    #[doc = "Edge filter is enabled"]
    ENABLE = 0x0,
    #[doc = "Edge filter is disabled"]
    DISABLE = 0x01,
}
impl Edfltdis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edfltdis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edfltdis {
    #[inline(always)]
    fn from(val: u8) -> Edfltdis {
        Edfltdis::from_bits(val)
    }
}
impl From<Edfltdis> for u8 {
    #[inline(always)]
    fn from(val: Edfltdis) -> u8 {
        Edfltdis::to_bits(val)
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
pub enum Errmsk {
    #[doc = "Error interrupt disabled."]
    ERROR_INT_DISABLED = 0x0,
    #[doc = "Error interrupt enabled."]
    ERROR_INT_ENABLED = 0x01,
}
impl Errmsk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Errmsk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Errmsk {
    #[inline(always)]
    fn from(val: u8) -> Errmsk {
        Errmsk::from_bits(val)
    }
}
impl From<Errmsk> for u8 {
    #[inline(always)]
    fn from(val: Errmsk) -> u8 {
        Errmsk::to_bits(val)
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
pub enum Fden {
    #[doc = "CAN FD is disabled. FlexCAN is able to receive and transmit messages in CAN 2.0 format."]
    CAN_FD_DISABLED = 0x0,
    #[doc = "CAN FD is enabled. FlexCAN is able to receive and transmit messages in both CAN FD and CAN 2.0 formats."]
    CAN_FD_ENABLED = 0x01,
}
impl Fden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fden {
    #[inline(always)]
    fn from(val: u8) -> Fden {
        Fden::from_bits(val)
    }
}
impl From<Fden> for u8 {
    #[inline(always)]
    fn from(val: Fden) -> u8 {
        Fden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fdrate {
    #[doc = "Transmit a frame in nominal rate. The BRS bit in the Tx MB has no effect."]
    NOMINAL = 0x0,
    #[doc = "Transmit a frame with bit rate switching if the BRS bit in the Tx MB is recessive."]
    BIT_RATE_SWITCHING = 0x01,
}
impl Fdrate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fdrate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fdrate {
    #[inline(always)]
    fn from(val: u8) -> Fdrate {
        Fdrate::from_bits(val)
    }
}
impl From<Fdrate> for u8 {
    #[inline(always)]
    fn from(val: Fdrate) -> u8 {
        Fdrate::to_bits(val)
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
pub enum Frz {
    #[doc = "Not enabled to enter Freeze mode."]
    FREEZE_MODE_DISABLED = 0x0,
    #[doc = "Enabled to enter Freeze mode."]
    FREEZE_MODE_ENABLED = 0x01,
}
impl Frz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Frz {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Frz {
    #[inline(always)]
    fn from(val: u8) -> Frz {
        Frz::from_bits(val)
    }
}
impl From<Frz> for u8 {
    #[inline(always)]
    fn from(val: Frz) -> u8 {
        Frz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Frzack {
    #[doc = "FlexCAN not in Freeze mode, prescaler running."]
    FREEZE_MODE_NO = 0x0,
    #[doc = "FlexCAN in Freeze mode, prescaler stopped."]
    FREEZE_MODE_YES = 0x01,
}
impl Frzack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Frzack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Frzack {
    #[inline(always)]
    fn from(val: u8) -> Frzack {
        Frzack::from_bits(val)
    }
}
impl From<Frzack> for u8 {
    #[inline(always)]
    fn from(val: Frzack) -> u8 {
        Frzack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Halt {
    #[doc = "No Freeze mode request."]
    HALT_DISABLE = 0x0,
    #[doc = "Enters Freeze mode if the FRZ bit is asserted."]
    HALT_ENABLE = 0x01,
}
impl Halt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Halt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Halt {
    #[inline(always)]
    fn from(val: u8) -> Halt {
        Halt::from_bits(val)
    }
}
impl From<Halt> for u8 {
    #[inline(always)]
    fn from(val: Halt) -> u8 {
        Halt::to_bits(val)
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
pub enum Imb {
    #[doc = "If ESR2\\[VPS\\] is asserted, the ESR2\\[LPTM\\] is not an inactive mailbox."]
    INACTIVE_MAILBOX_NO = 0x0,
    #[doc = "If ESR2\\[VPS\\] is asserted, there is at least one inactive mailbox. LPTM content is the number of the first one."]
    INACTIVE_MAILBOX_YES = 0x01,
}
impl Imb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Imb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Imb {
    #[inline(always)]
    fn from(val: u8) -> Imb {
        Imb::from_bits(val)
    }
}
impl From<Imb> for u8 {
    #[inline(always)]
    fn from(val: Imb) -> u8 {
        Imb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Irmq {
    #[doc = "Individual Rx masking and queue feature are disabled. For backward compatibility with legacy applications, the reading of C/S word locks the MB even if it is EMPTY."]
    INDIVIDUAL_RX_MASKING_DISABLED = 0x0,
    #[doc = "Individual Rx masking and queue feature are enabled."]
    INDIVIDUAL_RX_MASKING_ENABLED = 0x01,
}
impl Irmq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Irmq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Irmq {
    #[inline(always)]
    fn from(val: u8) -> Irmq {
        Irmq::from_bits(val)
    }
}
impl From<Irmq> for u8 {
    #[inline(always)]
    fn from(val: Irmq) -> u8 {
        Irmq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isocanfden {
    #[doc = "FlexCAN operates using the non-ISO CAN FD protocol."]
    NON_ISO = 0x0,
    #[doc = "FlexCAN operates using the ISO CAN FD protocol (ISO 11898-1)."]
    ISO = 0x01,
}
impl Isocanfden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isocanfden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isocanfden {
    #[inline(always)]
    fn from(val: u8) -> Isocanfden {
        Isocanfden::from_bits(val)
    }
}
impl From<Isocanfden> for u8 {
    #[inline(always)]
    fn from(val: Isocanfden) -> u8 {
        Isocanfden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lbuf {
    #[doc = "Buffer with highest priority is transmitted first."]
    HIGHEST_BUFFER_FIRST = 0x0,
    #[doc = "Lowest number buffer is transmitted first."]
    LOWEST_BUFFER_FIRST = 0x01,
}
impl Lbuf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lbuf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lbuf {
    #[inline(always)]
    fn from(val: u8) -> Lbuf {
        Lbuf::from_bits(val)
    }
}
impl From<Lbuf> for u8 {
    #[inline(always)]
    fn from(val: Lbuf) -> u8 {
        Lbuf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lom {
    #[doc = "Listen-Only mode is deactivated."]
    LISTEN_ONLY_MODE_DISABLED = 0x0,
    #[doc = "FlexCAN module operates in Listen-Only mode."]
    LISTEN_ONLY_MODE_ENABLED = 0x01,
}
impl Lom {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lom {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lom {
    #[inline(always)]
    fn from(val: u8) -> Lom {
        Lom::from_bits(val)
    }
}
impl From<Lom> for u8 {
    #[inline(always)]
    fn from(val: Lom) -> u8 {
        Lom::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpb {
    #[doc = "Loop Back disabled."]
    LOOPBACK_DISABLED = 0x0,
    #[doc = "Loop Back enabled."]
    LOOPBACK_ENABLED = 0x01,
}
impl Lpb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpb {
    #[inline(always)]
    fn from(val: u8) -> Lpb {
        Lpb::from_bits(val)
    }
}
impl From<Lpb> for u8 {
    #[inline(always)]
    fn from(val: Lpb) -> u8 {
        Lpb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpmack {
    #[doc = "FlexCAN is not in a low-power mode."]
    LOW_POWER_NO = 0x0,
    #[doc = "FlexCAN is in a low-power mode."]
    LOW_POWER_YES = 0x01,
}
impl Lpmack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpmack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpmack {
    #[inline(always)]
    fn from(val: u8) -> Lpmack {
        Lpmack::from_bits(val)
    }
}
impl From<Lpmack> for u8 {
    #[inline(always)]
    fn from(val: Lpmack) -> u8 {
        Lpmack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lprioen {
    #[doc = "Local Priority disabled."]
    LOCAL_PRIORITY_DISABLED = 0x0,
    #[doc = "Local Priority enabled."]
    LOCAL_PRIORITY_ENABLED = 0x01,
}
impl Lprioen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lprioen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lprioen {
    #[inline(always)]
    fn from(val: u8) -> Lprioen {
        Lprioen::from_bits(val)
    }
}
impl From<Lprioen> for u8 {
    #[inline(always)]
    fn from(val: Lprioen) -> u8 {
        Lprioen::to_bits(val)
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
pub enum Mdis {
    #[doc = "Enable the FlexCAN module."]
    FLEXCAN_ENABLED = 0x0,
    #[doc = "Disable the FlexCAN module."]
    FLEXCAN_DISABLED = 0x01,
}
impl Mdis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mdis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mdis {
    #[inline(always)]
    fn from(val: u8) -> Mdis {
        Mdis::from_bits(val)
    }
}
impl From<Mdis> for u8 {
    #[inline(always)]
    fn from(val: Mdis) -> u8 {
        Mdis::to_bits(val)
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
pub enum Rfen {
    #[doc = "Rx FIFO not enabled."]
    ID2 = 0x0,
    #[doc = "Rx FIFO enabled."]
    ID4 = 0x01,
}
impl Rfen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rfen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rfen {
    #[inline(always)]
    fn from(val: u8) -> Rfen {
        Rfen::from_bits(val)
    }
}
impl From<Rfen> for u8 {
    #[inline(always)]
    fn from(val: Rfen) -> u8 {
        Rfen::to_bits(val)
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
pub enum Rwrnmsk {
    #[doc = "Rx Warning interrupt disabled."]
    RX_WARNING_INT_DISABLED = 0x0,
    #[doc = "Rx Warning interrupt enabled."]
    RX_WARNING_INT_ENABLED = 0x01,
}
impl Rwrnmsk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rwrnmsk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rwrnmsk {
    #[inline(always)]
    fn from(val: u8) -> Rwrnmsk {
        Rwrnmsk::from_bits(val)
    }
}
impl From<Rwrnmsk> for u8 {
    #[inline(always)]
    fn from(val: Rwrnmsk) -> u8 {
        Rwrnmsk::to_bits(val)
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
pub enum Slfwak {
    #[doc = "FlexCAN Self Wake Up feature is disabled."]
    SELF_WAKEUP_DISABLED = 0x0,
    #[doc = "FlexCAN Self Wake Up feature is enabled."]
    SELF_WAKEUP_ENABLED = 0x01,
}
impl Slfwak {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Slfwak {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Slfwak {
    #[inline(always)]
    fn from(val: u8) -> Slfwak {
        Slfwak::from_bits(val)
    }
}
impl From<Slfwak> for u8 {
    #[inline(always)]
    fn from(val: Slfwak) -> u8 {
        Slfwak::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Smp {
    #[doc = "Just one sample is used to determine the bit value."]
    ONE_SAMPLE = 0x0,
    #[doc = "Three samples are used to determine the value of the received bit: the regular one (sample point) and two preceding samples; a majority rule is used."]
    THREE_SAMPLE = 0x01,
}
impl Smp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Smp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Smp {
    #[inline(always)]
    fn from(val: u8) -> Smp {
        Smp::from_bits(val)
    }
}
impl From<Smp> for u8 {
    #[inline(always)]
    fn from(val: Smp) -> u8 {
        Smp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Softrst {
    #[doc = "No reset request."]
    SOFTRST_NO_RESET_REQUEST = 0x0,
    #[doc = "Resets the registers affected by soft reset."]
    SOFTRST_RESET_REGISTERS = 0x01,
}
impl Softrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Softrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Softrst {
    #[inline(always)]
    fn from(val: u8) -> Softrst {
        Softrst::from_bits(val)
    }
}
impl From<Softrst> for u8 {
    #[inline(always)]
    fn from(val: Softrst) -> u8 {
        Softrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Srxdis {
    #[doc = "Self-reception enabled."]
    SELF_RECEPTION_ENABLED = 0x0,
    #[doc = "Self-reception disabled."]
    SELF_RECEPTION_DISABLED = 0x01,
}
impl Srxdis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Srxdis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Srxdis {
    #[inline(always)]
    fn from(val: u8) -> Srxdis {
        Srxdis::from_bits(val)
    }
}
impl From<Srxdis> for u8 {
    #[inline(always)]
    fn from(val: Srxdis) -> u8 {
        Srxdis::to_bits(val)
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
pub enum Supv {
    #[doc = "FlexCAN is in User mode. Affected registers allow both Supervisor and Unrestricted accesses."]
    ID2 = 0x0,
    #[doc = "FlexCAN is in Supervisor mode. Affected registers allow only Supervisor access. Unrestricted access behaves as though the access was done to an unimplemented register location."]
    ID4 = 0x01,
}
impl Supv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Supv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Supv {
    #[inline(always)]
    fn from(val: u8) -> Supv {
        Supv::from_bits(val)
    }
}
impl From<Supv> for u8 {
    #[inline(always)]
    fn from(val: Supv) -> u8 {
        Supv::to_bits(val)
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
pub enum Tsyn {
    #[doc = "Timer sync feature disabled"]
    TIMER_SYNC_DISABLED = 0x0,
    #[doc = "Timer sync feature enabled"]
    TIMER_SYNC_ENABLED = 0x01,
}
impl Tsyn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsyn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsyn {
    #[inline(always)]
    fn from(val: u8) -> Tsyn {
        Tsyn::from_bits(val)
    }
}
impl From<Tsyn> for u8 {
    #[inline(always)]
    fn from(val: Tsyn) -> u8 {
        Tsyn::to_bits(val)
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
pub enum Twrnmsk {
    #[doc = "Tx Warning interrupt disabled."]
    TX_WARNING_INT_DISABLED = 0x0,
    #[doc = "Tx Warning interrupt enabled."]
    TX_WARNING_INT_ENABLED = 0x01,
}
impl Twrnmsk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Twrnmsk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Twrnmsk {
    #[inline(always)]
    fn from(val: u8) -> Twrnmsk {
        Twrnmsk::from_bits(val)
    }
}
impl From<Twrnmsk> for u8 {
    #[inline(always)]
    fn from(val: Twrnmsk) -> u8 {
        Twrnmsk::to_bits(val)
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wakmsk {
    #[doc = "Wake Up interrupt is disabled."]
    WAKEUP_INTERRUPT_DISABLED = 0x0,
    #[doc = "Wake Up interrupt is enabled."]
    WAKEUP_INTERRUPT_ENABLED = 0x01,
}
impl Wakmsk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wakmsk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wakmsk {
    #[inline(always)]
    fn from(val: u8) -> Wakmsk {
        Wakmsk::from_bits(val)
    }
}
impl From<Wakmsk> for u8 {
    #[inline(always)]
    fn from(val: Wakmsk) -> u8 {
        Wakmsk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wrnen {
    #[doc = "TWRNINT and RWRNINT bits are zero, independent of the values in the error counters."]
    TWRNINT_RWRNINT_INACTIVE = 0x0,
    #[doc = "TWRNINT and RWRNINT bits are set when the respective error counter transitions from less than 96 to greater than or equal to 96."]
    TWRNINT_RWRNINT_ACTIVE = 0x01,
}
impl Wrnen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wrnen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wrnen {
    #[inline(always)]
    fn from(val: u8) -> Wrnen {
        Wrnen::from_bits(val)
    }
}
impl From<Wrnen> for u8 {
    #[inline(always)]
    fn from(val: Wrnen) -> u8 {
        Wrnen::to_bits(val)
    }
}
