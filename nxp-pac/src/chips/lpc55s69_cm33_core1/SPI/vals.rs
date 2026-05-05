#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CPHA {
    #[doc = "Change. The SPI captures serial data on the first clock transition of the transfer (when the clock changes away from the rest state). Data is changed on the following edge."]
    CHANGE = 0x0,
    #[doc = "Capture. The SPI changes serial data on the first clock transition of the transfer (when the clock changes away from the rest state). Data is captured on the following edge."]
    CAPTURE = 0x01,
}
impl CPHA {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CPHA {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CPHA {
    #[inline(always)]
    fn from(val: u8) -> CPHA {
        CPHA::from_bits(val)
    }
}
impl From<CPHA> for u8 {
    #[inline(always)]
    fn from(val: CPHA) -> u8 {
        CPHA::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CPOL {
    #[doc = "Low. The rest state of the clock (between transfers) is low."]
    LOW = 0x0,
    #[doc = "High. The rest state of the clock (between transfers) is high."]
    HIGH = 0x01,
}
impl CPOL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CPOL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CPOL {
    #[inline(always)]
    fn from(val: u8) -> CPOL {
        CPOL::from_bits(val)
    }
}
impl From<CPOL> for u8 {
    #[inline(always)]
    fn from(val: CPOL) -> u8 {
        CPOL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LSBF {
    #[doc = "Standard. Data is transmitted and received in standard MSB first order."]
    STANDARD = 0x0,
    #[doc = "Reverse. Data is transmitted and received in reverse order (LSB first)."]
    REVERSE = 0x01,
}
impl LSBF {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LSBF {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LSBF {
    #[inline(always)]
    fn from(val: u8) -> LSBF {
        LSBF::from_bits(val)
    }
}
impl From<LSBF> for u8 {
    #[inline(always)]
    fn from(val: LSBF) -> u8 {
        LSBF::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MASTER {
    #[doc = "Slave mode. The SPI will operate in slave mode. SCK, MOSI, and the SSEL signals are inputs, MISO is an output."]
    SLAVE_MODE = 0x0,
    #[doc = "Master mode. The SPI will operate in master mode. SCK, MOSI, and the SSEL signals are outputs, MISO is an input."]
    MASTER_MODE = 0x01,
}
impl MASTER {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MASTER {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MASTER {
    #[inline(always)]
    fn from(val: u8) -> MASTER {
        MASTER::from_bits(val)
    }
}
impl From<MASTER> for u8 {
    #[inline(always)]
    fn from(val: MASTER) -> u8 {
        MASTER::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RXIGNORE {
    #[doc = "Read received data. Received data must be read in order to allow transmission to progress. SPI transmit will halt when the receive data FIFO is full. In slave mode, an overrun error will occur if received data is not read before new data is received."]
    READ = 0x0,
    #[doc = "Ignore received data. Received data is ignored, allowing transmission without reading unneeded received data. No receiver flags are generated."]
    IGNORE = 0x01,
}
impl RXIGNORE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RXIGNORE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RXIGNORE {
    #[inline(always)]
    fn from(val: u8) -> RXIGNORE {
        RXIGNORE::from_bits(val)
    }
}
impl From<RXIGNORE> for u8 {
    #[inline(always)]
    fn from(val: RXIGNORE) -> u8 {
        RXIGNORE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SPOL0 {
    #[doc = "Low. The SSEL0 pin is active low."]
    LOW = 0x0,
    #[doc = "High. The SSEL0 pin is active high."]
    HIGH = 0x01,
}
impl SPOL0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SPOL0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SPOL0 {
    #[inline(always)]
    fn from(val: u8) -> SPOL0 {
        SPOL0::from_bits(val)
    }
}
impl From<SPOL0> for u8 {
    #[inline(always)]
    fn from(val: SPOL0) -> u8 {
        SPOL0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SPOL1 {
    #[doc = "Low. The SSEL1 pin is active low."]
    LOW = 0x0,
    #[doc = "High. The SSEL1 pin is active high."]
    HIGH = 0x01,
}
impl SPOL1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SPOL1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SPOL1 {
    #[inline(always)]
    fn from(val: u8) -> SPOL1 {
        SPOL1::from_bits(val)
    }
}
impl From<SPOL1> for u8 {
    #[inline(always)]
    fn from(val: SPOL1) -> u8 {
        SPOL1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SPOL2 {
    #[doc = "Low. The SSEL2 pin is active low."]
    LOW = 0x0,
    #[doc = "High. The SSEL2 pin is active high."]
    HIGH = 0x01,
}
impl SPOL2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SPOL2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SPOL2 {
    #[inline(always)]
    fn from(val: u8) -> SPOL2 {
        SPOL2::from_bits(val)
    }
}
impl From<SPOL2> for u8 {
    #[inline(always)]
    fn from(val: SPOL2) -> u8 {
        SPOL2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SPOL3 {
    #[doc = "Low. The SSEL3 pin is active low."]
    LOW = 0x0,
    #[doc = "High. The SSEL3 pin is active high."]
    HIGH = 0x01,
}
impl SPOL3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SPOL3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SPOL3 {
    #[inline(always)]
    fn from(val: u8) -> SPOL3 {
        SPOL3::from_bits(val)
    }
}
impl From<SPOL3> for u8 {
    #[inline(always)]
    fn from(val: SPOL3) -> u8 {
        SPOL3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TXSSEL0_N {
    #[doc = "SSEL0 asserted."]
    ASSERTED = 0x0,
    #[doc = "SSEL0 not asserted."]
    NOT_ASSERTED = 0x01,
}
impl TXSSEL0_N {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TXSSEL0_N {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TXSSEL0_N {
    #[inline(always)]
    fn from(val: u8) -> TXSSEL0_N {
        TXSSEL0_N::from_bits(val)
    }
}
impl From<TXSSEL0_N> for u8 {
    #[inline(always)]
    fn from(val: TXSSEL0_N) -> u8 {
        TXSSEL0_N::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TXSSEL1_N {
    #[doc = "SSEL1 asserted."]
    ASSERTED = 0x0,
    #[doc = "SSEL1 not asserted."]
    NOT_ASSERTED = 0x01,
}
impl TXSSEL1_N {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TXSSEL1_N {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TXSSEL1_N {
    #[inline(always)]
    fn from(val: u8) -> TXSSEL1_N {
        TXSSEL1_N::from_bits(val)
    }
}
impl From<TXSSEL1_N> for u8 {
    #[inline(always)]
    fn from(val: TXSSEL1_N) -> u8 {
        TXSSEL1_N::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TXSSEL2_N {
    #[doc = "SSEL2 asserted."]
    ASSERTED = 0x0,
    #[doc = "SSEL2 not asserted."]
    NOT_ASSERTED = 0x01,
}
impl TXSSEL2_N {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TXSSEL2_N {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TXSSEL2_N {
    #[inline(always)]
    fn from(val: u8) -> TXSSEL2_N {
        TXSSEL2_N::from_bits(val)
    }
}
impl From<TXSSEL2_N> for u8 {
    #[inline(always)]
    fn from(val: TXSSEL2_N) -> u8 {
        TXSSEL2_N::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TXSSEL3_N {
    #[doc = "SSEL3 asserted."]
    ASSERTED = 0x0,
    #[doc = "SSEL3 not asserted."]
    NOT_ASSERTED = 0x01,
}
impl TXSSEL3_N {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TXSSEL3_N {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TXSSEL3_N {
    #[inline(always)]
    fn from(val: u8) -> TXSSEL3_N {
        TXSSEL3_N::from_bits(val)
    }
}
impl From<TXSSEL3_N> for u8 {
    #[inline(always)]
    fn from(val: TXSSEL3_N) -> u8 {
        TXSSEL3_N::to_bits(val)
    }
}
