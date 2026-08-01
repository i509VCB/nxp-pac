#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "LP_FLEXCOMM."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpFlexcomm {
    ptr: *mut u8,
}
unsafe impl Send for LpFlexcomm {}
unsafe impl Sync for LpFlexcomm {}
impl LpFlexcomm {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Interrupt Status."]
    #[inline(always)]
    pub const fn istat(self) -> crate::pac::common::Reg<Istat, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff4usize) as _) }
    }
    #[doc = "Peripheral Select and ID."]
    #[inline(always)]
    pub const fn pselid(self) -> crate::pac::common::Reg<Pselid, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff8usize) as _) }
    }
}
#[doc = "Interrupt Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Istat(pub u32);
impl Istat {
    #[doc = "UART TX Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn uarttx(&self) -> Uarttx {
        let val = (self.0 >> 0usize) & 0x01;
        Uarttx::from_bits(val as u8)
    }
    #[doc = "UART TX Interrupt."]
    #[inline(always)]
    pub const fn set_uarttx(&mut self, val: Uarttx) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "UART RX Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn uartrx(&self) -> Uartrx {
        let val = (self.0 >> 1usize) & 0x01;
        Uartrx::from_bits(val as u8)
    }
    #[doc = "UART RX Interrupt."]
    #[inline(always)]
    pub const fn set_uartrx(&mut self, val: Uartrx) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "SPI Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn spi(&self) -> Spi {
        let val = (self.0 >> 2usize) & 0x01;
        Spi::from_bits(val as u8)
    }
    #[doc = "SPI Interrupt."]
    #[inline(always)]
    pub const fn set_spi(&mut self, val: Spi) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "I2C Controller Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn i2cm(&self) -> I2cm {
        let val = (self.0 >> 4usize) & 0x01;
        I2cm::from_bits(val as u8)
    }
    #[doc = "I2C Controller Interrupt."]
    #[inline(always)]
    pub const fn set_i2cm(&mut self, val: I2cm) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "I2C Subordinate Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn i2cs(&self) -> I2cs {
        let val = (self.0 >> 5usize) & 0x01;
        I2cs::from_bits(val as u8)
    }
    #[doc = "I2C Subordinate Interrupt."]
    #[inline(always)]
    pub const fn set_i2cs(&mut self, val: I2cs) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
}
impl Default for Istat {
    #[inline(always)]
    fn default() -> Istat {
        Istat(0)
    }
}
impl core::fmt::Debug for Istat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Istat")
            .field("uarttx", &self.uarttx())
            .field("uartrx", &self.uartrx())
            .field("spi", &self.spi())
            .field("i2cm", &self.i2cm())
            .field("i2cs", &self.i2cs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Istat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Istat {{ uarttx: {:?}, uartrx: {:?}, spi: {:?}, i2cm: {:?}, i2cs: {:?} }}",
            self.uarttx(),
            self.uartrx(),
            self.spi(),
            self.i2cm(),
            self.i2cs()
        )
    }
}
#[doc = "Peripheral Select and ID."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pselid(pub u32);
impl Pselid {
    #[doc = "Peripheral Select."]
    #[must_use]
    #[inline(always)]
    pub const fn persel(&self) -> Persel {
        let val = (self.0 >> 0usize) & 0x07;
        Persel::from_bits(val as u8)
    }
    #[doc = "Peripheral Select."]
    #[inline(always)]
    pub const fn set_persel(&mut self, val: Persel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Lock."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "UART Present."]
    #[must_use]
    #[inline(always)]
    pub const fn uartpresent(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "UART Present."]
    #[inline(always)]
    pub const fn set_uartpresent(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "SPI Present."]
    #[must_use]
    #[inline(always)]
    pub const fn spipresent(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "SPI Present."]
    #[inline(always)]
    pub const fn set_spipresent(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "I2C Present."]
    #[must_use]
    #[inline(always)]
    pub const fn i2cpresent(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "I2C Present."]
    #[inline(always)]
    pub const fn set_i2cpresent(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "LP_FLEXCOMM interface ID."]
    #[must_use]
    #[inline(always)]
    pub const fn id(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "LP_FLEXCOMM interface ID."]
    #[inline(always)]
    pub const fn set_id(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Pselid {
    #[inline(always)]
    fn default() -> Pselid {
        Pselid(0)
    }
}
impl core::fmt::Debug for Pselid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pselid")
            .field("persel", &self.persel())
            .field("lock", &self.lock())
            .field("uartpresent", &self.uartpresent())
            .field("spipresent", &self.spipresent())
            .field("i2cpresent", &self.i2cpresent())
            .field("id", &self.id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pselid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pselid {{ persel: {:?}, lock: {=bool:?}, uartpresent: {=bool:?}, spipresent: {=bool:?}, i2cpresent: {=bool:?}, id: {=u32:?} }}",
            self.persel(),
            self.lock(),
            self.uartpresent(),
            self.spipresent(),
            self.i2cpresent(),
            self.id()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I2cm {
    #[doc = "Clear."]
    Clr = 0x0,
    #[doc = "Set."]
    Set = 0x01,
}
impl I2cm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I2cm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I2cm {
    #[inline(always)]
    fn from(val: u8) -> I2cm {
        I2cm::from_bits(val)
    }
}
impl From<I2cm> for u8 {
    #[inline(always)]
    fn from(val: I2cm) -> u8 {
        I2cm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I2cs {
    #[doc = "Clear."]
    Clr = 0x0,
    #[doc = "Set."]
    Set = 0x01,
}
impl I2cs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I2cs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I2cs {
    #[inline(always)]
    fn from(val: u8) -> I2cs {
        I2cs::from_bits(val)
    }
}
impl From<I2cs> for u8 {
    #[inline(always)]
    fn from(val: I2cs) -> u8 {
        I2cs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Persel {
    #[doc = "No peripheral selected."]
    None = 0x0,
    #[doc = "UART."]
    Uart = 0x01,
    #[doc = "SPI."]
    Spi = 0x02,
    #[doc = "I2C."]
    I2c = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "UART and I2C."]
    Uarti2c = 0x07,
}
impl Persel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Persel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Persel {
    #[inline(always)]
    fn from(val: u8) -> Persel {
        Persel::from_bits(val)
    }
}
impl From<Persel> for u8 {
    #[inline(always)]
    fn from(val: Persel) -> u8 {
        Persel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spi {
    #[doc = "Clear."]
    Clr = 0x0,
    #[doc = "Set."]
    Set = 0x01,
}
impl Spi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spi {
    #[inline(always)]
    fn from(val: u8) -> Spi {
        Spi::from_bits(val)
    }
}
impl From<Spi> for u8 {
    #[inline(always)]
    fn from(val: Spi) -> u8 {
        Spi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Uartrx {
    #[doc = "Clear."]
    Clr = 0x0,
    #[doc = "Set."]
    Set = 0x01,
}
impl Uartrx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Uartrx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Uartrx {
    #[inline(always)]
    fn from(val: u8) -> Uartrx {
        Uartrx::from_bits(val)
    }
}
impl From<Uartrx> for u8 {
    #[inline(always)]
    fn from(val: Uartrx) -> u8 {
        Uartrx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Uarttx {
    #[doc = "Clear."]
    Clr = 0x0,
    #[doc = "Set."]
    Set = 0x01,
}
impl Uarttx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Uarttx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Uarttx {
    #[inline(always)]
    fn from(val: u8) -> Uarttx {
        Uarttx::from_bits(val)
    }
}
impl From<Uarttx> for u8 {
    #[inline(always)]
    fn from(val: Uarttx) -> u8 {
        Uarttx::to_bits(val)
    }
}
