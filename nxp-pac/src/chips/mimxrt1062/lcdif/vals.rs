#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl2ClrEvenLinePattern {
    #[doc = "RGB"]
    RGB = 0x0,
    #[doc = "RBG"]
    RBG = 0x01,
    #[doc = "GBR"]
    GBR = 0x02,
    #[doc = "GRB"]
    GRB = 0x03,
    #[doc = "BRG"]
    BRG = 0x04,
    #[doc = "BGR"]
    BGR = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ctrl2ClrEvenLinePattern {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl2ClrEvenLinePattern {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl2ClrEvenLinePattern {
    #[inline(always)]
    fn from(val: u8) -> Ctrl2ClrEvenLinePattern {
        Ctrl2ClrEvenLinePattern::from_bits(val)
    }
}
impl From<Ctrl2ClrEvenLinePattern> for u8 {
    #[inline(always)]
    fn from(val: Ctrl2ClrEvenLinePattern) -> u8 {
        Ctrl2ClrEvenLinePattern::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl2ClrOddLinePattern {
    #[doc = "RGB"]
    RGB = 0x0,
    #[doc = "RBG"]
    RBG = 0x01,
    #[doc = "GBR"]
    GBR = 0x02,
    #[doc = "GRB"]
    GRB = 0x03,
    #[doc = "BRG"]
    BRG = 0x04,
    #[doc = "BGR"]
    BGR = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ctrl2ClrOddLinePattern {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl2ClrOddLinePattern {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl2ClrOddLinePattern {
    #[inline(always)]
    fn from(val: u8) -> Ctrl2ClrOddLinePattern {
        Ctrl2ClrOddLinePattern::from_bits(val)
    }
}
impl From<Ctrl2ClrOddLinePattern> for u8 {
    #[inline(always)]
    fn from(val: Ctrl2ClrOddLinePattern) -> u8 {
        Ctrl2ClrOddLinePattern::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl2ClrOutstandingReqs {
    #[doc = "REQ_1"]
    REQ_1 = 0x0,
    #[doc = "REQ_2"]
    REQ_2 = 0x01,
    #[doc = "REQ_4"]
    REQ_4 = 0x02,
    #[doc = "REQ_8"]
    REQ_8 = 0x03,
    #[doc = "REQ_16"]
    REQ_16 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ctrl2ClrOutstandingReqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl2ClrOutstandingReqs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl2ClrOutstandingReqs {
    #[inline(always)]
    fn from(val: u8) -> Ctrl2ClrOutstandingReqs {
        Ctrl2ClrOutstandingReqs::from_bits(val)
    }
}
impl From<Ctrl2ClrOutstandingReqs> for u8 {
    #[inline(always)]
    fn from(val: Ctrl2ClrOutstandingReqs) -> u8 {
        Ctrl2ClrOutstandingReqs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl2EvenLinePattern {
    #[doc = "RGB"]
    RGB = 0x0,
    #[doc = "RBG"]
    RBG = 0x01,
    #[doc = "GBR"]
    GBR = 0x02,
    #[doc = "GRB"]
    GRB = 0x03,
    #[doc = "BRG"]
    BRG = 0x04,
    #[doc = "BGR"]
    BGR = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ctrl2EvenLinePattern {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl2EvenLinePattern {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl2EvenLinePattern {
    #[inline(always)]
    fn from(val: u8) -> Ctrl2EvenLinePattern {
        Ctrl2EvenLinePattern::from_bits(val)
    }
}
impl From<Ctrl2EvenLinePattern> for u8 {
    #[inline(always)]
    fn from(val: Ctrl2EvenLinePattern) -> u8 {
        Ctrl2EvenLinePattern::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl2OddLinePattern {
    #[doc = "RGB"]
    RGB = 0x0,
    #[doc = "RBG"]
    RBG = 0x01,
    #[doc = "GBR"]
    GBR = 0x02,
    #[doc = "GRB"]
    GRB = 0x03,
    #[doc = "BRG"]
    BRG = 0x04,
    #[doc = "BGR"]
    BGR = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ctrl2OddLinePattern {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl2OddLinePattern {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl2OddLinePattern {
    #[inline(always)]
    fn from(val: u8) -> Ctrl2OddLinePattern {
        Ctrl2OddLinePattern::from_bits(val)
    }
}
impl From<Ctrl2OddLinePattern> for u8 {
    #[inline(always)]
    fn from(val: Ctrl2OddLinePattern) -> u8 {
        Ctrl2OddLinePattern::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl2OutstandingReqs {
    #[doc = "REQ_1"]
    REQ_1 = 0x0,
    #[doc = "REQ_2"]
    REQ_2 = 0x01,
    #[doc = "REQ_4"]
    REQ_4 = 0x02,
    #[doc = "REQ_8"]
    REQ_8 = 0x03,
    #[doc = "REQ_16"]
    REQ_16 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ctrl2OutstandingReqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl2OutstandingReqs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl2OutstandingReqs {
    #[inline(always)]
    fn from(val: u8) -> Ctrl2OutstandingReqs {
        Ctrl2OutstandingReqs::from_bits(val)
    }
}
impl From<Ctrl2OutstandingReqs> for u8 {
    #[inline(always)]
    fn from(val: Ctrl2OutstandingReqs) -> u8 {
        Ctrl2OutstandingReqs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl2SetEvenLinePattern {
    #[doc = "RGB"]
    RGB = 0x0,
    #[doc = "RBG"]
    RBG = 0x01,
    #[doc = "GBR"]
    GBR = 0x02,
    #[doc = "GRB"]
    GRB = 0x03,
    #[doc = "BRG"]
    BRG = 0x04,
    #[doc = "BGR"]
    BGR = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ctrl2SetEvenLinePattern {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl2SetEvenLinePattern {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl2SetEvenLinePattern {
    #[inline(always)]
    fn from(val: u8) -> Ctrl2SetEvenLinePattern {
        Ctrl2SetEvenLinePattern::from_bits(val)
    }
}
impl From<Ctrl2SetEvenLinePattern> for u8 {
    #[inline(always)]
    fn from(val: Ctrl2SetEvenLinePattern) -> u8 {
        Ctrl2SetEvenLinePattern::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl2SetOddLinePattern {
    #[doc = "RGB"]
    RGB = 0x0,
    #[doc = "RBG"]
    RBG = 0x01,
    #[doc = "GBR"]
    GBR = 0x02,
    #[doc = "GRB"]
    GRB = 0x03,
    #[doc = "BRG"]
    BRG = 0x04,
    #[doc = "BGR"]
    BGR = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ctrl2SetOddLinePattern {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl2SetOddLinePattern {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl2SetOddLinePattern {
    #[inline(always)]
    fn from(val: u8) -> Ctrl2SetOddLinePattern {
        Ctrl2SetOddLinePattern::from_bits(val)
    }
}
impl From<Ctrl2SetOddLinePattern> for u8 {
    #[inline(always)]
    fn from(val: Ctrl2SetOddLinePattern) -> u8 {
        Ctrl2SetOddLinePattern::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl2SetOutstandingReqs {
    #[doc = "REQ_1"]
    REQ_1 = 0x0,
    #[doc = "REQ_2"]
    REQ_2 = 0x01,
    #[doc = "REQ_4"]
    REQ_4 = 0x02,
    #[doc = "REQ_8"]
    REQ_8 = 0x03,
    #[doc = "REQ_16"]
    REQ_16 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ctrl2SetOutstandingReqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl2SetOutstandingReqs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl2SetOutstandingReqs {
    #[inline(always)]
    fn from(val: u8) -> Ctrl2SetOutstandingReqs {
        Ctrl2SetOutstandingReqs::from_bits(val)
    }
}
impl From<Ctrl2SetOutstandingReqs> for u8 {
    #[inline(always)]
    fn from(val: Ctrl2SetOutstandingReqs) -> u8 {
        Ctrl2SetOutstandingReqs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl2TogEvenLinePattern {
    #[doc = "RGB"]
    RGB = 0x0,
    #[doc = "RBG"]
    RBG = 0x01,
    #[doc = "GBR"]
    GBR = 0x02,
    #[doc = "GRB"]
    GRB = 0x03,
    #[doc = "BRG"]
    BRG = 0x04,
    #[doc = "BGR"]
    BGR = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ctrl2TogEvenLinePattern {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl2TogEvenLinePattern {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl2TogEvenLinePattern {
    #[inline(always)]
    fn from(val: u8) -> Ctrl2TogEvenLinePattern {
        Ctrl2TogEvenLinePattern::from_bits(val)
    }
}
impl From<Ctrl2TogEvenLinePattern> for u8 {
    #[inline(always)]
    fn from(val: Ctrl2TogEvenLinePattern) -> u8 {
        Ctrl2TogEvenLinePattern::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl2TogOddLinePattern {
    #[doc = "RGB"]
    RGB = 0x0,
    #[doc = "RBG"]
    RBG = 0x01,
    #[doc = "GBR"]
    GBR = 0x02,
    #[doc = "GRB"]
    GRB = 0x03,
    #[doc = "BRG"]
    BRG = 0x04,
    #[doc = "BGR"]
    BGR = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ctrl2TogOddLinePattern {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl2TogOddLinePattern {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl2TogOddLinePattern {
    #[inline(always)]
    fn from(val: u8) -> Ctrl2TogOddLinePattern {
        Ctrl2TogOddLinePattern::from_bits(val)
    }
}
impl From<Ctrl2TogOddLinePattern> for u8 {
    #[inline(always)]
    fn from(val: Ctrl2TogOddLinePattern) -> u8 {
        Ctrl2TogOddLinePattern::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctrl2TogOutstandingReqs {
    #[doc = "REQ_1"]
    REQ_1 = 0x0,
    #[doc = "REQ_2"]
    REQ_2 = 0x01,
    #[doc = "REQ_4"]
    REQ_4 = 0x02,
    #[doc = "REQ_8"]
    REQ_8 = 0x03,
    #[doc = "REQ_16"]
    REQ_16 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Ctrl2TogOutstandingReqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctrl2TogOutstandingReqs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctrl2TogOutstandingReqs {
    #[inline(always)]
    fn from(val: u8) -> Ctrl2TogOutstandingReqs {
        Ctrl2TogOutstandingReqs::from_bits(val)
    }
}
impl From<Ctrl2TogOutstandingReqs> for u8 {
    #[inline(always)]
    fn from(val: Ctrl2TogOutstandingReqs) -> u8 {
        Ctrl2TogOutstandingReqs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlClrCscDataSwizzle {
    #[doc = "No byte swapping.(Little endian)"]
    NO_SWAP = 0x0,
    #[doc = "Big Endian swap (swap bytes 0,3 and 1,2)."]
    BIG_ENDIAN_SWAP = 0x01,
    #[doc = "Swap half-words."]
    HWD_SWAP = 0x02,
    #[doc = "Swap bytes within each half-word."]
    HWD_BYTE_SWAP = 0x03,
}
impl CtrlClrCscDataSwizzle {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlClrCscDataSwizzle {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlClrCscDataSwizzle {
    #[inline(always)]
    fn from(val: u8) -> CtrlClrCscDataSwizzle {
        CtrlClrCscDataSwizzle::from_bits(val)
    }
}
impl From<CtrlClrCscDataSwizzle> for u8 {
    #[inline(always)]
    fn from(val: CtrlClrCscDataSwizzle) -> u8 {
        CtrlClrCscDataSwizzle::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlClrDataFormat18Bit {
    #[doc = "Data input to the block is in 18 bpp format, such that lower 18 bits contain RGB 666 and upper 14 bits do not contain any useful data."]
    LOWER_18_BITS_VALID = 0x0,
    #[doc = "Data input to the block is in 18 bpp format, such that upper 18 bits contain RGB 666 and lower 14 bits do not contain any useful data."]
    UPPER_18_BITS_VALID = 0x01,
}
impl CtrlClrDataFormat18Bit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlClrDataFormat18Bit {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlClrDataFormat18Bit {
    #[inline(always)]
    fn from(val: u8) -> CtrlClrDataFormat18Bit {
        CtrlClrDataFormat18Bit::from_bits(val)
    }
}
impl From<CtrlClrDataFormat18Bit> for u8 {
    #[inline(always)]
    fn from(val: CtrlClrDataFormat18Bit) -> u8 {
        CtrlClrDataFormat18Bit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlClrDataFormat24Bit {
    #[doc = "Data input to the block is in 24 bpp format, such that all RGB 888 data is contained in 24 bits."]
    ALL_24_BITS_VALID = 0x0,
    #[doc = "Data input to the block is actually RGB 18 bpp, but there is 1 color per byte, hence the upper 2 bits in each byte do not contain any useful data, and should be dropped."]
    DROP_UPPER_2_BITS_PER_BYTE = 0x01,
}
impl CtrlClrDataFormat24Bit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlClrDataFormat24Bit {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlClrDataFormat24Bit {
    #[inline(always)]
    fn from(val: u8) -> CtrlClrDataFormat24Bit {
        CtrlClrDataFormat24Bit::from_bits(val)
    }
}
impl From<CtrlClrDataFormat24Bit> for u8 {
    #[inline(always)]
    fn from(val: CtrlClrDataFormat24Bit) -> u8 {
        CtrlClrDataFormat24Bit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlClrDataShiftDir {
    #[doc = "Data to be transmitted is shifted LEFT by SHIFT_NUM_BITS bits."]
    TXDATA_SHIFT_LEFT = 0x0,
    #[doc = "Data to be transmitted is shifted RIGHT by SHIFT_NUM_BITS bits."]
    TXDATA_SHIFT_RIGHT = 0x01,
}
impl CtrlClrDataShiftDir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlClrDataShiftDir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlClrDataShiftDir {
    #[inline(always)]
    fn from(val: u8) -> CtrlClrDataShiftDir {
        CtrlClrDataShiftDir::from_bits(val)
    }
}
impl From<CtrlClrDataShiftDir> for u8 {
    #[inline(always)]
    fn from(val: CtrlClrDataShiftDir) -> u8 {
        CtrlClrDataShiftDir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlClrInputDataSwizzle {
    #[doc = "No byte swapping.(Little endian)"]
    NO_SWAP = 0x0,
    #[doc = "Big Endian swap (swap bytes 0,3 and 1,2)."]
    BIG_ENDIAN_SWAP = 0x01,
    #[doc = "Swap half-words."]
    HWD_SWAP = 0x02,
    #[doc = "Swap bytes within each half-word."]
    HWD_BYTE_SWAP = 0x03,
}
impl CtrlClrInputDataSwizzle {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlClrInputDataSwizzle {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlClrInputDataSwizzle {
    #[inline(always)]
    fn from(val: u8) -> CtrlClrInputDataSwizzle {
        CtrlClrInputDataSwizzle::from_bits(val)
    }
}
impl From<CtrlClrInputDataSwizzle> for u8 {
    #[inline(always)]
    fn from(val: CtrlClrInputDataSwizzle) -> u8 {
        CtrlClrInputDataSwizzle::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlClrLcdDatabusWidth {
    #[doc = "16-bit data bus mode."]
    _16_BIT = 0x0,
    #[doc = "8-bit data bus mode."]
    _8_BIT = 0x01,
    #[doc = "18-bit data bus mode."]
    _18_BIT = 0x02,
    #[doc = "24-bit data bus mode."]
    _24_BIT = 0x03,
}
impl CtrlClrLcdDatabusWidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlClrLcdDatabusWidth {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlClrLcdDatabusWidth {
    #[inline(always)]
    fn from(val: u8) -> CtrlClrLcdDatabusWidth {
        CtrlClrLcdDatabusWidth::from_bits(val)
    }
}
impl From<CtrlClrLcdDatabusWidth> for u8 {
    #[inline(always)]
    fn from(val: CtrlClrLcdDatabusWidth) -> u8 {
        CtrlClrLcdDatabusWidth::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlClrWordLength {
    #[doc = "Input data is 16 bits per pixel."]
    _16_BIT = 0x0,
    #[doc = "Input data is 8 bits wide."]
    _8_BIT = 0x01,
    #[doc = "Input data is 18 bits per pixel."]
    _18_BIT = 0x02,
    #[doc = "Input data is 24 bits per pixel."]
    _24_BIT = 0x03,
}
impl CtrlClrWordLength {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlClrWordLength {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlClrWordLength {
    #[inline(always)]
    fn from(val: u8) -> CtrlClrWordLength {
        CtrlClrWordLength::from_bits(val)
    }
}
impl From<CtrlClrWordLength> for u8 {
    #[inline(always)]
    fn from(val: CtrlClrWordLength) -> u8 {
        CtrlClrWordLength::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlCscDataSwizzle {
    #[doc = "No byte swapping.(Little endian)"]
    NO_SWAP = 0x0,
    #[doc = "Big Endian swap (swap bytes 0,3 and 1,2)."]
    BIG_ENDIAN_SWAP = 0x01,
    #[doc = "Swap half-words."]
    HWD_SWAP = 0x02,
    #[doc = "Swap bytes within each half-word."]
    HWD_BYTE_SWAP = 0x03,
}
impl CtrlCscDataSwizzle {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlCscDataSwizzle {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlCscDataSwizzle {
    #[inline(always)]
    fn from(val: u8) -> CtrlCscDataSwizzle {
        CtrlCscDataSwizzle::from_bits(val)
    }
}
impl From<CtrlCscDataSwizzle> for u8 {
    #[inline(always)]
    fn from(val: CtrlCscDataSwizzle) -> u8 {
        CtrlCscDataSwizzle::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlDataFormat18Bit {
    #[doc = "Data input to the block is in 18 bpp format, such that lower 18 bits contain RGB 666 and upper 14 bits do not contain any useful data."]
    LOWER_18_BITS_VALID = 0x0,
    #[doc = "Data input to the block is in 18 bpp format, such that upper 18 bits contain RGB 666 and lower 14 bits do not contain any useful data."]
    UPPER_18_BITS_VALID = 0x01,
}
impl CtrlDataFormat18Bit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlDataFormat18Bit {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlDataFormat18Bit {
    #[inline(always)]
    fn from(val: u8) -> CtrlDataFormat18Bit {
        CtrlDataFormat18Bit::from_bits(val)
    }
}
impl From<CtrlDataFormat18Bit> for u8 {
    #[inline(always)]
    fn from(val: CtrlDataFormat18Bit) -> u8 {
        CtrlDataFormat18Bit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlDataFormat24Bit {
    #[doc = "Data input to the block is in 24 bpp format, such that all RGB 888 data is contained in 24 bits."]
    ALL_24_BITS_VALID = 0x0,
    #[doc = "Data input to the block is actually RGB 18 bpp, but there is 1 color per byte, hence the upper 2 bits in each byte do not contain any useful data, and should be dropped."]
    DROP_UPPER_2_BITS_PER_BYTE = 0x01,
}
impl CtrlDataFormat24Bit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlDataFormat24Bit {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlDataFormat24Bit {
    #[inline(always)]
    fn from(val: u8) -> CtrlDataFormat24Bit {
        CtrlDataFormat24Bit::from_bits(val)
    }
}
impl From<CtrlDataFormat24Bit> for u8 {
    #[inline(always)]
    fn from(val: CtrlDataFormat24Bit) -> u8 {
        CtrlDataFormat24Bit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlDataShiftDir {
    #[doc = "Data to be transmitted is shifted LEFT by SHIFT_NUM_BITS bits."]
    TXDATA_SHIFT_LEFT = 0x0,
    #[doc = "Data to be transmitted is shifted RIGHT by SHIFT_NUM_BITS bits."]
    TXDATA_SHIFT_RIGHT = 0x01,
}
impl CtrlDataShiftDir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlDataShiftDir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlDataShiftDir {
    #[inline(always)]
    fn from(val: u8) -> CtrlDataShiftDir {
        CtrlDataShiftDir::from_bits(val)
    }
}
impl From<CtrlDataShiftDir> for u8 {
    #[inline(always)]
    fn from(val: CtrlDataShiftDir) -> u8 {
        CtrlDataShiftDir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlInputDataSwizzle {
    #[doc = "No byte swapping.(Little endian)"]
    NO_SWAP = 0x0,
    #[doc = "Big Endian swap (swap bytes 0,3 and 1,2)."]
    BIG_ENDIAN_SWAP = 0x01,
    #[doc = "Swap half-words."]
    HWD_SWAP = 0x02,
    #[doc = "Swap bytes within each half-word."]
    HWD_BYTE_SWAP = 0x03,
}
impl CtrlInputDataSwizzle {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlInputDataSwizzle {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlInputDataSwizzle {
    #[inline(always)]
    fn from(val: u8) -> CtrlInputDataSwizzle {
        CtrlInputDataSwizzle::from_bits(val)
    }
}
impl From<CtrlInputDataSwizzle> for u8 {
    #[inline(always)]
    fn from(val: CtrlInputDataSwizzle) -> u8 {
        CtrlInputDataSwizzle::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlLcdDatabusWidth {
    #[doc = "16-bit data bus mode."]
    _16_BIT = 0x0,
    #[doc = "8-bit data bus mode."]
    _8_BIT = 0x01,
    #[doc = "18-bit data bus mode."]
    _18_BIT = 0x02,
    #[doc = "24-bit data bus mode."]
    _24_BIT = 0x03,
}
impl CtrlLcdDatabusWidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlLcdDatabusWidth {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlLcdDatabusWidth {
    #[inline(always)]
    fn from(val: u8) -> CtrlLcdDatabusWidth {
        CtrlLcdDatabusWidth::from_bits(val)
    }
}
impl From<CtrlLcdDatabusWidth> for u8 {
    #[inline(always)]
    fn from(val: CtrlLcdDatabusWidth) -> u8 {
        CtrlLcdDatabusWidth::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlSetCscDataSwizzle {
    #[doc = "No byte swapping.(Little endian)"]
    NO_SWAP = 0x0,
    #[doc = "Big Endian swap (swap bytes 0,3 and 1,2)."]
    BIG_ENDIAN_SWAP = 0x01,
    #[doc = "Swap half-words."]
    HWD_SWAP = 0x02,
    #[doc = "Swap bytes within each half-word."]
    HWD_BYTE_SWAP = 0x03,
}
impl CtrlSetCscDataSwizzle {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlSetCscDataSwizzle {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlSetCscDataSwizzle {
    #[inline(always)]
    fn from(val: u8) -> CtrlSetCscDataSwizzle {
        CtrlSetCscDataSwizzle::from_bits(val)
    }
}
impl From<CtrlSetCscDataSwizzle> for u8 {
    #[inline(always)]
    fn from(val: CtrlSetCscDataSwizzle) -> u8 {
        CtrlSetCscDataSwizzle::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlSetDataFormat18Bit {
    #[doc = "Data input to the block is in 18 bpp format, such that lower 18 bits contain RGB 666 and upper 14 bits do not contain any useful data."]
    LOWER_18_BITS_VALID = 0x0,
    #[doc = "Data input to the block is in 18 bpp format, such that upper 18 bits contain RGB 666 and lower 14 bits do not contain any useful data."]
    UPPER_18_BITS_VALID = 0x01,
}
impl CtrlSetDataFormat18Bit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlSetDataFormat18Bit {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlSetDataFormat18Bit {
    #[inline(always)]
    fn from(val: u8) -> CtrlSetDataFormat18Bit {
        CtrlSetDataFormat18Bit::from_bits(val)
    }
}
impl From<CtrlSetDataFormat18Bit> for u8 {
    #[inline(always)]
    fn from(val: CtrlSetDataFormat18Bit) -> u8 {
        CtrlSetDataFormat18Bit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlSetDataFormat24Bit {
    #[doc = "Data input to the block is in 24 bpp format, such that all RGB 888 data is contained in 24 bits."]
    ALL_24_BITS_VALID = 0x0,
    #[doc = "Data input to the block is actually RGB 18 bpp, but there is 1 color per byte, hence the upper 2 bits in each byte do not contain any useful data, and should be dropped."]
    DROP_UPPER_2_BITS_PER_BYTE = 0x01,
}
impl CtrlSetDataFormat24Bit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlSetDataFormat24Bit {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlSetDataFormat24Bit {
    #[inline(always)]
    fn from(val: u8) -> CtrlSetDataFormat24Bit {
        CtrlSetDataFormat24Bit::from_bits(val)
    }
}
impl From<CtrlSetDataFormat24Bit> for u8 {
    #[inline(always)]
    fn from(val: CtrlSetDataFormat24Bit) -> u8 {
        CtrlSetDataFormat24Bit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlSetDataShiftDir {
    #[doc = "Data to be transmitted is shifted LEFT by SHIFT_NUM_BITS bits."]
    TXDATA_SHIFT_LEFT = 0x0,
    #[doc = "Data to be transmitted is shifted RIGHT by SHIFT_NUM_BITS bits."]
    TXDATA_SHIFT_RIGHT = 0x01,
}
impl CtrlSetDataShiftDir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlSetDataShiftDir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlSetDataShiftDir {
    #[inline(always)]
    fn from(val: u8) -> CtrlSetDataShiftDir {
        CtrlSetDataShiftDir::from_bits(val)
    }
}
impl From<CtrlSetDataShiftDir> for u8 {
    #[inline(always)]
    fn from(val: CtrlSetDataShiftDir) -> u8 {
        CtrlSetDataShiftDir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlSetInputDataSwizzle {
    #[doc = "No byte swapping.(Little endian)"]
    NO_SWAP = 0x0,
    #[doc = "Big Endian swap (swap bytes 0,3 and 1,2)."]
    BIG_ENDIAN_SWAP = 0x01,
    #[doc = "Swap half-words."]
    HWD_SWAP = 0x02,
    #[doc = "Swap bytes within each half-word."]
    HWD_BYTE_SWAP = 0x03,
}
impl CtrlSetInputDataSwizzle {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlSetInputDataSwizzle {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlSetInputDataSwizzle {
    #[inline(always)]
    fn from(val: u8) -> CtrlSetInputDataSwizzle {
        CtrlSetInputDataSwizzle::from_bits(val)
    }
}
impl From<CtrlSetInputDataSwizzle> for u8 {
    #[inline(always)]
    fn from(val: CtrlSetInputDataSwizzle) -> u8 {
        CtrlSetInputDataSwizzle::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlSetLcdDatabusWidth {
    #[doc = "16-bit data bus mode."]
    _16_BIT = 0x0,
    #[doc = "8-bit data bus mode."]
    _8_BIT = 0x01,
    #[doc = "18-bit data bus mode."]
    _18_BIT = 0x02,
    #[doc = "24-bit data bus mode."]
    _24_BIT = 0x03,
}
impl CtrlSetLcdDatabusWidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlSetLcdDatabusWidth {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlSetLcdDatabusWidth {
    #[inline(always)]
    fn from(val: u8) -> CtrlSetLcdDatabusWidth {
        CtrlSetLcdDatabusWidth::from_bits(val)
    }
}
impl From<CtrlSetLcdDatabusWidth> for u8 {
    #[inline(always)]
    fn from(val: CtrlSetLcdDatabusWidth) -> u8 {
        CtrlSetLcdDatabusWidth::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlSetWordLength {
    #[doc = "Input data is 16 bits per pixel."]
    _16_BIT = 0x0,
    #[doc = "Input data is 8 bits wide."]
    _8_BIT = 0x01,
    #[doc = "Input data is 18 bits per pixel."]
    _18_BIT = 0x02,
    #[doc = "Input data is 24 bits per pixel."]
    _24_BIT = 0x03,
}
impl CtrlSetWordLength {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlSetWordLength {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlSetWordLength {
    #[inline(always)]
    fn from(val: u8) -> CtrlSetWordLength {
        CtrlSetWordLength::from_bits(val)
    }
}
impl From<CtrlSetWordLength> for u8 {
    #[inline(always)]
    fn from(val: CtrlSetWordLength) -> u8 {
        CtrlSetWordLength::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlTogCscDataSwizzle {
    #[doc = "No byte swapping.(Little endian)"]
    NO_SWAP = 0x0,
    #[doc = "Big Endian swap (swap bytes 0,3 and 1,2)."]
    BIG_ENDIAN_SWAP = 0x01,
    #[doc = "Swap half-words."]
    HWD_SWAP = 0x02,
    #[doc = "Swap bytes within each half-word."]
    HWD_BYTE_SWAP = 0x03,
}
impl CtrlTogCscDataSwizzle {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlTogCscDataSwizzle {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlTogCscDataSwizzle {
    #[inline(always)]
    fn from(val: u8) -> CtrlTogCscDataSwizzle {
        CtrlTogCscDataSwizzle::from_bits(val)
    }
}
impl From<CtrlTogCscDataSwizzle> for u8 {
    #[inline(always)]
    fn from(val: CtrlTogCscDataSwizzle) -> u8 {
        CtrlTogCscDataSwizzle::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlTogDataFormat18Bit {
    #[doc = "Data input to the block is in 18 bpp format, such that lower 18 bits contain RGB 666 and upper 14 bits do not contain any useful data."]
    LOWER_18_BITS_VALID = 0x0,
    #[doc = "Data input to the block is in 18 bpp format, such that upper 18 bits contain RGB 666 and lower 14 bits do not contain any useful data."]
    UPPER_18_BITS_VALID = 0x01,
}
impl CtrlTogDataFormat18Bit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlTogDataFormat18Bit {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlTogDataFormat18Bit {
    #[inline(always)]
    fn from(val: u8) -> CtrlTogDataFormat18Bit {
        CtrlTogDataFormat18Bit::from_bits(val)
    }
}
impl From<CtrlTogDataFormat18Bit> for u8 {
    #[inline(always)]
    fn from(val: CtrlTogDataFormat18Bit) -> u8 {
        CtrlTogDataFormat18Bit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlTogDataFormat24Bit {
    #[doc = "Data input to the block is in 24 bpp format, such that all RGB 888 data is contained in 24 bits."]
    ALL_24_BITS_VALID = 0x0,
    #[doc = "Data input to the block is actually RGB 18 bpp, but there is 1 color per byte, hence the upper 2 bits in each byte do not contain any useful data, and should be dropped."]
    DROP_UPPER_2_BITS_PER_BYTE = 0x01,
}
impl CtrlTogDataFormat24Bit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlTogDataFormat24Bit {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlTogDataFormat24Bit {
    #[inline(always)]
    fn from(val: u8) -> CtrlTogDataFormat24Bit {
        CtrlTogDataFormat24Bit::from_bits(val)
    }
}
impl From<CtrlTogDataFormat24Bit> for u8 {
    #[inline(always)]
    fn from(val: CtrlTogDataFormat24Bit) -> u8 {
        CtrlTogDataFormat24Bit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlTogDataShiftDir {
    #[doc = "Data to be transmitted is shifted LEFT by SHIFT_NUM_BITS bits."]
    TXDATA_SHIFT_LEFT = 0x0,
    #[doc = "Data to be transmitted is shifted RIGHT by SHIFT_NUM_BITS bits."]
    TXDATA_SHIFT_RIGHT = 0x01,
}
impl CtrlTogDataShiftDir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlTogDataShiftDir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlTogDataShiftDir {
    #[inline(always)]
    fn from(val: u8) -> CtrlTogDataShiftDir {
        CtrlTogDataShiftDir::from_bits(val)
    }
}
impl From<CtrlTogDataShiftDir> for u8 {
    #[inline(always)]
    fn from(val: CtrlTogDataShiftDir) -> u8 {
        CtrlTogDataShiftDir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlTogInputDataSwizzle {
    #[doc = "No byte swapping.(Little endian)"]
    NO_SWAP = 0x0,
    #[doc = "Big Endian swap (swap bytes 0,3 and 1,2)."]
    BIG_ENDIAN_SWAP = 0x01,
    #[doc = "Swap half-words."]
    HWD_SWAP = 0x02,
    #[doc = "Swap bytes within each half-word."]
    HWD_BYTE_SWAP = 0x03,
}
impl CtrlTogInputDataSwizzle {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlTogInputDataSwizzle {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlTogInputDataSwizzle {
    #[inline(always)]
    fn from(val: u8) -> CtrlTogInputDataSwizzle {
        CtrlTogInputDataSwizzle::from_bits(val)
    }
}
impl From<CtrlTogInputDataSwizzle> for u8 {
    #[inline(always)]
    fn from(val: CtrlTogInputDataSwizzle) -> u8 {
        CtrlTogInputDataSwizzle::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlTogLcdDatabusWidth {
    #[doc = "16-bit data bus mode."]
    _16_BIT = 0x0,
    #[doc = "8-bit data bus mode."]
    _8_BIT = 0x01,
    #[doc = "18-bit data bus mode."]
    _18_BIT = 0x02,
    #[doc = "24-bit data bus mode."]
    _24_BIT = 0x03,
}
impl CtrlTogLcdDatabusWidth {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlTogLcdDatabusWidth {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlTogLcdDatabusWidth {
    #[inline(always)]
    fn from(val: u8) -> CtrlTogLcdDatabusWidth {
        CtrlTogLcdDatabusWidth::from_bits(val)
    }
}
impl From<CtrlTogLcdDatabusWidth> for u8 {
    #[inline(always)]
    fn from(val: CtrlTogLcdDatabusWidth) -> u8 {
        CtrlTogLcdDatabusWidth::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlTogWordLength {
    #[doc = "Input data is 16 bits per pixel."]
    _16_BIT = 0x0,
    #[doc = "Input data is 8 bits wide."]
    _8_BIT = 0x01,
    #[doc = "Input data is 18 bits per pixel."]
    _18_BIT = 0x02,
    #[doc = "Input data is 24 bits per pixel."]
    _24_BIT = 0x03,
}
impl CtrlTogWordLength {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlTogWordLength {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlTogWordLength {
    #[inline(always)]
    fn from(val: u8) -> CtrlTogWordLength {
        CtrlTogWordLength::from_bits(val)
    }
}
impl From<CtrlTogWordLength> for u8 {
    #[inline(always)]
    fn from(val: CtrlTogWordLength) -> u8 {
        CtrlTogWordLength::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlWordLength {
    #[doc = "Input data is 16 bits per pixel."]
    _16_BIT = 0x0,
    #[doc = "Input data is 8 bits wide."]
    _8_BIT = 0x01,
    #[doc = "Input data is 18 bits per pixel."]
    _18_BIT = 0x02,
    #[doc = "Input data is 24 bits per pixel."]
    _24_BIT = 0x03,
}
impl CtrlWordLength {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlWordLength {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlWordLength {
    #[inline(always)]
    fn from(val: u8) -> CtrlWordLength {
        CtrlWordLength::from_bits(val)
    }
}
impl From<CtrlWordLength> for u8 {
    #[inline(always)]
    fn from(val: CtrlWordLength) -> u8 {
        CtrlWordLength::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon00IncSel {
    #[doc = "pclk"]
    PCLK = 0x0,
    #[doc = "Line start pulse"]
    LINE = 0x01,
    #[doc = "Frame start pulse"]
    FRAME = 0x02,
    #[doc = "Use another signal as tick event"]
    SIG_ANOTHER = 0x03,
}
impl Pigeon00IncSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon00IncSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon00IncSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon00IncSel {
        Pigeon00IncSel::from_bits(val)
    }
}
impl From<Pigeon00IncSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon00IncSel) -> u8 {
        Pigeon00IncSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon00MaskCntSel {
    #[doc = "pclk counter within one hscan state"]
    HSTATE_CNT = 0x0,
    #[doc = "pclk cycle within one hscan state"]
    HSTATE_CYCLE = 0x01,
    #[doc = "line counter within one vscan state"]
    VSTATE_CNT = 0x02,
    #[doc = "line cycle within one vscan state"]
    VSTATE_CYCLE = 0x03,
    #[doc = "frame counter"]
    FRAME_CNT = 0x04,
    #[doc = "frame cycle"]
    FRAME_CYCLE = 0x05,
    #[doc = "horizontal counter (pclk counter within one line )"]
    HCNT = 0x06,
    #[doc = "vertical counter (line counter within one frame)"]
    VCNT = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pigeon00MaskCntSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon00MaskCntSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon00MaskCntSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon00MaskCntSel {
        Pigeon00MaskCntSel::from_bits(val)
    }
}
impl From<Pigeon00MaskCntSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon00MaskCntSel) -> u8 {
        Pigeon00MaskCntSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon00Pol {
    #[doc = "Normal Signal (Active high)"]
    ACTIVE_HIGH = 0x0,
    #[doc = "Inverted signal (Active low)"]
    ACTIVE_LOW = 0x01,
}
impl Pigeon00Pol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon00Pol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon00Pol {
    #[inline(always)]
    fn from(val: u8) -> Pigeon00Pol {
        Pigeon00Pol::from_bits(val)
    }
}
impl From<Pigeon00Pol> for u8 {
    #[inline(always)]
    fn from(val: Pigeon00Pol) -> u8 {
        Pigeon00Pol::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon00StateMask(u8);
impl Pigeon00StateMask {
    #[doc = "FRAME SYNC"]
    pub const FS: Self = Self(0x01);
    #[doc = "FRAME BEGIN"]
    pub const FB: Self = Self(0x02);
    #[doc = "FRAME DATA"]
    pub const FD: Self = Self(0x04);
    #[doc = "FRAME END"]
    pub const FE: Self = Self(0x08);
    #[doc = "LINE SYNC"]
    pub const LS: Self = Self(0x10);
    #[doc = "LINE BEGIN"]
    pub const LB: Self = Self(0x20);
    #[doc = "LINE DATA"]
    pub const LD: Self = Self(0x40);
    #[doc = "LINE END"]
    pub const LE: Self = Self(0x80);
}
impl Pigeon00StateMask {
    pub const fn from_bits(val: u8) -> Pigeon00StateMask {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon00StateMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("FS"),
            0x02 => f.write_str("FB"),
            0x04 => f.write_str("FD"),
            0x08 => f.write_str("FE"),
            0x10 => f.write_str("LS"),
            0x20 => f.write_str("LB"),
            0x40 => f.write_str("LD"),
            0x80 => f.write_str("LE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon00StateMask {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "FS"),
            0x02 => defmt::write!(f, "FB"),
            0x04 => defmt::write!(f, "FD"),
            0x08 => defmt::write!(f, "FE"),
            0x10 => defmt::write!(f, "LS"),
            0x20 => defmt::write!(f, "LB"),
            0x40 => defmt::write!(f, "LD"),
            0x80 => defmt::write!(f, "LE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Pigeon00StateMask {
    #[inline(always)]
    fn from(val: u8) -> Pigeon00StateMask {
        Pigeon00StateMask::from_bits(val)
    }
}
impl From<Pigeon00StateMask> for u8 {
    #[inline(always)]
    fn from(val: Pigeon00StateMask) -> u8 {
        Pigeon00StateMask::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon01ClrCnt(u16);
impl Pigeon01ClrCnt {
    #[doc = "Keep active until mask off"]
    pub const CLEAR_USING_MASK: Self = Self(0x0);
}
impl Pigeon01ClrCnt {
    pub const fn from_bits(val: u16) -> Pigeon01ClrCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon01ClrCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("CLEAR_USING_MASK"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon01ClrCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "CLEAR_USING_MASK"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon01ClrCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon01ClrCnt {
        Pigeon01ClrCnt::from_bits(val)
    }
}
impl From<Pigeon01ClrCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon01ClrCnt) -> u16 {
        Pigeon01ClrCnt::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon01SetCnt(u16);
impl Pigeon01SetCnt {
    #[doc = "Start as active"]
    pub const START_ACTIVE: Self = Self(0x0);
}
impl Pigeon01SetCnt {
    pub const fn from_bits(val: u16) -> Pigeon01SetCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon01SetCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("START_ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon01SetCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "START_ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon01SetCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon01SetCnt {
        Pigeon01SetCnt::from_bits(val)
    }
}
impl From<Pigeon01SetCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon01SetCnt) -> u16 {
        Pigeon01SetCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon02SigAnother {
    #[doc = "Keep active until mask off"]
    CLEAR_USING_MASK = 0x0,
    _RESERVED_1 = 0x01,
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
    _RESERVED_1f = 0x1f,
}
impl Pigeon02SigAnother {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon02SigAnother {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon02SigAnother {
    #[inline(always)]
    fn from(val: u8) -> Pigeon02SigAnother {
        Pigeon02SigAnother::from_bits(val)
    }
}
impl From<Pigeon02SigAnother> for u8 {
    #[inline(always)]
    fn from(val: Pigeon02SigAnother) -> u8 {
        Pigeon02SigAnother::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon02SigLogic {
    #[doc = "No logic operation"]
    DIS = 0x0,
    #[doc = "sigout = sig_another AND this_sig"]
    AND = 0x01,
    #[doc = "sigout = sig_another OR this_sig"]
    OR = 0x02,
    #[doc = "mask = sig_another AND other_masks"]
    MASK = 0x03,
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
impl Pigeon02SigLogic {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon02SigLogic {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon02SigLogic {
    #[inline(always)]
    fn from(val: u8) -> Pigeon02SigLogic {
        Pigeon02SigLogic::from_bits(val)
    }
}
impl From<Pigeon02SigLogic> for u8 {
    #[inline(always)]
    fn from(val: Pigeon02SigLogic) -> u8 {
        Pigeon02SigLogic::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon100IncSel {
    #[doc = "pclk"]
    PCLK = 0x0,
    #[doc = "Line start pulse"]
    LINE = 0x01,
    #[doc = "Frame start pulse"]
    FRAME = 0x02,
    #[doc = "Use another signal as tick event"]
    SIG_ANOTHER = 0x03,
}
impl Pigeon100IncSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon100IncSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon100IncSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon100IncSel {
        Pigeon100IncSel::from_bits(val)
    }
}
impl From<Pigeon100IncSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon100IncSel) -> u8 {
        Pigeon100IncSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon100MaskCntSel {
    #[doc = "pclk counter within one hscan state"]
    HSTATE_CNT = 0x0,
    #[doc = "pclk cycle within one hscan state"]
    HSTATE_CYCLE = 0x01,
    #[doc = "line counter within one vscan state"]
    VSTATE_CNT = 0x02,
    #[doc = "line cycle within one vscan state"]
    VSTATE_CYCLE = 0x03,
    #[doc = "frame counter"]
    FRAME_CNT = 0x04,
    #[doc = "frame cycle"]
    FRAME_CYCLE = 0x05,
    #[doc = "horizontal counter (pclk counter within one line )"]
    HCNT = 0x06,
    #[doc = "vertical counter (line counter within one frame)"]
    VCNT = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pigeon100MaskCntSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon100MaskCntSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon100MaskCntSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon100MaskCntSel {
        Pigeon100MaskCntSel::from_bits(val)
    }
}
impl From<Pigeon100MaskCntSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon100MaskCntSel) -> u8 {
        Pigeon100MaskCntSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon100Pol {
    #[doc = "Normal Signal (Active high)"]
    ACTIVE_HIGH = 0x0,
    #[doc = "Inverted signal (Active low)"]
    ACTIVE_LOW = 0x01,
}
impl Pigeon100Pol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon100Pol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon100Pol {
    #[inline(always)]
    fn from(val: u8) -> Pigeon100Pol {
        Pigeon100Pol::from_bits(val)
    }
}
impl From<Pigeon100Pol> for u8 {
    #[inline(always)]
    fn from(val: Pigeon100Pol) -> u8 {
        Pigeon100Pol::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon100StateMask(u8);
impl Pigeon100StateMask {
    #[doc = "FRAME SYNC"]
    pub const FS: Self = Self(0x01);
    #[doc = "FRAME BEGIN"]
    pub const FB: Self = Self(0x02);
    #[doc = "FRAME DATA"]
    pub const FD: Self = Self(0x04);
    #[doc = "FRAME END"]
    pub const FE: Self = Self(0x08);
    #[doc = "LINE SYNC"]
    pub const LS: Self = Self(0x10);
    #[doc = "LINE BEGIN"]
    pub const LB: Self = Self(0x20);
    #[doc = "LINE DATA"]
    pub const LD: Self = Self(0x40);
    #[doc = "LINE END"]
    pub const LE: Self = Self(0x80);
}
impl Pigeon100StateMask {
    pub const fn from_bits(val: u8) -> Pigeon100StateMask {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon100StateMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("FS"),
            0x02 => f.write_str("FB"),
            0x04 => f.write_str("FD"),
            0x08 => f.write_str("FE"),
            0x10 => f.write_str("LS"),
            0x20 => f.write_str("LB"),
            0x40 => f.write_str("LD"),
            0x80 => f.write_str("LE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon100StateMask {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "FS"),
            0x02 => defmt::write!(f, "FB"),
            0x04 => defmt::write!(f, "FD"),
            0x08 => defmt::write!(f, "FE"),
            0x10 => defmt::write!(f, "LS"),
            0x20 => defmt::write!(f, "LB"),
            0x40 => defmt::write!(f, "LD"),
            0x80 => defmt::write!(f, "LE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Pigeon100StateMask {
    #[inline(always)]
    fn from(val: u8) -> Pigeon100StateMask {
        Pigeon100StateMask::from_bits(val)
    }
}
impl From<Pigeon100StateMask> for u8 {
    #[inline(always)]
    fn from(val: Pigeon100StateMask) -> u8 {
        Pigeon100StateMask::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon101ClrCnt(u16);
impl Pigeon101ClrCnt {
    #[doc = "Keep active until mask off"]
    pub const CLEAR_USING_MASK: Self = Self(0x0);
}
impl Pigeon101ClrCnt {
    pub const fn from_bits(val: u16) -> Pigeon101ClrCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon101ClrCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("CLEAR_USING_MASK"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon101ClrCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "CLEAR_USING_MASK"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon101ClrCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon101ClrCnt {
        Pigeon101ClrCnt::from_bits(val)
    }
}
impl From<Pigeon101ClrCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon101ClrCnt) -> u16 {
        Pigeon101ClrCnt::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon101SetCnt(u16);
impl Pigeon101SetCnt {
    #[doc = "Start as active"]
    pub const START_ACTIVE: Self = Self(0x0);
}
impl Pigeon101SetCnt {
    pub const fn from_bits(val: u16) -> Pigeon101SetCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon101SetCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("START_ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon101SetCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "START_ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon101SetCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon101SetCnt {
        Pigeon101SetCnt::from_bits(val)
    }
}
impl From<Pigeon101SetCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon101SetCnt) -> u16 {
        Pigeon101SetCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon102SigAnother {
    #[doc = "Keep active until mask off"]
    CLEAR_USING_MASK = 0x0,
    _RESERVED_1 = 0x01,
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
    _RESERVED_1f = 0x1f,
}
impl Pigeon102SigAnother {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon102SigAnother {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon102SigAnother {
    #[inline(always)]
    fn from(val: u8) -> Pigeon102SigAnother {
        Pigeon102SigAnother::from_bits(val)
    }
}
impl From<Pigeon102SigAnother> for u8 {
    #[inline(always)]
    fn from(val: Pigeon102SigAnother) -> u8 {
        Pigeon102SigAnother::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon102SigLogic {
    #[doc = "No logic operation"]
    DIS = 0x0,
    #[doc = "sigout = sig_another AND this_sig"]
    AND = 0x01,
    #[doc = "sigout = sig_another OR this_sig"]
    OR = 0x02,
    #[doc = "mask = sig_another AND other_masks"]
    MASK = 0x03,
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
impl Pigeon102SigLogic {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon102SigLogic {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon102SigLogic {
    #[inline(always)]
    fn from(val: u8) -> Pigeon102SigLogic {
        Pigeon102SigLogic::from_bits(val)
    }
}
impl From<Pigeon102SigLogic> for u8 {
    #[inline(always)]
    fn from(val: Pigeon102SigLogic) -> u8 {
        Pigeon102SigLogic::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon10IncSel {
    #[doc = "pclk"]
    PCLK = 0x0,
    #[doc = "Line start pulse"]
    LINE = 0x01,
    #[doc = "Frame start pulse"]
    FRAME = 0x02,
    #[doc = "Use another signal as tick event"]
    SIG_ANOTHER = 0x03,
}
impl Pigeon10IncSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon10IncSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon10IncSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon10IncSel {
        Pigeon10IncSel::from_bits(val)
    }
}
impl From<Pigeon10IncSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon10IncSel) -> u8 {
        Pigeon10IncSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon10MaskCntSel {
    #[doc = "pclk counter within one hscan state"]
    HSTATE_CNT = 0x0,
    #[doc = "pclk cycle within one hscan state"]
    HSTATE_CYCLE = 0x01,
    #[doc = "line counter within one vscan state"]
    VSTATE_CNT = 0x02,
    #[doc = "line cycle within one vscan state"]
    VSTATE_CYCLE = 0x03,
    #[doc = "frame counter"]
    FRAME_CNT = 0x04,
    #[doc = "frame cycle"]
    FRAME_CYCLE = 0x05,
    #[doc = "horizontal counter (pclk counter within one line )"]
    HCNT = 0x06,
    #[doc = "vertical counter (line counter within one frame)"]
    VCNT = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pigeon10MaskCntSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon10MaskCntSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon10MaskCntSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon10MaskCntSel {
        Pigeon10MaskCntSel::from_bits(val)
    }
}
impl From<Pigeon10MaskCntSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon10MaskCntSel) -> u8 {
        Pigeon10MaskCntSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon10Pol {
    #[doc = "Normal Signal (Active high)"]
    ACTIVE_HIGH = 0x0,
    #[doc = "Inverted signal (Active low)"]
    ACTIVE_LOW = 0x01,
}
impl Pigeon10Pol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon10Pol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon10Pol {
    #[inline(always)]
    fn from(val: u8) -> Pigeon10Pol {
        Pigeon10Pol::from_bits(val)
    }
}
impl From<Pigeon10Pol> for u8 {
    #[inline(always)]
    fn from(val: Pigeon10Pol) -> u8 {
        Pigeon10Pol::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon10StateMask(u8);
impl Pigeon10StateMask {
    #[doc = "FRAME SYNC"]
    pub const FS: Self = Self(0x01);
    #[doc = "FRAME BEGIN"]
    pub const FB: Self = Self(0x02);
    #[doc = "FRAME DATA"]
    pub const FD: Self = Self(0x04);
    #[doc = "FRAME END"]
    pub const FE: Self = Self(0x08);
    #[doc = "LINE SYNC"]
    pub const LS: Self = Self(0x10);
    #[doc = "LINE BEGIN"]
    pub const LB: Self = Self(0x20);
    #[doc = "LINE DATA"]
    pub const LD: Self = Self(0x40);
    #[doc = "LINE END"]
    pub const LE: Self = Self(0x80);
}
impl Pigeon10StateMask {
    pub const fn from_bits(val: u8) -> Pigeon10StateMask {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon10StateMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("FS"),
            0x02 => f.write_str("FB"),
            0x04 => f.write_str("FD"),
            0x08 => f.write_str("FE"),
            0x10 => f.write_str("LS"),
            0x20 => f.write_str("LB"),
            0x40 => f.write_str("LD"),
            0x80 => f.write_str("LE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon10StateMask {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "FS"),
            0x02 => defmt::write!(f, "FB"),
            0x04 => defmt::write!(f, "FD"),
            0x08 => defmt::write!(f, "FE"),
            0x10 => defmt::write!(f, "LS"),
            0x20 => defmt::write!(f, "LB"),
            0x40 => defmt::write!(f, "LD"),
            0x80 => defmt::write!(f, "LE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Pigeon10StateMask {
    #[inline(always)]
    fn from(val: u8) -> Pigeon10StateMask {
        Pigeon10StateMask::from_bits(val)
    }
}
impl From<Pigeon10StateMask> for u8 {
    #[inline(always)]
    fn from(val: Pigeon10StateMask) -> u8 {
        Pigeon10StateMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon110IncSel {
    #[doc = "pclk"]
    PCLK = 0x0,
    #[doc = "Line start pulse"]
    LINE = 0x01,
    #[doc = "Frame start pulse"]
    FRAME = 0x02,
    #[doc = "Use another signal as tick event"]
    SIG_ANOTHER = 0x03,
}
impl Pigeon110IncSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon110IncSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon110IncSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon110IncSel {
        Pigeon110IncSel::from_bits(val)
    }
}
impl From<Pigeon110IncSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon110IncSel) -> u8 {
        Pigeon110IncSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon110MaskCntSel {
    #[doc = "pclk counter within one hscan state"]
    HSTATE_CNT = 0x0,
    #[doc = "pclk cycle within one hscan state"]
    HSTATE_CYCLE = 0x01,
    #[doc = "line counter within one vscan state"]
    VSTATE_CNT = 0x02,
    #[doc = "line cycle within one vscan state"]
    VSTATE_CYCLE = 0x03,
    #[doc = "frame counter"]
    FRAME_CNT = 0x04,
    #[doc = "frame cycle"]
    FRAME_CYCLE = 0x05,
    #[doc = "horizontal counter (pclk counter within one line )"]
    HCNT = 0x06,
    #[doc = "vertical counter (line counter within one frame)"]
    VCNT = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pigeon110MaskCntSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon110MaskCntSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon110MaskCntSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon110MaskCntSel {
        Pigeon110MaskCntSel::from_bits(val)
    }
}
impl From<Pigeon110MaskCntSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon110MaskCntSel) -> u8 {
        Pigeon110MaskCntSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon110Pol {
    #[doc = "Normal Signal (Active high)"]
    ACTIVE_HIGH = 0x0,
    #[doc = "Inverted signal (Active low)"]
    ACTIVE_LOW = 0x01,
}
impl Pigeon110Pol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon110Pol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon110Pol {
    #[inline(always)]
    fn from(val: u8) -> Pigeon110Pol {
        Pigeon110Pol::from_bits(val)
    }
}
impl From<Pigeon110Pol> for u8 {
    #[inline(always)]
    fn from(val: Pigeon110Pol) -> u8 {
        Pigeon110Pol::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon110StateMask(u8);
impl Pigeon110StateMask {
    #[doc = "FRAME SYNC"]
    pub const FS: Self = Self(0x01);
    #[doc = "FRAME BEGIN"]
    pub const FB: Self = Self(0x02);
    #[doc = "FRAME DATA"]
    pub const FD: Self = Self(0x04);
    #[doc = "FRAME END"]
    pub const FE: Self = Self(0x08);
    #[doc = "LINE SYNC"]
    pub const LS: Self = Self(0x10);
    #[doc = "LINE BEGIN"]
    pub const LB: Self = Self(0x20);
    #[doc = "LINE DATA"]
    pub const LD: Self = Self(0x40);
    #[doc = "LINE END"]
    pub const LE: Self = Self(0x80);
}
impl Pigeon110StateMask {
    pub const fn from_bits(val: u8) -> Pigeon110StateMask {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon110StateMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("FS"),
            0x02 => f.write_str("FB"),
            0x04 => f.write_str("FD"),
            0x08 => f.write_str("FE"),
            0x10 => f.write_str("LS"),
            0x20 => f.write_str("LB"),
            0x40 => f.write_str("LD"),
            0x80 => f.write_str("LE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon110StateMask {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "FS"),
            0x02 => defmt::write!(f, "FB"),
            0x04 => defmt::write!(f, "FD"),
            0x08 => defmt::write!(f, "FE"),
            0x10 => defmt::write!(f, "LS"),
            0x20 => defmt::write!(f, "LB"),
            0x40 => defmt::write!(f, "LD"),
            0x80 => defmt::write!(f, "LE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Pigeon110StateMask {
    #[inline(always)]
    fn from(val: u8) -> Pigeon110StateMask {
        Pigeon110StateMask::from_bits(val)
    }
}
impl From<Pigeon110StateMask> for u8 {
    #[inline(always)]
    fn from(val: Pigeon110StateMask) -> u8 {
        Pigeon110StateMask::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon111ClrCnt(u16);
impl Pigeon111ClrCnt {
    #[doc = "Keep active until mask off"]
    pub const CLEAR_USING_MASK: Self = Self(0x0);
}
impl Pigeon111ClrCnt {
    pub const fn from_bits(val: u16) -> Pigeon111ClrCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon111ClrCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("CLEAR_USING_MASK"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon111ClrCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "CLEAR_USING_MASK"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon111ClrCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon111ClrCnt {
        Pigeon111ClrCnt::from_bits(val)
    }
}
impl From<Pigeon111ClrCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon111ClrCnt) -> u16 {
        Pigeon111ClrCnt::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon111SetCnt(u16);
impl Pigeon111SetCnt {
    #[doc = "Start as active"]
    pub const START_ACTIVE: Self = Self(0x0);
}
impl Pigeon111SetCnt {
    pub const fn from_bits(val: u16) -> Pigeon111SetCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon111SetCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("START_ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon111SetCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "START_ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon111SetCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon111SetCnt {
        Pigeon111SetCnt::from_bits(val)
    }
}
impl From<Pigeon111SetCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon111SetCnt) -> u16 {
        Pigeon111SetCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon112SigAnother {
    #[doc = "Keep active until mask off"]
    CLEAR_USING_MASK = 0x0,
    _RESERVED_1 = 0x01,
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
    _RESERVED_1f = 0x1f,
}
impl Pigeon112SigAnother {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon112SigAnother {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon112SigAnother {
    #[inline(always)]
    fn from(val: u8) -> Pigeon112SigAnother {
        Pigeon112SigAnother::from_bits(val)
    }
}
impl From<Pigeon112SigAnother> for u8 {
    #[inline(always)]
    fn from(val: Pigeon112SigAnother) -> u8 {
        Pigeon112SigAnother::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon112SigLogic {
    #[doc = "No logic operation"]
    DIS = 0x0,
    #[doc = "sigout = sig_another AND this_sig"]
    AND = 0x01,
    #[doc = "sigout = sig_another OR this_sig"]
    OR = 0x02,
    #[doc = "mask = sig_another AND other_masks"]
    MASK = 0x03,
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
impl Pigeon112SigLogic {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon112SigLogic {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon112SigLogic {
    #[inline(always)]
    fn from(val: u8) -> Pigeon112SigLogic {
        Pigeon112SigLogic::from_bits(val)
    }
}
impl From<Pigeon112SigLogic> for u8 {
    #[inline(always)]
    fn from(val: Pigeon112SigLogic) -> u8 {
        Pigeon112SigLogic::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon11ClrCnt(u16);
impl Pigeon11ClrCnt {
    #[doc = "Keep active until mask off"]
    pub const CLEAR_USING_MASK: Self = Self(0x0);
}
impl Pigeon11ClrCnt {
    pub const fn from_bits(val: u16) -> Pigeon11ClrCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon11ClrCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("CLEAR_USING_MASK"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon11ClrCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "CLEAR_USING_MASK"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon11ClrCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon11ClrCnt {
        Pigeon11ClrCnt::from_bits(val)
    }
}
impl From<Pigeon11ClrCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon11ClrCnt) -> u16 {
        Pigeon11ClrCnt::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon11SetCnt(u16);
impl Pigeon11SetCnt {
    #[doc = "Start as active"]
    pub const START_ACTIVE: Self = Self(0x0);
}
impl Pigeon11SetCnt {
    pub const fn from_bits(val: u16) -> Pigeon11SetCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon11SetCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("START_ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon11SetCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "START_ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon11SetCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon11SetCnt {
        Pigeon11SetCnt::from_bits(val)
    }
}
impl From<Pigeon11SetCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon11SetCnt) -> u16 {
        Pigeon11SetCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon12SigAnother {
    #[doc = "Keep active until mask off"]
    CLEAR_USING_MASK = 0x0,
    _RESERVED_1 = 0x01,
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
    _RESERVED_1f = 0x1f,
}
impl Pigeon12SigAnother {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon12SigAnother {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon12SigAnother {
    #[inline(always)]
    fn from(val: u8) -> Pigeon12SigAnother {
        Pigeon12SigAnother::from_bits(val)
    }
}
impl From<Pigeon12SigAnother> for u8 {
    #[inline(always)]
    fn from(val: Pigeon12SigAnother) -> u8 {
        Pigeon12SigAnother::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon12SigLogic {
    #[doc = "No logic operation"]
    DIS = 0x0,
    #[doc = "sigout = sig_another AND this_sig"]
    AND = 0x01,
    #[doc = "sigout = sig_another OR this_sig"]
    OR = 0x02,
    #[doc = "mask = sig_another AND other_masks"]
    MASK = 0x03,
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
impl Pigeon12SigLogic {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon12SigLogic {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon12SigLogic {
    #[inline(always)]
    fn from(val: u8) -> Pigeon12SigLogic {
        Pigeon12SigLogic::from_bits(val)
    }
}
impl From<Pigeon12SigLogic> for u8 {
    #[inline(always)]
    fn from(val: Pigeon12SigLogic) -> u8 {
        Pigeon12SigLogic::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon20IncSel {
    #[doc = "pclk"]
    PCLK = 0x0,
    #[doc = "Line start pulse"]
    LINE = 0x01,
    #[doc = "Frame start pulse"]
    FRAME = 0x02,
    #[doc = "Use another signal as tick event"]
    SIG_ANOTHER = 0x03,
}
impl Pigeon20IncSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon20IncSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon20IncSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon20IncSel {
        Pigeon20IncSel::from_bits(val)
    }
}
impl From<Pigeon20IncSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon20IncSel) -> u8 {
        Pigeon20IncSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon20MaskCntSel {
    #[doc = "pclk counter within one hscan state"]
    HSTATE_CNT = 0x0,
    #[doc = "pclk cycle within one hscan state"]
    HSTATE_CYCLE = 0x01,
    #[doc = "line counter within one vscan state"]
    VSTATE_CNT = 0x02,
    #[doc = "line cycle within one vscan state"]
    VSTATE_CYCLE = 0x03,
    #[doc = "frame counter"]
    FRAME_CNT = 0x04,
    #[doc = "frame cycle"]
    FRAME_CYCLE = 0x05,
    #[doc = "horizontal counter (pclk counter within one line )"]
    HCNT = 0x06,
    #[doc = "vertical counter (line counter within one frame)"]
    VCNT = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pigeon20MaskCntSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon20MaskCntSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon20MaskCntSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon20MaskCntSel {
        Pigeon20MaskCntSel::from_bits(val)
    }
}
impl From<Pigeon20MaskCntSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon20MaskCntSel) -> u8 {
        Pigeon20MaskCntSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon20Pol {
    #[doc = "Normal Signal (Active high)"]
    ACTIVE_HIGH = 0x0,
    #[doc = "Inverted signal (Active low)"]
    ACTIVE_LOW = 0x01,
}
impl Pigeon20Pol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon20Pol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon20Pol {
    #[inline(always)]
    fn from(val: u8) -> Pigeon20Pol {
        Pigeon20Pol::from_bits(val)
    }
}
impl From<Pigeon20Pol> for u8 {
    #[inline(always)]
    fn from(val: Pigeon20Pol) -> u8 {
        Pigeon20Pol::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon20StateMask(u8);
impl Pigeon20StateMask {
    #[doc = "FRAME SYNC"]
    pub const FS: Self = Self(0x01);
    #[doc = "FRAME BEGIN"]
    pub const FB: Self = Self(0x02);
    #[doc = "FRAME DATA"]
    pub const FD: Self = Self(0x04);
    #[doc = "FRAME END"]
    pub const FE: Self = Self(0x08);
    #[doc = "LINE SYNC"]
    pub const LS: Self = Self(0x10);
    #[doc = "LINE BEGIN"]
    pub const LB: Self = Self(0x20);
    #[doc = "LINE DATA"]
    pub const LD: Self = Self(0x40);
    #[doc = "LINE END"]
    pub const LE: Self = Self(0x80);
}
impl Pigeon20StateMask {
    pub const fn from_bits(val: u8) -> Pigeon20StateMask {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon20StateMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("FS"),
            0x02 => f.write_str("FB"),
            0x04 => f.write_str("FD"),
            0x08 => f.write_str("FE"),
            0x10 => f.write_str("LS"),
            0x20 => f.write_str("LB"),
            0x40 => f.write_str("LD"),
            0x80 => f.write_str("LE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon20StateMask {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "FS"),
            0x02 => defmt::write!(f, "FB"),
            0x04 => defmt::write!(f, "FD"),
            0x08 => defmt::write!(f, "FE"),
            0x10 => defmt::write!(f, "LS"),
            0x20 => defmt::write!(f, "LB"),
            0x40 => defmt::write!(f, "LD"),
            0x80 => defmt::write!(f, "LE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Pigeon20StateMask {
    #[inline(always)]
    fn from(val: u8) -> Pigeon20StateMask {
        Pigeon20StateMask::from_bits(val)
    }
}
impl From<Pigeon20StateMask> for u8 {
    #[inline(always)]
    fn from(val: Pigeon20StateMask) -> u8 {
        Pigeon20StateMask::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon21ClrCnt(u16);
impl Pigeon21ClrCnt {
    #[doc = "Keep active until mask off"]
    pub const CLEAR_USING_MASK: Self = Self(0x0);
}
impl Pigeon21ClrCnt {
    pub const fn from_bits(val: u16) -> Pigeon21ClrCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon21ClrCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("CLEAR_USING_MASK"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon21ClrCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "CLEAR_USING_MASK"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon21ClrCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon21ClrCnt {
        Pigeon21ClrCnt::from_bits(val)
    }
}
impl From<Pigeon21ClrCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon21ClrCnt) -> u16 {
        Pigeon21ClrCnt::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon21SetCnt(u16);
impl Pigeon21SetCnt {
    #[doc = "Start as active"]
    pub const START_ACTIVE: Self = Self(0x0);
}
impl Pigeon21SetCnt {
    pub const fn from_bits(val: u16) -> Pigeon21SetCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon21SetCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("START_ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon21SetCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "START_ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon21SetCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon21SetCnt {
        Pigeon21SetCnt::from_bits(val)
    }
}
impl From<Pigeon21SetCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon21SetCnt) -> u16 {
        Pigeon21SetCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon22SigAnother {
    #[doc = "Keep active until mask off"]
    CLEAR_USING_MASK = 0x0,
    _RESERVED_1 = 0x01,
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
    _RESERVED_1f = 0x1f,
}
impl Pigeon22SigAnother {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon22SigAnother {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon22SigAnother {
    #[inline(always)]
    fn from(val: u8) -> Pigeon22SigAnother {
        Pigeon22SigAnother::from_bits(val)
    }
}
impl From<Pigeon22SigAnother> for u8 {
    #[inline(always)]
    fn from(val: Pigeon22SigAnother) -> u8 {
        Pigeon22SigAnother::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon22SigLogic {
    #[doc = "No logic operation"]
    DIS = 0x0,
    #[doc = "sigout = sig_another AND this_sig"]
    AND = 0x01,
    #[doc = "sigout = sig_another OR this_sig"]
    OR = 0x02,
    #[doc = "mask = sig_another AND other_masks"]
    MASK = 0x03,
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
impl Pigeon22SigLogic {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon22SigLogic {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon22SigLogic {
    #[inline(always)]
    fn from(val: u8) -> Pigeon22SigLogic {
        Pigeon22SigLogic::from_bits(val)
    }
}
impl From<Pigeon22SigLogic> for u8 {
    #[inline(always)]
    fn from(val: Pigeon22SigLogic) -> u8 {
        Pigeon22SigLogic::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon30IncSel {
    #[doc = "pclk"]
    PCLK = 0x0,
    #[doc = "Line start pulse"]
    LINE = 0x01,
    #[doc = "Frame start pulse"]
    FRAME = 0x02,
    #[doc = "Use another signal as tick event"]
    SIG_ANOTHER = 0x03,
}
impl Pigeon30IncSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon30IncSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon30IncSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon30IncSel {
        Pigeon30IncSel::from_bits(val)
    }
}
impl From<Pigeon30IncSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon30IncSel) -> u8 {
        Pigeon30IncSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon30MaskCntSel {
    #[doc = "pclk counter within one hscan state"]
    HSTATE_CNT = 0x0,
    #[doc = "pclk cycle within one hscan state"]
    HSTATE_CYCLE = 0x01,
    #[doc = "line counter within one vscan state"]
    VSTATE_CNT = 0x02,
    #[doc = "line cycle within one vscan state"]
    VSTATE_CYCLE = 0x03,
    #[doc = "frame counter"]
    FRAME_CNT = 0x04,
    #[doc = "frame cycle"]
    FRAME_CYCLE = 0x05,
    #[doc = "horizontal counter (pclk counter within one line )"]
    HCNT = 0x06,
    #[doc = "vertical counter (line counter within one frame)"]
    VCNT = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pigeon30MaskCntSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon30MaskCntSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon30MaskCntSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon30MaskCntSel {
        Pigeon30MaskCntSel::from_bits(val)
    }
}
impl From<Pigeon30MaskCntSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon30MaskCntSel) -> u8 {
        Pigeon30MaskCntSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon30Pol {
    #[doc = "Normal Signal (Active high)"]
    ACTIVE_HIGH = 0x0,
    #[doc = "Inverted signal (Active low)"]
    ACTIVE_LOW = 0x01,
}
impl Pigeon30Pol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon30Pol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon30Pol {
    #[inline(always)]
    fn from(val: u8) -> Pigeon30Pol {
        Pigeon30Pol::from_bits(val)
    }
}
impl From<Pigeon30Pol> for u8 {
    #[inline(always)]
    fn from(val: Pigeon30Pol) -> u8 {
        Pigeon30Pol::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon30StateMask(u8);
impl Pigeon30StateMask {
    #[doc = "FRAME SYNC"]
    pub const FS: Self = Self(0x01);
    #[doc = "FRAME BEGIN"]
    pub const FB: Self = Self(0x02);
    #[doc = "FRAME DATA"]
    pub const FD: Self = Self(0x04);
    #[doc = "FRAME END"]
    pub const FE: Self = Self(0x08);
    #[doc = "LINE SYNC"]
    pub const LS: Self = Self(0x10);
    #[doc = "LINE BEGIN"]
    pub const LB: Self = Self(0x20);
    #[doc = "LINE DATA"]
    pub const LD: Self = Self(0x40);
    #[doc = "LINE END"]
    pub const LE: Self = Self(0x80);
}
impl Pigeon30StateMask {
    pub const fn from_bits(val: u8) -> Pigeon30StateMask {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon30StateMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("FS"),
            0x02 => f.write_str("FB"),
            0x04 => f.write_str("FD"),
            0x08 => f.write_str("FE"),
            0x10 => f.write_str("LS"),
            0x20 => f.write_str("LB"),
            0x40 => f.write_str("LD"),
            0x80 => f.write_str("LE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon30StateMask {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "FS"),
            0x02 => defmt::write!(f, "FB"),
            0x04 => defmt::write!(f, "FD"),
            0x08 => defmt::write!(f, "FE"),
            0x10 => defmt::write!(f, "LS"),
            0x20 => defmt::write!(f, "LB"),
            0x40 => defmt::write!(f, "LD"),
            0x80 => defmt::write!(f, "LE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Pigeon30StateMask {
    #[inline(always)]
    fn from(val: u8) -> Pigeon30StateMask {
        Pigeon30StateMask::from_bits(val)
    }
}
impl From<Pigeon30StateMask> for u8 {
    #[inline(always)]
    fn from(val: Pigeon30StateMask) -> u8 {
        Pigeon30StateMask::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon31ClrCnt(u16);
impl Pigeon31ClrCnt {
    #[doc = "Keep active until mask off"]
    pub const CLEAR_USING_MASK: Self = Self(0x0);
}
impl Pigeon31ClrCnt {
    pub const fn from_bits(val: u16) -> Pigeon31ClrCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon31ClrCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("CLEAR_USING_MASK"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon31ClrCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "CLEAR_USING_MASK"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon31ClrCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon31ClrCnt {
        Pigeon31ClrCnt::from_bits(val)
    }
}
impl From<Pigeon31ClrCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon31ClrCnt) -> u16 {
        Pigeon31ClrCnt::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon31SetCnt(u16);
impl Pigeon31SetCnt {
    #[doc = "Start as active"]
    pub const START_ACTIVE: Self = Self(0x0);
}
impl Pigeon31SetCnt {
    pub const fn from_bits(val: u16) -> Pigeon31SetCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon31SetCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("START_ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon31SetCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "START_ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon31SetCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon31SetCnt {
        Pigeon31SetCnt::from_bits(val)
    }
}
impl From<Pigeon31SetCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon31SetCnt) -> u16 {
        Pigeon31SetCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon32SigAnother {
    #[doc = "Keep active until mask off"]
    CLEAR_USING_MASK = 0x0,
    _RESERVED_1 = 0x01,
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
    _RESERVED_1f = 0x1f,
}
impl Pigeon32SigAnother {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon32SigAnother {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon32SigAnother {
    #[inline(always)]
    fn from(val: u8) -> Pigeon32SigAnother {
        Pigeon32SigAnother::from_bits(val)
    }
}
impl From<Pigeon32SigAnother> for u8 {
    #[inline(always)]
    fn from(val: Pigeon32SigAnother) -> u8 {
        Pigeon32SigAnother::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon32SigLogic {
    #[doc = "No logic operation"]
    DIS = 0x0,
    #[doc = "sigout = sig_another AND this_sig"]
    AND = 0x01,
    #[doc = "sigout = sig_another OR this_sig"]
    OR = 0x02,
    #[doc = "mask = sig_another AND other_masks"]
    MASK = 0x03,
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
impl Pigeon32SigLogic {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon32SigLogic {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon32SigLogic {
    #[inline(always)]
    fn from(val: u8) -> Pigeon32SigLogic {
        Pigeon32SigLogic::from_bits(val)
    }
}
impl From<Pigeon32SigLogic> for u8 {
    #[inline(always)]
    fn from(val: Pigeon32SigLogic) -> u8 {
        Pigeon32SigLogic::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon40IncSel {
    #[doc = "pclk"]
    PCLK = 0x0,
    #[doc = "Line start pulse"]
    LINE = 0x01,
    #[doc = "Frame start pulse"]
    FRAME = 0x02,
    #[doc = "Use another signal as tick event"]
    SIG_ANOTHER = 0x03,
}
impl Pigeon40IncSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon40IncSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon40IncSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon40IncSel {
        Pigeon40IncSel::from_bits(val)
    }
}
impl From<Pigeon40IncSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon40IncSel) -> u8 {
        Pigeon40IncSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon40MaskCntSel {
    #[doc = "pclk counter within one hscan state"]
    HSTATE_CNT = 0x0,
    #[doc = "pclk cycle within one hscan state"]
    HSTATE_CYCLE = 0x01,
    #[doc = "line counter within one vscan state"]
    VSTATE_CNT = 0x02,
    #[doc = "line cycle within one vscan state"]
    VSTATE_CYCLE = 0x03,
    #[doc = "frame counter"]
    FRAME_CNT = 0x04,
    #[doc = "frame cycle"]
    FRAME_CYCLE = 0x05,
    #[doc = "horizontal counter (pclk counter within one line )"]
    HCNT = 0x06,
    #[doc = "vertical counter (line counter within one frame)"]
    VCNT = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pigeon40MaskCntSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon40MaskCntSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon40MaskCntSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon40MaskCntSel {
        Pigeon40MaskCntSel::from_bits(val)
    }
}
impl From<Pigeon40MaskCntSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon40MaskCntSel) -> u8 {
        Pigeon40MaskCntSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon40Pol {
    #[doc = "Normal Signal (Active high)"]
    ACTIVE_HIGH = 0x0,
    #[doc = "Inverted signal (Active low)"]
    ACTIVE_LOW = 0x01,
}
impl Pigeon40Pol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon40Pol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon40Pol {
    #[inline(always)]
    fn from(val: u8) -> Pigeon40Pol {
        Pigeon40Pol::from_bits(val)
    }
}
impl From<Pigeon40Pol> for u8 {
    #[inline(always)]
    fn from(val: Pigeon40Pol) -> u8 {
        Pigeon40Pol::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon40StateMask(u8);
impl Pigeon40StateMask {
    #[doc = "FRAME SYNC"]
    pub const FS: Self = Self(0x01);
    #[doc = "FRAME BEGIN"]
    pub const FB: Self = Self(0x02);
    #[doc = "FRAME DATA"]
    pub const FD: Self = Self(0x04);
    #[doc = "FRAME END"]
    pub const FE: Self = Self(0x08);
    #[doc = "LINE SYNC"]
    pub const LS: Self = Self(0x10);
    #[doc = "LINE BEGIN"]
    pub const LB: Self = Self(0x20);
    #[doc = "LINE DATA"]
    pub const LD: Self = Self(0x40);
    #[doc = "LINE END"]
    pub const LE: Self = Self(0x80);
}
impl Pigeon40StateMask {
    pub const fn from_bits(val: u8) -> Pigeon40StateMask {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon40StateMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("FS"),
            0x02 => f.write_str("FB"),
            0x04 => f.write_str("FD"),
            0x08 => f.write_str("FE"),
            0x10 => f.write_str("LS"),
            0x20 => f.write_str("LB"),
            0x40 => f.write_str("LD"),
            0x80 => f.write_str("LE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon40StateMask {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "FS"),
            0x02 => defmt::write!(f, "FB"),
            0x04 => defmt::write!(f, "FD"),
            0x08 => defmt::write!(f, "FE"),
            0x10 => defmt::write!(f, "LS"),
            0x20 => defmt::write!(f, "LB"),
            0x40 => defmt::write!(f, "LD"),
            0x80 => defmt::write!(f, "LE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Pigeon40StateMask {
    #[inline(always)]
    fn from(val: u8) -> Pigeon40StateMask {
        Pigeon40StateMask::from_bits(val)
    }
}
impl From<Pigeon40StateMask> for u8 {
    #[inline(always)]
    fn from(val: Pigeon40StateMask) -> u8 {
        Pigeon40StateMask::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon41ClrCnt(u16);
impl Pigeon41ClrCnt {
    #[doc = "Keep active until mask off"]
    pub const CLEAR_USING_MASK: Self = Self(0x0);
}
impl Pigeon41ClrCnt {
    pub const fn from_bits(val: u16) -> Pigeon41ClrCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon41ClrCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("CLEAR_USING_MASK"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon41ClrCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "CLEAR_USING_MASK"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon41ClrCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon41ClrCnt {
        Pigeon41ClrCnt::from_bits(val)
    }
}
impl From<Pigeon41ClrCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon41ClrCnt) -> u16 {
        Pigeon41ClrCnt::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon41SetCnt(u16);
impl Pigeon41SetCnt {
    #[doc = "Start as active"]
    pub const START_ACTIVE: Self = Self(0x0);
}
impl Pigeon41SetCnt {
    pub const fn from_bits(val: u16) -> Pigeon41SetCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon41SetCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("START_ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon41SetCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "START_ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon41SetCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon41SetCnt {
        Pigeon41SetCnt::from_bits(val)
    }
}
impl From<Pigeon41SetCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon41SetCnt) -> u16 {
        Pigeon41SetCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon42SigAnother {
    #[doc = "Keep active until mask off"]
    CLEAR_USING_MASK = 0x0,
    _RESERVED_1 = 0x01,
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
    _RESERVED_1f = 0x1f,
}
impl Pigeon42SigAnother {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon42SigAnother {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon42SigAnother {
    #[inline(always)]
    fn from(val: u8) -> Pigeon42SigAnother {
        Pigeon42SigAnother::from_bits(val)
    }
}
impl From<Pigeon42SigAnother> for u8 {
    #[inline(always)]
    fn from(val: Pigeon42SigAnother) -> u8 {
        Pigeon42SigAnother::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon42SigLogic {
    #[doc = "No logic operation"]
    DIS = 0x0,
    #[doc = "sigout = sig_another AND this_sig"]
    AND = 0x01,
    #[doc = "sigout = sig_another OR this_sig"]
    OR = 0x02,
    #[doc = "mask = sig_another AND other_masks"]
    MASK = 0x03,
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
impl Pigeon42SigLogic {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon42SigLogic {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon42SigLogic {
    #[inline(always)]
    fn from(val: u8) -> Pigeon42SigLogic {
        Pigeon42SigLogic::from_bits(val)
    }
}
impl From<Pigeon42SigLogic> for u8 {
    #[inline(always)]
    fn from(val: Pigeon42SigLogic) -> u8 {
        Pigeon42SigLogic::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon50IncSel {
    #[doc = "pclk"]
    PCLK = 0x0,
    #[doc = "Line start pulse"]
    LINE = 0x01,
    #[doc = "Frame start pulse"]
    FRAME = 0x02,
    #[doc = "Use another signal as tick event"]
    SIG_ANOTHER = 0x03,
}
impl Pigeon50IncSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon50IncSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon50IncSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon50IncSel {
        Pigeon50IncSel::from_bits(val)
    }
}
impl From<Pigeon50IncSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon50IncSel) -> u8 {
        Pigeon50IncSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon50MaskCntSel {
    #[doc = "pclk counter within one hscan state"]
    HSTATE_CNT = 0x0,
    #[doc = "pclk cycle within one hscan state"]
    HSTATE_CYCLE = 0x01,
    #[doc = "line counter within one vscan state"]
    VSTATE_CNT = 0x02,
    #[doc = "line cycle within one vscan state"]
    VSTATE_CYCLE = 0x03,
    #[doc = "frame counter"]
    FRAME_CNT = 0x04,
    #[doc = "frame cycle"]
    FRAME_CYCLE = 0x05,
    #[doc = "horizontal counter (pclk counter within one line )"]
    HCNT = 0x06,
    #[doc = "vertical counter (line counter within one frame)"]
    VCNT = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pigeon50MaskCntSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon50MaskCntSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon50MaskCntSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon50MaskCntSel {
        Pigeon50MaskCntSel::from_bits(val)
    }
}
impl From<Pigeon50MaskCntSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon50MaskCntSel) -> u8 {
        Pigeon50MaskCntSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon50Pol {
    #[doc = "Normal Signal (Active high)"]
    ACTIVE_HIGH = 0x0,
    #[doc = "Inverted signal (Active low)"]
    ACTIVE_LOW = 0x01,
}
impl Pigeon50Pol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon50Pol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon50Pol {
    #[inline(always)]
    fn from(val: u8) -> Pigeon50Pol {
        Pigeon50Pol::from_bits(val)
    }
}
impl From<Pigeon50Pol> for u8 {
    #[inline(always)]
    fn from(val: Pigeon50Pol) -> u8 {
        Pigeon50Pol::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon50StateMask(u8);
impl Pigeon50StateMask {
    #[doc = "FRAME SYNC"]
    pub const FS: Self = Self(0x01);
    #[doc = "FRAME BEGIN"]
    pub const FB: Self = Self(0x02);
    #[doc = "FRAME DATA"]
    pub const FD: Self = Self(0x04);
    #[doc = "FRAME END"]
    pub const FE: Self = Self(0x08);
    #[doc = "LINE SYNC"]
    pub const LS: Self = Self(0x10);
    #[doc = "LINE BEGIN"]
    pub const LB: Self = Self(0x20);
    #[doc = "LINE DATA"]
    pub const LD: Self = Self(0x40);
    #[doc = "LINE END"]
    pub const LE: Self = Self(0x80);
}
impl Pigeon50StateMask {
    pub const fn from_bits(val: u8) -> Pigeon50StateMask {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon50StateMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("FS"),
            0x02 => f.write_str("FB"),
            0x04 => f.write_str("FD"),
            0x08 => f.write_str("FE"),
            0x10 => f.write_str("LS"),
            0x20 => f.write_str("LB"),
            0x40 => f.write_str("LD"),
            0x80 => f.write_str("LE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon50StateMask {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "FS"),
            0x02 => defmt::write!(f, "FB"),
            0x04 => defmt::write!(f, "FD"),
            0x08 => defmt::write!(f, "FE"),
            0x10 => defmt::write!(f, "LS"),
            0x20 => defmt::write!(f, "LB"),
            0x40 => defmt::write!(f, "LD"),
            0x80 => defmt::write!(f, "LE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Pigeon50StateMask {
    #[inline(always)]
    fn from(val: u8) -> Pigeon50StateMask {
        Pigeon50StateMask::from_bits(val)
    }
}
impl From<Pigeon50StateMask> for u8 {
    #[inline(always)]
    fn from(val: Pigeon50StateMask) -> u8 {
        Pigeon50StateMask::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon51ClrCnt(u16);
impl Pigeon51ClrCnt {
    #[doc = "Keep active until mask off"]
    pub const CLEAR_USING_MASK: Self = Self(0x0);
}
impl Pigeon51ClrCnt {
    pub const fn from_bits(val: u16) -> Pigeon51ClrCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon51ClrCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("CLEAR_USING_MASK"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon51ClrCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "CLEAR_USING_MASK"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon51ClrCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon51ClrCnt {
        Pigeon51ClrCnt::from_bits(val)
    }
}
impl From<Pigeon51ClrCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon51ClrCnt) -> u16 {
        Pigeon51ClrCnt::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon51SetCnt(u16);
impl Pigeon51SetCnt {
    #[doc = "Start as active"]
    pub const START_ACTIVE: Self = Self(0x0);
}
impl Pigeon51SetCnt {
    pub const fn from_bits(val: u16) -> Pigeon51SetCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon51SetCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("START_ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon51SetCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "START_ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon51SetCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon51SetCnt {
        Pigeon51SetCnt::from_bits(val)
    }
}
impl From<Pigeon51SetCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon51SetCnt) -> u16 {
        Pigeon51SetCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon52SigAnother {
    #[doc = "Keep active until mask off"]
    CLEAR_USING_MASK = 0x0,
    _RESERVED_1 = 0x01,
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
    _RESERVED_1f = 0x1f,
}
impl Pigeon52SigAnother {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon52SigAnother {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon52SigAnother {
    #[inline(always)]
    fn from(val: u8) -> Pigeon52SigAnother {
        Pigeon52SigAnother::from_bits(val)
    }
}
impl From<Pigeon52SigAnother> for u8 {
    #[inline(always)]
    fn from(val: Pigeon52SigAnother) -> u8 {
        Pigeon52SigAnother::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon52SigLogic {
    #[doc = "No logic operation"]
    DIS = 0x0,
    #[doc = "sigout = sig_another AND this_sig"]
    AND = 0x01,
    #[doc = "sigout = sig_another OR this_sig"]
    OR = 0x02,
    #[doc = "mask = sig_another AND other_masks"]
    MASK = 0x03,
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
impl Pigeon52SigLogic {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon52SigLogic {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon52SigLogic {
    #[inline(always)]
    fn from(val: u8) -> Pigeon52SigLogic {
        Pigeon52SigLogic::from_bits(val)
    }
}
impl From<Pigeon52SigLogic> for u8 {
    #[inline(always)]
    fn from(val: Pigeon52SigLogic) -> u8 {
        Pigeon52SigLogic::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon60IncSel {
    #[doc = "pclk"]
    PCLK = 0x0,
    #[doc = "Line start pulse"]
    LINE = 0x01,
    #[doc = "Frame start pulse"]
    FRAME = 0x02,
    #[doc = "Use another signal as tick event"]
    SIG_ANOTHER = 0x03,
}
impl Pigeon60IncSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon60IncSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon60IncSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon60IncSel {
        Pigeon60IncSel::from_bits(val)
    }
}
impl From<Pigeon60IncSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon60IncSel) -> u8 {
        Pigeon60IncSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon60MaskCntSel {
    #[doc = "pclk counter within one hscan state"]
    HSTATE_CNT = 0x0,
    #[doc = "pclk cycle within one hscan state"]
    HSTATE_CYCLE = 0x01,
    #[doc = "line counter within one vscan state"]
    VSTATE_CNT = 0x02,
    #[doc = "line cycle within one vscan state"]
    VSTATE_CYCLE = 0x03,
    #[doc = "frame counter"]
    FRAME_CNT = 0x04,
    #[doc = "frame cycle"]
    FRAME_CYCLE = 0x05,
    #[doc = "horizontal counter (pclk counter within one line )"]
    HCNT = 0x06,
    #[doc = "vertical counter (line counter within one frame)"]
    VCNT = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pigeon60MaskCntSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon60MaskCntSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon60MaskCntSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon60MaskCntSel {
        Pigeon60MaskCntSel::from_bits(val)
    }
}
impl From<Pigeon60MaskCntSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon60MaskCntSel) -> u8 {
        Pigeon60MaskCntSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon60Pol {
    #[doc = "Normal Signal (Active high)"]
    ACTIVE_HIGH = 0x0,
    #[doc = "Inverted signal (Active low)"]
    ACTIVE_LOW = 0x01,
}
impl Pigeon60Pol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon60Pol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon60Pol {
    #[inline(always)]
    fn from(val: u8) -> Pigeon60Pol {
        Pigeon60Pol::from_bits(val)
    }
}
impl From<Pigeon60Pol> for u8 {
    #[inline(always)]
    fn from(val: Pigeon60Pol) -> u8 {
        Pigeon60Pol::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon60StateMask(u8);
impl Pigeon60StateMask {
    #[doc = "FRAME SYNC"]
    pub const FS: Self = Self(0x01);
    #[doc = "FRAME BEGIN"]
    pub const FB: Self = Self(0x02);
    #[doc = "FRAME DATA"]
    pub const FD: Self = Self(0x04);
    #[doc = "FRAME END"]
    pub const FE: Self = Self(0x08);
    #[doc = "LINE SYNC"]
    pub const LS: Self = Self(0x10);
    #[doc = "LINE BEGIN"]
    pub const LB: Self = Self(0x20);
    #[doc = "LINE DATA"]
    pub const LD: Self = Self(0x40);
    #[doc = "LINE END"]
    pub const LE: Self = Self(0x80);
}
impl Pigeon60StateMask {
    pub const fn from_bits(val: u8) -> Pigeon60StateMask {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon60StateMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("FS"),
            0x02 => f.write_str("FB"),
            0x04 => f.write_str("FD"),
            0x08 => f.write_str("FE"),
            0x10 => f.write_str("LS"),
            0x20 => f.write_str("LB"),
            0x40 => f.write_str("LD"),
            0x80 => f.write_str("LE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon60StateMask {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "FS"),
            0x02 => defmt::write!(f, "FB"),
            0x04 => defmt::write!(f, "FD"),
            0x08 => defmt::write!(f, "FE"),
            0x10 => defmt::write!(f, "LS"),
            0x20 => defmt::write!(f, "LB"),
            0x40 => defmt::write!(f, "LD"),
            0x80 => defmt::write!(f, "LE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Pigeon60StateMask {
    #[inline(always)]
    fn from(val: u8) -> Pigeon60StateMask {
        Pigeon60StateMask::from_bits(val)
    }
}
impl From<Pigeon60StateMask> for u8 {
    #[inline(always)]
    fn from(val: Pigeon60StateMask) -> u8 {
        Pigeon60StateMask::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon61ClrCnt(u16);
impl Pigeon61ClrCnt {
    #[doc = "Keep active until mask off"]
    pub const CLEAR_USING_MASK: Self = Self(0x0);
}
impl Pigeon61ClrCnt {
    pub const fn from_bits(val: u16) -> Pigeon61ClrCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon61ClrCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("CLEAR_USING_MASK"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon61ClrCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "CLEAR_USING_MASK"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon61ClrCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon61ClrCnt {
        Pigeon61ClrCnt::from_bits(val)
    }
}
impl From<Pigeon61ClrCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon61ClrCnt) -> u16 {
        Pigeon61ClrCnt::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon61SetCnt(u16);
impl Pigeon61SetCnt {
    #[doc = "Start as active"]
    pub const START_ACTIVE: Self = Self(0x0);
}
impl Pigeon61SetCnt {
    pub const fn from_bits(val: u16) -> Pigeon61SetCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon61SetCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("START_ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon61SetCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "START_ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon61SetCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon61SetCnt {
        Pigeon61SetCnt::from_bits(val)
    }
}
impl From<Pigeon61SetCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon61SetCnt) -> u16 {
        Pigeon61SetCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon62SigAnother {
    #[doc = "Keep active until mask off"]
    CLEAR_USING_MASK = 0x0,
    _RESERVED_1 = 0x01,
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
    _RESERVED_1f = 0x1f,
}
impl Pigeon62SigAnother {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon62SigAnother {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon62SigAnother {
    #[inline(always)]
    fn from(val: u8) -> Pigeon62SigAnother {
        Pigeon62SigAnother::from_bits(val)
    }
}
impl From<Pigeon62SigAnother> for u8 {
    #[inline(always)]
    fn from(val: Pigeon62SigAnother) -> u8 {
        Pigeon62SigAnother::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon62SigLogic {
    #[doc = "No logic operation"]
    DIS = 0x0,
    #[doc = "sigout = sig_another AND this_sig"]
    AND = 0x01,
    #[doc = "sigout = sig_another OR this_sig"]
    OR = 0x02,
    #[doc = "mask = sig_another AND other_masks"]
    MASK = 0x03,
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
impl Pigeon62SigLogic {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon62SigLogic {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon62SigLogic {
    #[inline(always)]
    fn from(val: u8) -> Pigeon62SigLogic {
        Pigeon62SigLogic::from_bits(val)
    }
}
impl From<Pigeon62SigLogic> for u8 {
    #[inline(always)]
    fn from(val: Pigeon62SigLogic) -> u8 {
        Pigeon62SigLogic::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon70IncSel {
    #[doc = "pclk"]
    PCLK = 0x0,
    #[doc = "Line start pulse"]
    LINE = 0x01,
    #[doc = "Frame start pulse"]
    FRAME = 0x02,
    #[doc = "Use another signal as tick event"]
    SIG_ANOTHER = 0x03,
}
impl Pigeon70IncSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon70IncSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon70IncSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon70IncSel {
        Pigeon70IncSel::from_bits(val)
    }
}
impl From<Pigeon70IncSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon70IncSel) -> u8 {
        Pigeon70IncSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon70MaskCntSel {
    #[doc = "pclk counter within one hscan state"]
    HSTATE_CNT = 0x0,
    #[doc = "pclk cycle within one hscan state"]
    HSTATE_CYCLE = 0x01,
    #[doc = "line counter within one vscan state"]
    VSTATE_CNT = 0x02,
    #[doc = "line cycle within one vscan state"]
    VSTATE_CYCLE = 0x03,
    #[doc = "frame counter"]
    FRAME_CNT = 0x04,
    #[doc = "frame cycle"]
    FRAME_CYCLE = 0x05,
    #[doc = "horizontal counter (pclk counter within one line )"]
    HCNT = 0x06,
    #[doc = "vertical counter (line counter within one frame)"]
    VCNT = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pigeon70MaskCntSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon70MaskCntSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon70MaskCntSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon70MaskCntSel {
        Pigeon70MaskCntSel::from_bits(val)
    }
}
impl From<Pigeon70MaskCntSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon70MaskCntSel) -> u8 {
        Pigeon70MaskCntSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon70Pol {
    #[doc = "Normal Signal (Active high)"]
    ACTIVE_HIGH = 0x0,
    #[doc = "Inverted signal (Active low)"]
    ACTIVE_LOW = 0x01,
}
impl Pigeon70Pol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon70Pol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon70Pol {
    #[inline(always)]
    fn from(val: u8) -> Pigeon70Pol {
        Pigeon70Pol::from_bits(val)
    }
}
impl From<Pigeon70Pol> for u8 {
    #[inline(always)]
    fn from(val: Pigeon70Pol) -> u8 {
        Pigeon70Pol::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon70StateMask(u8);
impl Pigeon70StateMask {
    #[doc = "FRAME SYNC"]
    pub const FS: Self = Self(0x01);
    #[doc = "FRAME BEGIN"]
    pub const FB: Self = Self(0x02);
    #[doc = "FRAME DATA"]
    pub const FD: Self = Self(0x04);
    #[doc = "FRAME END"]
    pub const FE: Self = Self(0x08);
    #[doc = "LINE SYNC"]
    pub const LS: Self = Self(0x10);
    #[doc = "LINE BEGIN"]
    pub const LB: Self = Self(0x20);
    #[doc = "LINE DATA"]
    pub const LD: Self = Self(0x40);
    #[doc = "LINE END"]
    pub const LE: Self = Self(0x80);
}
impl Pigeon70StateMask {
    pub const fn from_bits(val: u8) -> Pigeon70StateMask {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon70StateMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("FS"),
            0x02 => f.write_str("FB"),
            0x04 => f.write_str("FD"),
            0x08 => f.write_str("FE"),
            0x10 => f.write_str("LS"),
            0x20 => f.write_str("LB"),
            0x40 => f.write_str("LD"),
            0x80 => f.write_str("LE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon70StateMask {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "FS"),
            0x02 => defmt::write!(f, "FB"),
            0x04 => defmt::write!(f, "FD"),
            0x08 => defmt::write!(f, "FE"),
            0x10 => defmt::write!(f, "LS"),
            0x20 => defmt::write!(f, "LB"),
            0x40 => defmt::write!(f, "LD"),
            0x80 => defmt::write!(f, "LE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Pigeon70StateMask {
    #[inline(always)]
    fn from(val: u8) -> Pigeon70StateMask {
        Pigeon70StateMask::from_bits(val)
    }
}
impl From<Pigeon70StateMask> for u8 {
    #[inline(always)]
    fn from(val: Pigeon70StateMask) -> u8 {
        Pigeon70StateMask::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon71ClrCnt(u16);
impl Pigeon71ClrCnt {
    #[doc = "Keep active until mask off"]
    pub const CLEAR_USING_MASK: Self = Self(0x0);
}
impl Pigeon71ClrCnt {
    pub const fn from_bits(val: u16) -> Pigeon71ClrCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon71ClrCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("CLEAR_USING_MASK"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon71ClrCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "CLEAR_USING_MASK"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon71ClrCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon71ClrCnt {
        Pigeon71ClrCnt::from_bits(val)
    }
}
impl From<Pigeon71ClrCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon71ClrCnt) -> u16 {
        Pigeon71ClrCnt::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon71SetCnt(u16);
impl Pigeon71SetCnt {
    #[doc = "Start as active"]
    pub const START_ACTIVE: Self = Self(0x0);
}
impl Pigeon71SetCnt {
    pub const fn from_bits(val: u16) -> Pigeon71SetCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon71SetCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("START_ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon71SetCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "START_ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon71SetCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon71SetCnt {
        Pigeon71SetCnt::from_bits(val)
    }
}
impl From<Pigeon71SetCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon71SetCnt) -> u16 {
        Pigeon71SetCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon72SigAnother {
    #[doc = "Keep active until mask off"]
    CLEAR_USING_MASK = 0x0,
    _RESERVED_1 = 0x01,
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
    _RESERVED_1f = 0x1f,
}
impl Pigeon72SigAnother {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon72SigAnother {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon72SigAnother {
    #[inline(always)]
    fn from(val: u8) -> Pigeon72SigAnother {
        Pigeon72SigAnother::from_bits(val)
    }
}
impl From<Pigeon72SigAnother> for u8 {
    #[inline(always)]
    fn from(val: Pigeon72SigAnother) -> u8 {
        Pigeon72SigAnother::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon72SigLogic {
    #[doc = "No logic operation"]
    DIS = 0x0,
    #[doc = "sigout = sig_another AND this_sig"]
    AND = 0x01,
    #[doc = "sigout = sig_another OR this_sig"]
    OR = 0x02,
    #[doc = "mask = sig_another AND other_masks"]
    MASK = 0x03,
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
impl Pigeon72SigLogic {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon72SigLogic {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon72SigLogic {
    #[inline(always)]
    fn from(val: u8) -> Pigeon72SigLogic {
        Pigeon72SigLogic::from_bits(val)
    }
}
impl From<Pigeon72SigLogic> for u8 {
    #[inline(always)]
    fn from(val: Pigeon72SigLogic) -> u8 {
        Pigeon72SigLogic::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon80IncSel {
    #[doc = "pclk"]
    PCLK = 0x0,
    #[doc = "Line start pulse"]
    LINE = 0x01,
    #[doc = "Frame start pulse"]
    FRAME = 0x02,
    #[doc = "Use another signal as tick event"]
    SIG_ANOTHER = 0x03,
}
impl Pigeon80IncSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon80IncSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon80IncSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon80IncSel {
        Pigeon80IncSel::from_bits(val)
    }
}
impl From<Pigeon80IncSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon80IncSel) -> u8 {
        Pigeon80IncSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon80MaskCntSel {
    #[doc = "pclk counter within one hscan state"]
    HSTATE_CNT = 0x0,
    #[doc = "pclk cycle within one hscan state"]
    HSTATE_CYCLE = 0x01,
    #[doc = "line counter within one vscan state"]
    VSTATE_CNT = 0x02,
    #[doc = "line cycle within one vscan state"]
    VSTATE_CYCLE = 0x03,
    #[doc = "frame counter"]
    FRAME_CNT = 0x04,
    #[doc = "frame cycle"]
    FRAME_CYCLE = 0x05,
    #[doc = "horizontal counter (pclk counter within one line )"]
    HCNT = 0x06,
    #[doc = "vertical counter (line counter within one frame)"]
    VCNT = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pigeon80MaskCntSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon80MaskCntSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon80MaskCntSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon80MaskCntSel {
        Pigeon80MaskCntSel::from_bits(val)
    }
}
impl From<Pigeon80MaskCntSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon80MaskCntSel) -> u8 {
        Pigeon80MaskCntSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon80Pol {
    #[doc = "Normal Signal (Active high)"]
    ACTIVE_HIGH = 0x0,
    #[doc = "Inverted signal (Active low)"]
    ACTIVE_LOW = 0x01,
}
impl Pigeon80Pol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon80Pol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon80Pol {
    #[inline(always)]
    fn from(val: u8) -> Pigeon80Pol {
        Pigeon80Pol::from_bits(val)
    }
}
impl From<Pigeon80Pol> for u8 {
    #[inline(always)]
    fn from(val: Pigeon80Pol) -> u8 {
        Pigeon80Pol::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon80StateMask(u8);
impl Pigeon80StateMask {
    #[doc = "FRAME SYNC"]
    pub const FS: Self = Self(0x01);
    #[doc = "FRAME BEGIN"]
    pub const FB: Self = Self(0x02);
    #[doc = "FRAME DATA"]
    pub const FD: Self = Self(0x04);
    #[doc = "FRAME END"]
    pub const FE: Self = Self(0x08);
    #[doc = "LINE SYNC"]
    pub const LS: Self = Self(0x10);
    #[doc = "LINE BEGIN"]
    pub const LB: Self = Self(0x20);
    #[doc = "LINE DATA"]
    pub const LD: Self = Self(0x40);
    #[doc = "LINE END"]
    pub const LE: Self = Self(0x80);
}
impl Pigeon80StateMask {
    pub const fn from_bits(val: u8) -> Pigeon80StateMask {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon80StateMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("FS"),
            0x02 => f.write_str("FB"),
            0x04 => f.write_str("FD"),
            0x08 => f.write_str("FE"),
            0x10 => f.write_str("LS"),
            0x20 => f.write_str("LB"),
            0x40 => f.write_str("LD"),
            0x80 => f.write_str("LE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon80StateMask {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "FS"),
            0x02 => defmt::write!(f, "FB"),
            0x04 => defmt::write!(f, "FD"),
            0x08 => defmt::write!(f, "FE"),
            0x10 => defmt::write!(f, "LS"),
            0x20 => defmt::write!(f, "LB"),
            0x40 => defmt::write!(f, "LD"),
            0x80 => defmt::write!(f, "LE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Pigeon80StateMask {
    #[inline(always)]
    fn from(val: u8) -> Pigeon80StateMask {
        Pigeon80StateMask::from_bits(val)
    }
}
impl From<Pigeon80StateMask> for u8 {
    #[inline(always)]
    fn from(val: Pigeon80StateMask) -> u8 {
        Pigeon80StateMask::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon81ClrCnt(u16);
impl Pigeon81ClrCnt {
    #[doc = "Keep active until mask off"]
    pub const CLEAR_USING_MASK: Self = Self(0x0);
}
impl Pigeon81ClrCnt {
    pub const fn from_bits(val: u16) -> Pigeon81ClrCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon81ClrCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("CLEAR_USING_MASK"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon81ClrCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "CLEAR_USING_MASK"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon81ClrCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon81ClrCnt {
        Pigeon81ClrCnt::from_bits(val)
    }
}
impl From<Pigeon81ClrCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon81ClrCnt) -> u16 {
        Pigeon81ClrCnt::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon81SetCnt(u16);
impl Pigeon81SetCnt {
    #[doc = "Start as active"]
    pub const START_ACTIVE: Self = Self(0x0);
}
impl Pigeon81SetCnt {
    pub const fn from_bits(val: u16) -> Pigeon81SetCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon81SetCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("START_ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon81SetCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "START_ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon81SetCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon81SetCnt {
        Pigeon81SetCnt::from_bits(val)
    }
}
impl From<Pigeon81SetCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon81SetCnt) -> u16 {
        Pigeon81SetCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon82SigAnother {
    #[doc = "Keep active until mask off"]
    CLEAR_USING_MASK = 0x0,
    _RESERVED_1 = 0x01,
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
    _RESERVED_1f = 0x1f,
}
impl Pigeon82SigAnother {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon82SigAnother {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon82SigAnother {
    #[inline(always)]
    fn from(val: u8) -> Pigeon82SigAnother {
        Pigeon82SigAnother::from_bits(val)
    }
}
impl From<Pigeon82SigAnother> for u8 {
    #[inline(always)]
    fn from(val: Pigeon82SigAnother) -> u8 {
        Pigeon82SigAnother::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon82SigLogic {
    #[doc = "No logic operation"]
    DIS = 0x0,
    #[doc = "sigout = sig_another AND this_sig"]
    AND = 0x01,
    #[doc = "sigout = sig_another OR this_sig"]
    OR = 0x02,
    #[doc = "mask = sig_another AND other_masks"]
    MASK = 0x03,
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
impl Pigeon82SigLogic {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon82SigLogic {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon82SigLogic {
    #[inline(always)]
    fn from(val: u8) -> Pigeon82SigLogic {
        Pigeon82SigLogic::from_bits(val)
    }
}
impl From<Pigeon82SigLogic> for u8 {
    #[inline(always)]
    fn from(val: Pigeon82SigLogic) -> u8 {
        Pigeon82SigLogic::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon90IncSel {
    #[doc = "pclk"]
    PCLK = 0x0,
    #[doc = "Line start pulse"]
    LINE = 0x01,
    #[doc = "Frame start pulse"]
    FRAME = 0x02,
    #[doc = "Use another signal as tick event"]
    SIG_ANOTHER = 0x03,
}
impl Pigeon90IncSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon90IncSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon90IncSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon90IncSel {
        Pigeon90IncSel::from_bits(val)
    }
}
impl From<Pigeon90IncSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon90IncSel) -> u8 {
        Pigeon90IncSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon90MaskCntSel {
    #[doc = "pclk counter within one hscan state"]
    HSTATE_CNT = 0x0,
    #[doc = "pclk cycle within one hscan state"]
    HSTATE_CYCLE = 0x01,
    #[doc = "line counter within one vscan state"]
    VSTATE_CNT = 0x02,
    #[doc = "line cycle within one vscan state"]
    VSTATE_CYCLE = 0x03,
    #[doc = "frame counter"]
    FRAME_CNT = 0x04,
    #[doc = "frame cycle"]
    FRAME_CYCLE = 0x05,
    #[doc = "horizontal counter (pclk counter within one line )"]
    HCNT = 0x06,
    #[doc = "vertical counter (line counter within one frame)"]
    VCNT = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Pigeon90MaskCntSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon90MaskCntSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon90MaskCntSel {
    #[inline(always)]
    fn from(val: u8) -> Pigeon90MaskCntSel {
        Pigeon90MaskCntSel::from_bits(val)
    }
}
impl From<Pigeon90MaskCntSel> for u8 {
    #[inline(always)]
    fn from(val: Pigeon90MaskCntSel) -> u8 {
        Pigeon90MaskCntSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon90Pol {
    #[doc = "Normal Signal (Active high)"]
    ACTIVE_HIGH = 0x0,
    #[doc = "Inverted signal (Active low)"]
    ACTIVE_LOW = 0x01,
}
impl Pigeon90Pol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon90Pol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon90Pol {
    #[inline(always)]
    fn from(val: u8) -> Pigeon90Pol {
        Pigeon90Pol::from_bits(val)
    }
}
impl From<Pigeon90Pol> for u8 {
    #[inline(always)]
    fn from(val: Pigeon90Pol) -> u8 {
        Pigeon90Pol::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon90StateMask(u8);
impl Pigeon90StateMask {
    #[doc = "FRAME SYNC"]
    pub const FS: Self = Self(0x01);
    #[doc = "FRAME BEGIN"]
    pub const FB: Self = Self(0x02);
    #[doc = "FRAME DATA"]
    pub const FD: Self = Self(0x04);
    #[doc = "FRAME END"]
    pub const FE: Self = Self(0x08);
    #[doc = "LINE SYNC"]
    pub const LS: Self = Self(0x10);
    #[doc = "LINE BEGIN"]
    pub const LB: Self = Self(0x20);
    #[doc = "LINE DATA"]
    pub const LD: Self = Self(0x40);
    #[doc = "LINE END"]
    pub const LE: Self = Self(0x80);
}
impl Pigeon90StateMask {
    pub const fn from_bits(val: u8) -> Pigeon90StateMask {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon90StateMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("FS"),
            0x02 => f.write_str("FB"),
            0x04 => f.write_str("FD"),
            0x08 => f.write_str("FE"),
            0x10 => f.write_str("LS"),
            0x20 => f.write_str("LB"),
            0x40 => f.write_str("LD"),
            0x80 => f.write_str("LE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon90StateMask {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "FS"),
            0x02 => defmt::write!(f, "FB"),
            0x04 => defmt::write!(f, "FD"),
            0x08 => defmt::write!(f, "FE"),
            0x10 => defmt::write!(f, "LS"),
            0x20 => defmt::write!(f, "LB"),
            0x40 => defmt::write!(f, "LD"),
            0x80 => defmt::write!(f, "LE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Pigeon90StateMask {
    #[inline(always)]
    fn from(val: u8) -> Pigeon90StateMask {
        Pigeon90StateMask::from_bits(val)
    }
}
impl From<Pigeon90StateMask> for u8 {
    #[inline(always)]
    fn from(val: Pigeon90StateMask) -> u8 {
        Pigeon90StateMask::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon91ClrCnt(u16);
impl Pigeon91ClrCnt {
    #[doc = "Keep active until mask off"]
    pub const CLEAR_USING_MASK: Self = Self(0x0);
}
impl Pigeon91ClrCnt {
    pub const fn from_bits(val: u16) -> Pigeon91ClrCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon91ClrCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("CLEAR_USING_MASK"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon91ClrCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "CLEAR_USING_MASK"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon91ClrCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon91ClrCnt {
        Pigeon91ClrCnt::from_bits(val)
    }
}
impl From<Pigeon91ClrCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon91ClrCnt) -> u16 {
        Pigeon91ClrCnt::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pigeon91SetCnt(u16);
impl Pigeon91SetCnt {
    #[doc = "Start as active"]
    pub const START_ACTIVE: Self = Self(0x0);
}
impl Pigeon91SetCnt {
    pub const fn from_bits(val: u16) -> Pigeon91SetCnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Pigeon91SetCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("START_ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pigeon91SetCnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "START_ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Pigeon91SetCnt {
    #[inline(always)]
    fn from(val: u16) -> Pigeon91SetCnt {
        Pigeon91SetCnt::from_bits(val)
    }
}
impl From<Pigeon91SetCnt> for u16 {
    #[inline(always)]
    fn from(val: Pigeon91SetCnt) -> u16 {
        Pigeon91SetCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon92SigAnother {
    #[doc = "Keep active until mask off"]
    CLEAR_USING_MASK = 0x0,
    _RESERVED_1 = 0x01,
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
    _RESERVED_1f = 0x1f,
}
impl Pigeon92SigAnother {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon92SigAnother {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon92SigAnother {
    #[inline(always)]
    fn from(val: u8) -> Pigeon92SigAnother {
        Pigeon92SigAnother::from_bits(val)
    }
}
impl From<Pigeon92SigAnother> for u8 {
    #[inline(always)]
    fn from(val: Pigeon92SigAnother) -> u8 {
        Pigeon92SigAnother::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pigeon92SigLogic {
    #[doc = "No logic operation"]
    DIS = 0x0,
    #[doc = "sigout = sig_another AND this_sig"]
    AND = 0x01,
    #[doc = "sigout = sig_another OR this_sig"]
    OR = 0x02,
    #[doc = "mask = sig_another AND other_masks"]
    MASK = 0x03,
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
impl Pigeon92SigLogic {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pigeon92SigLogic {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pigeon92SigLogic {
    #[inline(always)]
    fn from(val: u8) -> Pigeon92SigLogic {
        Pigeon92SigLogic::from_bits(val)
    }
}
impl From<Pigeon92SigLogic> for u8 {
    #[inline(always)]
    fn from(val: Pigeon92SigLogic) -> u8 {
        Pigeon92SigLogic::to_bits(val)
    }
}
