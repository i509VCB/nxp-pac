#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ackerr {
    #[doc = "No error"]
    ACK_ERROR_NO = 0x0,
    #[doc = "Error occurred since last read of this register."]
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
    #[doc = "Disabled"]
    ABORT_DISABLED = 0x0,
    #[doc = "Enabled"]
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
pub enum Asd {
    #[doc = "Enabled"]
    ENABLE = 0x0,
    #[doc = "Disabled"]
    DISABLE = 0x01,
}
impl Asd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Asd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Asd {
    #[inline(always)]
    fn from(val: u8) -> Asd {
        Asd::from_bits(val)
    }
}
impl From<Asd> for u8 {
    #[inline(always)]
    fn from(val: Asd) -> u8 {
        Asd::to_bits(val)
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
    #[doc = "At least one bit transmitted as dominant is received as recessive."]
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
    #[doc = "At least one bit transmitted as recessive is received as dominant."]
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
    #[doc = "No such occurrence"]
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
    #[doc = "Interrupt disabled"]
    BUS_OFF_INT_DISABLED = 0x0,
    #[doc = "Interrupt enabled"]
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
    #[doc = "Enabled"]
    AUTO_RECOVER_ENABLED = 0x0,
    #[doc = "Disabled"]
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
    #[doc = "MB0 has no occurrence of successfully completed transmission or reception."]
    BUFFER_TX_RX_NOT_COMPLETE = 0x0,
    #[doc = "MB0 has successfully completed transmission or reception."]
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
    #[doc = "No occurrence of completed transmission or reception, or no frames available"]
    ID1 = 0x0,
    #[doc = "MB5 completed transmission or reception, or frames available"]
    ID2 = 0x01,
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
    #[doc = "No occurrence of MB6 completing transmission or reception, or FIFO not almost full."]
    ID1 = 0x0,
    #[doc = "MB6 completed transmission or reception, or FIFO almost full."]
    ID2 = 0x01,
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
    #[doc = "No occurrence of MB7 completing transmission or reception, or no FIFO overflow."]
    ID1 = 0x0,
    #[doc = "MB7 completed transmission or reception, or FIFO overflow."]
    ID2 = 0x01,
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
    #[doc = "No error"]
    CRC_ERROR_NO = 0x0,
    #[doc = "Error occurred since last read of this register."]
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
    #[doc = "Disable"]
    ID1 = 0x0,
    #[doc = "Enable"]
    ID2 = 0x01,
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
    #[doc = "Disable"]
    LOW_POWER_DOZE_DISABLED = 0x0,
    #[doc = "Enable"]
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
    #[doc = "Disable"]
    RTR_COMPARE_NO = 0x0,
    #[doc = "Enable"]
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
    #[doc = "Enabled"]
    ENABLE = 0x0,
    #[doc = "Disabled"]
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
pub enum Erfclr {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clear enhanced RX FIFO content"]
    CLEAR = 0x01,
}
impl Erfclr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Erfclr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Erfclr {
    #[inline(always)]
    fn from(val: u8) -> Erfclr {
        Erfclr::from_bits(val)
    }
}
impl From<Erfclr> for u8 {
    #[inline(always)]
    fn from(val: Erfclr) -> u8 {
        Erfclr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Erfwmi {
    #[doc = "No such occurrence"]
    WATERMARK_NO = 0x0,
    #[doc = "Number of messages in FIFO is greater than the watermark"]
    WATERMARK_YES = 0x01,
}
impl Erfwmi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Erfwmi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Erfwmi {
    #[inline(always)]
    fn from(val: u8) -> Erfwmi {
        Erfwmi::from_bits(val)
    }
}
impl From<Erfwmi> for u8 {
    #[inline(always)]
    fn from(val: Erfwmi) -> u8 {
        Erfwmi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ErrintFast {
    #[doc = "No such occurrence."]
    ERRORS_DATA_PHASE_NO = 0x0,
    #[doc = "Error flag set in the data phase of CAN FD frames that have BRS = 1."]
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
    #[doc = "Interrupt disabled"]
    ERROR_INT_DISABLED = 0x0,
    #[doc = "Interrupt enabled"]
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
    #[doc = "No overrun"]
    OVERRUN_NOT_OCCURRED = 0x0,
    #[doc = "Overrun"]
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
pub enum Etdcfail {
    #[doc = "In range"]
    IN_RANGE = 0x0,
    #[doc = "Out of range"]
    OUT_OF_RANGE = 0x01,
}
impl Etdcfail {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Etdcfail {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Etdcfail {
    #[inline(always)]
    fn from(val: u8) -> Etdcfail {
        Etdcfail::from_bits(val)
    }
}
impl From<Etdcfail> for u8 {
    #[inline(always)]
    fn from(val: Etdcfail) -> u8 {
        Etdcfail::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fcs {
    #[doc = "Message ID filtering only"]
    ID_FILTERING = 0x0,
    #[doc = "Message ID filtering and payload filtering"]
    ID_PAYLOAD_FILTERING = 0x01,
    #[doc = "Message ID filtering occurring a specified number of times"]
    ID_FILTERING_NUMBER = 0x02,
    #[doc = "Message ID filtering and payload filtering a specified number of times"]
    ID_PAYLOAD_FILTERING_NUMBER = 0x03,
}
impl Fcs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fcs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fcs {
    #[inline(always)]
    fn from(val: u8) -> Fcs {
        Fcs::from_bits(val)
    }
}
impl From<Fcs> for u8 {
    #[inline(always)]
    fn from(val: Fcs) -> u8 {
        Fcs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fden {
    #[doc = "Disable"]
    CAN_FD_DISABLED = 0x0,
    #[doc = "Enable"]
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
    #[doc = "Disable"]
    NOMINAL = 0x0,
    #[doc = "Enable"]
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
pub enum FltIde {
    #[doc = "Standard"]
    STANDARD = 0x0,
    #[doc = "Extended"]
    EXTENDED = 0x01,
}
impl FltIde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FltIde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FltIde {
    #[inline(always)]
    fn from(val: u8) -> FltIde {
        FltIde::from_bits(val)
    }
}
impl From<FltIde> for u8 {
    #[inline(always)]
    fn from(val: FltIde) -> u8 {
        FltIde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FltRtr {
    #[doc = "Reject remote frame (accept data frame)"]
    REJECT = 0x0,
    #[doc = "Accept remote frame"]
    ACCEPT = 0x01,
}
impl FltRtr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FltRtr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FltRtr {
    #[inline(always)]
    fn from(val: u8) -> FltRtr {
        FltRtr::from_bits(val)
    }
}
impl From<FltRtr> for u8 {
    #[inline(always)]
    fn from(val: FltRtr) -> u8 {
        FltRtr::to_bits(val)
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
    #[doc = "No error"]
    FORM_ERROR_NO = 0x0,
    #[doc = "Error occurred since last read of this register."]
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
    #[doc = "Disable"]
    FREEZE_MODE_DISABLED = 0x0,
    #[doc = "Enable"]
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
    #[doc = "Not in Freeze mode, prescaler running."]
    FREEZE_MODE_NO = 0x0,
    #[doc = "In Freeze mode, prescaler stopped."]
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
    #[doc = "No request"]
    HALT_DISABLE = 0x0,
    #[doc = "Enter Freeze mode, if MCR\\[FRZ\\] = 1."]
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
pub enum Ide {
    #[doc = "Standard"]
    STANDARD = 0x0,
    #[doc = "Extended"]
    EXTENDED = 0x01,
}
impl Ide {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ide {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ide {
    #[inline(always)]
    fn from(val: u8) -> Ide {
        Ide::from_bits(val)
    }
}
impl From<Ide> for u8 {
    #[inline(always)]
    fn from(val: Ide) -> u8 {
        Ide::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IdeMsk {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    FRAME_FORMAT_NO = 0x0,
    #[doc = "The corresponding bit in the filter is checked."]
    FRAME_FORMAT_YES = 0x01,
}
impl IdeMsk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IdeMsk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IdeMsk {
    #[inline(always)]
    fn from(val: u8) -> IdeMsk {
        IdeMsk::from_bits(val)
    }
}
impl From<IdeMsk> for u8 {
    #[inline(always)]
    fn from(val: IdeMsk) -> u8 {
        IdeMsk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Idfs {
    #[doc = "Match ID contents to an exact target value"]
    MATCH_EXACT = 0x0,
    #[doc = "Match an ID value greater than or equal to a specified target value"]
    MATCH_GTE = 0x01,
    #[doc = "Match an ID value smaller than or equal to a specified target value"]
    MATCH_LTE = 0x02,
    #[doc = "Match an ID value within a range of values, inclusive"]
    MATCH_RANGE = 0x03,
}
impl Idfs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Idfs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Idfs {
    #[inline(always)]
    fn from(val: u8) -> Idfs {
        Idfs::from_bits(val)
    }
}
impl From<Idfs> for u8 {
    #[inline(always)]
    fn from(val: Idfs) -> u8 {
        Idfs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Idle {
    #[doc = "Not IDLE"]
    CAN_BUS_NOT_IDLE = 0x0,
    #[doc = "IDLE"]
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
    #[doc = "Message buffer indicated by ESR2\\[LPTM\\] is not inactive."]
    INACTIVE_MAILBOX_NO = 0x0,
    #[doc = "At least one message buffer is inactive."]
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
    #[doc = "Disable"]
    INDIVIDUAL_RX_MASKING_DISABLED = 0x0,
    #[doc = "Enable"]
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
    #[doc = "Disable"]
    NON_ISO = 0x0,
    #[doc = "Enable"]
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
    #[doc = "Disabled"]
    LOOPBACK_DISABLED = 0x0,
    #[doc = "Enabled"]
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
    #[doc = "Not in a low-power mode"]
    LOW_POWER_NO = 0x0,
    #[doc = "In a low-power mode"]
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
    #[doc = "Disable"]
    LOCAL_PRIORITY_DISABLED = 0x0,
    #[doc = "Enable"]
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
    #[doc = "8 bytes"]
    R0_8_BYTES = 0x0,
    #[doc = "16 bytes"]
    R0_16_BYTES = 0x01,
    #[doc = "32 bytes"]
    R0_32_BYTES = 0x02,
    #[doc = "64 bytes"]
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
pub enum Mdis {
    #[doc = "Enable"]
    FLEXCAN_ENABLED = 0x0,
    #[doc = "Disable"]
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
    #[doc = "Matching starts from Legacy RX FIFO or Enhanced RX FIFO and continues on message buffers."]
    ID1 = 0x0,
    #[doc = "Matching starts from message buffers and continues on Legacy RX FIFO or Enhanced RX FIFO."]
    ID3 = 0x01,
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Nmatch(u8);
impl Nmatch {
    #[doc = "Once"]
    pub const MATCH_1: Self = Self(0x01);
    #[doc = "Twice"]
    pub const MATCH_2: Self = Self(0x02);
    #[doc = "255 times"]
    pub const MATCH_255: Self = Self(0xff);
}
impl Nmatch {
    pub const fn from_bits(val: u8) -> Nmatch {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Nmatch {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("MATCH_1"),
            0x02 => f.write_str("MATCH_2"),
            0xff => f.write_str("MATCH_255"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nmatch {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "MATCH_1"),
            0x02 => defmt::write!(f, "MATCH_2"),
            0xff => defmt::write!(f, "MATCH_255"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Nmatch {
    #[inline(always)]
    fn from(val: u8) -> Nmatch {
        Nmatch::from_bits(val)
    }
}
impl From<Nmatch> for u8 {
    #[inline(always)]
    fn from(val: Nmatch) -> u8 {
        Nmatch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Notrdy {
    #[doc = "FlexCAN is in Normal mode, Listen-Only mode, or Loopback mode."]
    ID1 = 0x0,
    #[doc = "FlexCAN is in Disable mode, Doze mode, Stop mode, or Freeze mode."]
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
pub enum Pes {
    #[doc = "Big-endian"]
    BIG_END = 0x0,
    #[doc = "Little-endian"]
    LITTLE_END = 0x01,
}
impl Pes {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pes {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pes {
    #[inline(always)]
    fn from(val: u8) -> Pes {
        Pes::from_bits(val)
    }
}
impl From<Pes> for u8 {
    #[inline(always)]
    fn from(val: Pes) -> u8 {
        Pes::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Plfs {
    #[doc = "Match payload contents to an exact target value"]
    MATCH_EXACT = 0x0,
    #[doc = "Match a payload value greater than or equal to a specified target value"]
    MATCH_GTE = 0x01,
    #[doc = "Match a payload value smaller than or equal to a specified target value"]
    MATCH_LTE = 0x02,
    #[doc = "Match upon a payload value within a range of values, inclusive"]
    MATCH_RANGE = 0x03,
}
impl Plfs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Plfs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Plfs {
    #[inline(always)]
    fn from(val: u8) -> Plfs {
        Plfs::from_bits(val)
    }
}
impl From<Plfs> for u8 {
    #[inline(always)]
    fn from(val: Plfs) -> u8 {
        Plfs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PnetEn {
    #[doc = "Disable"]
    PN_DISABLED = 0x0,
    #[doc = "Enable"]
    PN_ENABLED = 0x01,
}
impl PnetEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PnetEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PnetEn {
    #[inline(always)]
    fn from(val: u8) -> PnetEn {
        PnetEn::from_bits(val)
    }
}
impl From<PnetEn> for u8 {
    #[inline(always)]
    fn from(val: PnetEn) -> u8 {
        PnetEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rfen {
    #[doc = "Disable"]
    ID1 = 0x0,
    #[doc = "Enable"]
    ID2 = 0x01,
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
    #[doc = "Generated"]
    REMOTE_RESPONSE_FRAME_NOT_GENERATED = 0x0,
    #[doc = "Stored"]
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
pub enum RtrMsk {
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    FRAME_TYPE_NO = 0x0,
    #[doc = "The corresponding bit in the filter is checked."]
    FRAME_TYPE_YES = 0x01,
}
impl RtrMsk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RtrMsk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RtrMsk {
    #[inline(always)]
    fn from(val: u8) -> RtrMsk {
        RtrMsk::from_bits(val)
    }
}
impl From<RtrMsk> for u8 {
    #[inline(always)]
    fn from(val: RtrMsk) -> u8 {
        RtrMsk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rwrnint {
    #[doc = "No such occurrence"]
    RX_WARNING_INT_NO = 0x0,
    #[doc = "RX error counter changed from less than 96 to greater than or equal to 96."]
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
    #[doc = "Disabled"]
    RX_WARNING_INT_DISABLED = 0x0,
    #[doc = "Enabled"]
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
    #[doc = "Disable"]
    SELF_WAKEUP_DISABLED = 0x0,
    #[doc = "Enable"]
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
    #[doc = "One sample is used to determine the bit value."]
    ONE_SAMPLE = 0x0,
    #[doc = "Three samples are used to determine the value of the received bit: the regular one (sample point) and two preceding samples. A majority rule is used."]
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
    #[doc = "No reset"]
    SOFTRST_NO_RESET_REQUEST = 0x0,
    #[doc = "Soft reset affects reset registers"]
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
pub enum Srr {
    #[doc = "Dominant"]
    DOMINANT = 0x0,
    #[doc = "Recessive"]
    RECESSIVE = 0x01,
}
impl Srr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Srr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Srr {
    #[inline(always)]
    fn from(val: u8) -> Srr {
        Srr::from_bits(val)
    }
}
impl From<Srr> for u8 {
    #[inline(always)]
    fn from(val: Srr) -> u8 {
        Srr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Srxdis {
    #[doc = "Enable"]
    SELF_RECEPTION_ENABLED = 0x0,
    #[doc = "Disable"]
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
    #[doc = "No error"]
    STUFFING_ERROR_NO = 0x0,
    #[doc = "Error occurred since last read of this register."]
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
    #[doc = "User mode"]
    ID1 = 0x0,
    #[doc = "Supervisor mode"]
    ID2 = 0x01,
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
    #[doc = "Not synchronized"]
    CAN_BUS_SYNC_NO = 0x0,
    #[doc = "Synchronized"]
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
    #[doc = "In range"]
    IN_RANGE = 0x0,
    #[doc = "Out of range"]
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
pub enum Tdmdis {
    #[doc = "Enable"]
    ENABLE = 0x0,
    #[doc = "Disable"]
    DISABLE = 0x01,
}
impl Tdmdis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tdmdis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tdmdis {
    #[inline(always)]
    fn from(val: u8) -> Tdmdis {
        Tdmdis::from_bits(val)
    }
}
impl From<Tdmdis> for u8 {
    #[inline(always)]
    fn from(val: Tdmdis) -> u8 {
        Tdmdis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsyn {
    #[doc = "Disable"]
    TIMER_SYNC_DISABLED = 0x0,
    #[doc = "Enable"]
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
    #[doc = "No such occurrence"]
    TX_WARNING_INT_NO = 0x0,
    #[doc = "TX error counter changed from less than 96 to greater than or equal to 96."]
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
    #[doc = "Disabled"]
    TX_WARNING_INT_DISABLED = 0x0,
    #[doc = "Enabled"]
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
    #[doc = "Not transmitting"]
    TRANSMIT_MESSAGE_NO = 0x0,
    #[doc = "Transmitting"]
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
    #[doc = "TXERRCNT is 96 or greater."]
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
    #[doc = "Disabled"]
    WAKEUP_INTERRUPT_DISABLED = 0x0,
    #[doc = "Enabled"]
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
    #[doc = "Disable"]
    TWRNINT_RWRNINT_INACTIVE = 0x0,
    #[doc = "Enable"]
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
