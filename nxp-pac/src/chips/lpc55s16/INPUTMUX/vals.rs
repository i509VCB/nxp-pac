#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DMA0_ITRIG_INMUX_INP {
    #[doc = "Pin interrupt 0."]
    val0 = 0x0,
    #[doc = "Pin interrupt 1."]
    val1 = 0x01,
    #[doc = "Pin interrupt 2."]
    val2 = 0x02,
    #[doc = "Pin interrupt 3."]
    val3 = 0x03,
    #[doc = "Timer CTIMER0 Match 0."]
    val4 = 0x04,
    #[doc = "Timer CTIMER0 Match 1."]
    val5 = 0x05,
    #[doc = "Timer CTIMER1 Match 0."]
    val6 = 0x06,
    #[doc = "Timer CTIMER1 Match 1."]
    val7 = 0x07,
    #[doc = "Timer CTIMER2 Match 0."]
    val8 = 0x08,
    #[doc = "Timer CTIMER2 Match 1."]
    val9 = 0x09,
    #[doc = "Timer CTIMER3 Match 0."]
    val10 = 0x0a,
    #[doc = "Timer CTIMER3 Match 1."]
    val11 = 0x0b,
    #[doc = "Timer CTIMER4 Match 0."]
    val12 = 0x0c,
    #[doc = "Timer CTIMER4 Match 1."]
    val13 = 0x0d,
    #[doc = "COMP_OUTPUT."]
    val14 = 0x0e,
    #[doc = "DMA0 output trigger mux 0."]
    val15 = 0x0f,
    #[doc = "DMA0 output trigger mux 1."]
    val16 = 0x10,
    #[doc = "DMA0 output trigger mux 1."]
    val17 = 0x11,
    #[doc = "DMA0 output trigger mux 3."]
    val18 = 0x12,
    #[doc = "SCT0 DMA request 0."]
    val19 = 0x13,
    #[doc = "SCT0 DMA request 1."]
    val20 = 0x14,
    #[doc = "HASH DMA RX trigger."]
    val21 = 0x15,
    #[doc = "None."]
    val22_22 = 0x16,
    #[doc = "None."]
    val22_23 = 0x17,
    #[doc = "None."]
    val22_24 = 0x18,
    #[doc = "None."]
    val22_25 = 0x19,
    #[doc = "None."]
    val22_26 = 0x1a,
    #[doc = "None."]
    val22_27 = 0x1b,
    #[doc = "None."]
    val22_28 = 0x1c,
    #[doc = "None."]
    val22_29 = 0x1d,
    #[doc = "None."]
    val22_30 = 0x1e,
    #[doc = "None."]
    val22_31 = 0x1f,
}
impl DMA0_ITRIG_INMUX_INP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DMA0_ITRIG_INMUX_INP {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DMA0_ITRIG_INMUX_INP {
    #[inline(always)]
    fn from(val: u8) -> DMA0_ITRIG_INMUX_INP {
        DMA0_ITRIG_INMUX_INP::from_bits(val)
    }
}
impl From<DMA0_ITRIG_INMUX_INP> for u8 {
    #[inline(always)]
    fn from(val: DMA0_ITRIG_INMUX_INP) -> u8 {
        DMA0_ITRIG_INMUX_INP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DMA1_ITRIG_INMUX_INP {
    #[doc = "Pin interrupt 0."]
    val0 = 0x0,
    #[doc = "Pin interrupt 1."]
    val1 = 0x01,
    #[doc = "Pin interrupt 2."]
    val2 = 0x02,
    #[doc = "Pin interrupt 3."]
    val3 = 0x03,
    #[doc = "Timer CTIMER0 Match 0."]
    val4 = 0x04,
    #[doc = "Timer CTIMER0 Match 1."]
    val5 = 0x05,
    #[doc = "Timer CTIMER2 Match 0."]
    val6 = 0x06,
    #[doc = "Timer CTIMER4 Match 0."]
    val7 = 0x07,
    #[doc = "DMA1 output trigger mux 0."]
    val8 = 0x08,
    #[doc = "DMA1 output trigger mux 1."]
    val9 = 0x09,
    #[doc = "DMA1 output trigger mux 2."]
    val10 = 0x0a,
    #[doc = "DMA1 output trigger mux 3."]
    val11 = 0x0b,
    #[doc = "SCT0 DMA request 0."]
    val12 = 0x0c,
    #[doc = "SCT0 DMA request 1."]
    val13 = 0x0d,
    #[doc = "HASH DMA RX trigger."]
    val14 = 0x0e,
    #[doc = "None."]
    val15 = 0x0f,
}
impl DMA1_ITRIG_INMUX_INP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DMA1_ITRIG_INMUX_INP {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DMA1_ITRIG_INMUX_INP {
    #[inline(always)]
    fn from(val: u8) -> DMA1_ITRIG_INMUX_INP {
        DMA1_ITRIG_INMUX_INP::from_bits(val)
    }
}
impl From<DMA1_ITRIG_INMUX_INP> for u8 {
    #[inline(always)]
    fn from(val: DMA1_ITRIG_INMUX_INP) -> u8 {
        DMA1_ITRIG_INMUX_INP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FREQMEAS_REF_CLKIN {
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
impl FREQMEAS_REF_CLKIN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FREQMEAS_REF_CLKIN {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FREQMEAS_REF_CLKIN {
    #[inline(always)]
    fn from(val: u8) -> FREQMEAS_REF_CLKIN {
        FREQMEAS_REF_CLKIN::from_bits(val)
    }
}
impl From<FREQMEAS_REF_CLKIN> for u8 {
    #[inline(always)]
    fn from(val: FREQMEAS_REF_CLKIN) -> u8 {
        FREQMEAS_REF_CLKIN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FREQMEAS_TARGET_CLKIN {
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
impl FREQMEAS_TARGET_CLKIN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FREQMEAS_TARGET_CLKIN {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FREQMEAS_TARGET_CLKIN {
    #[inline(always)]
    fn from(val: u8) -> FREQMEAS_TARGET_CLKIN {
        FREQMEAS_TARGET_CLKIN::from_bits(val)
    }
}
impl From<FREQMEAS_TARGET_CLKIN> for u8 {
    #[inline(always)]
    fn from(val: FREQMEAS_TARGET_CLKIN) -> u8 {
        FREQMEAS_TARGET_CLKIN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum INP_N {
    #[doc = "SCT_GPI0 function selected from IOCON register."]
    val0 = 0x0,
    #[doc = "SCT_GPI1 function selected from IOCON register."]
    val1 = 0x01,
    #[doc = "SCT_GPI2 function selected from IOCON register."]
    val2 = 0x02,
    #[doc = "SCT_GPI3 function selected from IOCON register."]
    val3 = 0x03,
    #[doc = "SCT_GPI4 function selected from IOCON register."]
    val4 = 0x04,
    #[doc = "SCT_GPI5 function selected from IOCON register."]
    val5 = 0x05,
    #[doc = "SCT_GPI6 function selected from IOCON register."]
    val6 = 0x06,
    #[doc = "SCT_GPI7 function selected from IOCON register."]
    val7 = 0x07,
    #[doc = "T0_OUT0 ctimer 0 match\\[0\\] output."]
    val8 = 0x08,
    #[doc = "T1_OUT0 ctimer 1 match\\[0\\] output."]
    val9 = 0x09,
    #[doc = "T2_OUT0 ctimer 2 match\\[0\\] output."]
    val10 = 0x0a,
    #[doc = "T3_OUT0 ctimer 3 match\\[0\\] output."]
    val11 = 0x0b,
    #[doc = "T4_OUT0 ctimer 4 match\\[0\\] output."]
    val12 = 0x0c,
    #[doc = "ADC_IRQ interrupt request from ADC."]
    val13 = 0x0d,
    #[doc = "GPIOINT_BMATCH."]
    val14 = 0x0e,
    #[doc = "USB0_FRAME_TOGGLE."]
    val15 = 0x0f,
    #[doc = "USB1_FRAME_TOGGLE."]
    val16 = 0x10,
    #[doc = "COMP_OUTPUT output from analog comparator."]
    val17 = 0x11,
    #[doc = "I2S_SHARED_SCK\\[0\\] output from I2S pin sharing."]
    val18 = 0x12,
    #[doc = "I2S_SHARED_SCK\\[1\\] output from I2S pin sharing."]
    val19 = 0x13,
    #[doc = "I2S_SHARED_WS\\[0\\] output from I2S pin sharing."]
    val20 = 0x14,
    #[doc = "I2S_SHARED_WS\\[1\\] output from I2S pin sharing."]
    val21 = 0x15,
    #[doc = "ARM_TXEV interrupt event from cpu0 or cpu1."]
    val22 = 0x16,
    #[doc = "DEBUG_HALTED from cpu0 or cpu1."]
    val23 = 0x17,
    #[doc = "None."]
    val24_24 = 0x18,
    #[doc = "None."]
    val24_25 = 0x19,
    #[doc = "None."]
    val24_26 = 0x1a,
    #[doc = "None."]
    val24_27 = 0x1b,
    #[doc = "None."]
    val24_28 = 0x1c,
    #[doc = "None."]
    val24_29 = 0x1d,
    #[doc = "None."]
    val24_30 = 0x1e,
    #[doc = "None."]
    val24_31 = 0x1f,
}
impl INP_N {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> INP_N {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for INP_N {
    #[inline(always)]
    fn from(val: u8) -> INP_N {
        INP_N::from_bits(val)
    }
}
impl From<INP_N> for u8 {
    #[inline(always)]
    fn from(val: INP_N) -> u8 {
        INP_N::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TIMER0CAPTSEL_CAPTSEL {
    #[doc = "CT_INP0 function selected from IOCON register."]
    val0 = 0x0,
    #[doc = "CT_INP1 function selected from IOCON register."]
    val1 = 0x01,
    #[doc = "CT_INP2 function selected from IOCON register."]
    val2 = 0x02,
    #[doc = "CT_INP3 function selected from IOCON register."]
    val3 = 0x03,
    #[doc = "CT_INP4 function selected from IOCON register."]
    val4 = 0x04,
    #[doc = "CT_INP5 function selected from IOCON register."]
    val5 = 0x05,
    #[doc = "CT_INP6 function selected from IOCON register."]
    val6 = 0x06,
    #[doc = "CT_INP7 function selected from IOCON register."]
    val7 = 0x07,
    #[doc = "CT_INP8 function selected from IOCON register."]
    val8 = 0x08,
    #[doc = "CT_INP9 function selected from IOCON register."]
    val9 = 0x09,
    #[doc = "CT_INP10 function selected from IOCON register."]
    val10 = 0x0a,
    #[doc = "CT_INP11 function selected from IOCON register."]
    val11 = 0x0b,
    #[doc = "CT_INP12 function selected from IOCON register."]
    val12 = 0x0c,
    #[doc = "CT_INP13 function selected from IOCON register."]
    val13 = 0x0d,
    #[doc = "CT_INP14 function selected from IOCON register."]
    val14 = 0x0e,
    #[doc = "CT_INP15 function selected from IOCON register."]
    val15 = 0x0f,
    #[doc = "CT_INP16 function selected from IOCON register."]
    val16 = 0x10,
    #[doc = "None."]
    val17 = 0x11,
    #[doc = "None."]
    val18 = 0x12,
    #[doc = "None."]
    val19 = 0x13,
    #[doc = "USB0_FRAME_TOGGLE."]
    val20 = 0x14,
    #[doc = "USB1_FRAME_TOGGLE."]
    val21 = 0x15,
    #[doc = "COMP_OUTPUT output from analog comparator."]
    val22 = 0x16,
    #[doc = "I2S_SHARED_WS\\[0\\] output from I2S pin sharing."]
    val23 = 0x17,
    #[doc = "I2S_SHARED_WS\\[1\\] output from I2S pin sharing."]
    val24 = 0x18,
    #[doc = "None."]
    val25_25 = 0x19,
    #[doc = "None."]
    val25_26 = 0x1a,
    #[doc = "None."]
    val25_27 = 0x1b,
    #[doc = "None."]
    val25_28 = 0x1c,
    #[doc = "None."]
    val25_29 = 0x1d,
    #[doc = "None."]
    val25_30 = 0x1e,
    #[doc = "None."]
    val25_31 = 0x1f,
}
impl TIMER0CAPTSEL_CAPTSEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TIMER0CAPTSEL_CAPTSEL {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TIMER0CAPTSEL_CAPTSEL {
    #[inline(always)]
    fn from(val: u8) -> TIMER0CAPTSEL_CAPTSEL {
        TIMER0CAPTSEL_CAPTSEL::from_bits(val)
    }
}
impl From<TIMER0CAPTSEL_CAPTSEL> for u8 {
    #[inline(always)]
    fn from(val: TIMER0CAPTSEL_CAPTSEL) -> u8 {
        TIMER0CAPTSEL_CAPTSEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TIMER1CAPTSEL_CAPTSEL {
    #[doc = "CT_INP0 function selected from IOCON register."]
    val0 = 0x0,
    #[doc = "CT_INP1 function selected from IOCON register."]
    val1 = 0x01,
    #[doc = "CT_INP2 function selected from IOCON register."]
    val2 = 0x02,
    #[doc = "CT_INP3 function selected from IOCON register."]
    val3 = 0x03,
    #[doc = "CT_INP4 function selected from IOCON register."]
    val4 = 0x04,
    #[doc = "CT_INP5 function selected from IOCON register."]
    val5 = 0x05,
    #[doc = "CT_INP6 function selected from IOCON register."]
    val6 = 0x06,
    #[doc = "CT_INP7 function selected from IOCON register."]
    val7 = 0x07,
    #[doc = "CT_INP8 function selected from IOCON register."]
    val8 = 0x08,
    #[doc = "CT_INP9 function selected from IOCON register."]
    val9 = 0x09,
    #[doc = "CT_INP10 function selected from IOCON register."]
    val10 = 0x0a,
    #[doc = "CT_INP11 function selected from IOCON register."]
    val11 = 0x0b,
    #[doc = "CT_INP12 function selected from IOCON register."]
    val12 = 0x0c,
    #[doc = "CT_INP13 function selected from IOCON register."]
    val13 = 0x0d,
    #[doc = "CT_INP14 function selected from IOCON register."]
    val14 = 0x0e,
    #[doc = "CT_INP15 function selected from IOCON register."]
    val15 = 0x0f,
    #[doc = "CT_INP16 function selected from IOCON register."]
    val16 = 0x10,
    #[doc = "None."]
    val17 = 0x11,
    #[doc = "None."]
    val18 = 0x12,
    #[doc = "None."]
    val19 = 0x13,
    #[doc = "USB0_FRAME_TOGGLE."]
    val20 = 0x14,
    #[doc = "USB1_FRAME_TOGGLE."]
    val21 = 0x15,
    #[doc = "COMP_OUTPUT output from analog comparator."]
    val22 = 0x16,
    #[doc = "I2S_SHARED_WS\\[0\\] output from I2S pin sharing."]
    val23 = 0x17,
    #[doc = "I2S_SHARED_WS\\[1\\] output from I2S pin sharing."]
    val24 = 0x18,
    #[doc = "None."]
    val25_25 = 0x19,
    #[doc = "None."]
    val25_26 = 0x1a,
    #[doc = "None."]
    val25_27 = 0x1b,
    #[doc = "None."]
    val25_28 = 0x1c,
    #[doc = "None."]
    val25_29 = 0x1d,
    #[doc = "None."]
    val25_30 = 0x1e,
    #[doc = "None."]
    val25_31 = 0x1f,
}
impl TIMER1CAPTSEL_CAPTSEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TIMER1CAPTSEL_CAPTSEL {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TIMER1CAPTSEL_CAPTSEL {
    #[inline(always)]
    fn from(val: u8) -> TIMER1CAPTSEL_CAPTSEL {
        TIMER1CAPTSEL_CAPTSEL::from_bits(val)
    }
}
impl From<TIMER1CAPTSEL_CAPTSEL> for u8 {
    #[inline(always)]
    fn from(val: TIMER1CAPTSEL_CAPTSEL) -> u8 {
        TIMER1CAPTSEL_CAPTSEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TIMER2CAPTSEL_CAPTSEL {
    #[doc = "CT_INP0 function selected from IOCON register."]
    val0 = 0x0,
    #[doc = "CT_INP1 function selected from IOCON register."]
    val1 = 0x01,
    #[doc = "CT_INP2 function selected from IOCON register."]
    val2 = 0x02,
    #[doc = "CT_INP3 function selected from IOCON register."]
    val3 = 0x03,
    #[doc = "CT_INP4 function selected from IOCON register."]
    val4 = 0x04,
    #[doc = "CT_INP5 function selected from IOCON register."]
    val5 = 0x05,
    #[doc = "CT_INP6 function selected from IOCON register."]
    val6 = 0x06,
    #[doc = "CT_INP7 function selected from IOCON register."]
    val7 = 0x07,
    #[doc = "CT_INP8 function selected from IOCON register."]
    val8 = 0x08,
    #[doc = "CT_INP9 function selected from IOCON register."]
    val9 = 0x09,
    #[doc = "CT_INP10 function selected from IOCON register."]
    val10 = 0x0a,
    #[doc = "CT_INP11 function selected from IOCON register."]
    val11 = 0x0b,
    #[doc = "CT_INP12 function selected from IOCON register."]
    val12 = 0x0c,
    #[doc = "CT_INP13 function selected from IOCON register."]
    val13 = 0x0d,
    #[doc = "CT_INP14 function selected from IOCON register."]
    val14 = 0x0e,
    #[doc = "CT_INP15 function selected from IOCON register."]
    val15 = 0x0f,
    #[doc = "CT_INP16 function selected from IOCON register."]
    val16 = 0x10,
    #[doc = "None."]
    val17 = 0x11,
    #[doc = "None."]
    val18 = 0x12,
    #[doc = "None."]
    val19 = 0x13,
    #[doc = "USB0_FRAME_TOGGLE."]
    val20 = 0x14,
    #[doc = "USB1_FRAME_TOGGLE."]
    val21 = 0x15,
    #[doc = "COMP_OUTPUT output from analog comparator."]
    val22 = 0x16,
    #[doc = "I2S_SHARED_WS\\[0\\] output from I2S pin sharing."]
    val23 = 0x17,
    #[doc = "I2S_SHARED_WS\\[1\\] output from I2S pin sharing."]
    val24 = 0x18,
    #[doc = "None."]
    val25_25 = 0x19,
    #[doc = "None."]
    val25_26 = 0x1a,
    #[doc = "None."]
    val25_27 = 0x1b,
    #[doc = "None."]
    val25_28 = 0x1c,
    #[doc = "None."]
    val25_29 = 0x1d,
    #[doc = "None."]
    val25_30 = 0x1e,
    #[doc = "None."]
    val25_31 = 0x1f,
}
impl TIMER2CAPTSEL_CAPTSEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TIMER2CAPTSEL_CAPTSEL {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TIMER2CAPTSEL_CAPTSEL {
    #[inline(always)]
    fn from(val: u8) -> TIMER2CAPTSEL_CAPTSEL {
        TIMER2CAPTSEL_CAPTSEL::from_bits(val)
    }
}
impl From<TIMER2CAPTSEL_CAPTSEL> for u8 {
    #[inline(always)]
    fn from(val: TIMER2CAPTSEL_CAPTSEL) -> u8 {
        TIMER2CAPTSEL_CAPTSEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TIMER3CAPTSEL_CAPTSEL {
    #[doc = "CT_INP0 function selected from IOCON register."]
    val0 = 0x0,
    #[doc = "CT_INP1 function selected from IOCON register."]
    val1 = 0x01,
    #[doc = "CT_INP2 function selected from IOCON register."]
    val2 = 0x02,
    #[doc = "CT_INP3 function selected from IOCON register."]
    val3 = 0x03,
    #[doc = "CT_INP4 function selected from IOCON register."]
    val4 = 0x04,
    #[doc = "CT_INP5 function selected from IOCON register."]
    val5 = 0x05,
    #[doc = "CT_INP6 function selected from IOCON register."]
    val6 = 0x06,
    #[doc = "CT_INP7 function selected from IOCON register."]
    val7 = 0x07,
    #[doc = "CT_INP8 function selected from IOCON register."]
    val8 = 0x08,
    #[doc = "CT_INP9 function selected from IOCON register."]
    val9 = 0x09,
    #[doc = "CT_INP10 function selected from IOCON register."]
    val10 = 0x0a,
    #[doc = "CT_INP11 function selected from IOCON register."]
    val11 = 0x0b,
    #[doc = "CT_INP12 function selected from IOCON register."]
    val12 = 0x0c,
    #[doc = "CT_INP13 function selected from IOCON register."]
    val13 = 0x0d,
    #[doc = "CT_INP14 function selected from IOCON register."]
    val14 = 0x0e,
    #[doc = "CT_INP15 function selected from IOCON register."]
    val15 = 0x0f,
    #[doc = "CT_INP16 function selected from IOCON register."]
    val16 = 0x10,
    #[doc = "None."]
    val17 = 0x11,
    #[doc = "None."]
    val18 = 0x12,
    #[doc = "None."]
    val19 = 0x13,
    #[doc = "USB0_FRAME_TOGGLE."]
    val20 = 0x14,
    #[doc = "USB1_FRAME_TOGGLE."]
    val21 = 0x15,
    #[doc = "COMP_OUTPUT output from analog comparator."]
    val22 = 0x16,
    #[doc = "I2S_SHARED_WS\\[0\\] output from I2S pin sharing."]
    val23 = 0x17,
    #[doc = "I2S_SHARED_WS\\[1\\] output from I2S pin sharing."]
    val24 = 0x18,
    #[doc = "None."]
    val25_25 = 0x19,
    #[doc = "None."]
    val25_26 = 0x1a,
    #[doc = "None."]
    val25_27 = 0x1b,
    #[doc = "None."]
    val25_28 = 0x1c,
    #[doc = "None."]
    val25_29 = 0x1d,
    #[doc = "None."]
    val25_30 = 0x1e,
    #[doc = "None."]
    val25_31 = 0x1f,
}
impl TIMER3CAPTSEL_CAPTSEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TIMER3CAPTSEL_CAPTSEL {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TIMER3CAPTSEL_CAPTSEL {
    #[inline(always)]
    fn from(val: u8) -> TIMER3CAPTSEL_CAPTSEL {
        TIMER3CAPTSEL_CAPTSEL::from_bits(val)
    }
}
impl From<TIMER3CAPTSEL_CAPTSEL> for u8 {
    #[inline(always)]
    fn from(val: TIMER3CAPTSEL_CAPTSEL) -> u8 {
        TIMER3CAPTSEL_CAPTSEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TIMER4CAPTSEL_CAPTSEL {
    #[doc = "CT_INP0 function selected from IOCON register."]
    val0 = 0x0,
    #[doc = "CT_INP1 function selected from IOCON register."]
    val1 = 0x01,
    #[doc = "CT_INP2 function selected from IOCON register."]
    val2 = 0x02,
    #[doc = "CT_INP3 function selected from IOCON register."]
    val3 = 0x03,
    #[doc = "CT_INP4 function selected from IOCON register."]
    val4 = 0x04,
    #[doc = "CT_INP5 function selected from IOCON register."]
    val5 = 0x05,
    #[doc = "CT_INP6 function selected from IOCON register."]
    val6 = 0x06,
    #[doc = "CT_INP7 function selected from IOCON register."]
    val7 = 0x07,
    #[doc = "CT_INP8 function selected from IOCON register."]
    val8 = 0x08,
    #[doc = "CT_INP9 function selected from IOCON register."]
    val9 = 0x09,
    #[doc = "CT_INP10 function selected from IOCON register."]
    val10 = 0x0a,
    #[doc = "CT_INP11 function selected from IOCON register."]
    val11 = 0x0b,
    #[doc = "CT_INP12 function selected from IOCON register."]
    val12 = 0x0c,
    #[doc = "CT_INP13 function selected from IOCON register."]
    val13 = 0x0d,
    #[doc = "CT_INP14 function selected from IOCON register."]
    val14 = 0x0e,
    #[doc = "CT_INP15 function selected from IOCON register."]
    val15 = 0x0f,
    #[doc = "CT_INP16 function selected from IOCON register."]
    val16 = 0x10,
    #[doc = "None."]
    val17 = 0x11,
    #[doc = "None."]
    val18 = 0x12,
    #[doc = "None."]
    val19 = 0x13,
    #[doc = "USB0_FRAME_TOGGLE."]
    val20 = 0x14,
    #[doc = "USB1_FRAME_TOGGLE."]
    val21 = 0x15,
    #[doc = "COMP_OUTPUT output from analog comparator."]
    val22 = 0x16,
    #[doc = "I2S_SHARED_WS\\[0\\] output from I2S pin sharing."]
    val23 = 0x17,
    #[doc = "I2S_SHARED_WS\\[1\\] output from I2S pin sharing."]
    val24 = 0x18,
    #[doc = "None."]
    val25_25 = 0x19,
    #[doc = "None."]
    val25_26 = 0x1a,
    #[doc = "None."]
    val25_27 = 0x1b,
    #[doc = "None."]
    val25_28 = 0x1c,
    #[doc = "None."]
    val25_29 = 0x1d,
    #[doc = "None."]
    val25_30 = 0x1e,
    #[doc = "None."]
    val25_31 = 0x1f,
}
impl TIMER4CAPTSEL_CAPTSEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TIMER4CAPTSEL_CAPTSEL {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TIMER4CAPTSEL_CAPTSEL {
    #[inline(always)]
    fn from(val: u8) -> TIMER4CAPTSEL_CAPTSEL {
        TIMER4CAPTSEL_CAPTSEL::from_bits(val)
    }
}
impl From<TIMER4CAPTSEL_CAPTSEL> for u8 {
    #[inline(always)]
    fn from(val: TIMER4CAPTSEL_CAPTSEL) -> u8 {
        TIMER4CAPTSEL_CAPTSEL::to_bits(val)
    }
}
