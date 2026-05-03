#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DSTINC {
    #[doc = "No increment. The destination address is not incremented for each transfer. This is the usual case when the destination is a peripheral device."]
    NO_INCREMENT = 0x0,
    #[doc = "1 x width. The destination address is incremented by the amount specified by Width for each transfer. This is the usual case when the destination is memory."]
    WIDTH_X_1 = 0x01,
    #[doc = "2 x width. The destination address is incremented by 2 times the amount specified by Width for each transfer."]
    WIDTH_X_2 = 0x02,
    #[doc = "4 x width. The destination address is incremented by 4 times the amount specified by Width for each transfer."]
    WIDTH_X_4 = 0x03,
}
impl DSTINC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DSTINC {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DSTINC {
    #[inline(always)]
    fn from(val: u8) -> DSTINC {
        DSTINC::from_bits(val)
    }
}
impl From<DSTINC> for u8 {
    #[inline(always)]
    fn from(val: DSTINC) -> u8 {
        DSTINC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SRCINC {
    #[doc = "No increment. The source address is not incremented for each transfer. This is the usual case when the source is a peripheral device."]
    NO_INCREMENT = 0x0,
    #[doc = "1 x width. The source address is incremented by the amount specified by Width for each transfer. This is the usual case when the source is memory."]
    WIDTH_X_1 = 0x01,
    #[doc = "2 x width. The source address is incremented by 2 times the amount specified by Width for each transfer."]
    WIDTH_X_2 = 0x02,
    #[doc = "4 x width. The source address is incremented by 4 times the amount specified by Width for each transfer."]
    WIDTH_X_4 = 0x03,
}
impl SRCINC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SRCINC {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SRCINC {
    #[inline(always)]
    fn from(val: u8) -> SRCINC {
        SRCINC::from_bits(val)
    }
}
impl From<SRCINC> for u8 {
    #[inline(always)]
    fn from(val: SRCINC) -> u8 {
        SRCINC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TRIGBURST {
    #[doc = "Single transfer. Hardware trigger causes a single transfer."]
    SINGLE = 0x0,
    #[doc = "Burst transfer. When the trigger for this channel is set to edge triggered, a hardware trigger causes a burst transfer, as defined by BURSTPOWER. When the trigger for this channel is set to level triggered, a hardware trigger causes transfers to continue as long as the trigger is asserted, unless the transfer is complete."]
    BURST = 0x01,
}
impl TRIGBURST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TRIGBURST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TRIGBURST {
    #[inline(always)]
    fn from(val: u8) -> TRIGBURST {
        TRIGBURST::from_bits(val)
    }
}
impl From<TRIGBURST> for u8 {
    #[inline(always)]
    fn from(val: TRIGBURST) -> u8 {
        TRIGBURST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TRIGPOL {
    #[doc = "Active low - falling edge. Hardware trigger is active low or falling edge triggered, based on TRIGTYPE."]
    ACTIVE_LOW_FALLING = 0x0,
    #[doc = "Active high - rising edge. Hardware trigger is active high or rising edge triggered, based on TRIGTYPE."]
    ACTIVE_HIGH_RISING = 0x01,
}
impl TRIGPOL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TRIGPOL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TRIGPOL {
    #[inline(always)]
    fn from(val: u8) -> TRIGPOL {
        TRIGPOL::from_bits(val)
    }
}
impl From<TRIGPOL> for u8 {
    #[inline(always)]
    fn from(val: TRIGPOL) -> u8 {
        TRIGPOL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TRIGTYPE {
    #[doc = "Edge. Hardware trigger is edge triggered. Transfers will be initiated and completed, as specified for a single trigger."]
    EDGE = 0x0,
    #[doc = "Level. Hardware trigger is level triggered. Note that when level triggering without burst (BURSTPOWER = 0) is selected, only hardware triggers should be used on that channel. Transfers continue as long as the trigger level is asserted. Once the trigger is de-asserted, the transfer will be paused until the trigger is, again, asserted. However, the transfer will not be paused until any remaining transfers within the current BURSTPOWER length are completed."]
    LEVEL = 0x01,
}
impl TRIGTYPE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TRIGTYPE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TRIGTYPE {
    #[inline(always)]
    fn from(val: u8) -> TRIGTYPE {
        TRIGTYPE::from_bits(val)
    }
}
impl From<TRIGTYPE> for u8 {
    #[inline(always)]
    fn from(val: TRIGTYPE) -> u8 {
        TRIGTYPE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VALIDPENDING {
    #[doc = "No effect. No effect on DMA operation."]
    NO_EFFECT = 0x0,
    #[doc = "Valid pending."]
    VALID_PENDING = 0x01,
}
impl VALIDPENDING {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VALIDPENDING {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VALIDPENDING {
    #[inline(always)]
    fn from(val: u8) -> VALIDPENDING {
        VALIDPENDING::from_bits(val)
    }
}
impl From<VALIDPENDING> for u8 {
    #[inline(always)]
    fn from(val: VALIDPENDING) -> u8 {
        VALIDPENDING::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WIDTH {
    #[doc = "8-bit. 8-bit transfers are performed (8-bit source reads and destination writes)."]
    BIT_8 = 0x0,
    #[doc = "16-bit. 6-bit transfers are performed (16-bit source reads and destination writes)."]
    BIT_16 = 0x01,
    #[doc = "32-bit. 32-bit transfers are performed (32-bit source reads and destination writes)."]
    BIT_32 = 0x02,
    _RESERVED_3 = 0x03,
}
impl WIDTH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WIDTH {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WIDTH {
    #[inline(always)]
    fn from(val: u8) -> WIDTH {
        WIDTH::from_bits(val)
    }
}
impl From<WIDTH> for u8 {
    #[inline(always)]
    fn from(val: WIDTH) -> u8 {
        WIDTH::to_bits(val)
    }
}
