#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Abtreq {
    #[doc = "No request to abort a command write sequence"]
    ABTREQ0 = 0x0,
    #[doc = "Request to abort a command write sequence"]
    ABTREQ1 = 0x01,
}
impl Abtreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Abtreq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Abtreq {
    #[inline(always)]
    fn from(val: u8) -> Abtreq {
        Abtreq::from_bits(val)
    }
}
impl From<Abtreq> for u8 {
    #[inline(always)]
    fn from(val: Abtreq) -> u8 {
        Abtreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Accerr {
    #[doc = "No access error detected"]
    ACCERR0 = 0x0,
    #[doc = "Access error detected"]
    ACCERR1 = 0x01,
}
impl Accerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Accerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Accerr {
    #[inline(always)]
    fn from(val: u8) -> Accerr {
        Accerr::from_bits(val)
    }
}
impl From<Accerr> for u8 {
    #[inline(always)]
    fn from(val: Accerr) -> u8 {
        Accerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ccie {
    #[doc = "Command complete interrupt disabled"]
    CCIE0 = 0x0,
    #[doc = "Command complete interrupt enabled"]
    CCIE1 = 0x01,
}
impl Ccie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ccie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ccie {
    #[inline(always)]
    fn from(val: u8) -> Ccie {
        Ccie::from_bits(val)
    }
}
impl From<Ccie> for u8 {
    #[inline(always)]
    fn from(val: Ccie) -> u8 {
        Ccie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ccif {
    #[doc = "Flash command, initialization, or power mode recovery in progress"]
    CCIF0 = 0x0,
    #[doc = "Flash command, initialization, or power mode recovery has completed"]
    CCIF1 = 0x01,
}
impl Ccif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ccif {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ccif {
    #[inline(always)]
    fn from(val: u8) -> Ccif {
        Ccif::from_bits(val)
    }
}
impl From<Ccif> for u8 {
    #[inline(always)]
    fn from(val: Ccif) -> u8 {
        Ccif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdabt {
    #[doc = "No command abort detected"]
    CMDABT0 = 0x0,
    #[doc = "Command abort detected"]
    CMDABT1 = 0x01,
}
impl Cmdabt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdabt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdabt {
    #[inline(always)]
    fn from(val: u8) -> Cmdabt {
        Cmdabt::from_bits(val)
    }
}
impl From<Cmdabt> for u8 {
    #[inline(always)]
    fn from(val: Cmdabt) -> u8 {
        Cmdabt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdp {
    #[doc = "Command protection level and domain ID are stale"]
    CMDP0 = 0x0,
    #[doc = "Command protection level (CMDPRT) and domain ID (CMDDID) are set"]
    CMDP1 = 0x01,
}
impl Cmdp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdp {
    #[inline(always)]
    fn from(val: u8) -> Cmdp {
        Cmdp::from_bits(val)
    }
}
impl From<Cmdp> for u8 {
    #[inline(always)]
    fn from(val: Cmdp) -> u8 {
        Cmdp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdprt {
    #[doc = "Secure, normal access"]
    CMDPRT00 = 0x0,
    #[doc = "Secure, privileged access"]
    CMDPRT01 = 0x01,
    #[doc = "Nonsecure, normal access"]
    CMDPRT10 = 0x02,
    #[doc = "Nonsecure, privileged access"]
    CMDPRT11 = 0x03,
}
impl Cmdprt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdprt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdprt {
    #[inline(always)]
    fn from(val: u8) -> Cmdprt {
        Cmdprt::from_bits(val)
    }
}
impl From<Cmdprt> for u8 {
    #[inline(always)]
    fn from(val: Cmdprt) -> u8 {
        Cmdprt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cwsabt {
    #[doc = "Command write sequence not aborted"]
    CWSABT0 = 0x0,
    #[doc = "Command write sequence aborted"]
    CWSABT1 = 0x01,
}
impl Cwsabt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cwsabt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cwsabt {
    #[inline(always)]
    fn from(val: u8) -> Cwsabt {
        Cwsabt::from_bits(val)
    }
}
impl From<Cwsabt> for u8 {
    #[inline(always)]
    fn from(val: Cwsabt) -> u8 {
        Cwsabt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dfdie {
    #[doc = "Double bit fault detect interrupt disabled"]
    DFDIE0 = 0x0,
    #[doc = "Double bit fault detect interrupt enabled"]
    DFDIE1 = 0x01,
}
impl Dfdie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dfdie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dfdie {
    #[inline(always)]
    fn from(val: u8) -> Dfdie {
        Dfdie::from_bits(val)
    }
}
impl From<Dfdie> for u8 {
    #[inline(always)]
    fn from(val: Dfdie) -> u8 {
        Dfdie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dfdif {
    #[doc = "Double bit fault not detected during a valid flash read access"]
    DFDIF0 = 0x0,
    #[doc = "Double bit fault detected (or FCTRL\\[FDFD\\] is set) during a valid flash read access"]
    DFDIF1 = 0x01,
}
impl Dfdif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dfdif {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dfdif {
    #[inline(always)]
    fn from(val: u8) -> Dfdif {
        Dfdif::from_bits(val)
    }
}
impl From<Dfdif> for u8 {
    #[inline(always)]
    fn from(val: Dfdif) -> u8 {
        Dfdif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ersien0 {
    #[doc = "Block 0 IFR Sector X is protected from erase by ERSSCR command"]
    ERSIEN00 = 0x0,
    #[doc = "Block 0 IFR Sector X is not protected from erase by ERSSCR command"]
    ERSIEN01 = 0x01,
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
impl Ersien0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ersien0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ersien0 {
    #[inline(always)]
    fn from(val: u8) -> Ersien0 {
        Ersien0::from_bits(val)
    }
}
impl From<Ersien0> for u8 {
    #[inline(always)]
    fn from(val: Ersien0) -> u8 {
        Ersien0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ersien1 {
    #[doc = "Block 1 IFR Sector X is protected from erase by ERSSCR command"]
    ERSIEN10 = 0x0,
    #[doc = "Block 1 IFR Sector X is not protected from erase by ERSSCR command"]
    ERSIEN11 = 0x01,
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
impl Ersien1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ersien1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ersien1 {
    #[inline(always)]
    fn from(val: u8) -> Ersien1 {
        Ersien1::from_bits(val)
    }
}
impl From<Ersien1> for u8 {
    #[inline(always)]
    fn from(val: Ersien1) -> u8 {
        Ersien1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ersreq {
    #[doc = "No request or request complete"]
    ERSREQ0 = 0x0,
    #[doc = "Request to run the Mass Erase operation"]
    ERSREQ1 = 0x01,
}
impl Ersreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ersreq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ersreq {
    #[inline(always)]
    fn from(val: u8) -> Ersreq {
        Ersreq::from_bits(val)
    }
}
impl From<Ersreq> for u8 {
    #[inline(always)]
    fn from(val: Ersreq) -> u8 {
        Ersreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fail {
    #[doc = "Error not detected"]
    FAIL0 = 0x0,
    #[doc = "Error detected"]
    FAIL1 = 0x01,
}
impl Fail {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fail {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fail {
    #[inline(always)]
    fn from(val: u8) -> Fail {
        Fail::from_bits(val)
    }
}
impl From<Fail> for u8 {
    #[inline(always)]
    fn from(val: Fail) -> u8 {
        Fail::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fdfd {
    #[doc = "FSTAT\\[DFDIF\\] sets only if a double bit fault is detected during a valid flash read access from the platform flash controller"]
    FDFD0 = 0x0,
    #[doc = "FSTAT\\[DFDIF\\] sets during any valid flash read access from the platform flash controller. An interrupt request is generated if the DFDIE bit is set."]
    FDFD1 = 0x01,
}
impl Fdfd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fdfd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fdfd {
    #[inline(always)]
    fn from(val: u8) -> Fdfd {
        Fdfd::from_bits(val)
    }
}
impl From<Fdfd> for u8 {
    #[inline(always)]
    fn from(val: Fdfd) -> u8 {
        Fdfd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lsactive {
    #[doc = "Full speed active mode requested"]
    LSACTIVE0 = 0x0,
    #[doc = "Low speed active mode requested"]
    LSACTIVE1 = 0x01,
}
impl Lsactive {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lsactive {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lsactive {
    #[inline(always)]
    fn from(val: u8) -> Lsactive {
        Lsactive::from_bits(val)
    }
}
impl From<Lsactive> for u8 {
    #[inline(always)]
    fn from(val: Lsactive) -> u8 {
        Lsactive::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Perdy {
    #[doc = "Program or sector erase command operation not stalled"]
    PERDY0 = 0x0,
    #[doc = "Program or sector erase command operation ready to execute"]
    PERDY1 = 0x01,
}
impl Perdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Perdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Perdy {
    #[inline(always)]
    fn from(val: u8) -> Perdy {
        Perdy::from_bits(val)
    }
}
impl From<Perdy> for u8 {
    #[inline(always)]
    fn from(val: Perdy) -> u8 {
        Perdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pewen {
    #[doc = "Writes are not enabled"]
    PEWEN00 = 0x0,
    #[doc = "Writes are enabled for one flash or IFR phrase (phrase programming, sector erase)"]
    PEWEN01 = 0x01,
    #[doc = "Writes are enabled for one flash or IFR page (page programming)"]
    PEWEN10 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Pewen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pewen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pewen {
    #[inline(always)]
    fn from(val: u8) -> Pewen {
        Pewen::from_bits(val)
    }
}
impl From<Pewen> for u8 {
    #[inline(always)]
    fn from(val: Pewen) -> u8 {
        Pewen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pviol {
    #[doc = "No protection violation detected"]
    PVIOL0 = 0x0,
    #[doc = "Protection violation detected"]
    PVIOL1 = 0x01,
}
impl Pviol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pviol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pviol {
    #[inline(always)]
    fn from(val: u8) -> Pviol {
        Pviol::from_bits(val)
    }
}
impl From<Pviol> for u8 {
    #[inline(always)]
    fn from(val: Pviol) -> u8 {
        Pviol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SalvUsed {
    #[doc = "Salvage not used during last operation"]
    SALV_USED0 = 0x0,
    #[doc = "Salvage used during the last erase operation"]
    SALV_USED1 = 0x01,
}
impl SalvUsed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SalvUsed {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SalvUsed {
    #[inline(always)]
    fn from(val: u8) -> SalvUsed {
        SalvUsed::from_bits(val)
    }
}
impl From<SalvUsed> for u8 {
    #[inline(always)]
    fn from(val: SalvUsed) -> u8 {
        SalvUsed::to_bits(val)
    }
}
