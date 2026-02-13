#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpha {
    #[doc = "Change. The SPI captures serial data on the first clock transition of the transfer (when the clock changes away from the rest state). Data is changed on the following edge."]
    CHANGE = 0x0,
    #[doc = "Capture. The SPI changes serial data on the first clock transition of the transfer (when the clock changes away from the rest state). Data is captured on the following edge."]
    CAPTURE = 0x01,
}
impl Cpha {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpha {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpha {
    #[inline(always)]
    fn from(val: u8) -> Cpha {
        Cpha::from_bits(val)
    }
}
impl From<Cpha> for u8 {
    #[inline(always)]
    fn from(val: Cpha) -> u8 {
        Cpha::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpol {
    #[doc = "Low. The rest state of the clock (between transfers) is low."]
    LOW = 0x0,
    #[doc = "High. The rest state of the clock (between transfers) is high."]
    HIGH = 0x01,
}
impl Cpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpol {
    #[inline(always)]
    fn from(val: u8) -> Cpol {
        Cpol::from_bits(val)
    }
}
impl From<Cpol> for u8 {
    #[inline(always)]
    fn from(val: Cpol) -> u8 {
        Cpol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lsbf {
    #[doc = "Standard. Data is transmitted and received in standard MSB first order."]
    STANDARD = 0x0,
    #[doc = "Reverse. Data is transmitted and received in reverse order (LSB first)."]
    REVERSE = 0x01,
}
impl Lsbf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lsbf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lsbf {
    #[inline(always)]
    fn from(val: u8) -> Lsbf {
        Lsbf::from_bits(val)
    }
}
impl From<Lsbf> for u8 {
    #[inline(always)]
    fn from(val: Lsbf) -> u8 {
        Lsbf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Master {
    #[doc = "Slave mode. The SPI will operate in slave mode. SCK, MOSI, and the SSEL signals are inputs, MISO is an output."]
    SLAVE_MODE = 0x0,
    #[doc = "Master mode. The SPI will operate in master mode. SCK, MOSI, and the SSEL signals are outputs, MISO is an input."]
    MASTER_MODE = 0x01,
}
impl Master {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Master {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Master {
    #[inline(always)]
    fn from(val: u8) -> Master {
        Master::from_bits(val)
    }
}
impl From<Master> for u8 {
    #[inline(always)]
    fn from(val: Master) -> u8 {
        Master::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxignore {
    #[doc = "Read received data. Received data must be read in order to allow transmission to progress. SPI transmit will halt when the receive data FIFO is full. In slave mode, an overrun error will occur if received data is not read before new data is received."]
    READ = 0x0,
    #[doc = "Ignore received data. Received data is ignored, allowing transmission without reading unneeded received data. No receiver flags are generated."]
    IGNORE = 0x01,
}
impl Rxignore {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxignore {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxignore {
    #[inline(always)]
    fn from(val: u8) -> Rxignore {
        Rxignore::from_bits(val)
    }
}
impl From<Rxignore> for u8 {
    #[inline(always)]
    fn from(val: Rxignore) -> u8 {
        Rxignore::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spol0 {
    #[doc = "Low. The SSEL0 pin is active low."]
    LOW = 0x0,
    #[doc = "High. The SSEL0 pin is active high."]
    HIGH = 0x01,
}
impl Spol0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spol0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spol0 {
    #[inline(always)]
    fn from(val: u8) -> Spol0 {
        Spol0::from_bits(val)
    }
}
impl From<Spol0> for u8 {
    #[inline(always)]
    fn from(val: Spol0) -> u8 {
        Spol0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spol1 {
    #[doc = "Low. The SSEL1 pin is active low."]
    LOW = 0x0,
    #[doc = "High. The SSEL1 pin is active high."]
    HIGH = 0x01,
}
impl Spol1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spol1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spol1 {
    #[inline(always)]
    fn from(val: u8) -> Spol1 {
        Spol1::from_bits(val)
    }
}
impl From<Spol1> for u8 {
    #[inline(always)]
    fn from(val: Spol1) -> u8 {
        Spol1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spol2 {
    #[doc = "Low. The SSEL2 pin is active low."]
    LOW = 0x0,
    #[doc = "High. The SSEL2 pin is active high."]
    HIGH = 0x01,
}
impl Spol2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spol2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spol2 {
    #[inline(always)]
    fn from(val: u8) -> Spol2 {
        Spol2::from_bits(val)
    }
}
impl From<Spol2> for u8 {
    #[inline(always)]
    fn from(val: Spol2) -> u8 {
        Spol2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spol3 {
    #[doc = "Low. The SSEL3 pin is active low."]
    LOW = 0x0,
    #[doc = "High. The SSEL3 pin is active high."]
    HIGH = 0x01,
}
impl Spol3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spol3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spol3 {
    #[inline(always)]
    fn from(val: u8) -> Spol3 {
        Spol3::from_bits(val)
    }
}
impl From<Spol3> for u8 {
    #[inline(always)]
    fn from(val: Spol3) -> u8 {
        Spol3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txssel0N {
    #[doc = "SSEL0 asserted."]
    ASSERTED = 0x0,
    #[doc = "SSEL0 not asserted."]
    NOT_ASSERTED = 0x01,
}
impl Txssel0N {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txssel0N {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txssel0N {
    #[inline(always)]
    fn from(val: u8) -> Txssel0N {
        Txssel0N::from_bits(val)
    }
}
impl From<Txssel0N> for u8 {
    #[inline(always)]
    fn from(val: Txssel0N) -> u8 {
        Txssel0N::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txssel1N {
    #[doc = "SSEL1 asserted."]
    ASSERTED = 0x0,
    #[doc = "SSEL1 not asserted."]
    NOT_ASSERTED = 0x01,
}
impl Txssel1N {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txssel1N {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txssel1N {
    #[inline(always)]
    fn from(val: u8) -> Txssel1N {
        Txssel1N::from_bits(val)
    }
}
impl From<Txssel1N> for u8 {
    #[inline(always)]
    fn from(val: Txssel1N) -> u8 {
        Txssel1N::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txssel2N {
    #[doc = "SSEL2 asserted."]
    ASSERTED = 0x0,
    #[doc = "SSEL2 not asserted."]
    NOT_ASSERTED = 0x01,
}
impl Txssel2N {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txssel2N {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txssel2N {
    #[inline(always)]
    fn from(val: u8) -> Txssel2N {
        Txssel2N::from_bits(val)
    }
}
impl From<Txssel2N> for u8 {
    #[inline(always)]
    fn from(val: Txssel2N) -> u8 {
        Txssel2N::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txssel3N {
    #[doc = "SSEL3 asserted."]
    ASSERTED = 0x0,
    #[doc = "SSEL3 not asserted."]
    NOT_ASSERTED = 0x01,
}
impl Txssel3N {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txssel3N {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txssel3N {
    #[inline(always)]
    fn from(val: u8) -> Txssel3N {
        Txssel3N::from_bits(val)
    }
}
impl From<Txssel3N> for u8 {
    #[inline(always)]
    fn from(val: Txssel3N) -> u8 {
        Txssel3N::to_bits(val)
    }
}
