#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active {
    #[doc = "eDMA is idle"]
    IDLE = 0x0,
    #[doc = "eDMA is executing a channel"]
    EXECUTION = 0x01,
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
pub enum Cx {
    #[doc = "Normal operation"]
    NORMAL_OPERATION = 0x0,
    #[doc = "Cancel the remaining data transfer"]
    DATA_TRANSFER_CANCEL = 0x01,
}
impl Cx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cx {
    #[inline(always)]
    fn from(val: u8) -> Cx {
        Cx::from_bits(val)
    }
}
impl From<Cx> for u8 {
    #[inline(always)]
    fn from(val: Cx) -> u8 {
        Cx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dae {
    #[doc = "No destination address configuration error"]
    NO_ERROR = 0x0,
    #[doc = "Last recorded error was a configuration error detected in the TCDn_DADDR field"]
    CONFIGURATION_ERROR = 0x01,
}
impl Dae {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dae {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dae {
    #[inline(always)]
    fn from(val: u8) -> Dae {
        Dae::from_bits(val)
    }
}
impl From<Dae> for u8 {
    #[inline(always)]
    fn from(val: Dae) -> u8 {
        Dae::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dbe {
    #[doc = "No destination bus error"]
    NO_ERROR = 0x0,
    #[doc = "Last recorded error was a bus error on a destination write"]
    BUS_ERROR = 0x01,
}
impl Dbe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dbe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dbe {
    #[inline(always)]
    fn from(val: u8) -> Dbe {
        Dbe::from_bits(val)
    }
}
impl From<Dbe> for u8 {
    #[inline(always)]
    fn from(val: Dbe) -> u8 {
        Dbe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Doe {
    #[doc = "No destination offset configuration error"]
    NO_ERROR = 0x0,
    #[doc = "Last recorded error was a configuration error detected in the TCDn_DOFF field"]
    CONFIGURATION_ERROR = 0x01,
}
impl Doe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Doe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Doe {
    #[inline(always)]
    fn from(val: u8) -> Doe {
        Doe::from_bits(val)
    }
}
impl From<Doe> for u8 {
    #[inline(always)]
    fn from(val: Doe) -> u8 {
        Doe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hae {
    #[doc = "Normal operation"]
    NORMAL_OPERATION = 0x0,
    #[doc = "Any error causes the HALT field to be set to 1"]
    HALT = 0x01,
}
impl Hae {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hae {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hae {
    #[inline(always)]
    fn from(val: u8) -> Hae {
        Hae::from_bits(val)
    }
}
impl From<Hae> for u8 {
    #[inline(always)]
    fn from(val: Hae) -> u8 {
        Hae::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Halt {
    #[doc = "Normal operation"]
    NORMAL_OPERATION = 0x0,
    #[doc = "Stall the start of any new channels"]
    STALL = 0x01,
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
pub enum MpCsrEcx {
    #[doc = "Normal operation"]
    NORMAL_OPERATION = 0x0,
    #[doc = "Cancel the remaining data transfer"]
    CANCEL = 0x01,
}
impl MpCsrEcx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MpCsrEcx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MpCsrEcx {
    #[inline(always)]
    fn from(val: u8) -> MpCsrEcx {
        MpCsrEcx::from_bits(val)
    }
}
impl From<MpCsrEcx> for u8 {
    #[inline(always)]
    fn from(val: MpCsrEcx) -> u8 {
        MpCsrEcx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MpEsEcx {
    #[doc = "No canceled transfers"]
    NO_CANCELED_TRANSFERS = 0x0,
    #[doc = "Last recorded entry was a canceled transfer by the error cancel transfer input"]
    CANCELED_TRANSFER = 0x01,
}
impl MpEsEcx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MpEsEcx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MpEsEcx {
    #[inline(always)]
    fn from(val: u8) -> MpEsEcx {
        MpEsEcx::from_bits(val)
    }
}
impl From<MpEsEcx> for u8 {
    #[inline(always)]
    fn from(val: MpEsEcx) -> u8 {
        MpEsEcx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nce {
    #[doc = "No NBYTES/CITER configuration error"]
    NO_ERROR = 0x0,
    #[doc = "The last recorded error was NBYTES equal to zero or a CITER not equal to BITER error"]
    CONFIGURATION_ERROR = 0x01,
}
impl Nce {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nce {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nce {
    #[inline(always)]
    fn from(val: u8) -> Nce {
        Nce::from_bits(val)
    }
}
impl From<Nce> for u8 {
    #[inline(always)]
    fn from(val: Nce) -> u8 {
        Nce::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sae {
    #[doc = "No source address configuration error"]
    NO_ERROR = 0x0,
    #[doc = "Last recorded error was a configuration error detected in the TCDn_SADDR field"]
    CONFIGURATION_ERROR = 0x01,
}
impl Sae {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sae {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sae {
    #[inline(always)]
    fn from(val: u8) -> Sae {
        Sae::from_bits(val)
    }
}
impl From<Sae> for u8 {
    #[inline(always)]
    fn from(val: Sae) -> u8 {
        Sae::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sbe {
    #[doc = "No source bus error"]
    NO_ERROR = 0x0,
    #[doc = "Last recorded error was a bus error on a source read"]
    BUS_ERROR = 0x01,
}
impl Sbe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sbe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sbe {
    #[inline(always)]
    fn from(val: u8) -> Sbe {
        Sbe::from_bits(val)
    }
}
impl From<Sbe> for u8 {
    #[inline(always)]
    fn from(val: Sbe) -> u8 {
        Sbe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sge {
    #[doc = "No scatter/gather configuration error"]
    NO_ERROR = 0x0,
    #[doc = "Last recorded error was a configuration error detected in the TCDn_DLAST_SGA field"]
    CONFIGURATION_ERROR = 0x01,
}
impl Sge {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sge {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sge {
    #[inline(always)]
    fn from(val: u8) -> Sge {
        Sge::from_bits(val)
    }
}
impl From<Sge> for u8 {
    #[inline(always)]
    fn from(val: Sge) -> u8 {
        Sge::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Soe {
    #[doc = "No source offset configuration error"]
    NO_ERROR = 0x0,
    #[doc = "Last recorded error was a configuration error detected in the TCDn_SOFF field"]
    CONFIGURATION_ERROR = 0x01,
}
impl Soe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Soe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Soe {
    #[inline(always)]
    fn from(val: u8) -> Soe {
        Soe::from_bits(val)
    }
}
impl From<Soe> for u8 {
    #[inline(always)]
    fn from(val: Soe) -> u8 {
        Soe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vld {
    #[doc = "No CHn_ES\\[ERR\\] fields are set to 1"]
    NO_FIELD_SET_ONE = 0x0,
    #[doc = "At least one CHn_ES\\[ERR\\] field is set to 1, indicating a valid error exists that software has not cleared"]
    ATLEAST_ONE_FIELD = 0x01,
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
