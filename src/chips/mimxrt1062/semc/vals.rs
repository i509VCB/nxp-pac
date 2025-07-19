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
