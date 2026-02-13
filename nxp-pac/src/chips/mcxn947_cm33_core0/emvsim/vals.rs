#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BgtErr {
    #[doc = "Sufficient"]
    BGT_ERR_SUFFICIENT = 0x0,
    #[doc = "Too small"]
    BGT_ERR_TOOSMALL = 0x01,
}
impl BgtErr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BgtErr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BgtErr {
    #[inline(always)]
    fn from(val: u8) -> BgtErr {
        BgtErr::from_bits(val)
    }
}
impl From<BgtErr> for u8 {
    #[inline(always)]
    fn from(val: BgtErr) -> u8 {
        BgtErr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BgtErrIm {
    #[doc = "Enable"]
    INT_ENABLED = 0x0,
    #[doc = "Masked"]
    INT_MASKED = 0x01,
}
impl BgtErrIm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BgtErrIm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BgtErrIm {
    #[inline(always)]
    fn from(val: u8) -> BgtErrIm {
        BgtErrIm::from_bits(val)
    }
}
impl From<BgtErrIm> for u8 {
    #[inline(always)]
    fn from(val: BgtErrIm) -> u8 {
        BgtErrIm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BwtErr {
    #[doc = "Not exceeded"]
    BWT_ERR_NO = 0x0,
    #[doc = "Exceeded"]
    BWT_ERR_YES = 0x01,
}
impl BwtErr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BwtErr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BwtErr {
    #[inline(always)]
    fn from(val: u8) -> BwtErr {
        BwtErr::from_bits(val)
    }
}
impl From<BwtErr> for u8 {
    #[inline(always)]
    fn from(val: BwtErr) -> u8 {
        BwtErr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BwtErrIm {
    #[doc = "Enable"]
    INT_ENABLED = 0x0,
    #[doc = "Masked"]
    INT_MASKED = 0x01,
}
impl BwtErrIm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BwtErrIm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BwtErrIm {
    #[inline(always)]
    fn from(val: u8) -> BwtErrIm {
        BwtErrIm::from_bits(val)
    }
}
impl From<BwtErrIm> for u8 {
    #[inline(always)]
    fn from(val: BwtErrIm) -> u8 {
        BwtErrIm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CrcOk {
    #[doc = "Current CRC value does not match remainder."]
    CRC_NOTOK = 0x0,
    #[doc = "Current calculated CRC value matches the expected result."]
    CRC_OK = 0x01,
}
impl CrcOk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CrcOk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CrcOk {
    #[inline(always)]
    fn from(val: u8) -> CrcOk {
        CrcOk::from_bits(val)
    }
}
impl From<CrcOk> for u8 {
    #[inline(always)]
    fn from(val: CrcOk) -> u8 {
        CrcOk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CwtErrIm {
    #[doc = "Enable"]
    INT_ENABLED = 0x0,
    #[doc = "Masked"]
    INT_DISABLED = 0x01,
}
impl CwtErrIm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CwtErrIm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CwtErrIm {
    #[inline(always)]
    fn from(val: u8) -> CwtErrIm {
        CwtErrIm::from_bits(val)
    }
}
impl From<CwtErrIm> for u8 {
    #[inline(always)]
    fn from(val: CwtErrIm) -> u8 {
        CwtErrIm::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct DivisorValue(u16);
impl DivisorValue {
    #[doc = "Invalid. As per ISO 7816 specification, the minimum value of F/D is 5."]
    pub const INVALID_0: Self = Self(0x0);
    #[doc = "Invalid. As per ISO 7816 specification, the minimum value of F/D is 5."]
    pub const INVALID_1: Self = Self(0x01);
    #[doc = "Invalid. As per ISO 7816 specification, the minimum value of F/D is 5."]
    pub const INVALID_2: Self = Self(0x02);
    #[doc = "Invalid. As per ISO 7816 specification, the minimum value of F/D is 5."]
    pub const INVALID_3: Self = Self(0x03);
    #[doc = "Invalid. As per ISO 7816 specification, the minimum value of F/D is 5."]
    pub const INVALID_4: Self = Self(0x04);
    #[doc = "Divisor value F/D"]
    pub const VALID_5: Self = Self(0x05);
    #[doc = "Divisor value F/D"]
    pub const VALID_6: Self = Self(0x06);
    #[doc = "Divisor value F/D"]
    pub const VALID_7: Self = Self(0x07);
    #[doc = "Divisor value F/D"]
    pub const VALID_8: Self = Self(0x08);
    #[doc = "Divisor value F/D"]
    pub const VALID_9: Self = Self(0x09);
}
impl DivisorValue {
    pub const fn from_bits(val: u16) -> DivisorValue {
        Self(val & 0x01ff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for DivisorValue {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("INVALID_0"),
            0x01 => f.write_str("INVALID_1"),
            0x02 => f.write_str("INVALID_2"),
            0x03 => f.write_str("INVALID_3"),
            0x04 => f.write_str("INVALID_4"),
            0x05 => f.write_str("VALID_5"),
            0x06 => f.write_str("VALID_6"),
            0x07 => f.write_str("VALID_7"),
            0x08 => f.write_str("VALID_8"),
            0x09 => f.write_str("VALID_9"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DivisorValue {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "INVALID_0"),
            0x01 => defmt::write!(f, "INVALID_1"),
            0x02 => defmt::write!(f, "INVALID_2"),
            0x03 => defmt::write!(f, "INVALID_3"),
            0x04 => defmt::write!(f, "INVALID_4"),
            0x05 => defmt::write!(f, "VALID_5"),
            0x06 => defmt::write!(f, "VALID_6"),
            0x07 => defmt::write!(f, "VALID_7"),
            0x08 => defmt::write!(f, "VALID_8"),
            0x09 => defmt::write!(f, "VALID_9"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for DivisorValue {
    #[inline(always)]
    fn from(val: u16) -> DivisorValue {
        DivisorValue::from_bits(val)
    }
}
impl From<DivisorValue> for u16 {
    #[inline(always)]
    fn from(val: DivisorValue) -> u16 {
        DivisorValue::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DozeEn {
    #[doc = "Disable"]
    DOZE_GATE = 0x0,
    #[doc = "Enable"]
    DOZE_NOGATE = 0x01,
}
impl DozeEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DozeEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DozeEn {
    #[inline(always)]
    fn from(val: u8) -> DozeEn {
        DozeEn::from_bits(val)
    }
}
impl From<DozeEn> for u8 {
    #[inline(always)]
    fn from(val: DozeEn) -> u8 {
        DozeEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EtcIm {
    #[doc = "Enable"]
    INT_ENABLED = 0x0,
    #[doc = "Masked"]
    INT_MASKED = 0x01,
}
impl EtcIm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EtcIm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EtcIm {
    #[inline(always)]
    fn from(val: u8) -> EtcIm {
        EtcIm::from_bits(val)
    }
}
impl From<EtcIm> for u8 {
    #[inline(always)]
    fn from(val: EtcIm) -> u8 {
        EtcIm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Etcf {
    #[doc = "Pending or incomplete"]
    ETX_PENDING = 0x0,
    #[doc = "Complete"]
    ETX_COMPLETE = 0x01,
}
impl Etcf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Etcf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Etcf {
    #[inline(always)]
    fn from(val: u8) -> Etcf {
        Etcf::from_bits(val)
    }
}
impl From<Etcf> for u8 {
    #[inline(always)]
    fn from(val: Etcf) -> u8 {
        Etcf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlshRx {
    #[doc = "Normal"]
    NORMALOP = 0x0,
    #[doc = "Reset"]
    RESETHOLD = 0x01,
}
impl FlshRx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlshRx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlshRx {
    #[inline(always)]
    fn from(val: u8) -> FlshRx {
        FlshRx::from_bits(val)
    }
}
impl From<FlshRx> for u8 {
    #[inline(always)]
    fn from(val: FlshRx) -> u8 {
        FlshRx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlshTx {
    #[doc = "Normal"]
    NORMALOP = 0x0,
    #[doc = "Reset"]
    RESETHOLD = 0x01,
}
impl FlshTx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlshTx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlshTx {
    #[inline(always)]
    fn from(val: u8) -> FlshTx {
        FlshTx::from_bits(val)
    }
}
impl From<FlshTx> for u8 {
    #[inline(always)]
    fn from(val: FlshTx) -> u8 {
        FlshTx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpcnt0ClkSel {
    #[doc = "Disable/reset"]
    DISABLED = 0x0,
    #[doc = "Card clock"]
    CARDCLK = 0x01,
    #[doc = "Receive clock"]
    RXCLK = 0x02,
    #[doc = "ETU clock (transmit clock)"]
    TXCLK = 0x03,
}
impl Gpcnt0ClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpcnt0ClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpcnt0ClkSel {
    #[inline(always)]
    fn from(val: u8) -> Gpcnt0ClkSel {
        Gpcnt0ClkSel::from_bits(val)
    }
}
impl From<Gpcnt0ClkSel> for u8 {
    #[inline(always)]
    fn from(val: Gpcnt0ClkSel) -> u8 {
        Gpcnt0ClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpcnt0Im {
    #[doc = "Enable"]
    INT_ENABLED = 0x0,
    #[doc = "Masked"]
    INT_MASKED = 0x01,
}
impl Gpcnt0Im {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpcnt0Im {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpcnt0Im {
    #[inline(always)]
    fn from(val: u8) -> Gpcnt0Im {
        Gpcnt0Im::from_bits(val)
    }
}
impl From<Gpcnt0Im> for u8 {
    #[inline(always)]
    fn from(val: Gpcnt0Im) -> u8 {
        Gpcnt0Im::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpcnt0To {
    #[doc = "GPCNT0 not reached, or flag cleared"]
    GPCNT0_TO_NOTREACHED = 0x0,
    #[doc = "GPCNT0 reached"]
    GPCNT0_TO_REACHED = 0x01,
}
impl Gpcnt0To {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpcnt0To {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpcnt0To {
    #[inline(always)]
    fn from(val: u8) -> Gpcnt0To {
        Gpcnt0To::from_bits(val)
    }
}
impl From<Gpcnt0To> for u8 {
    #[inline(always)]
    fn from(val: Gpcnt0To) -> u8 {
        Gpcnt0To::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpcnt1ClkSel {
    #[doc = "Disable/reset"]
    DISABLED = 0x0,
    #[doc = "Card clock"]
    CARDCLK = 0x01,
    #[doc = "Receive clock"]
    RXCLK = 0x02,
    #[doc = "ETU clock (transmit clock)"]
    TXCLK = 0x03,
}
impl Gpcnt1ClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpcnt1ClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpcnt1ClkSel {
    #[inline(always)]
    fn from(val: u8) -> Gpcnt1ClkSel {
        Gpcnt1ClkSel::from_bits(val)
    }
}
impl From<Gpcnt1ClkSel> for u8 {
    #[inline(always)]
    fn from(val: Gpcnt1ClkSel) -> u8 {
        Gpcnt1ClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpcnt1Im {
    #[doc = "Enable"]
    INT_ENABLED = 0x0,
    #[doc = "Masked"]
    INT_MASKED = 0x01,
}
impl Gpcnt1Im {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpcnt1Im {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpcnt1Im {
    #[inline(always)]
    fn from(val: u8) -> Gpcnt1Im {
        Gpcnt1Im::from_bits(val)
    }
}
impl From<Gpcnt1Im> for u8 {
    #[inline(always)]
    fn from(val: Gpcnt1Im) -> u8 {
        Gpcnt1Im::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpcnt1To {
    #[doc = "GPCNT1 not reached, or flag cleared"]
    GPCNT1_TO_NOTREACHED = 0x0,
    #[doc = "GPCNT1 reached"]
    GPCNT1_TO_REACHED = 0x01,
}
impl Gpcnt1To {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpcnt1To {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpcnt1To {
    #[inline(always)]
    fn from(val: u8) -> Gpcnt1To {
        Gpcnt1To::from_bits(val)
    }
}
impl From<Gpcnt1To> for u8 {
    #[inline(always)]
    fn from(val: Gpcnt1To) -> u8 {
        Gpcnt1To::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ic {
    #[doc = "Direct"]
    DIR_CONVENTION = 0x0,
    #[doc = "Inverse"]
    INV_CONVENTION = 0x01,
}
impl Ic {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ic {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ic {
    #[inline(always)]
    fn from(val: u8) -> Ic {
        Ic::from_bits(val)
    }
}
impl From<Ic> for u8 {
    #[inline(always)]
    fn from(val: Ic) -> u8 {
        Ic::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KillClocks {
    #[doc = "Enable"]
    INCLK_ENABLED = 0x0,
    #[doc = "Disable"]
    INCLK_DISABLED = 0x01,
}
impl KillClocks {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KillClocks {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KillClocks {
    #[inline(always)]
    fn from(val: u8) -> KillClocks {
        KillClocks::from_bits(val)
    }
}
impl From<KillClocks> for u8 {
    #[inline(always)]
    fn from(val: KillClocks) -> u8 {
        KillClocks::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LrcOk {
    #[doc = "No match"]
    LRC_NOTOK = 0x0,
    #[doc = "Match"]
    LRC_OK = 0x01,
}
impl LrcOk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LrcOk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LrcOk {
    #[inline(always)]
    fn from(val: u8) -> LrcOk {
        LrcOk::from_bits(val)
    }
}
impl From<LrcOk> for u8 {
    #[inline(always)]
    fn from(val: LrcOk) -> u8 {
        LrcOk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PefIm {
    #[doc = "Enable"]
    INT_ENABLED = 0x0,
    #[doc = "Masked"]
    INT_MASKED = 0x01,
}
impl PefIm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PefIm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PefIm {
    #[inline(always)]
    fn from(val: u8) -> PefIm {
        PefIm::from_bits(val)
    }
}
impl From<PefIm> for u8 {
    #[inline(always)]
    fn from(val: PefIm) -> u8 {
        PefIm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rcvr11 {
    #[doc = "12 ETU operation"]
    RCVR_12 = 0x0,
    #[doc = "11 ETU operation"]
    RCVR_11 = 0x01,
}
impl Rcvr11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcvr11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcvr11 {
    #[inline(always)]
    fn from(val: u8) -> Rcvr11 {
        Rcvr11::from_bits(val)
    }
}
impl From<Rcvr11> for u8 {
    #[inline(always)]
    fn from(val: Rcvr11) -> u8 {
        Rcvr11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RdtIm {
    #[doc = "Enable"]
    INT_ENABLED = 0x0,
    #[doc = "Masked"]
    INT_MASKED = 0x01,
}
impl RdtIm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RdtIm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RdtIm {
    #[inline(always)]
    fn from(val: u8) -> RdtIm {
        RdtIm::from_bits(val)
    }
}
impl From<RdtIm> for u8 {
    #[inline(always)]
    fn from(val: RdtIm) -> u8 {
        RdtIm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rdtf {
    #[doc = "Less than threshold"]
    LESSTHAN_RXTHRESH = 0x0,
    #[doc = "Greater than or equal to threshold"]
    GREATER_EQ_RXTHRESH = 0x01,
}
impl Rdtf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rdtf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rdtf {
    #[inline(always)]
    fn from(val: u8) -> Rdtf {
        Rdtf::from_bits(val)
    }
}
impl From<Rdtf> for u8 {
    #[inline(always)]
    fn from(val: Rdtf) -> u8 {
        Rdtf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rfo {
    #[doc = "No overrun error"]
    NO_OVERRUN = 0x0,
    #[doc = "Overrun error"]
    OVERFLOW = 0x01,
}
impl Rfo {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rfo {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rfo {
    #[inline(always)]
    fn from(val: u8) -> Rfo {
        Rfo::from_bits(val)
    }
}
impl From<Rfo> for u8 {
    #[inline(always)]
    fn from(val: Rfo) -> u8 {
        Rfo::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RfoIm {
    #[doc = "Enable"]
    INT_ENABLED = 0x0,
    #[doc = "Masked"]
    INT_MASKED = 0x01,
}
impl RfoIm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RfoIm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RfoIm {
    #[inline(always)]
    fn from(val: u8) -> RfoIm {
        RfoIm::from_bits(val)
    }
}
impl From<RfoIm> for u8 {
    #[inline(always)]
    fn from(val: RfoIm) -> u8 {
        RfoIm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RnackIm {
    #[doc = "Enable"]
    INT_ENABLED = 0x0,
    #[doc = "Masked"]
    INT_MASKED = 0x01,
}
impl RnackIm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RnackIm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RnackIm {
    #[inline(always)]
    fn from(val: u8) -> RnackIm {
        RnackIm::from_bits(val)
    }
}
impl From<RnackIm> for u8 {
    #[inline(always)]
    fn from(val: RnackIm) -> u8 {
        RnackIm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rte {
    #[doc = "Less than"]
    LESSTHAN_NACKTHRESH = 0x0,
    #[doc = "Equal to"]
    GREATER_EQ_NACKTHRESH = 0x01,
}
impl Rte {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rte {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rte {
    #[inline(always)]
    fn from(val: u8) -> Rte {
        Rte::from_bits(val)
    }
}
impl From<Rte> for u8 {
    #[inline(always)]
    fn from(val: Rte) -> u8 {
        Rte::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RxCnt {
    #[doc = "FIFO empty"]
    FIFO_EMPTY = 0x0,
    _RESERVED_1 = 0x01,
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
impl RxCnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxCnt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxCnt {
    #[inline(always)]
    fn from(val: u8) -> RxCnt {
        RxCnt::from_bits(val)
    }
}
impl From<RxCnt> for u8 {
    #[inline(always)]
    fn from(val: RxCnt) -> u8 {
        RxCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RxDataIm {
    #[doc = "Enable"]
    INT_ENABLED = 0x0,
    #[doc = "Masked"]
    INT_MASKED = 0x01,
}
impl RxDataIm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxDataIm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxDataIm {
    #[inline(always)]
    fn from(val: u8) -> RxDataIm {
        RxDataIm::from_bits(val)
    }
}
impl From<RxDataIm> for u8 {
    #[inline(always)]
    fn from(val: RxDataIm) -> u8 {
        RxDataIm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Scsp {
    #[doc = "Logic 0"]
    SCSP_LOGIC0 = 0x0,
    #[doc = "Logic 1"]
    SCSP_LOGIC1 = 0x01,
}
impl Scsp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Scsp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Scsp {
    #[inline(always)]
    fn from(val: u8) -> Scsp {
        Scsp::from_bits(val)
    }
}
impl From<Scsp> for u8 {
    #[inline(always)]
    fn from(val: Scsp) -> u8 {
        Scsp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spd {
    #[doc = "No"]
    NO_EFFECT = 0x0,
    #[doc = "Yes"]
    POWERDOWN = 0x01,
}
impl Spd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spd {
    #[inline(always)]
    fn from(val: u8) -> Spd {
        Spd::from_bits(val)
    }
}
impl From<Spd> for u8 {
    #[inline(always)]
    fn from(val: Spd) -> u8 {
        Spd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spdes {
    #[doc = "Falling edge"]
    FALLING_EDGE = 0x0,
    #[doc = "Rising edge"]
    RISING_EDGE = 0x01,
}
impl Spdes {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spdes {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spdes {
    #[inline(always)]
    fn from(val: u8) -> Spdes {
        Spdes::from_bits(val)
    }
}
impl From<Spdes> for u8 {
    #[inline(always)]
    fn from(val: Spdes) -> u8 {
        Spdes::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spdim {
    #[doc = "Enable"]
    INT_ENABLED = 0x0,
    #[doc = "Mask"]
    INT_MASKED = 0x01,
}
impl Spdim {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spdim {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spdim {
    #[inline(always)]
    fn from(val: u8) -> Spdim {
        Spdim::from_bits(val)
    }
}
impl From<Spdim> for u8 {
    #[inline(always)]
    fn from(val: Spdim) -> u8 {
        Spdim::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spdp {
    #[doc = "Logic low"]
    LOGIC_LOW = 0x0,
    #[doc = "Logic high"]
    LOGIC_HIGH = 0x01,
}
impl Spdp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spdp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spdp {
    #[inline(always)]
    fn from(val: u8) -> Spdp {
        Spdp::from_bits(val)
    }
}
impl From<Spdp> for u8 {
    #[inline(always)]
    fn from(val: Spdp) -> u8 {
        Spdp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Srst {
    #[doc = "Assert"]
    ASSERTED = 0x0,
    #[doc = "Deassert"]
    DE_ASSERTED = 0x01,
}
impl Srst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Srst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Srst {
    #[inline(always)]
    fn from(val: u8) -> Srst {
        Srst::from_bits(val)
    }
}
impl From<Srst> for u8 {
    #[inline(always)]
    fn from(val: Srst) -> u8 {
        Srst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StopEn {
    #[doc = "Disable"]
    STOP_ALL_CLKS = 0x0,
    #[doc = "Enable"]
    ONLY_SCK_ON = 0x01,
}
impl StopEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StopEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StopEn {
    #[inline(always)]
    fn from(val: u8) -> StopEn {
        StopEn::from_bits(val)
    }
}
impl From<StopEn> for u8 {
    #[inline(always)]
    fn from(val: StopEn) -> u8 {
        StopEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwRst {
    #[doc = "Normal"]
    NORMALOP = 0x0,
    #[doc = "Reset"]
    RESETHOLD = 0x01,
}
impl SwRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwRst {
    #[inline(always)]
    fn from(val: u8) -> SwRst {
        SwRst::from_bits(val)
    }
}
impl From<SwRst> for u8 {
    #[inline(always)]
    fn from(val: SwRst) -> u8 {
        SwRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcIm {
    #[doc = "Enable"]
    INT_ENABLED = 0x0,
    #[doc = "Masked"]
    INT_MASKED = 0x01,
}
impl TcIm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcIm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcIm {
    #[inline(always)]
    fn from(val: u8) -> TcIm {
        TcIm::from_bits(val)
    }
}
impl From<TcIm> for u8 {
    #[inline(always)]
    fn from(val: TcIm) -> u8 {
        TcIm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcf {
    #[doc = "Pending or incomplete"]
    TX_PENDING = 0x0,
    #[doc = "Complete"]
    TX_COMPLETE = 0x01,
}
impl Tcf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcf {
    #[inline(always)]
    fn from(val: u8) -> Tcf {
        Tcf::from_bits(val)
    }
}
impl From<Tcf> for u8 {
    #[inline(always)]
    fn from(val: Tcf) -> u8 {
        Tcf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TdtIm {
    #[doc = "Enable"]
    INT_ENABLED = 0x0,
    #[doc = "Masked"]
    INT_MASKED = 0x01,
}
impl TdtIm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TdtIm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TdtIm {
    #[inline(always)]
    fn from(val: u8) -> TdtIm {
        TdtIm::from_bits(val)
    }
}
impl From<TdtIm> for u8 {
    #[inline(always)]
    fn from(val: TdtIm) -> u8 {
        TdtIm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tdtf {
    #[doc = "Threshold exceeded or this field written to 0"]
    LESSTHAN_TXTHRESH = 0x0,
    #[doc = "Threshold not exceeded"]
    GREATER_EQ_TXTHRESH = 0x01,
}
impl Tdtf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tdtf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tdtf {
    #[inline(always)]
    fn from(val: u8) -> Tdtf {
        Tdtf::from_bits(val)
    }
}
impl From<Tdtf> for u8 {
    #[inline(always)]
    fn from(val: Tdtf) -> u8 {
        Tdtf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tfe {
    #[doc = "Not empty"]
    FIFO_EMPTY = 0x0,
    #[doc = "Empty"]
    FIFO_NOTEMPTY = 0x01,
}
impl Tfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tfe {
    #[inline(always)]
    fn from(val: u8) -> Tfe {
        Tfe::from_bits(val)
    }
}
impl From<Tfe> for u8 {
    #[inline(always)]
    fn from(val: Tfe) -> u8 {
        Tfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TfeIm {
    #[doc = "Enable"]
    INT_ENABLED = 0x0,
    #[doc = "Masked"]
    INT_MASKED = 0x01,
}
impl TfeIm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TfeIm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TfeIm {
    #[inline(always)]
    fn from(val: u8) -> TfeIm {
        TfeIm::from_bits(val)
    }
}
impl From<TfeIm> for u8 {
    #[inline(always)]
    fn from(val: TfeIm) -> u8 {
        TfeIm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tff {
    #[doc = "Not full"]
    TX_FIFO_NOTFULL = 0x0,
    #[doc = "Full"]
    TX_FIFO_FULL = 0x01,
}
impl Tff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tff {
    #[inline(always)]
    fn from(val: u8) -> Tff {
        Tff::from_bits(val)
    }
}
impl From<Tff> for u8 {
    #[inline(always)]
    fn from(val: Tff) -> u8 {
        Tff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TffIm {
    #[doc = "Enable"]
    INT_ENABLED = 0x0,
    #[doc = "Masked"]
    INT_MASKED = 0x01,
}
impl TffIm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TffIm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TffIm {
    #[inline(always)]
    fn from(val: u8) -> TffIm {
        TffIm::from_bits(val)
    }
}
impl From<TffIm> for u8 {
    #[inline(always)]
    fn from(val: TffIm) -> u8 {
        TffIm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TnackIm {
    #[doc = "Enable"]
    INT_ENABLED = 0x0,
    #[doc = "Masked"]
    INT_MASKED = 0x01,
}
impl TnackIm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TnackIm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TnackIm {
    #[inline(always)]
    fn from(val: u8) -> TnackIm {
        TnackIm::from_bits(val)
    }
}
impl From<TnackIm> for u8 {
    #[inline(always)]
    fn from(val: TnackIm) -> u8 {
        TnackIm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tnte {
    #[doc = "Threshold not reached"]
    LESSTHAN_NACKTHRESH = 0x0,
    #[doc = "Threshold reached"]
    GREATER_EQ_NACKTHRESH = 0x01,
}
impl Tnte {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tnte {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tnte {
    #[inline(always)]
    fn from(val: u8) -> Tnte {
        Tnte::from_bits(val)
    }
}
impl From<Tnte> for u8 {
    #[inline(always)]
    fn from(val: Tnte) -> u8 {
        Tnte::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TxCnt {
    #[doc = "FIFO empty"]
    FIFO_EMPTY = 0x0,
    _RESERVED_1 = 0x01,
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
impl TxCnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TxCnt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TxCnt {
    #[inline(always)]
    fn from(val: u8) -> TxCnt {
        TxCnt::from_bits(val)
    }
}
impl From<TxCnt> for u8 {
    #[inline(always)]
    fn from(val: TxCnt) -> u8 {
        TxCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vccenp {
    #[doc = "Active high"]
    ACTIVE_HIGH = 0x0,
    #[doc = "Active low"]
    ACTIVE_LOW = 0x01,
}
impl Vccenp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vccenp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vccenp {
    #[inline(always)]
    fn from(val: u8) -> Vccenp {
        Vccenp::from_bits(val)
    }
}
impl From<Vccenp> for u8 {
    #[inline(always)]
    fn from(val: Vccenp) -> u8 {
        Vccenp::to_bits(val)
    }
}
