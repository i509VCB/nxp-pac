#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma0ItrigInmuxInp {
    #[doc = "Pin interrupt 0"]
    VAL0 = 0x0,
    #[doc = "Pin interrupt 1"]
    VAL1 = 0x01,
    #[doc = "Pin interrupt 2"]
    VAL2 = 0x02,
    #[doc = "Pin interrupt 3"]
    VAL3 = 0x03,
    #[doc = "Timer CTIMER0 Match 0"]
    VAL4 = 0x04,
    #[doc = "Timer CTIMER0 Match 1"]
    VAL5 = 0x05,
    #[doc = "Timer CTIMER1 Match 0"]
    VAL6 = 0x06,
    #[doc = "Timer CTIMER1 Match 1"]
    VAL7 = 0x07,
    #[doc = "Timer CTIMER2 Match 0"]
    VAL8 = 0x08,
    #[doc = "Timer CTIMER2 Match 1"]
    VAL9 = 0x09,
    #[doc = "Timer CTIMER3 Match 0"]
    VAL10 = 0x0a,
    #[doc = "Timer CTIMER3 Match 1"]
    VAL11 = 0x0b,
    #[doc = "Timer CTIMER4 Match 0"]
    VAL12 = 0x0c,
    #[doc = "Timer CTIMER4 Match 1"]
    VAL13 = 0x0d,
    #[doc = "COMP_OUTPUT"]
    VAL14 = 0x0e,
    #[doc = "DMA0 output trigger mux 0"]
    VAL15 = 0x0f,
    #[doc = "DMA0 output trigger mux 1"]
    VAL16 = 0x10,
    #[doc = "DMA0 output trigger mux 1"]
    VAL17 = 0x11,
    #[doc = "DMA0 output trigger mux 3"]
    VAL18 = 0x12,
    #[doc = "SCT0 DMA request 0"]
    VAL19 = 0x13,
    #[doc = "SCT0 DMA request 1"]
    VAL20 = 0x14,
    #[doc = "HASH DMA RX trigger"]
    VAL21 = 0x15,
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
impl Dma0ItrigInmuxInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma0ItrigInmuxInp {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma0ItrigInmuxInp {
    #[inline(always)]
    fn from(val: u8) -> Dma0ItrigInmuxInp {
        Dma0ItrigInmuxInp::from_bits(val)
    }
}
impl From<Dma0ItrigInmuxInp> for u8 {
    #[inline(always)]
    fn from(val: Dma0ItrigInmuxInp) -> u8 {
        Dma0ItrigInmuxInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma1ItrigInmuxInp {
    #[doc = "Pin interrupt 0"]
    VAL0 = 0x0,
    #[doc = "Pin interrupt 1"]
    VAL1 = 0x01,
    #[doc = "Pin interrupt 2"]
    VAL2 = 0x02,
    #[doc = "Pin interrupt 3"]
    VAL3 = 0x03,
    #[doc = "Timer CTIMER0 Match 0"]
    VAL4 = 0x04,
    #[doc = "Timer CTIMER0 Match 1"]
    VAL5 = 0x05,
    #[doc = "Timer CTIMER2 Match 0"]
    VAL6 = 0x06,
    #[doc = "Timer CTIMER4 Match 0"]
    VAL7 = 0x07,
    #[doc = "DMA1 output trigger mux 0"]
    VAL8 = 0x08,
    #[doc = "DMA1 output trigger mux 1"]
    VAL9 = 0x09,
    #[doc = "DMA1 output trigger mux 2"]
    VAL10 = 0x0a,
    #[doc = "DMA1 output trigger mux 3"]
    VAL11 = 0x0b,
    #[doc = "SCT0 DMA request 0"]
    VAL12 = 0x0c,
    #[doc = "SCT0 DMA request 1"]
    VAL13 = 0x0d,
    #[doc = "HASH DMA RX trigger"]
    VAL14 = 0x0e,
    #[doc = "None"]
    VAL15 = 0x0f,
}
impl Dma1ItrigInmuxInp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma1ItrigInmuxInp {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma1ItrigInmuxInp {
    #[inline(always)]
    fn from(val: u8) -> Dma1ItrigInmuxInp {
        Dma1ItrigInmuxInp::from_bits(val)
    }
}
impl From<Dma1ItrigInmuxInp> for u8 {
    #[inline(always)]
    fn from(val: Dma1ItrigInmuxInp) -> u8 {
        Dma1ItrigInmuxInp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FreqmeasRefClkin {
    #[doc = "External main crystal oscilator (Clock_in)."]
    VALUE0 = 0x0,
    #[doc = "FRO 12MHz clock."]
    VALUE1 = 0x01,
    #[doc = "FRO 96MHz clock."]
    VALUE2 = 0x02,
    #[doc = "Watchdog oscillator / FRO1MHz clock."]
    VALUE3 = 0x03,
    #[doc = "32 kHz oscillator (32k_clk) clock."]
    VALUE4 = 0x04,
    #[doc = "main clock (main_clock)."]
    VALUE5 = 0x05,
    #[doc = "FREQME_GPIO_CLK_A."]
    VALUE6 = 0x06,
    #[doc = "FREQME_GPIO_CLK_B."]
    VALUE7 = 0x07,
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
impl FreqmeasRefClkin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FreqmeasRefClkin {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FreqmeasRefClkin {
    #[inline(always)]
    fn from(val: u8) -> FreqmeasRefClkin {
        FreqmeasRefClkin::from_bits(val)
    }
}
impl From<FreqmeasRefClkin> for u8 {
    #[inline(always)]
    fn from(val: FreqmeasRefClkin) -> u8 {
        FreqmeasRefClkin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FreqmeasTargetClkin {
    #[doc = "External main crystal oscilator (Clock_in)."]
    VALUE0 = 0x0,
    #[doc = "FRO 12MHz clock."]
    VALUE1 = 0x01,
    #[doc = "FRO 96MHz clock."]
    VALUE2 = 0x02,
    #[doc = "Watchdog oscillator / FRO1MHz clock."]
    VALUE3 = 0x03,
    #[doc = "32 kHz oscillator (32k_clk) clock."]
    VALUE4 = 0x04,
    #[doc = "main clock (main_clock)."]
    VALUE5 = 0x05,
    #[doc = "FREQME_GPIO_CLK_A."]
    VALUE6 = 0x06,
    #[doc = "FREQME_GPIO_CLK_B."]
    VALUE7 = 0x07,
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
impl FreqmeasTargetClkin {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FreqmeasTargetClkin {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FreqmeasTargetClkin {
    #[inline(always)]
    fn from(val: u8) -> FreqmeasTargetClkin {
        FreqmeasTargetClkin::from_bits(val)
    }
}
impl From<FreqmeasTargetClkin> for u8 {
    #[inline(always)]
    fn from(val: FreqmeasTargetClkin) -> u8 {
        FreqmeasTargetClkin::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InpN {
    #[doc = "SCT_GPI0 function selected from IOCON register"]
    VAL0 = 0x0,
    #[doc = "SCT_GPI1 function selected from IOCON register"]
    VAL1 = 0x01,
    #[doc = "SCT_GPI2 function selected from IOCON register"]
    VAL2 = 0x02,
    #[doc = "SCT_GPI3 function selected from IOCON register"]
    VAL3 = 0x03,
    #[doc = "SCT_GPI4 function selected from IOCON register"]
    VAL4 = 0x04,
    #[doc = "SCT_GPI5 function selected from IOCON register"]
    VAL5 = 0x05,
    #[doc = "SCT_GPI6 function selected from IOCON register"]
    VAL6 = 0x06,
    #[doc = "SCT_GPI7 function selected from IOCON register"]
    VAL7 = 0x07,
    #[doc = "T0_OUT0 ctimer 0 match\\[0\\] output"]
    VAL8 = 0x08,
    #[doc = "T1_OUT0 ctimer 1 match\\[0\\] output"]
    VAL9 = 0x09,
    #[doc = "T2_OUT0 ctimer 2 match\\[0\\] output"]
    VAL10 = 0x0a,
    #[doc = "T3_OUT0 ctimer 3 match\\[0\\] output"]
    VAL11 = 0x0b,
    #[doc = "T4_OUT0 ctimer 4 match\\[0\\] output"]
    VAL12 = 0x0c,
    #[doc = "ADC_IRQ interrupt request from ADC"]
    VAL13 = 0x0d,
    #[doc = "GPIOINT_BMATCH"]
    VAL14 = 0x0e,
    #[doc = "USB0_FRAME_TOGGLE"]
    VAL15 = 0x0f,
    #[doc = "USB1_FRAME_TOGGLE"]
    VAL16 = 0x10,
    #[doc = "COMP_OUTPUT output from analog comparator"]
    VAL17 = 0x11,
    #[doc = "I2S_SHARED_SCK\\[0\\] output from I2S pin sharing"]
    VAL18 = 0x12,
    #[doc = "I2S_SHARED_SCK\\[1\\] output from I2S pin sharing"]
    VAL19 = 0x13,
    #[doc = "I2S_SHARED_WS\\[0\\] output from I2S pin sharing"]
    VAL20 = 0x14,
    #[doc = "I2S_SHARED_WS\\[1\\] output from I2S pin sharing"]
    VAL21 = 0x15,
    #[doc = "ARM_TXEV interrupt event from cpu0 or cpu1"]
    VAL22 = 0x16,
    #[doc = "DEBUG_HALTED from cpu0 or cpu1"]
    VAL23 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl InpN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InpN {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InpN {
    #[inline(always)]
    fn from(val: u8) -> InpN {
        InpN::from_bits(val)
    }
}
impl From<InpN> for u8 {
    #[inline(always)]
    fn from(val: InpN) -> u8 {
        InpN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer0captselCaptsel {
    #[doc = "CT_INP0 function selected from IOCON register"]
    VAL0 = 0x0,
    #[doc = "CT_INP1 function selected from IOCON register"]
    VAL1 = 0x01,
    #[doc = "CT_INP2 function selected from IOCON register"]
    VAL2 = 0x02,
    #[doc = "CT_INP3 function selected from IOCON register"]
    VAL3 = 0x03,
    #[doc = "CT_INP4 function selected from IOCON register"]
    VAL4 = 0x04,
    #[doc = "CT_INP5 function selected from IOCON register"]
    VAL5 = 0x05,
    #[doc = "CT_INP6 function selected from IOCON register"]
    VAL6 = 0x06,
    #[doc = "CT_INP7 function selected from IOCON register"]
    VAL7 = 0x07,
    #[doc = "CT_INP8 function selected from IOCON register"]
    VAL8 = 0x08,
    #[doc = "CT_INP9 function selected from IOCON register"]
    VAL9 = 0x09,
    #[doc = "CT_INP10 function selected from IOCON register"]
    VAL10 = 0x0a,
    #[doc = "CT_INP11 function selected from IOCON register"]
    VAL11 = 0x0b,
    #[doc = "CT_INP12 function selected from IOCON register"]
    VAL12 = 0x0c,
    #[doc = "CT_INP13 function selected from IOCON register"]
    VAL13 = 0x0d,
    #[doc = "CT_INP14 function selected from IOCON register"]
    VAL14 = 0x0e,
    #[doc = "CT_INP15 function selected from IOCON register"]
    VAL15 = 0x0f,
    #[doc = "CT_INP16 function selected from IOCON register"]
    VAL16 = 0x10,
    #[doc = "None"]
    VAL17 = 0x11,
    #[doc = "None"]
    VAL18 = 0x12,
    #[doc = "None"]
    VAL19 = 0x13,
    #[doc = "USB0_FRAME_TOGGLE"]
    VAL20 = 0x14,
    #[doc = "USB1_FRAME_TOGGLE"]
    VAL21 = 0x15,
    #[doc = "COMP_OUTPUT output from analog comparator"]
    VAL22 = 0x16,
    #[doc = "I2S_SHARED_WS\\[0\\] output from I2S pin sharing"]
    VAL23 = 0x17,
    #[doc = "I2S_SHARED_WS\\[1\\] output from I2S pin sharing"]
    VAL24 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Timer0captselCaptsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer0captselCaptsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer0captselCaptsel {
    #[inline(always)]
    fn from(val: u8) -> Timer0captselCaptsel {
        Timer0captselCaptsel::from_bits(val)
    }
}
impl From<Timer0captselCaptsel> for u8 {
    #[inline(always)]
    fn from(val: Timer0captselCaptsel) -> u8 {
        Timer0captselCaptsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer1captselCaptsel {
    #[doc = "CT_INP0 function selected from IOCON register"]
    VAL0 = 0x0,
    #[doc = "CT_INP1 function selected from IOCON register"]
    VAL1 = 0x01,
    #[doc = "CT_INP2 function selected from IOCON register"]
    VAL2 = 0x02,
    #[doc = "CT_INP3 function selected from IOCON register"]
    VAL3 = 0x03,
    #[doc = "CT_INP4 function selected from IOCON register"]
    VAL4 = 0x04,
    #[doc = "CT_INP5 function selected from IOCON register"]
    VAL5 = 0x05,
    #[doc = "CT_INP6 function selected from IOCON register"]
    VAL6 = 0x06,
    #[doc = "CT_INP7 function selected from IOCON register"]
    VAL7 = 0x07,
    #[doc = "CT_INP8 function selected from IOCON register"]
    VAL8 = 0x08,
    #[doc = "CT_INP9 function selected from IOCON register"]
    VAL9 = 0x09,
    #[doc = "CT_INP10 function selected from IOCON register"]
    VAL10 = 0x0a,
    #[doc = "CT_INP11 function selected from IOCON register"]
    VAL11 = 0x0b,
    #[doc = "CT_INP12 function selected from IOCON register"]
    VAL12 = 0x0c,
    #[doc = "CT_INP13 function selected from IOCON register"]
    VAL13 = 0x0d,
    #[doc = "CT_INP14 function selected from IOCON register"]
    VAL14 = 0x0e,
    #[doc = "CT_INP15 function selected from IOCON register"]
    VAL15 = 0x0f,
    #[doc = "CT_INP16 function selected from IOCON register"]
    VAL16 = 0x10,
    #[doc = "None"]
    VAL17 = 0x11,
    #[doc = "None"]
    VAL18 = 0x12,
    #[doc = "None"]
    VAL19 = 0x13,
    #[doc = "USB0_FRAME_TOGGLE"]
    VAL20 = 0x14,
    #[doc = "USB1_FRAME_TOGGLE"]
    VAL21 = 0x15,
    #[doc = "COMP_OUTPUT output from analog comparator"]
    VAL22 = 0x16,
    #[doc = "I2S_SHARED_WS\\[0\\] output from I2S pin sharing"]
    VAL23 = 0x17,
    #[doc = "I2S_SHARED_WS\\[1\\] output from I2S pin sharing"]
    VAL24 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Timer1captselCaptsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer1captselCaptsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer1captselCaptsel {
    #[inline(always)]
    fn from(val: u8) -> Timer1captselCaptsel {
        Timer1captselCaptsel::from_bits(val)
    }
}
impl From<Timer1captselCaptsel> for u8 {
    #[inline(always)]
    fn from(val: Timer1captselCaptsel) -> u8 {
        Timer1captselCaptsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer2captselCaptsel {
    #[doc = "CT_INP0 function selected from IOCON register"]
    VAL0 = 0x0,
    #[doc = "CT_INP1 function selected from IOCON register"]
    VAL1 = 0x01,
    #[doc = "CT_INP2 function selected from IOCON register"]
    VAL2 = 0x02,
    #[doc = "CT_INP3 function selected from IOCON register"]
    VAL3 = 0x03,
    #[doc = "CT_INP4 function selected from IOCON register"]
    VAL4 = 0x04,
    #[doc = "CT_INP5 function selected from IOCON register"]
    VAL5 = 0x05,
    #[doc = "CT_INP6 function selected from IOCON register"]
    VAL6 = 0x06,
    #[doc = "CT_INP7 function selected from IOCON register"]
    VAL7 = 0x07,
    #[doc = "CT_INP8 function selected from IOCON register"]
    VAL8 = 0x08,
    #[doc = "CT_INP9 function selected from IOCON register"]
    VAL9 = 0x09,
    #[doc = "CT_INP10 function selected from IOCON register"]
    VAL10 = 0x0a,
    #[doc = "CT_INP11 function selected from IOCON register"]
    VAL11 = 0x0b,
    #[doc = "CT_INP12 function selected from IOCON register"]
    VAL12 = 0x0c,
    #[doc = "CT_INP13 function selected from IOCON register"]
    VAL13 = 0x0d,
    #[doc = "CT_INP14 function selected from IOCON register"]
    VAL14 = 0x0e,
    #[doc = "CT_INP15 function selected from IOCON register"]
    VAL15 = 0x0f,
    #[doc = "CT_INP16 function selected from IOCON register"]
    VAL16 = 0x10,
    #[doc = "None"]
    VAL17 = 0x11,
    #[doc = "None"]
    VAL18 = 0x12,
    #[doc = "None"]
    VAL19 = 0x13,
    #[doc = "USB0_FRAME_TOGGLE"]
    VAL20 = 0x14,
    #[doc = "USB1_FRAME_TOGGLE"]
    VAL21 = 0x15,
    #[doc = "COMP_OUTPUT output from analog comparator"]
    VAL22 = 0x16,
    #[doc = "I2S_SHARED_WS\\[0\\] output from I2S pin sharing"]
    VAL23 = 0x17,
    #[doc = "I2S_SHARED_WS\\[1\\] output from I2S pin sharing"]
    VAL24 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Timer2captselCaptsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer2captselCaptsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer2captselCaptsel {
    #[inline(always)]
    fn from(val: u8) -> Timer2captselCaptsel {
        Timer2captselCaptsel::from_bits(val)
    }
}
impl From<Timer2captselCaptsel> for u8 {
    #[inline(always)]
    fn from(val: Timer2captselCaptsel) -> u8 {
        Timer2captselCaptsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer3captselCaptsel {
    #[doc = "CT_INP0 function selected from IOCON register"]
    VAL0 = 0x0,
    #[doc = "CT_INP1 function selected from IOCON register"]
    VAL1 = 0x01,
    #[doc = "CT_INP2 function selected from IOCON register"]
    VAL2 = 0x02,
    #[doc = "CT_INP3 function selected from IOCON register"]
    VAL3 = 0x03,
    #[doc = "CT_INP4 function selected from IOCON register"]
    VAL4 = 0x04,
    #[doc = "CT_INP5 function selected from IOCON register"]
    VAL5 = 0x05,
    #[doc = "CT_INP6 function selected from IOCON register"]
    VAL6 = 0x06,
    #[doc = "CT_INP7 function selected from IOCON register"]
    VAL7 = 0x07,
    #[doc = "CT_INP8 function selected from IOCON register"]
    VAL8 = 0x08,
    #[doc = "CT_INP9 function selected from IOCON register"]
    VAL9 = 0x09,
    #[doc = "CT_INP10 function selected from IOCON register"]
    VAL10 = 0x0a,
    #[doc = "CT_INP11 function selected from IOCON register"]
    VAL11 = 0x0b,
    #[doc = "CT_INP12 function selected from IOCON register"]
    VAL12 = 0x0c,
    #[doc = "CT_INP13 function selected from IOCON register"]
    VAL13 = 0x0d,
    #[doc = "CT_INP14 function selected from IOCON register"]
    VAL14 = 0x0e,
    #[doc = "CT_INP15 function selected from IOCON register"]
    VAL15 = 0x0f,
    #[doc = "CT_INP16 function selected from IOCON register"]
    VAL16 = 0x10,
    #[doc = "CT_INP17 function selected from IOCON register"]
    VAL17 = 0x11,
    #[doc = "CT_INP18 function selected from IOCON register"]
    VAL18 = 0x12,
    #[doc = "CT_INP19 function selected from IOCON register"]
    VAL19 = 0x13,
    #[doc = "USB0_FRAME_TOGGLE"]
    VAL20 = 0x14,
    #[doc = "USB1_FRAME_TOGGLE"]
    VAL21 = 0x15,
    #[doc = "COMP_OUTPUT output from analog comparator"]
    VAL22 = 0x16,
    #[doc = "I2S_SHARED_WS\\[0\\] output from I2S pin sharing"]
    VAL23 = 0x17,
    #[doc = "I2S_SHARED_WS\\[1\\] output from I2S pin sharing"]
    VAL24 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Timer3captselCaptsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer3captselCaptsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer3captselCaptsel {
    #[inline(always)]
    fn from(val: u8) -> Timer3captselCaptsel {
        Timer3captselCaptsel::from_bits(val)
    }
}
impl From<Timer3captselCaptsel> for u8 {
    #[inline(always)]
    fn from(val: Timer3captselCaptsel) -> u8 {
        Timer3captselCaptsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer4captselCaptsel {
    #[doc = "CT_INP0 function selected from IOCON register"]
    VAL0 = 0x0,
    #[doc = "CT_INP1 function selected from IOCON register"]
    VAL1 = 0x01,
    #[doc = "CT_INP2 function selected from IOCON register"]
    VAL2 = 0x02,
    #[doc = "CT_INP3 function selected from IOCON register"]
    VAL3 = 0x03,
    #[doc = "CT_INP4 function selected from IOCON register"]
    VAL4 = 0x04,
    #[doc = "CT_INP5 function selected from IOCON register"]
    VAL5 = 0x05,
    #[doc = "CT_INP6 function selected from IOCON register"]
    VAL6 = 0x06,
    #[doc = "CT_INP7 function selected from IOCON register"]
    VAL7 = 0x07,
    #[doc = "CT_INP8 function selected from IOCON register"]
    VAL8 = 0x08,
    #[doc = "CT_INP9 function selected from IOCON register"]
    VAL9 = 0x09,
    #[doc = "CT_INP10 function selected from IOCON register"]
    VAL10 = 0x0a,
    #[doc = "CT_INP11 function selected from IOCON register"]
    VAL11 = 0x0b,
    #[doc = "CT_INP12 function selected from IOCON register"]
    VAL12 = 0x0c,
    #[doc = "CT_INP13 function selected from IOCON register"]
    VAL13 = 0x0d,
    #[doc = "CT_INP14 function selected from IOCON register"]
    VAL14 = 0x0e,
    #[doc = "CT_INP15 function selected from IOCON register"]
    VAL15 = 0x0f,
    #[doc = "CT_INP16 function selected from IOCON register"]
    VAL16 = 0x10,
    #[doc = "CT_INP17 function selected from IOCON register"]
    VAL17 = 0x11,
    #[doc = "CT_INP18 function selected from IOCON register"]
    VAL18 = 0x12,
    #[doc = "CT_INP19 function selected from IOCON register"]
    VAL19 = 0x13,
    #[doc = "USB0_FRAME_TOGGLE"]
    VAL20 = 0x14,
    #[doc = "USB1_FRAME_TOGGLE"]
    VAL21 = 0x15,
    #[doc = "COMP_OUTPUT output from analog comparator"]
    VAL22 = 0x16,
    #[doc = "I2S_SHARED_WS\\[0\\] output from I2S pin sharing"]
    VAL23 = 0x17,
    #[doc = "I2S_SHARED_WS\\[1\\] output from I2S pin sharing"]
    VAL24 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Timer4captselCaptsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer4captselCaptsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer4captselCaptsel {
    #[inline(always)]
    fn from(val: u8) -> Timer4captselCaptsel {
        Timer4captselCaptsel::from_bits(val)
    }
}
impl From<Timer4captselCaptsel> for u8 {
    #[inline(always)]
    fn from(val: Timer4captselCaptsel) -> u8 {
        Timer4captselCaptsel::to_bits(val)
    }
}
