#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpie {
    #[doc = "Disabled"]
    CMPIE_0 = 0x0,
    #[doc = "Enabled"]
    CMPIE_1 = 0x01,
}
impl Cmpie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpie {
    #[inline(always)]
    fn from(val: u8) -> Cmpie {
        Cmpie::from_bits(val)
    }
}
impl From<Cmpie> for u8 {
    #[inline(always)]
    fn from(val: Cmpie) -> u8 {
        Cmpie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpirq {
    #[doc = "No match has occurred (the counter does not match the COMP value)"]
    CMPIRQ_0 = 0x0,
    #[doc = "COMP match has occurred (the counter matches the COMP value)"]
    CMPIRQ_1 = 0x01,
}
impl Cmpirq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpirq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpirq {
    #[inline(always)]
    fn from(val: u8) -> Cmpirq {
        Cmpirq::from_bits(val)
    }
}
impl From<Cmpirq> for u8 {
    #[inline(always)]
    fn from(val: Cmpirq) -> u8 {
        Cmpirq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Die {
    #[doc = "Disabled"]
    DIE_0 = 0x0,
    #[doc = "Enabled"]
    DIE_1 = 0x01,
}
impl Die {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Die {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Die {
    #[inline(always)]
    fn from(val: u8) -> Die {
        Die::from_bits(val)
    }
}
impl From<Die> for u8 {
    #[inline(always)]
    fn from(val: Die) -> u8 {
        Die::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dir {
    #[doc = "Last count was in the down direction"]
    DIR_0 = 0x0,
    #[doc = "Last count was in the up direction"]
    DIR_1 = 0x01,
}
impl Dir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dir {
    #[inline(always)]
    fn from(val: u8) -> Dir {
        Dir::from_bits(val)
    }
}
impl From<Dir> for u8 {
    #[inline(always)]
    fn from(val: Dir) -> u8 {
        Dir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dirq {
    #[doc = "No Watchdog timeout interrupt has occurred"]
    DIRQ_0 = 0x0,
    #[doc = "Watchdog timeout interrupt has occurred"]
    DIRQ_1 = 0x01,
}
impl Dirq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dirq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dirq {
    #[inline(always)]
    fn from(val: u8) -> Dirq {
        Dirq::from_bits(val)
    }
}
impl From<Dirq> for u8 {
    #[inline(always)]
    fn from(val: Dirq) -> u8 {
        Dirq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hie {
    #[doc = "Disabled"]
    HIE_0 = 0x0,
    #[doc = "Enabled"]
    HIE_1 = 0x01,
}
impl Hie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hie {
    #[inline(always)]
    fn from(val: u8) -> Hie {
        Hie::from_bits(val)
    }
}
impl From<Hie> for u8 {
    #[inline(always)]
    fn from(val: Hie) -> u8 {
        Hie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hip {
    #[doc = "No action"]
    HIP_0 = 0x0,
    #[doc = "HOME signal initializes the position counter"]
    HIP_1 = 0x01,
}
impl Hip {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hip {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hip {
    #[inline(always)]
    fn from(val: u8) -> Hip {
        Hip::from_bits(val)
    }
}
impl From<Hip> for u8 {
    #[inline(always)]
    fn from(val: Hip) -> u8 {
        Hip::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hirq {
    #[doc = "No transition on the HOME signal has occurred"]
    HIRQ_0 = 0x0,
    #[doc = "A transition on the HOME signal has occurred"]
    HIRQ_1 = 0x01,
}
impl Hirq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hirq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hirq {
    #[inline(always)]
    fn from(val: u8) -> Hirq {
        Hirq::from_bits(val)
    }
}
impl From<Hirq> for u8 {
    #[inline(always)]
    fn from(val: Hirq) -> u8 {
        Hirq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hne {
    #[doc = "Use positive-going edge-to-trigger initialization of position counters UPOS and LPOS"]
    HNE_0 = 0x0,
    #[doc = "Use negative-going edge-to-trigger initialization of position counters UPOS and LPOS"]
    HNE_1 = 0x01,
}
impl Hne {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hne {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hne {
    #[inline(always)]
    fn from(val: u8) -> Hne {
        Hne::from_bits(val)
    }
}
impl From<Hne> for u8 {
    #[inline(always)]
    fn from(val: Hne) -> u8 {
        Hne::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mod {
    #[doc = "Disable modulo counting"]
    MOD_0 = 0x0,
    #[doc = "Enable modulo counting"]
    MOD_1 = 0x01,
}
impl Mod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mod {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mod {
    #[inline(always)]
    fn from(val: u8) -> Mod {
        Mod::from_bits(val)
    }
}
impl From<Mod> for u8 {
    #[inline(always)]
    fn from(val: Mod) -> u8 {
        Mod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Outctl {
    #[doc = "POSMATCH pulses when a match occurs between the position counters (POS) and the corresponding compare value (COMP )"]
    OUTCTL_0 = 0x0,
    #[doc = "POSMATCH pulses when the UPOS, LPOS, REV, or POSD registers are read"]
    OUTCTL_1 = 0x01,
}
impl Outctl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Outctl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Outctl {
    #[inline(always)]
    fn from(val: u8) -> Outctl {
        Outctl::from_bits(val)
    }
}
impl From<Outctl> for u8 {
    #[inline(always)]
    fn from(val: Outctl) -> u8 {
        Outctl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ph1 {
    #[doc = "Use the standard quadrature decoder, where PHASEA and PHASEB represent a two-phase quadrature signal."]
    PH1_0 = 0x0,
    #[doc = "Bypass the quadrature decoder. A positive transition of the PHASEA input generates a count signal. The PHASEB input and the REV bit control the counter direction: If CTRL\\[REV\\] = 0, PHASEB = 0, then count up If CTRL\\[REV\\] = 1, PHASEB = 1, then count up If CTRL\\[REV\\] = 0, PHASEB = 1, then count down If CTRL\\[REV\\] = 1, PHASEB = 0, then count down"]
    PH1_1 = 0x01,
}
impl Ph1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ph1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ph1 {
    #[inline(always)]
    fn from(val: u8) -> Ph1 {
        Ph1::from_bits(val)
    }
}
impl From<Ph1> for u8 {
    #[inline(always)]
    fn from(val: Ph1) -> u8 {
        Ph1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qdn {
    #[doc = "Generates a positive quadrature decoder signal"]
    QDN_0 = 0x0,
    #[doc = "Generates a negative quadrature decoder signal"]
    QDN_1 = 0x01,
}
impl Qdn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qdn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qdn {
    #[inline(always)]
    fn from(val: u8) -> Qdn {
        Qdn::from_bits(val)
    }
}
impl From<Qdn> for u8 {
    #[inline(always)]
    fn from(val: Qdn) -> u8 {
        Qdn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rev {
    #[doc = "Count normally"]
    REV_0 = 0x0,
    #[doc = "Count in the reverse direction"]
    REV_1 = 0x01,
}
impl Rev {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rev {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rev {
    #[inline(always)]
    fn from(val: u8) -> Rev {
        Rev::from_bits(val)
    }
}
impl From<Rev> for u8 {
    #[inline(always)]
    fn from(val: Rev) -> u8 {
        Rev::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Revmod {
    #[doc = "Use INDEX pulse to increment/decrement revolution counter (REV)"]
    REVMOD_0 = 0x0,
    #[doc = "Use modulus counting roll-over/under to increment/decrement revolution counter (REV)"]
    REVMOD_1 = 0x01,
}
impl Revmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Revmod {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Revmod {
    #[inline(always)]
    fn from(val: u8) -> Revmod {
        Revmod::from_bits(val)
    }
}
impl From<Revmod> for u8 {
    #[inline(always)]
    fn from(val: Revmod) -> u8 {
        Revmod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Roie {
    #[doc = "Disabled"]
    ROIE_0 = 0x0,
    #[doc = "Enabled"]
    ROIE_1 = 0x01,
}
impl Roie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Roie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Roie {
    #[inline(always)]
    fn from(val: u8) -> Roie {
        Roie::from_bits(val)
    }
}
impl From<Roie> for u8 {
    #[inline(always)]
    fn from(val: Roie) -> u8 {
        Roie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Roirq {
    #[doc = "No roll-over has occurred"]
    ROIRQ_0 = 0x0,
    #[doc = "Roll-over has occurred"]
    ROIRQ_1 = 0x01,
}
impl Roirq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Roirq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Roirq {
    #[inline(always)]
    fn from(val: u8) -> Roirq {
        Roirq::from_bits(val)
    }
}
impl From<Roirq> for u8 {
    #[inline(always)]
    fn from(val: Roirq) -> u8 {
        Roirq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ruie {
    #[doc = "Disabled"]
    RUIE_0 = 0x0,
    #[doc = "Enabled"]
    RUIE_1 = 0x01,
}
impl Ruie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ruie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ruie {
    #[inline(always)]
    fn from(val: u8) -> Ruie {
        Ruie::from_bits(val)
    }
}
impl From<Ruie> for u8 {
    #[inline(always)]
    fn from(val: Ruie) -> u8 {
        Ruie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ruirq {
    #[doc = "No roll-under has occurred"]
    RUIRQ_0 = 0x0,
    #[doc = "Roll-under has occurred"]
    RUIRQ_1 = 0x01,
}
impl Ruirq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ruirq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ruirq {
    #[inline(always)]
    fn from(val: u8) -> Ruirq {
        Ruirq::from_bits(val)
    }
}
impl From<Ruirq> for u8 {
    #[inline(always)]
    fn from(val: Ruirq) -> u8 {
        Ruirq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swip {
    #[doc = "No action"]
    SWIP_0 = 0x0,
    #[doc = "Initialize position counter (using upper and lower initialization registers, UINIT and LINIT)"]
    SWIP_1 = 0x01,
}
impl Swip {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swip {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swip {
    #[inline(always)]
    fn from(val: u8) -> Swip {
        Swip::from_bits(val)
    }
}
impl From<Swip> for u8 {
    #[inline(always)]
    fn from(val: Swip) -> u8 {
        Swip::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tce {
    #[doc = "Disabled"]
    TCE_0 = 0x0,
    #[doc = "Enabled"]
    TCE_1 = 0x01,
}
impl Tce {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tce {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tce {
    #[inline(always)]
    fn from(val: u8) -> Tce {
        Tce::from_bits(val)
    }
}
impl From<Tce> for u8 {
    #[inline(always)]
    fn from(val: Tce) -> u8 {
        Tce::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ten {
    #[doc = "Disabled"]
    TEN_0 = 0x0,
    #[doc = "Enabled"]
    TEN_1 = 0x01,
}
impl Ten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ten {
    #[inline(always)]
    fn from(val: u8) -> Ten {
        Ten::from_bits(val)
    }
}
impl From<Ten> for u8 {
    #[inline(always)]
    fn from(val: Ten) -> u8 {
        Ten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Updhld {
    #[doc = "Disable updates of hold registers on the rising edge of TRIGGER input signal"]
    UPDHLD_0 = 0x0,
    #[doc = "Enable updates of hold registers on the rising edge of TRIGGER input signal"]
    UPDHLD_1 = 0x01,
}
impl Updhld {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Updhld {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Updhld {
    #[inline(always)]
    fn from(val: u8) -> Updhld {
        Updhld::from_bits(val)
    }
}
impl From<Updhld> for u8 {
    #[inline(always)]
    fn from(val: Updhld) -> u8 {
        Updhld::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Updpos {
    #[doc = "No action for POSD, REV, UPOS and LPOS registers on rising edge of TRIGGER"]
    UPDPOS_0 = 0x0,
    #[doc = "Clear POSD, REV, UPOS and LPOS registers on rising edge of TRIGGER"]
    UPDPOS_1 = 0x01,
}
impl Updpos {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Updpos {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Updpos {
    #[inline(always)]
    fn from(val: u8) -> Updpos {
        Updpos::from_bits(val)
    }
}
impl From<Updpos> for u8 {
    #[inline(always)]
    fn from(val: Updpos) -> u8 {
        Updpos::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wde {
    #[doc = "Disabled"]
    WDE_0 = 0x0,
    #[doc = "Enabled"]
    WDE_1 = 0x01,
}
impl Wde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wde {
    #[inline(always)]
    fn from(val: u8) -> Wde {
        Wde::from_bits(val)
    }
}
impl From<Wde> for u8 {
    #[inline(always)]
    fn from(val: Wde) -> u8 {
        Wde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xie {
    #[doc = "Disabled"]
    XIE_0 = 0x0,
    #[doc = "Enabled"]
    XIE_1 = 0x01,
}
impl Xie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xie {
    #[inline(always)]
    fn from(val: u8) -> Xie {
        Xie::from_bits(val)
    }
}
impl From<Xie> for u8 {
    #[inline(always)]
    fn from(val: Xie) -> u8 {
        Xie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xip {
    #[doc = "INDEX pulse does not initialize the position counter"]
    XIP_0 = 0x0,
    #[doc = "INDEX pulse initializes the position counter"]
    XIP_1 = 0x01,
}
impl Xip {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xip {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xip {
    #[inline(always)]
    fn from(val: u8) -> Xip {
        Xip::from_bits(val)
    }
}
impl From<Xip> for u8 {
    #[inline(always)]
    fn from(val: Xip) -> u8 {
        Xip::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xirq {
    #[doc = "INDEX pulse has not occurred"]
    XIRQ_0 = 0x0,
    #[doc = "INDEX pulse has occurred"]
    XIRQ_1 = 0x01,
}
impl Xirq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xirq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xirq {
    #[inline(always)]
    fn from(val: u8) -> Xirq {
        Xirq::from_bits(val)
    }
}
impl From<Xirq> for u8 {
    #[inline(always)]
    fn from(val: Xirq) -> u8 {
        Xirq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xne {
    #[doc = "Use positive edge of INDEX pulse"]
    XNE_0 = 0x0,
    #[doc = "Use negative edge of INDEX pulse"]
    XNE_1 = 0x01,
}
impl Xne {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xne {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xne {
    #[inline(always)]
    fn from(val: u8) -> Xne {
        Xne::from_bits(val)
    }
}
impl From<Xne> for u8 {
    #[inline(always)]
    fn from(val: Xne) -> u8 {
        Xne::to_bits(val)
    }
}
