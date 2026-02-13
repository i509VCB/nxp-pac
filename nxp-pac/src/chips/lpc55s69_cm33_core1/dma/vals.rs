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
