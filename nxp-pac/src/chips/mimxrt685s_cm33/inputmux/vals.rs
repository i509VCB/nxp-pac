#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CaPnSel {
    #[doc = "CT_INP0."]
    CtInp0 = 0x0,
    #[doc = "CT_INP1."]
    CtInp1 = 0x01,
    #[doc = "CT_INP2."]
    CtInp2 = 0x02,
    #[doc = "CT_INP3."]
    CtInp3 = 0x03,
    #[doc = "CT_INP4."]
    CtInp4 = 0x04,
    #[doc = "CT_INP5."]
    CtInp5 = 0x05,
    #[doc = "CT_INP6."]
    CtInp6 = 0x06,
    #[doc = "CT_INP7."]
    CtInp7 = 0x07,
    #[doc = "CT_INP8."]
    CtInp8 = 0x08,
    #[doc = "CT_INP9."]
    CtInp9 = 0x09,
    #[doc = "CT_INP10."]
    CtInp10 = 0x0a,
    #[doc = "CT_INP11."]
    CtInp11 = 0x0b,
    #[doc = "CT_INP12."]
    CtInp12 = 0x0c,
    #[doc = "CT_INP13."]
    CtInp13 = 0x0d,
    #[doc = "CT_INP14."]
    CtInp14 = 0x0e,
    #[doc = "CT_INP15."]
    CtInp15 = 0x0f,
    #[doc = "SHARED I2S0_WS."]
    SharedI2s0Ws = 0x10,
    #[doc = "SHARED I2S1_WS."]
    SharedI2s1Ws = 0x11,
    #[doc = "USB1_FRAME_TOGGLE."]
    Usb1FrameToggle = 0x12,
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
impl CaPnSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CaPnSel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CaPnSel {
    #[inline(always)]
    fn from(val: u8) -> CaPnSel {
        CaPnSel::from_bits(val)
    }
}
impl From<CaPnSel> for u8 {
    #[inline(always)]
    fn from(val: CaPnSel) -> u8 {
        CaPnSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma0ItrigSel {
    #[doc = "NSGPIOPINT0_INT0."]
    Nsgpiopint0Int0 = 0x0,
    #[doc = "NSGPIOPINT0_INT1."]
    Nsgpiopint0Int1 = 0x01,
    #[doc = "NSGPIOPINT0_INT2."]
    Nsgpiopint0Int2 = 0x02,
    #[doc = "NSGPIOPINT0_INT3."]
    Nsgpiopint0Int3 = 0x03,
    #[doc = "CT32BIT0_DMAREQ_M0."]
    Ct32bit0DmareqM0 = 0x04,
    #[doc = "CT32BIT0_DMAREQ_M1."]
    Ct32bit0DmareqM1 = 0x05,
    #[doc = "CT32BIT1_DMAREQ_M0."]
    Ct32bit1DmareqM0 = 0x06,
    #[doc = "CT32BIT1_DMAREQ_M1."]
    Ct32bit1DmareqM1 = 0x07,
    #[doc = "CT32BIT2_DMAREQ_M0."]
    Ct32bit2DmareqM0 = 0x08,
    #[doc = "CT32BIT2_DMAREQ_M1."]
    Ct32bit2DmareqM1 = 0x09,
    #[doc = "CT32BIT3_DMAREQ_M0."]
    Ct32bit3DmareqM0 = 0x0a,
    #[doc = "CT32BIT3_DMAREQ_M1."]
    Ct32bit3DmareqM1 = 0x0b,
    #[doc = "CT32BIT4_DMAREQ_M0."]
    Ct32bit4DmareqM0 = 0x0c,
    #[doc = "CT32BIT4_DMAREQ_M1."]
    Ct32bit4DmareqM1 = 0x0d,
    #[doc = "DMAC0_TRIGOUT_A."]
    Dmac0TrigoutA = 0x0e,
    #[doc = "DMAC0_TRIGOUT_B."]
    Dmac0TrigoutB = 0x0f,
    #[doc = "DMAC0_TRIGOUT_C."]
    Dmac0TrigoutC = 0x10,
    #[doc = "DMAC0_TRIGOUT_D."]
    Dmac0TrigoutD = 0x11,
    #[doc = "SCT0_DMA0."]
    Sct0Dma0 = 0x12,
    #[doc = "SCT0_DMA1."]
    Sct0Dma1 = 0x13,
    #[doc = "HASHCRYPT_OUT_DMA."]
    HashcryptOutDma = 0x14,
    #[doc = "ACMP_DMA."]
    AcmpDma = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    #[doc = "ADC_DMAC."]
    AdcDmac = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    #[doc = "FLEXSPI_RX."]
    FlexspiRx = 0x1c,
    #[doc = "FLEXSPI_TX."]
    FlexspiTx = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Dma0ItrigSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma0ItrigSel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma0ItrigSel {
    #[inline(always)]
    fn from(val: u8) -> Dma0ItrigSel {
        Dma0ItrigSel::from_bits(val)
    }
}
impl From<Dma0ItrigSel> for u8 {
    #[inline(always)]
    fn from(val: Dma0ItrigSel) -> u8 {
        Dma0ItrigSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma1ItrigSel {
    #[doc = "NSGPIOPINT0_INT0."]
    Nsgpiopint0Int0 = 0x0,
    #[doc = "NSGPIOPINT0_INT1."]
    Nsgpiopint0Int1 = 0x01,
    #[doc = "NSGPIOPINT0_INT2."]
    Nsgpiopint0Int2 = 0x02,
    #[doc = "NSGPIOPINT0_INT3."]
    Nsgpiopint0Int3 = 0x03,
    #[doc = "CT32BIT0_DMAREQ_M0."]
    Ct32bit0DmareqM0 = 0x04,
    #[doc = "CT32BIT0_DMAREQ_M1."]
    Ct32bit0DmareqM1 = 0x05,
    #[doc = "CT32BIT1_DMAREQ_M0."]
    Ct32bit1DmareqM0 = 0x06,
    #[doc = "CT32BIT1_DMAREQ_M1."]
    Ct32bit1DmareqM1 = 0x07,
    #[doc = "CT32BIT2_DMAREQ_M0."]
    Ct32bit2DmareqM0 = 0x08,
    #[doc = "CT32BIT2_DMAREQ_M1."]
    Ct32bit2DmareqM1 = 0x09,
    #[doc = "CT32BIT3_DMAREQ_M0."]
    Ct32bit3DmareqM0 = 0x0a,
    #[doc = "CT32BIT3_DMAREQ_M1."]
    Ct32bit3DmareqM1 = 0x0b,
    #[doc = "CT32BIT4_DMAREQ_M0."]
    Ct32bit4DmareqM0 = 0x0c,
    #[doc = "CT32BIT4_DMAREQ_M1."]
    Ct32bit4DmareqM1 = 0x0d,
    #[doc = "DMAC1_TRIGOUT_A."]
    Dmac1TrigoutA = 0x0e,
    #[doc = "DMAC1_TRIGOUT_B."]
    Dmac1TrigoutB = 0x0f,
    #[doc = "DMAC1_TRIGOUT_C."]
    Dmac1TrigoutC = 0x10,
    #[doc = "DMAC0_TRIGOUT_D."]
    Dmac1TrigoutD = 0x11,
    #[doc = "SCT0_DMAC0."]
    Sct0Dmac0 = 0x12,
    #[doc = "SCT0_DMAC1."]
    Sct0Dmac1 = 0x13,
    #[doc = "HASHCRYPT_OUT_DMA."]
    HashcryptOutDma = 0x14,
    #[doc = "ACMP_DMA."]
    AcmpDma = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    #[doc = "ADC_DMAC."]
    AdcDmac = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    #[doc = "FLEXSPI_RX."]
    FlexspiRx = 0x1c,
    #[doc = "FLEXSPI_TX."]
    FlexspiTx = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Dma1ItrigSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma1ItrigSel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma1ItrigSel {
    #[inline(always)]
    fn from(val: u8) -> Dma1ItrigSel {
        Dma1ItrigSel::from_bits(val)
    }
}
impl From<Dma1ItrigSel> for u8 {
    #[inline(always)]
    fn from(val: Dma1ItrigSel) -> u8 {
        Dma1ItrigSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0ClrDmac0ItrigInmux0 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ItrigEna0ClrDmac0ItrigInmux0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0ClrDmac0ItrigInmux0 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux0 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux0::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0ClrDmac0ItrigInmux0> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0ClrDmac0ItrigInmux0) -> u8 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0ClrDmac0ItrigInmux1 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ItrigEna0ClrDmac0ItrigInmux1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0ClrDmac0ItrigInmux1 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux1 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux1::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0ClrDmac0ItrigInmux1> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0ClrDmac0ItrigInmux1) -> u8 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0ClrDmac0ItrigInmux10 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ItrigEna0ClrDmac0ItrigInmux10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0ClrDmac0ItrigInmux10 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux10 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux10::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0ClrDmac0ItrigInmux10> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0ClrDmac0ItrigInmux10) -> u8 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0ClrDmac0ItrigInmux11 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ItrigEna0ClrDmac0ItrigInmux11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0ClrDmac0ItrigInmux11 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux11 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux11::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0ClrDmac0ItrigInmux11> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0ClrDmac0ItrigInmux11) -> u8 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0ClrDmac0ItrigInmux12 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ItrigEna0ClrDmac0ItrigInmux12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0ClrDmac0ItrigInmux12 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux12 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux12::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0ClrDmac0ItrigInmux12> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0ClrDmac0ItrigInmux12) -> u8 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0ClrDmac0ItrigInmux13 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ItrigEna0ClrDmac0ItrigInmux13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0ClrDmac0ItrigInmux13 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux13 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux13::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0ClrDmac0ItrigInmux13> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0ClrDmac0ItrigInmux13) -> u8 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0ClrDmac0ItrigInmux14 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ItrigEna0ClrDmac0ItrigInmux14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0ClrDmac0ItrigInmux14 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux14 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux14::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0ClrDmac0ItrigInmux14> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0ClrDmac0ItrigInmux14) -> u8 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0ClrDmac0ItrigInmux15 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ItrigEna0ClrDmac0ItrigInmux15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0ClrDmac0ItrigInmux15 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux15 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux15::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0ClrDmac0ItrigInmux15> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0ClrDmac0ItrigInmux15) -> u8 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0ClrDmac0ItrigInmux16 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ItrigEna0ClrDmac0ItrigInmux16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0ClrDmac0ItrigInmux16 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux16 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux16::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0ClrDmac0ItrigInmux16> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0ClrDmac0ItrigInmux16) -> u8 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0ClrDmac0ItrigInmux17 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ItrigEna0ClrDmac0ItrigInmux17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0ClrDmac0ItrigInmux17 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux17 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux17::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0ClrDmac0ItrigInmux17> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0ClrDmac0ItrigInmux17) -> u8 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0ClrDmac0ItrigInmux18 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ItrigEna0ClrDmac0ItrigInmux18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0ClrDmac0ItrigInmux18 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux18 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux18::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0ClrDmac0ItrigInmux18> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0ClrDmac0ItrigInmux18) -> u8 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0ClrDmac0ItrigInmux19 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ItrigEna0ClrDmac0ItrigInmux19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0ClrDmac0ItrigInmux19 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux19 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux19::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0ClrDmac0ItrigInmux19> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0ClrDmac0ItrigInmux19) -> u8 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0ClrDmac0ItrigInmux20 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ItrigEna0ClrDmac0ItrigInmux20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0ClrDmac0ItrigInmux20 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux20 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux20::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0ClrDmac0ItrigInmux20> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0ClrDmac0ItrigInmux20) -> u8 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0ClrDmac0ItrigInmux21 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ItrigEna0ClrDmac0ItrigInmux21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0ClrDmac0ItrigInmux21 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux21 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux21::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0ClrDmac0ItrigInmux21> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0ClrDmac0ItrigInmux21) -> u8 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0ClrDmac0ItrigInmux22 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ItrigEna0ClrDmac0ItrigInmux22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0ClrDmac0ItrigInmux22 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux22 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux22::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0ClrDmac0ItrigInmux22> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0ClrDmac0ItrigInmux22) -> u8 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0ClrDmac0ItrigInmux23 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ItrigEna0ClrDmac0ItrigInmux23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0ClrDmac0ItrigInmux23 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux23 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux23::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0ClrDmac0ItrigInmux23> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0ClrDmac0ItrigInmux23) -> u8 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0ClrDmac0ItrigInmux24 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ItrigEna0ClrDmac0ItrigInmux24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0ClrDmac0ItrigInmux24 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux24 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux24::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0ClrDmac0ItrigInmux24> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0ClrDmac0ItrigInmux24) -> u8 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0ClrDmac0ItrigInmux25 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ItrigEna0ClrDmac0ItrigInmux25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0ClrDmac0ItrigInmux25 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux25 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux25::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0ClrDmac0ItrigInmux25> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0ClrDmac0ItrigInmux25) -> u8 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0ClrDmac0ItrigInmux26 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ItrigEna0ClrDmac0ItrigInmux26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0ClrDmac0ItrigInmux26 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux26 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux26::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0ClrDmac0ItrigInmux26> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0ClrDmac0ItrigInmux26) -> u8 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0ClrDmac0ItrigInmux27 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ItrigEna0ClrDmac0ItrigInmux27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0ClrDmac0ItrigInmux27 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux27 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux27::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0ClrDmac0ItrigInmux27> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0ClrDmac0ItrigInmux27) -> u8 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0ClrDmac0ItrigInmux28 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ItrigEna0ClrDmac0ItrigInmux28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0ClrDmac0ItrigInmux28 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux28 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux28::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0ClrDmac0ItrigInmux28> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0ClrDmac0ItrigInmux28) -> u8 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0ClrDmac0ItrigInmux29 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ItrigEna0ClrDmac0ItrigInmux29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0ClrDmac0ItrigInmux29 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux29 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux29::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0ClrDmac0ItrigInmux29> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0ClrDmac0ItrigInmux29) -> u8 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0ClrDmac0ItrigInmux3 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ItrigEna0ClrDmac0ItrigInmux3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0ClrDmac0ItrigInmux3 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux3 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux3::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0ClrDmac0ItrigInmux3> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0ClrDmac0ItrigInmux3) -> u8 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0ClrDmac0ItrigInmux30 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ItrigEna0ClrDmac0ItrigInmux30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0ClrDmac0ItrigInmux30 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux30 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux30::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0ClrDmac0ItrigInmux30> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0ClrDmac0ItrigInmux30) -> u8 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0ClrDmac0ItrigInmux31 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ItrigEna0ClrDmac0ItrigInmux31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0ClrDmac0ItrigInmux31 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux31 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux31::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0ClrDmac0ItrigInmux31> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0ClrDmac0ItrigInmux31) -> u8 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0ClrDmac0ItrigInmux4 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ItrigEna0ClrDmac0ItrigInmux4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0ClrDmac0ItrigInmux4 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux4 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux4::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0ClrDmac0ItrigInmux4> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0ClrDmac0ItrigInmux4) -> u8 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0ClrDmac0ItrigInmux5 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ItrigEna0ClrDmac0ItrigInmux5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0ClrDmac0ItrigInmux5 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux5 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux5::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0ClrDmac0ItrigInmux5> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0ClrDmac0ItrigInmux5) -> u8 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0ClrDmac0ItrigInmux6 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ItrigEna0ClrDmac0ItrigInmux6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0ClrDmac0ItrigInmux6 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux6 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux6::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0ClrDmac0ItrigInmux6> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0ClrDmac0ItrigInmux6) -> u8 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0ClrDmac0ItrigInmux7 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ItrigEna0ClrDmac0ItrigInmux7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0ClrDmac0ItrigInmux7 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux7 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux7::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0ClrDmac0ItrigInmux7> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0ClrDmac0ItrigInmux7) -> u8 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0ClrDmac0ItrigInmux8 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ItrigEna0ClrDmac0ItrigInmux8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0ClrDmac0ItrigInmux8 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux8 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux8::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0ClrDmac0ItrigInmux8> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0ClrDmac0ItrigInmux8) -> u8 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0ClrDmac0ItrigInmux9 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ItrigEna0ClrDmac0ItrigInmux9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0ClrDmac0ItrigInmux9 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0ClrDmac0ItrigInmux9 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux9::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0ClrDmac0ItrigInmux9> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0ClrDmac0ItrigInmux9) -> u8 {
        Dmac0ItrigEna0ClrDmac0ItrigInmux9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0SetDmac0ItrigInmux0 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ItrigEna0SetDmac0ItrigInmux0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0SetDmac0ItrigInmux0 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux0 {
        Dmac0ItrigEna0SetDmac0ItrigInmux0::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0SetDmac0ItrigInmux0> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0SetDmac0ItrigInmux0) -> u8 {
        Dmac0ItrigEna0SetDmac0ItrigInmux0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0SetDmac0ItrigInmux1 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ItrigEna0SetDmac0ItrigInmux1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0SetDmac0ItrigInmux1 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux1 {
        Dmac0ItrigEna0SetDmac0ItrigInmux1::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0SetDmac0ItrigInmux1> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0SetDmac0ItrigInmux1) -> u8 {
        Dmac0ItrigEna0SetDmac0ItrigInmux1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0SetDmac0ItrigInmux10 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ItrigEna0SetDmac0ItrigInmux10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0SetDmac0ItrigInmux10 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux10 {
        Dmac0ItrigEna0SetDmac0ItrigInmux10::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0SetDmac0ItrigInmux10> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0SetDmac0ItrigInmux10) -> u8 {
        Dmac0ItrigEna0SetDmac0ItrigInmux10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0SetDmac0ItrigInmux11 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ItrigEna0SetDmac0ItrigInmux11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0SetDmac0ItrigInmux11 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux11 {
        Dmac0ItrigEna0SetDmac0ItrigInmux11::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0SetDmac0ItrigInmux11> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0SetDmac0ItrigInmux11) -> u8 {
        Dmac0ItrigEna0SetDmac0ItrigInmux11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0SetDmac0ItrigInmux12 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ItrigEna0SetDmac0ItrigInmux12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0SetDmac0ItrigInmux12 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux12 {
        Dmac0ItrigEna0SetDmac0ItrigInmux12::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0SetDmac0ItrigInmux12> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0SetDmac0ItrigInmux12) -> u8 {
        Dmac0ItrigEna0SetDmac0ItrigInmux12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0SetDmac0ItrigInmux13 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ItrigEna0SetDmac0ItrigInmux13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0SetDmac0ItrigInmux13 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux13 {
        Dmac0ItrigEna0SetDmac0ItrigInmux13::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0SetDmac0ItrigInmux13> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0SetDmac0ItrigInmux13) -> u8 {
        Dmac0ItrigEna0SetDmac0ItrigInmux13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0SetDmac0ItrigInmux14 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ItrigEna0SetDmac0ItrigInmux14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0SetDmac0ItrigInmux14 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux14 {
        Dmac0ItrigEna0SetDmac0ItrigInmux14::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0SetDmac0ItrigInmux14> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0SetDmac0ItrigInmux14) -> u8 {
        Dmac0ItrigEna0SetDmac0ItrigInmux14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0SetDmac0ItrigInmux15 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ItrigEna0SetDmac0ItrigInmux15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0SetDmac0ItrigInmux15 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux15 {
        Dmac0ItrigEna0SetDmac0ItrigInmux15::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0SetDmac0ItrigInmux15> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0SetDmac0ItrigInmux15) -> u8 {
        Dmac0ItrigEna0SetDmac0ItrigInmux15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0SetDmac0ItrigInmux16 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ItrigEna0SetDmac0ItrigInmux16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0SetDmac0ItrigInmux16 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux16 {
        Dmac0ItrigEna0SetDmac0ItrigInmux16::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0SetDmac0ItrigInmux16> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0SetDmac0ItrigInmux16) -> u8 {
        Dmac0ItrigEna0SetDmac0ItrigInmux16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0SetDmac0ItrigInmux17 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ItrigEna0SetDmac0ItrigInmux17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0SetDmac0ItrigInmux17 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux17 {
        Dmac0ItrigEna0SetDmac0ItrigInmux17::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0SetDmac0ItrigInmux17> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0SetDmac0ItrigInmux17) -> u8 {
        Dmac0ItrigEna0SetDmac0ItrigInmux17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0SetDmac0ItrigInmux18 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ItrigEna0SetDmac0ItrigInmux18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0SetDmac0ItrigInmux18 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux18 {
        Dmac0ItrigEna0SetDmac0ItrigInmux18::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0SetDmac0ItrigInmux18> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0SetDmac0ItrigInmux18) -> u8 {
        Dmac0ItrigEna0SetDmac0ItrigInmux18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0SetDmac0ItrigInmux19 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ItrigEna0SetDmac0ItrigInmux19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0SetDmac0ItrigInmux19 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux19 {
        Dmac0ItrigEna0SetDmac0ItrigInmux19::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0SetDmac0ItrigInmux19> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0SetDmac0ItrigInmux19) -> u8 {
        Dmac0ItrigEna0SetDmac0ItrigInmux19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0SetDmac0ItrigInmux20 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ItrigEna0SetDmac0ItrigInmux20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0SetDmac0ItrigInmux20 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux20 {
        Dmac0ItrigEna0SetDmac0ItrigInmux20::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0SetDmac0ItrigInmux20> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0SetDmac0ItrigInmux20) -> u8 {
        Dmac0ItrigEna0SetDmac0ItrigInmux20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0SetDmac0ItrigInmux21 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ItrigEna0SetDmac0ItrigInmux21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0SetDmac0ItrigInmux21 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux21 {
        Dmac0ItrigEna0SetDmac0ItrigInmux21::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0SetDmac0ItrigInmux21> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0SetDmac0ItrigInmux21) -> u8 {
        Dmac0ItrigEna0SetDmac0ItrigInmux21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0SetDmac0ItrigInmux22 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ItrigEna0SetDmac0ItrigInmux22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0SetDmac0ItrigInmux22 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux22 {
        Dmac0ItrigEna0SetDmac0ItrigInmux22::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0SetDmac0ItrigInmux22> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0SetDmac0ItrigInmux22) -> u8 {
        Dmac0ItrigEna0SetDmac0ItrigInmux22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0SetDmac0ItrigInmux23 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ItrigEna0SetDmac0ItrigInmux23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0SetDmac0ItrigInmux23 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux23 {
        Dmac0ItrigEna0SetDmac0ItrigInmux23::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0SetDmac0ItrigInmux23> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0SetDmac0ItrigInmux23) -> u8 {
        Dmac0ItrigEna0SetDmac0ItrigInmux23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0SetDmac0ItrigInmux24 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ItrigEna0SetDmac0ItrigInmux24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0SetDmac0ItrigInmux24 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux24 {
        Dmac0ItrigEna0SetDmac0ItrigInmux24::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0SetDmac0ItrigInmux24> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0SetDmac0ItrigInmux24) -> u8 {
        Dmac0ItrigEna0SetDmac0ItrigInmux24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0SetDmac0ItrigInmux25 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ItrigEna0SetDmac0ItrigInmux25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0SetDmac0ItrigInmux25 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux25 {
        Dmac0ItrigEna0SetDmac0ItrigInmux25::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0SetDmac0ItrigInmux25> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0SetDmac0ItrigInmux25) -> u8 {
        Dmac0ItrigEna0SetDmac0ItrigInmux25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0SetDmac0ItrigInmux26 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ItrigEna0SetDmac0ItrigInmux26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0SetDmac0ItrigInmux26 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux26 {
        Dmac0ItrigEna0SetDmac0ItrigInmux26::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0SetDmac0ItrigInmux26> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0SetDmac0ItrigInmux26) -> u8 {
        Dmac0ItrigEna0SetDmac0ItrigInmux26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0SetDmac0ItrigInmux27 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ItrigEna0SetDmac0ItrigInmux27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0SetDmac0ItrigInmux27 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux27 {
        Dmac0ItrigEna0SetDmac0ItrigInmux27::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0SetDmac0ItrigInmux27> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0SetDmac0ItrigInmux27) -> u8 {
        Dmac0ItrigEna0SetDmac0ItrigInmux27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0SetDmac0ItrigInmux28 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ItrigEna0SetDmac0ItrigInmux28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0SetDmac0ItrigInmux28 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux28 {
        Dmac0ItrigEna0SetDmac0ItrigInmux28::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0SetDmac0ItrigInmux28> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0SetDmac0ItrigInmux28) -> u8 {
        Dmac0ItrigEna0SetDmac0ItrigInmux28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0SetDmac0ItrigInmux29 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ItrigEna0SetDmac0ItrigInmux29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0SetDmac0ItrigInmux29 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux29 {
        Dmac0ItrigEna0SetDmac0ItrigInmux29::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0SetDmac0ItrigInmux29> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0SetDmac0ItrigInmux29) -> u8 {
        Dmac0ItrigEna0SetDmac0ItrigInmux29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0SetDmac0ItrigInmux3 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ItrigEna0SetDmac0ItrigInmux3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0SetDmac0ItrigInmux3 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux3 {
        Dmac0ItrigEna0SetDmac0ItrigInmux3::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0SetDmac0ItrigInmux3> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0SetDmac0ItrigInmux3) -> u8 {
        Dmac0ItrigEna0SetDmac0ItrigInmux3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0SetDmac0ItrigInmux30 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ItrigEna0SetDmac0ItrigInmux30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0SetDmac0ItrigInmux30 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux30 {
        Dmac0ItrigEna0SetDmac0ItrigInmux30::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0SetDmac0ItrigInmux30> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0SetDmac0ItrigInmux30) -> u8 {
        Dmac0ItrigEna0SetDmac0ItrigInmux30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0SetDmac0ItrigInmux31 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ItrigEna0SetDmac0ItrigInmux31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0SetDmac0ItrigInmux31 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux31 {
        Dmac0ItrigEna0SetDmac0ItrigInmux31::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0SetDmac0ItrigInmux31> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0SetDmac0ItrigInmux31) -> u8 {
        Dmac0ItrigEna0SetDmac0ItrigInmux31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0SetDmac0ItrigInmux4 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ItrigEna0SetDmac0ItrigInmux4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0SetDmac0ItrigInmux4 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux4 {
        Dmac0ItrigEna0SetDmac0ItrigInmux4::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0SetDmac0ItrigInmux4> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0SetDmac0ItrigInmux4) -> u8 {
        Dmac0ItrigEna0SetDmac0ItrigInmux4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0SetDmac0ItrigInmux5 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ItrigEna0SetDmac0ItrigInmux5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0SetDmac0ItrigInmux5 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux5 {
        Dmac0ItrigEna0SetDmac0ItrigInmux5::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0SetDmac0ItrigInmux5> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0SetDmac0ItrigInmux5) -> u8 {
        Dmac0ItrigEna0SetDmac0ItrigInmux5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0SetDmac0ItrigInmux6 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ItrigEna0SetDmac0ItrigInmux6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0SetDmac0ItrigInmux6 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux6 {
        Dmac0ItrigEna0SetDmac0ItrigInmux6::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0SetDmac0ItrigInmux6> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0SetDmac0ItrigInmux6) -> u8 {
        Dmac0ItrigEna0SetDmac0ItrigInmux6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0SetDmac0ItrigInmux7 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ItrigEna0SetDmac0ItrigInmux7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0SetDmac0ItrigInmux7 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux7 {
        Dmac0ItrigEna0SetDmac0ItrigInmux7::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0SetDmac0ItrigInmux7> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0SetDmac0ItrigInmux7) -> u8 {
        Dmac0ItrigEna0SetDmac0ItrigInmux7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0SetDmac0ItrigInmux8 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ItrigEna0SetDmac0ItrigInmux8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0SetDmac0ItrigInmux8 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux8 {
        Dmac0ItrigEna0SetDmac0ItrigInmux8::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0SetDmac0ItrigInmux8> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0SetDmac0ItrigInmux8) -> u8 {
        Dmac0ItrigEna0SetDmac0ItrigInmux8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ItrigEna0SetDmac0ItrigInmux9 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ItrigEna0SetDmac0ItrigInmux9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ItrigEna0SetDmac0ItrigInmux9 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ItrigEna0SetDmac0ItrigInmux9 {
        Dmac0ItrigEna0SetDmac0ItrigInmux9::from_bits(val)
    }
}
impl From<Dmac0ItrigEna0SetDmac0ItrigInmux9> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ItrigEna0SetDmac0ItrigInmux9) -> u8 {
        Dmac0ItrigEna0SetDmac0ItrigInmux9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0OtrigSel {
    #[doc = "DMAC0_OTRIG_CH0."]
    Dmac0OtrigCh0 = 0x0,
    #[doc = "DMAC0_OTRIG_CH1."]
    Dmac0OtrigCh1 = 0x01,
    #[doc = "DMAC0_OTRIG_CH2."]
    Dmac0OtrigCh2 = 0x02,
    #[doc = "DMAC0_OTRIG_CH3."]
    Dmac0OtrigCh3 = 0x03,
    #[doc = "DMAC0_OTRIG_CH4."]
    Dmac0OtrigCh4 = 0x04,
    #[doc = "DMAC0_OTRIG_CH5."]
    Dmac0OtrigCh5 = 0x05,
    #[doc = "DMAC0_OTRIG_CH6."]
    Dmac0OtrigCh6 = 0x06,
    #[doc = "DMAC0_OTRIG_CH7."]
    Dmac0OtrigCh7 = 0x07,
    #[doc = "DMAC0_OTRIG_CH8."]
    Dmac0OtrigCh8 = 0x08,
    #[doc = "DMAC0_OTRIG_CH9."]
    Dmac0OtrigCh9 = 0x09,
    #[doc = "DMAC0_OTRIG_CH10."]
    Dmac0OtrigCh10 = 0x0a,
    #[doc = "DMAC0_OTRIG_CH11."]
    Dmac0OtrigCh11 = 0x0b,
    #[doc = "DMAC0_OTRIG_CH12."]
    Dmac0OtrigCh12 = 0x0c,
    #[doc = "DMAC0_OTRIG_CH13."]
    Dmac0OtrigCh13 = 0x0d,
    #[doc = "DMAC0_OTRIG_CH14."]
    Dmac0OtrigCh14 = 0x0e,
    #[doc = "DMAC0_OTRIG_CH15."]
    Dmac0OtrigCh15 = 0x0f,
    #[doc = "DMAC0_OTRIG_CH16."]
    Dmac0OtrigCh16 = 0x10,
    #[doc = "DMAC0_OTRIG_CH17."]
    Dmac0OtrigCh17 = 0x11,
    #[doc = "DMAC0_OTRIG_CH18."]
    Dmac0OtrigCh18 = 0x12,
    #[doc = "DMAC0_OTRIG_CH19."]
    Dmac0OtrigCh19 = 0x13,
    #[doc = "DMAC0_OTRIG_CH20."]
    Dmac0OtrigCh20 = 0x14,
    #[doc = "DMAC0_OTRIG_CH21."]
    Dmac0OtrigCh21 = 0x15,
    #[doc = "DMAC0_OTRIG_CH22."]
    Dmac0OtrigCh22 = 0x16,
    #[doc = "DMAC0_OTRIG_CH23."]
    Dmac0OtrigCh23 = 0x17,
    #[doc = "DMAC0_OTRIG_CH24."]
    Dmac0OtrigCh24 = 0x18,
    #[doc = "DMAC0_OTRIG_CH25."]
    Dmac0OtrigCh25 = 0x19,
    #[doc = "DMAC0_OTRIG_CH26."]
    Dmac0OtrigCh26 = 0x1a,
    #[doc = "DMAC0_OTRIG_CH27."]
    Dmac0OtrigCh27 = 0x1b,
    #[doc = "DMAC0_OTRIG_CH28."]
    Dmac0OtrigCh28 = 0x1c,
    #[doc = "DMAC0_OTRIG_CH29."]
    Dmac0OtrigCh29 = 0x1d,
    #[doc = "DMAC0_OTRIG_CH30."]
    Dmac0OtrigCh30 = 0x1e,
    #[doc = "DMAC0_OTRIG_CH31."]
    Dmac0OtrigCh31 = 0x1f,
    #[doc = "DMAC0_OTRIG_CH32."]
    Dmac0OtrigCh32 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Dmac0OtrigSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0OtrigSel {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0OtrigSel {
    #[inline(always)]
    fn from(val: u8) -> Dmac0OtrigSel {
        Dmac0OtrigSel::from_bits(val)
    }
}
impl From<Dmac0OtrigSel> for u8 {
    #[inline(always)]
    fn from(val: Dmac0OtrigSel) -> u8 {
        Dmac0OtrigSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0ClrDmic0ch0 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ReqEna0ClrDmic0ch0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0ClrDmic0ch0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0ClrDmic0ch0 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0ClrDmic0ch0 {
        Dmac0ReqEna0ClrDmic0ch0::from_bits(val)
    }
}
impl From<Dmac0ReqEna0ClrDmic0ch0> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0ClrDmic0ch0) -> u8 {
        Dmac0ReqEna0ClrDmic0ch0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0ClrDmic0ch1 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ReqEna0ClrDmic0ch1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0ClrDmic0ch1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0ClrDmic0ch1 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0ClrDmic0ch1 {
        Dmac0ReqEna0ClrDmic0ch1::from_bits(val)
    }
}
impl From<Dmac0ReqEna0ClrDmic0ch1> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0ClrDmic0ch1) -> u8 {
        Dmac0ReqEna0ClrDmic0ch1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0ClrDmic0ch2 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ReqEna0ClrDmic0ch2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0ClrDmic0ch2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0ClrDmic0ch2 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0ClrDmic0ch2 {
        Dmac0ReqEna0ClrDmic0ch2::from_bits(val)
    }
}
impl From<Dmac0ReqEna0ClrDmic0ch2> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0ClrDmic0ch2) -> u8 {
        Dmac0ReqEna0ClrDmic0ch2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0ClrDmic0ch3 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ReqEna0ClrDmic0ch3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0ClrDmic0ch3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0ClrDmic0ch3 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0ClrDmic0ch3 {
        Dmac0ReqEna0ClrDmic0ch3::from_bits(val)
    }
}
impl From<Dmac0ReqEna0ClrDmic0ch3> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0ClrDmic0ch3) -> u8 {
        Dmac0ReqEna0ClrDmic0ch3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0ClrDmic0ch4 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ReqEna0ClrDmic0ch4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0ClrDmic0ch4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0ClrDmic0ch4 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0ClrDmic0ch4 {
        Dmac0ReqEna0ClrDmic0ch4::from_bits(val)
    }
}
impl From<Dmac0ReqEna0ClrDmic0ch4> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0ClrDmic0ch4) -> u8 {
        Dmac0ReqEna0ClrDmic0ch4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0ClrDmic0ch5 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ReqEna0ClrDmic0ch5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0ClrDmic0ch5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0ClrDmic0ch5 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0ClrDmic0ch5 {
        Dmac0ReqEna0ClrDmic0ch5::from_bits(val)
    }
}
impl From<Dmac0ReqEna0ClrDmic0ch5> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0ClrDmic0ch5) -> u8 {
        Dmac0ReqEna0ClrDmic0ch5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0ClrDmic0ch6 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ReqEna0ClrDmic0ch6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0ClrDmic0ch6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0ClrDmic0ch6 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0ClrDmic0ch6 {
        Dmac0ReqEna0ClrDmic0ch6::from_bits(val)
    }
}
impl From<Dmac0ReqEna0ClrDmic0ch6> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0ClrDmic0ch6) -> u8 {
        Dmac0ReqEna0ClrDmic0ch6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0ClrDmic0ch7 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ReqEna0ClrDmic0ch7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0ClrDmic0ch7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0ClrDmic0ch7 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0ClrDmic0ch7 {
        Dmac0ReqEna0ClrDmic0ch7::from_bits(val)
    }
}
impl From<Dmac0ReqEna0ClrDmic0ch7> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0ClrDmic0ch7) -> u8 {
        Dmac0ReqEna0ClrDmic0ch7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0ClrEspiCh0 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ReqEna0ClrEspiCh0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0ClrEspiCh0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0ClrEspiCh0 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0ClrEspiCh0 {
        Dmac0ReqEna0ClrEspiCh0::from_bits(val)
    }
}
impl From<Dmac0ReqEna0ClrEspiCh0> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0ClrEspiCh0) -> u8 {
        Dmac0ReqEna0ClrEspiCh0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0ClrFlexcomm0Rx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ReqEna0ClrFlexcomm0Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0ClrFlexcomm0Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0ClrFlexcomm0Rx {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0ClrFlexcomm0Rx {
        Dmac0ReqEna0ClrFlexcomm0Rx::from_bits(val)
    }
}
impl From<Dmac0ReqEna0ClrFlexcomm0Rx> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0ClrFlexcomm0Rx) -> u8 {
        Dmac0ReqEna0ClrFlexcomm0Rx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0ClrFlexcomm0Tx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ReqEna0ClrFlexcomm0Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0ClrFlexcomm0Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0ClrFlexcomm0Tx {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0ClrFlexcomm0Tx {
        Dmac0ReqEna0ClrFlexcomm0Tx::from_bits(val)
    }
}
impl From<Dmac0ReqEna0ClrFlexcomm0Tx> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0ClrFlexcomm0Tx) -> u8 {
        Dmac0ReqEna0ClrFlexcomm0Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0ClrFlexcomm14Rx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ReqEna0ClrFlexcomm14Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0ClrFlexcomm14Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0ClrFlexcomm14Rx {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0ClrFlexcomm14Rx {
        Dmac0ReqEna0ClrFlexcomm14Rx::from_bits(val)
    }
}
impl From<Dmac0ReqEna0ClrFlexcomm14Rx> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0ClrFlexcomm14Rx) -> u8 {
        Dmac0ReqEna0ClrFlexcomm14Rx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0ClrFlexcomm14Tx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ReqEna0ClrFlexcomm14Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0ClrFlexcomm14Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0ClrFlexcomm14Tx {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0ClrFlexcomm14Tx {
        Dmac0ReqEna0ClrFlexcomm14Tx::from_bits(val)
    }
}
impl From<Dmac0ReqEna0ClrFlexcomm14Tx> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0ClrFlexcomm14Tx) -> u8 {
        Dmac0ReqEna0ClrFlexcomm14Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0ClrFlexcomm1Rx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ReqEna0ClrFlexcomm1Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0ClrFlexcomm1Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0ClrFlexcomm1Rx {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0ClrFlexcomm1Rx {
        Dmac0ReqEna0ClrFlexcomm1Rx::from_bits(val)
    }
}
impl From<Dmac0ReqEna0ClrFlexcomm1Rx> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0ClrFlexcomm1Rx) -> u8 {
        Dmac0ReqEna0ClrFlexcomm1Rx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0ClrFlexcomm1Tx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ReqEna0ClrFlexcomm1Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0ClrFlexcomm1Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0ClrFlexcomm1Tx {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0ClrFlexcomm1Tx {
        Dmac0ReqEna0ClrFlexcomm1Tx::from_bits(val)
    }
}
impl From<Dmac0ReqEna0ClrFlexcomm1Tx> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0ClrFlexcomm1Tx) -> u8 {
        Dmac0ReqEna0ClrFlexcomm1Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0ClrFlexcomm2Rx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ReqEna0ClrFlexcomm2Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0ClrFlexcomm2Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0ClrFlexcomm2Rx {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0ClrFlexcomm2Rx {
        Dmac0ReqEna0ClrFlexcomm2Rx::from_bits(val)
    }
}
impl From<Dmac0ReqEna0ClrFlexcomm2Rx> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0ClrFlexcomm2Rx) -> u8 {
        Dmac0ReqEna0ClrFlexcomm2Rx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0ClrFlexcomm2Tx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ReqEna0ClrFlexcomm2Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0ClrFlexcomm2Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0ClrFlexcomm2Tx {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0ClrFlexcomm2Tx {
        Dmac0ReqEna0ClrFlexcomm2Tx::from_bits(val)
    }
}
impl From<Dmac0ReqEna0ClrFlexcomm2Tx> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0ClrFlexcomm2Tx) -> u8 {
        Dmac0ReqEna0ClrFlexcomm2Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0ClrFlexcomm3Rx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ReqEna0ClrFlexcomm3Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0ClrFlexcomm3Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0ClrFlexcomm3Rx {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0ClrFlexcomm3Rx {
        Dmac0ReqEna0ClrFlexcomm3Rx::from_bits(val)
    }
}
impl From<Dmac0ReqEna0ClrFlexcomm3Rx> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0ClrFlexcomm3Rx) -> u8 {
        Dmac0ReqEna0ClrFlexcomm3Rx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0ClrFlexcomm3Tx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ReqEna0ClrFlexcomm3Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0ClrFlexcomm3Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0ClrFlexcomm3Tx {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0ClrFlexcomm3Tx {
        Dmac0ReqEna0ClrFlexcomm3Tx::from_bits(val)
    }
}
impl From<Dmac0ReqEna0ClrFlexcomm3Tx> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0ClrFlexcomm3Tx) -> u8 {
        Dmac0ReqEna0ClrFlexcomm3Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0ClrFlexcomm4Rx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ReqEna0ClrFlexcomm4Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0ClrFlexcomm4Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0ClrFlexcomm4Rx {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0ClrFlexcomm4Rx {
        Dmac0ReqEna0ClrFlexcomm4Rx::from_bits(val)
    }
}
impl From<Dmac0ReqEna0ClrFlexcomm4Rx> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0ClrFlexcomm4Rx) -> u8 {
        Dmac0ReqEna0ClrFlexcomm4Rx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0ClrFlexcomm4Tx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ReqEna0ClrFlexcomm4Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0ClrFlexcomm4Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0ClrFlexcomm4Tx {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0ClrFlexcomm4Tx {
        Dmac0ReqEna0ClrFlexcomm4Tx::from_bits(val)
    }
}
impl From<Dmac0ReqEna0ClrFlexcomm4Tx> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0ClrFlexcomm4Tx) -> u8 {
        Dmac0ReqEna0ClrFlexcomm4Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0ClrFlexcomm5Rx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ReqEna0ClrFlexcomm5Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0ClrFlexcomm5Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0ClrFlexcomm5Rx {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0ClrFlexcomm5Rx {
        Dmac0ReqEna0ClrFlexcomm5Rx::from_bits(val)
    }
}
impl From<Dmac0ReqEna0ClrFlexcomm5Rx> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0ClrFlexcomm5Rx) -> u8 {
        Dmac0ReqEna0ClrFlexcomm5Rx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0ClrFlexcomm5Tx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ReqEna0ClrFlexcomm5Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0ClrFlexcomm5Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0ClrFlexcomm5Tx {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0ClrFlexcomm5Tx {
        Dmac0ReqEna0ClrFlexcomm5Tx::from_bits(val)
    }
}
impl From<Dmac0ReqEna0ClrFlexcomm5Tx> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0ClrFlexcomm5Tx) -> u8 {
        Dmac0ReqEna0ClrFlexcomm5Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0ClrFlexcomm6Rx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ReqEna0ClrFlexcomm6Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0ClrFlexcomm6Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0ClrFlexcomm6Rx {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0ClrFlexcomm6Rx {
        Dmac0ReqEna0ClrFlexcomm6Rx::from_bits(val)
    }
}
impl From<Dmac0ReqEna0ClrFlexcomm6Rx> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0ClrFlexcomm6Rx) -> u8 {
        Dmac0ReqEna0ClrFlexcomm6Rx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0ClrFlexcomm6Tx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ReqEna0ClrFlexcomm6Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0ClrFlexcomm6Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0ClrFlexcomm6Tx {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0ClrFlexcomm6Tx {
        Dmac0ReqEna0ClrFlexcomm6Tx::from_bits(val)
    }
}
impl From<Dmac0ReqEna0ClrFlexcomm6Tx> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0ClrFlexcomm6Tx) -> u8 {
        Dmac0ReqEna0ClrFlexcomm6Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0ClrFlexcomm7Rx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ReqEna0ClrFlexcomm7Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0ClrFlexcomm7Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0ClrFlexcomm7Rx {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0ClrFlexcomm7Rx {
        Dmac0ReqEna0ClrFlexcomm7Rx::from_bits(val)
    }
}
impl From<Dmac0ReqEna0ClrFlexcomm7Rx> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0ClrFlexcomm7Rx) -> u8 {
        Dmac0ReqEna0ClrFlexcomm7Rx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0ClrFlexcomm7Tx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ReqEna0ClrFlexcomm7Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0ClrFlexcomm7Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0ClrFlexcomm7Tx {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0ClrFlexcomm7Tx {
        Dmac0ReqEna0ClrFlexcomm7Tx::from_bits(val)
    }
}
impl From<Dmac0ReqEna0ClrFlexcomm7Tx> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0ClrFlexcomm7Tx) -> u8 {
        Dmac0ReqEna0ClrFlexcomm7Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0ClrHashcrypt {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ReqEna0ClrHashcrypt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0ClrHashcrypt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0ClrHashcrypt {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0ClrHashcrypt {
        Dmac0ReqEna0ClrHashcrypt::from_bits(val)
    }
}
impl From<Dmac0ReqEna0ClrHashcrypt> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0ClrHashcrypt) -> u8 {
        Dmac0ReqEna0ClrHashcrypt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0ClrI3c0Rx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ReqEna0ClrI3c0Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0ClrI3c0Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0ClrI3c0Rx {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0ClrI3c0Rx {
        Dmac0ReqEna0ClrI3c0Rx::from_bits(val)
    }
}
impl From<Dmac0ReqEna0ClrI3c0Rx> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0ClrI3c0Rx) -> u8 {
        Dmac0ReqEna0ClrI3c0Rx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0ClrI3c0Tx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ReqEna0ClrI3c0Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0ClrI3c0Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0ClrI3c0Tx {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0ClrI3c0Tx {
        Dmac0ReqEna0ClrI3c0Tx::from_bits(val)
    }
}
impl From<Dmac0ReqEna0ClrI3c0Tx> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0ClrI3c0Tx) -> u8 {
        Dmac0ReqEna0ClrI3c0Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0SetDmic0ch0 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ReqEna0SetDmic0ch0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0SetDmic0ch0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0SetDmic0ch0 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0SetDmic0ch0 {
        Dmac0ReqEna0SetDmic0ch0::from_bits(val)
    }
}
impl From<Dmac0ReqEna0SetDmic0ch0> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0SetDmic0ch0) -> u8 {
        Dmac0ReqEna0SetDmic0ch0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0SetDmic0ch1 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ReqEna0SetDmic0ch1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0SetDmic0ch1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0SetDmic0ch1 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0SetDmic0ch1 {
        Dmac0ReqEna0SetDmic0ch1::from_bits(val)
    }
}
impl From<Dmac0ReqEna0SetDmic0ch1> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0SetDmic0ch1) -> u8 {
        Dmac0ReqEna0SetDmic0ch1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0SetDmic0ch2 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ReqEna0SetDmic0ch2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0SetDmic0ch2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0SetDmic0ch2 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0SetDmic0ch2 {
        Dmac0ReqEna0SetDmic0ch2::from_bits(val)
    }
}
impl From<Dmac0ReqEna0SetDmic0ch2> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0SetDmic0ch2) -> u8 {
        Dmac0ReqEna0SetDmic0ch2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0SetDmic0ch3 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ReqEna0SetDmic0ch3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0SetDmic0ch3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0SetDmic0ch3 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0SetDmic0ch3 {
        Dmac0ReqEna0SetDmic0ch3::from_bits(val)
    }
}
impl From<Dmac0ReqEna0SetDmic0ch3> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0SetDmic0ch3) -> u8 {
        Dmac0ReqEna0SetDmic0ch3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0SetDmic0ch4 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ReqEna0SetDmic0ch4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0SetDmic0ch4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0SetDmic0ch4 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0SetDmic0ch4 {
        Dmac0ReqEna0SetDmic0ch4::from_bits(val)
    }
}
impl From<Dmac0ReqEna0SetDmic0ch4> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0SetDmic0ch4) -> u8 {
        Dmac0ReqEna0SetDmic0ch4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0SetDmic0ch5 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ReqEna0SetDmic0ch5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0SetDmic0ch5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0SetDmic0ch5 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0SetDmic0ch5 {
        Dmac0ReqEna0SetDmic0ch5::from_bits(val)
    }
}
impl From<Dmac0ReqEna0SetDmic0ch5> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0SetDmic0ch5) -> u8 {
        Dmac0ReqEna0SetDmic0ch5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0SetDmic0ch6 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ReqEna0SetDmic0ch6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0SetDmic0ch6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0SetDmic0ch6 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0SetDmic0ch6 {
        Dmac0ReqEna0SetDmic0ch6::from_bits(val)
    }
}
impl From<Dmac0ReqEna0SetDmic0ch6> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0SetDmic0ch6) -> u8 {
        Dmac0ReqEna0SetDmic0ch6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0SetDmic0ch7 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ReqEna0SetDmic0ch7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0SetDmic0ch7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0SetDmic0ch7 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0SetDmic0ch7 {
        Dmac0ReqEna0SetDmic0ch7::from_bits(val)
    }
}
impl From<Dmac0ReqEna0SetDmic0ch7> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0SetDmic0ch7) -> u8 {
        Dmac0ReqEna0SetDmic0ch7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0SetEspiCh0 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ReqEna0SetEspiCh0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0SetEspiCh0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0SetEspiCh0 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0SetEspiCh0 {
        Dmac0ReqEna0SetEspiCh0::from_bits(val)
    }
}
impl From<Dmac0ReqEna0SetEspiCh0> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0SetEspiCh0) -> u8 {
        Dmac0ReqEna0SetEspiCh0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0SetFlexcomm0Rx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ReqEna0SetFlexcomm0Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0SetFlexcomm0Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0SetFlexcomm0Rx {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0SetFlexcomm0Rx {
        Dmac0ReqEna0SetFlexcomm0Rx::from_bits(val)
    }
}
impl From<Dmac0ReqEna0SetFlexcomm0Rx> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0SetFlexcomm0Rx) -> u8 {
        Dmac0ReqEna0SetFlexcomm0Rx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0SetFlexcomm0Tx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ReqEna0SetFlexcomm0Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0SetFlexcomm0Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0SetFlexcomm0Tx {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0SetFlexcomm0Tx {
        Dmac0ReqEna0SetFlexcomm0Tx::from_bits(val)
    }
}
impl From<Dmac0ReqEna0SetFlexcomm0Tx> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0SetFlexcomm0Tx) -> u8 {
        Dmac0ReqEna0SetFlexcomm0Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0SetFlexcomm14Rx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ReqEna0SetFlexcomm14Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0SetFlexcomm14Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0SetFlexcomm14Rx {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0SetFlexcomm14Rx {
        Dmac0ReqEna0SetFlexcomm14Rx::from_bits(val)
    }
}
impl From<Dmac0ReqEna0SetFlexcomm14Rx> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0SetFlexcomm14Rx) -> u8 {
        Dmac0ReqEna0SetFlexcomm14Rx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0SetFlexcomm14Tx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ReqEna0SetFlexcomm14Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0SetFlexcomm14Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0SetFlexcomm14Tx {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0SetFlexcomm14Tx {
        Dmac0ReqEna0SetFlexcomm14Tx::from_bits(val)
    }
}
impl From<Dmac0ReqEna0SetFlexcomm14Tx> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0SetFlexcomm14Tx) -> u8 {
        Dmac0ReqEna0SetFlexcomm14Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0SetFlexcomm1Rx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ReqEna0SetFlexcomm1Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0SetFlexcomm1Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0SetFlexcomm1Rx {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0SetFlexcomm1Rx {
        Dmac0ReqEna0SetFlexcomm1Rx::from_bits(val)
    }
}
impl From<Dmac0ReqEna0SetFlexcomm1Rx> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0SetFlexcomm1Rx) -> u8 {
        Dmac0ReqEna0SetFlexcomm1Rx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0SetFlexcomm1Tx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ReqEna0SetFlexcomm1Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0SetFlexcomm1Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0SetFlexcomm1Tx {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0SetFlexcomm1Tx {
        Dmac0ReqEna0SetFlexcomm1Tx::from_bits(val)
    }
}
impl From<Dmac0ReqEna0SetFlexcomm1Tx> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0SetFlexcomm1Tx) -> u8 {
        Dmac0ReqEna0SetFlexcomm1Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0SetFlexcomm2Rx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ReqEna0SetFlexcomm2Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0SetFlexcomm2Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0SetFlexcomm2Rx {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0SetFlexcomm2Rx {
        Dmac0ReqEna0SetFlexcomm2Rx::from_bits(val)
    }
}
impl From<Dmac0ReqEna0SetFlexcomm2Rx> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0SetFlexcomm2Rx) -> u8 {
        Dmac0ReqEna0SetFlexcomm2Rx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0SetFlexcomm2Tx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ReqEna0SetFlexcomm2Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0SetFlexcomm2Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0SetFlexcomm2Tx {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0SetFlexcomm2Tx {
        Dmac0ReqEna0SetFlexcomm2Tx::from_bits(val)
    }
}
impl From<Dmac0ReqEna0SetFlexcomm2Tx> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0SetFlexcomm2Tx) -> u8 {
        Dmac0ReqEna0SetFlexcomm2Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0SetFlexcomm3Rx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ReqEna0SetFlexcomm3Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0SetFlexcomm3Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0SetFlexcomm3Rx {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0SetFlexcomm3Rx {
        Dmac0ReqEna0SetFlexcomm3Rx::from_bits(val)
    }
}
impl From<Dmac0ReqEna0SetFlexcomm3Rx> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0SetFlexcomm3Rx) -> u8 {
        Dmac0ReqEna0SetFlexcomm3Rx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0SetFlexcomm3Tx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ReqEna0SetFlexcomm3Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0SetFlexcomm3Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0SetFlexcomm3Tx {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0SetFlexcomm3Tx {
        Dmac0ReqEna0SetFlexcomm3Tx::from_bits(val)
    }
}
impl From<Dmac0ReqEna0SetFlexcomm3Tx> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0SetFlexcomm3Tx) -> u8 {
        Dmac0ReqEna0SetFlexcomm3Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0SetFlexcomm4Rx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ReqEna0SetFlexcomm4Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0SetFlexcomm4Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0SetFlexcomm4Rx {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0SetFlexcomm4Rx {
        Dmac0ReqEna0SetFlexcomm4Rx::from_bits(val)
    }
}
impl From<Dmac0ReqEna0SetFlexcomm4Rx> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0SetFlexcomm4Rx) -> u8 {
        Dmac0ReqEna0SetFlexcomm4Rx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0SetFlexcomm4Tx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ReqEna0SetFlexcomm4Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0SetFlexcomm4Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0SetFlexcomm4Tx {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0SetFlexcomm4Tx {
        Dmac0ReqEna0SetFlexcomm4Tx::from_bits(val)
    }
}
impl From<Dmac0ReqEna0SetFlexcomm4Tx> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0SetFlexcomm4Tx) -> u8 {
        Dmac0ReqEna0SetFlexcomm4Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0SetFlexcomm5Rx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ReqEna0SetFlexcomm5Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0SetFlexcomm5Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0SetFlexcomm5Rx {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0SetFlexcomm5Rx {
        Dmac0ReqEna0SetFlexcomm5Rx::from_bits(val)
    }
}
impl From<Dmac0ReqEna0SetFlexcomm5Rx> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0SetFlexcomm5Rx) -> u8 {
        Dmac0ReqEna0SetFlexcomm5Rx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0SetFlexcomm5Tx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ReqEna0SetFlexcomm5Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0SetFlexcomm5Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0SetFlexcomm5Tx {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0SetFlexcomm5Tx {
        Dmac0ReqEna0SetFlexcomm5Tx::from_bits(val)
    }
}
impl From<Dmac0ReqEna0SetFlexcomm5Tx> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0SetFlexcomm5Tx) -> u8 {
        Dmac0ReqEna0SetFlexcomm5Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0SetFlexcomm6Rx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ReqEna0SetFlexcomm6Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0SetFlexcomm6Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0SetFlexcomm6Rx {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0SetFlexcomm6Rx {
        Dmac0ReqEna0SetFlexcomm6Rx::from_bits(val)
    }
}
impl From<Dmac0ReqEna0SetFlexcomm6Rx> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0SetFlexcomm6Rx) -> u8 {
        Dmac0ReqEna0SetFlexcomm6Rx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0SetFlexcomm6Tx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ReqEna0SetFlexcomm6Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0SetFlexcomm6Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0SetFlexcomm6Tx {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0SetFlexcomm6Tx {
        Dmac0ReqEna0SetFlexcomm6Tx::from_bits(val)
    }
}
impl From<Dmac0ReqEna0SetFlexcomm6Tx> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0SetFlexcomm6Tx) -> u8 {
        Dmac0ReqEna0SetFlexcomm6Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0SetFlexcomm7Rx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ReqEna0SetFlexcomm7Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0SetFlexcomm7Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0SetFlexcomm7Rx {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0SetFlexcomm7Rx {
        Dmac0ReqEna0SetFlexcomm7Rx::from_bits(val)
    }
}
impl From<Dmac0ReqEna0SetFlexcomm7Rx> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0SetFlexcomm7Rx) -> u8 {
        Dmac0ReqEna0SetFlexcomm7Rx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0SetFlexcomm7Tx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ReqEna0SetFlexcomm7Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0SetFlexcomm7Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0SetFlexcomm7Tx {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0SetFlexcomm7Tx {
        Dmac0ReqEna0SetFlexcomm7Tx::from_bits(val)
    }
}
impl From<Dmac0ReqEna0SetFlexcomm7Tx> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0SetFlexcomm7Tx) -> u8 {
        Dmac0ReqEna0SetFlexcomm7Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0SetHashcrypt {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ReqEna0SetHashcrypt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0SetHashcrypt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0SetHashcrypt {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0SetHashcrypt {
        Dmac0ReqEna0SetHashcrypt::from_bits(val)
    }
}
impl From<Dmac0ReqEna0SetHashcrypt> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0SetHashcrypt) -> u8 {
        Dmac0ReqEna0SetHashcrypt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0SetI3c0Rx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ReqEna0SetI3c0Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0SetI3c0Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0SetI3c0Rx {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0SetI3c0Rx {
        Dmac0ReqEna0SetI3c0Rx::from_bits(val)
    }
}
impl From<Dmac0ReqEna0SetI3c0Rx> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0SetI3c0Rx) -> u8 {
        Dmac0ReqEna0SetI3c0Rx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna0SetI3c0Tx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ReqEna0SetI3c0Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna0SetI3c0Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna0SetI3c0Tx {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna0SetI3c0Tx {
        Dmac0ReqEna0SetI3c0Tx::from_bits(val)
    }
}
impl From<Dmac0ReqEna0SetI3c0Tx> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna0SetI3c0Tx) -> u8 {
        Dmac0ReqEna0SetI3c0Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna1ClrEspiCh1 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA1 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac0ReqEna1ClrEspiCh1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna1ClrEspiCh1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna1ClrEspiCh1 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna1ClrEspiCh1 {
        Dmac0ReqEna1ClrEspiCh1::from_bits(val)
    }
}
impl From<Dmac0ReqEna1ClrEspiCh1> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna1ClrEspiCh1) -> u8 {
        Dmac0ReqEna1ClrEspiCh1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac0ReqEna1SetEspiCh1 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA1 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac0ReqEna1SetEspiCh1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac0ReqEna1SetEspiCh1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac0ReqEna1SetEspiCh1 {
    #[inline(always)]
    fn from(val: u8) -> Dmac0ReqEna1SetEspiCh1 {
        Dmac0ReqEna1SetEspiCh1::from_bits(val)
    }
}
impl From<Dmac0ReqEna1SetEspiCh1> for u8 {
    #[inline(always)]
    fn from(val: Dmac0ReqEna1SetEspiCh1) -> u8 {
        Dmac0ReqEna1SetEspiCh1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0ClrDmac1ItrigInmux0 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ItrigEna0ClrDmac1ItrigInmux0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0ClrDmac1ItrigInmux0 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux0 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux0::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0ClrDmac1ItrigInmux0> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0ClrDmac1ItrigInmux0) -> u8 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0ClrDmac1ItrigInmux1 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ItrigEna0ClrDmac1ItrigInmux1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0ClrDmac1ItrigInmux1 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux1 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux1::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0ClrDmac1ItrigInmux1> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0ClrDmac1ItrigInmux1) -> u8 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0ClrDmac1ItrigInmux10 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ItrigEna0ClrDmac1ItrigInmux10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0ClrDmac1ItrigInmux10 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux10 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux10::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0ClrDmac1ItrigInmux10> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0ClrDmac1ItrigInmux10) -> u8 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0ClrDmac1ItrigInmux11 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ItrigEna0ClrDmac1ItrigInmux11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0ClrDmac1ItrigInmux11 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux11 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux11::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0ClrDmac1ItrigInmux11> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0ClrDmac1ItrigInmux11) -> u8 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0ClrDmac1ItrigInmux12 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ItrigEna0ClrDmac1ItrigInmux12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0ClrDmac1ItrigInmux12 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux12 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux12::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0ClrDmac1ItrigInmux12> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0ClrDmac1ItrigInmux12) -> u8 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0ClrDmac1ItrigInmux13 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ItrigEna0ClrDmac1ItrigInmux13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0ClrDmac1ItrigInmux13 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux13 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux13::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0ClrDmac1ItrigInmux13> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0ClrDmac1ItrigInmux13) -> u8 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0ClrDmac1ItrigInmux14 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ItrigEna0ClrDmac1ItrigInmux14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0ClrDmac1ItrigInmux14 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux14 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux14::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0ClrDmac1ItrigInmux14> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0ClrDmac1ItrigInmux14) -> u8 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0ClrDmac1ItrigInmux15 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ItrigEna0ClrDmac1ItrigInmux15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0ClrDmac1ItrigInmux15 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux15 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux15::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0ClrDmac1ItrigInmux15> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0ClrDmac1ItrigInmux15) -> u8 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0ClrDmac1ItrigInmux16 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ItrigEna0ClrDmac1ItrigInmux16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0ClrDmac1ItrigInmux16 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux16 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux16::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0ClrDmac1ItrigInmux16> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0ClrDmac1ItrigInmux16) -> u8 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0ClrDmac1ItrigInmux17 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ItrigEna0ClrDmac1ItrigInmux17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0ClrDmac1ItrigInmux17 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux17 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux17::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0ClrDmac1ItrigInmux17> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0ClrDmac1ItrigInmux17) -> u8 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0ClrDmac1ItrigInmux18 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ItrigEna0ClrDmac1ItrigInmux18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0ClrDmac1ItrigInmux18 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux18 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux18::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0ClrDmac1ItrigInmux18> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0ClrDmac1ItrigInmux18) -> u8 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0ClrDmac1ItrigInmux19 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ItrigEna0ClrDmac1ItrigInmux19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0ClrDmac1ItrigInmux19 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux19 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux19::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0ClrDmac1ItrigInmux19> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0ClrDmac1ItrigInmux19) -> u8 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0ClrDmac1ItrigInmux20 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ItrigEna0ClrDmac1ItrigInmux20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0ClrDmac1ItrigInmux20 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux20 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux20::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0ClrDmac1ItrigInmux20> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0ClrDmac1ItrigInmux20) -> u8 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0ClrDmac1ItrigInmux21 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ItrigEna0ClrDmac1ItrigInmux21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0ClrDmac1ItrigInmux21 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux21 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux21::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0ClrDmac1ItrigInmux21> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0ClrDmac1ItrigInmux21) -> u8 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0ClrDmac1ItrigInmux22 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ItrigEna0ClrDmac1ItrigInmux22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0ClrDmac1ItrigInmux22 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux22 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux22::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0ClrDmac1ItrigInmux22> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0ClrDmac1ItrigInmux22) -> u8 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0ClrDmac1ItrigInmux23 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ItrigEna0ClrDmac1ItrigInmux23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0ClrDmac1ItrigInmux23 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux23 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux23::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0ClrDmac1ItrigInmux23> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0ClrDmac1ItrigInmux23) -> u8 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0ClrDmac1ItrigInmux24 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ItrigEna0ClrDmac1ItrigInmux24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0ClrDmac1ItrigInmux24 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux24 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux24::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0ClrDmac1ItrigInmux24> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0ClrDmac1ItrigInmux24) -> u8 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0ClrDmac1ItrigInmux25 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ItrigEna0ClrDmac1ItrigInmux25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0ClrDmac1ItrigInmux25 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux25 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux25::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0ClrDmac1ItrigInmux25> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0ClrDmac1ItrigInmux25) -> u8 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0ClrDmac1ItrigInmux26 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ItrigEna0ClrDmac1ItrigInmux26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0ClrDmac1ItrigInmux26 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux26 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux26::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0ClrDmac1ItrigInmux26> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0ClrDmac1ItrigInmux26) -> u8 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0ClrDmac1ItrigInmux27 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ItrigEna0ClrDmac1ItrigInmux27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0ClrDmac1ItrigInmux27 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux27 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux27::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0ClrDmac1ItrigInmux27> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0ClrDmac1ItrigInmux27) -> u8 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0ClrDmac1ItrigInmux28 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ItrigEna0ClrDmac1ItrigInmux28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0ClrDmac1ItrigInmux28 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux28 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux28::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0ClrDmac1ItrigInmux28> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0ClrDmac1ItrigInmux28) -> u8 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0ClrDmac1ItrigInmux29 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ItrigEna0ClrDmac1ItrigInmux29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0ClrDmac1ItrigInmux29 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux29 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux29::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0ClrDmac1ItrigInmux29> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0ClrDmac1ItrigInmux29) -> u8 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0ClrDmac1ItrigInmux3 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ItrigEna0ClrDmac1ItrigInmux3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0ClrDmac1ItrigInmux3 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux3 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux3::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0ClrDmac1ItrigInmux3> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0ClrDmac1ItrigInmux3) -> u8 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0ClrDmac1ItrigInmux30 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ItrigEna0ClrDmac1ItrigInmux30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0ClrDmac1ItrigInmux30 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux30 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux30::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0ClrDmac1ItrigInmux30> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0ClrDmac1ItrigInmux30) -> u8 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0ClrDmac1ItrigInmux31 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ItrigEna0ClrDmac1ItrigInmux31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0ClrDmac1ItrigInmux31 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux31 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux31::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0ClrDmac1ItrigInmux31> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0ClrDmac1ItrigInmux31) -> u8 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0ClrDmac1ItrigInmux4 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ItrigEna0ClrDmac1ItrigInmux4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0ClrDmac1ItrigInmux4 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux4 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux4::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0ClrDmac1ItrigInmux4> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0ClrDmac1ItrigInmux4) -> u8 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0ClrDmac1ItrigInmux5 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ItrigEna0ClrDmac1ItrigInmux5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0ClrDmac1ItrigInmux5 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux5 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux5::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0ClrDmac1ItrigInmux5> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0ClrDmac1ItrigInmux5) -> u8 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0ClrDmac1ItrigInmux6 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ItrigEna0ClrDmac1ItrigInmux6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0ClrDmac1ItrigInmux6 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux6 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux6::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0ClrDmac1ItrigInmux6> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0ClrDmac1ItrigInmux6) -> u8 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0ClrDmac1ItrigInmux7 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ItrigEna0ClrDmac1ItrigInmux7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0ClrDmac1ItrigInmux7 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux7 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux7::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0ClrDmac1ItrigInmux7> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0ClrDmac1ItrigInmux7) -> u8 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0ClrDmac1ItrigInmux8 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ItrigEna0ClrDmac1ItrigInmux8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0ClrDmac1ItrigInmux8 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux8 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux8::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0ClrDmac1ItrigInmux8> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0ClrDmac1ItrigInmux8) -> u8 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0ClrDmac1ItrigInmux9 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ItrigEna0ClrDmac1ItrigInmux9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0ClrDmac1ItrigInmux9 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0ClrDmac1ItrigInmux9 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux9::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0ClrDmac1ItrigInmux9> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0ClrDmac1ItrigInmux9) -> u8 {
        Dmac1ItrigEna0ClrDmac1ItrigInmux9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0SetDmac1ItrigInmux0 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ItrigEna0SetDmac1ItrigInmux0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0SetDmac1ItrigInmux0 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux0 {
        Dmac1ItrigEna0SetDmac1ItrigInmux0::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0SetDmac1ItrigInmux0> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0SetDmac1ItrigInmux0) -> u8 {
        Dmac1ItrigEna0SetDmac1ItrigInmux0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0SetDmac1ItrigInmux1 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ItrigEna0SetDmac1ItrigInmux1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0SetDmac1ItrigInmux1 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux1 {
        Dmac1ItrigEna0SetDmac1ItrigInmux1::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0SetDmac1ItrigInmux1> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0SetDmac1ItrigInmux1) -> u8 {
        Dmac1ItrigEna0SetDmac1ItrigInmux1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0SetDmac1ItrigInmux10 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ItrigEna0SetDmac1ItrigInmux10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0SetDmac1ItrigInmux10 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux10 {
        Dmac1ItrigEna0SetDmac1ItrigInmux10::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0SetDmac1ItrigInmux10> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0SetDmac1ItrigInmux10) -> u8 {
        Dmac1ItrigEna0SetDmac1ItrigInmux10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0SetDmac1ItrigInmux11 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ItrigEna0SetDmac1ItrigInmux11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0SetDmac1ItrigInmux11 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux11 {
        Dmac1ItrigEna0SetDmac1ItrigInmux11::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0SetDmac1ItrigInmux11> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0SetDmac1ItrigInmux11) -> u8 {
        Dmac1ItrigEna0SetDmac1ItrigInmux11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0SetDmac1ItrigInmux12 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ItrigEna0SetDmac1ItrigInmux12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0SetDmac1ItrigInmux12 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux12 {
        Dmac1ItrigEna0SetDmac1ItrigInmux12::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0SetDmac1ItrigInmux12> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0SetDmac1ItrigInmux12) -> u8 {
        Dmac1ItrigEna0SetDmac1ItrigInmux12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0SetDmac1ItrigInmux13 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ItrigEna0SetDmac1ItrigInmux13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0SetDmac1ItrigInmux13 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux13 {
        Dmac1ItrigEna0SetDmac1ItrigInmux13::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0SetDmac1ItrigInmux13> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0SetDmac1ItrigInmux13) -> u8 {
        Dmac1ItrigEna0SetDmac1ItrigInmux13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0SetDmac1ItrigInmux14 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ItrigEna0SetDmac1ItrigInmux14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0SetDmac1ItrigInmux14 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux14 {
        Dmac1ItrigEna0SetDmac1ItrigInmux14::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0SetDmac1ItrigInmux14> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0SetDmac1ItrigInmux14) -> u8 {
        Dmac1ItrigEna0SetDmac1ItrigInmux14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0SetDmac1ItrigInmux15 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ItrigEna0SetDmac1ItrigInmux15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0SetDmac1ItrigInmux15 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux15 {
        Dmac1ItrigEna0SetDmac1ItrigInmux15::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0SetDmac1ItrigInmux15> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0SetDmac1ItrigInmux15) -> u8 {
        Dmac1ItrigEna0SetDmac1ItrigInmux15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0SetDmac1ItrigInmux16 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ItrigEna0SetDmac1ItrigInmux16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0SetDmac1ItrigInmux16 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux16 {
        Dmac1ItrigEna0SetDmac1ItrigInmux16::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0SetDmac1ItrigInmux16> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0SetDmac1ItrigInmux16) -> u8 {
        Dmac1ItrigEna0SetDmac1ItrigInmux16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0SetDmac1ItrigInmux17 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ItrigEna0SetDmac1ItrigInmux17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0SetDmac1ItrigInmux17 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux17 {
        Dmac1ItrigEna0SetDmac1ItrigInmux17::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0SetDmac1ItrigInmux17> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0SetDmac1ItrigInmux17) -> u8 {
        Dmac1ItrigEna0SetDmac1ItrigInmux17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0SetDmac1ItrigInmux18 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ItrigEna0SetDmac1ItrigInmux18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0SetDmac1ItrigInmux18 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux18 {
        Dmac1ItrigEna0SetDmac1ItrigInmux18::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0SetDmac1ItrigInmux18> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0SetDmac1ItrigInmux18) -> u8 {
        Dmac1ItrigEna0SetDmac1ItrigInmux18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0SetDmac1ItrigInmux19 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ItrigEna0SetDmac1ItrigInmux19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0SetDmac1ItrigInmux19 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux19 {
        Dmac1ItrigEna0SetDmac1ItrigInmux19::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0SetDmac1ItrigInmux19> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0SetDmac1ItrigInmux19) -> u8 {
        Dmac1ItrigEna0SetDmac1ItrigInmux19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0SetDmac1ItrigInmux20 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ItrigEna0SetDmac1ItrigInmux20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0SetDmac1ItrigInmux20 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux20 {
        Dmac1ItrigEna0SetDmac1ItrigInmux20::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0SetDmac1ItrigInmux20> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0SetDmac1ItrigInmux20) -> u8 {
        Dmac1ItrigEna0SetDmac1ItrigInmux20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0SetDmac1ItrigInmux21 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ItrigEna0SetDmac1ItrigInmux21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0SetDmac1ItrigInmux21 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux21 {
        Dmac1ItrigEna0SetDmac1ItrigInmux21::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0SetDmac1ItrigInmux21> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0SetDmac1ItrigInmux21) -> u8 {
        Dmac1ItrigEna0SetDmac1ItrigInmux21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0SetDmac1ItrigInmux22 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ItrigEna0SetDmac1ItrigInmux22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0SetDmac1ItrigInmux22 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux22 {
        Dmac1ItrigEna0SetDmac1ItrigInmux22::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0SetDmac1ItrigInmux22> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0SetDmac1ItrigInmux22) -> u8 {
        Dmac1ItrigEna0SetDmac1ItrigInmux22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0SetDmac1ItrigInmux23 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ItrigEna0SetDmac1ItrigInmux23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0SetDmac1ItrigInmux23 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux23 {
        Dmac1ItrigEna0SetDmac1ItrigInmux23::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0SetDmac1ItrigInmux23> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0SetDmac1ItrigInmux23) -> u8 {
        Dmac1ItrigEna0SetDmac1ItrigInmux23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0SetDmac1ItrigInmux24 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ItrigEna0SetDmac1ItrigInmux24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0SetDmac1ItrigInmux24 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux24 {
        Dmac1ItrigEna0SetDmac1ItrigInmux24::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0SetDmac1ItrigInmux24> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0SetDmac1ItrigInmux24) -> u8 {
        Dmac1ItrigEna0SetDmac1ItrigInmux24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0SetDmac1ItrigInmux25 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ItrigEna0SetDmac1ItrigInmux25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0SetDmac1ItrigInmux25 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux25 {
        Dmac1ItrigEna0SetDmac1ItrigInmux25::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0SetDmac1ItrigInmux25> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0SetDmac1ItrigInmux25) -> u8 {
        Dmac1ItrigEna0SetDmac1ItrigInmux25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0SetDmac1ItrigInmux26 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ItrigEna0SetDmac1ItrigInmux26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0SetDmac1ItrigInmux26 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux26 {
        Dmac1ItrigEna0SetDmac1ItrigInmux26::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0SetDmac1ItrigInmux26> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0SetDmac1ItrigInmux26) -> u8 {
        Dmac1ItrigEna0SetDmac1ItrigInmux26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0SetDmac1ItrigInmux27 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ItrigEna0SetDmac1ItrigInmux27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0SetDmac1ItrigInmux27 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux27 {
        Dmac1ItrigEna0SetDmac1ItrigInmux27::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0SetDmac1ItrigInmux27> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0SetDmac1ItrigInmux27) -> u8 {
        Dmac1ItrigEna0SetDmac1ItrigInmux27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0SetDmac1ItrigInmux28 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ItrigEna0SetDmac1ItrigInmux28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0SetDmac1ItrigInmux28 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux28 {
        Dmac1ItrigEna0SetDmac1ItrigInmux28::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0SetDmac1ItrigInmux28> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0SetDmac1ItrigInmux28) -> u8 {
        Dmac1ItrigEna0SetDmac1ItrigInmux28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0SetDmac1ItrigInmux29 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ItrigEna0SetDmac1ItrigInmux29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0SetDmac1ItrigInmux29 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux29 {
        Dmac1ItrigEna0SetDmac1ItrigInmux29::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0SetDmac1ItrigInmux29> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0SetDmac1ItrigInmux29) -> u8 {
        Dmac1ItrigEna0SetDmac1ItrigInmux29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0SetDmac1ItrigInmux3 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ItrigEna0SetDmac1ItrigInmux3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0SetDmac1ItrigInmux3 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux3 {
        Dmac1ItrigEna0SetDmac1ItrigInmux3::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0SetDmac1ItrigInmux3> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0SetDmac1ItrigInmux3) -> u8 {
        Dmac1ItrigEna0SetDmac1ItrigInmux3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0SetDmac1ItrigInmux30 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ItrigEna0SetDmac1ItrigInmux30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0SetDmac1ItrigInmux30 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux30 {
        Dmac1ItrigEna0SetDmac1ItrigInmux30::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0SetDmac1ItrigInmux30> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0SetDmac1ItrigInmux30) -> u8 {
        Dmac1ItrigEna0SetDmac1ItrigInmux30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0SetDmac1ItrigInmux31 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ItrigEna0SetDmac1ItrigInmux31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0SetDmac1ItrigInmux31 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux31 {
        Dmac1ItrigEna0SetDmac1ItrigInmux31::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0SetDmac1ItrigInmux31> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0SetDmac1ItrigInmux31) -> u8 {
        Dmac1ItrigEna0SetDmac1ItrigInmux31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0SetDmac1ItrigInmux4 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ItrigEna0SetDmac1ItrigInmux4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0SetDmac1ItrigInmux4 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux4 {
        Dmac1ItrigEna0SetDmac1ItrigInmux4::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0SetDmac1ItrigInmux4> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0SetDmac1ItrigInmux4) -> u8 {
        Dmac1ItrigEna0SetDmac1ItrigInmux4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0SetDmac1ItrigInmux5 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ItrigEna0SetDmac1ItrigInmux5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0SetDmac1ItrigInmux5 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux5 {
        Dmac1ItrigEna0SetDmac1ItrigInmux5::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0SetDmac1ItrigInmux5> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0SetDmac1ItrigInmux5) -> u8 {
        Dmac1ItrigEna0SetDmac1ItrigInmux5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0SetDmac1ItrigInmux6 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ItrigEna0SetDmac1ItrigInmux6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0SetDmac1ItrigInmux6 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux6 {
        Dmac1ItrigEna0SetDmac1ItrigInmux6::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0SetDmac1ItrigInmux6> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0SetDmac1ItrigInmux6) -> u8 {
        Dmac1ItrigEna0SetDmac1ItrigInmux6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0SetDmac1ItrigInmux7 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ItrigEna0SetDmac1ItrigInmux7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0SetDmac1ItrigInmux7 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux7 {
        Dmac1ItrigEna0SetDmac1ItrigInmux7::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0SetDmac1ItrigInmux7> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0SetDmac1ItrigInmux7) -> u8 {
        Dmac1ItrigEna0SetDmac1ItrigInmux7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0SetDmac1ItrigInmux8 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ItrigEna0SetDmac1ItrigInmux8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0SetDmac1ItrigInmux8 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux8 {
        Dmac1ItrigEna0SetDmac1ItrigInmux8::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0SetDmac1ItrigInmux8> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0SetDmac1ItrigInmux8) -> u8 {
        Dmac1ItrigEna0SetDmac1ItrigInmux8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ItrigEna0SetDmac1ItrigInmux9 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ItrigEna0SetDmac1ItrigInmux9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ItrigEna0SetDmac1ItrigInmux9 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ItrigEna0SetDmac1ItrigInmux9 {
        Dmac1ItrigEna0SetDmac1ItrigInmux9::from_bits(val)
    }
}
impl From<Dmac1ItrigEna0SetDmac1ItrigInmux9> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ItrigEna0SetDmac1ItrigInmux9) -> u8 {
        Dmac1ItrigEna0SetDmac1ItrigInmux9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1OtrigSel {
    #[doc = "DMAC1_OTRIG_CH0."]
    Dmac1OtrigCh0 = 0x0,
    #[doc = "DMAC1_OTRIG_CH1."]
    Dmac1OtrigCh1 = 0x01,
    #[doc = "DMAC1_OTRIG_CH2."]
    Dmac1OtrigCh2 = 0x02,
    #[doc = "DMAC1_OTRIG_CH3."]
    Dmac1OtrigCh3 = 0x03,
    #[doc = "DMAC1_OTRIG_CH4."]
    Dmac1OtrigCh4 = 0x04,
    #[doc = "DMAC1_OTRIG_CH5."]
    Dmac1OtrigCh5 = 0x05,
    #[doc = "DMAC1_OTRIG_CH6."]
    Dmac1OtrigCh6 = 0x06,
    #[doc = "DMAC1_OTRIG_CH7."]
    Dmac1OtrigCh7 = 0x07,
    #[doc = "DMAC1_OTRIG_CH8."]
    Dmac1OtrigCh8 = 0x08,
    #[doc = "DMAC1_OTRIG_CH9."]
    Dmac1OtrigCh9 = 0x09,
    #[doc = "DMAC1_OTRIG_CH10."]
    Dmac1OtrigCh10 = 0x0a,
    #[doc = "DMAC1_OTRIG_CH11."]
    Dmac1OtrigCh11 = 0x0b,
    #[doc = "DMAC1_OTRIG_CH12."]
    Dmac1OtrigCh12 = 0x0c,
    #[doc = "DMAC1_OTRIG_CH13."]
    Dmac1OtrigCh13 = 0x0d,
    #[doc = "DMAC1_OTRIG_CH14."]
    Dmac1OtrigCh14 = 0x0e,
    #[doc = "DMAC1_OTRIG_CH15."]
    Dmac1OtrigCh15 = 0x0f,
    #[doc = "DMAC1_OTRIG_CH16."]
    Dmac1OtrigCh16 = 0x10,
    #[doc = "DMAC1_OTRIG_CH17."]
    Dmac1OtrigCh17 = 0x11,
    #[doc = "DMAC1_OTRIG_CH18."]
    Dmac1OtrigCh18 = 0x12,
    #[doc = "DMAC1_OTRIG_CH19."]
    Dmac1OtrigCh19 = 0x13,
    #[doc = "DMAC1_OTRIG_CH20."]
    Dmac1OtrigCh20 = 0x14,
    #[doc = "DMAC1_OTRIG_CH21."]
    Dmac1OtrigCh21 = 0x15,
    #[doc = "DMAC1_OTRIG_CH22."]
    Dmac1OtrigCh22 = 0x16,
    #[doc = "DMAC1_OTRIG_CH23."]
    Dmac1OtrigCh23 = 0x17,
    #[doc = "DMAC1_OTRIG_CH24."]
    Dmac1OtrigCh24 = 0x18,
    #[doc = "DMAC1_OTRIG_CH25."]
    Dmac1OtrigCh25 = 0x19,
    #[doc = "DMAC1_OTRIG_CH26."]
    Dmac1OtrigCh26 = 0x1a,
    #[doc = "DMAC1_OTRIG_CH27."]
    Dmac1OtrigCh27 = 0x1b,
    #[doc = "DMAC1_OTRIG_CH28."]
    Dmac1OtrigCh28 = 0x1c,
    #[doc = "DMAC1_OTRIG_CH29."]
    Dmac1OtrigCh29 = 0x1d,
    #[doc = "DMAC1_OTRIG_CH30."]
    Dmac1OtrigCh30 = 0x1e,
    #[doc = "DMAC1_OTRIG_CH31."]
    Dmac1OtrigCh31 = 0x1f,
    #[doc = "DMAC1_OTRIG_CH32."]
    Dmac1OtrigCh32 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl Dmac1OtrigSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1OtrigSel {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1OtrigSel {
    #[inline(always)]
    fn from(val: u8) -> Dmac1OtrigSel {
        Dmac1OtrigSel::from_bits(val)
    }
}
impl From<Dmac1OtrigSel> for u8 {
    #[inline(always)]
    fn from(val: Dmac1OtrigSel) -> u8 {
        Dmac1OtrigSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0ClrDmic0ch0 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ReqEna0ClrDmic0ch0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0ClrDmic0ch0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0ClrDmic0ch0 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0ClrDmic0ch0 {
        Dmac1ReqEna0ClrDmic0ch0::from_bits(val)
    }
}
impl From<Dmac1ReqEna0ClrDmic0ch0> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0ClrDmic0ch0) -> u8 {
        Dmac1ReqEna0ClrDmic0ch0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0ClrDmic0ch1 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ReqEna0ClrDmic0ch1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0ClrDmic0ch1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0ClrDmic0ch1 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0ClrDmic0ch1 {
        Dmac1ReqEna0ClrDmic0ch1::from_bits(val)
    }
}
impl From<Dmac1ReqEna0ClrDmic0ch1> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0ClrDmic0ch1) -> u8 {
        Dmac1ReqEna0ClrDmic0ch1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0ClrDmic0ch2 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ReqEna0ClrDmic0ch2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0ClrDmic0ch2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0ClrDmic0ch2 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0ClrDmic0ch2 {
        Dmac1ReqEna0ClrDmic0ch2::from_bits(val)
    }
}
impl From<Dmac1ReqEna0ClrDmic0ch2> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0ClrDmic0ch2) -> u8 {
        Dmac1ReqEna0ClrDmic0ch2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0ClrDmic0ch3 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ReqEna0ClrDmic0ch3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0ClrDmic0ch3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0ClrDmic0ch3 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0ClrDmic0ch3 {
        Dmac1ReqEna0ClrDmic0ch3::from_bits(val)
    }
}
impl From<Dmac1ReqEna0ClrDmic0ch3> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0ClrDmic0ch3) -> u8 {
        Dmac1ReqEna0ClrDmic0ch3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0ClrDmic0ch4 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ReqEna0ClrDmic0ch4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0ClrDmic0ch4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0ClrDmic0ch4 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0ClrDmic0ch4 {
        Dmac1ReqEna0ClrDmic0ch4::from_bits(val)
    }
}
impl From<Dmac1ReqEna0ClrDmic0ch4> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0ClrDmic0ch4) -> u8 {
        Dmac1ReqEna0ClrDmic0ch4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0ClrDmic0ch5 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ReqEna0ClrDmic0ch5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0ClrDmic0ch5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0ClrDmic0ch5 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0ClrDmic0ch5 {
        Dmac1ReqEna0ClrDmic0ch5::from_bits(val)
    }
}
impl From<Dmac1ReqEna0ClrDmic0ch5> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0ClrDmic0ch5) -> u8 {
        Dmac1ReqEna0ClrDmic0ch5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0ClrDmic0ch6 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ReqEna0ClrDmic0ch6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0ClrDmic0ch6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0ClrDmic0ch6 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0ClrDmic0ch6 {
        Dmac1ReqEna0ClrDmic0ch6::from_bits(val)
    }
}
impl From<Dmac1ReqEna0ClrDmic0ch6> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0ClrDmic0ch6) -> u8 {
        Dmac1ReqEna0ClrDmic0ch6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0ClrDmic0ch7 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ReqEna0ClrDmic0ch7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0ClrDmic0ch7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0ClrDmic0ch7 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0ClrDmic0ch7 {
        Dmac1ReqEna0ClrDmic0ch7::from_bits(val)
    }
}
impl From<Dmac1ReqEna0ClrDmic0ch7> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0ClrDmic0ch7) -> u8 {
        Dmac1ReqEna0ClrDmic0ch7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0ClrEspiCh0 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ReqEna0ClrEspiCh0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0ClrEspiCh0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0ClrEspiCh0 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0ClrEspiCh0 {
        Dmac1ReqEna0ClrEspiCh0::from_bits(val)
    }
}
impl From<Dmac1ReqEna0ClrEspiCh0> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0ClrEspiCh0) -> u8 {
        Dmac1ReqEna0ClrEspiCh0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0ClrFlexcomm0Rx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ReqEna0ClrFlexcomm0Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0ClrFlexcomm0Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0ClrFlexcomm0Rx {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0ClrFlexcomm0Rx {
        Dmac1ReqEna0ClrFlexcomm0Rx::from_bits(val)
    }
}
impl From<Dmac1ReqEna0ClrFlexcomm0Rx> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0ClrFlexcomm0Rx) -> u8 {
        Dmac1ReqEna0ClrFlexcomm0Rx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0ClrFlexcomm0Tx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ReqEna0ClrFlexcomm0Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0ClrFlexcomm0Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0ClrFlexcomm0Tx {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0ClrFlexcomm0Tx {
        Dmac1ReqEna0ClrFlexcomm0Tx::from_bits(val)
    }
}
impl From<Dmac1ReqEna0ClrFlexcomm0Tx> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0ClrFlexcomm0Tx) -> u8 {
        Dmac1ReqEna0ClrFlexcomm0Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0ClrFlexcomm14Rx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ReqEna0ClrFlexcomm14Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0ClrFlexcomm14Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0ClrFlexcomm14Rx {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0ClrFlexcomm14Rx {
        Dmac1ReqEna0ClrFlexcomm14Rx::from_bits(val)
    }
}
impl From<Dmac1ReqEna0ClrFlexcomm14Rx> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0ClrFlexcomm14Rx) -> u8 {
        Dmac1ReqEna0ClrFlexcomm14Rx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0ClrFlexcomm14Tx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ReqEna0ClrFlexcomm14Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0ClrFlexcomm14Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0ClrFlexcomm14Tx {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0ClrFlexcomm14Tx {
        Dmac1ReqEna0ClrFlexcomm14Tx::from_bits(val)
    }
}
impl From<Dmac1ReqEna0ClrFlexcomm14Tx> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0ClrFlexcomm14Tx) -> u8 {
        Dmac1ReqEna0ClrFlexcomm14Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0ClrFlexcomm1Rx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ReqEna0ClrFlexcomm1Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0ClrFlexcomm1Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0ClrFlexcomm1Rx {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0ClrFlexcomm1Rx {
        Dmac1ReqEna0ClrFlexcomm1Rx::from_bits(val)
    }
}
impl From<Dmac1ReqEna0ClrFlexcomm1Rx> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0ClrFlexcomm1Rx) -> u8 {
        Dmac1ReqEna0ClrFlexcomm1Rx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0ClrFlexcomm1Tx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ReqEna0ClrFlexcomm1Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0ClrFlexcomm1Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0ClrFlexcomm1Tx {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0ClrFlexcomm1Tx {
        Dmac1ReqEna0ClrFlexcomm1Tx::from_bits(val)
    }
}
impl From<Dmac1ReqEna0ClrFlexcomm1Tx> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0ClrFlexcomm1Tx) -> u8 {
        Dmac1ReqEna0ClrFlexcomm1Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0ClrFlexcomm2Rx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ReqEna0ClrFlexcomm2Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0ClrFlexcomm2Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0ClrFlexcomm2Rx {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0ClrFlexcomm2Rx {
        Dmac1ReqEna0ClrFlexcomm2Rx::from_bits(val)
    }
}
impl From<Dmac1ReqEna0ClrFlexcomm2Rx> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0ClrFlexcomm2Rx) -> u8 {
        Dmac1ReqEna0ClrFlexcomm2Rx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0ClrFlexcomm2Tx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ReqEna0ClrFlexcomm2Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0ClrFlexcomm2Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0ClrFlexcomm2Tx {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0ClrFlexcomm2Tx {
        Dmac1ReqEna0ClrFlexcomm2Tx::from_bits(val)
    }
}
impl From<Dmac1ReqEna0ClrFlexcomm2Tx> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0ClrFlexcomm2Tx) -> u8 {
        Dmac1ReqEna0ClrFlexcomm2Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0ClrFlexcomm3Rx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ReqEna0ClrFlexcomm3Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0ClrFlexcomm3Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0ClrFlexcomm3Rx {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0ClrFlexcomm3Rx {
        Dmac1ReqEna0ClrFlexcomm3Rx::from_bits(val)
    }
}
impl From<Dmac1ReqEna0ClrFlexcomm3Rx> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0ClrFlexcomm3Rx) -> u8 {
        Dmac1ReqEna0ClrFlexcomm3Rx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0ClrFlexcomm3Tx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ReqEna0ClrFlexcomm3Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0ClrFlexcomm3Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0ClrFlexcomm3Tx {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0ClrFlexcomm3Tx {
        Dmac1ReqEna0ClrFlexcomm3Tx::from_bits(val)
    }
}
impl From<Dmac1ReqEna0ClrFlexcomm3Tx> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0ClrFlexcomm3Tx) -> u8 {
        Dmac1ReqEna0ClrFlexcomm3Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0ClrFlexcomm4Rx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ReqEna0ClrFlexcomm4Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0ClrFlexcomm4Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0ClrFlexcomm4Rx {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0ClrFlexcomm4Rx {
        Dmac1ReqEna0ClrFlexcomm4Rx::from_bits(val)
    }
}
impl From<Dmac1ReqEna0ClrFlexcomm4Rx> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0ClrFlexcomm4Rx) -> u8 {
        Dmac1ReqEna0ClrFlexcomm4Rx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0ClrFlexcomm4Tx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ReqEna0ClrFlexcomm4Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0ClrFlexcomm4Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0ClrFlexcomm4Tx {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0ClrFlexcomm4Tx {
        Dmac1ReqEna0ClrFlexcomm4Tx::from_bits(val)
    }
}
impl From<Dmac1ReqEna0ClrFlexcomm4Tx> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0ClrFlexcomm4Tx) -> u8 {
        Dmac1ReqEna0ClrFlexcomm4Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0ClrFlexcomm5Rx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ReqEna0ClrFlexcomm5Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0ClrFlexcomm5Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0ClrFlexcomm5Rx {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0ClrFlexcomm5Rx {
        Dmac1ReqEna0ClrFlexcomm5Rx::from_bits(val)
    }
}
impl From<Dmac1ReqEna0ClrFlexcomm5Rx> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0ClrFlexcomm5Rx) -> u8 {
        Dmac1ReqEna0ClrFlexcomm5Rx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0ClrFlexcomm5Tx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ReqEna0ClrFlexcomm5Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0ClrFlexcomm5Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0ClrFlexcomm5Tx {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0ClrFlexcomm5Tx {
        Dmac1ReqEna0ClrFlexcomm5Tx::from_bits(val)
    }
}
impl From<Dmac1ReqEna0ClrFlexcomm5Tx> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0ClrFlexcomm5Tx) -> u8 {
        Dmac1ReqEna0ClrFlexcomm5Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0ClrFlexcomm6Rx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ReqEna0ClrFlexcomm6Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0ClrFlexcomm6Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0ClrFlexcomm6Rx {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0ClrFlexcomm6Rx {
        Dmac1ReqEna0ClrFlexcomm6Rx::from_bits(val)
    }
}
impl From<Dmac1ReqEna0ClrFlexcomm6Rx> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0ClrFlexcomm6Rx) -> u8 {
        Dmac1ReqEna0ClrFlexcomm6Rx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0ClrFlexcomm6Tx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ReqEna0ClrFlexcomm6Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0ClrFlexcomm6Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0ClrFlexcomm6Tx {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0ClrFlexcomm6Tx {
        Dmac1ReqEna0ClrFlexcomm6Tx::from_bits(val)
    }
}
impl From<Dmac1ReqEna0ClrFlexcomm6Tx> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0ClrFlexcomm6Tx) -> u8 {
        Dmac1ReqEna0ClrFlexcomm6Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0ClrFlexcomm7Rx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ReqEna0ClrFlexcomm7Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0ClrFlexcomm7Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0ClrFlexcomm7Rx {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0ClrFlexcomm7Rx {
        Dmac1ReqEna0ClrFlexcomm7Rx::from_bits(val)
    }
}
impl From<Dmac1ReqEna0ClrFlexcomm7Rx> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0ClrFlexcomm7Rx) -> u8 {
        Dmac1ReqEna0ClrFlexcomm7Rx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0ClrFlexcomm7Tx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ReqEna0ClrFlexcomm7Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0ClrFlexcomm7Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0ClrFlexcomm7Tx {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0ClrFlexcomm7Tx {
        Dmac1ReqEna0ClrFlexcomm7Tx::from_bits(val)
    }
}
impl From<Dmac1ReqEna0ClrFlexcomm7Tx> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0ClrFlexcomm7Tx) -> u8 {
        Dmac1ReqEna0ClrFlexcomm7Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0ClrHashcrypt {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ReqEna0ClrHashcrypt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0ClrHashcrypt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0ClrHashcrypt {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0ClrHashcrypt {
        Dmac1ReqEna0ClrHashcrypt::from_bits(val)
    }
}
impl From<Dmac1ReqEna0ClrHashcrypt> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0ClrHashcrypt) -> u8 {
        Dmac1ReqEna0ClrHashcrypt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0ClrI3c0Rx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ReqEna0ClrI3c0Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0ClrI3c0Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0ClrI3c0Rx {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0ClrI3c0Rx {
        Dmac1ReqEna0ClrI3c0Rx::from_bits(val)
    }
}
impl From<Dmac1ReqEna0ClrI3c0Rx> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0ClrI3c0Rx) -> u8 {
        Dmac1ReqEna0ClrI3c0Rx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0ClrI3c0Tx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA0 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ReqEna0ClrI3c0Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0ClrI3c0Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0ClrI3c0Tx {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0ClrI3c0Tx {
        Dmac1ReqEna0ClrI3c0Tx::from_bits(val)
    }
}
impl From<Dmac1ReqEna0ClrI3c0Tx> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0ClrI3c0Tx) -> u8 {
        Dmac1ReqEna0ClrI3c0Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0SetDmic0ch0 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ReqEna0SetDmic0ch0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0SetDmic0ch0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0SetDmic0ch0 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0SetDmic0ch0 {
        Dmac1ReqEna0SetDmic0ch0::from_bits(val)
    }
}
impl From<Dmac1ReqEna0SetDmic0ch0> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0SetDmic0ch0) -> u8 {
        Dmac1ReqEna0SetDmic0ch0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0SetDmic0ch1 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ReqEna0SetDmic0ch1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0SetDmic0ch1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0SetDmic0ch1 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0SetDmic0ch1 {
        Dmac1ReqEna0SetDmic0ch1::from_bits(val)
    }
}
impl From<Dmac1ReqEna0SetDmic0ch1> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0SetDmic0ch1) -> u8 {
        Dmac1ReqEna0SetDmic0ch1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0SetDmic0ch2 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ReqEna0SetDmic0ch2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0SetDmic0ch2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0SetDmic0ch2 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0SetDmic0ch2 {
        Dmac1ReqEna0SetDmic0ch2::from_bits(val)
    }
}
impl From<Dmac1ReqEna0SetDmic0ch2> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0SetDmic0ch2) -> u8 {
        Dmac1ReqEna0SetDmic0ch2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0SetDmic0ch3 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ReqEna0SetDmic0ch3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0SetDmic0ch3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0SetDmic0ch3 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0SetDmic0ch3 {
        Dmac1ReqEna0SetDmic0ch3::from_bits(val)
    }
}
impl From<Dmac1ReqEna0SetDmic0ch3> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0SetDmic0ch3) -> u8 {
        Dmac1ReqEna0SetDmic0ch3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0SetDmic0ch4 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ReqEna0SetDmic0ch4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0SetDmic0ch4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0SetDmic0ch4 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0SetDmic0ch4 {
        Dmac1ReqEna0SetDmic0ch4::from_bits(val)
    }
}
impl From<Dmac1ReqEna0SetDmic0ch4> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0SetDmic0ch4) -> u8 {
        Dmac1ReqEna0SetDmic0ch4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0SetDmic0ch5 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ReqEna0SetDmic0ch5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0SetDmic0ch5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0SetDmic0ch5 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0SetDmic0ch5 {
        Dmac1ReqEna0SetDmic0ch5::from_bits(val)
    }
}
impl From<Dmac1ReqEna0SetDmic0ch5> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0SetDmic0ch5) -> u8 {
        Dmac1ReqEna0SetDmic0ch5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0SetDmic0ch6 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ReqEna0SetDmic0ch6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0SetDmic0ch6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0SetDmic0ch6 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0SetDmic0ch6 {
        Dmac1ReqEna0SetDmic0ch6::from_bits(val)
    }
}
impl From<Dmac1ReqEna0SetDmic0ch6> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0SetDmic0ch6) -> u8 {
        Dmac1ReqEna0SetDmic0ch6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0SetDmic0ch7 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ReqEna0SetDmic0ch7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0SetDmic0ch7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0SetDmic0ch7 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0SetDmic0ch7 {
        Dmac1ReqEna0SetDmic0ch7::from_bits(val)
    }
}
impl From<Dmac1ReqEna0SetDmic0ch7> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0SetDmic0ch7) -> u8 {
        Dmac1ReqEna0SetDmic0ch7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0SetEspiCh0 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ReqEna0SetEspiCh0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0SetEspiCh0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0SetEspiCh0 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0SetEspiCh0 {
        Dmac1ReqEna0SetEspiCh0::from_bits(val)
    }
}
impl From<Dmac1ReqEna0SetEspiCh0> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0SetEspiCh0) -> u8 {
        Dmac1ReqEna0SetEspiCh0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0SetFlexcomm0Rx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ReqEna0SetFlexcomm0Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0SetFlexcomm0Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0SetFlexcomm0Rx {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0SetFlexcomm0Rx {
        Dmac1ReqEna0SetFlexcomm0Rx::from_bits(val)
    }
}
impl From<Dmac1ReqEna0SetFlexcomm0Rx> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0SetFlexcomm0Rx) -> u8 {
        Dmac1ReqEna0SetFlexcomm0Rx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0SetFlexcomm0Tx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ReqEna0SetFlexcomm0Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0SetFlexcomm0Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0SetFlexcomm0Tx {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0SetFlexcomm0Tx {
        Dmac1ReqEna0SetFlexcomm0Tx::from_bits(val)
    }
}
impl From<Dmac1ReqEna0SetFlexcomm0Tx> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0SetFlexcomm0Tx) -> u8 {
        Dmac1ReqEna0SetFlexcomm0Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0SetFlexcomm14Rx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ReqEna0SetFlexcomm14Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0SetFlexcomm14Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0SetFlexcomm14Rx {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0SetFlexcomm14Rx {
        Dmac1ReqEna0SetFlexcomm14Rx::from_bits(val)
    }
}
impl From<Dmac1ReqEna0SetFlexcomm14Rx> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0SetFlexcomm14Rx) -> u8 {
        Dmac1ReqEna0SetFlexcomm14Rx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0SetFlexcomm14Tx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ReqEna0SetFlexcomm14Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0SetFlexcomm14Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0SetFlexcomm14Tx {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0SetFlexcomm14Tx {
        Dmac1ReqEna0SetFlexcomm14Tx::from_bits(val)
    }
}
impl From<Dmac1ReqEna0SetFlexcomm14Tx> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0SetFlexcomm14Tx) -> u8 {
        Dmac1ReqEna0SetFlexcomm14Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0SetFlexcomm1Rx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ReqEna0SetFlexcomm1Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0SetFlexcomm1Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0SetFlexcomm1Rx {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0SetFlexcomm1Rx {
        Dmac1ReqEna0SetFlexcomm1Rx::from_bits(val)
    }
}
impl From<Dmac1ReqEna0SetFlexcomm1Rx> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0SetFlexcomm1Rx) -> u8 {
        Dmac1ReqEna0SetFlexcomm1Rx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0SetFlexcomm1Tx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ReqEna0SetFlexcomm1Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0SetFlexcomm1Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0SetFlexcomm1Tx {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0SetFlexcomm1Tx {
        Dmac1ReqEna0SetFlexcomm1Tx::from_bits(val)
    }
}
impl From<Dmac1ReqEna0SetFlexcomm1Tx> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0SetFlexcomm1Tx) -> u8 {
        Dmac1ReqEna0SetFlexcomm1Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0SetFlexcomm2Rx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ReqEna0SetFlexcomm2Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0SetFlexcomm2Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0SetFlexcomm2Rx {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0SetFlexcomm2Rx {
        Dmac1ReqEna0SetFlexcomm2Rx::from_bits(val)
    }
}
impl From<Dmac1ReqEna0SetFlexcomm2Rx> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0SetFlexcomm2Rx) -> u8 {
        Dmac1ReqEna0SetFlexcomm2Rx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0SetFlexcomm2Tx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ReqEna0SetFlexcomm2Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0SetFlexcomm2Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0SetFlexcomm2Tx {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0SetFlexcomm2Tx {
        Dmac1ReqEna0SetFlexcomm2Tx::from_bits(val)
    }
}
impl From<Dmac1ReqEna0SetFlexcomm2Tx> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0SetFlexcomm2Tx) -> u8 {
        Dmac1ReqEna0SetFlexcomm2Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0SetFlexcomm3Rx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ReqEna0SetFlexcomm3Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0SetFlexcomm3Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0SetFlexcomm3Rx {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0SetFlexcomm3Rx {
        Dmac1ReqEna0SetFlexcomm3Rx::from_bits(val)
    }
}
impl From<Dmac1ReqEna0SetFlexcomm3Rx> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0SetFlexcomm3Rx) -> u8 {
        Dmac1ReqEna0SetFlexcomm3Rx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0SetFlexcomm3Tx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ReqEna0SetFlexcomm3Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0SetFlexcomm3Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0SetFlexcomm3Tx {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0SetFlexcomm3Tx {
        Dmac1ReqEna0SetFlexcomm3Tx::from_bits(val)
    }
}
impl From<Dmac1ReqEna0SetFlexcomm3Tx> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0SetFlexcomm3Tx) -> u8 {
        Dmac1ReqEna0SetFlexcomm3Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0SetFlexcomm4Rx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ReqEna0SetFlexcomm4Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0SetFlexcomm4Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0SetFlexcomm4Rx {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0SetFlexcomm4Rx {
        Dmac1ReqEna0SetFlexcomm4Rx::from_bits(val)
    }
}
impl From<Dmac1ReqEna0SetFlexcomm4Rx> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0SetFlexcomm4Rx) -> u8 {
        Dmac1ReqEna0SetFlexcomm4Rx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0SetFlexcomm4Tx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ReqEna0SetFlexcomm4Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0SetFlexcomm4Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0SetFlexcomm4Tx {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0SetFlexcomm4Tx {
        Dmac1ReqEna0SetFlexcomm4Tx::from_bits(val)
    }
}
impl From<Dmac1ReqEna0SetFlexcomm4Tx> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0SetFlexcomm4Tx) -> u8 {
        Dmac1ReqEna0SetFlexcomm4Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0SetFlexcomm5Rx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ReqEna0SetFlexcomm5Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0SetFlexcomm5Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0SetFlexcomm5Rx {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0SetFlexcomm5Rx {
        Dmac1ReqEna0SetFlexcomm5Rx::from_bits(val)
    }
}
impl From<Dmac1ReqEna0SetFlexcomm5Rx> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0SetFlexcomm5Rx) -> u8 {
        Dmac1ReqEna0SetFlexcomm5Rx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0SetFlexcomm5Tx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ReqEna0SetFlexcomm5Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0SetFlexcomm5Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0SetFlexcomm5Tx {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0SetFlexcomm5Tx {
        Dmac1ReqEna0SetFlexcomm5Tx::from_bits(val)
    }
}
impl From<Dmac1ReqEna0SetFlexcomm5Tx> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0SetFlexcomm5Tx) -> u8 {
        Dmac1ReqEna0SetFlexcomm5Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0SetFlexcomm6Rx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ReqEna0SetFlexcomm6Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0SetFlexcomm6Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0SetFlexcomm6Rx {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0SetFlexcomm6Rx {
        Dmac1ReqEna0SetFlexcomm6Rx::from_bits(val)
    }
}
impl From<Dmac1ReqEna0SetFlexcomm6Rx> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0SetFlexcomm6Rx) -> u8 {
        Dmac1ReqEna0SetFlexcomm6Rx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0SetFlexcomm6Tx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ReqEna0SetFlexcomm6Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0SetFlexcomm6Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0SetFlexcomm6Tx {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0SetFlexcomm6Tx {
        Dmac1ReqEna0SetFlexcomm6Tx::from_bits(val)
    }
}
impl From<Dmac1ReqEna0SetFlexcomm6Tx> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0SetFlexcomm6Tx) -> u8 {
        Dmac1ReqEna0SetFlexcomm6Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0SetFlexcomm7Rx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ReqEna0SetFlexcomm7Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0SetFlexcomm7Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0SetFlexcomm7Rx {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0SetFlexcomm7Rx {
        Dmac1ReqEna0SetFlexcomm7Rx::from_bits(val)
    }
}
impl From<Dmac1ReqEna0SetFlexcomm7Rx> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0SetFlexcomm7Rx) -> u8 {
        Dmac1ReqEna0SetFlexcomm7Rx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0SetFlexcomm7Tx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ReqEna0SetFlexcomm7Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0SetFlexcomm7Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0SetFlexcomm7Tx {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0SetFlexcomm7Tx {
        Dmac1ReqEna0SetFlexcomm7Tx::from_bits(val)
    }
}
impl From<Dmac1ReqEna0SetFlexcomm7Tx> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0SetFlexcomm7Tx) -> u8 {
        Dmac1ReqEna0SetFlexcomm7Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0SetHashcrypt {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ReqEna0SetHashcrypt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0SetHashcrypt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0SetHashcrypt {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0SetHashcrypt {
        Dmac1ReqEna0SetHashcrypt::from_bits(val)
    }
}
impl From<Dmac1ReqEna0SetHashcrypt> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0SetHashcrypt) -> u8 {
        Dmac1ReqEna0SetHashcrypt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0SetI3c0Rx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ReqEna0SetI3c0Rx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0SetI3c0Rx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0SetI3c0Rx {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0SetI3c0Rx {
        Dmac1ReqEna0SetI3c0Rx::from_bits(val)
    }
}
impl From<Dmac1ReqEna0SetI3c0Rx> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0SetI3c0Rx) -> u8 {
        Dmac1ReqEna0SetI3c0Rx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna0SetI3c0Tx {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA0 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ReqEna0SetI3c0Tx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna0SetI3c0Tx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna0SetI3c0Tx {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna0SetI3c0Tx {
        Dmac1ReqEna0SetI3c0Tx::from_bits(val)
    }
}
impl From<Dmac1ReqEna0SetI3c0Tx> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna0SetI3c0Tx) -> u8 {
        Dmac1ReqEna0SetI3c0Tx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna1ClrEspiCh1 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Clears the ENA1 Bit."]
    ClrEna0Bit = 0x01,
}
impl Dmac1ReqEna1ClrEspiCh1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna1ClrEspiCh1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna1ClrEspiCh1 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna1ClrEspiCh1 {
        Dmac1ReqEna1ClrEspiCh1::from_bits(val)
    }
}
impl From<Dmac1ReqEna1ClrEspiCh1> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna1ClrEspiCh1) -> u8 {
        Dmac1ReqEna1ClrEspiCh1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmac1ReqEna1SetEspiCh1 {
    #[doc = "No Effect."]
    NoEffect = 0x0,
    #[doc = "Sets the ENA1 Bit."]
    SetEna0Bit = 0x01,
}
impl Dmac1ReqEna1SetEspiCh1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmac1ReqEna1SetEspiCh1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmac1ReqEna1SetEspiCh1 {
    #[inline(always)]
    fn from(val: u8) -> Dmac1ReqEna1SetEspiCh1 {
        Dmac1ReqEna1SetEspiCh1::from_bits(val)
    }
}
impl From<Dmac1ReqEna1SetEspiCh1> for u8 {
    #[inline(always)]
    fn from(val: Dmac1ReqEna1SetEspiCh1) -> u8 {
        Dmac1ReqEna1SetEspiCh1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DspIntSel {
    #[doc = "FLEXCOMM0."]
    Flexcomm0 = 0x0,
    #[doc = "FLEXCOMM1."]
    Flexcomm1 = 0x01,
    #[doc = "FLEXCOMM2."]
    Flexcomm2 = 0x02,
    #[doc = "FLEXCOMM3."]
    Flexcomm3 = 0x03,
    #[doc = "FLEXCOMM4."]
    Flexcomm4 = 0x04,
    #[doc = "FLEXCOMM5."]
    Flexcomm5 = 0x05,
    #[doc = "FLEXCOMM6."]
    Flexcomm6 = 0x06,
    #[doc = "FLEXCOMM7."]
    Flexcomm7 = 0x07,
    #[doc = "GPIO_INT0_IRQ0."]
    GpioInt0Irq0 = 0x08,
    #[doc = "GPIO_INT0_IRQ1."]
    GpioInt0Irq1 = 0x09,
    #[doc = "GPIO_INT0_IRQ2."]
    GpioInt0Irq2 = 0x0a,
    #[doc = "GPIO_INT0_IRQ3."]
    GpioInt0Irq3 = 0x0b,
    #[doc = "GPIO_INT0_IRQ4."]
    GpioInt0Irq4 = 0x0c,
    #[doc = "GPIO_INT0_IRQ5."]
    GpioInt0Irq5 = 0x0d,
    #[doc = "GPIO_INT0_IRQ6."]
    GpioInt0Irq6 = 0x0e,
    #[doc = "GPIO_INT0_IRQ7."]
    GpioInt0Irq7 = 0x0f,
    #[doc = "NSHSGPIO_INT0."]
    NshsgpioInt0 = 0x10,
    #[doc = "NSHSGPIO_INT1."]
    NshsgpioInt1 = 0x11,
    #[doc = "WDT1."]
    Wdt1 = 0x12,
    #[doc = "DMAC0."]
    Dmac0 = 0x13,
    #[doc = "DMAC1."]
    Dmac1 = 0x14,
    #[doc = "MU."]
    Mu = 0x15,
    #[doc = "UTICK0."]
    Utick0 = 0x16,
    #[doc = "MRT0."]
    Mrt0 = 0x17,
    #[doc = "OS_EVENT_TIMER or OS_EVENT_WAKEUP."]
    OsEventTimerOrOsEventWakeup = 0x18,
    #[doc = "CT32BIT0."]
    Ct32bit0 = 0x19,
    #[doc = "CT32BIT1."]
    Ct32bit1 = 0x1a,
    #[doc = "CT32BIT2."]
    Ct32bit2 = 0x1b,
    #[doc = "CT32BIT3."]
    Ct32bit3 = 0x1c,
    #[doc = "CT32BIT4."]
    Ct32bit4 = 0x1d,
    #[doc = "RTC_LITE0_ALARM or RTC_LITE0_WAKEUP."]
    RtcLite0AlarmOrRtcLite0Wakeup = 0x1e,
    #[doc = "I3C0."]
    I3c0 = 0x1f,
    #[doc = "DMIC0."]
    Dmic0 = 0x20,
    #[doc = "HWVAD0."]
    Hwvad0 = 0x21,
    #[doc = "FLEXSPI."]
    Flexspi = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
}
impl DspIntSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DspIntSel {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DspIntSel {
    #[inline(always)]
    fn from(val: u8) -> DspIntSel {
        DspIntSel::from_bits(val)
    }
}
impl From<DspIntSel> for u8 {
    #[inline(always)]
    fn from(val: DspIntSel) -> u8 {
        DspIntSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FmeasureSel {
    #[doc = "XTALIN."]
    Xtalin = 0x0,
    #[doc = "SFRO."]
    Sfro = 0x01,
    #[doc = "FFRO."]
    Ffro = 0x02,
    #[doc = "Low Power Oscillator Clock (LPOSC)."]
    Lposc = 0x03,
    #[doc = "RTC 32KHz OSC."]
    Rtc32kHzOsc = 0x04,
    #[doc = "MAIN_SYS_CLOCK."]
    MainSysClock = 0x05,
    #[doc = "FREQME_GPIO_CLK."]
    FreqmeGpioClk = 0x06,
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
impl FmeasureSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FmeasureSel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FmeasureSel {
    #[inline(always)]
    fn from(val: u8) -> FmeasureSel {
        FmeasureSel::from_bits(val)
    }
}
impl From<FmeasureSel> for u8 {
    #[inline(always)]
    fn from(val: FmeasureSel) -> u8 {
        FmeasureSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SctInSel {
    #[doc = "SCT0_PIN_INP0."]
    Sct0PinInp0 = 0x0,
    #[doc = "SCT0_PIN_INP1."]
    Sct0PinInp1 = 0x01,
    #[doc = "SCT0_PIN_INP2."]
    Sct0PinInp2 = 0x02,
    #[doc = "SCT0_PIN_INP3."]
    Sct0PinInp3 = 0x03,
    #[doc = "SCT0_PIN_INP4."]
    Sct0PinInp4 = 0x04,
    #[doc = "SCT0_PIN_INP5."]
    Sct0PinInp5 = 0x05,
    #[doc = "SCT0_PIN_INP6."]
    Sct0PinInp6 = 0x06,
    #[doc = "SCT0_PIN_INP7."]
    Sct0PinInp7 = 0x07,
    #[doc = "CT32BIT0_MAT0."]
    Ct32bit0Mat0 = 0x08,
    #[doc = "CT32BIT1_MAT0."]
    Ct32bit1Mat0 = 0x09,
    #[doc = "CT32BIT2_MAT0."]
    Ct32bit2Mat0 = 0x0a,
    #[doc = "CT32BIT3_MAT0."]
    Ct32bit3Mat0 = 0x0b,
    #[doc = "CT32BIT4_MAT0."]
    Ct32bit4Mat0 = 0x0c,
    #[doc = "ADCIRQ."]
    Adcirq = 0x0d,
    #[doc = "GPIOINT_BMATCH."]
    GpiointBmatch = 0x0e,
    #[doc = "USB1_FRAME_TOGGLE."]
    Usb1FrameToggle = 0x0f,
    #[doc = "CMP0_OUT."]
    Cmp0Out = 0x10,
    #[doc = "SHARED I2S0_SCLK."]
    SharedI2s0Sclk = 0x11,
    #[doc = "SHARED I2S1_SCLK."]
    SharedI2s1Sclk = 0x12,
    #[doc = "SHARED I2S0_WS."]
    SharedI2s0Ws = 0x13,
    #[doc = "SHARED I2S1_WS."]
    SharedI2s1Ws = 0x14,
    #[doc = "MCLK."]
    Mclk = 0x15,
    #[doc = "ARM_TXEV."]
    ArmTxev = 0x16,
    #[doc = "DEBUG_HALTED."]
    DebugHalted = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl SctInSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SctInSel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SctInSel {
    #[inline(always)]
    fn from(val: u8) -> SctInSel {
        SctInSel::from_bits(val)
    }
}
impl From<SctInSel> for u8 {
    #[inline(always)]
    fn from(val: SctInSel) -> u8 {
        SctInSel::to_bits(val)
    }
}
