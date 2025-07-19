#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Autocmd12ErrStatusSmpClkSel {
    #[doc = "Fixed clock is used to sample data"]
    SMP_CLK_B = 0x0,
    #[doc = "Tuned clock is used to sample data"]
    SMP_CLK_A = 0x01,
}
impl Autocmd12ErrStatusSmpClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Autocmd12ErrStatusSmpClkSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Autocmd12ErrStatusSmpClkSel {
    #[inline(always)]
    fn from(val: u8) -> Autocmd12ErrStatusSmpClkSel {
        Autocmd12ErrStatusSmpClkSel::from_bits(val)
    }
}
impl From<Autocmd12ErrStatusSmpClkSel> for u8 {
    #[inline(always)]
    fn from(val: Autocmd12ErrStatusSmpClkSel) -> u8 {
        Autocmd12ErrStatusSmpClkSel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Blkcnt(u16);
impl Blkcnt {
    #[doc = "Stop count"]
    pub const BLKCNT_D: Self = Self(0x0);
    #[doc = "1 block"]
    pub const BLKCNT_C: Self = Self(0x01);
    #[doc = "2 blocks"]
    pub const BLKCNT_B: Self = Self(0x02);
    #[doc = "65535 blocks"]
    pub const BLKCNT_A: Self = Self(0xffff);
}
impl Blkcnt {
    pub const fn from_bits(val: u16) -> Blkcnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Blkcnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("BLKCNT_D"),
            0x01 => f.write_str("BLKCNT_C"),
            0x02 => f.write_str("BLKCNT_B"),
            0xffff => f.write_str("BLKCNT_A"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Blkcnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "BLKCNT_D"),
            0x01 => defmt::write!(f, "BLKCNT_C"),
            0x02 => defmt::write!(f, "BLKCNT_B"),
            0xffff => defmt::write!(f, "BLKCNT_A"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Blkcnt {
    #[inline(always)]
    fn from(val: u16) -> Blkcnt {
        Blkcnt::from_bits(val)
    }
}
impl From<Blkcnt> for u16 {
    #[inline(always)]
    fn from(val: Blkcnt) -> u16 {
        Blkcnt::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Blksize(u16);
impl Blksize {
    #[doc = "No data transfer"]
    pub const BLK_ATT_I: Self = Self(0x0);
    #[doc = "1 byte"]
    pub const BLK_ATT_H: Self = Self(0x01);
    #[doc = "2 bytes"]
    pub const BLK_ATT_G: Self = Self(0x02);
    #[doc = "3 bytes"]
    pub const BLK_ATT_F: Self = Self(0x03);
    #[doc = "4 bytes"]
    pub const BLK_ATT_E: Self = Self(0x04);
    #[doc = "511 bytes"]
    pub const BLK_ATT_D: Self = Self(0x01ff);
    #[doc = "512 bytes"]
    pub const BLK_ATT_C: Self = Self(0x0200);
    #[doc = "2048 bytes"]
    pub const BLK_ATT_B: Self = Self(0x0800);
    #[doc = "4096 bytes"]
    pub const BLK_ATT_A: Self = Self(0x1000);
}
impl Blksize {
    pub const fn from_bits(val: u16) -> Blksize {
        Self(val & 0x1fff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Blksize {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("BLK_ATT_I"),
            0x01 => f.write_str("BLK_ATT_H"),
            0x02 => f.write_str("BLK_ATT_G"),
            0x03 => f.write_str("BLK_ATT_F"),
            0x04 => f.write_str("BLK_ATT_E"),
            0x01ff => f.write_str("BLK_ATT_D"),
            0x0200 => f.write_str("BLK_ATT_C"),
            0x0800 => f.write_str("BLK_ATT_B"),
            0x1000 => f.write_str("BLK_ATT_A"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Blksize {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "BLK_ATT_I"),
            0x01 => defmt::write!(f, "BLK_ATT_H"),
            0x02 => defmt::write!(f, "BLK_ATT_G"),
            0x03 => defmt::write!(f, "BLK_ATT_F"),
            0x04 => defmt::write!(f, "BLK_ATT_E"),
            0x01ff => defmt::write!(f, "BLK_ATT_D"),
            0x0200 => defmt::write!(f, "BLK_ATT_C"),
            0x0800 => defmt::write!(f, "BLK_ATT_B"),
            0x1000 => defmt::write!(f, "BLK_ATT_A"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Blksize {
    #[inline(always)]
    fn from(val: u16) -> Blksize {
        Blksize::from_bits(val)
    }
}
impl From<Blksize> for u16 {
    #[inline(always)]
    fn from(val: Blksize) -> u16 {
        Blksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BootMode {
    #[doc = "Normal boot"]
    BOOT_MODE_A = 0x0,
    #[doc = "Alternative boot"]
    BOOT_MODE_B = 0x01,
}
impl BootMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BootMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BootMode {
    #[inline(always)]
    fn from(val: u8) -> BootMode {
        BootMode::from_bits(val)
    }
}
impl From<BootMode> for u8 {
    #[inline(always)]
    fn from(val: BootMode) -> u8 {
        BootMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BurstLenEn {
    _RESERVED_0 = 0x0,
    #[doc = "Burst length is enabled for INCR."]
    BURST_A = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl BurstLenEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BurstLenEn {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BurstLenEn {
    #[inline(always)]
    fn from(val: u8) -> BurstLenEn {
        BurstLenEn::from_bits(val)
    }
}
impl From<BurstLenEn> for u8 {
    #[inline(always)]
    fn from(val: BurstLenEn) -> u8 {
        BurstLenEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdtyp {
    #[doc = "Normal other commands"]
    CMDTYP_D = 0x0,
    #[doc = "Suspend CMD52 for writing bus suspend in CCCR"]
    CMDTYP_C = 0x01,
    #[doc = "Resume CMD52 for writing function select in CCCR"]
    CMDTYP_B = 0x02,
    #[doc = "Abort CMD12, CMD52 for writing I/O Abort in CCCR"]
    CMDTYP_A = 0x03,
}
impl Cmdtyp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdtyp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdtyp {
    #[inline(always)]
    fn from(val: u8) -> Cmdtyp {
        Cmdtyp::from_bits(val)
    }
}
impl From<Cmdtyp> for u8 {
    #[inline(always)]
    fn from(val: Cmdtyp) -> u8 {
        Cmdtyp::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Dlsl(u8);
impl Dlsl {
    #[doc = "Data 0 line signal level"]
    pub const DATA0: Self = Self(0x0);
    #[doc = "Data 1 line signal level"]
    pub const DATA1: Self = Self(0x01);
    #[doc = "Data 2 line signal level"]
    pub const DATA2: Self = Self(0x02);
    #[doc = "Data 3 line signal level"]
    pub const DATA3: Self = Self(0x03);
    #[doc = "Data 4 line signal level"]
    pub const DATA4: Self = Self(0x04);
    #[doc = "Data 5 line signal level"]
    pub const DATA5: Self = Self(0x05);
    #[doc = "Data 6 line signal level"]
    pub const DATA6: Self = Self(0x06);
    #[doc = "Data 7 line signal level"]
    pub const DATA7: Self = Self(0x07);
}
impl Dlsl {
    pub const fn from_bits(val: u8) -> Dlsl {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Dlsl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("DATA0"),
            0x01 => f.write_str("DATA1"),
            0x02 => f.write_str("DATA2"),
            0x03 => f.write_str("DATA3"),
            0x04 => f.write_str("DATA4"),
            0x05 => f.write_str("DATA5"),
            0x06 => f.write_str("DATA6"),
            0x07 => f.write_str("DATA7"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dlsl {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "DATA0"),
            0x01 => defmt::write!(f, "DATA1"),
            0x02 => defmt::write!(f, "DATA2"),
            0x03 => defmt::write!(f, "DATA3"),
            0x04 => defmt::write!(f, "DATA4"),
            0x05 => defmt::write!(f, "DATA5"),
            0x06 => defmt::write!(f, "DATA6"),
            0x07 => defmt::write!(f, "DATA7"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Dlsl {
    #[inline(always)]
    fn from(val: u8) -> Dlsl {
        Dlsl::from_bits(val)
    }
}
impl From<Dlsl> for u8 {
    #[inline(always)]
    fn from(val: Dlsl) -> u8 {
        Dlsl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmasel {
    #[doc = "No DMA or simple DMA is selected."]
    DMASEL_A = 0x0,
    #[doc = "ADMA1 is selected."]
    DMASEL_B = 0x01,
    #[doc = "ADMA2 is selected."]
    DMASEL_C = 0x02,
    _RESERVED_3 = 0x03,
}
impl Dmasel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmasel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmasel {
    #[inline(always)]
    fn from(val: u8) -> Dmasel {
        Dmasel::from_bits(val)
    }
}
impl From<Dmasel> for u8 {
    #[inline(always)]
    fn from(val: Dmasel) -> u8 {
        Dmasel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dpsel {
    #[doc = "No data present"]
    DPSEL_B = 0x0,
    #[doc = "Data present"]
    DPSEL_A = 0x01,
}
impl Dpsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dpsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dpsel {
    #[inline(always)]
    fn from(val: u8) -> Dpsel {
        Dpsel::from_bits(val)
    }
}
impl From<Dpsel> for u8 {
    #[inline(always)]
    fn from(val: Dpsel) -> u8 {
        Dpsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dtdsel {
    #[doc = "Write (Host to card)"]
    DTDSEL_B = 0x0,
    #[doc = "Read (Card to host)"]
    DTDSEL_A = 0x01,
}
impl Dtdsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dtdsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dtdsel {
    #[inline(always)]
    fn from(val: u8) -> Dtdsel {
        Dtdsel::from_bits(val)
    }
}
impl From<Dtdsel> for u8 {
    #[inline(always)]
    fn from(val: Dtdsel) -> u8 {
        Dtdsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dtocv {
    #[doc = "SDCLK x 2 14"]
    DTOCV_Q = 0x0,
    #[doc = "SDCLK x 2 15"]
    DTOCV_P = 0x01,
    #[doc = "SDCLK x 2 16"]
    DTOCV_O = 0x02,
    #[doc = "SDCLK x 2 17"]
    DTOCV_N = 0x03,
    #[doc = "SDCLK x 2 18"]
    DTOCV_M = 0x04,
    #[doc = "SDCLK x 2 19"]
    DTOCV_L = 0x05,
    #[doc = "SDCLK x 2 20"]
    DTOCV_K = 0x06,
    #[doc = "SDCLK x 2 21"]
    DTOCV_J = 0x07,
    #[doc = "SDCLK x 2 22"]
    DTOCV_I = 0x08,
    #[doc = "SDCLK x 2 23"]
    DTOCV_H = 0x09,
    #[doc = "SDCLK x 2 24"]
    DTOCV_G = 0x0a,
    #[doc = "SDCLK x 2 25"]
    DTOCV_F = 0x0b,
    #[doc = "SDCLK x 2 26"]
    DTOCV_E = 0x0c,
    #[doc = "SDCLK x 2 27"]
    DTOCV_D = 0x0d,
    #[doc = "SDCLK x 2 28"]
    DTOCV_C = 0x0e,
    #[doc = "SDCLK x 2 29"]
    DTOCV_A = 0x0f,
}
impl Dtocv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dtocv {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dtocv {
    #[inline(always)]
    fn from(val: u8) -> Dtocv {
        Dtocv::from_bits(val)
    }
}
impl From<Dtocv> for u8 {
    #[inline(always)]
    fn from(val: Dtocv) -> u8 {
        Dtocv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DtocvAck {
    #[doc = "SDCLK x 2^14"]
    DTOCV_ACK_A = 0x0,
    #[doc = "SDCLK x 2^15"]
    DTOCV_ACK_B = 0x01,
    #[doc = "SDCLK x 2^16"]
    DTOCV_ACK_C = 0x02,
    #[doc = "SDCLK x 2^17"]
    DTOCV_ACK_D = 0x03,
    #[doc = "SDCLK x 2^18"]
    DTOCV_ACK_E = 0x04,
    #[doc = "SDCLK x 2^19"]
    DTOCV_ACK_F = 0x05,
    #[doc = "SDCLK x 2^20"]
    DTOCV_ACK_G = 0x06,
    #[doc = "SDCLK x 2^21"]
    DTOCV_ACK_H = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "SDCLK x 2^28"]
    DTOCV_ACK_I = 0x0e,
    #[doc = "SDCLK x 2^29"]
    DTOCV_ACK_J = 0x0f,
}
impl DtocvAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DtocvAck {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DtocvAck {
    #[inline(always)]
    fn from(val: u8) -> DtocvAck {
        DtocvAck::from_bits(val)
    }
}
impl From<DtocvAck> for u8 {
    #[inline(always)]
    fn from(val: DtocvAck) -> u8 {
        DtocvAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dtw {
    #[doc = "1-bit mode"]
    DTW_C = 0x0,
    #[doc = "4-bit mode"]
    DTW_B = 0x01,
    #[doc = "8-bit mode"]
    DTW_A = 0x02,
    _RESERVED_3 = 0x03,
}
impl Dtw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dtw {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dtw {
    #[inline(always)]
    fn from(val: u8) -> Dtw {
        Dtw::from_bits(val)
    }
}
impl From<Dtw> for u8 {
    #[inline(always)]
    fn from(val: Dtw) -> u8 {
        Dtw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dvs {
    #[doc = "Divide-by-1"]
    DVS_A = 0x0,
    #[doc = "Divide-by-2"]
    DVS_B = 0x01,
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
    #[doc = "Divide-by-15"]
    DVS_C = 0x0e,
    #[doc = "Divide-by-16"]
    DVS_D = 0x0f,
}
impl Dvs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dvs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dvs {
    #[inline(always)]
    fn from(val: u8) -> Dvs {
        Dvs::from_bits(val)
    }
}
impl From<Dvs> for u8 {
    #[inline(always)]
    fn from(val: Dvs) -> u8 {
        Dvs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Emode {
    #[doc = "Big endian mode"]
    EMODE_A = 0x0,
    #[doc = "Half word big endian mode"]
    EMODE_B = 0x01,
    #[doc = "Little endian mode"]
    EMODE_C = 0x02,
    _RESERVED_3 = 0x03,
}
impl Emode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emode {
    #[inline(always)]
    fn from(val: u8) -> Emode {
        Emode::from_bits(val)
    }
}
impl From<Emode> for u8 {
    #[inline(always)]
    fn from(val: Emode) -> u8 {
        Emode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FbclkSel {
    #[doc = "Feedback clock comes from the loopback CLK"]
    FBCLK_B = 0x0,
    #[doc = "Feedback clock comes from the ipp_card_clk_out"]
    FBCLK_A = 0x01,
}
impl FbclkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FbclkSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FbclkSel {
    #[inline(always)]
    fn from(val: u8) -> FbclkSel {
        FbclkSel::from_bits(val)
    }
}
impl From<FbclkSel> for u8 {
    #[inline(always)]
    fn from(val: FbclkSel) -> u8 {
        FbclkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbl {
    #[doc = "512 bytes"]
    MBL_A = 0x0,
    #[doc = "1024 bytes"]
    MBL_B = 0x01,
    #[doc = "2048 bytes"]
    MBL_C = 0x02,
    #[doc = "4096 bytes"]
    MBL_D = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Mbl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbl {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbl {
    #[inline(always)]
    fn from(val: u8) -> Mbl {
        Mbl::from_bits(val)
    }
}
impl From<Mbl> for u8 {
    #[inline(always)]
    fn from(val: Mbl) -> u8 {
        Mbl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MixCtrlSmpClkSel {
    #[doc = "Fixed clock is used to sample data / cmd"]
    SMPSEL_B = 0x0,
    #[doc = "Tuned clock is used to sample data / cmd"]
    SMPSEL_A = 0x01,
}
impl MixCtrlSmpClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MixCtrlSmpClkSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MixCtrlSmpClkSel {
    #[inline(always)]
    fn from(val: u8) -> MixCtrlSmpClkSel {
        MixCtrlSmpClkSel::from_bits(val)
    }
}
impl From<MixCtrlSmpClkSel> for u8 {
    #[inline(always)]
    fn from(val: MixCtrlSmpClkSel) -> u8 {
        MixCtrlSmpClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Msbsel {
    #[doc = "Single block"]
    MSBSEL_B = 0x0,
    #[doc = "Multiple blocks"]
    MSBSEL_A = 0x01,
}
impl Msbsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Msbsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Msbsel {
    #[inline(always)]
    fn from(val: u8) -> Msbsel {
        Msbsel::from_bits(val)
    }
}
impl From<Msbsel> for u8 {
    #[inline(always)]
    fn from(val: Msbsel) -> u8 {
        Msbsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rsptyp {
    #[doc = "No response"]
    RSPTYP_A = 0x0,
    #[doc = "Response length 136"]
    RSPTYP_B = 0x01,
    #[doc = "Response length 48"]
    RSPTYP_C = 0x02,
    #[doc = "Response length 48, check busy after response"]
    RSPTYP_D = 0x03,
}
impl Rsptyp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rsptyp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rsptyp {
    #[inline(always)]
    fn from(val: u8) -> Rsptyp {
        Rsptyp::from_bits(val)
    }
}
impl From<Rsptyp> for u8 {
    #[inline(always)]
    fn from(val: Rsptyp) -> u8 {
        Rsptyp::to_bits(val)
    }
}
