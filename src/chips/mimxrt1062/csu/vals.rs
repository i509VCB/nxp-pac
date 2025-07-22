#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hp0LEnet2 {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_ENET2_0 = 0x0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_ENET2_1 = 0x01,
}
impl Hp0LEnet2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hp0LEnet2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hp0LEnet2 {
    #[inline(always)]
    fn from(val: u8) -> Hp0LEnet2 {
        Hp0LEnet2::from_bits(val)
    }
}
impl From<Hp0LEnet2> for u8 {
    #[inline(always)]
    fn from(val: Hp0LEnet2) -> u8 {
        Hp0LEnet2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HpEnet2 {
    #[doc = "The hprot1 input signal value is routed to the csu_hprot1 output for the corresponding master."]
    HP_ENET2_0 = 0x0,
    #[doc = "The HP register bit is routed to the csu_hprot1 output for the corresponding master."]
    HP_ENET2_1 = 0x01,
}
impl HpEnet2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HpEnet2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HpEnet2 {
    #[inline(always)]
    fn from(val: u8) -> HpEnet2 {
        HpEnet2::from_bits(val)
    }
}
impl From<HpEnet2> for u8 {
    #[inline(always)]
    fn from(val: HpEnet2) -> u8 {
        HpEnet2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HpcEnet2 {
    #[doc = "User mode for the corresponding master"]
    HPC_ENET2_0 = 0x0,
    #[doc = "Supervisor mode for the corresponding master"]
    HPC_ENET2_1 = 0x01,
}
impl HpcEnet2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HpcEnet2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HpcEnet2 {
    #[inline(always)]
    fn from(val: u8) -> HpcEnet2 {
        HpcEnet2::from_bits(val)
    }
}
impl From<HpcEnet2> for u8 {
    #[inline(always)]
    fn from(val: HpcEnet2) -> u8 {
        HpcEnet2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hpcontrol0LEnet2 {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_ENET2_0 = 0x0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_ENET2_1 = 0x01,
}
impl Hpcontrol0LEnet2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hpcontrol0LEnet2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hpcontrol0LEnet2 {
    #[inline(always)]
    fn from(val: u8) -> Hpcontrol0LEnet2 {
        Hpcontrol0LEnet2::from_bits(val)
    }
}
impl From<Hpcontrol0LEnet2> for u8 {
    #[inline(always)]
    fn from(val: Hpcontrol0LEnet2) -> u8 {
        Hpcontrol0LEnet2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NsaEnet2 {
    #[doc = "Secure access for the corresponding type-1 master"]
    NSA_ENET2_0 = 0x0,
    #[doc = "Non-secure access for the corresponding type-1 master"]
    NSA_ENET2_1 = 0x01,
}
impl NsaEnet2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NsaEnet2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NsaEnet2 {
    #[inline(always)]
    fn from(val: u8) -> NsaEnet2 {
        NsaEnet2::from_bits(val)
    }
}
impl From<NsaEnet2> for u8 {
    #[inline(always)]
    fn from(val: NsaEnet2) -> u8 {
        NsaEnet2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SaLEnet2 {
    #[doc = "No lock-the adjacent (next lower) bit can be written by the software."]
    L_ENET2_0 = 0x0,
    #[doc = "Lock-the adjacent (next lower) bit can't be written by the software."]
    L_ENET2_1 = 0x01,
}
impl SaLEnet2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SaLEnet2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SaLEnet2 {
    #[inline(always)]
    fn from(val: u8) -> SaLEnet2 {
        SaLEnet2::from_bits(val)
    }
}
impl From<SaLEnet2> for u8 {
    #[inline(always)]
    fn from(val: SaLEnet2) -> u8 {
        SaLEnet2::to_bits(val)
    }
}
