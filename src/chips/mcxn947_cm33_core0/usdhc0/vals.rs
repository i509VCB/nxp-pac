#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ac12ce {
    #[doc = "No CRC error"]
    AC12CE_B = 0x0,
    #[doc = "CRC error met in Auto CMD12/23 response"]
    AC12CE_A = 0x01,
}
impl Ac12ce {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ac12ce {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ac12ce {
    #[inline(always)]
    fn from(val: u8) -> Ac12ce {
        Ac12ce::from_bits(val)
    }
}
impl From<Ac12ce> for u8 {
    #[inline(always)]
    fn from(val: Ac12ce) -> u8 {
        Ac12ce::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ac12e {
    #[doc = "No error"]
    AC12E_A = 0x0,
    #[doc = "Error"]
    AC12E_B = 0x01,
}
impl Ac12e {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ac12e {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ac12e {
    #[inline(always)]
    fn from(val: u8) -> Ac12e {
        Ac12e::from_bits(val)
    }
}
impl From<Ac12e> for u8 {
    #[inline(always)]
    fn from(val: Ac12e) -> u8 {
        Ac12e::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ac12ebe {
    #[doc = "No error"]
    AC12EBE_B = 0x0,
    #[doc = "End bit error generated"]
    AC12EBE_A = 0x01,
}
impl Ac12ebe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ac12ebe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ac12ebe {
    #[inline(always)]
    fn from(val: u8) -> Ac12ebe {
        Ac12ebe::from_bits(val)
    }
}
impl From<Ac12ebe> for u8 {
    #[inline(always)]
    fn from(val: Ac12ebe) -> u8 {
        Ac12ebe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ac12eien {
    #[doc = "Masked"]
    AC12EIEN_B = 0x0,
    #[doc = "Enabled"]
    AC12EIEN_A = 0x01,
}
impl Ac12eien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ac12eien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ac12eien {
    #[inline(always)]
    fn from(val: u8) -> Ac12eien {
        Ac12eien::from_bits(val)
    }
}
impl From<Ac12eien> for u8 {
    #[inline(always)]
    fn from(val: Ac12eien) -> u8 {
        Ac12eien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ac12esen {
    #[doc = "Masked"]
    AC12ESEN_A = 0x0,
    #[doc = "Enabled"]
    AC12ESEN_B = 0x01,
}
impl Ac12esen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ac12esen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ac12esen {
    #[inline(always)]
    fn from(val: u8) -> Ac12esen {
        Ac12esen::from_bits(val)
    }
}
impl From<Ac12esen> for u8 {
    #[inline(always)]
    fn from(val: Ac12esen) -> u8 {
        Ac12esen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ac12ie {
    #[doc = "No error"]
    AC12IE_B = 0x0,
    #[doc = "Error, the CMD index in response is not CMD12/23"]
    AC12IE_A = 0x01,
}
impl Ac12ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ac12ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ac12ie {
    #[inline(always)]
    fn from(val: u8) -> Ac12ie {
        Ac12ie::from_bits(val)
    }
}
impl From<Ac12ie> for u8 {
    #[inline(always)]
    fn from(val: Ac12ie) -> u8 {
        Ac12ie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ac12ne {
    #[doc = "Executed"]
    AC12NE_B = 0x0,
    #[doc = "Not executed"]
    AC12NE_A = 0x01,
}
impl Ac12ne {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ac12ne {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ac12ne {
    #[inline(always)]
    fn from(val: u8) -> Ac12ne {
        Ac12ne::from_bits(val)
    }
}
impl From<Ac12ne> for u8 {
    #[inline(always)]
    fn from(val: Ac12ne) -> u8 {
        Ac12ne::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ac12toe {
    #[doc = "No error"]
    AC12TOE_B = 0x0,
    #[doc = "Time out"]
    AC12TOE_A = 0x01,
}
impl Ac12toe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ac12toe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ac12toe {
    #[inline(always)]
    fn from(val: u8) -> Ac12toe {
        Ac12toe::from_bits(val)
    }
}
impl From<Ac12toe> for u8 {
    #[inline(always)]
    fn from(val: Ac12toe) -> u8 {
        Ac12toe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ac23en {
    #[doc = "Disable"]
    CMD_XFR_TYP7_B = 0x0,
    #[doc = "Enable"]
    CMD_XFR_TYP7_A = 0x01,
}
impl Ac23en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ac23en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ac23en {
    #[inline(always)]
    fn from(val: u8) -> Ac23en {
        Ac23en::from_bits(val)
    }
}
impl From<Ac23en> for u8 {
    #[inline(always)]
    fn from(val: Ac23en) -> u8 {
        Ac23en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Admadce {
    #[doc = "No error"]
    ADMADCE_B = 0x0,
    #[doc = "Error"]
    ADMADCE_A = 0x01,
}
impl Admadce {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Admadce {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Admadce {
    #[inline(always)]
    fn from(val: u8) -> Admadce {
        Admadce::from_bits(val)
    }
}
impl From<Admadce> for u8 {
    #[inline(always)]
    fn from(val: Admadce) -> u8 {
        Admadce::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Admalme {
    #[doc = "No error"]
    ADMAES_B = 0x0,
    #[doc = "Error"]
    ADMAES_A = 0x01,
}
impl Admalme {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Admalme {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Admalme {
    #[inline(always)]
    fn from(val: u8) -> Admalme {
        Admalme::from_bits(val)
    }
}
impl From<Admalme> for u8 {
    #[inline(always)]
    fn from(val: Admalme) -> u8 {
        Admalme::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Admas {
    #[doc = "Advanced DMA not supported"]
    ADMAS_B = 0x0,
    #[doc = "Advanced DMA supported"]
    ADMAS_A = 0x01,
}
impl Admas {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Admas {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Admas {
    #[inline(always)]
    fn from(val: u8) -> Admas {
        Admas::from_bits(val)
    }
}
impl From<Admas> for u8 {
    #[inline(always)]
    fn from(val: Admas) -> u8 {
        Admas::to_bits(val)
    }
}
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bge {
    #[doc = "No block gap event"]
    BGE_B = 0x0,
    #[doc = "Transaction stopped at block gap"]
    BGE_A = 0x01,
}
impl Bge {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bge {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bge {
    #[inline(always)]
    fn from(val: u8) -> Bge {
        Bge::from_bits(val)
    }
}
impl From<Bge> for u8 {
    #[inline(always)]
    fn from(val: Bge) -> u8 {
        Bge::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bgeien {
    #[doc = "Masked"]
    BGIEN_A = 0x0,
    #[doc = "Enabled"]
    BGIEN_B = 0x01,
}
impl Bgeien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bgeien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bgeien {
    #[inline(always)]
    fn from(val: u8) -> Bgeien {
        Bgeien::from_bits(val)
    }
}
impl From<Bgeien> for u8 {
    #[inline(always)]
    fn from(val: Bgeien) -> u8 {
        Bgeien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bgesen {
    #[doc = "Masked"]
    BGESEN_A = 0x0,
    #[doc = "Enabled"]
    BGESEN_B = 0x01,
}
impl Bgesen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bgesen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bgesen {
    #[inline(always)]
    fn from(val: u8) -> Bgesen {
        Bgesen::from_bits(val)
    }
}
impl From<Bgesen> for u8 {
    #[inline(always)]
    fn from(val: Bgesen) -> u8 {
        Bgesen::to_bits(val)
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
pub enum BootAck {
    #[doc = "No ack"]
    BOOT_ACK_A = 0x0,
    #[doc = "Ack"]
    BOOT_ACK_B = 0x01,
}
impl BootAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BootAck {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BootAck {
    #[inline(always)]
    fn from(val: u8) -> BootAck {
        BootAck::from_bits(val)
    }
}
impl From<BootAck> for u8 {
    #[inline(always)]
    fn from(val: BootAck) -> u8 {
        BootAck::to_bits(val)
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
pub enum Bren {
    #[doc = "Read disable"]
    BREN_B = 0x0,
    #[doc = "Read enable"]
    BREN_A = 0x01,
}
impl Bren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bren {
    #[inline(always)]
    fn from(val: u8) -> Bren {
        Bren::from_bits(val)
    }
}
impl From<Bren> for u8 {
    #[inline(always)]
    fn from(val: Bren) -> u8 {
        Bren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Brr {
    #[doc = "Not ready to read buffer"]
    BRR_B = 0x0,
    #[doc = "Ready to read buffer"]
    BRR_A = 0x01,
}
impl Brr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Brr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Brr {
    #[inline(always)]
    fn from(val: u8) -> Brr {
        Brr::from_bits(val)
    }
}
impl From<Brr> for u8 {
    #[inline(always)]
    fn from(val: Brr) -> u8 {
        Brr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Brrien {
    #[doc = "Masked"]
    BRRIEN_B = 0x0,
    #[doc = "Enabled"]
    BRRIEN_A = 0x01,
}
impl Brrien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Brrien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Brrien {
    #[inline(always)]
    fn from(val: u8) -> Brrien {
        Brrien::from_bits(val)
    }
}
impl From<Brrien> for u8 {
    #[inline(always)]
    fn from(val: Brrien) -> u8 {
        Brrien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Brrsen {
    #[doc = "Masked"]
    BRRSEN_A = 0x0,
    #[doc = "Enabled"]
    BRREN_B = 0x01,
}
impl Brrsen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Brrsen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Brrsen {
    #[inline(always)]
    fn from(val: u8) -> Brrsen {
        Brrsen::from_bits(val)
    }
}
impl From<Brrsen> for u8 {
    #[inline(always)]
    fn from(val: Brrsen) -> u8 {
        Brrsen::to_bits(val)
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
pub enum Bwen {
    #[doc = "Write disable"]
    BWEN_B = 0x0,
    #[doc = "Write enable"]
    BWEN_A = 0x01,
}
impl Bwen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bwen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bwen {
    #[inline(always)]
    fn from(val: u8) -> Bwen {
        Bwen::from_bits(val)
    }
}
impl From<Bwen> for u8 {
    #[inline(always)]
    fn from(val: Bwen) -> u8 {
        Bwen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bwr {
    #[doc = "Not ready to write buffer"]
    BWR_B = 0x0,
    #[doc = "Ready to write buffer"]
    BWR_A = 0x01,
}
impl Bwr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bwr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bwr {
    #[inline(always)]
    fn from(val: u8) -> Bwr {
        Bwr::from_bits(val)
    }
}
impl From<Bwr> for u8 {
    #[inline(always)]
    fn from(val: Bwr) -> u8 {
        Bwr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bwrien {
    #[doc = "Masked"]
    BWRIEN_A = 0x0,
    #[doc = "Enabled"]
    BWRIEN_B = 0x01,
}
impl Bwrien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bwrien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bwrien {
    #[inline(always)]
    fn from(val: u8) -> Bwrien {
        Bwrien::from_bits(val)
    }
}
impl From<Bwrien> for u8 {
    #[inline(always)]
    fn from(val: Bwrien) -> u8 {
        Bwrien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bwrsen {
    #[doc = "Masked"]
    BWRSEN_A = 0x0,
    #[doc = "Enabled"]
    BWRSEN_B = 0x01,
}
impl Bwrsen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bwrsen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bwrsen {
    #[inline(always)]
    fn from(val: u8) -> Bwrsen {
        Bwrsen::from_bits(val)
    }
}
impl From<Bwrsen> for u8 {
    #[inline(always)]
    fn from(val: Bwrsen) -> u8 {
        Bwrsen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CardIntD3Test {
    #[doc = "Check the card interrupt only when DATA3 is high."]
    CARD_INT_D3_A = 0x0,
    #[doc = "Check the card interrupt by ignoring the status of DATA3."]
    CARD_INT_D3_B = 0x01,
}
impl CardIntD3Test {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CardIntD3Test {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CardIntD3Test {
    #[inline(always)]
    fn from(val: u8) -> CardIntD3Test {
        CardIntD3Test::from_bits(val)
    }
}
impl From<CardIntD3Test> for u8 {
    #[inline(always)]
    fn from(val: CardIntD3Test) -> u8 {
        CardIntD3Test::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cc {
    #[doc = "Command not complete"]
    CC_B = 0x0,
    #[doc = "Command complete"]
    CC_A = 0x01,
}
impl Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cc {
    #[inline(always)]
    fn from(val: u8) -> Cc {
        Cc::from_bits(val)
    }
}
impl From<Cc> for u8 {
    #[inline(always)]
    fn from(val: Cc) -> u8 {
        Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cccen {
    #[doc = "Disables command CRC check"]
    CCCEN_B = 0x0,
    #[doc = "Enables command CRC check"]
    CCCEN_A = 0x01,
}
impl Cccen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cccen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cccen {
    #[inline(always)]
    fn from(val: u8) -> Cccen {
        Cccen::from_bits(val)
    }
}
impl From<Cccen> for u8 {
    #[inline(always)]
    fn from(val: Cccen) -> u8 {
        Cccen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cce {
    #[doc = "No error"]
    CCE_A = 0x0,
    #[doc = "CRC error generated"]
    CCE_B = 0x01,
}
impl Cce {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cce {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cce {
    #[inline(always)]
    fn from(val: u8) -> Cce {
        Cce::from_bits(val)
    }
}
impl From<Cce> for u8 {
    #[inline(always)]
    fn from(val: Cce) -> u8 {
        Cce::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cceien {
    #[doc = "Masked"]
    CCEIEN_B = 0x0,
    #[doc = "Enabled"]
    CCEIEN_A = 0x01,
}
impl Cceien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cceien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cceien {
    #[inline(always)]
    fn from(val: u8) -> Cceien {
        Cceien::from_bits(val)
    }
}
impl From<Cceien> for u8 {
    #[inline(always)]
    fn from(val: Cceien) -> u8 {
        Cceien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ccesen {
    #[doc = "Masked"]
    CCESEN_A = 0x0,
    #[doc = "Enabled"]
    CCESEN_B = 0x01,
}
impl Ccesen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ccesen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ccesen {
    #[inline(always)]
    fn from(val: u8) -> Ccesen {
        Ccesen::from_bits(val)
    }
}
impl From<Ccesen> for u8 {
    #[inline(always)]
    fn from(val: Ccesen) -> u8 {
        Ccesen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ccien {
    #[doc = "Masked"]
    CCIEN_A = 0x0,
    #[doc = "Enabled"]
    CCIEN_B = 0x01,
}
impl Ccien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ccien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ccien {
    #[inline(always)]
    fn from(val: u8) -> Ccien {
        Ccien::from_bits(val)
    }
}
impl From<Ccien> for u8 {
    #[inline(always)]
    fn from(val: Ccien) -> u8 {
        Ccien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ccsen {
    #[doc = "Masked"]
    CCSEN_A = 0x0,
    #[doc = "Enabled"]
    CCSEN_B = 0x01,
}
impl Ccsen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ccsen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ccsen {
    #[inline(always)]
    fn from(val: u8) -> Ccsen {
        Ccsen::from_bits(val)
    }
}
impl From<Ccsen> for u8 {
    #[inline(always)]
    fn from(val: Ccsen) -> u8 {
        Ccsen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cdihb {
    #[doc = "Can issue command that uses the DATA line"]
    CDIHB_B = 0x0,
    #[doc = "Cannot issue command that uses the DATA line"]
    CDIHB_A = 0x01,
}
impl Cdihb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cdihb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cdihb {
    #[inline(always)]
    fn from(val: u8) -> Cdihb {
        Cdihb::from_bits(val)
    }
}
impl From<Cdihb> for u8 {
    #[inline(always)]
    fn from(val: Cdihb) -> u8 {
        Cdihb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cebe {
    #[doc = "No error"]
    CEBE_A = 0x0,
    #[doc = "End bit error generated"]
    CEBE_B = 0x01,
}
impl Cebe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cebe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cebe {
    #[inline(always)]
    fn from(val: u8) -> Cebe {
        Cebe::from_bits(val)
    }
}
impl From<Cebe> for u8 {
    #[inline(always)]
    fn from(val: Cebe) -> u8 {
        Cebe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cebeien {
    #[doc = "Masked"]
    CEBEIEN_B = 0x0,
    #[doc = "Enabled"]
    CEBEIEN_A = 0x01,
}
impl Cebeien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cebeien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cebeien {
    #[inline(always)]
    fn from(val: u8) -> Cebeien {
        Cebeien::from_bits(val)
    }
}
impl From<Cebeien> for u8 {
    #[inline(always)]
    fn from(val: Cebeien) -> u8 {
        Cebeien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cebesen {
    #[doc = "Masked"]
    CEBESEN_A = 0x0,
    #[doc = "Enabled"]
    CEBESEN_B = 0x01,
}
impl Cebesen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cebesen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cebesen {
    #[inline(always)]
    fn from(val: u8) -> Cebesen {
        Cebesen::from_bits(val)
    }
}
impl From<Cebesen> for u8 {
    #[inline(always)]
    fn from(val: Cebesen) -> u8 {
        Cebesen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cicen {
    #[doc = "Disable command index check"]
    CICEN_B = 0x0,
    #[doc = "Enables command index check"]
    CICEN_A = 0x01,
}
impl Cicen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cicen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cicen {
    #[inline(always)]
    fn from(val: u8) -> Cicen {
        Cicen::from_bits(val)
    }
}
impl From<Cicen> for u8 {
    #[inline(always)]
    fn from(val: Cicen) -> u8 {
        Cicen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cie {
    #[doc = "No error"]
    CIE_A = 0x0,
    #[doc = "Error"]
    CIE_B = 0x01,
}
impl Cie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cie {
    #[inline(always)]
    fn from(val: u8) -> Cie {
        Cie::from_bits(val)
    }
}
impl From<Cie> for u8 {
    #[inline(always)]
    fn from(val: Cie) -> u8 {
        Cie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cieien {
    #[doc = "Masked"]
    CIEIEN_B = 0x0,
    #[doc = "Enabled"]
    CIEIEN_A = 0x01,
}
impl Cieien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cieien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cieien {
    #[inline(always)]
    fn from(val: u8) -> Cieien {
        Cieien::from_bits(val)
    }
}
impl From<Cieien> for u8 {
    #[inline(always)]
    fn from(val: Cieien) -> u8 {
        Cieien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ciesen {
    #[doc = "Masked"]
    CIESEN_A = 0x0,
    #[doc = "Enabled"]
    CIESEN_B = 0x01,
}
impl Ciesen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ciesen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ciesen {
    #[inline(always)]
    fn from(val: u8) -> Ciesen {
        Ciesen::from_bits(val)
    }
}
impl From<Ciesen> for u8 {
    #[inline(always)]
    fn from(val: Ciesen) -> u8 {
        Ciesen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cihb {
    #[doc = "Can issue command using only CMD line"]
    CIHB_A = 0x0,
    #[doc = "Cannot issue command"]
    CIHB_B = 0x01,
}
impl Cihb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cihb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cihb {
    #[inline(always)]
    fn from(val: u8) -> Cihb {
        Cihb::from_bits(val)
    }
}
impl From<Cihb> for u8 {
    #[inline(always)]
    fn from(val: Cihb) -> u8 {
        Cihb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cins {
    #[doc = "Card state unstable or removed"]
    BWR_B = 0x0,
    #[doc = "Card inserted"]
    BWR_A = 0x01,
}
impl Cins {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cins {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cins {
    #[inline(always)]
    fn from(val: u8) -> Cins {
        Cins::from_bits(val)
    }
}
impl From<Cins> for u8 {
    #[inline(always)]
    fn from(val: Cins) -> u8 {
        Cins::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cinsien {
    #[doc = "Masked"]
    CINSIEN_A = 0x0,
    #[doc = "Enabled"]
    CINSIEN_B = 0x01,
}
impl Cinsien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cinsien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cinsien {
    #[inline(always)]
    fn from(val: u8) -> Cinsien {
        Cinsien::from_bits(val)
    }
}
impl From<Cinsien> for u8 {
    #[inline(always)]
    fn from(val: Cinsien) -> u8 {
        Cinsien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cinssen {
    #[doc = "Masked"]
    CINSEN_A = 0x0,
    #[doc = "Enabled"]
    CINSEN_B = 0x01,
}
impl Cinssen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cinssen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cinssen {
    #[inline(always)]
    fn from(val: u8) -> Cinssen {
        Cinssen::from_bits(val)
    }
}
impl From<Cinssen> for u8 {
    #[inline(always)]
    fn from(val: Cinssen) -> u8 {
        Cinssen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cinst {
    #[doc = "Power on reset or no card"]
    CINST_A = 0x0,
    #[doc = "Card inserted"]
    CINST_B = 0x01,
}
impl Cinst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cinst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cinst {
    #[inline(always)]
    fn from(val: u8) -> Cinst {
        Cinst::from_bits(val)
    }
}
impl From<Cinst> for u8 {
    #[inline(always)]
    fn from(val: Cinst) -> u8 {
        Cinst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cint {
    #[doc = "No card interrupt"]
    CINT_A = 0x0,
    #[doc = "Generate card interrupt"]
    CINT_B = 0x01,
}
impl Cint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cint {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cint {
    #[inline(always)]
    fn from(val: u8) -> Cint {
        Cint::from_bits(val)
    }
}
impl From<Cint> for u8 {
    #[inline(always)]
    fn from(val: Cint) -> u8 {
        Cint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cintien {
    #[doc = "Masked"]
    CINTIEN_B = 0x0,
    #[doc = "Enabled"]
    CINTIEN_A = 0x01,
}
impl Cintien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cintien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cintien {
    #[inline(always)]
    fn from(val: u8) -> Cintien {
        Cintien::from_bits(val)
    }
}
impl From<Cintien> for u8 {
    #[inline(always)]
    fn from(val: Cintien) -> u8 {
        Cintien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cintsen {
    #[doc = "Masked"]
    CINTSEN_A = 0x0,
    #[doc = "Enabled"]
    CINTSEN_B = 0x01,
}
impl Cintsen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cintsen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cintsen {
    #[inline(always)]
    fn from(val: u8) -> Cintsen {
        Cintsen::from_bits(val)
    }
}
impl From<Cintsen> for u8 {
    #[inline(always)]
    fn from(val: Cintsen) -> u8 {
        Cintsen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmdXfrTypAc12en {
    #[doc = "Disable"]
    CMD_XFR_TYP2_B = 0x0,
    #[doc = "Enable"]
    CMD_XFR_TYP2_A = 0x01,
}
impl CmdXfrTypAc12en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmdXfrTypAc12en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmdXfrTypAc12en {
    #[inline(always)]
    fn from(val: u8) -> CmdXfrTypAc12en {
        CmdXfrTypAc12en::from_bits(val)
    }
}
impl From<CmdXfrTypAc12en> for u8 {
    #[inline(always)]
    fn from(val: CmdXfrTypAc12en) -> u8 {
        CmdXfrTypAc12en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmdXfrTypBcen {
    #[doc = "Disable"]
    CMD_XFR_TYP1_B = 0x0,
    #[doc = "Enable"]
    CMD_XFR_TYP1_A = 0x01,
}
impl CmdXfrTypBcen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmdXfrTypBcen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmdXfrTypBcen {
    #[inline(always)]
    fn from(val: u8) -> CmdXfrTypBcen {
        CmdXfrTypBcen::from_bits(val)
    }
}
impl From<CmdXfrTypBcen> for u8 {
    #[inline(always)]
    fn from(val: CmdXfrTypBcen) -> u8 {
        CmdXfrTypBcen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmdXfrTypDmaen {
    #[doc = "Disable"]
    CMD_XFR_TYP_0B = 0x0,
    #[doc = "Enable"]
    CMD_XFR_TYP_0A = 0x01,
}
impl CmdXfrTypDmaen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmdXfrTypDmaen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmdXfrTypDmaen {
    #[inline(always)]
    fn from(val: u8) -> CmdXfrTypDmaen {
        CmdXfrTypDmaen::from_bits(val)
    }
}
impl From<CmdXfrTypDmaen> for u8 {
    #[inline(always)]
    fn from(val: CmdXfrTypDmaen) -> u8 {
        CmdXfrTypDmaen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmdXfrTypDtdsel {
    #[doc = "Disable"]
    CMD_XFR_TYP4_B = 0x0,
    #[doc = "Enable"]
    CMD_XFR_TYP4_A = 0x01,
}
impl CmdXfrTypDtdsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmdXfrTypDtdsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmdXfrTypDtdsel {
    #[inline(always)]
    fn from(val: u8) -> CmdXfrTypDtdsel {
        CmdXfrTypDtdsel::from_bits(val)
    }
}
impl From<CmdXfrTypDtdsel> for u8 {
    #[inline(always)]
    fn from(val: CmdXfrTypDtdsel) -> u8 {
        CmdXfrTypDtdsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmdXfrTypMsbsel {
    #[doc = "Disable"]
    CMD_XFR_TYP5_B = 0x0,
    #[doc = "Enable"]
    CMD_XFR_TYP5_A = 0x01,
}
impl CmdXfrTypMsbsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmdXfrTypMsbsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmdXfrTypMsbsel {
    #[inline(always)]
    fn from(val: u8) -> CmdXfrTypMsbsel {
        CmdXfrTypMsbsel::from_bits(val)
    }
}
impl From<CmdXfrTypMsbsel> for u8 {
    #[inline(always)]
    fn from(val: CmdXfrTypMsbsel) -> u8 {
        CmdXfrTypMsbsel::to_bits(val)
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cnibac12e {
    #[doc = "No error"]
    CNIBAC12E_B = 0x0,
    #[doc = "Not issued"]
    CNIBAC12E_A = 0x01,
}
impl Cnibac12e {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cnibac12e {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cnibac12e {
    #[inline(always)]
    fn from(val: u8) -> Cnibac12e {
        Cnibac12e::from_bits(val)
    }
}
impl From<Cnibac12e> for u8 {
    #[inline(always)]
    fn from(val: Cnibac12e) -> u8 {
        Cnibac12e::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CrcChkDis {
    #[doc = "Check CRC16 for every read data packet and check CRC fields for every write data packet"]
    CRC_CHK_DIS_A = 0x0,
    #[doc = "Ignore CRC16 check for every read data packet and ignore CRC fields check for every write data packet"]
    CRC_CHK_DIS_B = 0x01,
}
impl CrcChkDis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CrcChkDis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CrcChkDis {
    #[inline(always)]
    fn from(val: u8) -> CrcChkDis {
        CrcChkDis::from_bits(val)
    }
}
impl From<CrcChkDis> for u8 {
    #[inline(always)]
    fn from(val: CrcChkDis) -> u8 {
        CrcChkDis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Creq {
    #[doc = "No effect"]
    CREQ_B = 0x0,
    #[doc = "Restart"]
    CREQ_A = 0x01,
}
impl Creq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Creq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Creq {
    #[inline(always)]
    fn from(val: u8) -> Creq {
        Creq::from_bits(val)
    }
}
impl From<Creq> for u8 {
    #[inline(always)]
    fn from(val: Creq) -> u8 {
        Creq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crm {
    #[doc = "Card state unstable or inserted"]
    CRM_A = 0x0,
    #[doc = "Card removed"]
    CRM_B = 0x01,
}
impl Crm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crm {
    #[inline(always)]
    fn from(val: u8) -> Crm {
        Crm::from_bits(val)
    }
}
impl From<Crm> for u8 {
    #[inline(always)]
    fn from(val: Crm) -> u8 {
        Crm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crmien {
    #[doc = "Masked"]
    CRMIEN_A = 0x0,
    #[doc = "Enabled"]
    CRMIEN_B = 0x01,
}
impl Crmien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crmien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crmien {
    #[inline(always)]
    fn from(val: u8) -> Crmien {
        Crmien::from_bits(val)
    }
}
impl From<Crmien> for u8 {
    #[inline(always)]
    fn from(val: Crmien) -> u8 {
        Crmien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crmsen {
    #[doc = "Masked"]
    CRMSEN_A = 0x0,
    #[doc = "Enabled"]
    CRMSEN_B = 0x01,
}
impl Crmsen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crmsen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crmsen {
    #[inline(always)]
    fn from(val: u8) -> Crmsen {
        Crmsen::from_bits(val)
    }
}
impl From<Crmsen> for u8 {
    #[inline(always)]
    fn from(val: Crmsen) -> u8 {
        Crmsen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctoe {
    #[doc = "No error"]
    CTOE_A = 0x0,
    #[doc = "Time out"]
    CTOE_B = 0x01,
}
impl Ctoe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctoe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctoe {
    #[inline(always)]
    fn from(val: u8) -> Ctoe {
        Ctoe::from_bits(val)
    }
}
impl From<Ctoe> for u8 {
    #[inline(always)]
    fn from(val: Ctoe) -> u8 {
        Ctoe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctoeien {
    #[doc = "Masked"]
    CTOEIEN_B = 0x0,
    #[doc = "Enabled"]
    CTOEIEN_A = 0x01,
}
impl Ctoeien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctoeien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctoeien {
    #[inline(always)]
    fn from(val: u8) -> Ctoeien {
        Ctoeien::from_bits(val)
    }
}
impl From<Ctoeien> for u8 {
    #[inline(always)]
    fn from(val: Ctoeien) -> u8 {
        Ctoeien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctoesen {
    #[doc = "Masked"]
    CTOSEN_A = 0x0,
    #[doc = "Enabled"]
    CTOSEN_B = 0x01,
}
impl Ctoesen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctoesen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctoesen {
    #[inline(always)]
    fn from(val: u8) -> Ctoesen {
        Ctoesen::from_bits(val)
    }
}
impl From<Ctoesen> for u8 {
    #[inline(always)]
    fn from(val: Ctoesen) -> u8 {
        Ctoesen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum D3cd {
    #[doc = "DATA3 does not monitor card insertion"]
    D3CD_B = 0x0,
    #[doc = "DATA3 as card detection pin"]
    D3CD_A = 0x01,
}
impl D3cd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> D3cd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for D3cd {
    #[inline(always)]
    fn from(val: u8) -> D3cd {
        D3cd::from_bits(val)
    }
}
impl From<D3cd> for u8 {
    #[inline(always)]
    fn from(val: D3cd) -> u8 {
        D3cd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dce {
    #[doc = "No error"]
    DCE_A = 0x0,
    #[doc = "Error"]
    DCE_B = 0x01,
}
impl Dce {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dce {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dce {
    #[inline(always)]
    fn from(val: u8) -> Dce {
        Dce::from_bits(val)
    }
}
impl From<Dce> for u8 {
    #[inline(always)]
    fn from(val: Dce) -> u8 {
        Dce::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dceien {
    #[doc = "Masked"]
    DCEIEN_B = 0x0,
    #[doc = "Enabled"]
    DCEIEN_A = 0x01,
}
impl Dceien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dceien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dceien {
    #[inline(always)]
    fn from(val: u8) -> Dceien {
        Dceien::from_bits(val)
    }
}
impl From<Dceien> for u8 {
    #[inline(always)]
    fn from(val: Dceien) -> u8 {
        Dceien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dcesen {
    #[doc = "Masked"]
    DCESEN_A = 0x0,
    #[doc = "Enabled"]
    DCESEN_B = 0x01,
}
impl Dcesen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dcesen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dcesen {
    #[inline(always)]
    fn from(val: u8) -> Dcesen {
        Dcesen::from_bits(val)
    }
}
impl From<Dcesen> for u8 {
    #[inline(always)]
    fn from(val: Dcesen) -> u8 {
        Dcesen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Debe {
    #[doc = "No error"]
    DEBE_A = 0x0,
    #[doc = "Error"]
    DEBE_B = 0x01,
}
impl Debe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Debe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Debe {
    #[inline(always)]
    fn from(val: u8) -> Debe {
        Debe::from_bits(val)
    }
}
impl From<Debe> for u8 {
    #[inline(always)]
    fn from(val: Debe) -> u8 {
        Debe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Debeien {
    #[doc = "Masked"]
    DEBEIEN_B = 0x0,
    #[doc = "Enabled"]
    DEBEIEN_A = 0x01,
}
impl Debeien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Debeien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Debeien {
    #[inline(always)]
    fn from(val: u8) -> Debeien {
        Debeien::from_bits(val)
    }
}
impl From<Debeien> for u8 {
    #[inline(always)]
    fn from(val: Debeien) -> u8 {
        Debeien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Debesen {
    #[doc = "Masked"]
    DBESEN_A = 0x0,
    #[doc = "Enabled"]
    DBESEN_B = 0x01,
}
impl Debesen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Debesen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Debesen {
    #[inline(always)]
    fn from(val: u8) -> Debesen {
        Debesen::from_bits(val)
    }
}
impl From<Debesen> for u8 {
    #[inline(always)]
    fn from(val: Debesen) -> u8 {
        Debesen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dint {
    #[doc = "No DMA interrupt"]
    DINT_B = 0x0,
    #[doc = "DMA interrupt is generated."]
    DINT_A = 0x01,
}
impl Dint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dint {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dint {
    #[inline(always)]
    fn from(val: u8) -> Dint {
        Dint::from_bits(val)
    }
}
impl From<Dint> for u8 {
    #[inline(always)]
    fn from(val: Dint) -> u8 {
        Dint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dintien {
    #[doc = "Masked"]
    DINTIEN_B = 0x0,
    #[doc = "Enabled"]
    DINTIEN_A = 0x01,
}
impl Dintien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dintien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dintien {
    #[inline(always)]
    fn from(val: u8) -> Dintien {
        Dintien::from_bits(val)
    }
}
impl From<Dintien> for u8 {
    #[inline(always)]
    fn from(val: Dintien) -> u8 {
        Dintien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dintsen {
    #[doc = "Masked"]
    DINTSEN_A = 0x0,
    #[doc = "Enabled"]
    DINTSEN_B = 0x01,
}
impl Dintsen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dintsen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dintsen {
    #[inline(always)]
    fn from(val: u8) -> Dintsen {
        Dintsen::from_bits(val)
    }
}
impl From<Dintsen> for u8 {
    #[inline(always)]
    fn from(val: Dintsen) -> u8 {
        Dintsen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisableTimeOut {
    #[doc = "Enable time out"]
    DISABLE_TIMEOUT_A = 0x0,
    #[doc = "Disable time out"]
    DISABLE_TIMEOUT_B = 0x01,
}
impl DisableTimeOut {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisableTimeOut {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisableTimeOut {
    #[inline(always)]
    fn from(val: u8) -> DisableTimeOut {
        DisableTimeOut::from_bits(val)
    }
}
impl From<DisableTimeOut> for u8 {
    #[inline(always)]
    fn from(val: DisableTimeOut) -> u8 {
        DisableTimeOut::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dla {
    #[doc = "DATA line inactive"]
    DLA_A = 0x0,
    #[doc = "DATA line active"]
    DLA_B = 0x01,
}
impl Dla {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dla {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dla {
    #[inline(always)]
    fn from(val: u8) -> Dla {
        Dla::from_bits(val)
    }
}
impl From<Dla> for u8 {
    #[inline(always)]
    fn from(val: Dla) -> u8 {
        Dla::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Dlsl(u8);
impl Dlsl {
    #[doc = "Data 0 line signal level"]
    pub const DATA0: Self = Self(0x01);
    #[doc = "Data 1 line signal level"]
    pub const DATA1: Self = Self(0x02);
    #[doc = "Data 2 line signal level"]
    pub const DATA2: Self = Self(0x04);
    #[doc = "Data 3 line signal level"]
    pub const DATA3: Self = Self(0x08);
    #[doc = "Data 4 line signal level"]
    pub const DATA4: Self = Self(0x10);
    #[doc = "Data 5 line signal level"]
    pub const DATA5: Self = Self(0x20);
    #[doc = "Data 6 line signal level"]
    pub const DATA6: Self = Self(0x40);
    #[doc = "Data 7 line signal level"]
    pub const DATA7: Self = Self(0x80);
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
            0x01 => f.write_str("DATA0"),
            0x02 => f.write_str("DATA1"),
            0x04 => f.write_str("DATA2"),
            0x08 => f.write_str("DATA3"),
            0x10 => f.write_str("DATA4"),
            0x20 => f.write_str("DATA5"),
            0x40 => f.write_str("DATA6"),
            0x80 => f.write_str("DATA7"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dlsl {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "DATA0"),
            0x02 => defmt::write!(f, "DATA1"),
            0x04 => defmt::write!(f, "DATA2"),
            0x08 => defmt::write!(f, "DATA3"),
            0x10 => defmt::write!(f, "DATA4"),
            0x20 => defmt::write!(f, "DATA5"),
            0x40 => defmt::write!(f, "DATA6"),
            0x80 => defmt::write!(f, "DATA7"),
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
pub enum Dmae {
    #[doc = "No error"]
    DMAE_A = 0x0,
    #[doc = "Error"]
    DMAE_B = 0x01,
}
impl Dmae {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmae {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmae {
    #[inline(always)]
    fn from(val: u8) -> Dmae {
        Dmae::from_bits(val)
    }
}
impl From<Dmae> for u8 {
    #[inline(always)]
    fn from(val: Dmae) -> u8 {
        Dmae::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmaeien {
    #[doc = "Masked"]
    DMAEIEN_B = 0x0,
    #[doc = "Enable"]
    DMAEIEN_A = 0x01,
}
impl Dmaeien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmaeien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmaeien {
    #[inline(always)]
    fn from(val: u8) -> Dmaeien {
        Dmaeien::from_bits(val)
    }
}
impl From<Dmaeien> for u8 {
    #[inline(always)]
    fn from(val: Dmaeien) -> u8 {
        Dmaeien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmaesen {
    #[doc = "Masked"]
    DMASEN_B = 0x0,
    #[doc = "Enabled"]
    DMASEN_A = 0x01,
}
impl Dmaesen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmaesen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmaesen {
    #[inline(always)]
    fn from(val: u8) -> Dmaesen {
        Dmaesen::from_bits(val)
    }
}
impl From<Dmaesen> for u8 {
    #[inline(always)]
    fn from(val: Dmaesen) -> u8 {
        Dmaesen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmas {
    #[doc = "DMA not supported"]
    DMAS_B = 0x0,
    #[doc = "DMA supported"]
    DMAS_A = 0x01,
}
impl Dmas {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmas {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmas {
    #[inline(always)]
    fn from(val: u8) -> Dmas {
        Dmas::from_bits(val)
    }
}
impl From<Dmas> for u8 {
    #[inline(always)]
    fn from(val: Dmas) -> u8 {
        Dmas::to_bits(val)
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
pub enum Dtocv {
    #[doc = "SDCLK x 2 32"]
    DTOCV_X = 0x0,
    #[doc = "SDCLK x 2 33"]
    DTOCV_W = 0x01,
    #[doc = "SDCLK x 2 18"]
    DTOCV_V = 0x02,
    #[doc = "SDCLK x 2 19"]
    DTOCV_U = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    #[doc = "SDCLK x 2 29, recommend to use for supported speed modes except SDR104 mode"]
    DTOCV_T = 0x0d,
    #[doc = "SDCLK x 2 30, recommend to use for SDR104 mode"]
    DTOCV_S = 0x0e,
    _RESERVED_f = 0x0f,
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
pub enum Dtoe {
    #[doc = "No error"]
    DTOE_A = 0x0,
    #[doc = "Time out"]
    DTOE_B = 0x01,
}
impl Dtoe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dtoe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dtoe {
    #[inline(always)]
    fn from(val: u8) -> Dtoe {
        Dtoe::from_bits(val)
    }
}
impl From<Dtoe> for u8 {
    #[inline(always)]
    fn from(val: Dtoe) -> u8 {
        Dtoe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dtoeien {
    #[doc = "Masked"]
    DTOEIEN_B = 0x0,
    #[doc = "Enabled"]
    DTOEIEN_A = 0x01,
}
impl Dtoeien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dtoeien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dtoeien {
    #[inline(always)]
    fn from(val: u8) -> Dtoeien {
        Dtoeien::from_bits(val)
    }
}
impl From<Dtoeien> for u8 {
    #[inline(always)]
    fn from(val: Dtoeien) -> u8 {
        Dtoeien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dtoesen {
    #[doc = "Masked"]
    DTOESEN_A = 0x0,
    #[doc = "Enabled"]
    DTOESEN_B = 0x01,
}
impl Dtoesen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dtoesen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dtoesen {
    #[inline(always)]
    fn from(val: u8) -> Dtoesen {
        Dtoesen::from_bits(val)
    }
}
impl From<Dtoesen> for u8 {
    #[inline(always)]
    fn from(val: Dtoesen) -> u8 {
        Dtoesen::to_bits(val)
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
pub enum En32kClk {
    #[doc = "Use the peripheral clock (ipg_clk) for card detection."]
    CD_CLK_SEL_A = 0x0,
    #[doc = "Use the low power clock (ipg_clk_lp) for card detection."]
    CD_CLK_SEL_B = 0x01,
}
impl En32kClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En32kClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En32kClk {
    #[inline(always)]
    fn from(val: u8) -> En32kClk {
        En32kClk::from_bits(val)
    }
}
impl From<En32kClk> for u8 {
    #[inline(always)]
    fn from(val: En32kClk) -> u8 {
        En32kClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ExeTune {
    #[doc = "Not tuned or tuning completed"]
    EXE_TUNE_D = 0x0,
    #[doc = "Execute tuning"]
    EXE_TUNE_C = 0x01,
}
impl ExeTune {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ExeTune {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ExeTune {
    #[inline(always)]
    fn from(val: u8) -> ExeTune {
        ExeTune::from_bits(val)
    }
}
impl From<ExeTune> for u8 {
    #[inline(always)]
    fn from(val: ExeTune) -> u8 {
        ExeTune::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ExecuteTuning {
    #[doc = "Tuning procedure is aborted"]
    EX_TUN_B = 0x0,
    #[doc = "Start tuning procedure"]
    EX_TUN_A = 0x01,
}
impl ExecuteTuning {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ExecuteTuning {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ExecuteTuning {
    #[inline(always)]
    fn from(val: u8) -> ExecuteTuning {
        ExecuteTuning::from_bits(val)
    }
}
impl From<ExecuteTuning> for u8 {
    #[inline(always)]
    fn from(val: ExecuteTuning) -> u8 {
        ExecuteTuning::to_bits(val)
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
pub enum FrcSdclkOn {
    #[doc = "CLK active or inactive is fully controlled by the hardware."]
    FRC_SDCLK_ON_A = 0x0,
    #[doc = "Force CLK active"]
    FRC_SDCLK_ON_B = 0x01,
}
impl FrcSdclkOn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrcSdclkOn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrcSdclkOn {
    #[inline(always)]
    fn from(val: u8) -> FrcSdclkOn {
        FrcSdclkOn::from_bits(val)
    }
}
impl From<FrcSdclkOn> for u8 {
    #[inline(always)]
    fn from(val: FrcSdclkOn) -> u8 {
        FrcSdclkOn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hss {
    #[doc = "High speed not supported"]
    HSS_B = 0x0,
    #[doc = "High speed supported"]
    HSS_A = 0x01,
}
impl Hss {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hss {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hss {
    #[inline(always)]
    fn from(val: u8) -> Hss {
        Hss::from_bits(val)
    }
}
impl From<Hss> for u8 {
    #[inline(always)]
    fn from(val: Hss) -> u8 {
        Hss::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iabg {
    #[doc = "Disables interrupt at block gap"]
    IABG_B = 0x0,
    #[doc = "Enables interrupt at block gap"]
    IABG_A = 0x01,
}
impl Iabg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iabg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iabg {
    #[inline(always)]
    fn from(val: u8) -> Iabg {
        Iabg::from_bits(val)
    }
}
impl From<Iabg> for u8 {
    #[inline(always)]
    fn from(val: Iabg) -> u8 {
        Iabg::to_bits(val)
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
pub enum MixCtrlAc12en {
    #[doc = "Disable"]
    AC12EN_B = 0x0,
    #[doc = "Enable"]
    AC12EN_A = 0x01,
}
impl MixCtrlAc12en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MixCtrlAc12en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MixCtrlAc12en {
    #[inline(always)]
    fn from(val: u8) -> MixCtrlAc12en {
        MixCtrlAc12en::from_bits(val)
    }
}
impl From<MixCtrlAc12en> for u8 {
    #[inline(always)]
    fn from(val: MixCtrlAc12en) -> u8 {
        MixCtrlAc12en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MixCtrlBcen {
    #[doc = "Disable"]
    BCEN_B = 0x0,
    #[doc = "Enable"]
    BCEN_A = 0x01,
}
impl MixCtrlBcen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MixCtrlBcen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MixCtrlBcen {
    #[inline(always)]
    fn from(val: u8) -> MixCtrlBcen {
        MixCtrlBcen::from_bits(val)
    }
}
impl From<MixCtrlBcen> for u8 {
    #[inline(always)]
    fn from(val: MixCtrlBcen) -> u8 {
        MixCtrlBcen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MixCtrlDmaen {
    #[doc = "Disable"]
    DMAIN_B = 0x0,
    #[doc = "Enable"]
    DMAIN_A = 0x01,
}
impl MixCtrlDmaen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MixCtrlDmaen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MixCtrlDmaen {
    #[inline(always)]
    fn from(val: u8) -> MixCtrlDmaen {
        MixCtrlDmaen::from_bits(val)
    }
}
impl From<MixCtrlDmaen> for u8 {
    #[inline(always)]
    fn from(val: MixCtrlDmaen) -> u8 {
        MixCtrlDmaen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MixCtrlDtdsel {
    #[doc = "Write (Host to card)"]
    DTDSEL_B = 0x0,
    #[doc = "Read (Card to host)"]
    DTDSEL_A = 0x01,
}
impl MixCtrlDtdsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MixCtrlDtdsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MixCtrlDtdsel {
    #[inline(always)]
    fn from(val: u8) -> MixCtrlDtdsel {
        MixCtrlDtdsel::from_bits(val)
    }
}
impl From<MixCtrlDtdsel> for u8 {
    #[inline(always)]
    fn from(val: MixCtrlDtdsel) -> u8 {
        MixCtrlDtdsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MixCtrlMsbsel {
    #[doc = "Single block"]
    MSBSEL_B = 0x0,
    #[doc = "Multiple blocks"]
    MSBSEL_A = 0x01,
}
impl MixCtrlMsbsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MixCtrlMsbsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MixCtrlMsbsel {
    #[inline(always)]
    fn from(val: u8) -> MixCtrlMsbsel {
        MixCtrlMsbsel::from_bits(val)
    }
}
impl From<MixCtrlMsbsel> for u8 {
    #[inline(always)]
    fn from(val: MixCtrlMsbsel) -> u8 {
        MixCtrlMsbsel::to_bits(val)
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
pub enum NibblePos {
    #[doc = "Disable"]
    CMD_XFR_TYP6_B = 0x0,
    #[doc = "Enable"]
    CMD_XFR_TYP6_A = 0x01,
}
impl NibblePos {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NibblePos {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NibblePos {
    #[inline(always)]
    fn from(val: u8) -> NibblePos {
        NibblePos::from_bits(val)
    }
}
impl From<NibblePos> for u8 {
    #[inline(always)]
    fn from(val: NibblePos) -> u8 {
        NibblePos::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NonExactBlkRd {
    #[doc = "The block read is exact block read. Host driver does not need to issue abort command to terminate this multi-block read."]
    EXACT_B = 0x0,
    #[doc = "The block read is non-exact block read. Host driver needs to issue abort command to terminate this multi-block read."]
    EXACT_A = 0x01,
}
impl NonExactBlkRd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NonExactBlkRd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NonExactBlkRd {
    #[inline(always)]
    fn from(val: u8) -> NonExactBlkRd {
        NonExactBlkRd::from_bits(val)
    }
}
impl From<NonExactBlkRd> for u8 {
    #[inline(always)]
    fn from(val: NonExactBlkRd) -> u8 {
        NonExactBlkRd::to_bits(val)
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rta {
    #[doc = "No valid data"]
    RTA_B = 0x0,
    #[doc = "Transferring data"]
    RTA_A = 0x01,
}
impl Rta {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rta {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rta {
    #[inline(always)]
    fn from(val: u8) -> Rta {
        Rta::from_bits(val)
    }
}
impl From<Rta> for u8 {
    #[inline(always)]
    fn from(val: Rta) -> u8 {
        Rta::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rte {
    #[doc = "Re-tuning is not required."]
    RTE_A = 0x0,
    #[doc = "Re-tuning should be performed."]
    RTE_B = 0x01,
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
pub enum Rteien {
    #[doc = "Masked"]
    RTEIEN_O = 0x0,
    #[doc = "Enabled"]
    RTEIEN_N = 0x01,
}
impl Rteien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rteien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rteien {
    #[inline(always)]
    fn from(val: u8) -> Rteien {
        Rteien::from_bits(val)
    }
}
impl From<Rteien> for u8 {
    #[inline(always)]
    fn from(val: Rteien) -> u8 {
        Rteien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rtesen {
    #[doc = "Masked"]
    RTESEN_A = 0x0,
    #[doc = "Enabled"]
    RTESEN_B = 0x01,
}
impl Rtesen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rtesen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rtesen {
    #[inline(always)]
    fn from(val: u8) -> Rtesen {
        Rtesen::from_bits(val)
    }
}
impl From<Rtesen> for u8 {
    #[inline(always)]
    fn from(val: Rtesen) -> u8 {
        Rtesen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rtr {
    #[doc = "Fixed or well tuned sampling clock"]
    RTR_B = 0x0,
    #[doc = "Sampling clock needs re-tuning"]
    RTR_A = 0x01,
}
impl Rtr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rtr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rtr {
    #[inline(always)]
    fn from(val: u8) -> Rtr {
        Rtr::from_bits(val)
    }
}
impl From<Rtr> for u8 {
    #[inline(always)]
    fn from(val: Rtr) -> u8 {
        Rtr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rwctl {
    #[doc = "Disables read wait control and stop SD clock at block gap when SABGREQ field is set"]
    RWCTL_B = 0x0,
    #[doc = "Enables read wait control and assert read wait without stopping SD clock at block gap when SABGREQ field is set"]
    RWCTL_A = 0x01,
}
impl Rwctl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rwctl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rwctl {
    #[inline(always)]
    fn from(val: u8) -> Rwctl {
        Rwctl::from_bits(val)
    }
}
impl From<Rwctl> for u8 {
    #[inline(always)]
    fn from(val: Rwctl) -> u8 {
        Rwctl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sabgreq {
    #[doc = "Transfer"]
    SABGREQ_B = 0x0,
    #[doc = "Stop"]
    SABGREQ_A = 0x01,
}
impl Sabgreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sabgreq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sabgreq {
    #[inline(always)]
    fn from(val: u8) -> Sabgreq {
        Sabgreq::from_bits(val)
    }
}
impl From<Sabgreq> for u8 {
    #[inline(always)]
    fn from(val: Sabgreq) -> u8 {
        Sabgreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sdstb {
    #[doc = "Clock is changing frequency and not stable."]
    SDSTB_B = 0x0,
    #[doc = "Clock is stable."]
    SDSTB_A = 0x01,
}
impl Sdstb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sdstb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sdstb {
    #[inline(always)]
    fn from(val: u8) -> Sdstb {
        Sdstb::from_bits(val)
    }
}
impl From<Sdstb> for u8 {
    #[inline(always)]
    fn from(val: Sdstb) -> u8 {
        Sdstb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Srs {
    #[doc = "Not supported"]
    SRS_B = 0x0,
    #[doc = "Supported"]
    SRS_A = 0x01,
}
impl Srs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Srs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Srs {
    #[inline(always)]
    fn from(val: u8) -> Srs {
        Srs::from_bits(val)
    }
}
impl From<Srs> for u8 {
    #[inline(always)]
    fn from(val: Srs) -> u8 {
        Srs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tc {
    #[doc = "Transfer does not complete"]
    TC_B = 0x0,
    #[doc = "Transfer complete"]
    TC_A = 0x01,
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
pub enum Tcien {
    #[doc = "Masked"]
    TCIEN_A = 0x0,
    #[doc = "Enabled"]
    TCIEN_B = 0x01,
}
impl Tcien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcien {
    #[inline(always)]
    fn from(val: u8) -> Tcien {
        Tcien::from_bits(val)
    }
}
impl From<Tcien> for u8 {
    #[inline(always)]
    fn from(val: Tcien) -> u8 {
        Tcien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcsen {
    #[doc = "Masked"]
    TCSEN_B = 0x0,
    #[doc = "Enabled"]
    TCSEN_A = 0x01,
}
impl Tcsen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcsen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcsen {
    #[inline(always)]
    fn from(val: u8) -> Tcsen {
        Tcsen::from_bits(val)
    }
}
impl From<Tcsen> for u8 {
    #[inline(always)]
    fn from(val: Tcsen) -> u8 {
        Tcsen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tneien {
    #[doc = "Masked"]
    TNEIEN_B = 0x0,
    #[doc = "Enabled"]
    TNEIEN_A = 0x01,
}
impl Tneien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tneien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tneien {
    #[inline(always)]
    fn from(val: u8) -> Tneien {
        Tneien::from_bits(val)
    }
}
impl From<Tneien> for u8 {
    #[inline(always)]
    fn from(val: Tneien) -> u8 {
        Tneien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tnesen {
    #[doc = "Masked"]
    TNESEN_A = 0x0,
    #[doc = "Enabled"]
    TNESEN_B = 0x01,
}
impl Tnesen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tnesen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tnesen {
    #[inline(always)]
    fn from(val: u8) -> Tnesen {
        Tnesen::from_bits(val)
    }
}
impl From<Tnesen> for u8 {
    #[inline(always)]
    fn from(val: Tnesen) -> u8 {
        Tnesen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpien {
    #[doc = "Masked"]
    TPIEN_R = 0x0,
    #[doc = "Enabled"]
    TPIEN_Q = 0x01,
}
impl Tpien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpien {
    #[inline(always)]
    fn from(val: u8) -> Tpien {
        Tpien::from_bits(val)
    }
}
impl From<Tpien> for u8 {
    #[inline(always)]
    fn from(val: Tpien) -> u8 {
        Tpien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpsen {
    #[doc = "Masked"]
    TPSEN_A = 0x0,
    #[doc = "Enabled"]
    TPSEN_B = 0x01,
}
impl Tpsen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpsen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpsen {
    #[inline(always)]
    fn from(val: u8) -> Tpsen {
        Tpsen::from_bits(val)
    }
}
impl From<Tpsen> for u8 {
    #[inline(always)]
    fn from(val: Tpsen) -> u8 {
        Tpsen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tscd {
    #[doc = "Delay cell select change is not finished."]
    TSCD_B = 0x0,
    #[doc = "Delay cell select change is finished."]
    TSCD_A = 0x01,
}
impl Tscd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tscd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tscd {
    #[inline(always)]
    fn from(val: u8) -> Tscd {
        Tscd::from_bits(val)
    }
}
impl From<Tscd> for u8 {
    #[inline(always)]
    fn from(val: Tscd) -> u8 {
        Tscd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TuningBitEn {
    #[doc = "Enable Tuning circuit for DATA\\[3:0\\]"]
    TUNING_BIT_EN_0 = 0x0,
    #[doc = "Enable Tuning circuit for DATA\\[7:0\\]"]
    TUNING_BIT_EN_1 = 0x01,
    #[doc = "Enable Tuning circuit for DATA\\[0\\]"]
    TUNING_BIT_EN_2 = 0x02,
    #[doc = "Invalid"]
    TUNING_BIT_EN_3 = 0x03,
}
impl TuningBitEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TuningBitEn {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TuningBitEn {
    #[inline(always)]
    fn from(val: u8) -> TuningBitEn {
        TuningBitEn::from_bits(val)
    }
}
impl From<TuningBitEn> for u8 {
    #[inline(always)]
    fn from(val: TuningBitEn) -> u8 {
        TuningBitEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UseTuningSdr50 {
    #[doc = "SDR50 does not support tuning"]
    USE_TUNING_B = 0x0,
    #[doc = "SDR50 supports tuning"]
    USE_TUNING_A = 0x01,
}
impl UseTuningSdr50 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UseTuningSdr50 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UseTuningSdr50 {
    #[inline(always)]
    fn from(val: u8) -> UseTuningSdr50 {
        UseTuningSdr50::from_bits(val)
    }
}
impl From<UseTuningSdr50> for u8 {
    #[inline(always)]
    fn from(val: UseTuningSdr50) -> u8 {
        UseTuningSdr50::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vs18 {
    #[doc = "1.8 V not supported"]
    VS18_B = 0x0,
    #[doc = "1.8 V supported"]
    VS18_A = 0x01,
}
impl Vs18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vs18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vs18 {
    #[inline(always)]
    fn from(val: u8) -> Vs18 {
        Vs18::from_bits(val)
    }
}
impl From<Vs18> for u8 {
    #[inline(always)]
    fn from(val: Vs18) -> u8 {
        Vs18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vs30 {
    #[doc = "3.0 V not supported"]
    VS30_B = 0x0,
    #[doc = "3.0 V supported"]
    VS30_A = 0x01,
}
impl Vs30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vs30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vs30 {
    #[inline(always)]
    fn from(val: u8) -> Vs30 {
        Vs30::from_bits(val)
    }
}
impl From<Vs30> for u8 {
    #[inline(always)]
    fn from(val: Vs30) -> u8 {
        Vs30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vs33 {
    #[doc = "3.3 V not supported"]
    VS33_B = 0x0,
    #[doc = "3.3 V supported"]
    VS33_A = 0x01,
}
impl Vs33 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vs33 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vs33 {
    #[inline(always)]
    fn from(val: u8) -> Vs33 {
        Vs33::from_bits(val)
    }
}
impl From<Vs33> for u8 {
    #[inline(always)]
    fn from(val: Vs33) -> u8 {
        Vs33::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wecins {
    #[doc = "Disable wakeup event enable on SD card insertion"]
    WECINS_B = 0x0,
    #[doc = "Enable wakeup event enable on SD card insertion"]
    WECINS_A = 0x01,
}
impl Wecins {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wecins {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wecins {
    #[inline(always)]
    fn from(val: u8) -> Wecins {
        Wecins::from_bits(val)
    }
}
impl From<Wecins> for u8 {
    #[inline(always)]
    fn from(val: Wecins) -> u8 {
        Wecins::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wecint {
    #[doc = "Disables wakeup event enable on card interrupt"]
    WECINT_A = 0x0,
    #[doc = "Enables wakeup event enable on card interrupt"]
    WECINT_B = 0x01,
}
impl Wecint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wecint {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wecint {
    #[inline(always)]
    fn from(val: u8) -> Wecint {
        Wecint::from_bits(val)
    }
}
impl From<Wecint> for u8 {
    #[inline(always)]
    fn from(val: Wecint) -> u8 {
        Wecint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wecrm {
    #[doc = "Disables wakeup event enable on SD card removal"]
    WECRM_B = 0x0,
    #[doc = "Enables wakeup event enable on SD card removal"]
    WECRM_A = 0x01,
}
impl Wecrm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wecrm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wecrm {
    #[inline(always)]
    fn from(val: u8) -> Wecrm {
        Wecrm::from_bits(val)
    }
}
impl From<Wecrm> for u8 {
    #[inline(always)]
    fn from(val: Wecrm) -> u8 {
        Wecrm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wta {
    #[doc = "No valid data"]
    WTA_B = 0x0,
    #[doc = "Transferring data"]
    WTA_A = 0x01,
}
impl Wta {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wta {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wta {
    #[inline(always)]
    fn from(val: u8) -> Wta {
        Wta::from_bits(val)
    }
}
impl From<Wta> for u8 {
    #[inline(always)]
    fn from(val: Wta) -> u8 {
        Wta::to_bits(val)
    }
}
