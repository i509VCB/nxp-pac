#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Abort6333(u32);
impl Abort6333 {
    #[doc = "No effect."]
    pub const NO_EFFECT: Self = Self(0x0);
    #[doc = "Aborts DMA operations on the relevant channel."]
    pub const EFFECT: Self = Self(0x01);
}
impl Abort6333 {
    pub const fn from_bits(val: u32) -> Abort6333 {
        Self(val & 0x7fff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Abort6333 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NO_EFFECT"),
            0x01 => f.write_str("EFFECT"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Abort6333 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NO_EFFECT"),
            0x01 => defmt::write!(f, "EFFECT"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Abort6333 {
    #[inline(always)]
    fn from(val: u32) -> Abort6333 {
        Abort6333::from_bits(val)
    }
}
impl From<Abort6333> for u32 {
    #[inline(always)]
    fn from(val: Abort6333) -> u32 {
        Abort6333::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Abortctrl(u32);
impl Abortctrl {
    #[doc = "No effect."]
    pub const NO_EFFECT: Self = Self(0x0);
    #[doc = "Aborts DMA operations on channel 0."]
    pub const EFFECT: Self = Self(0x01);
}
impl Abortctrl {
    pub const fn from_bits(val: u32) -> Abortctrl {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Abortctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NO_EFFECT"),
            0x01 => f.write_str("EFFECT"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Abortctrl {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NO_EFFECT"),
            0x01 => defmt::write!(f, "EFFECT"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Abortctrl {
    #[inline(always)]
    fn from(val: u32) -> Abortctrl {
        Abortctrl::from_bits(val)
    }
}
impl From<Abortctrl> for u32 {
    #[inline(always)]
    fn from(val: Abortctrl) -> u32 {
        Abortctrl::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Act(u32);
impl Act {
    #[doc = "DMAchannel 0 is not active."]
    pub const NOT_ACTIVE: Self = Self(0x0);
    #[doc = "DMAchannel 0 is active."]
    pub const ACTIVE: Self = Self(0x01);
}
impl Act {
    pub const fn from_bits(val: u32) -> Act {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Act {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NOT_ACTIVE"),
            0x01 => f.write_str("ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Act {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NOT_ACTIVE"),
            0x01 => defmt::write!(f, "ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Act {
    #[inline(always)]
    fn from(val: u32) -> Act {
        Act::from_bits(val)
    }
}
impl From<Act> for u32 {
    #[inline(always)]
    fn from(val: Act) -> u32 {
        Act::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Active6333(u32);
impl Active6333 {
    #[doc = "The relevant DMA channel is not active."]
    pub const NOT_ACTIVE: Self = Self(0x0);
    #[doc = "The relevant DMA channel is active."]
    pub const ACTIVE: Self = Self(0x01);
}
impl Active6333 {
    pub const fn from_bits(val: u32) -> Active6333 {
        Self(val & 0x7fff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Active6333 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NOT_ACTIVE"),
            0x01 => f.write_str("ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Active6333 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NOT_ACTIVE"),
            0x01 => defmt::write!(f, "ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Active6333 {
    #[inline(always)]
    fn from(val: u32) -> Active6333 {
        Active6333::from_bits(val)
    }
}
impl From<Active6333> for u32 {
    #[inline(always)]
    fn from(val: Active6333) -> u32 {
        Active6333::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Bsy(u32);
impl Bsy {
    #[doc = "DMAchannel 0 is not busy."]
    pub const NOT_BUSY: Self = Self(0x0);
    #[doc = "DMAchannel 0 is busy."]
    pub const BUSY: Self = Self(0x01);
}
impl Bsy {
    pub const fn from_bits(val: u32) -> Bsy {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Bsy {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NOT_BUSY"),
            0x01 => f.write_str("BUSY"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bsy {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NOT_BUSY"),
            0x01 => defmt::write!(f, "BUSY"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Bsy {
    #[inline(always)]
    fn from(val: u32) -> Bsy {
        Bsy::from_bits(val)
    }
}
impl From<Bsy> for u32 {
    #[inline(always)]
    fn from(val: Bsy) -> u32 {
        Bsy::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Busy6333(u32);
impl Busy6333 {
    #[doc = "The relevant DMA channel is not busy."]
    pub const NOT_BUSY: Self = Self(0x0);
    #[doc = "The relevant DMA channel is busy."]
    pub const BUSY: Self = Self(0x01);
}
impl Busy6333 {
    pub const fn from_bits(val: u32) -> Busy6333 {
        Self(val & 0x7fff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Busy6333 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NOT_BUSY"),
            0x01 => f.write_str("BUSY"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Busy6333 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NOT_BUSY"),
            0x01 => defmt::write!(f, "BUSY"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Busy6333 {
    #[inline(always)]
    fn from(val: u32) -> Busy6333 {
        Busy6333::from_bits(val)
    }
}
impl From<Busy6333> for u32 {
    #[inline(always)]
    fn from(val: Busy6333) -> u32 {
        Busy6333::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dstinc {
    #[doc = "No increment. The destination address is not incremented for each transfer. This is the usual case when the destination is a peripheral device."]
    NO_INCREMENT = 0x0,
    #[doc = "1 x width. The destination address is incremented by the amount specified by Width for each transfer. This is the usual case when the destination is memory."]
    WIDTH_X_1 = 0x01,
    #[doc = "2 x width. The destination address is incremented by 2 times the amount specified by Width for each transfer."]
    WIDTH_X_2 = 0x02,
    #[doc = "4 x width. The destination address is incremented by 4 times the amount specified by Width for each transfer."]
    WIDTH_X_4 = 0x03,
}
impl Dstinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dstinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dstinc {
    #[inline(always)]
    fn from(val: u8) -> Dstinc {
        Dstinc::from_bits(val)
    }
}
impl From<Dstinc> for u8 {
    #[inline(always)]
    fn from(val: Dstinc) -> u8 {
        Dstinc::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ena(u32);
impl Ena {
    #[doc = "DMAchannel 0 is disabled."]
    pub const DISABLED: Self = Self(0x0);
    #[doc = "DMAchannel 0 is enabled."]
    pub const ENABLED: Self = Self(0x01);
}
impl Ena {
    pub const fn from_bits(val: u32) -> Ena {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Ena {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("DISABLED"),
            0x01 => f.write_str("ENABLED"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ena {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "DISABLED"),
            0x01 => defmt::write!(f, "ENABLED"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Ena {
    #[inline(always)]
    fn from(val: u32) -> Ena {
        Ena::from_bits(val)
    }
}
impl From<Ena> for u32 {
    #[inline(always)]
    fn from(val: Ena) -> u32 {
        Ena::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Enable6333(u32);
impl Enable6333 {
    #[doc = "The relevant DMA channel is disabled."]
    pub const DISABLED: Self = Self(0x0);
    #[doc = "The relevant DMA channel is enabled."]
    pub const ENABLED: Self = Self(0x01);
}
impl Enable6333 {
    pub const fn from_bits(val: u32) -> Enable6333 {
        Self(val & 0x7fff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Enable6333 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("DISABLED"),
            0x01 => f.write_str("ENABLED"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Enable6333 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "DISABLED"),
            0x01 => defmt::write!(f, "ENABLED"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Enable6333 {
    #[inline(always)]
    fn from(val: u32) -> Enable6333 {
        Enable6333::from_bits(val)
    }
}
impl From<Enable6333> for u32 {
    #[inline(always)]
    fn from(val: Enable6333) -> u32 {
        Enable6333::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Err(u32);
impl Err {
    #[doc = "The Error Interrupt is not active for DMA channel 0."]
    pub const NOT_ACTIVE: Self = Self(0x0);
    #[doc = "The Error Interrupt is pending for DMA channel 0."]
    pub const PENDING: Self = Self(0x01);
}
impl Err {
    pub const fn from_bits(val: u32) -> Err {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Err {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NOT_ACTIVE"),
            0x01 => f.write_str("PENDING"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Err {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NOT_ACTIVE"),
            0x01 => defmt::write!(f, "PENDING"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Err {
    #[inline(always)]
    fn from(val: u32) -> Err {
        Err::from_bits(val)
    }
}
impl From<Err> for u32 {
    #[inline(always)]
    fn from(val: Err) -> u32 {
        Err::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Err32 {
    #[doc = "The Error Interrupt is not active for DMA channel 32."]
    NOT_ACTIVE = 0x0,
    #[doc = "The Error Interrupt is pending for DMA channel 32."]
    PENDING = 0x01,
}
impl Err32 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Err32 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Err32 {
    #[inline(always)]
    fn from(val: u8) -> Err32 {
        Err32::from_bits(val)
    }
}
impl From<Err32> for u8 {
    #[inline(always)]
    fn from(val: Err32) -> u8 {
        Err32::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Err6333(u32);
impl Err6333 {
    #[doc = "The Error Interrupt is not active for the relevant DMA channel."]
    pub const NOT_ACTIVE: Self = Self(0x0);
    #[doc = "The Error Interrupt is pending for the relevant DMA channel."]
    pub const PENDING: Self = Self(0x01);
}
impl Err6333 {
    pub const fn from_bits(val: u32) -> Err6333 {
        Self(val & 0x7fff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Err6333 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NOT_ACTIVE"),
            0x01 => f.write_str("PENDING"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Err6333 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NOT_ACTIVE"),
            0x01 => defmt::write!(f, "PENDING"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Err6333 {
    #[inline(always)]
    fn from(val: u32) -> Err6333 {
        Err6333::from_bits(val)
    }
}
impl From<Err6333> for u32 {
    #[inline(always)]
    fn from(val: Err6333) -> u32 {
        Err6333::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ia(u32);
impl Ia {
    #[doc = "The DMAchannel 0 interrupt A is not active."]
    pub const NOT_ACTIVE: Self = Self(0x0);
    #[doc = "The DMAchannel 0 interrupt A is active."]
    pub const ACTIVE: Self = Self(0x01);
}
impl Ia {
    pub const fn from_bits(val: u32) -> Ia {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Ia {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NOT_ACTIVE"),
            0x01 => f.write_str("ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ia {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NOT_ACTIVE"),
            0x01 => defmt::write!(f, "ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Ia {
    #[inline(always)]
    fn from(val: u32) -> Ia {
        Ia::from_bits(val)
    }
}
impl From<Ia> for u32 {
    #[inline(always)]
    fn from(val: Ia) -> u32 {
        Ia::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ib(u32);
impl Ib {
    #[doc = "The DMAchannel 0 interrupt B is not active."]
    pub const NOT_ACTIVE: Self = Self(0x0);
    #[doc = "The DMAchannel 0 interrupt B is active."]
    pub const ACTIVE: Self = Self(0x01);
}
impl Ib {
    pub const fn from_bits(val: u32) -> Ib {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Ib {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NOT_ACTIVE"),
            0x01 => f.write_str("ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ib {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NOT_ACTIVE"),
            0x01 => defmt::write!(f, "ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Ib {
    #[inline(always)]
    fn from(val: u32) -> Ib {
        Ib::from_bits(val)
    }
}
impl From<Ib> for u32 {
    #[inline(always)]
    fn from(val: Ib) -> u32 {
        Ib::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Inta6333(u32);
impl Inta6333 {
    #[doc = "Interrupt A is not active for the relevant DMA channel."]
    pub const NOT_ACTIVE: Self = Self(0x0);
    #[doc = "Interrupt A is active for the relevant DMA channel."]
    pub const ACTIVE: Self = Self(0x01);
}
impl Inta6333 {
    pub const fn from_bits(val: u32) -> Inta6333 {
        Self(val & 0x7fff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Inta6333 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NOT_ACTIVE"),
            0x01 => f.write_str("ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Inta6333 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NOT_ACTIVE"),
            0x01 => defmt::write!(f, "ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Inta6333 {
    #[inline(always)]
    fn from(val: u32) -> Inta6333 {
        Inta6333::from_bits(val)
    }
}
impl From<Inta6333> for u32 {
    #[inline(always)]
    fn from(val: Inta6333) -> u32 {
        Inta6333::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Intb6333(u32);
impl Intb6333 {
    #[doc = "Interrupt B is not active for the relevant DMA channel."]
    pub const NOT_ACTIVE: Self = Self(0x0);
    #[doc = "Interrupt B is active for the relevant DMA channel."]
    pub const ACTIVE: Self = Self(0x01);
}
impl Intb6333 {
    pub const fn from_bits(val: u32) -> Intb6333 {
        Self(val & 0x7fff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Intb6333 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NOT_ACTIVE"),
            0x01 => f.write_str("ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intb6333 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NOT_ACTIVE"),
            0x01 => defmt::write!(f, "ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Intb6333 {
    #[inline(always)]
    fn from(val: u32) -> Intb6333 {
        Intb6333::from_bits(val)
    }
}
impl From<Intb6333> for u32 {
    #[inline(always)]
    fn from(val: Intb6333) -> u32 {
        Intb6333::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Inten(u32);
impl Inten {
    #[doc = "The Interrupt for DMA channel 0 is disabled."]
    pub const DISABLED: Self = Self(0x0);
    #[doc = "The Interrupt for DMA channel 0 is enabled."]
    pub const ENABLED: Self = Self(0x01);
}
impl Inten {
    pub const fn from_bits(val: u32) -> Inten {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Inten {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("DISABLED"),
            0x01 => f.write_str("ENABLED"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Inten {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "DISABLED"),
            0x01 => defmt::write!(f, "ENABLED"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Inten {
    #[inline(always)]
    fn from(val: u32) -> Inten {
        Inten::from_bits(val)
    }
}
impl From<Inten> for u32 {
    #[inline(always)]
    fn from(val: Inten) -> u32 {
        Inten::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Inten6333(u32);
impl Inten6333 {
    #[doc = "The Interrupt for the relevant DMA channel is disabled."]
    pub const DISABLED: Self = Self(0x0);
    #[doc = "The Interrupt for the relevant DMA channel is enabled."]
    pub const ENABLED: Self = Self(0x01);
}
impl Inten6333 {
    pub const fn from_bits(val: u32) -> Inten6333 {
        Self(val & 0x7fff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Inten6333 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("DISABLED"),
            0x01 => f.write_str("ENABLED"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Inten6333 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "DISABLED"),
            0x01 => defmt::write!(f, "ENABLED"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Inten6333 {
    #[inline(always)]
    fn from(val: u32) -> Inten6333 {
        Inten6333::from_bits(val)
    }
}
impl From<Inten6333> for u32 {
    #[inline(always)]
    fn from(val: Inten6333) -> u32 {
        Inten6333::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Settrig0Trig(u32);
impl Settrig0Trig {
    #[doc = "No effect."]
    pub const NO_EFFECT: Self = Self(0x0);
    #[doc = "Sets the Trig bit for DMA channel 0."]
    pub const EFFECT: Self = Self(0x01);
}
impl Settrig0Trig {
    pub const fn from_bits(val: u32) -> Settrig0Trig {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Settrig0Trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NO_EFFECT"),
            0x01 => f.write_str("EFFECT"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Settrig0Trig {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NO_EFFECT"),
            0x01 => defmt::write!(f, "EFFECT"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Settrig0Trig {
    #[inline(always)]
    fn from(val: u32) -> Settrig0Trig {
        Settrig0Trig::from_bits(val)
    }
}
impl From<Settrig0Trig> for u32 {
    #[inline(always)]
    fn from(val: Settrig0Trig) -> u32 {
        Settrig0Trig::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Settrig6333(u32);
impl Settrig6333 {
    #[doc = "No effect."]
    pub const NO_EFFECT: Self = Self(0x0);
    #[doc = "Sets the Trig bit for DMA channel for the relevant DMA channel."]
    pub const EFFECT: Self = Self(0x01);
}
impl Settrig6333 {
    pub const fn from_bits(val: u32) -> Settrig6333 {
        Self(val & 0x7fff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Settrig6333 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NO_EFFECT"),
            0x01 => f.write_str("EFFECT"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Settrig6333 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NO_EFFECT"),
            0x01 => defmt::write!(f, "EFFECT"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Settrig6333 {
    #[inline(always)]
    fn from(val: u32) -> Settrig6333 {
        Settrig6333::from_bits(val)
    }
}
impl From<Settrig6333> for u32 {
    #[inline(always)]
    fn from(val: Settrig6333) -> u32 {
        Settrig6333::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Setvalid6333(u32);
impl Setvalid6333 {
    #[doc = "No effect."]
    pub const NO_EFFECT: Self = Self(0x0);
    #[doc = "Sets the ValidPending control bit for the relevant DMA channel."]
    pub const EFFECT: Self = Self(0x01);
}
impl Setvalid6333 {
    pub const fn from_bits(val: u32) -> Setvalid6333 {
        Self(val & 0x7fff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Setvalid6333 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NO_EFFECT"),
            0x01 => f.write_str("EFFECT"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Setvalid6333 {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NO_EFFECT"),
            0x01 => defmt::write!(f, "EFFECT"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Setvalid6333 {
    #[inline(always)]
    fn from(val: u32) -> Setvalid6333 {
        Setvalid6333::from_bits(val)
    }
}
impl From<Setvalid6333> for u32 {
    #[inline(always)]
    fn from(val: Setvalid6333) -> u32 {
        Setvalid6333::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Srcinc {
    #[doc = "No increment. The source address is not incremented for each transfer. This is the usual case when the source is a peripheral device."]
    NO_INCREMENT = 0x0,
    #[doc = "1 x width. The source address is incremented by the amount specified by Width for each transfer. This is the usual case when the source is memory."]
    WIDTH_X_1 = 0x01,
    #[doc = "2 x width. The source address is incremented by 2 times the amount specified by Width for each transfer."]
    WIDTH_X_2 = 0x02,
    #[doc = "4 x width. The source address is incremented by 4 times the amount specified by Width for each transfer."]
    WIDTH_X_4 = 0x03,
}
impl Srcinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Srcinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Srcinc {
    #[inline(always)]
    fn from(val: u8) -> Srcinc {
        Srcinc::from_bits(val)
    }
}
impl From<Srcinc> for u8 {
    #[inline(always)]
    fn from(val: Srcinc) -> u8 {
        Srcinc::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Sv(u32);
impl Sv {
    #[doc = "No effect."]
    pub const NO_EFFECT: Self = Self(0x0);
    #[doc = "Sets the ValidPending control bit for DMA channel 0."]
    pub const EFFECT: Self = Self(0x01);
}
impl Sv {
    pub const fn from_bits(val: u32) -> Sv {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Sv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NO_EFFECT"),
            0x01 => f.write_str("EFFECT"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sv {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NO_EFFECT"),
            0x01 => defmt::write!(f, "EFFECT"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Sv {
    #[inline(always)]
    fn from(val: u32) -> Sv {
        Sv::from_bits(val)
    }
}
impl From<Sv> for u32 {
    #[inline(always)]
    fn from(val: Sv) -> u32 {
        Sv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trigburst {
    #[doc = "Single transfer. Hardware trigger causes a single transfer."]
    SINGLE = 0x0,
    #[doc = "Burst transfer. When the trigger for this channel is set to edge triggered, a hardware trigger causes a burst transfer, as defined by BURSTPOWER. When the trigger for this channel is set to level triggered, a hardware trigger causes transfers to continue as long as the trigger is asserted, unless the transfer is complete."]
    BURST = 0x01,
}
impl Trigburst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trigburst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trigburst {
    #[inline(always)]
    fn from(val: u8) -> Trigburst {
        Trigburst::from_bits(val)
    }
}
impl From<Trigburst> for u8 {
    #[inline(always)]
    fn from(val: Trigburst) -> u8 {
        Trigburst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trigpol {
    #[doc = "Active low - falling edge. Hardware trigger is active low or falling edge triggered, based on TRIGTYPE."]
    ACTIVE_LOW_FALLING = 0x0,
    #[doc = "Active high - rising edge. Hardware trigger is active high or rising edge triggered, based on TRIGTYPE."]
    ACTIVE_HIGH_RISING = 0x01,
}
impl Trigpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trigpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trigpol {
    #[inline(always)]
    fn from(val: u8) -> Trigpol {
        Trigpol::from_bits(val)
    }
}
impl From<Trigpol> for u8 {
    #[inline(always)]
    fn from(val: Trigpol) -> u8 {
        Trigpol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trigtype {
    #[doc = "Edge. Hardware trigger is edge triggered. Transfers will be initiated and completed, as specified for a single trigger."]
    EDGE = 0x0,
    #[doc = "Level. Hardware trigger is level triggered. Note that when level triggering without burst (BURSTPOWER = 0) is selected, only hardware triggers should be used on that channel. Transfers continue as long as the trigger level is asserted. Once the trigger is de-asserted, the transfer will be paused until the trigger is, again, asserted. However, the transfer will not be paused until any remaining transfers within the current BURSTPOWER length are completed."]
    LEVEL = 0x01,
}
impl Trigtype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trigtype {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trigtype {
    #[inline(always)]
    fn from(val: u8) -> Trigtype {
        Trigtype::from_bits(val)
    }
}
impl From<Trigtype> for u8 {
    #[inline(always)]
    fn from(val: Trigtype) -> u8 {
        Trigtype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Validpending {
    #[doc = "No effect. No effect on DMA operation."]
    NO_EFFECT = 0x0,
    #[doc = "Valid pending."]
    VALID_PENDING = 0x01,
}
impl Validpending {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Validpending {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Validpending {
    #[inline(always)]
    fn from(val: u8) -> Validpending {
        Validpending::from_bits(val)
    }
}
impl From<Validpending> for u8 {
    #[inline(always)]
    fn from(val: Validpending) -> u8 {
        Validpending::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Width {
    #[doc = "8-bit. 8-bit transfers are performed (8-bit source reads and destination writes)."]
    BIT_8 = 0x0,
    #[doc = "16-bit. 6-bit transfers are performed (16-bit source reads and destination writes)."]
    BIT_16 = 0x01,
    #[doc = "32-bit. 32-bit transfers are performed (32-bit source reads and destination writes)."]
    BIT_32 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Width {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Width {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Width {
    #[inline(always)]
    fn from(val: u8) -> Width {
        Width::from_bits(val)
    }
}
impl From<Width> for u8 {
    #[inline(always)]
    fn from(val: Width) -> u8 {
        Width::to_bits(val)
    }
}
