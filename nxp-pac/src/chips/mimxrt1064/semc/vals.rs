#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Axibuserr {
    #[doc = "No AXI bus error."]
    Axibuserr0 = 0x0,
    #[doc = "AXI bus error occurs."]
    Axibuserr1 = 0x01,
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
    #[doc = "Interrupt is disabled."]
    Axibuserren0 = 0x0,
    #[doc = "Interrupt is enabled."]
    Axibuserren1 = 0x01,
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
    Axicmderr0 = 0x0,
    #[doc = "AXI command error occurs."]
    Axicmderr1 = 0x01,
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
    #[doc = "Interrupt is disabled."]
    Axicmderren0 = 0x0,
    #[doc = "Interrupt is enabled."]
    Axicmderren1 = 0x01,
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
    Bank20 = 0x0,
    #[doc = "SDRAM device has 2 banks."]
    Bank21 = 0x01,
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
    #[doc = "Byte is unmasked."]
    Bm00 = 0x0,
    #[doc = "Byte is masked."]
    Bm01 = 0x01,
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
    #[doc = "Byte is unmasked."]
    Bm10 = 0x0,
    #[doc = "Byte is masked."]
    Bm11 = 0x01,
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
    #[doc = "Byte is unmasked."]
    Bm20 = 0x0,
    #[doc = "Byte is masked."]
    Bm21 = 0x01,
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
    #[doc = "Byte is unmasked."]
    Bm30 = 0x0,
    #[doc = "Byte is masked."]
    Bm31 = 0x01,
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
    #[doc = "255*1."]
    Bto0 = 0x0,
    #[doc = "255*2."]
    Bto1 = 0x01,
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
    #[doc = "255*231."]
    Bto31 = 0x1f,
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
    #[doc = "1."]
    Cl0 = 0x0,
    #[doc = "1."]
    Cl1 = 0x01,
    #[doc = "2."]
    Cl2 = 0x02,
    #[doc = "3."]
    Cl3 = 0x03,
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
    Col80 = 0x0,
    #[doc = "Column address bit number is 8. COL field is ignored."]
    Col81 = 0x01,
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
    #[doc = "4."]
    Datsz0 = 0x0,
    #[doc = "1."]
    Datsz1 = 0x01,
    #[doc = "2."]
    Datsz2 = 0x02,
    #[doc = "3."]
    Datsz3 = 0x03,
    #[doc = "4."]
    Datsz4 = 0x04,
    #[doc = "4."]
    Datsz5 = 0x05,
    #[doc = "4."]
    Datsz6 = 0x06,
    #[doc = "4."]
    Datsz7 = 0x07,
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
    #[doc = "1."]
    Bl0 = 0x0,
    #[doc = "2."]
    Bl1 = 0x01,
    #[doc = "4."]
    Bl2 = 0x02,
    #[doc = "8."]
    Bl3 = 0x03,
    #[doc = "16."]
    Bl4 = 0x04,
    #[doc = "32."]
    Bl5 = 0x05,
    #[doc = "64."]
    Bl6 = 0x06,
    #[doc = "64."]
    Bl7 = 0x07,
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
    #[doc = "12 Bits."]
    Col0 = 0x0,
    #[doc = "11 Bits."]
    Col1 = 0x01,
    #[doc = "10 Bits."]
    Col2 = 0x02,
    #[doc = "9 Bits."]
    Col3 = 0x03,
    #[doc = "8 Bits."]
    Col4 = 0x04,
    #[doc = "7 Bits."]
    Col5 = 0x05,
    #[doc = "6 Bits."]
    Col6 = 0x06,
    #[doc = "5 Bits."]
    Col7 = 0x07,
    #[doc = "4 Bits."]
    Col8 = 0x08,
    #[doc = "3 Bits."]
    Col9 = 0x09,
    #[doc = "2 Bits."]
    Col10 = 0x0a,
    #[doc = "12 Bits."]
    Col11 = 0x0b,
    #[doc = "12 Bits."]
    Col12 = 0x0c,
    #[doc = "12 Bits."]
    Col13 = 0x0d,
    #[doc = "12 Bits."]
    Col14 = 0x0e,
    #[doc = "12 Bits."]
    Col15 = 0x0f,
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
    #[doc = "8bit."]
    Ps0 = 0x0,
    #[doc = "16bit."]
    Ps1 = 0x01,
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
    Dllen0 = 0x0,
    #[doc = "DLL calibration is enabled."]
    Dllen1 = 0x01,
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
    Dllreset0 = 0x0,
    #[doc = "DLL is reset."]
    Dllreset1 = 0x01,
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
    #[doc = "Dummy read strobe loopbacked internally."]
    Dqsmd0 = 0x0,
    #[doc = "Dummy read strobe loopbacked from DQS pad."]
    Dqsmd1 = 0x01,
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
    #[doc = "EDO mode disabled."]
    Edo0 = 0x0,
    #[doc = "EDO mode enabled."]
    Edo1 = 0x01,
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
    Ipcmddone0 = 0x0,
    #[doc = "IP command is done."]
    Ipcmddone1 = 0x01,
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
    #[doc = "Interrupt is disabled."]
    Ipcmddoneen0 = 0x0,
    #[doc = "Interrupt is enabled."]
    Ipcmddoneen1 = 0x01,
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
    Ipcmderr0 = 0x0,
    #[doc = "IP command error occurs."]
    Ipcmderr1 = 0x01,
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
    #[doc = "Interrupt is disabled."]
    Ipcmderren0 = 0x0,
    #[doc = "Interrupt is enabled."]
    Ipcmderren1 = 0x01,
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
    pub const Ito0: Self = Self(0x0);
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    pub const Ito1: Self = Self(0x01);
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    pub const Ito2: Self = Self(0x02);
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    pub const Ito3: Self = Self(0x03);
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    pub const Ito4: Self = Self(0x04);
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    pub const Ito5: Self = Self(0x05);
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    pub const Ito6: Self = Self(0x06);
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    pub const Ito7: Self = Self(0x07);
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    pub const Ito8: Self = Self(0x08);
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    pub const Ito9: Self = Self(0x09);
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
            0x0 => f.write_str("Ito0"),
            0x01 => f.write_str("Ito1"),
            0x02 => f.write_str("Ito2"),
            0x03 => f.write_str("Ito3"),
            0x04 => f.write_str("Ito4"),
            0x05 => f.write_str("Ito5"),
            0x06 => f.write_str("Ito6"),
            0x07 => f.write_str("Ito7"),
            0x08 => f.write_str("Ito8"),
            0x09 => f.write_str("Ito9"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ito {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Ito0"),
            0x01 => defmt::write!(f, "Ito1"),
            0x02 => defmt::write!(f, "Ito2"),
            0x03 => defmt::write!(f, "Ito3"),
            0x04 => defmt::write!(f, "Ito4"),
            0x05 => defmt::write!(f, "Ito5"),
            0x06 => defmt::write!(f, "Ito6"),
            0x07 => defmt::write!(f, "Ito7"),
            0x08 => defmt::write!(f, "Ito8"),
            0x09 => defmt::write!(f, "Ito9"),
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
    #[doc = "Module enabled."]
    Mdis0 = 0x0,
    #[doc = "Module disabled."]
    Mdis1 = 0x01,
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
    #[doc = "4KB."]
    Ms0 = 0x0,
    #[doc = "8KB."]
    Ms1 = 0x01,
    #[doc = "16KB."]
    Ms2 = 0x02,
    #[doc = "32KB."]
    Ms3 = 0x03,
    #[doc = "64KB."]
    Ms4 = 0x04,
    #[doc = "128KB."]
    Ms5 = 0x05,
    #[doc = "256KB."]
    Ms6 = 0x06,
    #[doc = "512KB."]
    Ms7 = 0x07,
    #[doc = "1MB."]
    Ms8 = 0x08,
    #[doc = "2MB."]
    Ms9 = 0x09,
    #[doc = "4MB."]
    Ms10 = 0x0a,
    #[doc = "8MB."]
    Ms11 = 0x0b,
    #[doc = "16MB."]
    Ms12 = 0x0c,
    #[doc = "32MB."]
    Ms13 = 0x0d,
    #[doc = "64MB."]
    Ms14 = 0x0e,
    #[doc = "128MB."]
    Ms15 = 0x0f,
    #[doc = "256MB."]
    Ms16 = 0x10,
    #[doc = "512MB."]
    Ms17 = 0x11,
    #[doc = "1GB."]
    Ms18 = 0x12,
    #[doc = "2GB."]
    Ms19 = 0x13,
    #[doc = "4GB."]
    Ms20 = 0x14,
    #[doc = "4GB."]
    Ms21 = 0x15,
    #[doc = "4GB."]
    Ms22 = 0x16,
    #[doc = "4GB."]
    Ms23 = 0x17,
    #[doc = "4GB."]
    Ms24 = 0x18,
    #[doc = "4GB."]
    Ms25 = 0x19,
    #[doc = "4GB."]
    Ms26 = 0x1a,
    #[doc = "4GB."]
    Ms27 = 0x1b,
    #[doc = "4GB."]
    Ms28 = 0x1c,
    #[doc = "4GB."]
    Ms29 = 0x1d,
    #[doc = "4GB."]
    Ms30 = 0x1e,
    #[doc = "4GB."]
    Ms31 = 0x1f,
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
    #[doc = "SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode."]
    MuxA80 = 0x0,
    #[doc = "NAND CE#."]
    MuxA81 = 0x01,
    #[doc = "NOR CE#."]
    MuxA82 = 0x02,
    #[doc = "SRAM CE#."]
    MuxA83 = 0x03,
    #[doc = "DBI CSX."]
    MuxA84 = 0x04,
    #[doc = "SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode."]
    MuxA85 = 0x05,
    #[doc = "SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode."]
    MuxA86 = 0x06,
    #[doc = "SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode."]
    MuxA87 = 0x07,
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
    #[doc = "NOR clock."]
    MuxClkx00 = 0x0,
    #[doc = "SRAM clock."]
    MuxClkx01 = 0x01,
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
    #[doc = "NOR clock."]
    MuxClkx10 = 0x0,
    #[doc = "SRAM clock."]
    MuxClkx11 = 0x01,
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
    #[doc = "SDRAM CS1."]
    MuxCsx01 = 0x01,
    #[doc = "SDRAM CS2."]
    MuxCsx02 = 0x02,
    #[doc = "SDRAM CS3."]
    MuxCsx03 = 0x03,
    #[doc = "NAND CE#."]
    MuxCsx04 = 0x04,
    #[doc = "NOR CE#."]
    MuxCsx05 = 0x05,
    #[doc = "SRAM CE#."]
    MuxCsx06 = 0x06,
    #[doc = "DBI CSX."]
    MuxCsx07 = 0x07,
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
    #[doc = "SDRAM CS1."]
    MuxCsx11 = 0x01,
    #[doc = "SDRAM CS2."]
    MuxCsx12 = 0x02,
    #[doc = "SDRAM CS3."]
    MuxCsx13 = 0x03,
    #[doc = "NAND CE#."]
    MuxCsx14 = 0x04,
    #[doc = "NOR CE#."]
    MuxCsx15 = 0x05,
    #[doc = "SRAM CE#."]
    MuxCsx16 = 0x06,
    #[doc = "DBI CSX."]
    MuxCsx17 = 0x07,
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
    #[doc = "SDRAM CS1."]
    MuxCsx21 = 0x01,
    #[doc = "SDRAM CS2."]
    MuxCsx22 = 0x02,
    #[doc = "SDRAM CS3."]
    MuxCsx23 = 0x03,
    #[doc = "NAND CE#."]
    MuxCsx24 = 0x04,
    #[doc = "NOR CE#."]
    MuxCsx25 = 0x05,
    #[doc = "SRAM CE#."]
    MuxCsx26 = 0x06,
    #[doc = "DBI CSX."]
    MuxCsx27 = 0x07,
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
    #[doc = "SDRAM CS1."]
    MuxCsx31 = 0x01,
    #[doc = "SDRAM CS2."]
    MuxCsx32 = 0x02,
    #[doc = "SDRAM CS3."]
    MuxCsx33 = 0x03,
    #[doc = "NAND CE#."]
    MuxCsx34 = 0x04,
    #[doc = "NOR CE#."]
    MuxCsx35 = 0x05,
    #[doc = "SRAM CE#."]
    MuxCsx36 = 0x06,
    #[doc = "DBI CSX."]
    MuxCsx37 = 0x07,
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
    #[doc = "NAND R/B# input."]
    MuxRdy0 = 0x0,
    #[doc = "SDRAM CS1."]
    MuxRdy1 = 0x01,
    #[doc = "SDRAM CS2."]
    MuxRdy2 = 0x02,
    #[doc = "SDRAM CS3."]
    MuxRdy3 = 0x03,
    #[doc = "NOR CE#."]
    MuxRdy4 = 0x04,
    #[doc = "SRAM CE#."]
    MuxRdy5 = 0x05,
    #[doc = "DBI CSX."]
    MuxRdy6 = 0x06,
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
    #[doc = "1."]
    Bl0 = 0x0,
    #[doc = "2."]
    Bl1 = 0x01,
    #[doc = "4."]
    Bl2 = 0x02,
    #[doc = "8."]
    Bl3 = 0x03,
    #[doc = "16."]
    Bl4 = 0x04,
    #[doc = "32."]
    Bl5 = 0x05,
    #[doc = "64."]
    Bl6 = 0x06,
    #[doc = "64."]
    Bl7 = 0x07,
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
    #[doc = "16."]
    Col0 = 0x0,
    #[doc = "15."]
    Col1 = 0x01,
    #[doc = "14."]
    Col2 = 0x02,
    #[doc = "13."]
    Col3 = 0x03,
    #[doc = "12."]
    Col4 = 0x04,
    #[doc = "11."]
    Col5 = 0x05,
    #[doc = "10."]
    Col6 = 0x06,
    #[doc = "9."]
    Col7 = 0x07,
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
    #[doc = "8bit."]
    Ps0 = 0x0,
    #[doc = "16bit."]
    Ps1 = 0x01,
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
    Syncen0 = 0x0,
    #[doc = "Synchronous mode is enabled."]
    Syncen1 = 0x01,
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
    #[doc = "NAND device is not ready."]
    Nardy0 = 0x0,
    #[doc = "NAND device is ready."]
    Nardy1 = 0x01,
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
    Ndnopend0 = 0x0,
    #[doc = "All NAND AXI write pending transactions are finished."]
    Ndnopend1 = 0x01,
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
    #[doc = "Interrupt is disabled."]
    Ndnopenden0 = 0x0,
    #[doc = "Interrupt is enabled."]
    Ndnopenden1 = 0x01,
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
    Ndpageend0 = 0x0,
    #[doc = "The last address of main space in the NAND is written by AXI command."]
    Ndpageend1 = 0x01,
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
    #[doc = "Interrupt is disabled."]
    Ndpageenden0 = 0x0,
    #[doc = "Interrupt is enabled."]
    Ndpageenden1 = 0x01,
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
    #[doc = "No pending."]
    Ndwrpend0 = 0x0,
    #[doc = "Pending."]
    Ndwrpend1 = 0x01,
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
    Advh0 = 0x0,
    #[doc = "ADV# is low during address hold state."]
    Advh1 = 0x01,
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
    Advp0 = 0x0,
    #[doc = "ADV# is active high."]
    Advp1 = 0x01,
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
    #[doc = "Address/Data MUX mode (ADMUX)."]
    Am0 = 0x0,
    #[doc = "Advanced Address/Data MUX mode (AADM)."]
    Am1 = 0x01,
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
    #[doc = "1."]
    Bl0 = 0x0,
    #[doc = "2."]
    Bl1 = 0x01,
    #[doc = "4."]
    Bl2 = 0x02,
    #[doc = "8."]
    Bl3 = 0x03,
    #[doc = "16."]
    Bl4 = 0x04,
    #[doc = "32."]
    Bl5 = 0x05,
    #[doc = "64."]
    Bl6 = 0x06,
    #[doc = "64."]
    Bl7 = 0x07,
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
    #[doc = "12 Bits."]
    Col0 = 0x0,
    #[doc = "11 Bits."]
    Col1 = 0x01,
    #[doc = "10 Bits."]
    Col2 = 0x02,
    #[doc = "9 Bits."]
    Col3 = 0x03,
    #[doc = "8 Bits."]
    Col4 = 0x04,
    #[doc = "7 Bits."]
    Col5 = 0x05,
    #[doc = "6 Bits."]
    Col6 = 0x06,
    #[doc = "5 Bits."]
    Col7 = 0x07,
    #[doc = "4 Bits."]
    Col8 = 0x08,
    #[doc = "3 Bits."]
    Col9 = 0x09,
    #[doc = "2 Bits."]
    Col10 = 0x0a,
    #[doc = "12 Bits."]
    Col11 = 0x0b,
    #[doc = "12 Bits."]
    Col12 = 0x0c,
    #[doc = "12 Bits."]
    Col13 = 0x0d,
    #[doc = "12 Bits."]
    Col14 = 0x0e,
    #[doc = "12 Bits."]
    Col15 = 0x0f,
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
    #[doc = "8bit."]
    Ps0 = 0x0,
    #[doc = "16bit."]
    Ps1 = 0x01,
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
    Syncen0 = 0x0,
    #[doc = "Synchronous mode is enabled. Only fixed latency mode is supported."]
    Syncen1 = 0x01,
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
    Ovrden0 = 0x0,
    #[doc = "The delay cell number is overridden."]
    Ovrden1 = 0x01,
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
    #[doc = "(256*16+1) clock cycles."]
    pub const Prescale0: Self = Self(0x0);
    #[doc = "(PRESCALE*16+1) clock cycles."]
    pub const Prescale1: Self = Self(0x01);
    #[doc = "(PRESCALE*16+1) clock cycles."]
    pub const Prescale2: Self = Self(0x02);
    #[doc = "(PRESCALE*16+1) clock cycles."]
    pub const Prescale3: Self = Self(0x03);
    #[doc = "(PRESCALE*16+1) clock cycles."]
    pub const Prescale4: Self = Self(0x04);
    #[doc = "(PRESCALE*16+1) clock cycles."]
    pub const Prescale5: Self = Self(0x05);
    #[doc = "(PRESCALE*16+1) clock cycles."]
    pub const Prescale6: Self = Self(0x06);
    #[doc = "(PRESCALE*16+1) clock cycles."]
    pub const Prescale7: Self = Self(0x07);
    #[doc = "(PRESCALE*16+1) clock cycles."]
    pub const Prescale8: Self = Self(0x08);
    #[doc = "(PRESCALE*16+1) clock cycles."]
    pub const Prescale9: Self = Self(0x09);
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
            0x0 => f.write_str("Prescale0"),
            0x01 => f.write_str("Prescale1"),
            0x02 => f.write_str("Prescale2"),
            0x03 => f.write_str("Prescale3"),
            0x04 => f.write_str("Prescale4"),
            0x05 => f.write_str("Prescale5"),
            0x06 => f.write_str("Prescale6"),
            0x07 => f.write_str("Prescale7"),
            0x08 => f.write_str("Prescale8"),
            0x09 => f.write_str("Prescale9"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prescale {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Prescale0"),
            0x01 => defmt::write!(f, "Prescale1"),
            0x02 => defmt::write!(f, "Prescale2"),
            0x03 => defmt::write!(f, "Prescale3"),
            0x04 => defmt::write!(f, "Prescale4"),
            0x05 => defmt::write!(f, "Prescale5"),
            0x06 => defmt::write!(f, "Prescale6"),
            0x07 => defmt::write!(f, "Prescale7"),
            0x08 => defmt::write!(f, "Prescale8"),
            0x09 => defmt::write!(f, "Prescale9"),
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
    #[doc = "1."]
    Rebl0 = 0x0,
    #[doc = "2."]
    Rebl1 = 0x01,
    #[doc = "3."]
    Rebl2 = 0x02,
    #[doc = "4."]
    Rebl3 = 0x03,
    #[doc = "5."]
    Rebl4 = 0x04,
    #[doc = "6."]
    Rebl5 = 0x05,
    #[doc = "7."]
    Rebl6 = 0x06,
    #[doc = "8."]
    Rebl7 = 0x07,
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
    Reflock0 = 0x0,
    #[doc = "Reference delay line is locked."]
    Reflock1 = 0x01,
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
    #[doc = "The SEMC does not send AUTO REFRESH command automatically."]
    Ren0 = 0x0,
    #[doc = "The SEMC sends AUTO REFRESH command automatically."]
    Ren1 = 0x01,
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
    #[doc = "(256+1)*(Prescaler period)."]
    pub const Rt0: Self = Self(0x0);
    #[doc = "(RT+1)*(Prescaler period)."]
    pub const Rt1: Self = Self(0x01);
    #[doc = "(RT+1)*(Prescaler period)."]
    pub const Rt2: Self = Self(0x02);
    #[doc = "(RT+1)*(Prescaler period)."]
    pub const Rt3: Self = Self(0x03);
    #[doc = "(RT+1)*(Prescaler period)."]
    pub const Rt4: Self = Self(0x04);
    #[doc = "(RT+1)*(Prescaler period)."]
    pub const Rt5: Self = Self(0x05);
    #[doc = "(RT+1)*(Prescaler period)."]
    pub const Rt6: Self = Self(0x06);
    #[doc = "(RT+1)*(Prescaler period)."]
    pub const Rt7: Self = Self(0x07);
    #[doc = "(RT+1)*(Prescaler period)."]
    pub const Rt8: Self = Self(0x08);
    #[doc = "(RT+1)*(Prescaler period)."]
    pub const Rt9: Self = Self(0x09);
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
            0x0 => f.write_str("Rt0"),
            0x01 => f.write_str("Rt1"),
            0x02 => f.write_str("Rt2"),
            0x03 => f.write_str("Rt3"),
            0x04 => f.write_str("Rt4"),
            0x05 => f.write_str("Rt5"),
            0x06 => f.write_str("Rt6"),
            0x07 => f.write_str("Rt7"),
            0x08 => f.write_str("Rt8"),
            0x09 => f.write_str("Rt9"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Rt0"),
            0x01 => defmt::write!(f, "Rt1"),
            0x02 => defmt::write!(f, "Rt2"),
            0x03 => defmt::write!(f, "Rt3"),
            0x04 => defmt::write!(f, "Rt4"),
            0x05 => defmt::write!(f, "Rt5"),
            0x06 => defmt::write!(f, "Rt6"),
            0x07 => defmt::write!(f, "Rt7"),
            0x08 => defmt::write!(f, "Rt8"),
            0x09 => defmt::write!(f, "Rt9"),
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
    #[doc = "1."]
    Bl0 = 0x0,
    #[doc = "2."]
    Bl1 = 0x01,
    #[doc = "4."]
    Bl2 = 0x02,
    #[doc = "8."]
    Bl3 = 0x03,
    #[doc = "8."]
    Bl4 = 0x04,
    #[doc = "8."]
    Bl5 = 0x05,
    #[doc = "8."]
    Bl6 = 0x06,
    #[doc = "8."]
    Bl7 = 0x07,
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
    #[doc = "12."]
    Col0 = 0x0,
    #[doc = "11."]
    Col1 = 0x01,
    #[doc = "10."]
    Col2 = 0x02,
    #[doc = "9."]
    Col3 = 0x03,
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
    #[doc = "8bit."]
    Ps0 = 0x0,
    #[doc = "16bit."]
    Ps1 = 0x01,
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
    Slvlock0 = 0x0,
    #[doc = "Slave delay line is locked."]
    Slvlock1 = 0x01,
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
    Advh0 = 0x0,
    #[doc = "ADV# is low during address hold state."]
    Advh1 = 0x01,
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
    Advp0 = 0x0,
    #[doc = "ADV# is active high."]
    Advp1 = 0x01,
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
    #[doc = "Address/Data MUX mode (ADMUX)."]
    Am0 = 0x0,
    #[doc = "Advanced Address/Data MUX mode (AADM)."]
    Am1 = 0x01,
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
    #[doc = "1."]
    Bl0 = 0x0,
    #[doc = "2."]
    Bl1 = 0x01,
    #[doc = "4."]
    Bl2 = 0x02,
    #[doc = "8."]
    Bl3 = 0x03,
    #[doc = "16."]
    Bl4 = 0x04,
    #[doc = "32."]
    Bl5 = 0x05,
    #[doc = "64."]
    Bl6 = 0x06,
    #[doc = "64."]
    Bl7 = 0x07,
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
    #[doc = "12 Bits."]
    Col0 = 0x0,
    #[doc = "11 Bits."]
    Col1 = 0x01,
    #[doc = "10 Bits."]
    Col2 = 0x02,
    #[doc = "9 Bits."]
    Col3 = 0x03,
    #[doc = "8 Bits."]
    Col4 = 0x04,
    #[doc = "7 Bits."]
    Col5 = 0x05,
    #[doc = "6 Bits."]
    Col6 = 0x06,
    #[doc = "5 Bits."]
    Col7 = 0x07,
    #[doc = "4 Bits."]
    Col8 = 0x08,
    #[doc = "3 Bits."]
    Col9 = 0x09,
    #[doc = "2 Bits."]
    Col10 = 0x0a,
    #[doc = "12 Bits."]
    Col11 = 0x0b,
    #[doc = "12 Bits."]
    Col12 = 0x0c,
    #[doc = "12 Bits."]
    Col13 = 0x0d,
    #[doc = "12 Bits."]
    Col14 = 0x0e,
    #[doc = "12 Bits."]
    Col15 = 0x0f,
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
    #[doc = "8bit."]
    Ps0 = 0x0,
    #[doc = "16bit."]
    Ps1 = 0x01,
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
    Syncen0 = 0x0,
    #[doc = "Synchronous mode is enabled. Only fixed latency mode is supported."]
    Syncen1 = 0x01,
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
    #[doc = "No reset."]
    Swrst0 = 0x0,
    #[doc = "Reset."]
    Swrst1 = 0x01,
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
    #[doc = "256*(Prescaler period)."]
    pub const Ut0: Self = Self(0x0);
    #[doc = "UT*(Prescaler period)."]
    pub const Ut1: Self = Self(0x01);
    #[doc = "UT*(Prescaler period)."]
    pub const Ut2: Self = Self(0x02);
    #[doc = "UT*(Prescaler period)."]
    pub const Ut3: Self = Self(0x03);
    #[doc = "UT*(Prescaler period)."]
    pub const Ut4: Self = Self(0x04);
    #[doc = "UT*(Prescaler period)."]
    pub const Ut5: Self = Self(0x05);
    #[doc = "UT*(Prescaler period)."]
    pub const Ut6: Self = Self(0x06);
    #[doc = "UT*(Prescaler period)."]
    pub const Ut7: Self = Self(0x07);
    #[doc = "UT*(Prescaler period)."]
    pub const Ut8: Self = Self(0x08);
    #[doc = "UT*(Prescaler period)."]
    pub const Ut9: Self = Self(0x09);
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
            0x0 => f.write_str("Ut0"),
            0x01 => f.write_str("Ut1"),
            0x02 => f.write_str("Ut2"),
            0x03 => f.write_str("Ut3"),
            0x04 => f.write_str("Ut4"),
            0x05 => f.write_str("Ut5"),
            0x06 => f.write_str("Ut6"),
            0x07 => f.write_str("Ut7"),
            0x08 => f.write_str("Ut8"),
            0x09 => f.write_str("Ut9"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ut {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Ut0"),
            0x01 => defmt::write!(f, "Ut1"),
            0x02 => defmt::write!(f, "Ut2"),
            0x03 => defmt::write!(f, "Ut3"),
            0x04 => defmt::write!(f, "Ut4"),
            0x05 => defmt::write!(f, "Ut5"),
            0x06 => defmt::write!(f, "Ut6"),
            0x07 => defmt::write!(f, "Ut7"),
            0x08 => defmt::write!(f, "Ut8"),
            0x09 => defmt::write!(f, "Ut9"),
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
    Vld0 = 0x0,
    #[doc = "The memory is valid, can be accessed."]
    Vld1 = 0x01,
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
    Wpol00 = 0x0,
    #[doc = "WAIT/RDY polarity is inverted."]
    Wpol01 = 0x01,
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
    Wpol10 = 0x0,
    #[doc = "R/B# polarity is inverted."]
    Wpol11 = 0x01,
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
