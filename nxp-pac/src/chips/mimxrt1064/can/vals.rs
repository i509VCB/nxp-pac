#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ackerr {
    #[doc = "No such occurrence."]
    Ackerr0 = 0x0,
    #[doc = "An ACK error occurred since last read of this register."]
    Ackerr1 = 0x01,
}
impl Ackerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ackerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ackerr {
    #[inline(always)]
    fn from(val: u8) -> Ackerr {
        Ackerr::from_bits(val)
    }
}
impl From<Ackerr> for u8 {
    #[inline(always)]
    fn from(val: Ackerr) -> u8 {
        Ackerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aen {
    #[doc = "Abort disabled."]
    Aen0 = 0x0,
    #[doc = "Abort enabled."]
    Aen1 = 0x01,
}
impl Aen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aen {
    #[inline(always)]
    fn from(val: u8) -> Aen {
        Aen::from_bits(val)
    }
}
impl From<Aen> for u8 {
    #[inline(always)]
    fn from(val: Aen) -> u8 {
        Aen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum App {
    #[doc = "No matching process ongoing."]
    App0 = 0x0,
    #[doc = "Matching process is in progress."]
    App1 = 0x01,
}
impl App {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> App {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for App {
    #[inline(always)]
    fn from(val: u8) -> App {
        App::from_bits(val)
    }
}
impl From<App> for u8 {
    #[inline(always)]
    fn from(val: App) -> u8 {
        App::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bit0err {
    #[doc = "No such occurrence."]
    Bit0err0 = 0x0,
    #[doc = "At least one bit sent as dominant is received as recessive."]
    Bit0err1 = 0x01,
}
impl Bit0err {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bit0err {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bit0err {
    #[inline(always)]
    fn from(val: u8) -> Bit0err {
        Bit0err::from_bits(val)
    }
}
impl From<Bit0err> for u8 {
    #[inline(always)]
    fn from(val: Bit0err) -> u8 {
        Bit0err::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bit1err {
    #[doc = "No such occurrence."]
    Bit1err0 = 0x0,
    #[doc = "At least one bit sent as recessive is received as dominant."]
    Bit1err1 = 0x01,
}
impl Bit1err {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bit1err {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bit1err {
    #[inline(always)]
    fn from(val: u8) -> Bit1err {
        Bit1err::from_bits(val)
    }
}
impl From<Bit1err> for u8 {
    #[inline(always)]
    fn from(val: Bit1err) -> u8 {
        Bit1err::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Boffint {
    #[doc = "No such occurrence."]
    Boffint0 = 0x0,
    #[doc = "FLEXCAN module entered 'Bus Off' state."]
    Boffint1 = 0x01,
}
impl Boffint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Boffint {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Boffint {
    #[inline(always)]
    fn from(val: u8) -> Boffint {
        Boffint::from_bits(val)
    }
}
impl From<Boffint> for u8 {
    #[inline(always)]
    fn from(val: Boffint) -> u8 {
        Boffint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Boffmsk {
    #[doc = "Bus Off interrupt disabled."]
    Boffmsk0 = 0x0,
    #[doc = "Bus Off interrupt enabled."]
    Boffmsk1 = 0x01,
}
impl Boffmsk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Boffmsk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Boffmsk {
    #[inline(always)]
    fn from(val: u8) -> Boffmsk {
        Boffmsk::from_bits(val)
    }
}
impl From<Boffmsk> for u8 {
    #[inline(always)]
    fn from(val: Boffmsk) -> u8 {
        Boffmsk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Boffrec {
    #[doc = "Automatic recovering from Bus Off state enabled, according to CAN Spec 2.0 part B."]
    Boffrec0 = 0x0,
    #[doc = "Automatic recovering from Bus Off state disabled."]
    Boffrec1 = 0x01,
}
impl Boffrec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Boffrec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Boffrec {
    #[inline(always)]
    fn from(val: u8) -> Boffrec {
        Boffrec::from_bits(val)
    }
}
impl From<Boffrec> for u8 {
    #[inline(always)]
    fn from(val: Boffrec) -> u8 {
        Boffrec::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Buf31to8i(u32);
impl Buf31to8i {
    #[doc = "No such occurrence."]
    pub const Buf31to8i0: Self = Self(0x0);
    #[doc = "The corresponding MB has successfully completed transmission or reception."]
    pub const Buf31to8i1: Self = Self(0x01);
}
impl Buf31to8i {
    pub const fn from_bits(val: u32) -> Buf31to8i {
        Self(val & 0x00ff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Buf31to8i {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Buf31to8i0"),
            0x01 => f.write_str("Buf31to8i1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Buf31to8i {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Buf31to8i0"),
            0x01 => defmt::write!(f, "Buf31to8i1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Buf31to8i {
    #[inline(always)]
    fn from(val: u32) -> Buf31to8i {
        Buf31to8i::from_bits(val)
    }
}
impl From<Buf31to8i> for u32 {
    #[inline(always)]
    fn from(val: Buf31to8i) -> u32 {
        Buf31to8i::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Buf4to0i {
    #[doc = "No such occurrence."]
    Buf4to0i0 = 0x0,
    #[doc = "Corresponding MB completed transmission/reception."]
    Buf4to0i1 = 0x01,
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
impl Buf4to0i {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Buf4to0i {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Buf4to0i {
    #[inline(always)]
    fn from(val: u8) -> Buf4to0i {
        Buf4to0i::from_bits(val)
    }
}
impl From<Buf4to0i> for u8 {
    #[inline(always)]
    fn from(val: Buf4to0i) -> u8 {
        Buf4to0i::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Buf5i {
    #[doc = "No such occurrence."]
    Buf5i0 = 0x0,
    #[doc = "MB5 completed transmission/reception or frames available in the FIFO."]
    Buf5i1 = 0x01,
}
impl Buf5i {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Buf5i {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Buf5i {
    #[inline(always)]
    fn from(val: u8) -> Buf5i {
        Buf5i::from_bits(val)
    }
}
impl From<Buf5i> for u8 {
    #[inline(always)]
    fn from(val: Buf5i) -> u8 {
        Buf5i::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Buf6i {
    #[doc = "No such occurrence."]
    Buf6i0 = 0x0,
    #[doc = "MB6 completed transmission/reception or FIFO almost full."]
    Buf6i1 = 0x01,
}
impl Buf6i {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Buf6i {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Buf6i {
    #[inline(always)]
    fn from(val: u8) -> Buf6i {
        Buf6i::from_bits(val)
    }
}
impl From<Buf6i> for u8 {
    #[inline(always)]
    fn from(val: Buf6i) -> u8 {
        Buf6i::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Buf7i {
    #[doc = "No such occurrence."]
    Buf7i0 = 0x0,
    #[doc = "MB7 completed transmission/reception or FIFO overflow."]
    Buf7i1 = 0x01,
}
impl Buf7i {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Buf7i {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Buf7i {
    #[inline(always)]
    fn from(val: u8) -> Buf7i {
        Buf7i::from_bits(val)
    }
}
impl From<Buf7i> for u8 {
    #[inline(always)]
    fn from(val: Buf7i) -> u8 {
        Buf7i::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Bufhi(u32);
impl Bufhi {
    #[doc = "No such occurrence."]
    pub const Bufhi0: Self = Self(0x0);
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    pub const Bufhi1: Self = Self(0x01);
}
impl Bufhi {
    pub const fn from_bits(val: u32) -> Bufhi {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Bufhi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Bufhi0"),
            0x01 => f.write_str("Bufhi1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bufhi {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Bufhi0"),
            0x01 => defmt::write!(f, "Bufhi1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Bufhi {
    #[inline(always)]
    fn from(val: u32) -> Bufhi {
        Bufhi::from_bits(val)
    }
}
impl From<Bufhi> for u32 {
    #[inline(always)]
    fn from(val: Bufhi) -> u32 {
        Bufhi::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Bufhm(u32);
impl Bufhm {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    pub const Bufhm0: Self = Self(0x0);
    #[doc = "The corresponding buffer Interrupt is enabled."]
    pub const Bufhm1: Self = Self(0x01);
}
impl Bufhm {
    pub const fn from_bits(val: u32) -> Bufhm {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Bufhm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Bufhm0"),
            0x01 => f.write_str("Bufhm1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bufhm {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Bufhm0"),
            0x01 => defmt::write!(f, "Bufhm1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Bufhm {
    #[inline(always)]
    fn from(val: u32) -> Bufhm {
        Bufhm::from_bits(val)
    }
}
impl From<Bufhm> for u32 {
    #[inline(always)]
    fn from(val: Bufhm) -> u32 {
        Bufhm::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Buflm(u32);
impl Buflm {
    #[doc = "The corresponding buffer Interrupt is disabled."]
    pub const Buflm0: Self = Self(0x0);
    #[doc = "The corresponding buffer Interrupt is enabled."]
    pub const Buflm1: Self = Self(0x01);
}
impl Buflm {
    pub const fn from_bits(val: u32) -> Buflm {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Buflm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Buflm0"),
            0x01 => f.write_str("Buflm1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Buflm {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Buflm0"),
            0x01 => defmt::write!(f, "Buflm1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Buflm {
    #[inline(always)]
    fn from(val: u32) -> Buflm {
        Buflm::from_bits(val)
    }
}
impl From<Buflm> for u32 {
    #[inline(always)]
    fn from(val: Buflm) -> u32 {
        Buflm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crcerr {
    #[doc = "No such occurrence."]
    Crcerr0 = 0x0,
    #[doc = "A CRC error occurred since last read of this register."]
    Crcerr1 = 0x01,
}
impl Crcerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crcerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crcerr {
    #[inline(always)]
    fn from(val: u8) -> Crcerr {
        Crcerr::from_bits(val)
    }
}
impl From<Crcerr> for u8 {
    #[inline(always)]
    fn from(val: Crcerr) -> u8 {
        Crcerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Eacen {
    #[doc = "Rx Mailbox filter's IDE bit is always compared and RTR is never compared despite mask bits."]
    Eacen0 = 0x0,
    #[doc = "Enables the comparison of both Rx Mailbox filter's IDE and RTR bit with their corresponding bits within the incoming frame. Mask bits do apply."]
    Eacen1 = 0x01,
}
impl Eacen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eacen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eacen {
    #[inline(always)]
    fn from(val: u8) -> Eacen {
        Eacen::from_bits(val)
    }
}
impl From<Eacen> for u8 {
    #[inline(always)]
    fn from(val: Eacen) -> u8 {
        Eacen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Errint {
    #[doc = "No such occurrence."]
    Errint0 = 0x0,
    #[doc = "Indicates setting of any Error Bit in the Error and Status Register."]
    Errint1 = 0x01,
}
impl Errint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Errint {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Errint {
    #[inline(always)]
    fn from(val: u8) -> Errint {
        Errint::from_bits(val)
    }
}
impl From<Errint> for u8 {
    #[inline(always)]
    fn from(val: Errint) -> u8 {
        Errint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Errmsk {
    #[doc = "Error interrupt disabled."]
    Errmsk0 = 0x0,
    #[doc = "Error interrupt enabled."]
    Errmsk1 = 0x01,
}
impl Errmsk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Errmsk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Errmsk {
    #[inline(always)]
    fn from(val: u8) -> Errmsk {
        Errmsk::from_bits(val)
    }
}
impl From<Errmsk> for u8 {
    #[inline(always)]
    fn from(val: Errmsk) -> u8 {
        Errmsk::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Fgm(u32);
impl Fgm {
    #[doc = "The corresponding bit in the filter is \"don't care\"."]
    pub const Fgm0: Self = Self(0x0);
    #[doc = "The corresponding bit in the filter is checked."]
    pub const Fgm1: Self = Self(0x01);
}
impl Fgm {
    pub const fn from_bits(val: u32) -> Fgm {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Fgm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Fgm0"),
            0x01 => f.write_str("Fgm1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fgm {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Fgm0"),
            0x01 => defmt::write!(f, "Fgm1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Fgm {
    #[inline(always)]
    fn from(val: u32) -> Fgm {
        Fgm::from_bits(val)
    }
}
impl From<Fgm> for u32 {
    #[inline(always)]
    fn from(val: Fgm) -> u32 {
        Fgm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fltconf {
    #[doc = "Error Active."]
    Fltconf0 = 0x0,
    #[doc = "Error Passive."]
    Fltconf1 = 0x01,
    #[doc = "Bus off."]
    Fltconf2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Fltconf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fltconf {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fltconf {
    #[inline(always)]
    fn from(val: u8) -> Fltconf {
        Fltconf::from_bits(val)
    }
}
impl From<Fltconf> for u8 {
    #[inline(always)]
    fn from(val: Fltconf) -> u8 {
        Fltconf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Frmerr {
    #[doc = "No such occurrence."]
    Frmerr0 = 0x0,
    #[doc = "A Form Error occurred since last read of this register."]
    Frmerr1 = 0x01,
}
impl Frmerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Frmerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Frmerr {
    #[inline(always)]
    fn from(val: u8) -> Frmerr {
        Frmerr::from_bits(val)
    }
}
impl From<Frmerr> for u8 {
    #[inline(always)]
    fn from(val: Frmerr) -> u8 {
        Frmerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Frz {
    #[doc = "Not enabled to enter Freeze Mode."]
    Frz0 = 0x0,
    #[doc = "Enabled to enter Freeze Mode."]
    Frz1 = 0x01,
}
impl Frz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Frz {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Frz {
    #[inline(always)]
    fn from(val: u8) -> Frz {
        Frz::from_bits(val)
    }
}
impl From<Frz> for u8 {
    #[inline(always)]
    fn from(val: Frz) -> u8 {
        Frz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Frzack {
    #[doc = "FLEXCAN not in Freeze Mode, prescaler running."]
    Frzack0 = 0x0,
    #[doc = "FLEXCAN in Freeze Mode, prescaler stopped."]
    Frzack1 = 0x01,
}
impl Frzack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Frzack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Frzack {
    #[inline(always)]
    fn from(val: u8) -> Frzack {
        Frzack::from_bits(val)
    }
}
impl From<Frzack> for u8 {
    #[inline(always)]
    fn from(val: Frzack) -> u8 {
        Frzack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Halt {
    #[doc = "No Freeze Mode request."]
    Halt0 = 0x0,
    #[doc = "Enters Freeze Mode if the FRZ bit is asserted."]
    Halt1 = 0x01,
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
pub enum Idam {
    #[doc = "Format A One full ID (standard or extended) per ID filter Table element."]
    Idam0 = 0x0,
    #[doc = "Format B Two full standard IDs or two partial 14-bit extended IDs per ID filter Table element."]
    Idam1 = 0x01,
    #[doc = "Format C Four partial 8-bit IDs (standard or extended) per ID filter Table element."]
    Idam2 = 0x02,
    #[doc = "Format D All frames rejected."]
    Idam3 = 0x03,
}
impl Idam {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Idam {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Idam {
    #[inline(always)]
    fn from(val: u8) -> Idam {
        Idam::from_bits(val)
    }
}
impl From<Idam> for u8 {
    #[inline(always)]
    fn from(val: Idam) -> u8 {
        Idam::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Idle {
    #[doc = "No such occurrence."]
    Idle0 = 0x0,
    #[doc = "CAN bus is now IDLE."]
    Idle1 = 0x01,
}
impl Idle {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Idle {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Idle {
    #[inline(always)]
    fn from(val: u8) -> Idle {
        Idle::from_bits(val)
    }
}
impl From<Idle> for u8 {
    #[inline(always)]
    fn from(val: Idle) -> u8 {
        Idle::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Imb {
    #[doc = "If ESR2\\[VPS\\] is asserted, the ESR2\\[LPTM\\] is not an inactive Mailbox."]
    Imb0 = 0x0,
    #[doc = "If ESR2\\[VPS\\] is asserted, there is at least one inactive Mailbox. LPTM content is the number of the first one."]
    Imb1 = 0x01,
}
impl Imb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Imb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Imb {
    #[inline(always)]
    fn from(val: u8) -> Imb {
        Imb::from_bits(val)
    }
}
impl From<Imb> for u8 {
    #[inline(always)]
    fn from(val: Imb) -> u8 {
        Imb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Irmq {
    #[doc = "Individual Rx masking and queue feature are disabled.For backward compatibility, the reading of C/S word locks the MB even if it is EMPTY."]
    Irmq0 = 0x0,
    #[doc = "Individual Rx masking and queue feature are enabled."]
    Irmq1 = 0x01,
}
impl Irmq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Irmq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Irmq {
    #[inline(always)]
    fn from(val: u8) -> Irmq {
        Irmq::from_bits(val)
    }
}
impl From<Irmq> for u8 {
    #[inline(always)]
    fn from(val: Irmq) -> u8 {
        Irmq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lbuf {
    #[doc = "Buffer with highest priority is transmitted first."]
    Lbuf0 = 0x0,
    #[doc = "Lowest number buffer is transmitted first."]
    Lbuf1 = 0x01,
}
impl Lbuf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lbuf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lbuf {
    #[inline(always)]
    fn from(val: u8) -> Lbuf {
        Lbuf::from_bits(val)
    }
}
impl From<Lbuf> for u8 {
    #[inline(always)]
    fn from(val: Lbuf) -> u8 {
        Lbuf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lom {
    #[doc = "Listen Only Mode is deactivated."]
    Lom0 = 0x0,
    #[doc = "FLEXCAN module operates in Listen Only Mode."]
    Lom1 = 0x01,
}
impl Lom {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lom {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lom {
    #[inline(always)]
    fn from(val: u8) -> Lom {
        Lom::from_bits(val)
    }
}
impl From<Lom> for u8 {
    #[inline(always)]
    fn from(val: Lom) -> u8 {
        Lom::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpb {
    #[doc = "Loop Back disabled."]
    Lpb0 = 0x0,
    #[doc = "Loop Back enabled."]
    Lpb1 = 0x01,
}
impl Lpb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpb {
    #[inline(always)]
    fn from(val: u8) -> Lpb {
        Lpb::from_bits(val)
    }
}
impl From<Lpb> for u8 {
    #[inline(always)]
    fn from(val: Lpb) -> u8 {
        Lpb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpmack {
    #[doc = "FLEXCAN not in any of the low power modes."]
    Lpmack0 = 0x0,
    #[doc = "FLEXCAN is either in Disable Mode, or Stop mode."]
    Lpmack1 = 0x01,
}
impl Lpmack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpmack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpmack {
    #[inline(always)]
    fn from(val: u8) -> Lpmack {
        Lpmack::from_bits(val)
    }
}
impl From<Lpmack> for u8 {
    #[inline(always)]
    fn from(val: Lpmack) -> u8 {
        Lpmack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lprioen {
    #[doc = "Local Priority disabled."]
    Lprioen0 = 0x0,
    #[doc = "Local Priority enabled."]
    Lprioen1 = 0x01,
}
impl Lprioen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lprioen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lprioen {
    #[inline(always)]
    fn from(val: u8) -> Lprioen {
        Lprioen::from_bits(val)
    }
}
impl From<Lprioen> for u8 {
    #[inline(always)]
    fn from(val: Lprioen) -> u8 {
        Lprioen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mdis {
    #[doc = "Enable the FLEXCAN module."]
    Mdis0 = 0x0,
    #[doc = "Disable the FLEXCAN module."]
    Mdis1 = 0x01,
}
impl Mdis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mdis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mdis {
    #[inline(always)]
    fn from(val: u8) -> Mdis {
        Mdis::from_bits(val)
    }
}
impl From<Mdis> for u8 {
    #[inline(always)]
    fn from(val: Mdis) -> u8 {
        Mdis::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Mg(u32);
impl Mg {
    #[doc = "the corresponding bit in the filter is \"don't care\"."]
    pub const Mg0: Self = Self(0x0);
    #[doc = "The corresponding bit in the filter is checked against the one received."]
    pub const Mg1: Self = Self(0x01);
}
impl Mg {
    pub const fn from_bits(val: u32) -> Mg {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Mg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Mg0"),
            0x01 => f.write_str("Mg1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mg {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Mg0"),
            0x01 => defmt::write!(f, "Mg1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Mg {
    #[inline(always)]
    fn from(val: u32) -> Mg {
        Mg::from_bits(val)
    }
}
impl From<Mg> for u32 {
    #[inline(always)]
    fn from(val: Mg) -> u32 {
        Mg::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Mi(u32);
impl Mi {
    #[doc = "the corresponding bit in the filter is \"don't care\"."]
    pub const Mi0: Self = Self(0x0);
    #[doc = "The corresponding bit in the filter is checked."]
    pub const Mi1: Self = Self(0x01);
}
impl Mi {
    pub const fn from_bits(val: u32) -> Mi {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Mi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Mi0"),
            0x01 => f.write_str("Mi1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mi {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Mi0"),
            0x01 => defmt::write!(f, "Mi1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Mi {
    #[inline(always)]
    fn from(val: u32) -> Mi {
        Mi::from_bits(val)
    }
}
impl From<Mi> for u32 {
    #[inline(always)]
    fn from(val: Mi) -> u32 {
        Mi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mpp {
    #[doc = "No matching process ongoing."]
    Mpp0 = 0x0,
    #[doc = "Matching process is in progress."]
    Mpp1 = 0x01,
}
impl Mpp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mpp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mpp {
    #[inline(always)]
    fn from(val: u8) -> Mpp {
        Mpp::from_bits(val)
    }
}
impl From<Mpp> for u8 {
    #[inline(always)]
    fn from(val: Mpp) -> u8 {
        Mpp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mrp {
    #[doc = "Matching starts from Rx FIFO and continues on Mailboxes."]
    Mrp0 = 0x0,
    #[doc = "Matching starts from Mailboxes and continues on Rx FIFO."]
    Mrp1 = 0x01,
}
impl Mrp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mrp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mrp {
    #[inline(always)]
    fn from(val: u8) -> Mrp {
        Mrp::from_bits(val)
    }
}
impl From<Mrp> for u8 {
    #[inline(always)]
    fn from(val: Mrp) -> u8 {
        Mrp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Notrdy {
    #[doc = "FLEXCAN module is either in Normal Mode, Listen-Only Mode or Loop-Back Mode."]
    Notrdy0 = 0x0,
    #[doc = "FLEXCAN module is either in Disable Mode, Stop Mode or Freeze Mode."]
    Notrdy1 = 0x01,
}
impl Notrdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Notrdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Notrdy {
    #[inline(always)]
    fn from(val: u8) -> Notrdy {
        Notrdy::from_bits(val)
    }
}
impl From<Notrdy> for u8 {
    #[inline(always)]
    fn from(val: Notrdy) -> u8 {
        Notrdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rfen {
    #[doc = "FIFO not enabled."]
    Rfen0 = 0x0,
    #[doc = "FIFO enabled."]
    Rfen1 = 0x01,
}
impl Rfen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rfen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rfen {
    #[inline(always)]
    fn from(val: u8) -> Rfen {
        Rfen::from_bits(val)
    }
}
impl From<Rfen> for u8 {
    #[inline(always)]
    fn from(val: Rfen) -> u8 {
        Rfen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rrs {
    #[doc = "Remote Response Frame is generated."]
    Rrs0 = 0x0,
    #[doc = "Remote Request Frame is stored."]
    Rrs1 = 0x01,
}
impl Rrs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rrs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rrs {
    #[inline(always)]
    fn from(val: u8) -> Rrs {
        Rrs::from_bits(val)
    }
}
impl From<Rrs> for u8 {
    #[inline(always)]
    fn from(val: Rrs) -> u8 {
        Rrs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rwrnint {
    #[doc = "No such occurrence."]
    Rwrnint0 = 0x0,
    #[doc = "The Rx error counter transition from < 96 to >= 96."]
    Rwrnint1 = 0x01,
}
impl Rwrnint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rwrnint {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rwrnint {
    #[inline(always)]
    fn from(val: u8) -> Rwrnint {
        Rwrnint::from_bits(val)
    }
}
impl From<Rwrnint> for u8 {
    #[inline(always)]
    fn from(val: Rwrnint) -> u8 {
        Rwrnint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rwrnmsk {
    #[doc = "Rx Warning Interrupt disabled."]
    Rwrnmsk0 = 0x0,
    #[doc = "Rx Warning Interrupt enabled."]
    Rwrnmsk1 = 0x01,
}
impl Rwrnmsk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rwrnmsk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rwrnmsk {
    #[inline(always)]
    fn from(val: u8) -> Rwrnmsk {
        Rwrnmsk::from_bits(val)
    }
}
impl From<Rwrnmsk> for u8 {
    #[inline(always)]
    fn from(val: Rwrnmsk) -> u8 {
        Rwrnmsk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rx {
    #[doc = "FLEXCAN is receiving a message."]
    Rx0 = 0x0,
    #[doc = "FLEXCAN is transmitting a message."]
    Rx1 = 0x01,
}
impl Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rx {
    #[inline(always)]
    fn from(val: u8) -> Rx {
        Rx::from_bits(val)
    }
}
impl From<Rx> for u8 {
    #[inline(always)]
    fn from(val: Rx) -> u8 {
        Rx::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Rx14m(u32);
impl Rx14m {
    #[doc = "the corresponding bit in the filter is \"don't care\"."]
    pub const Rx14m0: Self = Self(0x0);
    #[doc = "The corresponding bit in the filter is checked."]
    pub const Rx14m1: Self = Self(0x01);
}
impl Rx14m {
    pub const fn from_bits(val: u32) -> Rx14m {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Rx14m {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Rx14m0"),
            0x01 => f.write_str("Rx14m1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rx14m {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Rx14m0"),
            0x01 => defmt::write!(f, "Rx14m1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Rx14m {
    #[inline(always)]
    fn from(val: u32) -> Rx14m {
        Rx14m::from_bits(val)
    }
}
impl From<Rx14m> for u32 {
    #[inline(always)]
    fn from(val: Rx14m) -> u32 {
        Rx14m::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Rx15m(u32);
impl Rx15m {
    #[doc = "the corresponding bit in the filter is \"don't care\"."]
    pub const Rx15m0: Self = Self(0x0);
    #[doc = "The corresponding bit in the filter is checked."]
    pub const Rx15m1: Self = Self(0x01);
}
impl Rx15m {
    pub const fn from_bits(val: u32) -> Rx15m {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Rx15m {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Rx15m0"),
            0x01 => f.write_str("Rx15m1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rx15m {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Rx15m0"),
            0x01 => defmt::write!(f, "Rx15m1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Rx15m {
    #[inline(always)]
    fn from(val: u32) -> Rx15m {
        Rx15m::from_bits(val)
    }
}
impl From<Rx15m> for u32 {
    #[inline(always)]
    fn from(val: Rx15m) -> u32 {
        Rx15m::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxwrn {
    #[doc = "No such occurrence."]
    Rxwrn0 = 0x0,
    #[doc = "Rx_Err_Counter >= 96."]
    Rxwrn1 = 0x01,
}
impl Rxwrn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxwrn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxwrn {
    #[inline(always)]
    fn from(val: u8) -> Rxwrn {
        Rxwrn::from_bits(val)
    }
}
impl From<Rxwrn> for u8 {
    #[inline(always)]
    fn from(val: Rxwrn) -> u8 {
        Rxwrn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Slfwak {
    #[doc = "FLEXCAN Self Wake Up feature is disabled."]
    Slfwak0 = 0x0,
    #[doc = "FLEXCAN Self Wake Up feature is enabled."]
    Slfwak1 = 0x01,
}
impl Slfwak {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Slfwak {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Slfwak {
    #[inline(always)]
    fn from(val: u8) -> Slfwak {
        Slfwak::from_bits(val)
    }
}
impl From<Slfwak> for u8 {
    #[inline(always)]
    fn from(val: Slfwak) -> u8 {
        Slfwak::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Smp {
    #[doc = "Just one sample is used to determine the bit value."]
    Smp0 = 0x0,
    #[doc = "Three samples are used to determine the value of the received bit: the regular one (sample point) and 2 preceding samples, a majority rule is used."]
    Smp1 = 0x01,
}
impl Smp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Smp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Smp {
    #[inline(always)]
    fn from(val: u8) -> Smp {
        Smp::from_bits(val)
    }
}
impl From<Smp> for u8 {
    #[inline(always)]
    fn from(val: Smp) -> u8 {
        Smp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Softrst {
    #[doc = "No reset request."]
    Softrst0 = 0x0,
    #[doc = "Reset the registers."]
    Softrst1 = 0x01,
}
impl Softrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Softrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Softrst {
    #[inline(always)]
    fn from(val: u8) -> Softrst {
        Softrst::from_bits(val)
    }
}
impl From<Softrst> for u8 {
    #[inline(always)]
    fn from(val: Softrst) -> u8 {
        Softrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Srxdis {
    #[doc = "Self reception enabled."]
    Srxdis0 = 0x0,
    #[doc = "Self reception disabled."]
    Srxdis1 = 0x01,
}
impl Srxdis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Srxdis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Srxdis {
    #[inline(always)]
    fn from(val: u8) -> Srxdis {
        Srxdis::from_bits(val)
    }
}
impl From<Srxdis> for u8 {
    #[inline(always)]
    fn from(val: Srxdis) -> u8 {
        Srxdis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Stferr {
    #[doc = "No such occurrence."]
    Stferr0 = 0x0,
    #[doc = "A Stuffing Error occurred since last read of this register."]
    Stferr1 = 0x01,
}
impl Stferr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stferr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stferr {
    #[inline(always)]
    fn from(val: u8) -> Stferr {
        Stferr::from_bits(val)
    }
}
impl From<Stferr> for u8 {
    #[inline(always)]
    fn from(val: Stferr) -> u8 {
        Stferr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Supv {
    #[doc = "FlexCAN is in User Mode. Affected registers allow both Supervisor and Unrestricted accesses."]
    Supv0 = 0x0,
    #[doc = "FlexCAN is in Supervisor Mode. Affected registers allow only Supervisor access. Unrestricted access behaves as though the access was done to an unimplemented register location."]
    Supv1 = 0x01,
}
impl Supv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Supv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Supv {
    #[inline(always)]
    fn from(val: u8) -> Supv {
        Supv::from_bits(val)
    }
}
impl From<Supv> for u8 {
    #[inline(always)]
    fn from(val: Supv) -> u8 {
        Supv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Synch {
    #[doc = "FlexCAN is not synchronized to the CAN bus."]
    Synch0 = 0x0,
    #[doc = "FlexCAN is synchronized to the CAN bus."]
    Synch1 = 0x01,
}
impl Synch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Synch {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Synch {
    #[inline(always)]
    fn from(val: u8) -> Synch {
        Synch::from_bits(val)
    }
}
impl From<Synch> for u8 {
    #[inline(always)]
    fn from(val: Synch) -> u8 {
        Synch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsyn {
    #[doc = "Timer Sync feature disabled."]
    Tsyn0 = 0x0,
    #[doc = "Timer Sync feature enabled."]
    Tsyn1 = 0x01,
}
impl Tsyn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsyn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsyn {
    #[inline(always)]
    fn from(val: u8) -> Tsyn {
        Tsyn::from_bits(val)
    }
}
impl From<Tsyn> for u8 {
    #[inline(always)]
    fn from(val: Tsyn) -> u8 {
        Tsyn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Twrnint {
    #[doc = "No such occurrence."]
    Twrnint0 = 0x0,
    #[doc = "The Tx error counter transition from < 96 to >= 96."]
    Twrnint1 = 0x01,
}
impl Twrnint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Twrnint {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Twrnint {
    #[inline(always)]
    fn from(val: u8) -> Twrnint {
        Twrnint::from_bits(val)
    }
}
impl From<Twrnint> for u8 {
    #[inline(always)]
    fn from(val: Twrnint) -> u8 {
        Twrnint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Twrnmsk {
    #[doc = "Tx Warning Interrupt disabled."]
    Twrnmsk0 = 0x0,
    #[doc = "Tx Warning Interrupt enabled."]
    Twrnmsk1 = 0x01,
}
impl Twrnmsk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Twrnmsk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Twrnmsk {
    #[inline(always)]
    fn from(val: u8) -> Twrnmsk {
        Twrnmsk::from_bits(val)
    }
}
impl From<Twrnmsk> for u8 {
    #[inline(always)]
    fn from(val: Twrnmsk) -> u8 {
        Twrnmsk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tx {
    #[doc = "FLEXCAN is receiving a message."]
    Tx0 = 0x0,
    #[doc = "FLEXCAN is transmitting a message."]
    Tx1 = 0x01,
}
impl Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tx {
    #[inline(always)]
    fn from(val: u8) -> Tx {
        Tx::from_bits(val)
    }
}
impl From<Tx> for u8 {
    #[inline(always)]
    fn from(val: Tx) -> u8 {
        Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txwrn {
    #[doc = "No such occurrence."]
    Txwrn0 = 0x0,
    #[doc = "TX_Err_Counter >= 96."]
    Txwrn1 = 0x01,
}
impl Txwrn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txwrn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txwrn {
    #[inline(always)]
    fn from(val: u8) -> Txwrn {
        Txwrn::from_bits(val)
    }
}
impl From<Txwrn> for u8 {
    #[inline(always)]
    fn from(val: Txwrn) -> u8 {
        Txwrn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vps {
    #[doc = "Contents of IMB and LPTM are invalid."]
    Vps0 = 0x0,
    #[doc = "Contents of IMB and LPTM are valid."]
    Vps1 = 0x01,
}
impl Vps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vps {
    #[inline(always)]
    fn from(val: u8) -> Vps {
        Vps::from_bits(val)
    }
}
impl From<Vps> for u8 {
    #[inline(always)]
    fn from(val: Vps) -> u8 {
        Vps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wakint {
    #[doc = "No such occurrence."]
    Wakint0 = 0x0,
    #[doc = "Indicates a recessive to dominant transition received on the CAN bus when the FLEXCAN module is in Stop Mode."]
    Wakint1 = 0x01,
}
impl Wakint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wakint {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wakint {
    #[inline(always)]
    fn from(val: u8) -> Wakint {
        Wakint::from_bits(val)
    }
}
impl From<Wakint> for u8 {
    #[inline(always)]
    fn from(val: Wakint) -> u8 {
        Wakint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wakmsk {
    #[doc = "Wake Up Interrupt is disabled."]
    Wakmsk0 = 0x0,
    #[doc = "Wake Up Interrupt is enabled."]
    Wakmsk1 = 0x01,
}
impl Wakmsk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wakmsk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wakmsk {
    #[inline(always)]
    fn from(val: u8) -> Wakmsk {
        Wakmsk::from_bits(val)
    }
}
impl From<Wakmsk> for u8 {
    #[inline(always)]
    fn from(val: Wakmsk) -> u8 {
        Wakmsk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Waksrc {
    #[doc = "FLEXCAN uses the unfiltered FLEXCAN_RX input to detect recessive to dominant edges on the CAN bus."]
    Waksrc0 = 0x0,
    #[doc = "FLEXCAN uses the filtered FLEXCAN_RX input to detect recessive to dominant edges on the CAN bus."]
    Waksrc1 = 0x01,
}
impl Waksrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Waksrc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Waksrc {
    #[inline(always)]
    fn from(val: u8) -> Waksrc {
        Waksrc::from_bits(val)
    }
}
impl From<Waksrc> for u8 {
    #[inline(always)]
    fn from(val: Waksrc) -> u8 {
        Waksrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wrmfrz {
    #[doc = "Keep the write access restricted in some regions of FlexCAN memory."]
    Wrmfrz0 = 0x0,
    #[doc = "Enable unrestricted write access to FlexCAN memory."]
    Wrmfrz1 = 0x01,
}
impl Wrmfrz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wrmfrz {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wrmfrz {
    #[inline(always)]
    fn from(val: u8) -> Wrmfrz {
        Wrmfrz::from_bits(val)
    }
}
impl From<Wrmfrz> for u8 {
    #[inline(always)]
    fn from(val: Wrmfrz) -> u8 {
        Wrmfrz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wrnen {
    #[doc = "TWRN_INT and RWRN_INT bits are zero, independent of the values in the error counters."]
    Wrnen0 = 0x0,
    #[doc = "TWRN_INT and RWRN_INT bits are set when the respective error counter transition from <96 to >= 96."]
    Wrnen1 = 0x01,
}
impl Wrnen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wrnen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wrnen {
    #[inline(always)]
    fn from(val: u8) -> Wrnen {
        Wrnen::from_bits(val)
    }
}
impl From<Wrnen> for u8 {
    #[inline(always)]
    fn from(val: Wrnen) -> u8 {
        Wrnen::to_bits(val)
    }
}
