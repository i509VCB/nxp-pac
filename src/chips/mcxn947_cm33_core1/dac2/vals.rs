#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BufEn {
    #[doc = "Not used"]
    USE_BUF = 0x0,
    #[doc = "Used"]
    NO_USE_BUF = 0x01,
}
impl BufEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BufEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BufEn {
    #[inline(always)]
    fn from(val: u8) -> BufEn {
        BufEn::from_bits(val)
    }
}
impl From<BufEn> for u8 {
    #[inline(always)]
    fn from(val: BufEn) -> u8 {
        BufEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dacen {
    #[doc = "Disables"]
    DISABLED = 0x0,
    #[doc = "Enables"]
    ENABLED = 0x01,
}
impl Dacen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dacen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dacen {
    #[inline(always)]
    fn from(val: u8) -> Dacen {
        Dacen::from_bits(val)
    }
}
impl From<Dacen> for u8 {
    #[inline(always)]
    fn from(val: Dacen) -> u8 {
        Dacen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Empty {
    #[doc = "Not empty"]
    NOT_EMPTY = 0x0,
    #[doc = "Empty"]
    EMPTY = 0x01,
}
impl Empty {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Empty {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Empty {
    #[inline(always)]
    fn from(val: u8) -> Empty {
        Empty::from_bits(val)
    }
}
impl From<Empty> for u8 {
    #[inline(always)]
    fn from(val: Empty) -> u8 {
        Empty::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EmptyDmaen {
    #[doc = "Disables"]
    DISABLED = 0x0,
    #[doc = "Enables"]
    ENABLED = 0x01,
}
impl EmptyDmaen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EmptyDmaen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EmptyDmaen {
    #[inline(always)]
    fn from(val: u8) -> EmptyDmaen {
        EmptyDmaen::from_bits(val)
    }
}
impl From<EmptyDmaen> for u8 {
    #[inline(always)]
    fn from(val: EmptyDmaen) -> u8 {
        EmptyDmaen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EmptyIe {
    #[doc = "Disables"]
    DISABLED = 0x0,
    #[doc = "Enables"]
    ENABLED = 0x01,
}
impl EmptyIe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EmptyIe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EmptyIe {
    #[inline(always)]
    fn from(val: u8) -> EmptyIe {
        EmptyIe::from_bits(val)
    }
}
impl From<EmptyIe> for u8 {
    #[inline(always)]
    fn from(val: EmptyIe) -> u8 {
        EmptyIe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fifoen {
    #[doc = "Enables FIFO mode and disables Buffer mode. Any data written to DATA\\[DATA\\] goes to buffer then goes to conversion."]
    BUFFER_MODE = 0x0,
    #[doc = "Enables FIFO mode. Data will be first read from FIFO to buffer and then goes to conversion."]
    FIFO_MODE = 0x01,
}
impl Fifoen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fifoen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fifoen {
    #[inline(always)]
    fn from(val: u8) -> Fifoen {
        Fifoen::from_bits(val)
    }
}
impl From<Fifoen> for u8 {
    #[inline(always)]
    fn from(val: Fifoen) -> u8 {
        Fifoen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fiforst {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "FIFO reset"]
    FIFO_RESET = 0x01,
}
impl Fiforst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fiforst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fiforst {
    #[inline(always)]
    fn from(val: u8) -> Fiforst {
        Fiforst::from_bits(val)
    }
}
impl From<Fiforst> for u8 {
    #[inline(always)]
    fn from(val: Fiforst) -> u8 {
        Fiforst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fifosz {
    _RESERVED_0 = 0x0,
    #[doc = "FIFO depth is 4"]
    VAL_1 = 0x01,
    #[doc = "FIFO depth is 8"]
    VAL_2 = 0x02,
    #[doc = "FIFO depth is 16"]
    VAL_3 = 0x03,
    #[doc = "FIFO depth is 32"]
    VAL_4 = 0x04,
    #[doc = "FIFO depth is 64"]
    VAL_5 = 0x05,
    #[doc = "FIFO depth is 128"]
    VAL_6 = 0x06,
    #[doc = "FIFO depth is 256"]
    VAL_7 = 0x07,
}
impl Fifosz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fifosz {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fifosz {
    #[inline(always)]
    fn from(val: u8) -> Fifosz {
        Fifosz::from_bits(val)
    }
}
impl From<Fifosz> for u8 {
    #[inline(always)]
    fn from(val: Fifosz) -> u8 {
        Fifosz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Full {
    #[doc = "Not full"]
    NOT_FULL = 0x0,
    #[doc = "Full"]
    FULL = 0x01,
}
impl Full {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Full {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Full {
    #[inline(always)]
    fn from(val: u8) -> Full {
        Full::from_bits(val)
    }
}
impl From<Full> for u8 {
    #[inline(always)]
    fn from(val: Full) -> u8 {
        Full::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FullIe {
    #[doc = "Disables"]
    DISABLED = 0x0,
    #[doc = "Enables"]
    ENABLED = 0x01,
}
impl FullIe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FullIe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FullIe {
    #[inline(always)]
    fn from(val: u8) -> FullIe {
        FullIe::from_bits(val)
    }
}
impl From<FullIe> for u8 {
    #[inline(always)]
    fn from(val: FullIe) -> u8 {
        FullIe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Of {
    #[doc = "No overflow has occurred since the last time the flag was cleared"]
    NO_OVERFLOW = 0x0,
    #[doc = "At least one FIFO overflow has occurred since the last time the flag was cleared"]
    OVERFLOW = 0x01,
}
impl Of {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Of {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Of {
    #[inline(always)]
    fn from(val: u8) -> Of {
        Of::from_bits(val)
    }
}
impl From<Of> for u8 {
    #[inline(always)]
    fn from(val: Of) -> u8 {
        Of::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OfIe {
    #[doc = "Disables"]
    DISABLED = 0x0,
    #[doc = "Enables"]
    ENABLED = 0x01,
}
impl OfIe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OfIe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OfIe {
    #[inline(always)]
    fn from(val: u8) -> OfIe {
        OfIe::from_bits(val)
    }
}
impl From<OfIe> for u8 {
    #[inline(always)]
    fn from(val: OfIe) -> u8 {
        OfIe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptgcoco {
    #[doc = "Not completed or not started"]
    NOT_START = 0x0,
    #[doc = "Completed"]
    COMPLETED = 0x01,
}
impl Ptgcoco {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptgcoco {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptgcoco {
    #[inline(always)]
    fn from(val: u8) -> Ptgcoco {
        Ptgcoco::from_bits(val)
    }
}
impl From<Ptgcoco> for u8 {
    #[inline(always)]
    fn from(val: Ptgcoco) -> u8 {
        Ptgcoco::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PtgcocoIe {
    #[doc = "Disables"]
    DISABLED = 0x0,
    #[doc = "Enables"]
    ENABLED = 0x01,
}
impl PtgcocoIe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PtgcocoIe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PtgcocoIe {
    #[inline(always)]
    fn from(val: u8) -> PtgcocoIe {
        PtgcocoIe::from_bits(val)
    }
}
impl From<PtgcocoIe> for u8 {
    #[inline(always)]
    fn from(val: PtgcocoIe) -> u8 {
        PtgcocoIe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptgen {
    #[doc = "Disables"]
    DISABLED = 0x0,
    #[doc = "Enables"]
    ENABLED = 0x01,
}
impl Ptgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptgen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptgen {
    #[inline(always)]
    fn from(val: u8) -> Ptgen {
        Ptgen::from_bits(val)
    }
}
impl From<Ptgen> for u8 {
    #[inline(always)]
    fn from(val: Ptgen) -> u8 {
        Ptgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swbk {
    #[doc = "No swing back cycle has completed since the last time the flag was cleared"]
    NO_SWING = 0x0,
    #[doc = "At least one swing back cycle has occurred since the last time the flag was cleared"]
    SWING_BACK = 0x01,
}
impl Swbk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swbk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swbk {
    #[inline(always)]
    fn from(val: u8) -> Swbk {
        Swbk::from_bits(val)
    }
}
impl From<Swbk> for u8 {
    #[inline(always)]
    fn from(val: Swbk) -> u8 {
        Swbk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwbkIe {
    #[doc = "Disables"]
    DISABLED = 0x0,
    #[doc = "Enables"]
    ENABLED = 0x01,
}
impl SwbkIe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwbkIe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwbkIe {
    #[inline(always)]
    fn from(val: u8) -> SwbkIe {
        SwbkIe::from_bits(val)
    }
}
impl From<SwbkIe> for u8 {
    #[inline(always)]
    fn from(val: SwbkIe) -> u8 {
        SwbkIe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swmd {
    #[doc = "Disables"]
    DISABLE = 0x0,
    #[doc = "Enables"]
    ENABLE = 0x01,
}
impl Swmd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swmd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swmd {
    #[inline(always)]
    fn from(val: u8) -> Swmd {
        Swmd::from_bits(val)
    }
}
impl From<Swmd> for u8 {
    #[inline(always)]
    fn from(val: Swmd) -> u8 {
        Swmd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swrst {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Software reset"]
    SOFTWARE_RESET = 0x01,
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swtrg {
    #[doc = "Not valid"]
    NOT_VALID = 0x0,
    #[doc = "Valid"]
    VALID = 0x01,
}
impl Swtrg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swtrg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swtrg {
    #[inline(always)]
    fn from(val: u8) -> Swtrg {
        Swtrg::from_bits(val)
    }
}
impl From<Swtrg> for u8 {
    #[inline(always)]
    fn from(val: Swtrg) -> u8 {
        Swtrg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trgsel {
    #[doc = "Hardware trigger"]
    HARDWARE = 0x0,
    #[doc = "Software trigger"]
    SOFTWARE = 0x01,
}
impl Trgsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trgsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trgsel {
    #[inline(always)]
    fn from(val: u8) -> Trgsel {
        Trgsel::from_bits(val)
    }
}
impl From<Trgsel> for u8 {
    #[inline(always)]
    fn from(val: Trgsel) -> u8 {
        Trgsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Uf {
    #[doc = "No underflow has occurred since the last time the flag was cleared"]
    NO_UNDERFLOW = 0x0,
    #[doc = "At least one trigger underflow has occurred since the last time the flag was cleared"]
    UNDERFLOW = 0x01,
}
impl Uf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Uf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Uf {
    #[inline(always)]
    fn from(val: u8) -> Uf {
        Uf::from_bits(val)
    }
}
impl From<Uf> for u8 {
    #[inline(always)]
    fn from(val: Uf) -> u8 {
        Uf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UfIe {
    #[doc = "Disables"]
    DISABLED = 0x0,
    #[doc = "Enables"]
    ENABLED = 0x01,
}
impl UfIe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UfIe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UfIe {
    #[inline(always)]
    fn from(val: u8) -> UfIe {
        UfIe::from_bits(val)
    }
}
impl From<UfIe> for u8 {
    #[inline(always)]
    fn from(val: UfIe) -> u8 {
        UfIe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wm {
    #[doc = "Data in FIFO is more than watermark level"]
    MORE_THAN_WLEVEL = 0x0,
    #[doc = "Data in FIFO is less than or equal to watermark level"]
    LESS_THAN_WLEVEL = 0x01,
}
impl Wm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wm {
    #[inline(always)]
    fn from(val: u8) -> Wm {
        Wm::from_bits(val)
    }
}
impl From<Wm> for u8 {
    #[inline(always)]
    fn from(val: Wm) -> u8 {
        Wm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WmDmaen {
    #[doc = "Disables"]
    DISABLED = 0x0,
    #[doc = "Enables"]
    ENABLED = 0x01,
}
impl WmDmaen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WmDmaen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WmDmaen {
    #[inline(always)]
    fn from(val: u8) -> WmDmaen {
        WmDmaen::from_bits(val)
    }
}
impl From<WmDmaen> for u8 {
    #[inline(always)]
    fn from(val: WmDmaen) -> u8 {
        WmDmaen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WmIe {
    #[doc = "Disables"]
    DISABLED = 0x0,
    #[doc = "Enables"]
    ENABLED = 0x01,
}
impl WmIe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WmIe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WmIe {
    #[inline(always)]
    fn from(val: u8) -> WmIe {
        WmIe::from_bits(val)
    }
}
impl From<WmIe> for u8 {
    #[inline(always)]
    fn from(val: WmIe) -> u8 {
        WmIe::to_bits(val)
    }
}
