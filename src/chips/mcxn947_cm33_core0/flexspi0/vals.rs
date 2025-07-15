#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active {
    #[doc = "No suspended AHB read prefetch command."]
    VAL0 = 0x0,
    #[doc = "An AHB read prefetch command sequence has been suspended."]
    VAL1 = 0x01,
}
impl Active {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active {
    #[inline(always)]
    fn from(val: u8) -> Active {
        Active::from_bits(val)
    }
}
impl From<Active> for u8 {
    #[inline(always)]
    fn from(val: Active) -> u8 {
        Active::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbbustimeouten {
    #[doc = "Disable interrupt or no impact"]
    VALUE0 = 0x0,
    #[doc = "Enable interrupt"]
    VALUE1 = 0x01,
}
impl Ahbbustimeouten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbbustimeouten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbbustimeouten {
    #[inline(always)]
    fn from(val: u8) -> Ahbbustimeouten {
        Ahbbustimeouten::from_bits(val)
    }
}
impl From<Ahbbustimeouten> for u8 {
    #[inline(always)]
    fn from(val: Ahbbustimeouten) -> u8 {
        Ahbbustimeouten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbcmderrcode {
    #[doc = "No error"]
    VAL0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "AHB Write command with JMP_ON_CS instruction used in the sequence"]
    VAL2 = 0x02,
    #[doc = "Unknown instruction opcode in the sequence"]
    VAL3 = 0x03,
    #[doc = "DUMMY_SDR or DUMMY_RWDS_SDR instruction used in DDR sequence"]
    VAL4 = 0x04,
    #[doc = "DUMMY_DDR or DUMMY_RWDS_DDR instruction used in SDR sequence"]
    VAL5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "Sequence execution timeout"]
    VAL6 = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Ahbcmderrcode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbcmderrcode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbcmderrcode {
    #[inline(always)]
    fn from(val: u8) -> Ahbcmderrcode {
        Ahbcmderrcode::from_bits(val)
    }
}
impl From<Ahbcmderrcode> for u8 {
    #[inline(always)]
    fn from(val: Ahbcmderrcode) -> u8 {
        Ahbcmderrcode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbcmderren {
    #[doc = "Disable interrupt or no impact"]
    VALUE0 = 0x0,
    #[doc = "Enable interrupt"]
    VALUE1 = 0x01,
}
impl Ahbcmderren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbcmderren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbcmderren {
    #[inline(always)]
    fn from(val: u8) -> Ahbcmderren {
        Ahbcmderren::from_bits(val)
    }
}
impl From<Ahbcmderren> for u8 {
    #[inline(always)]
    fn from(val: Ahbcmderren) -> u8 {
        Ahbcmderren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbcmdgeen {
    #[doc = "Disable interrupt or no impact"]
    VALUE0 = 0x0,
    #[doc = "Enable interrupt"]
    VALUE1 = 0x01,
}
impl Ahbcmdgeen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbcmdgeen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbcmdgeen {
    #[inline(always)]
    fn from(val: u8) -> Ahbcmdgeen {
        Ahbcmdgeen::from_bits(val)
    }
}
impl From<Ahbcmdgeen> for u8 {
    #[inline(always)]
    fn from(val: Ahbcmdgeen) -> u8 {
        Ahbcmdgeen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbcrPrefetchen {
    #[doc = "Disable"]
    VALUE0 = 0x0,
    #[doc = "Enable"]
    VALUE1 = 0x01,
}
impl AhbcrPrefetchen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbcrPrefetchen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbcrPrefetchen {
    #[inline(always)]
    fn from(val: u8) -> AhbcrPrefetchen {
        AhbcrPrefetchen::from_bits(val)
    }
}
impl From<AhbcrPrefetchen> for u8 {
    #[inline(always)]
    fn from(val: AhbcrPrefetchen) -> u8 {
        AhbcrPrefetchen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbgcmerren {
    #[doc = "Disable interrupt or no impact"]
    VALUE0 = 0x0,
    #[doc = "Enable interrupt"]
    VALUE1 = 0x01,
}
impl Ahbgcmerren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbgcmerren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbgcmerren {
    #[inline(always)]
    fn from(val: u8) -> Ahbgcmerren {
        Ahbgcmerren::from_bits(val)
    }
}
impl From<Ahbgcmerren> for u8 {
    #[inline(always)]
    fn from(val: Ahbgcmerren) -> u8 {
        Ahbgcmerren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbgcmrd {
    #[doc = "Disable"]
    VAL0 = 0x0,
    #[doc = "Enable"]
    VAL1 = 0x01,
}
impl Ahbgcmrd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbgcmrd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbgcmrd {
    #[inline(always)]
    fn from(val: u8) -> Ahbgcmrd {
        Ahbgcmrd::from_bits(val)
    }
}
impl From<Ahbgcmrd> for u8 {
    #[inline(always)]
    fn from(val: Ahbgcmrd) -> u8 {
        Ahbgcmrd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbrdEn {
    #[doc = "Disable"]
    VAL0 = 0x0,
    #[doc = "Enable"]
    VAL1 = 0x01,
}
impl AhbrdEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbrdEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbrdEn {
    #[inline(always)]
    fn from(val: u8) -> AhbrdEn {
        AhbrdEn::from_bits(val)
    }
}
impl From<AhbrdEn> for u8 {
    #[inline(always)]
    fn from(val: AhbrdEn) -> u8 {
        AhbrdEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf0cr0Prefetchen {
    #[doc = "Disabled"]
    VALUE0 = 0x0,
    #[doc = "Enabled when is enabled."]
    VALUE1 = 0x01,
}
impl Ahbrxbuf0cr0Prefetchen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf0cr0Prefetchen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf0cr0Prefetchen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf0cr0Prefetchen {
        Ahbrxbuf0cr0Prefetchen::from_bits(val)
    }
}
impl From<Ahbrxbuf0cr0Prefetchen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf0cr0Prefetchen) -> u8 {
        Ahbrxbuf0cr0Prefetchen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf0cr0Regionen {
    #[doc = "Disabled. The buffer hit is based on the value of MSTRID only."]
    VALUE0 = 0x0,
    #[doc = "Enabled. The buffer hit is based on the value of MSTRID and the address within AHBBUFREGIONSTARTn and AHBREGIONENDn."]
    VALUE1 = 0x01,
}
impl Ahbrxbuf0cr0Regionen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf0cr0Regionen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf0cr0Regionen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf0cr0Regionen {
        Ahbrxbuf0cr0Regionen::from_bits(val)
    }
}
impl From<Ahbrxbuf0cr0Regionen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf0cr0Regionen) -> u8 {
        Ahbrxbuf0cr0Regionen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf1cr0Prefetchen {
    #[doc = "Disabled"]
    VALUE0 = 0x0,
    #[doc = "Enabled when is enabled."]
    VALUE1 = 0x01,
}
impl Ahbrxbuf1cr0Prefetchen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf1cr0Prefetchen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf1cr0Prefetchen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf1cr0Prefetchen {
        Ahbrxbuf1cr0Prefetchen::from_bits(val)
    }
}
impl From<Ahbrxbuf1cr0Prefetchen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf1cr0Prefetchen) -> u8 {
        Ahbrxbuf1cr0Prefetchen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf1cr0Regionen {
    #[doc = "Disabled. The buffer hit is based on the value of MSTRID only."]
    VALUE0 = 0x0,
    #[doc = "Enabled. The buffer hit is based on the value of MSTRID and the address within AHBBUFREGIONSTARTn and AHBREGIONENDn."]
    VALUE1 = 0x01,
}
impl Ahbrxbuf1cr0Regionen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf1cr0Regionen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf1cr0Regionen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf1cr0Regionen {
        Ahbrxbuf1cr0Regionen::from_bits(val)
    }
}
impl From<Ahbrxbuf1cr0Regionen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf1cr0Regionen) -> u8 {
        Ahbrxbuf1cr0Regionen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf2cr0Prefetchen {
    #[doc = "Disabled"]
    VALUE0 = 0x0,
    #[doc = "Enabled when is enabled."]
    VALUE1 = 0x01,
}
impl Ahbrxbuf2cr0Prefetchen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf2cr0Prefetchen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf2cr0Prefetchen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf2cr0Prefetchen {
        Ahbrxbuf2cr0Prefetchen::from_bits(val)
    }
}
impl From<Ahbrxbuf2cr0Prefetchen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf2cr0Prefetchen) -> u8 {
        Ahbrxbuf2cr0Prefetchen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf2cr0Regionen {
    #[doc = "Disabled. The buffer hit is based on the value of MSTRID only."]
    VALUE0 = 0x0,
    #[doc = "Enabled. The buffer hit is based on the value of MSTRID and the address within AHBBUFREGIONSTARTn and AHBREGIONENDn."]
    VALUE1 = 0x01,
}
impl Ahbrxbuf2cr0Regionen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf2cr0Regionen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf2cr0Regionen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf2cr0Regionen {
        Ahbrxbuf2cr0Regionen::from_bits(val)
    }
}
impl From<Ahbrxbuf2cr0Regionen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf2cr0Regionen) -> u8 {
        Ahbrxbuf2cr0Regionen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf3cr0Prefetchen {
    #[doc = "Disabled"]
    VALUE0 = 0x0,
    #[doc = "Enabled when is enabled."]
    VALUE1 = 0x01,
}
impl Ahbrxbuf3cr0Prefetchen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf3cr0Prefetchen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf3cr0Prefetchen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf3cr0Prefetchen {
        Ahbrxbuf3cr0Prefetchen::from_bits(val)
    }
}
impl From<Ahbrxbuf3cr0Prefetchen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf3cr0Prefetchen) -> u8 {
        Ahbrxbuf3cr0Prefetchen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf3cr0Regionen {
    #[doc = "Disabled. The buffer hit is based on the value of MSTRID only."]
    VALUE0 = 0x0,
    #[doc = "Enabled. The buffer hit is based on the value of MSTRID and the address within AHBBUFREGIONSTARTn and AHBREGIONENDn."]
    VALUE1 = 0x01,
}
impl Ahbrxbuf3cr0Regionen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf3cr0Regionen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf3cr0Regionen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf3cr0Regionen {
        Ahbrxbuf3cr0Regionen::from_bits(val)
    }
}
impl From<Ahbrxbuf3cr0Regionen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf3cr0Regionen) -> u8 {
        Ahbrxbuf3cr0Regionen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf4cr0Prefetchen {
    #[doc = "Disabled"]
    VALUE0 = 0x0,
    #[doc = "Enabled when is enabled."]
    VALUE1 = 0x01,
}
impl Ahbrxbuf4cr0Prefetchen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf4cr0Prefetchen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf4cr0Prefetchen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf4cr0Prefetchen {
        Ahbrxbuf4cr0Prefetchen::from_bits(val)
    }
}
impl From<Ahbrxbuf4cr0Prefetchen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf4cr0Prefetchen) -> u8 {
        Ahbrxbuf4cr0Prefetchen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf4cr0Regionen {
    #[doc = "Disabled. The buffer hit is based on the value of MSTRID only."]
    VALUE0 = 0x0,
    #[doc = "Enabled. The buffer hit is based on the value of MSTRID and the address within AHBBUFREGIONSTARTn and AHBREGIONENDn."]
    VALUE1 = 0x01,
}
impl Ahbrxbuf4cr0Regionen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf4cr0Regionen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf4cr0Regionen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf4cr0Regionen {
        Ahbrxbuf4cr0Regionen::from_bits(val)
    }
}
impl From<Ahbrxbuf4cr0Regionen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf4cr0Regionen) -> u8 {
        Ahbrxbuf4cr0Regionen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf5cr0Prefetchen {
    #[doc = "Disabled"]
    VALUE0 = 0x0,
    #[doc = "Enabled when is enabled."]
    VALUE1 = 0x01,
}
impl Ahbrxbuf5cr0Prefetchen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf5cr0Prefetchen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf5cr0Prefetchen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf5cr0Prefetchen {
        Ahbrxbuf5cr0Prefetchen::from_bits(val)
    }
}
impl From<Ahbrxbuf5cr0Prefetchen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf5cr0Prefetchen) -> u8 {
        Ahbrxbuf5cr0Prefetchen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf5cr0Regionen {
    #[doc = "Disabled. The buffer hit is based on the value of MSTRID only."]
    VALUE0 = 0x0,
    #[doc = "Enabled. The buffer hit is based on the value of MSTRID and the address within AHBBUFREGIONSTARTn and AHBREGIONENDn."]
    VALUE1 = 0x01,
}
impl Ahbrxbuf5cr0Regionen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf5cr0Regionen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf5cr0Regionen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf5cr0Regionen {
        Ahbrxbuf5cr0Regionen::from_bits(val)
    }
}
impl From<Ahbrxbuf5cr0Regionen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf5cr0Regionen) -> u8 {
        Ahbrxbuf5cr0Regionen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf6cr0Prefetchen {
    #[doc = "Disabled"]
    VALUE0 = 0x0,
    #[doc = "Enabled when is enabled."]
    VALUE1 = 0x01,
}
impl Ahbrxbuf6cr0Prefetchen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf6cr0Prefetchen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf6cr0Prefetchen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf6cr0Prefetchen {
        Ahbrxbuf6cr0Prefetchen::from_bits(val)
    }
}
impl From<Ahbrxbuf6cr0Prefetchen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf6cr0Prefetchen) -> u8 {
        Ahbrxbuf6cr0Prefetchen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf6cr0Regionen {
    #[doc = "Disabled. The buffer hit is based on the value of MSTRID only."]
    VALUE0 = 0x0,
    #[doc = "Enabled. The buffer hit is based on the value of MSTRID and the address within AHBBUFREGIONSTARTn and AHBREGIONENDn."]
    VALUE1 = 0x01,
}
impl Ahbrxbuf6cr0Regionen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf6cr0Regionen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf6cr0Regionen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf6cr0Regionen {
        Ahbrxbuf6cr0Regionen::from_bits(val)
    }
}
impl From<Ahbrxbuf6cr0Regionen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf6cr0Regionen) -> u8 {
        Ahbrxbuf6cr0Regionen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf7cr0Prefetchen {
    #[doc = "Disabled"]
    VALUE0 = 0x0,
    #[doc = "Enabled when is enabled."]
    VALUE1 = 0x01,
}
impl Ahbrxbuf7cr0Prefetchen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf7cr0Prefetchen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf7cr0Prefetchen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf7cr0Prefetchen {
        Ahbrxbuf7cr0Prefetchen::from_bits(val)
    }
}
impl From<Ahbrxbuf7cr0Prefetchen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf7cr0Prefetchen) -> u8 {
        Ahbrxbuf7cr0Prefetchen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf7cr0Regionen {
    #[doc = "Disabled. The buffer hit is based on the value of MSTRID only."]
    VALUE0 = 0x0,
    #[doc = "Enabled. The buffer hit is based on the value of MSTRID and the address within AHBBUFREGIONSTARTn and AHBREGIONENDn."]
    VALUE1 = 0x01,
}
impl Ahbrxbuf7cr0Regionen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf7cr0Regionen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf7cr0Regionen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf7cr0Regionen {
        Ahbrxbuf7cr0Regionen::from_bits(val)
    }
}
impl From<Ahbrxbuf7cr0Regionen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf7cr0Regionen) -> u8 {
        Ahbrxbuf7cr0Regionen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbwrEn {
    #[doc = "Disable"]
    VAL0 = 0x0,
    #[doc = "Enable"]
    VAL1 = 0x01,
}
impl AhbwrEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbwrEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbwrEn {
    #[inline(always)]
    fn from(val: u8) -> AhbwrEn {
        AhbwrEn::from_bits(val)
    }
}
impl From<AhbwrEn> for u8 {
    #[inline(always)]
    fn from(val: AhbwrEn) -> u8 {
        AhbwrEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahgcmwr {
    #[doc = "Disable"]
    VAL0 = 0x0,
    #[doc = "Enable"]
    VAL1 = 0x01,
}
impl Ahgcmwr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahgcmwr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahgcmwr {
    #[inline(always)]
    fn from(val: u8) -> Ahgcmwr {
        Ahgcmwr::from_bits(val)
    }
}
impl From<Ahgcmwr> for u8 {
    #[inline(always)]
    fn from(val: Ahgcmwr) -> u8 {
        Ahgcmwr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Alignment {
    #[doc = "No limit"]
    BIT0 = 0x0,
    #[doc = "1 KB"]
    BIT1 = 0x01,
    #[doc = "512 bytes"]
    BIT2 = 0x02,
    #[doc = "256 bytes"]
    BIT3 = 0x03,
}
impl Alignment {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Alignment {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Alignment {
    #[inline(always)]
    fn from(val: u8) -> Alignment {
        Alignment::from_bits(val)
    }
}
impl From<Alignment> for u8 {
    #[inline(always)]
    fn from(val: Alignment) -> u8 {
        Alignment::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aparen {
    #[doc = "Flash is accessed in Individual mode."]
    INDIVIDUAL = 0x0,
    #[doc = "Flash is accessed in Parallel mode."]
    ENABLE = 0x01,
}
impl Aparen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aparen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aparen {
    #[inline(always)]
    fn from(val: u8) -> Aparen {
        Aparen::from_bits(val)
    }
}
impl From<Aparen> for u8 {
    #[inline(always)]
    fn from(val: Aparen) -> u8 {
        Aparen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Arbcmdsrc {
    #[doc = "Trigger source is AHB read command."]
    VAL0 = 0x0,
    #[doc = "Trigger source is AHB write command."]
    VAL1 = 0x01,
    #[doc = "Trigger source is IP command (by writing 1 to IPCMD\\[TRG\\])."]
    VAL2 = 0x02,
    #[doc = "Trigger source is a suspended command that has resumed."]
    VAL3 = 0x03,
}
impl Arbcmdsrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Arbcmdsrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Arbcmdsrc {
    #[inline(always)]
    fn from(val: u8) -> Arbcmdsrc {
        Arbcmdsrc::from_bits(val)
    }
}
impl From<Arbcmdsrc> for u8 {
    #[inline(always)]
    fn from(val: Arbcmdsrc) -> u8 {
        Arbcmdsrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Arbidle {
    #[doc = "Not idle"]
    VALUE0 = 0x0,
    #[doc = "Idle"]
    VALUE1 = 0x01,
}
impl Arbidle {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Arbidle {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Arbidle {
    #[inline(always)]
    fn from(val: u8) -> Arbidle {
        Arbidle::from_bits(val)
    }
}
impl From<Arbidle> for u8 {
    #[inline(always)]
    fn from(val: Arbidle) -> u8 {
        Arbidle::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ardfen {
    #[doc = "AHB read access disabled. IP bus reads IP receive FIFO. AHB Bus read access to IP receive FIFO memory space produces bus error."]
    VAL0 = 0x0,
    #[doc = "AHB read access enabled. AHB bus reads IP receive FIFO. IP Bus read access to IP receive FIFO memory space returns data zero and causes no bus error."]
    VAL1 = 0x01,
}
impl Ardfen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ardfen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ardfen {
    #[inline(always)]
    fn from(val: u8) -> Ardfen {
        Ardfen::from_bits(val)
    }
}
impl From<Ardfen> for u8 {
    #[inline(always)]
    fn from(val: Ardfen) -> u8 {
        Ardfen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Areflock {
    #[doc = "Not locked"]
    VAL0 = 0x0,
    #[doc = "Locked"]
    VAL1 = 0x01,
}
impl Areflock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Areflock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Areflock {
    #[inline(always)]
    fn from(val: u8) -> Areflock {
        Areflock::from_bits(val)
    }
}
impl From<Areflock> for u8 {
    #[inline(always)]
    fn from(val: Areflock) -> u8 {
        Areflock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aslvlock {
    #[doc = "Not locked"]
    VAL0 = 0x0,
    #[doc = "Locked"]
    VAL1 = 0x01,
}
impl Aslvlock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aslvlock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aslvlock {
    #[inline(always)]
    fn from(val: u8) -> Aslvlock {
        Aslvlock::from_bits(val)
    }
}
impl From<Aslvlock> for u8 {
    #[inline(always)]
    fn from(val: Aslvlock) -> u8 {
        Aslvlock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Atdfen {
    #[doc = "AHB write access disabled. IP bus writes to IP transmit FIFO. AHB bus write access to IP transmit FIFO memory space produces bus error."]
    VAL0 = 0x0,
    #[doc = "AHB write access enabled. AHB bus writes to IP transmit FIFO. IP Bus write access to IP transmit FIFO memory space is ignored and causes no bus error."]
    VAL1 = 0x01,
}
impl Atdfen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Atdfen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Atdfen {
    #[inline(always)]
    fn from(val: u8) -> Atdfen {
        Atdfen::from_bits(val)
    }
}
impl From<Atdfen> for u8 {
    #[inline(always)]
    fn from(val: Atdfen) -> u8 {
        Atdfen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Awrwaitunit {
    #[doc = "2"]
    VAL0 = 0x0,
    #[doc = "8"]
    VAL1 = 0x01,
    #[doc = "32"]
    VAL2 = 0x02,
    #[doc = "128"]
    VAL3 = 0x03,
    #[doc = "512"]
    VAL4 = 0x04,
    #[doc = "2048"]
    VAL5 = 0x05,
    #[doc = "8192"]
    VAL6 = 0x06,
    #[doc = "32768"]
    VAL7 = 0x07,
}
impl Awrwaitunit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Awrwaitunit {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Awrwaitunit {
    #[inline(always)]
    fn from(val: u8) -> Awrwaitunit {
        Awrwaitunit::from_bits(val)
    }
}
impl From<Awrwaitunit> for u8 {
    #[inline(always)]
    fn from(val: Awrwaitunit) -> u8 {
        Awrwaitunit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Breflock {
    #[doc = "Not locked"]
    VAL0 = 0x0,
    #[doc = "Locked"]
    VAL1 = 0x01,
}
impl Breflock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Breflock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Breflock {
    #[inline(always)]
    fn from(val: u8) -> Breflock {
        Breflock::from_bits(val)
    }
}
impl From<Breflock> for u8 {
    #[inline(always)]
    fn from(val: Breflock) -> u8 {
        Breflock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bslvlock {
    #[doc = "Not locked"]
    VAL0 = 0x0,
    #[doc = "Locked"]
    VAL1 = 0x01,
}
impl Bslvlock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bslvlock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bslvlock {
    #[inline(always)]
    fn from(val: u8) -> Bslvlock {
        Bslvlock::from_bits(val)
    }
}
impl From<Bslvlock> for u8 {
    #[inline(always)]
    fn from(val: Bslvlock) -> u8 {
        Bslvlock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bufferableen {
    #[doc = "Disabled. For all AHB write accesses (bufferable or nonbufferable), FlexSPI returns AHB Bus Ready after transmitting all data and finishing command."]
    VAL0 = 0x0,
    #[doc = "Enabled. For AHB bufferable write access, FlexSPI returns AHB Bus Ready when the arbitrator grants the AHB command. FlexSPI does not wait for the AHB command to finish."]
    VAL1 = 0x01,
}
impl Bufferableen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bufferableen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bufferableen {
    #[inline(always)]
    fn from(val: u8) -> Bufferableen {
        Bufferableen::from_bits(val)
    }
}
impl From<Bufferableen> for u8 {
    #[inline(always)]
    fn from(val: Bufferableen) -> u8 {
        Bufferableen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cachableen {
    #[doc = "Disabled. When an AHB bus cacheable read access occurs, FlexSPI does not check whether it hit the AHB transmit buffer."]
    VAL0 = 0x0,
    #[doc = "Enabled. When an AHB bus cacheable read access occurs, FlexSPI first checks whether the access hit the AHB transmit buffer."]
    VAL1 = 0x01,
}
impl Cachableen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cachableen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cachableen {
    #[inline(always)]
    fn from(val: u8) -> Cachableen {
        Cachableen::from_bits(val)
    }
}
impl From<Cachableen> for u8 {
    #[inline(always)]
    fn from(val: Cachableen) -> u8 {
        Cachableen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrahbbufopt {
    #[doc = "Not cleared automatically"]
    VAL0 = 0x0,
    #[doc = "Cleared automatically"]
    VAL1 = 0x01,
}
impl Clrahbbufopt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrahbbufopt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrahbbufopt {
    #[inline(always)]
    fn from(val: u8) -> Clrahbbufopt {
        Clrahbbufopt::from_bits(val)
    }
}
impl From<Clrahbbufopt> for u8 {
    #[inline(always)]
    fn from(val: Clrahbbufopt) -> u8 {
        Clrahbbufopt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrahbrxbuf {
    #[doc = "No impact."]
    VAL0 = 0x0,
    #[doc = "Enable clear operation."]
    VAL1 = 0x01,
}
impl Clrahbrxbuf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrahbrxbuf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrahbrxbuf {
    #[inline(always)]
    fn from(val: u8) -> Clrahbrxbuf {
        Clrahbrxbuf::from_bits(val)
    }
}
impl From<Clrahbrxbuf> for u8 {
    #[inline(always)]
    fn from(val: Clrahbrxbuf) -> u8 {
        Clrahbrxbuf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrahbtxbuf {
    #[doc = "No impact."]
    VAL0 = 0x0,
    #[doc = "Enable clear operation."]
    VAL1 = 0x01,
}
impl Clrahbtxbuf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrahbtxbuf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrahbtxbuf {
    #[inline(always)]
    fn from(val: u8) -> Clrahbtxbuf {
        Clrahbtxbuf::from_bits(val)
    }
}
impl From<Clrahbtxbuf> for u8 {
    #[inline(always)]
    fn from(val: Clrahbtxbuf) -> u8 {
        Clrahbtxbuf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clriprxf {
    #[doc = "No function"]
    VALUE0 = 0x0,
    #[doc = "A clock cycle pulse clears all valid data entries in IP receive FIFO."]
    VALUE1 = 0x01,
}
impl Clriprxf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clriprxf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clriprxf {
    #[inline(always)]
    fn from(val: u8) -> Clriprxf {
        Clriprxf::from_bits(val)
    }
}
impl From<Clriprxf> for u8 {
    #[inline(always)]
    fn from(val: Clriprxf) -> u8 {
        Clriprxf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clriptxf {
    #[doc = "No function"]
    VALUE0 = 0x0,
    #[doc = "A clock cycle pulse clears all valid data entries in the IP transmit FIFO."]
    VALUE1 = 0x01,
}
impl Clriptxf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clriptxf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clriptxf {
    #[inline(always)]
    fn from(val: u8) -> Clriptxf {
        Clriptxf::from_bits(val)
    }
}
impl From<Clriptxf> for u8 {
    #[inline(always)]
    fn from(val: Clriptxf) -> u8 {
        Clriptxf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrlearnphase {
    #[doc = "No impact"]
    VAL0 = 0x0,
    #[doc = "Reset sample clock phase selection to 0"]
    VAL1 = 0x01,
}
impl Clrlearnphase {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrlearnphase {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrlearnphase {
    #[inline(always)]
    fn from(val: u8) -> Clrlearnphase {
        Clrlearnphase::from_bits(val)
    }
}
impl From<Clrlearnphase> for u8 {
    #[inline(always)]
    fn from(val: Clrlearnphase) -> u8 {
        Clrlearnphase::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Config {
    #[doc = "Fully pipelined"]
    VAL0 = 0x0,
    #[doc = "Not fully pipelined"]
    VAL1 = 0x01,
}
impl Config {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Config {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Config {
    #[inline(always)]
    fn from(val: u8) -> Config {
        Config::from_bits(val)
    }
}
impl From<Config> for u8 {
    #[inline(always)]
    fn from(val: Config) -> u8 {
        Config::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csintervalunit {
    #[doc = "1 serial clock cycle"]
    VAL0 = 0x0,
    #[doc = "256 serial clock cycles"]
    VAL1 = 0x01,
}
impl Csintervalunit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csintervalunit {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csintervalunit {
    #[inline(always)]
    fn from(val: u8) -> Csintervalunit {
        Csintervalunit::from_bits(val)
    }
}
impl From<Csintervalunit> for u8 {
    #[inline(always)]
    fn from(val: Csintervalunit) -> u8 {
        Csintervalunit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Datalearnfailen {
    #[doc = "Disable interrupt or no impact"]
    VALUE0 = 0x0,
    #[doc = "Enable interrupt"]
    VALUE1 = 0x01,
}
impl Datalearnfailen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Datalearnfailen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Datalearnfailen {
    #[inline(always)]
    fn from(val: u8) -> Datalearnfailen {
        Datalearnfailen::from_bits(val)
    }
}
impl From<Datalearnfailen> for u8 {
    #[inline(always)]
    fn from(val: Datalearnfailen) -> u8 {
        Datalearnfailen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dllen {
    #[doc = "Disable"]
    VALUE0 = 0x0,
    #[doc = "Enable"]
    VALUE1 = 0x01,
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
    #[doc = "No function"]
    VALUE0 = 0x0,
    #[doc = "Force DLL reset."]
    VALUE1 = 0x01,
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
pub enum Dozeen {
    #[doc = "Disable"]
    VAL0 = 0x0,
    #[doc = "Enable"]
    VAL1 = 0x01,
}
impl Dozeen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dozeen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dozeen {
    #[inline(always)]
    fn from(val: u8) -> Dozeen {
        Dozeen::from_bits(val)
    }
}
impl From<Dozeen> for u8 {
    #[inline(always)]
    fn from(val: Dozeen) -> u8 {
        Dozeen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsen {
    #[doc = "Disable"]
    VAL0 = 0x0,
    #[doc = "Enable"]
    VAL1 = 0x01,
}
impl Hsen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsen {
    #[inline(always)]
    fn from(val: u8) -> Hsen {
        Hsen::from_bits(val)
    }
}
impl From<Hsen> for u8 {
    #[inline(always)]
    fn from(val: Hsen) -> u8 {
        Hsen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipblkahback {
    #[doc = "IP commands do not block AHB command acknowledgment."]
    VALUE0 = 0x0,
    #[doc = "IP commands block AHB command acknowledgment."]
    VALUE1 = 0x01,
}
impl Ipblkahback {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipblkahback {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipblkahback {
    #[inline(always)]
    fn from(val: u8) -> Ipblkahback {
        Ipblkahback::from_bits(val)
    }
}
impl From<Ipblkahback> for u8 {
    #[inline(always)]
    fn from(val: Ipblkahback) -> u8 {
        Ipblkahback::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipblkahbreq {
    #[doc = "IP commands do not block AHB command requests."]
    VALUE0 = 0x0,
    #[doc = "IP commands block AHB command requests."]
    VALUE1 = 0x01,
}
impl Ipblkahbreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipblkahbreq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipblkahbreq {
    #[inline(always)]
    fn from(val: u8) -> Ipblkahbreq {
        Ipblkahbreq::from_bits(val)
    }
}
impl From<Ipblkahbreq> for u8 {
    #[inline(always)]
    fn from(val: Ipblkahbreq) -> u8 {
        Ipblkahbreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipblkallahb {
    #[doc = "IP commands only block AHB commands that affect the IPED region."]
    VALUE0 = 0x0,
    #[doc = "IP commands block all AHB commands."]
    VALUE1 = 0x01,
}
impl Ipblkallahb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipblkallahb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipblkallahb {
    #[inline(always)]
    fn from(val: u8) -> Ipblkallahb {
        Ipblkallahb::from_bits(val)
    }
}
impl From<Ipblkallahb> for u8 {
    #[inline(always)]
    fn from(val: Ipblkallahb) -> u8 {
        Ipblkallahb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipcmddoneen {
    #[doc = "Disable interrupt or no impact"]
    VALUE0 = 0x0,
    #[doc = "Enable interrupt"]
    VALUE1 = 0x01,
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
pub enum Ipcmderrcode {
    #[doc = "No error"]
    VAL0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "IP command with JMP_ON_CS instruction used in the sequence"]
    VAL2 = 0x02,
    #[doc = "Unknown instruction opcode in the sequence"]
    VAL3 = 0x03,
    #[doc = "DUMMY_SDR or DUMMY_RWDS_SDR instruction used in DDR sequence"]
    VAL4 = 0x04,
    #[doc = "DUMMY_DDR or DUMMY_RWDS_DDR instruction used in SDR sequence"]
    VAL5 = 0x05,
    #[doc = "Flash memory access start address exceeds entire flash address range (A1, A2, B1, and B2)"]
    VAL6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "Sequence execution timeout"]
    VAL7 = 0x0e,
    #[doc = "Flash boundary crossed"]
    VAL8 = 0x0f,
}
impl Ipcmderrcode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipcmderrcode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipcmderrcode {
    #[inline(always)]
    fn from(val: u8) -> Ipcmderrcode {
        Ipcmderrcode::from_bits(val)
    }
}
impl From<Ipcmderrcode> for u8 {
    #[inline(always)]
    fn from(val: Ipcmderrcode) -> u8 {
        Ipcmderrcode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipcmderren {
    #[doc = "Disable interrupt or no impact"]
    VALUE0 = 0x0,
    #[doc = "Enable interrupt"]
    VALUE1 = 0x01,
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipcmdgeen {
    #[doc = "Disable interrupt or no impact"]
    VALUE0 = 0x0,
    #[doc = "Enable interrupt"]
    VALUE1 = 0x01,
}
impl Ipcmdgeen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipcmdgeen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipcmdgeen {
    #[inline(always)]
    fn from(val: u8) -> Ipcmdgeen {
        Ipcmdgeen::from_bits(val)
    }
}
impl From<Ipcmdgeen> for u8 {
    #[inline(always)]
    fn from(val: Ipcmdgeen) -> u8 {
        Ipcmdgeen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipcmdsecurevioen {
    #[doc = "Disable interrupt or no impact"]
    VALUE0 = 0x0,
    #[doc = "Enable interrupt"]
    VALUE1 = 0x01,
}
impl Ipcmdsecurevioen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipcmdsecurevioen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipcmdsecurevioen {
    #[inline(always)]
    fn from(val: u8) -> Ipcmdsecurevioen {
        Ipcmdsecurevioen::from_bits(val)
    }
}
impl From<Ipcmdsecurevioen> for u8 {
    #[inline(always)]
    fn from(val: Ipcmdsecurevioen) -> u8 {
        Ipcmdsecurevioen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IpedEn {
    #[doc = "Disable"]
    VAL0 = 0x0,
    #[doc = "Enable"]
    VAL1 = 0x01,
}
impl IpedEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IpedEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IpedEn {
    #[inline(always)]
    fn from(val: u8) -> IpedEn {
        IpedEn::from_bits(val)
    }
}
impl From<IpedEn> for u8 {
    #[inline(always)]
    fn from(val: IpedEn) -> u8 {
        IpedEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IpedProtect {
    #[doc = "No restrictions"]
    VAL0 = 0x0,
    #[doc = "Only privileged controllers can write IPED registers."]
    VAL1 = 0x01,
}
impl IpedProtect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IpedProtect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IpedProtect {
    #[inline(always)]
    fn from(val: u8) -> IpedProtect {
        IpedProtect::from_bits(val)
    }
}
impl From<IpedProtect> for u8 {
    #[inline(always)]
    fn from(val: IpedProtect) -> u8 {
        IpedProtect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IpedSwreset {
    #[doc = "No function."]
    VAL0 = 0x0,
    #[doc = "Aborts current decryption or encryption and waits for the next start operation."]
    VAL1 = 0x01,
}
impl IpedSwreset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IpedSwreset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IpedSwreset {
    #[inline(always)]
    fn from(val: u8) -> IpedSwreset {
        IpedSwreset::from_bits(val)
    }
}
impl From<IpedSwreset> for u8 {
    #[inline(always)]
    fn from(val: IpedSwreset) -> u8 {
        IpedSwreset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipedctx0startAhbbuserrorDis {
    #[doc = "AHB bus errors enabled"]
    VALUE0 = 0x0,
    #[doc = "AHB bus errors disabled"]
    VALUE1 = 0x01,
}
impl Ipedctx0startAhbbuserrorDis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipedctx0startAhbbuserrorDis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipedctx0startAhbbuserrorDis {
    #[inline(always)]
    fn from(val: u8) -> Ipedctx0startAhbbuserrorDis {
        Ipedctx0startAhbbuserrorDis::from_bits(val)
    }
}
impl From<Ipedctx0startAhbbuserrorDis> for u8 {
    #[inline(always)]
    fn from(val: Ipedctx0startAhbbuserrorDis) -> u8 {
        Ipedctx0startAhbbuserrorDis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipedctx0startGcm {
    #[doc = "Disabled. CTR mode is used."]
    VALUE0 = 0x0,
    #[doc = "Enabled. GCM mode is used."]
    VALUE1 = 0x01,
}
impl Ipedctx0startGcm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipedctx0startGcm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipedctx0startGcm {
    #[inline(always)]
    fn from(val: u8) -> Ipedctx0startGcm {
        Ipedctx0startGcm::from_bits(val)
    }
}
impl From<Ipedctx0startGcm> for u8 {
    #[inline(always)]
    fn from(val: Ipedctx0startGcm) -> u8 {
        Ipedctx0startGcm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipedctx1startAhbbuserrorDis {
    #[doc = "AHB bus errors enabled"]
    VALUE0 = 0x0,
    #[doc = "AHB bus errors disabled"]
    VALUE1 = 0x01,
}
impl Ipedctx1startAhbbuserrorDis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipedctx1startAhbbuserrorDis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipedctx1startAhbbuserrorDis {
    #[inline(always)]
    fn from(val: u8) -> Ipedctx1startAhbbuserrorDis {
        Ipedctx1startAhbbuserrorDis::from_bits(val)
    }
}
impl From<Ipedctx1startAhbbuserrorDis> for u8 {
    #[inline(always)]
    fn from(val: Ipedctx1startAhbbuserrorDis) -> u8 {
        Ipedctx1startAhbbuserrorDis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipedctx1startGcm {
    #[doc = "Disabled. CTR mode is used."]
    VALUE0 = 0x0,
    #[doc = "Enabled. GCM mode is used."]
    VALUE1 = 0x01,
}
impl Ipedctx1startGcm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipedctx1startGcm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipedctx1startGcm {
    #[inline(always)]
    fn from(val: u8) -> Ipedctx1startGcm {
        Ipedctx1startGcm::from_bits(val)
    }
}
impl From<Ipedctx1startGcm> for u8 {
    #[inline(always)]
    fn from(val: Ipedctx1startGcm) -> u8 {
        Ipedctx1startGcm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipedctx2startAhbbuserrorDis {
    #[doc = "AHB bus errors enabled"]
    VALUE0 = 0x0,
    #[doc = "AHB bus errors disabled"]
    VALUE1 = 0x01,
}
impl Ipedctx2startAhbbuserrorDis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipedctx2startAhbbuserrorDis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipedctx2startAhbbuserrorDis {
    #[inline(always)]
    fn from(val: u8) -> Ipedctx2startAhbbuserrorDis {
        Ipedctx2startAhbbuserrorDis::from_bits(val)
    }
}
impl From<Ipedctx2startAhbbuserrorDis> for u8 {
    #[inline(always)]
    fn from(val: Ipedctx2startAhbbuserrorDis) -> u8 {
        Ipedctx2startAhbbuserrorDis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipedctx2startGcm {
    #[doc = "Disabled. CTR mode is used."]
    VALUE0 = 0x0,
    #[doc = "Enabled. GCM mode is used."]
    VALUE1 = 0x01,
}
impl Ipedctx2startGcm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipedctx2startGcm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipedctx2startGcm {
    #[inline(always)]
    fn from(val: u8) -> Ipedctx2startGcm {
        Ipedctx2startGcm::from_bits(val)
    }
}
impl From<Ipedctx2startGcm> for u8 {
    #[inline(always)]
    fn from(val: Ipedctx2startGcm) -> u8 {
        Ipedctx2startGcm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipedctx3startAhbbuserrorDis {
    #[doc = "AHB bus errors enabled"]
    VALUE0 = 0x0,
    #[doc = "AHB bus errors disabled"]
    VALUE1 = 0x01,
}
impl Ipedctx3startAhbbuserrorDis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipedctx3startAhbbuserrorDis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipedctx3startAhbbuserrorDis {
    #[inline(always)]
    fn from(val: u8) -> Ipedctx3startAhbbuserrorDis {
        Ipedctx3startAhbbuserrorDis::from_bits(val)
    }
}
impl From<Ipedctx3startAhbbuserrorDis> for u8 {
    #[inline(always)]
    fn from(val: Ipedctx3startAhbbuserrorDis) -> u8 {
        Ipedctx3startAhbbuserrorDis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipedctx3startGcm {
    #[doc = "Disabled. CTR mode is used."]
    VALUE0 = 0x0,
    #[doc = "Enabled. GCM mode is used."]
    VALUE1 = 0x01,
}
impl Ipedctx3startGcm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipedctx3startGcm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipedctx3startGcm {
    #[inline(always)]
    fn from(val: u8) -> Ipedctx3startGcm {
        Ipedctx3startGcm::from_bits(val)
    }
}
impl From<Ipedctx3startGcm> for u8 {
    #[inline(always)]
    fn from(val: Ipedctx3startGcm) -> u8 {
        Ipedctx3startGcm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipedctx4startAhbbuserrorDis {
    #[doc = "AHB bus errors enabled"]
    VALUE0 = 0x0,
    #[doc = "AHB bus errors disabled"]
    VALUE1 = 0x01,
}
impl Ipedctx4startAhbbuserrorDis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipedctx4startAhbbuserrorDis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipedctx4startAhbbuserrorDis {
    #[inline(always)]
    fn from(val: u8) -> Ipedctx4startAhbbuserrorDis {
        Ipedctx4startAhbbuserrorDis::from_bits(val)
    }
}
impl From<Ipedctx4startAhbbuserrorDis> for u8 {
    #[inline(always)]
    fn from(val: Ipedctx4startAhbbuserrorDis) -> u8 {
        Ipedctx4startAhbbuserrorDis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipedctx4startGcm {
    #[doc = "Disabled. CTR mode is used."]
    VALUE0 = 0x0,
    #[doc = "Enabled. GCM mode is used."]
    VALUE1 = 0x01,
}
impl Ipedctx4startGcm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipedctx4startGcm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipedctx4startGcm {
    #[inline(always)]
    fn from(val: u8) -> Ipedctx4startGcm {
        Ipedctx4startGcm::from_bits(val)
    }
}
impl From<Ipedctx4startGcm> for u8 {
    #[inline(always)]
    fn from(val: Ipedctx4startGcm) -> u8 {
        Ipedctx4startGcm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipedctx5startAhbbuserrorDis {
    #[doc = "AHB bus errors enabled"]
    VALUE0 = 0x0,
    #[doc = "AHB bus errors disabled"]
    VALUE1 = 0x01,
}
impl Ipedctx5startAhbbuserrorDis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipedctx5startAhbbuserrorDis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipedctx5startAhbbuserrorDis {
    #[inline(always)]
    fn from(val: u8) -> Ipedctx5startAhbbuserrorDis {
        Ipedctx5startAhbbuserrorDis::from_bits(val)
    }
}
impl From<Ipedctx5startAhbbuserrorDis> for u8 {
    #[inline(always)]
    fn from(val: Ipedctx5startAhbbuserrorDis) -> u8 {
        Ipedctx5startAhbbuserrorDis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipedctx5startGcm {
    #[doc = "Disabled. CTR mode is used."]
    VALUE0 = 0x0,
    #[doc = "Enabled. GCM mode is used."]
    VALUE1 = 0x01,
}
impl Ipedctx5startGcm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipedctx5startGcm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipedctx5startGcm {
    #[inline(always)]
    fn from(val: u8) -> Ipedctx5startGcm {
        Ipedctx5startGcm::from_bits(val)
    }
}
impl From<Ipedctx5startGcm> for u8 {
    #[inline(always)]
    fn from(val: Ipedctx5startGcm) -> u8 {
        Ipedctx5startGcm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipedctx6startAhbbuserrorDis {
    #[doc = "AHB bus errors enabled"]
    VALUE0 = 0x0,
    #[doc = "AHB bus errors disabled"]
    VALUE1 = 0x01,
}
impl Ipedctx6startAhbbuserrorDis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipedctx6startAhbbuserrorDis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipedctx6startAhbbuserrorDis {
    #[inline(always)]
    fn from(val: u8) -> Ipedctx6startAhbbuserrorDis {
        Ipedctx6startAhbbuserrorDis::from_bits(val)
    }
}
impl From<Ipedctx6startAhbbuserrorDis> for u8 {
    #[inline(always)]
    fn from(val: Ipedctx6startAhbbuserrorDis) -> u8 {
        Ipedctx6startAhbbuserrorDis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipedctx6startGcm {
    #[doc = "Disabled. CTR mode is used."]
    VALUE0 = 0x0,
    #[doc = "Enabled. GCM mode is used."]
    VALUE1 = 0x01,
}
impl Ipedctx6startGcm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipedctx6startGcm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipedctx6startGcm {
    #[inline(always)]
    fn from(val: u8) -> Ipedctx6startGcm {
        Ipedctx6startGcm::from_bits(val)
    }
}
impl From<Ipedctx6startGcm> for u8 {
    #[inline(always)]
    fn from(val: Ipedctx6startGcm) -> u8 {
        Ipedctx6startGcm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipgcmwr {
    #[doc = "Disabled"]
    VAL0 = 0x0,
    #[doc = "Enabled"]
    VAL1 = 0x01,
}
impl Ipgcmwr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipgcmwr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipgcmwr {
    #[inline(always)]
    fn from(val: u8) -> Ipgcmwr {
        Ipgcmwr::from_bits(val)
    }
}
impl From<Ipgcmwr> for u8 {
    #[inline(always)]
    fn from(val: Ipgcmwr) -> u8 {
        Ipgcmwr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iprxwaen {
    #[doc = "Disable interrupt or no impact"]
    VALUE0 = 0x0,
    #[doc = "Enable interrupt"]
    VALUE1 = 0x01,
}
impl Iprxwaen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iprxwaen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iprxwaen {
    #[inline(always)]
    fn from(val: u8) -> Iprxwaen {
        Iprxwaen::from_bits(val)
    }
}
impl From<Iprxwaen> for u8 {
    #[inline(always)]
    fn from(val: Iprxwaen) -> u8 {
        Iprxwaen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iptxween {
    #[doc = "Disable interrupt or no impact"]
    VALUE0 = 0x0,
    #[doc = "Enable interrupt"]
    VALUE1 = 0x01,
}
impl Iptxween {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iptxween {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iptxween {
    #[inline(always)]
    fn from(val: u8) -> Iptxween {
        Iptxween::from_bits(val)
    }
}
impl From<Iptxween> for u8 {
    #[inline(always)]
    fn from(val: Iptxween) -> u8 {
        Iptxween::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IpwrEn {
    #[doc = "Disable"]
    VAL0 = 0x0,
    #[doc = "Enable"]
    VAL1 = 0x01,
}
impl IpwrEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IpwrEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IpwrEn {
    #[inline(always)]
    fn from(val: u8) -> IpwrEn {
        IpwrEn::from_bits(val)
    }
}
impl From<IpwrEn> for u8 {
    #[inline(always)]
    fn from(val: IpwrEn) -> u8 {
        IpwrEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lock {
    #[doc = "LUT is unlocked (LUTCR\\[UNLOCK\\] must be 1)"]
    VALUE0 = 0x0,
    #[doc = "LUT is locked and cannot be written"]
    VALUE1 = 0x01,
}
impl Lock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lock {
    #[inline(always)]
    fn from(val: u8) -> Lock {
        Lock::from_bits(val)
    }
}
impl From<Lock> for u8 {
    #[inline(always)]
    fn from(val: Lock) -> u8 {
        Lock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mdis {
    #[doc = "No impact"]
    VAL0 = 0x0,
    #[doc = "Module disable"]
    VAL1 = 0x01,
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
pub enum Ovrden {
    #[doc = "Disable"]
    VALUE0 = 0x0,
    #[doc = "Enable"]
    VALUE1 = 0x01,
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Protect {
    #[doc = "Not protected. All IPS controllers can access LUTCR and LUT memory."]
    VALUE0 = 0x0,
    #[doc = "Protected. Only secure IPS controller can change the value of LUTCR and write to LUT memory."]
    VALUE1 = 0x01,
}
impl Protect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Protect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Protect {
    #[inline(always)]
    fn from(val: u8) -> Protect {
        Protect::from_bits(val)
    }
}
impl From<Protect> for u8 {
    #[inline(always)]
    fn from(val: Protect) -> u8 {
        Protect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Readaddropt {
    #[doc = "AHB read burst start address alignment is limited when flash memory is accessed in parallel mode or flash is word-addressable."]
    VAL0 = 0x0,
    #[doc = "AHB read burst start address alignment is not limited. FlexSPI fetches more data than the AHB burst requires for address alignment."]
    VAL1 = 0x01,
}
impl Readaddropt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Readaddropt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Readaddropt {
    #[inline(always)]
    fn from(val: u8) -> Readaddropt {
        Readaddropt::from_bits(val)
    }
}
impl From<Readaddropt> for u8 {
    #[inline(always)]
    fn from(val: Readaddropt) -> u8 {
        Readaddropt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Readszalign {
    #[doc = "Register settings such as PREFETCH_EN determine AHB read size."]
    VAL0 = 0x0,
    #[doc = "AHB read size to up size to 8 bytes aligned, no prefetching"]
    VAL1 = 0x01,
}
impl Readszalign {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Readszalign {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Readszalign {
    #[inline(always)]
    fn from(val: u8) -> Readszalign {
        Readszalign::from_bits(val)
    }
}
impl From<Readszalign> for u8 {
    #[inline(always)]
    fn from(val: Readszalign) -> u8 {
        Readszalign::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Remapen {
    #[doc = "HADDR REMAP Disabled"]
    VAL0 = 0x0,
    #[doc = "HADDR REMAP Enabled"]
    VAL1 = 0x01,
}
impl Remapen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Remapen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Remapen {
    #[inline(always)]
    fn from(val: u8) -> Remapen {
        Remapen::from_bits(val)
    }
}
impl From<Remapen> for u8 {
    #[inline(always)]
    fn from(val: Remapen) -> u8 {
        Remapen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Resumedisable {
    #[doc = "Suspended AHB read prefetch resumes when AHB is IDLE."]
    VAL0 = 0x0,
    #[doc = "Suspended AHB read prefetch does not resume once aborted,"]
    VAL1 = 0x01,
}
impl Resumedisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Resumedisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Resumedisable {
    #[inline(always)]
    fn from(val: u8) -> Resumedisable {
        Resumedisable::from_bits(val)
    }
}
impl From<Resumedisable> for u8 {
    #[inline(always)]
    fn from(val: Resumedisable) -> u8 {
        Resumedisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RxClkSrcDiff {
    #[doc = "Use MCR0\\[RXCLKSRC\\] for Port A and Port B. MCR2\\[RXCLKSRC_B\\] is ignored and MCR0\\[RXCLKSRC\\] selects the Sample Clock source for Flash Reading of both ports A and B."]
    VALUE0 = 0x0,
    #[doc = "Use MCR0\\[RXCLKSRC\\] for Port A, and MCR2\\[RXCLKSRC_B\\] for Port B. MCR0\\[RXCLKSRC\\] selects the Sample Clock source for Flash Reading of port A (A_SCLK) and MCR2\\[RXCLKSRC_B\\] selects the Sample Clock source for Flash Reading of port B (B_SCLK)."]
    VALUE1 = 0x01,
}
impl RxClkSrcDiff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxClkSrcDiff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxClkSrcDiff {
    #[inline(always)]
    fn from(val: u8) -> RxClkSrcDiff {
        RxClkSrcDiff::from_bits(val)
    }
}
impl From<RxClkSrcDiff> for u8 {
    #[inline(always)]
    fn from(val: RxClkSrcDiff) -> u8 {
        RxClkSrcDiff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxclksrc {
    #[doc = "Dummy Read strobe that FlexSPI generates, looped back internally"]
    VAL0 = 0x0,
    #[doc = "Dummy Read strobe that FlexSPI generates, looped back from DQS pad"]
    VAL1 = 0x01,
    #[doc = "SCLK output clock and looped back from SCLK pad"]
    VAL2 = 0x02,
    #[doc = "Flash-memory-provided read strobe and input from DQS pad"]
    VAL3 = 0x03,
}
impl Rxclksrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxclksrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxclksrc {
    #[inline(always)]
    fn from(val: u8) -> Rxclksrc {
        Rxclksrc::from_bits(val)
    }
}
impl From<Rxclksrc> for u8 {
    #[inline(always)]
    fn from(val: Rxclksrc) -> u8 {
        Rxclksrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RxclksrcB {
    #[doc = "Dummy read strobe that FlexSPI generates, looped back internally."]
    VAL0 = 0x0,
    #[doc = "Dummy read strobe that FlexSPI generates, looped back from DQS pad."]
    VAL1 = 0x01,
    #[doc = "SCLK output clock and looped back from SCLK pad"]
    VAL2 = 0x02,
    #[doc = "Flash-memory-provided read strobe and input from DQS pad"]
    VAL3 = 0x03,
}
impl RxclksrcB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxclksrcB {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxclksrcB {
    #[inline(always)]
    fn from(val: u8) -> RxclksrcB {
        RxclksrcB::from_bits(val)
    }
}
impl From<RxclksrcB> for u8 {
    #[inline(always)]
    fn from(val: RxclksrcB) -> u8 {
        RxclksrcB::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxdmaen {
    #[doc = "Disabled. The processor reads the FIFO."]
    VAL0 = 0x0,
    #[doc = "Enabled. DMA reads the FIFO."]
    VAL1 = 0x01,
}
impl Rxdmaen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxdmaen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxdmaen {
    #[inline(always)]
    fn from(val: u8) -> Rxdmaen {
        Rxdmaen::from_bits(val)
    }
}
impl From<Rxdmaen> for u8 {
    #[inline(always)]
    fn from(val: Rxdmaen) -> u8 {
        Rxdmaen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Samedeviceen {
    #[doc = "In Individual mode, FLSHA1CRx and FLSHA2CRx, FLSHB1CRx and FLSHB2CRx settings are applied to Flash A1, A2, B1, B2 separately. In Parallel mode, FLSHA1CRx register setting is applied to Flash A1 and B1, FLSHA2CRx register setting is applied to Flash A2 and B2. FLSHB1CRx and FLSHB2CRx register settings are ignored."]
    INDIVIDUAL_PARALLEL = 0x0,
    #[doc = "FLSHA1CR0, FLSHA1CR1, and FLSHA1CR2 register settings are applied to Flash A1, A2, B1, B2. FLSHA2CRx, FLSHB1CRx, and FLSHB2CRx settings are ignored."]
    ENABLE = 0x01,
}
impl Samedeviceen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Samedeviceen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Samedeviceen {
    #[inline(always)]
    fn from(val: u8) -> Samedeviceen {
        Samedeviceen::from_bits(val)
    }
}
impl From<Samedeviceen> for u8 {
    #[inline(always)]
    fn from(val: Samedeviceen) -> u8 {
        Samedeviceen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sckbdiffopt {
    #[doc = "Use B_SCLK pad as port B SCLK clock output. Port B flash memory access is available."]
    VAL1 = 0x0,
    #[doc = "Use B_SCLK pad as port A SCLK inverted clock output (Differential clock to A_SCLK). Port B flash memory access is not available."]
    VAL0 = 0x01,
}
impl Sckbdiffopt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sckbdiffopt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sckbdiffopt {
    #[inline(always)]
    fn from(val: u8) -> Sckbdiffopt {
        Sckbdiffopt::from_bits(val)
    }
}
impl From<Sckbdiffopt> for u8 {
    #[inline(always)]
    fn from(val: Sckbdiffopt) -> u8 {
        Sckbdiffopt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sckstopbyrden {
    #[doc = "Disable interrupt or no impact"]
    VALUE0 = 0x0,
    #[doc = "Enable interrupt"]
    VALUE1 = 0x01,
}
impl Sckstopbyrden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sckstopbyrden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sckstopbyrden {
    #[inline(always)]
    fn from(val: u8) -> Sckstopbyrden {
        Sckstopbyrden::from_bits(val)
    }
}
impl From<Sckstopbyrden> for u8 {
    #[inline(always)]
    fn from(val: Sckstopbyrden) -> u8 {
        Sckstopbyrden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sckstopbywren {
    #[doc = "Disable interrupt or no impact"]
    VALUE0 = 0x0,
    #[doc = "Enable interrupt"]
    VALUE1 = 0x01,
}
impl Sckstopbywren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sckstopbywren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sckstopbywren {
    #[inline(always)]
    fn from(val: u8) -> Sckstopbywren {
        Sckstopbywren::from_bits(val)
    }
}
impl From<Sckstopbywren> for u8 {
    #[inline(always)]
    fn from(val: Sckstopbywren) -> u8 {
        Sckstopbywren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Seqidle {
    #[doc = "Not idle"]
    VALUE0 = 0x0,
    #[doc = "Idle"]
    VALUE1 = 0x01,
}
impl Seqidle {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Seqidle {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Seqidle {
    #[inline(always)]
    fn from(val: u8) -> Seqidle {
        Seqidle::from_bits(val)
    }
}
impl From<Seqidle> for u8 {
    #[inline(always)]
    fn from(val: Seqidle) -> u8 {
        Seqidle::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Seqtimeouten {
    #[doc = "Disable interrupt or no impact"]
    VALUE0 = 0x0,
    #[doc = "Enable interrupt"]
    VALUE1 = 0x01,
}
impl Seqtimeouten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Seqtimeouten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Seqtimeouten {
    #[inline(always)]
    fn from(val: u8) -> Seqtimeouten {
        Seqtimeouten::from_bits(val)
    }
}
impl From<Seqtimeouten> for u8 {
    #[inline(always)]
    fn from(val: Seqtimeouten) -> u8 {
        Seqtimeouten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Serclkdiv {
    #[doc = "Divided by 1"]
    VAL0 = 0x0,
    #[doc = "Divided by 2"]
    VAL1 = 0x01,
    #[doc = "Divided by 3"]
    VAL2 = 0x02,
    #[doc = "Divided by 4"]
    VAL3 = 0x03,
    #[doc = "Divided by 5"]
    VAL4 = 0x04,
    #[doc = "Divided by 6"]
    VAL5 = 0x05,
    #[doc = "Divided by 7"]
    VAL6 = 0x06,
    #[doc = "Divided by 8"]
    VAL7 = 0x07,
}
impl Serclkdiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Serclkdiv {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Serclkdiv {
    #[inline(always)]
    fn from(val: u8) -> Serclkdiv {
        Serclkdiv::from_bits(val)
    }
}
impl From<Serclkdiv> for u8 {
    #[inline(always)]
    fn from(val: Serclkdiv) -> u8 {
        Serclkdiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swreset {
    #[doc = "No impact"]
    VAL0 = 0x0,
    #[doc = "Software reset"]
    VAL1 = 0x01,
}
impl Swreset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swreset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swreset {
    #[inline(always)]
    fn from(val: u8) -> Swreset {
        Swreset::from_bits(val)
    }
}
impl From<Swreset> for u8 {
    #[inline(always)]
    fn from(val: Swreset) -> u8 {
        Swreset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trg {
    #[doc = "No action"]
    VALUE0 = 0x0,
    #[doc = "Start the IP command that the IPCR0 and IPCR1 registers define."]
    VALUE1 = 0x01,
}
impl Trg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trg {
    #[inline(always)]
    fn from(val: u8) -> Trg {
        Trg::from_bits(val)
    }
}
impl From<Trg> for u8 {
    #[inline(always)]
    fn from(val: Trg) -> u8 {
        Trg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txdmaen {
    #[doc = "Processor"]
    VAL0 = 0x0,
    #[doc = "DMA"]
    VAL1 = 0x01,
}
impl Txdmaen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txdmaen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txdmaen {
    #[inline(always)]
    fn from(val: u8) -> Txdmaen {
        Txdmaen::from_bits(val)
    }
}
impl From<Txdmaen> for u8 {
    #[inline(always)]
    fn from(val: Txdmaen) -> u8 {
        Txdmaen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Unlock {
    #[doc = "LUT is locked (LUTCR\\[LOCK\\] must be 1)"]
    VALUE0 = 0x0,
    #[doc = "LUT is unlocked and can be written"]
    VALUE1 = 0x01,
}
impl Unlock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Unlock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Unlock {
    #[inline(always)]
    fn from(val: u8) -> Unlock {
        Unlock::from_bits(val)
    }
}
impl From<Unlock> for u8 {
    #[inline(always)]
    fn from(val: Unlock) -> u8 {
        Unlock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wa {
    #[doc = "Byte-addressable"]
    VALUE0 = 0x0,
    #[doc = "Word-addressable"]
    VALUE1 = 0x01,
}
impl Wa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wa {
    #[inline(always)]
    fn from(val: u8) -> Wa {
        Wa::from_bits(val)
    }
}
impl From<Wa> for u8 {
    #[inline(always)]
    fn from(val: Wa) -> u8 {
        Wa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wmena {
    #[doc = "Disabled. When writing to external device, DQS(RWDS) pin is not driven."]
    VAL0 = 0x0,
    #[doc = "Enabled. When writing to external device, FlexSPI drives DQS(RWDS) pin as write mask output."]
    VAL1 = 0x01,
}
impl Wmena {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wmena {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wmena {
    #[inline(always)]
    fn from(val: u8) -> Wmena {
        Wmena::from_bits(val)
    }
}
impl From<Wmena> for u8 {
    #[inline(always)]
    fn from(val: Wmena) -> u8 {
        Wmena::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wmenb {
    #[doc = "Disabled. When writing to external device, DQS(RWDS) pin is not driven."]
    VAL0 = 0x0,
    #[doc = "Enabled. When writing to external device, FlexSPI drives DQS(RWDS) pin as write mask output."]
    VAL1 = 0x01,
}
impl Wmenb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wmenb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wmenb {
    #[inline(always)]
    fn from(val: u8) -> Wmenb {
        Wmenb::from_bits(val)
    }
}
impl From<Wmenb> for u8 {
    #[inline(always)]
    fn from(val: Wmenb) -> u8 {
        Wmenb::to_bits(val)
    }
}
