#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Buserror {
    #[doc = "No error"]
    NO_ERROR = 0x0,
    #[doc = "Error on bus"]
    ERROR = 0x01,
}
impl Buserror {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Buserror {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Buserror {
    #[inline(always)]
    fn from(val: u8) -> Buserror {
        Buserror::from_bits(val)
    }
}
impl From<Buserror> for u8 {
    #[inline(always)]
    fn from(val: Buserror) -> u8 {
        Buserror::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CppreSat {
    #[doc = "No saturation"]
    DISABLE = 0x0,
    #[doc = "Forces sub-32 bit saturation"]
    ENABLE = 0x01,
}
impl CppreSat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CppreSat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CppreSat {
    #[inline(always)]
    fn from(val: u8) -> CppreSat {
        CppreSat::from_bits(val)
    }
}
impl From<CppreSat> for u8 {
    #[inline(always)]
    fn from(val: CppreSat) -> u8 {
        CppreSat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CppreSat8 {
    #[doc = "8 bits"]
    SAT_8_BITS = 0x0,
    #[doc = "16 bits"]
    SAT_16_BITS = 0x01,
}
impl CppreSat8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CppreSat8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CppreSat8 {
    #[inline(always)]
    fn from(val: u8) -> CppreSat8 {
        CppreSat8::from_bits(val)
    }
}
impl From<CppreSat8> for u8 {
    #[inline(always)]
    fn from(val: CppreSat8) -> u8 {
        CppreSat8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cursory {
    #[doc = "Disable cursory mode, full floating-point accuracy (24-bit mantissa + 2 bits before rounding)."]
    DISABLE = 0x0,
    #[doc = "Enable cursory Mode, 16-bit mantissa (bottom bits are zeroed for inputs and outputs of MACs)."]
    ENABLE = 0x01,
}
impl Cursory {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cursory {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cursory {
    #[inline(always)]
    fn from(val: u8) -> Cursory {
        Cursory::from_bits(val)
    }
}
impl From<Cursory> for u8 {
    #[inline(always)]
    fn from(val: Cursory) -> u8 {
        Cursory::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DecodeMachine {
    #[doc = "Coprocessor"]
    COPROCESSOR = 0x0,
    #[doc = "Matrix engine"]
    MATRIX = 0x01,
    #[doc = "Transform engine"]
    TRANSFORM = 0x02,
    #[doc = "Filter engine"]
    FILTER = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CORDIC engine"]
    CORDIC = 0x05,
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
impl DecodeMachine {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DecodeMachine {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DecodeMachine {
    #[inline(always)]
    fn from(val: u8) -> DecodeMachine {
        DecodeMachine::from_bits(val)
    }
}
impl From<DecodeMachine> for u8 {
    #[inline(always)]
    fn from(val: DecodeMachine) -> u8 {
        DecodeMachine::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EventBerr {
    #[doc = "Disable event trigger"]
    DISABLE = 0x0,
    #[doc = "Enable event trigger"]
    ENABLE = 0x01,
}
impl EventBerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventBerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventBerr {
    #[inline(always)]
    fn from(val: u8) -> EventBerr {
        EventBerr::from_bits(val)
    }
}
impl From<EventBerr> for u8 {
    #[inline(always)]
    fn from(val: EventBerr) -> u8 {
        EventBerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EventComp {
    #[doc = "Disable event trigger"]
    DISABLE = 0x0,
    #[doc = "Enable event trigger"]
    ENABLE = 0x01,
}
impl EventComp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventComp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventComp {
    #[inline(always)]
    fn from(val: u8) -> EventComp {
        EventComp::from_bits(val)
    }
}
impl From<EventComp> for u8 {
    #[inline(always)]
    fn from(val: EventComp) -> u8 {
        EventComp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EventFixed {
    #[doc = "Disable event trigger"]
    DISABLE = 0x0,
    #[doc = "Enable event trigger"]
    ENABLE = 0x01,
}
impl EventFixed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventFixed {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventFixed {
    #[inline(always)]
    fn from(val: u8) -> EventFixed {
        EventFixed::from_bits(val)
    }
}
impl From<EventFixed> for u8 {
    #[inline(always)]
    fn from(val: EventFixed) -> u8 {
        EventFixed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EventNan {
    #[doc = "Disable event trigger"]
    DISABLE = 0x0,
    #[doc = "Enable event trigger"]
    ENABLE = 0x01,
}
impl EventNan {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventNan {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventNan {
    #[inline(always)]
    fn from(val: u8) -> EventNan {
        EventNan::from_bits(val)
    }
}
impl From<EventNan> for u8 {
    #[inline(always)]
    fn from(val: EventNan) -> u8 {
        EventNan::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EventOflow {
    #[doc = "Disable event trigger"]
    DISABLE = 0x0,
    #[doc = "Enable event trigger"]
    ENABLE = 0x01,
}
impl EventOflow {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventOflow {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventOflow {
    #[inline(always)]
    fn from(val: u8) -> EventOflow {
        EventOflow::from_bits(val)
    }
}
impl From<EventOflow> for u8 {
    #[inline(always)]
    fn from(val: EventOflow) -> u8 {
        EventOflow::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EventUflow {
    #[doc = "Disable event trigger"]
    DISABLE = 0x0,
    #[doc = "Enable event trigger"]
    ENABLE = 0x01,
}
impl EventUflow {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EventUflow {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EventUflow {
    #[inline(always)]
    fn from(val: u8) -> EventUflow {
        EventUflow::from_bits(val)
    }
}
impl From<EventUflow> for u8 {
    #[inline(always)]
    fn from(val: EventUflow) -> u8 {
        EventUflow::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fixedoverflow {
    #[doc = "No error"]
    NO_ERROR = 0x0,
    #[doc = "Error on fixed-point overflow"]
    ERROR = 0x01,
}
impl Fixedoverflow {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fixedoverflow {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fixedoverflow {
    #[inline(always)]
    fn from(val: u8) -> Fixedoverflow {
        Fixedoverflow::from_bits(val)
    }
}
impl From<Fixedoverflow> for u8 {
    #[inline(always)]
    fn from(val: Fixedoverflow) -> u8 {
        Fixedoverflow::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InaFormatext {
    #[doc = "Q15 16-bit fixed-point integer"]
    Q15 = 0x0,
    #[doc = "Q31 32-bit fixed-point integer"]
    Q31 = 0x01,
    #[doc = "F32 32-bit floating-point format"]
    FLOAT = 0x02,
    _RESERVED_3 = 0x03,
}
impl InaFormatext {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InaFormatext {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InaFormatext {
    #[inline(always)]
    fn from(val: u8) -> InaFormatext {
        InaFormatext::from_bits(val)
    }
}
impl From<InaFormatext> for u8 {
    #[inline(always)]
    fn from(val: InaFormatext) -> u8 {
        InaFormatext::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InaFormatint {
    #[doc = "Q15 16-bit fixed-point integer"]
    Q15 = 0x0,
    #[doc = "Q31 32-bit fixed-point integer"]
    Q31 = 0x01,
    #[doc = "F32 32-bit floating-point format"]
    FLOAT = 0x02,
    _RESERVED_3 = 0x03,
}
impl InaFormatint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InaFormatint {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InaFormatint {
    #[inline(always)]
    fn from(val: u8) -> InaFormatint {
        InaFormatint::from_bits(val)
    }
}
impl From<InaFormatint> for u8 {
    #[inline(always)]
    fn from(val: InaFormatint) -> u8 {
        InaFormatint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InbFormatext {
    #[doc = "Q15 16-bit fixed-point integer"]
    Q15 = 0x0,
    #[doc = "Q31 32-bit fixed-point integer"]
    Q31 = 0x01,
    #[doc = "F32 32-bit floating-point format"]
    FLOAT = 0x02,
    _RESERVED_3 = 0x03,
}
impl InbFormatext {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InbFormatext {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InbFormatext {
    #[inline(always)]
    fn from(val: u8) -> InbFormatext {
        InbFormatext::from_bits(val)
    }
}
impl From<InbFormatext> for u8 {
    #[inline(always)]
    fn from(val: InbFormatext) -> u8 {
        InbFormatext::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InbFormatint {
    #[doc = "Q15 16-bit fixed-point integer"]
    Q15 = 0x0,
    #[doc = "Q31 32-bit fixed-point integer"]
    Q31 = 0x01,
    #[doc = "F32 32-bit floating-point format"]
    FLOAT = 0x02,
    _RESERVED_3 = 0x03,
}
impl InbFormatint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InbFormatint {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InbFormatint {
    #[inline(always)]
    fn from(val: u8) -> InbFormatint {
        InbFormatint::from_bits(val)
    }
}
impl From<InbFormatint> for u8 {
    #[inline(always)]
    fn from(val: InbFormatint) -> u8 {
        InbFormatint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InstBusy {
    #[doc = "Not busy"]
    NOT_BUSY = 0x0,
    #[doc = "Busy"]
    BUSY = 0x01,
}
impl InstBusy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InstBusy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InstBusy {
    #[inline(always)]
    fn from(val: u8) -> InstBusy {
        InstBusy::from_bits(val)
    }
}
impl From<InstBusy> for u8 {
    #[inline(always)]
    fn from(val: InstBusy) -> u8 {
        InstBusy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntrBerr {
    #[doc = "Disable interrupt"]
    DISABLE = 0x0,
    #[doc = "Enable interrupt"]
    ENABLE = 0x01,
}
impl IntrBerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntrBerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntrBerr {
    #[inline(always)]
    fn from(val: u8) -> IntrBerr {
        IntrBerr::from_bits(val)
    }
}
impl From<IntrBerr> for u8 {
    #[inline(always)]
    fn from(val: IntrBerr) -> u8 {
        IntrBerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntrComp {
    #[doc = "Disable interrupt"]
    DISABLE = 0x0,
    #[doc = "Enable interrupt"]
    ENABLE = 0x01,
}
impl IntrComp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntrComp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntrComp {
    #[inline(always)]
    fn from(val: u8) -> IntrComp {
        IntrComp::from_bits(val)
    }
}
impl From<IntrComp> for u8 {
    #[inline(always)]
    fn from(val: IntrComp) -> u8 {
        IntrComp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntrFixed {
    #[doc = "Disable interrupt"]
    DISABLE = 0x0,
    #[doc = "Enable interrupt"]
    ENABLE = 0x01,
}
impl IntrFixed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntrFixed {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntrFixed {
    #[inline(always)]
    fn from(val: u8) -> IntrFixed {
        IntrFixed::from_bits(val)
    }
}
impl From<IntrFixed> for u8 {
    #[inline(always)]
    fn from(val: IntrFixed) -> u8 {
        IntrFixed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntrNan {
    #[doc = "Disable interrupt"]
    DISABLE = 0x0,
    #[doc = "Enable interrupt"]
    ENABLE = 0x01,
}
impl IntrNan {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntrNan {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntrNan {
    #[inline(always)]
    fn from(val: u8) -> IntrNan {
        IntrNan::from_bits(val)
    }
}
impl From<IntrNan> for u8 {
    #[inline(always)]
    fn from(val: IntrNan) -> u8 {
        IntrNan::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntrOflow {
    #[doc = "Disable interrupt"]
    DISABLE = 0x0,
    #[doc = "Enable interrupt"]
    ENABLE = 0x01,
}
impl IntrOflow {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntrOflow {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntrOflow {
    #[inline(always)]
    fn from(val: u8) -> IntrOflow {
        IntrOflow::from_bits(val)
    }
}
impl From<IntrOflow> for u8 {
    #[inline(always)]
    fn from(val: IntrOflow) -> u8 {
        IntrOflow::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntrStat {
    #[doc = "No new interrupt"]
    DISABLE = 0x0,
    #[doc = "Interrupt captured"]
    ENABLE = 0x01,
}
impl IntrStat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntrStat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntrStat {
    #[inline(always)]
    fn from(val: u8) -> IntrStat {
        IntrStat::from_bits(val)
    }
}
impl From<IntrStat> for u8 {
    #[inline(always)]
    fn from(val: IntrStat) -> u8 {
        IntrStat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntrUflow {
    #[doc = "Disable interrupt"]
    DISABLE = 0x0,
    #[doc = "Enable interrupt"]
    ENABLE = 0x01,
}
impl IntrUflow {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntrUflow {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntrUflow {
    #[inline(always)]
    fn from(val: u8) -> IntrUflow {
        IntrUflow::from_bits(val)
    }
}
impl From<IntrUflow> for u8 {
    #[inline(always)]
    fn from(val: IntrUflow) -> u8 {
        IntrUflow::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nan {
    #[doc = "No error"]
    NO_ERROR = 0x0,
    #[doc = "Error on floating-point NaN"]
    ERROR = 0x01,
}
impl Nan {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nan {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nan {
    #[inline(always)]
    fn from(val: u8) -> Nan {
        Nan::from_bits(val)
    }
}
impl From<Nan> for u8 {
    #[inline(always)]
    fn from(val: Nan) -> u8 {
        Nan::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OutFormatext {
    #[doc = "Q15 16-bit fixed-point integer"]
    Q15 = 0x0,
    #[doc = "Q31 32-bit fixed-point integer"]
    Q31 = 0x01,
    #[doc = "F32 32-bit floating-point format"]
    FLOAT = 0x02,
    _RESERVED_3 = 0x03,
}
impl OutFormatext {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutFormatext {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutFormatext {
    #[inline(always)]
    fn from(val: u8) -> OutFormatext {
        OutFormatext::from_bits(val)
    }
}
impl From<OutFormatext> for u8 {
    #[inline(always)]
    fn from(val: OutFormatext) -> u8 {
        OutFormatext::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OutFormatint {
    #[doc = "Q15 16-bit fixed-point integer"]
    Q15 = 0x0,
    #[doc = "Q31 32-bit fixed-point integer"]
    Q31 = 0x01,
    #[doc = "F32 32-bit floating-point format"]
    FLOAT = 0x02,
    _RESERVED_3 = 0x03,
}
impl OutFormatint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutFormatint {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutFormatint {
    #[inline(always)]
    fn from(val: u8) -> OutFormatint {
        OutFormatint::from_bits(val)
    }
}
impl From<OutFormatint> for u8 {
    #[inline(always)]
    fn from(val: OutFormatint) -> u8 {
        OutFormatint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Overflow {
    #[doc = "No error"]
    NO_ERROR = 0x0,
    #[doc = "Error on floating-point overflow"]
    ERROR = 0x01,
}
impl Overflow {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Overflow {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Overflow {
    #[inline(always)]
    fn from(val: u8) -> Overflow {
        Overflow::from_bits(val)
    }
}
impl From<Overflow> for u8 {
    #[inline(always)]
    fn from(val: Overflow) -> u8 {
        Overflow::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TmpFormatext {
    #[doc = "Q15 16-bit fixed-point integer"]
    Q15 = 0x0,
    #[doc = "Q31 32-bit fixed-point integer"]
    Q31 = 0x01,
    #[doc = "F32 32-bit floating-point format"]
    FLOAT = 0x02,
    _RESERVED_3 = 0x03,
}
impl TmpFormatext {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TmpFormatext {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TmpFormatext {
    #[inline(always)]
    fn from(val: u8) -> TmpFormatext {
        TmpFormatext::from_bits(val)
    }
}
impl From<TmpFormatext> for u8 {
    #[inline(always)]
    fn from(val: TmpFormatext) -> u8 {
        TmpFormatext::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TmpFormatint {
    #[doc = "Q15 16-bit fixed-point integer"]
    Q15 = 0x0,
    #[doc = "Q31 32-bit fixed-point integer"]
    Q31 = 0x01,
    #[doc = "F32 32-bit floating-point format"]
    FLOAT = 0x02,
    _RESERVED_3 = 0x03,
}
impl TmpFormatint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TmpFormatint {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TmpFormatint {
    #[inline(always)]
    fn from(val: u8) -> TmpFormatint {
        TmpFormatint::from_bits(val)
    }
}
impl From<TmpFormatint> for u8 {
    #[inline(always)]
    fn from(val: TmpFormatint) -> u8 {
        TmpFormatint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Underflow {
    #[doc = "No error"]
    NO_ERROR = 0x0,
    #[doc = "Error on underflow"]
    ERROR = 0x01,
}
impl Underflow {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Underflow {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Underflow {
    #[inline(always)]
    fn from(val: u8) -> Underflow {
        Underflow::from_bits(val)
    }
}
impl From<Underflow> for u8 {
    #[inline(always)]
    fn from(val: Underflow) -> u8 {
        Underflow::to_bits(val)
    }
}
