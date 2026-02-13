#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Axibuserr {
    #[doc = "No AXI bus error."]
    AXIBUSERR_0 = 0x0,
    #[doc = "AXI bus error occurs."]
    AXIBUSERR_1 = 0x01,
}
impl Axibuserr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Axibuserr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Axibuserr {
    #[inline(always)]
    fn from(val: u8) -> Axibuserr {
        Axibuserr::from_bits(val)
    }
}
impl From<Axibuserr> for u8 {
    #[inline(always)]
    fn from(val: Axibuserr) -> u8 {
        Axibuserr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Axibuserren {
    #[doc = "Interrupt is disabled"]
    AXIBUSERREN_0 = 0x0,
    #[doc = "Interrupt is enabled"]
    AXIBUSERREN_1 = 0x01,
}
impl Axibuserren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Axibuserren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Axibuserren {
    #[inline(always)]
    fn from(val: u8) -> Axibuserren {
        Axibuserren::from_bits(val)
    }
}
impl From<Axibuserren> for u8 {
    #[inline(always)]
    fn from(val: Axibuserren) -> u8 {
        Axibuserren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Axicmderr {
    #[doc = "No AXI command error."]
    AXICMDERR_0 = 0x0,
    #[doc = "AXI command error occurs."]
    AXICMDERR_1 = 0x01,
}
impl Axicmderr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Axicmderr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Axicmderr {
    #[inline(always)]
    fn from(val: u8) -> Axicmderr {
        Axicmderr::from_bits(val)
    }
}
impl From<Axicmderr> for u8 {
    #[inline(always)]
    fn from(val: Axicmderr) -> u8 {
        Axicmderr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Axicmderren {
    #[doc = "Interrupt is disabled"]
    AXICMDERREN_0 = 0x0,
    #[doc = "Interrupt is enabled"]
    AXICMDERREN_1 = 0x01,
}
impl Axicmderren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Axicmderren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Axicmderren {
    #[inline(always)]
    fn from(val: u8) -> Axicmderren {
        Axicmderren::from_bits(val)
    }
}
impl From<Axicmderren> for u8 {
    #[inline(always)]
    fn from(val: Axicmderren) -> u8 {
        Axicmderren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bank2 {
    #[doc = "SDRAM device has 4 banks."]
    BANK2_0 = 0x0,
    #[doc = "SDRAM device has 2 banks."]
    BANK2_1 = 0x01,
}
impl Bank2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bank2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bank2 {
    #[inline(always)]
    fn from(val: u8) -> Bank2 {
        Bank2::from_bits(val)
    }
}
impl From<Bank2> for u8 {
    #[inline(always)]
    fn from(val: Bank2) -> u8 {
        Bank2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bm0 {
    #[doc = "Byte is unmasked"]
    BM0_0 = 0x0,
    #[doc = "Byte is masked"]
    BM0_1 = 0x01,
}
impl Bm0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bm0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bm0 {
    #[inline(always)]
    fn from(val: u8) -> Bm0 {
        Bm0::from_bits(val)
    }
}
impl From<Bm0> for u8 {
    #[inline(always)]
    fn from(val: Bm0) -> u8 {
        Bm0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bm1 {
    #[doc = "Byte is unmasked"]
    BM1_0 = 0x0,
    #[doc = "Byte is masked"]
    BM1_1 = 0x01,
}
impl Bm1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bm1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bm1 {
    #[inline(always)]
    fn from(val: u8) -> Bm1 {
        Bm1::from_bits(val)
    }
}
impl From<Bm1> for u8 {
    #[inline(always)]
    fn from(val: Bm1) -> u8 {
        Bm1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bm2 {
    #[doc = "Byte is unmasked"]
    BM2_0 = 0x0,
    #[doc = "Byte is masked"]
    BM2_1 = 0x01,
}
impl Bm2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bm2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bm2 {
    #[inline(always)]
    fn from(val: u8) -> Bm2 {
        Bm2::from_bits(val)
    }
}
impl From<Bm2> for u8 {
    #[inline(always)]
    fn from(val: Bm2) -> u8 {
        Bm2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bm3 {
    #[doc = "Byte is unmasked"]
    BM3_0 = 0x0,
    #[doc = "Byte is masked"]
    BM3_1 = 0x01,
}
impl Bm3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bm3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bm3 {
    #[inline(always)]
    fn from(val: u8) -> Bm3 {
        Bm3::from_bits(val)
    }
}
impl From<Bm3> for u8 {
    #[inline(always)]
    fn from(val: Bm3) -> u8 {
        Bm3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bto {
    #[doc = "255*1"]
    BTO_0 = 0x0,
    #[doc = "255*2"]
    BTO_1 = 0x01,
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
    #[doc = "255*231"]
    BTO_31 = 0x1f,
}
impl Bto {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bto {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bto {
    #[inline(always)]
    fn from(val: u8) -> Bto {
        Bto::from_bits(val)
    }
}
impl From<Bto> for u8 {
    #[inline(always)]
    fn from(val: Bto) -> u8 {
        Bto::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cl {
    #[doc = "1"]
    CL_0 = 0x0,
    #[doc = "1"]
    CL_1 = 0x01,
    #[doc = "2"]
    CL_2 = 0x02,
    #[doc = "3"]
    CL_3 = 0x03,
}
impl Cl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cl {
    #[inline(always)]
    fn from(val: u8) -> Cl {
        Cl::from_bits(val)
    }
}
impl From<Cl> for u8 {
    #[inline(always)]
    fn from(val: Cl) -> u8 {
        Cl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Col8 {
    #[doc = "Column address bit number is decided by COL field."]
    COL8_0 = 0x0,
    #[doc = "Column address bit number is 8. COL field is ignored."]
    COL8_1 = 0x01,
}
impl Col8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Col8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Col8 {
    #[inline(always)]
    fn from(val: u8) -> Col8 {
        Col8::from_bits(val)
    }
}
impl From<Col8> for u8 {
    #[inline(always)]
    fn from(val: Col8) -> u8 {
        Col8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Datsz {
    #[doc = "4"]
    DATSZ_0 = 0x0,
    #[doc = "1"]
    DATSZ_1 = 0x01,
    #[doc = "2"]
    DATSZ_2 = 0x02,
    #[doc = "3"]
    DATSZ_3 = 0x03,
    #[doc = "4"]
    DATSZ_4 = 0x04,
    #[doc = "4"]
    DATSZ_5 = 0x05,
    #[doc = "4"]
    DATSZ_6 = 0x06,
    #[doc = "4"]
    DATSZ_7 = 0x07,
}
impl Datsz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Datsz {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Datsz {
    #[inline(always)]
    fn from(val: u8) -> Datsz {
        Datsz::from_bits(val)
    }
}
impl From<Datsz> for u8 {
    #[inline(always)]
    fn from(val: Datsz) -> u8 {
        Datsz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dbicr0Bl {
    #[doc = "1"]
    BL_0 = 0x0,
    #[doc = "2"]
    BL_1 = 0x01,
    #[doc = "4"]
    BL_2 = 0x02,
    #[doc = "8"]
    BL_3 = 0x03,
    #[doc = "16"]
    BL_4 = 0x04,
    #[doc = "32"]
    BL_5 = 0x05,
    #[doc = "64"]
    BL_6 = 0x06,
    #[doc = "64"]
    BL_7 = 0x07,
}
impl Dbicr0Bl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dbicr0Bl {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dbicr0Bl {
    #[inline(always)]
    fn from(val: u8) -> Dbicr0Bl {
        Dbicr0Bl::from_bits(val)
    }
}
impl From<Dbicr0Bl> for u8 {
    #[inline(always)]
    fn from(val: Dbicr0Bl) -> u8 {
        Dbicr0Bl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dbicr0Col {
    #[doc = "12 Bits"]
    COL_0 = 0x0,
    #[doc = "11 Bits"]
    COL_1 = 0x01,
    #[doc = "10 Bits"]
    COL_2 = 0x02,
    #[doc = "9 Bits"]
    COL_3 = 0x03,
    #[doc = "8 Bits"]
    COL_4 = 0x04,
    #[doc = "7 Bits"]
    COL_5 = 0x05,
    #[doc = "6 Bits"]
    COL_6 = 0x06,
    #[doc = "5 Bits"]
    COL_7 = 0x07,
    #[doc = "4 Bits"]
    COL_8 = 0x08,
    #[doc = "3 Bits"]
    COL_9 = 0x09,
    #[doc = "2 Bits"]
    COL_10 = 0x0a,
    #[doc = "12 Bits"]
    COL_11 = 0x0b,
    #[doc = "12 Bits"]
    COL_12 = 0x0c,
    #[doc = "12 Bits"]
    COL_13 = 0x0d,
    #[doc = "12 Bits"]
    COL_14 = 0x0e,
    #[doc = "12 Bits"]
    COL_15 = 0x0f,
}
impl Dbicr0Col {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dbicr0Col {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dbicr0Col {
    #[inline(always)]
    fn from(val: u8) -> Dbicr0Col {
        Dbicr0Col::from_bits(val)
    }
}
impl From<Dbicr0Col> for u8 {
    #[inline(always)]
    fn from(val: Dbicr0Col) -> u8 {
        Dbicr0Col::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dbicr0Ps {
    #[doc = "8bit"]
    PS_0 = 0x0,
    #[doc = "16bit"]
    PS_1 = 0x01,
}
impl Dbicr0Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dbicr0Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dbicr0Ps {
    #[inline(always)]
    fn from(val: u8) -> Dbicr0Ps {
        Dbicr0Ps::from_bits(val)
    }
}
impl From<Dbicr0Ps> for u8 {
    #[inline(always)]
    fn from(val: Dbicr0Ps) -> u8 {
        Dbicr0Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dllen {
    #[doc = "DLL calibration is disabled."]
    DLLEN_0 = 0x0,
    #[doc = "DLL calibration is enabled."]
    DLLEN_1 = 0x01,
}
impl Dllen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dllen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dllen {
    #[inline(always)]
    fn from(val: u8) -> Dllen {
        Dllen::from_bits(val)
    }
}
impl From<Dllen> for u8 {
    #[inline(always)]
    fn from(val: Dllen) -> u8 {
        Dllen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dllreset {
    #[doc = "DLL is not reset."]
    DLLRESET_0 = 0x0,
    #[doc = "DLL is reset."]
    DLLRESET_1 = 0x01,
}
impl Dllreset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dllreset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dllreset {
    #[inline(always)]
    fn from(val: u8) -> Dllreset {
        Dllreset::from_bits(val)
    }
}
impl From<Dllreset> for u8 {
    #[inline(always)]
    fn from(val: Dllreset) -> u8 {
        Dllreset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dqsmd {
    #[doc = "Dummy read strobe loopbacked internally"]
    DQSMD_0 = 0x0,
    #[doc = "Dummy read strobe loopbacked from DQS pad"]
    DQSMD_1 = 0x01,
}
impl Dqsmd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dqsmd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dqsmd {
    #[inline(always)]
    fn from(val: u8) -> Dqsmd {
        Dqsmd::from_bits(val)
    }
}
impl From<Dqsmd> for u8 {
    #[inline(always)]
    fn from(val: Dqsmd) -> u8 {
        Dqsmd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edo {
    #[doc = "EDO mode disabled"]
    EDO_0 = 0x0,
    #[doc = "EDO mode enabled"]
    EDO_1 = 0x01,
}
impl Edo {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edo {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edo {
    #[inline(always)]
    fn from(val: u8) -> Edo {
        Edo::from_bits(val)
    }
}
impl From<Edo> for u8 {
    #[inline(always)]
    fn from(val: Edo) -> u8 {
        Edo::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipcmddone {
    #[doc = "IP command is not done."]
    IPCMDDONE_0 = 0x0,
    #[doc = "IP command is done."]
    IPCMDDONE_1 = 0x01,
}
impl Ipcmddone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipcmddone {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipcmddone {
    #[inline(always)]
    fn from(val: u8) -> Ipcmddone {
        Ipcmddone::from_bits(val)
    }
}
impl From<Ipcmddone> for u8 {
    #[inline(always)]
    fn from(val: Ipcmddone) -> u8 {
        Ipcmddone::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipcmddoneen {
    #[doc = "Interrupt is disabled"]
    IPCMDDONEEN_0 = 0x0,
    #[doc = "Interrupt is enabled"]
    IPCMDDONEEN_1 = 0x01,
}
impl Ipcmddoneen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipcmddoneen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipcmddoneen {
    #[inline(always)]
    fn from(val: u8) -> Ipcmddoneen {
        Ipcmddoneen::from_bits(val)
    }
}
impl From<Ipcmddoneen> for u8 {
    #[inline(always)]
    fn from(val: Ipcmddoneen) -> u8 {
        Ipcmddoneen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipcmderr {
    #[doc = "No IP command error."]
    IPCMDERR_0 = 0x0,
    #[doc = "IP command error occurs."]
    IPCMDERR_1 = 0x01,
}
impl Ipcmderr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipcmderr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipcmderr {
    #[inline(always)]
    fn from(val: u8) -> Ipcmderr {
        Ipcmderr::from_bits(val)
    }
}
impl From<Ipcmderr> for u8 {
    #[inline(always)]
    fn from(val: Ipcmderr) -> u8 {
        Ipcmderr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipcmderren {
    #[doc = "Interrupt is disabled"]
    IPCMDERREN_0 = 0x0,
    #[doc = "Interrupt is enabled"]
    IPCMDERREN_1 = 0x01,
}
impl Ipcmderren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipcmderren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipcmderren {
    #[inline(always)]
    fn from(val: u8) -> Ipcmderren {
        Ipcmderren::from_bits(val)
    }
}
impl From<Ipcmderren> for u8 {
    #[inline(always)]
    fn from(val: Ipcmderren) -> u8 {
        Ipcmderren::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ito(u8);
impl Ito {
    #[doc = "IDLE timeout period is 256*Prescale period."]
    pub const ITO_0: Self = Self(0x0);
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    pub const ITO_1: Self = Self(0x01);
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    pub const ITO_2: Self = Self(0x02);
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    pub const ITO_3: Self = Self(0x03);
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    pub const ITO_4: Self = Self(0x04);
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    pub const ITO_5: Self = Self(0x05);
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    pub const ITO_6: Self = Self(0x06);
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    pub const ITO_7: Self = Self(0x07);
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    pub const ITO_8: Self = Self(0x08);
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    pub const ITO_9: Self = Self(0x09);
}
impl Ito {
    pub const fn from_bits(val: u8) -> Ito {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Ito {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("ITO_0"),
            0x01 => f.write_str("ITO_1"),
            0x02 => f.write_str("ITO_2"),
            0x03 => f.write_str("ITO_3"),
            0x04 => f.write_str("ITO_4"),
            0x05 => f.write_str("ITO_5"),
            0x06 => f.write_str("ITO_6"),
            0x07 => f.write_str("ITO_7"),
            0x08 => f.write_str("ITO_8"),
            0x09 => f.write_str("ITO_9"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ito {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "ITO_0"),
            0x01 => defmt::write!(f, "ITO_1"),
            0x02 => defmt::write!(f, "ITO_2"),
            0x03 => defmt::write!(f, "ITO_3"),
            0x04 => defmt::write!(f, "ITO_4"),
            0x05 => defmt::write!(f, "ITO_5"),
            0x06 => defmt::write!(f, "ITO_6"),
            0x07 => defmt::write!(f, "ITO_7"),
            0x08 => defmt::write!(f, "ITO_8"),
            0x09 => defmt::write!(f, "ITO_9"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Ito {
    #[inline(always)]
    fn from(val: u8) -> Ito {
        Ito::from_bits(val)
    }
}
impl From<Ito> for u8 {
    #[inline(always)]
    fn from(val: Ito) -> u8 {
        Ito::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mdis {
    #[doc = "Module enabled"]
    MDIS_0 = 0x0,
    #[doc = "Module disabled"]
    MDIS_1 = 0x01,
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
pub enum Ms {
    #[doc = "4KB"]
    MS_0 = 0x0,
    #[doc = "8KB"]
    MS_1 = 0x01,
    #[doc = "16KB"]
    MS_2 = 0x02,
    #[doc = "32KB"]
    MS_3 = 0x03,
    #[doc = "64KB"]
    MS_4 = 0x04,
    #[doc = "128KB"]
    MS_5 = 0x05,
    #[doc = "256KB"]
    MS_6 = 0x06,
    #[doc = "512KB"]
    MS_7 = 0x07,
    #[doc = "1MB"]
    MS_8 = 0x08,
    #[doc = "2MB"]
    MS_9 = 0x09,
    #[doc = "4MB"]
    MS_10 = 0x0a,
    #[doc = "8MB"]
    MS_11 = 0x0b,
    #[doc = "16MB"]
    MS_12 = 0x0c,
    #[doc = "32MB"]
    MS_13 = 0x0d,
    #[doc = "64MB"]
    MS_14 = 0x0e,
    #[doc = "128MB"]
    MS_15 = 0x0f,
    #[doc = "256MB"]
    MS_16 = 0x10,
    #[doc = "512MB"]
    MS_17 = 0x11,
    #[doc = "1GB"]
    MS_18 = 0x12,
    #[doc = "2GB"]
    MS_19 = 0x13,
    #[doc = "4GB"]
    MS_20 = 0x14,
    #[doc = "4GB"]
    MS_21 = 0x15,
    #[doc = "4GB"]
    MS_22 = 0x16,
    #[doc = "4GB"]
    MS_23 = 0x17,
    #[doc = "4GB"]
    MS_24 = 0x18,
    #[doc = "4GB"]
    MS_25 = 0x19,
    #[doc = "4GB"]
    MS_26 = 0x1a,
    #[doc = "4GB"]
    MS_27 = 0x1b,
    #[doc = "4GB"]
    MS_28 = 0x1c,
    #[doc = "4GB"]
    MS_29 = 0x1d,
    #[doc = "4GB"]
    MS_30 = 0x1e,
    #[doc = "4GB"]
    MS_31 = 0x1f,
}
impl Ms {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ms {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ms {
    #[inline(always)]
    fn from(val: u8) -> Ms {
        Ms::from_bits(val)
    }
}
impl From<Ms> for u8 {
    #[inline(always)]
    fn from(val: Ms) -> u8 {
        Ms::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MuxA8 {
    #[doc = "SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode"]
    MUX_A8_0 = 0x0,
    #[doc = "NAND CE#"]
    MUX_A8_1 = 0x01,
    #[doc = "NOR CE#"]
    MUX_A8_2 = 0x02,
    #[doc = "SRAM CE#"]
    MUX_A8_3 = 0x03,
    #[doc = "DBI CSX"]
    MUX_A8_4 = 0x04,
    #[doc = "SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode"]
    MUX_A8_5 = 0x05,
    #[doc = "SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode"]
    MUX_A8_6 = 0x06,
    #[doc = "SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode"]
    MUX_A8_7 = 0x07,
}
impl MuxA8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MuxA8 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MuxA8 {
    #[inline(always)]
    fn from(val: u8) -> MuxA8 {
        MuxA8::from_bits(val)
    }
}
impl From<MuxA8> for u8 {
    #[inline(always)]
    fn from(val: MuxA8) -> u8 {
        MuxA8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MuxClkx0 {
    #[doc = "NOR clock"]
    MUX_CLKX0_0 = 0x0,
    #[doc = "SRAM clock"]
    MUX_CLKX0_1 = 0x01,
}
impl MuxClkx0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MuxClkx0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MuxClkx0 {
    #[inline(always)]
    fn from(val: u8) -> MuxClkx0 {
        MuxClkx0::from_bits(val)
    }
}
impl From<MuxClkx0> for u8 {
    #[inline(always)]
    fn from(val: MuxClkx0) -> u8 {
        MuxClkx0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MuxClkx1 {
    #[doc = "NOR clock"]
    MUX_CLKX1_0 = 0x0,
    #[doc = "SRAM clock"]
    MUX_CLKX1_1 = 0x01,
}
impl MuxClkx1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MuxClkx1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MuxClkx1 {
    #[inline(always)]
    fn from(val: u8) -> MuxClkx1 {
        MuxClkx1::from_bits(val)
    }
}
impl From<MuxClkx1> for u8 {
    #[inline(always)]
    fn from(val: MuxClkx1) -> u8 {
        MuxClkx1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MuxCsx0 {
    _RESERVED_0 = 0x0,
    #[doc = "SDRAM CS1"]
    MUX_CSX0_1 = 0x01,
    #[doc = "SDRAM CS2"]
    MUX_CSX0_2 = 0x02,
    #[doc = "SDRAM CS3"]
    MUX_CSX0_3 = 0x03,
    #[doc = "NAND CE#"]
    MUX_CSX0_4 = 0x04,
    #[doc = "NOR CE#"]
    MUX_CSX0_5 = 0x05,
    #[doc = "SRAM CE#"]
    MUX_CSX0_6 = 0x06,
    #[doc = "DBI CSX"]
    MUX_CSX0_7 = 0x07,
}
impl MuxCsx0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MuxCsx0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MuxCsx0 {
    #[inline(always)]
    fn from(val: u8) -> MuxCsx0 {
        MuxCsx0::from_bits(val)
    }
}
impl From<MuxCsx0> for u8 {
    #[inline(always)]
    fn from(val: MuxCsx0) -> u8 {
        MuxCsx0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MuxCsx1 {
    _RESERVED_0 = 0x0,
    #[doc = "SDRAM CS1"]
    MUX_CSX1_1 = 0x01,
    #[doc = "SDRAM CS2"]
    MUX_CSX1_2 = 0x02,
    #[doc = "SDRAM CS3"]
    MUX_CSX1_3 = 0x03,
    #[doc = "NAND CE#"]
    MUX_CSX1_4 = 0x04,
    #[doc = "NOR CE#"]
    MUX_CSX1_5 = 0x05,
    #[doc = "SRAM CE#"]
    MUX_CSX1_6 = 0x06,
    #[doc = "DBI CSX"]
    MUX_CSX1_7 = 0x07,
}
impl MuxCsx1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MuxCsx1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MuxCsx1 {
    #[inline(always)]
    fn from(val: u8) -> MuxCsx1 {
        MuxCsx1::from_bits(val)
    }
}
impl From<MuxCsx1> for u8 {
    #[inline(always)]
    fn from(val: MuxCsx1) -> u8 {
        MuxCsx1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MuxCsx2 {
    _RESERVED_0 = 0x0,
    #[doc = "SDRAM CS1"]
    MUX_CSX2_1 = 0x01,
    #[doc = "SDRAM CS2"]
    MUX_CSX2_2 = 0x02,
    #[doc = "SDRAM CS3"]
    MUX_CSX2_3 = 0x03,
    #[doc = "NAND CE#"]
    MUX_CSX2_4 = 0x04,
    #[doc = "NOR CE#"]
    MUX_CSX2_5 = 0x05,
    #[doc = "SRAM CE#"]
    MUX_CSX2_6 = 0x06,
    #[doc = "DBI CSX"]
    MUX_CSX2_7 = 0x07,
}
impl MuxCsx2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MuxCsx2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MuxCsx2 {
    #[inline(always)]
    fn from(val: u8) -> MuxCsx2 {
        MuxCsx2::from_bits(val)
    }
}
impl From<MuxCsx2> for u8 {
    #[inline(always)]
    fn from(val: MuxCsx2) -> u8 {
        MuxCsx2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MuxCsx3 {
    _RESERVED_0 = 0x0,
    #[doc = "SDRAM CS1"]
    MUX_CSX3_1 = 0x01,
    #[doc = "SDRAM CS2"]
    MUX_CSX3_2 = 0x02,
    #[doc = "SDRAM CS3"]
    MUX_CSX3_3 = 0x03,
    #[doc = "NAND CE#"]
    MUX_CSX3_4 = 0x04,
    #[doc = "NOR CE#"]
    MUX_CSX3_5 = 0x05,
    #[doc = "SRAM CE#"]
    MUX_CSX3_6 = 0x06,
    #[doc = "DBI CSX"]
    MUX_CSX3_7 = 0x07,
}
impl MuxCsx3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MuxCsx3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MuxCsx3 {
    #[inline(always)]
    fn from(val: u8) -> MuxCsx3 {
        MuxCsx3::from_bits(val)
    }
}
impl From<MuxCsx3> for u8 {
    #[inline(always)]
    fn from(val: MuxCsx3) -> u8 {
        MuxCsx3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MuxRdy {
    #[doc = "NAND R/B# input"]
    MUX_RDY_0 = 0x0,
    #[doc = "SDRAM CS1"]
    MUX_RDY_1 = 0x01,
    #[doc = "SDRAM CS2"]
    MUX_RDY_2 = 0x02,
    #[doc = "SDRAM CS3"]
    MUX_RDY_3 = 0x03,
    #[doc = "NOR CE#"]
    MUX_RDY_4 = 0x04,
    #[doc = "SRAM CE#"]
    MUX_RDY_5 = 0x05,
    #[doc = "DBI CSX"]
    MUX_RDY_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl MuxRdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MuxRdy {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MuxRdy {
    #[inline(always)]
    fn from(val: u8) -> MuxRdy {
        MuxRdy::from_bits(val)
    }
}
impl From<MuxRdy> for u8 {
    #[inline(always)]
    fn from(val: MuxRdy) -> u8 {
        MuxRdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nandcr0Bl {
    #[doc = "1"]
    BL_0 = 0x0,
    #[doc = "2"]
    BL_1 = 0x01,
    #[doc = "4"]
    BL_2 = 0x02,
    #[doc = "8"]
    BL_3 = 0x03,
    #[doc = "16"]
    BL_4 = 0x04,
    #[doc = "32"]
    BL_5 = 0x05,
    #[doc = "64"]
    BL_6 = 0x06,
    #[doc = "64"]
    BL_7 = 0x07,
}
impl Nandcr0Bl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nandcr0Bl {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nandcr0Bl {
    #[inline(always)]
    fn from(val: u8) -> Nandcr0Bl {
        Nandcr0Bl::from_bits(val)
    }
}
impl From<Nandcr0Bl> for u8 {
    #[inline(always)]
    fn from(val: Nandcr0Bl) -> u8 {
        Nandcr0Bl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nandcr0Col {
    #[doc = "16"]
    COL_0 = 0x0,
    #[doc = "15"]
    COL_1 = 0x01,
    #[doc = "14"]
    COL_2 = 0x02,
    #[doc = "13"]
    COL_3 = 0x03,
    #[doc = "12"]
    COL_4 = 0x04,
    #[doc = "11"]
    COL_5 = 0x05,
    #[doc = "10"]
    COL_6 = 0x06,
    #[doc = "9"]
    COL_7 = 0x07,
}
impl Nandcr0Col {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nandcr0Col {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nandcr0Col {
    #[inline(always)]
    fn from(val: u8) -> Nandcr0Col {
        Nandcr0Col::from_bits(val)
    }
}
impl From<Nandcr0Col> for u8 {
    #[inline(always)]
    fn from(val: Nandcr0Col) -> u8 {
        Nandcr0Col::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nandcr0Ps {
    #[doc = "8bit"]
    PS_0 = 0x0,
    #[doc = "16bit"]
    PS_1 = 0x01,
}
impl Nandcr0Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nandcr0Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nandcr0Ps {
    #[inline(always)]
    fn from(val: u8) -> Nandcr0Ps {
        Nandcr0Ps::from_bits(val)
    }
}
impl From<Nandcr0Ps> for u8 {
    #[inline(always)]
    fn from(val: Nandcr0Ps) -> u8 {
        Nandcr0Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nandcr0Syncen {
    #[doc = "Asynchronous mode is enabled."]
    SYNCEN_0 = 0x0,
    #[doc = "Synchronous mode is enabled."]
    SYNCEN_1 = 0x01,
}
impl Nandcr0Syncen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nandcr0Syncen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nandcr0Syncen {
    #[inline(always)]
    fn from(val: u8) -> Nandcr0Syncen {
        Nandcr0Syncen::from_bits(val)
    }
}
impl From<Nandcr0Syncen> for u8 {
    #[inline(always)]
    fn from(val: Nandcr0Syncen) -> u8 {
        Nandcr0Syncen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nardy {
    #[doc = "NAND device is not ready"]
    NARDY_0 = 0x0,
    #[doc = "NAND device is ready"]
    NARDY_1 = 0x01,
}
impl Nardy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nardy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nardy {
    #[inline(always)]
    fn from(val: u8) -> Nardy {
        Nardy::from_bits(val)
    }
}
impl From<Nardy> for u8 {
    #[inline(always)]
    fn from(val: Nardy) -> u8 {
        Nardy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ndnopend {
    #[doc = "At least one NAND AXI write transaction is pending or no NAND write transaction is sent to the queue."]
    NDNOPEND_0 = 0x0,
    #[doc = "All NAND AXI write pending transactions are finished."]
    NDNOPEND_1 = 0x01,
}
impl Ndnopend {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ndnopend {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ndnopend {
    #[inline(always)]
    fn from(val: u8) -> Ndnopend {
        Ndnopend::from_bits(val)
    }
}
impl From<Ndnopend> for u8 {
    #[inline(always)]
    fn from(val: Ndnopend) -> u8 {
        Ndnopend::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ndnopenden {
    #[doc = "Interrupt is disabled"]
    NDNOPENDEN_0 = 0x0,
    #[doc = "Interrupt is enabled"]
    NDNOPENDEN_1 = 0x01,
}
impl Ndnopenden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ndnopenden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ndnopenden {
    #[inline(always)]
    fn from(val: u8) -> Ndnopenden {
        Ndnopenden::from_bits(val)
    }
}
impl From<Ndnopenden> for u8 {
    #[inline(always)]
    fn from(val: Ndnopenden) -> u8 {
        Ndnopenden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ndpageend {
    #[doc = "The last address of main space in the NAND is not written by AXI command."]
    NDPAGEEND_0 = 0x0,
    #[doc = "The last address of main space in the NAND is written by AXI command."]
    NDPAGEEND_1 = 0x01,
}
impl Ndpageend {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ndpageend {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ndpageend {
    #[inline(always)]
    fn from(val: u8) -> Ndpageend {
        Ndpageend::from_bits(val)
    }
}
impl From<Ndpageend> for u8 {
    #[inline(always)]
    fn from(val: Ndpageend) -> u8 {
        Ndpageend::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ndpageenden {
    #[doc = "Interrupt is disabled"]
    NDPAGEENDEN_0 = 0x0,
    #[doc = "Interrupt is enabled"]
    NDPAGEENDEN_1 = 0x01,
}
impl Ndpageenden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ndpageenden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ndpageenden {
    #[inline(always)]
    fn from(val: u8) -> Ndpageenden {
        Ndpageenden::from_bits(val)
    }
}
impl From<Ndpageenden> for u8 {
    #[inline(always)]
    fn from(val: Ndpageenden) -> u8 {
        Ndpageenden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ndwrpend {
    #[doc = "No pending"]
    NDWRPEND_0 = 0x0,
    #[doc = "Pending"]
    NDWRPEND_1 = 0x01,
}
impl Ndwrpend {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ndwrpend {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ndwrpend {
    #[inline(always)]
    fn from(val: u8) -> Ndwrpend {
        Ndwrpend::from_bits(val)
    }
}
impl From<Ndwrpend> for u8 {
    #[inline(always)]
    fn from(val: Ndwrpend) -> u8 {
        Ndwrpend::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Norcr0Advh {
    #[doc = "ADV# is high during address hold state."]
    ADVH_0 = 0x0,
    #[doc = "ADV# is low during address hold state."]
    ADVH_1 = 0x01,
}
impl Norcr0Advh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Norcr0Advh {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Norcr0Advh {
    #[inline(always)]
    fn from(val: u8) -> Norcr0Advh {
        Norcr0Advh::from_bits(val)
    }
}
impl From<Norcr0Advh> for u8 {
    #[inline(always)]
    fn from(val: Norcr0Advh) -> u8 {
        Norcr0Advh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Norcr0Advp {
    #[doc = "ADV# is active low."]
    ADVP_0 = 0x0,
    #[doc = "ADV# is active high."]
    ADVP_1 = 0x01,
}
impl Norcr0Advp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Norcr0Advp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Norcr0Advp {
    #[inline(always)]
    fn from(val: u8) -> Norcr0Advp {
        Norcr0Advp::from_bits(val)
    }
}
impl From<Norcr0Advp> for u8 {
    #[inline(always)]
    fn from(val: Norcr0Advp) -> u8 {
        Norcr0Advp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Norcr0Am {
    #[doc = "Address/Data MUX mode (ADMUX)"]
    AM_0 = 0x0,
    #[doc = "Advanced Address/Data MUX mode (AADM)"]
    AM_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Norcr0Am {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Norcr0Am {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Norcr0Am {
    #[inline(always)]
    fn from(val: u8) -> Norcr0Am {
        Norcr0Am::from_bits(val)
    }
}
impl From<Norcr0Am> for u8 {
    #[inline(always)]
    fn from(val: Norcr0Am) -> u8 {
        Norcr0Am::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Norcr0Bl {
    #[doc = "1"]
    BL_0 = 0x0,
    #[doc = "2"]
    BL_1 = 0x01,
    #[doc = "4"]
    BL_2 = 0x02,
    #[doc = "8"]
    BL_3 = 0x03,
    #[doc = "16"]
    BL_4 = 0x04,
    #[doc = "32"]
    BL_5 = 0x05,
    #[doc = "64"]
    BL_6 = 0x06,
    #[doc = "64"]
    BL_7 = 0x07,
}
impl Norcr0Bl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Norcr0Bl {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Norcr0Bl {
    #[inline(always)]
    fn from(val: u8) -> Norcr0Bl {
        Norcr0Bl::from_bits(val)
    }
}
impl From<Norcr0Bl> for u8 {
    #[inline(always)]
    fn from(val: Norcr0Bl) -> u8 {
        Norcr0Bl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Norcr0Col {
    #[doc = "12 Bits"]
    COL_0 = 0x0,
    #[doc = "11 Bits"]
    COL_1 = 0x01,
    #[doc = "10 Bits"]
    COL_2 = 0x02,
    #[doc = "9 Bits"]
    COL_3 = 0x03,
    #[doc = "8 Bits"]
    COL_4 = 0x04,
    #[doc = "7 Bits"]
    COL_5 = 0x05,
    #[doc = "6 Bits"]
    COL_6 = 0x06,
    #[doc = "5 Bits"]
    COL_7 = 0x07,
    #[doc = "4 Bits"]
    COL_8 = 0x08,
    #[doc = "3 Bits"]
    COL_9 = 0x09,
    #[doc = "2 Bits"]
    COL_10 = 0x0a,
    #[doc = "12 Bits"]
    COL_11 = 0x0b,
    #[doc = "12 Bits"]
    COL_12 = 0x0c,
    #[doc = "12 Bits"]
    COL_13 = 0x0d,
    #[doc = "12 Bits"]
    COL_14 = 0x0e,
    #[doc = "12 Bits"]
    COL_15 = 0x0f,
}
impl Norcr0Col {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Norcr0Col {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Norcr0Col {
    #[inline(always)]
    fn from(val: u8) -> Norcr0Col {
        Norcr0Col::from_bits(val)
    }
}
impl From<Norcr0Col> for u8 {
    #[inline(always)]
    fn from(val: Norcr0Col) -> u8 {
        Norcr0Col::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Norcr0Ps {
    #[doc = "8bit"]
    PS_0 = 0x0,
    #[doc = "16bit"]
    PS_1 = 0x01,
}
impl Norcr0Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Norcr0Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Norcr0Ps {
    #[inline(always)]
    fn from(val: u8) -> Norcr0Ps {
        Norcr0Ps::from_bits(val)
    }
}
impl From<Norcr0Ps> for u8 {
    #[inline(always)]
    fn from(val: Norcr0Ps) -> u8 {
        Norcr0Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Norcr0Syncen {
    #[doc = "Asynchronous mode is enabled."]
    SYNCEN_0 = 0x0,
    #[doc = "Synchronous mode is enabled. Only fixed latency mode is supported."]
    SYNCEN_1 = 0x01,
}
impl Norcr0Syncen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Norcr0Syncen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Norcr0Syncen {
    #[inline(always)]
    fn from(val: u8) -> Norcr0Syncen {
        Norcr0Syncen::from_bits(val)
    }
}
impl From<Norcr0Syncen> for u8 {
    #[inline(always)]
    fn from(val: Norcr0Syncen) -> u8 {
        Norcr0Syncen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ovrden {
    #[doc = "The delay cell number is not overridden."]
    OVRDEN_0 = 0x0,
    #[doc = "The delay cell number is overridden."]
    OVRDEN_1 = 0x01,
}
impl Ovrden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ovrden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ovrden {
    #[inline(always)]
    fn from(val: u8) -> Ovrden {
        Ovrden::from_bits(val)
    }
}
impl From<Ovrden> for u8 {
    #[inline(always)]
    fn from(val: Ovrden) -> u8 {
        Ovrden::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Prescale(u8);
impl Prescale {
    #[doc = "(256*16+1) clock cycles"]
    pub const PRESCALE_0: Self = Self(0x0);
    #[doc = "(PRESCALE*16+1) clock cycles"]
    pub const PRESCALE_1: Self = Self(0x01);
    #[doc = "(PRESCALE*16+1) clock cycles"]
    pub const PRESCALE_2: Self = Self(0x02);
    #[doc = "(PRESCALE*16+1) clock cycles"]
    pub const PRESCALE_3: Self = Self(0x03);
    #[doc = "(PRESCALE*16+1) clock cycles"]
    pub const PRESCALE_4: Self = Self(0x04);
    #[doc = "(PRESCALE*16+1) clock cycles"]
    pub const PRESCALE_5: Self = Self(0x05);
    #[doc = "(PRESCALE*16+1) clock cycles"]
    pub const PRESCALE_6: Self = Self(0x06);
    #[doc = "(PRESCALE*16+1) clock cycles"]
    pub const PRESCALE_7: Self = Self(0x07);
    #[doc = "(PRESCALE*16+1) clock cycles"]
    pub const PRESCALE_8: Self = Self(0x08);
    #[doc = "(PRESCALE*16+1) clock cycles"]
    pub const PRESCALE_9: Self = Self(0x09);
}
impl Prescale {
    pub const fn from_bits(val: u8) -> Prescale {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Prescale {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("PRESCALE_0"),
            0x01 => f.write_str("PRESCALE_1"),
            0x02 => f.write_str("PRESCALE_2"),
            0x03 => f.write_str("PRESCALE_3"),
            0x04 => f.write_str("PRESCALE_4"),
            0x05 => f.write_str("PRESCALE_5"),
            0x06 => f.write_str("PRESCALE_6"),
            0x07 => f.write_str("PRESCALE_7"),
            0x08 => f.write_str("PRESCALE_8"),
            0x09 => f.write_str("PRESCALE_9"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prescale {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "PRESCALE_0"),
            0x01 => defmt::write!(f, "PRESCALE_1"),
            0x02 => defmt::write!(f, "PRESCALE_2"),
            0x03 => defmt::write!(f, "PRESCALE_3"),
            0x04 => defmt::write!(f, "PRESCALE_4"),
            0x05 => defmt::write!(f, "PRESCALE_5"),
            0x06 => defmt::write!(f, "PRESCALE_6"),
            0x07 => defmt::write!(f, "PRESCALE_7"),
            0x08 => defmt::write!(f, "PRESCALE_8"),
            0x09 => defmt::write!(f, "PRESCALE_9"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Prescale {
    #[inline(always)]
    fn from(val: u8) -> Prescale {
        Prescale::from_bits(val)
    }
}
impl From<Prescale> for u8 {
    #[inline(always)]
    fn from(val: Prescale) -> u8 {
        Prescale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rebl {
    #[doc = "1"]
    REBL_0 = 0x0,
    #[doc = "2"]
    REBL_1 = 0x01,
    #[doc = "3"]
    REBL_2 = 0x02,
    #[doc = "4"]
    REBL_3 = 0x03,
    #[doc = "5"]
    REBL_4 = 0x04,
    #[doc = "6"]
    REBL_5 = 0x05,
    #[doc = "7"]
    REBL_6 = 0x06,
    #[doc = "8"]
    REBL_7 = 0x07,
}
impl Rebl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rebl {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rebl {
    #[inline(always)]
    fn from(val: u8) -> Rebl {
        Rebl::from_bits(val)
    }
}
impl From<Rebl> for u8 {
    #[inline(always)]
    fn from(val: Rebl) -> u8 {
        Rebl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reflock {
    #[doc = "Reference delay line is not locked."]
    REFLOCK_0 = 0x0,
    #[doc = "Reference delay line is locked."]
    REFLOCK_1 = 0x01,
}
impl Reflock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Reflock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Reflock {
    #[inline(always)]
    fn from(val: u8) -> Reflock {
        Reflock::from_bits(val)
    }
}
impl From<Reflock> for u8 {
    #[inline(always)]
    fn from(val: Reflock) -> u8 {
        Reflock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ren {
    #[doc = "The SEMC does not send AUTO REFRESH command automatically"]
    REN_0 = 0x0,
    #[doc = "The SEMC sends AUTO REFRESH command automatically"]
    REN_1 = 0x01,
}
impl Ren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ren {
    #[inline(always)]
    fn from(val: u8) -> Ren {
        Ren::from_bits(val)
    }
}
impl From<Ren> for u8 {
    #[inline(always)]
    fn from(val: Ren) -> u8 {
        Ren::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Rt(u8);
impl Rt {
    #[doc = "(256+1)*(Prescaler period)"]
    pub const RT_0: Self = Self(0x0);
    #[doc = "(RT+1)*(Prescaler period)"]
    pub const RT_1: Self = Self(0x01);
    #[doc = "(RT+1)*(Prescaler period)"]
    pub const RT_2: Self = Self(0x02);
    #[doc = "(RT+1)*(Prescaler period)"]
    pub const RT_3: Self = Self(0x03);
    #[doc = "(RT+1)*(Prescaler period)"]
    pub const RT_4: Self = Self(0x04);
    #[doc = "(RT+1)*(Prescaler period)"]
    pub const RT_5: Self = Self(0x05);
    #[doc = "(RT+1)*(Prescaler period)"]
    pub const RT_6: Self = Self(0x06);
    #[doc = "(RT+1)*(Prescaler period)"]
    pub const RT_7: Self = Self(0x07);
    #[doc = "(RT+1)*(Prescaler period)"]
    pub const RT_8: Self = Self(0x08);
    #[doc = "(RT+1)*(Prescaler period)"]
    pub const RT_9: Self = Self(0x09);
}
impl Rt {
    pub const fn from_bits(val: u8) -> Rt {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Rt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("RT_0"),
            0x01 => f.write_str("RT_1"),
            0x02 => f.write_str("RT_2"),
            0x03 => f.write_str("RT_3"),
            0x04 => f.write_str("RT_4"),
            0x05 => f.write_str("RT_5"),
            0x06 => f.write_str("RT_6"),
            0x07 => f.write_str("RT_7"),
            0x08 => f.write_str("RT_8"),
            0x09 => f.write_str("RT_9"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "RT_0"),
            0x01 => defmt::write!(f, "RT_1"),
            0x02 => defmt::write!(f, "RT_2"),
            0x03 => defmt::write!(f, "RT_3"),
            0x04 => defmt::write!(f, "RT_4"),
            0x05 => defmt::write!(f, "RT_5"),
            0x06 => defmt::write!(f, "RT_6"),
            0x07 => defmt::write!(f, "RT_7"),
            0x08 => defmt::write!(f, "RT_8"),
            0x09 => defmt::write!(f, "RT_9"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Rt {
    #[inline(always)]
    fn from(val: u8) -> Rt {
        Rt::from_bits(val)
    }
}
impl From<Rt> for u8 {
    #[inline(always)]
    fn from(val: Rt) -> u8 {
        Rt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sdramcr0Bl {
    #[doc = "1"]
    BL_0 = 0x0,
    #[doc = "2"]
    BL_1 = 0x01,
    #[doc = "4"]
    BL_2 = 0x02,
    #[doc = "8"]
    BL_3 = 0x03,
    #[doc = "8"]
    BL_4 = 0x04,
    #[doc = "8"]
    BL_5 = 0x05,
    #[doc = "8"]
    BL_6 = 0x06,
    #[doc = "8"]
    BL_7 = 0x07,
}
impl Sdramcr0Bl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sdramcr0Bl {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sdramcr0Bl {
    #[inline(always)]
    fn from(val: u8) -> Sdramcr0Bl {
        Sdramcr0Bl::from_bits(val)
    }
}
impl From<Sdramcr0Bl> for u8 {
    #[inline(always)]
    fn from(val: Sdramcr0Bl) -> u8 {
        Sdramcr0Bl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sdramcr0Col {
    #[doc = "12"]
    COL_0 = 0x0,
    #[doc = "11"]
    COL_1 = 0x01,
    #[doc = "10"]
    COL_2 = 0x02,
    #[doc = "9"]
    COL_3 = 0x03,
}
impl Sdramcr0Col {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sdramcr0Col {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sdramcr0Col {
    #[inline(always)]
    fn from(val: u8) -> Sdramcr0Col {
        Sdramcr0Col::from_bits(val)
    }
}
impl From<Sdramcr0Col> for u8 {
    #[inline(always)]
    fn from(val: Sdramcr0Col) -> u8 {
        Sdramcr0Col::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sdramcr0Ps {
    #[doc = "8bit"]
    PS_0 = 0x0,
    #[doc = "16bit"]
    PS_1 = 0x01,
}
impl Sdramcr0Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sdramcr0Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sdramcr0Ps {
    #[inline(always)]
    fn from(val: u8) -> Sdramcr0Ps {
        Sdramcr0Ps::from_bits(val)
    }
}
impl From<Sdramcr0Ps> for u8 {
    #[inline(always)]
    fn from(val: Sdramcr0Ps) -> u8 {
        Sdramcr0Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Slvlock {
    #[doc = "Slave delay line is not locked."]
    SLVLOCK_0 = 0x0,
    #[doc = "Slave delay line is locked."]
    SLVLOCK_1 = 0x01,
}
impl Slvlock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Slvlock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Slvlock {
    #[inline(always)]
    fn from(val: u8) -> Slvlock {
        Slvlock::from_bits(val)
    }
}
impl From<Slvlock> for u8 {
    #[inline(always)]
    fn from(val: Slvlock) -> u8 {
        Slvlock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sramcr0Advh {
    #[doc = "ADV# is high during address hold state."]
    ADVH_0 = 0x0,
    #[doc = "ADV# is low during address hold state."]
    ADVH_1 = 0x01,
}
impl Sramcr0Advh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sramcr0Advh {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sramcr0Advh {
    #[inline(always)]
    fn from(val: u8) -> Sramcr0Advh {
        Sramcr0Advh::from_bits(val)
    }
}
impl From<Sramcr0Advh> for u8 {
    #[inline(always)]
    fn from(val: Sramcr0Advh) -> u8 {
        Sramcr0Advh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sramcr0Advp {
    #[doc = "ADV# is active low."]
    ADVP_0 = 0x0,
    #[doc = "ADV# is active high."]
    ADVP_1 = 0x01,
}
impl Sramcr0Advp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sramcr0Advp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sramcr0Advp {
    #[inline(always)]
    fn from(val: u8) -> Sramcr0Advp {
        Sramcr0Advp::from_bits(val)
    }
}
impl From<Sramcr0Advp> for u8 {
    #[inline(always)]
    fn from(val: Sramcr0Advp) -> u8 {
        Sramcr0Advp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sramcr0Am {
    #[doc = "Address/Data MUX mode (ADMUX)"]
    AM_0 = 0x0,
    #[doc = "Advanced Address/Data MUX mode (AADM)"]
    AM_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sramcr0Am {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sramcr0Am {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sramcr0Am {
    #[inline(always)]
    fn from(val: u8) -> Sramcr0Am {
        Sramcr0Am::from_bits(val)
    }
}
impl From<Sramcr0Am> for u8 {
    #[inline(always)]
    fn from(val: Sramcr0Am) -> u8 {
        Sramcr0Am::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sramcr0Bl {
    #[doc = "1"]
    BL_0 = 0x0,
    #[doc = "2"]
    BL_1 = 0x01,
    #[doc = "4"]
    BL_2 = 0x02,
    #[doc = "8"]
    BL_3 = 0x03,
    #[doc = "16"]
    BL_4 = 0x04,
    #[doc = "32"]
    BL_5 = 0x05,
    #[doc = "64"]
    BL_6 = 0x06,
    #[doc = "64"]
    BL_7 = 0x07,
}
impl Sramcr0Bl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sramcr0Bl {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sramcr0Bl {
    #[inline(always)]
    fn from(val: u8) -> Sramcr0Bl {
        Sramcr0Bl::from_bits(val)
    }
}
impl From<Sramcr0Bl> for u8 {
    #[inline(always)]
    fn from(val: Sramcr0Bl) -> u8 {
        Sramcr0Bl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sramcr0Col {
    #[doc = "12 Bits"]
    COL_0 = 0x0,
    #[doc = "11 Bits"]
    COL_1 = 0x01,
    #[doc = "10 Bits"]
    COL_2 = 0x02,
    #[doc = "9 Bits"]
    COL_3 = 0x03,
    #[doc = "8 Bits"]
    COL_4 = 0x04,
    #[doc = "7 Bits"]
    COL_5 = 0x05,
    #[doc = "6 Bits"]
    COL_6 = 0x06,
    #[doc = "5 Bits"]
    COL_7 = 0x07,
    #[doc = "4 Bits"]
    COL_8 = 0x08,
    #[doc = "3 Bits"]
    COL_9 = 0x09,
    #[doc = "2 Bits"]
    COL_10 = 0x0a,
    #[doc = "12 Bits"]
    COL_11 = 0x0b,
    #[doc = "12 Bits"]
    COL_12 = 0x0c,
    #[doc = "12 Bits"]
    COL_13 = 0x0d,
    #[doc = "12 Bits"]
    COL_14 = 0x0e,
    #[doc = "12 Bits"]
    COL_15 = 0x0f,
}
impl Sramcr0Col {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sramcr0Col {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sramcr0Col {
    #[inline(always)]
    fn from(val: u8) -> Sramcr0Col {
        Sramcr0Col::from_bits(val)
    }
}
impl From<Sramcr0Col> for u8 {
    #[inline(always)]
    fn from(val: Sramcr0Col) -> u8 {
        Sramcr0Col::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sramcr0Ps {
    #[doc = "8bit"]
    PS_0 = 0x0,
    #[doc = "16bit"]
    PS_1 = 0x01,
}
impl Sramcr0Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sramcr0Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sramcr0Ps {
    #[inline(always)]
    fn from(val: u8) -> Sramcr0Ps {
        Sramcr0Ps::from_bits(val)
    }
}
impl From<Sramcr0Ps> for u8 {
    #[inline(always)]
    fn from(val: Sramcr0Ps) -> u8 {
        Sramcr0Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sramcr0Syncen {
    #[doc = "Asynchronous mode is enabled."]
    SYNCEN_0 = 0x0,
    #[doc = "Synchronous mode is enabled. Only fixed latency mode is supported."]
    SYNCEN_1 = 0x01,
}
impl Sramcr0Syncen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sramcr0Syncen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sramcr0Syncen {
    #[inline(always)]
    fn from(val: u8) -> Sramcr0Syncen {
        Sramcr0Syncen::from_bits(val)
    }
}
impl From<Sramcr0Syncen> for u8 {
    #[inline(always)]
    fn from(val: Sramcr0Syncen) -> u8 {
        Sramcr0Syncen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swrst {
    #[doc = "No reset"]
    SWRST_0 = 0x0,
    #[doc = "Reset"]
    SWRST_1 = 0x01,
}
impl Swrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swrst {
    #[inline(always)]
    fn from(val: u8) -> Swrst {
        Swrst::from_bits(val)
    }
}
impl From<Swrst> for u8 {
    #[inline(always)]
    fn from(val: Swrst) -> u8 {
        Swrst::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ut(u8);
impl Ut {
    #[doc = "256*(Prescaler period)"]
    pub const UT_0: Self = Self(0x0);
    #[doc = "UT*(Prescaler period)"]
    pub const UT_1: Self = Self(0x01);
    #[doc = "UT*(Prescaler period)"]
    pub const UT_2: Self = Self(0x02);
    #[doc = "UT*(Prescaler period)"]
    pub const UT_3: Self = Self(0x03);
    #[doc = "UT*(Prescaler period)"]
    pub const UT_4: Self = Self(0x04);
    #[doc = "UT*(Prescaler period)"]
    pub const UT_5: Self = Self(0x05);
    #[doc = "UT*(Prescaler period)"]
    pub const UT_6: Self = Self(0x06);
    #[doc = "UT*(Prescaler period)"]
    pub const UT_7: Self = Self(0x07);
    #[doc = "UT*(Prescaler period)"]
    pub const UT_8: Self = Self(0x08);
    #[doc = "UT*(Prescaler period)"]
    pub const UT_9: Self = Self(0x09);
}
impl Ut {
    pub const fn from_bits(val: u8) -> Ut {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Ut {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("UT_0"),
            0x01 => f.write_str("UT_1"),
            0x02 => f.write_str("UT_2"),
            0x03 => f.write_str("UT_3"),
            0x04 => f.write_str("UT_4"),
            0x05 => f.write_str("UT_5"),
            0x06 => f.write_str("UT_6"),
            0x07 => f.write_str("UT_7"),
            0x08 => f.write_str("UT_8"),
            0x09 => f.write_str("UT_9"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ut {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "UT_0"),
            0x01 => defmt::write!(f, "UT_1"),
            0x02 => defmt::write!(f, "UT_2"),
            0x03 => defmt::write!(f, "UT_3"),
            0x04 => defmt::write!(f, "UT_4"),
            0x05 => defmt::write!(f, "UT_5"),
            0x06 => defmt::write!(f, "UT_6"),
            0x07 => defmt::write!(f, "UT_7"),
            0x08 => defmt::write!(f, "UT_8"),
            0x09 => defmt::write!(f, "UT_9"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Ut {
    #[inline(always)]
    fn from(val: u8) -> Ut {
        Ut::from_bits(val)
    }
}
impl From<Ut> for u8 {
    #[inline(always)]
    fn from(val: Ut) -> u8 {
        Ut::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vld {
    #[doc = "The memory is invalid, can not be accessed."]
    VLD_0 = 0x0,
    #[doc = "The memory is valid, can be accessed."]
    VLD_1 = 0x01,
}
impl Vld {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vld {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vld {
    #[inline(always)]
    fn from(val: u8) -> Vld {
        Vld::from_bits(val)
    }
}
impl From<Vld> for u8 {
    #[inline(always)]
    fn from(val: Vld) -> u8 {
        Vld::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wpol0 {
    #[doc = "WAIT/RDY polarity is not changed."]
    WPOL0_0 = 0x0,
    #[doc = "WAIT/RDY polarity is inverted."]
    WPOL0_1 = 0x01,
}
impl Wpol0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wpol0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wpol0 {
    #[inline(always)]
    fn from(val: u8) -> Wpol0 {
        Wpol0::from_bits(val)
    }
}
impl From<Wpol0> for u8 {
    #[inline(always)]
    fn from(val: Wpol0) -> u8 {
        Wpol0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wpol1 {
    #[doc = "R/B# polarity is not changed."]
    WPOL1_0 = 0x0,
    #[doc = "R/B# polarity is inverted."]
    WPOL1_1 = 0x01,
}
impl Wpol1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wpol1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wpol1 {
    #[inline(always)]
    fn from(val: u8) -> Wpol1 {
        Wpol1::from_bits(val)
    }
}
impl From<Wpol1> for u8 {
    #[inline(always)]
    fn from(val: Wpol1) -> u8 {
        Wpol1::to_bits(val)
    }
}
