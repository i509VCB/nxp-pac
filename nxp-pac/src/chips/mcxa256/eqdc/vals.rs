#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp0ie {
    #[doc = "Disabled"]
    CMP0IE0 = 0x0,
    #[doc = "Enabled"]
    CMP0IE1 = 0x01,
}
impl Cmp0ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp0ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp0ie {
    #[inline(always)]
    fn from(val: u8) -> Cmp0ie {
        Cmp0ie::from_bits(val)
    }
}
impl From<Cmp0ie> for u8 {
    #[inline(always)]
    fn from(val: Cmp0ie) -> u8 {
        Cmp0ie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp0irq {
    #[doc = "No match has occurred (the position counter does not match the COMP0 value)"]
    CMP0IRQ0 = 0x0,
    #[doc = "COMP match has occurred (the position counter matches the COMP0 value)"]
    CMP0IRQ1 = 0x01,
}
impl Cmp0irq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp0irq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp0irq {
    #[inline(always)]
    fn from(val: u8) -> Cmp0irq {
        Cmp0irq::from_bits(val)
    }
}
impl From<Cmp0irq> for u8 {
    #[inline(always)]
    fn from(val: Cmp0irq) -> u8 {
        Cmp0irq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp1f {
    #[doc = "When the position counter is less than value of COMP1 register"]
    CMP1F0 = 0x0,
    #[doc = "When the position counter is greater or equal than value of COMP1 register"]
    CMP1F1 = 0x01,
}
impl Cmp1f {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp1f {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp1f {
    #[inline(always)]
    fn from(val: u8) -> Cmp1f {
        Cmp1f::from_bits(val)
    }
}
impl From<Cmp1f> for u8 {
    #[inline(always)]
    fn from(val: Cmp1f) -> u8 {
        Cmp1f::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp1ie {
    #[doc = "Disabled"]
    CMP1IE0 = 0x0,
    #[doc = "Enabled"]
    CMP1IE1 = 0x01,
}
impl Cmp1ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp1ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp1ie {
    #[inline(always)]
    fn from(val: u8) -> Cmp1ie {
        Cmp1ie::from_bits(val)
    }
}
impl From<Cmp1ie> for u8 {
    #[inline(always)]
    fn from(val: Cmp1ie) -> u8 {
        Cmp1ie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp1irq {
    #[doc = "No match has occurred (the position counter does not match the COMP1 value)"]
    CMP1IRQ0 = 0x0,
    #[doc = "COMP1 match has occurred (the position counter matches the COMP1 value)"]
    CMP1IRQ1 = 0x01,
}
impl Cmp1irq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp1irq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp1irq {
    #[inline(always)]
    fn from(val: u8) -> Cmp1irq {
        Cmp1irq::from_bits(val)
    }
}
impl From<Cmp1irq> for u8 {
    #[inline(always)]
    fn from(val: Cmp1irq) -> u8 {
        Cmp1irq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp2f {
    #[doc = "When the position counter is less than value of COMP2 register"]
    CMP2F0 = 0x0,
    #[doc = "When the position counter is greater or equal than value of COMP2 register"]
    CMP2F1 = 0x01,
}
impl Cmp2f {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp2f {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp2f {
    #[inline(always)]
    fn from(val: u8) -> Cmp2f {
        Cmp2f::from_bits(val)
    }
}
impl From<Cmp2f> for u8 {
    #[inline(always)]
    fn from(val: Cmp2f) -> u8 {
        Cmp2f::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp2ie {
    #[doc = "Disabled"]
    CMP2IE0 = 0x0,
    #[doc = "Enabled"]
    CMP2IE1 = 0x01,
}
impl Cmp2ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp2ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp2ie {
    #[inline(always)]
    fn from(val: u8) -> Cmp2ie {
        Cmp2ie::from_bits(val)
    }
}
impl From<Cmp2ie> for u8 {
    #[inline(always)]
    fn from(val: Cmp2ie) -> u8 {
        Cmp2ie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp2irq {
    #[doc = "No match has occurred (the position counter does not match the COMP2 value)"]
    CMP2IRQ0 = 0x0,
    #[doc = "COMP2 match has occurred (the position counter matches the COMP2 value)"]
    CMP2IRQ1 = 0x01,
}
impl Cmp2irq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp2irq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp2irq {
    #[inline(always)]
    fn from(val: u8) -> Cmp2irq {
        Cmp2irq::from_bits(val)
    }
}
impl From<Cmp2irq> for u8 {
    #[inline(always)]
    fn from(val: Cmp2irq) -> u8 {
        Cmp2irq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp3f {
    #[doc = "When the position counter value is less than value of COMP3 register"]
    CMP3F0 = 0x0,
    #[doc = "When the position counter is greater or equal than value of COMP3 register"]
    CMP3F1 = 0x01,
}
impl Cmp3f {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp3f {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp3f {
    #[inline(always)]
    fn from(val: u8) -> Cmp3f {
        Cmp3f::from_bits(val)
    }
}
impl From<Cmp3f> for u8 {
    #[inline(always)]
    fn from(val: Cmp3f) -> u8 {
        Cmp3f::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp3ie {
    #[doc = "Disabled"]
    CMP3IE0 = 0x0,
    #[doc = "Enabled"]
    CMP3IE1 = 0x01,
}
impl Cmp3ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp3ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp3ie {
    #[inline(always)]
    fn from(val: u8) -> Cmp3ie {
        Cmp3ie::from_bits(val)
    }
}
impl From<Cmp3ie> for u8 {
    #[inline(always)]
    fn from(val: Cmp3ie) -> u8 {
        Cmp3ie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp3irq {
    #[doc = "No match has occurred (the position counter does not match the COMP3 value)"]
    CMP3IRQ0 = 0x0,
    #[doc = "COMP3 match has occurred (the position counter matches the COMP3 value)"]
    CMP3IRQ1 = 0x01,
}
impl Cmp3irq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp3irq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp3irq {
    #[inline(always)]
    fn from(val: u8) -> Cmp3irq {
        Cmp3irq::from_bits(val)
    }
}
impl From<Cmp3irq> for u8 {
    #[inline(always)]
    fn from(val: Cmp3irq) -> u8 {
        Cmp3irq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmpf0 {
    #[doc = "When the position counter is less than value of COMP0 register"]
    CMPF00 = 0x0,
    #[doc = "When the position counter is greater or equal than value of COMP0 register"]
    CMPF01 = 0x01,
}
impl Cmpf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmpf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmpf0 {
    #[inline(always)]
    fn from(val: u8) -> Cmpf0 {
        Cmpf0::from_bits(val)
    }
}
impl From<Cmpf0> for u8 {
    #[inline(always)]
    fn from(val: Cmpf0) -> u8 {
        Cmpf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dir {
    #[doc = "Current count was in the down direction"]
    DIR0 = 0x0,
    #[doc = "Current count was in the up direction"]
    DIR1 = 0x01,
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
pub enum Dirie {
    #[doc = "Disabled"]
    DIRIE0 = 0x0,
    #[doc = "Enabled"]
    DIRIE1 = 0x01,
}
impl Dirie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dirie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dirie {
    #[inline(always)]
    fn from(val: u8) -> Dirie {
        Dirie::from_bits(val)
    }
}
impl From<Dirie> for u8 {
    #[inline(always)]
    fn from(val: Dirie) -> u8 {
        Dirie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dirirq {
    #[doc = "Count direction unchanged"]
    DIRIRQ0 = 0x0,
    #[doc = "Count direction changed"]
    DIRIRQ1 = 0x01,
}
impl Dirirq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dirirq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dirirq {
    #[inline(always)]
    fn from(val: u8) -> Dirirq {
        Dirirq::from_bits(val)
    }
}
impl From<Dirirq> for u8 {
    #[inline(always)]
    fn from(val: Dirirq) -> u8 {
        Dirirq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmaen {
    #[doc = "DMA is disabled"]
    DMAEN_0 = 0x0,
    #[doc = "DMA is enabled. DMA request asserts automatically when the values in the outer-set of buffered compare registers (UCOMP0/LCOMP0;UCOMP1/LCOMP1;UCOMP2/LCOMP2;UCOMP3/LCOMP3), initial registers(UINIT/LINIT) and modulus registers (UMOD/LMOD) are loaded into the inner-set of buffer and then LDOK is cleared automatically. After the completion of this DMA transfer, LDOK is set automatically, it ensures outer-set values can be loaded into inner-set which in turn triggers DMA again."]
    DMAEN_1 = 0x01,
}
impl Dmaen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmaen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmaen {
    #[inline(always)]
    fn from(val: u8) -> Dmaen {
        Dmaen::from_bits(val)
    }
}
impl From<Dmaen> for u8 {
    #[inline(always)]
    fn from(val: Dmaen) -> u8 {
        Dmaen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Emip {
    #[doc = "disables the position counter to be initialized by Index Event Edge Mark"]
    EMIP0 = 0x0,
    #[doc = "enables the position counter to be initialized by Index Event Edge Mark."]
    EMIP1 = 0x01,
}
impl Emip {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emip {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emip {
    #[inline(always)]
    fn from(val: u8) -> Emip {
        Emip::from_bits(val)
    }
}
impl From<Emip> for u8 {
    #[inline(always)]
    fn from(val: Emip) -> u8 {
        Emip::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FiltCs {
    #[doc = "Peripheral Clock"]
    FILT_CS0 = 0x0,
    #[doc = "Prescaled peripheral clock by PRSC"]
    FILT_CS1 = 0x01,
}
impl FiltCs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FiltCs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FiltCs {
    #[inline(always)]
    fn from(val: u8) -> FiltCs {
        FiltCs::from_bits(val)
    }
}
impl From<FiltCs> for u8 {
    #[inline(always)]
    fn from(val: FiltCs) -> u8 {
        FiltCs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hie {
    #[doc = "Disabled"]
    HIE0 = 0x0,
    #[doc = "Enabled"]
    HIE1 = 0x01,
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
    HIP0 = 0x0,
    #[doc = "HOME signal initializes the position counter"]
    HIP1 = 0x01,
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
    #[doc = "No transition on the HOME/ENABLE signal has occurred"]
    HIRQ0 = 0x0,
    #[doc = "A transition on the HOME/ENABLE signal has occurred"]
    HIRQ1 = 0x01,
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
    #[doc = "When CTRL\\[OPMODE\\] = 0,use HOME positive edge to trigger initialization of position counters. When CTRL\\[OPMODE\\] = 1,use ENABLE high level to enable POS/POSD/WDG/REV counters"]
    HNE0 = 0x0,
    #[doc = "When CTRL\\[OPMODE\\] = 0,use HOME negative edge to trigger initialization of position counters. When CTRL\\[OPMODE\\] = 1,use ENABLE low level to enable POS/POSD/WDG/REV counters"]
    HNE1 = 0x01,
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
pub enum Initpos {
    #[doc = "Don't initialize position counter on rising edge of TRIGGER"]
    INITPOS0 = 0x0,
    #[doc = "Initialize position counter on rising edge of TRIGGER"]
    INITPOS1 = 0x01,
}
impl Initpos {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Initpos {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Initpos {
    #[inline(always)]
    fn from(val: u8) -> Initpos {
        Initpos::from_bits(val)
    }
}
impl From<Initpos> for u8 {
    #[inline(always)]
    fn from(val: Initpos) -> u8 {
        Initpos::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ldmod {
    #[doc = "Buffered registers are loaded and take effect immediately upon CTRL\\[LDOK\\] is set."]
    LDMOD0 = 0x0,
    #[doc = "Buffered registers are loaded and take effect at the next roll-over or roll-under if CTRL\\[LDOK\\] is set."]
    LDMOD1 = 0x01,
}
impl Ldmod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ldmod {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ldmod {
    #[inline(always)]
    fn from(val: u8) -> Ldmod {
        Ldmod::from_bits(val)
    }
}
impl From<Ldmod> for u8 {
    #[inline(always)]
    fn from(val: Ldmod) -> u8 {
        Ldmod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ldok {
    #[doc = "No loading action taken. Users can write new values to buffered registers (writing into outer-set of these buffered registers)"]
    LDOK0 = 0x0,
    #[doc = "Outer-set values are ready to be loaded into inner-set and take effect. The loading time point depends on CTRL2\\[LDMOD\\]."]
    LDOK1 = 0x01,
}
impl Ldok {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ldok {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ldok {
    #[inline(always)]
    fn from(val: u8) -> Ldok {
        Ldok::from_bits(val)
    }
}
impl From<Ldok> for u8 {
    #[inline(always)]
    fn from(val: Ldok) -> u8 {
        Ldok::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Once {
    #[doc = "Position counter counts repeatedly"]
    ONCE0 = 0x0,
    #[doc = "Position counter counts until roll-over or roll-under, then stop."]
    ONCE1 = 0x01,
}
impl Once {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Once {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Once {
    #[inline(always)]
    fn from(val: u8) -> Once {
        Once::from_bits(val)
    }
}
impl From<Once> for u8 {
    #[inline(always)]
    fn from(val: Once) -> u8 {
        Once::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Opmode {
    #[doc = "Decode Mode: Input nodes INDEX/PRESET and HOME/ENABLE are assigned to function of INDEX and HOME."]
    OPMODE0 = 0x0,
    #[doc = "Count Mode: Input nodes INDEX/PRESET and HOME/ENABLE are assigned to functions of PRESET and ENABLE. In this mode: (1)only when ENABLE=1, all counters (position/position difference/revolution/watchdog) can run, when ENABLE=0, all counters (position/position difference/revolution/watchdog) can't run. (2) the rising edge of PRESET input can initialize position/revolution/watchdog counters (position counter initialization also need referring to bit CTRL\\[REV\\])."]
    OPMODE1 = 0x01,
}
impl Opmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Opmode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Opmode {
    #[inline(always)]
    fn from(val: u8) -> Opmode {
        Opmode::from_bits(val)
    }
}
impl From<Opmode> for u8 {
    #[inline(always)]
    fn from(val: Opmode) -> u8 {
        Opmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Outctl {
    #[doc = "POS_MATCH\\[x\\](x range is 0-3) is asserted when the Position Counter is equal to according compare value (UCOMPx/LCOMPx)(x range is 0-3), and de-asserted when the Position Counter not equal to the compare value (UCOMPx/LCOMPx)(x range is 0-3)"]
    OUTCTL0 = 0x0,
    #[doc = "All POS_MATCH\\[x\\](x range is 0-3) are asserted a pulse, when the UPOS, LPOS, REV, or POSD registers are read"]
    OUTCTL1 = 0x01,
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
    #[doc = "Standard quadrature decoder, where PHASEA and PHASEB represent a two-phase quadrature signal."]
    PH10 = 0x0,
    #[doc = "Single phase mode, bypass the quadrature decoder, refer to CTRL2\\[CMODE\\] description"]
    PH11 = 0x01,
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
pub enum Pmen {
    #[doc = "Period measurement functions are not used. POSD is loaded to POSDH and then cleared whenever POSD, UPOS, LPOS or REV is read."]
    PMEN0 = 0x0,
    #[doc = "Period measurement functions are used. POSD is loaded into POSDH and then cleared only when POSD is read."]
    PMEN1 = 0x01,
}
impl Pmen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pmen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pmen {
    #[inline(always)]
    fn from(val: u8) -> Pmen {
        Pmen::from_bits(val)
    }
}
impl From<Pmen> for u8 {
    #[inline(always)]
    fn from(val: Pmen) -> u8 {
        Pmen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qdn {
    #[doc = "Generates a positive quadrature decoder signal"]
    QDN0 = 0x0,
    #[doc = "Generates a negative quadrature decoder signal"]
    QDN1 = 0x01,
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
    #[doc = "Count normally and the position counter initialization uses upper/lower initialization register UINIT/LINIT"]
    REV0 = 0x0,
    #[doc = "Count in the reverse direction and the position counter initialization uses upper/lower modulus register UMOD/LMOD"]
    REV1 = 0x01,
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
    REVMOD0 = 0x0,
    #[doc = "Use modulus counting roll-over/under to increment/decrement revolution counter (REV)"]
    REVMOD1 = 0x01,
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
    ROIE = 0x0,
    #[doc = "Enabled"]
    ROIE1 = 0x01,
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
    ROIRQ0 = 0x0,
    #[doc = "Roll-over has occurred"]
    ROIRQ1 = 0x01,
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
    RUIE0 = 0x0,
    #[doc = "Enabled"]
    RUIE1 = 0x01,
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
    RUIRQ0 = 0x0,
    #[doc = "Roll-under has occurred"]
    RUIRQ1 = 0x01,
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
pub enum Sabie {
    #[doc = "Disabled"]
    SABIE0 = 0x0,
    #[doc = "Enabled"]
    SABIE1 = 0x01,
}
impl Sabie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sabie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sabie {
    #[inline(always)]
    fn from(val: u8) -> Sabie {
        Sabie::from_bits(val)
    }
}
impl From<Sabie> for u8 {
    #[inline(always)]
    fn from(val: Sabie) -> u8 {
        Sabie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sabirq {
    #[doc = "No simultaneous change of PHASEA and PHASEB has occurred"]
    SABIRQ0 = 0x0,
    #[doc = "A simultaneous change of PHASEA and PHASEB has occurred"]
    SABIRQ1 = 0x01,
}
impl Sabirq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sabirq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sabirq {
    #[inline(always)]
    fn from(val: u8) -> Sabirq {
        Sabirq::from_bits(val)
    }
}
impl From<Sabirq> for u8 {
    #[inline(always)]
    fn from(val: Sabirq) -> u8 {
        Sabirq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swip {
    #[doc = "No action"]
    SWIP0 = 0x0,
    #[doc = "Initialize position counter"]
    SWIP1 = 0x01,
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
    TCE0 = 0x0,
    #[doc = "Enabled"]
    TCE1 = 0x01,
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
    TEN0 = 0x0,
    #[doc = "Enabled"]
    TEN1 = 0x01,
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
pub enum Wde {
    #[doc = "Disabled"]
    WDE0 = 0x0,
    #[doc = "Enabled"]
    WDE1 = 0x01,
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
pub enum Wdie {
    #[doc = "Disabled"]
    WDIE0 = 0x0,
    #[doc = "Enabled"]
    WDIE1 = 0x01,
}
impl Wdie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdie {
    #[inline(always)]
    fn from(val: u8) -> Wdie {
        Wdie::from_bits(val)
    }
}
impl From<Wdie> for u8 {
    #[inline(always)]
    fn from(val: Wdie) -> u8 {
        Wdie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdirq {
    #[doc = "No Watchdog timeout interrupt has occurred"]
    WDIRQ0 = 0x0,
    #[doc = "Watchdog timeout interrupt has occurred"]
    WDIRQ1 = 0x01,
}
impl Wdirq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdirq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdirq {
    #[inline(always)]
    fn from(val: u8) -> Wdirq {
        Wdirq::from_bits(val)
    }
}
impl From<Wdirq> for u8 {
    #[inline(always)]
    fn from(val: Wdirq) -> u8 {
        Wdirq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xie {
    #[doc = "Disabled"]
    XIE0 = 0x0,
    #[doc = "Enabled"]
    XIE1 = 0x01,
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
    XIP0 = 0x0,
    #[doc = "INDEX pulse initializes the position counter"]
    XIP1 = 0x01,
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
    #[doc = "INDEX/PRESET pulse has not occurred"]
    XIRQ0 = 0x0,
    #[doc = "INDEX/PRESET pulse has occurred"]
    XIRQ1 = 0x01,
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
    #[doc = "Use positive edge of INDEX/PRESET pulse"]
    XNE0 = 0x0,
    #[doc = "Use negative edge of INDEX/PRESET pulse"]
    XNE1 = 0x01,
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
