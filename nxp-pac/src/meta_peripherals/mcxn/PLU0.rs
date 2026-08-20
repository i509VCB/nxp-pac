#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "Array of registers: LUT_INP_MUX%s."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lut {
    ptr: *mut u8,
}
unsafe impl Send for Lut {}
unsafe impl Sync for Lut {}
impl Lut {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Input select register for LUTn (0 to 25), Inputx (5 inputs)."]
    #[inline(always)]
    pub const fn lut_inp_mux(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<LutInpMux, crate::pac::common::RW> {
        assert!(n < 5usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _)
        }
    }
}
#[doc = "Programmable Logic Unit (PLU)."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Plu0 {
    ptr: *mut u8,
}
unsafe impl Send for Plu0 {}
unsafe impl Sync for Plu0 {}
impl Plu0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Array of registers: LUT_INP_MUX%s."]
    #[inline(always)]
    pub const fn lut(self, n: usize) -> Lut {
        assert!(n < 26usize);
        unsafe { Lut::from_ptr(self.ptr.wrapping_add(0x0usize + n * 32usize) as _) }
    }
    #[doc = "PLU LUT truth table."]
    #[inline(always)]
    pub const fn lut_truth(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<LutTruth, crate::pac::common::RW> {
        assert!(n < 26usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0800usize + n * 4usize) as _)
        }
    }
    #[doc = "PLU outputs."]
    #[inline(always)]
    pub const fn outputs(self) -> crate::pac::common::Reg<Outputs, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0900usize) as _) }
    }
    #[doc = "Wakeup interrupt control."]
    #[inline(always)]
    pub const fn wakeint_ctrl(
        self,
    ) -> crate::pac::common::Reg<WakeintCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0904usize) as _) }
    }
    #[doc = "PLU output multiplexer."]
    #[inline(always)]
    pub const fn output_mux(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<OutputMux, crate::pac::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c00usize + n * 4usize) as _)
        }
    }
}
#[doc = "Input select register for LUTn (0 to 25), Inputx (5 inputs)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LutInpMux(pub u32);
impl LutInpMux {
    #[doc = "Selects the input source to be connected to LUTn_INPx."]
    #[must_use]
    #[inline(always)]
    pub const fn lu_tn_in_px(&self) -> LuTnInPx {
        let val = (self.0 >> 0usize) & 0x3f;
        LuTnInPx::from_bits(val as u8)
    }
    #[doc = "Selects the input source to be connected to LUTn_INPx."]
    #[inline(always)]
    pub const fn set_lu_tn_in_px(&mut self, val: LuTnInPx) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for LutInpMux {
    #[inline(always)]
    fn default() -> LutInpMux {
        LutInpMux(0)
    }
}
impl core::fmt::Debug for LutInpMux {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LutInpMux")
            .field("lu_tn_in_px", &self.lu_tn_in_px())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LutInpMux {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LutInpMux {{ lu_tn_in_px: {:?} }}", self.lu_tn_in_px())
    }
}
#[doc = "PLU LUT truth table."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LutTruth(pub u32);
impl LutTruth {
    #[doc = "LUT truth table."]
    #[must_use]
    #[inline(always)]
    pub const fn lut_truth(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "LUT truth table."]
    #[inline(always)]
    pub const fn set_lut_truth(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for LutTruth {
    #[inline(always)]
    fn default() -> LutTruth {
        LutTruth(0)
    }
}
impl core::fmt::Debug for LutTruth {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LutTruth")
            .field("lut_truth", &self.lut_truth())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LutTruth {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LutTruth {{ lut_truth: {=u32:?} }}", self.lut_truth())
    }
}
#[doc = "PLU output multiplexer."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutputMux(pub u32);
impl OutputMux {
    #[doc = "Selects the source to be connected to PLU output n."]
    #[must_use]
    #[inline(always)]
    pub const fn output(&self) -> Output {
        let val = (self.0 >> 0usize) & 0x1f;
        Output::from_bits(val as u8)
    }
    #[doc = "Selects the source to be connected to PLU output n."]
    #[inline(always)]
    pub const fn set_output(&mut self, val: Output) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
}
impl Default for OutputMux {
    #[inline(always)]
    fn default() -> OutputMux {
        OutputMux(0)
    }
}
impl core::fmt::Debug for OutputMux {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OutputMux")
            .field("output", &self.output())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OutputMux {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "OutputMux {{ output: {:?} }}", self.output())
    }
}
#[doc = "PLU outputs."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Outputs(pub u32);
impl Outputs {
    #[doc = "Output state."]
    #[must_use]
    #[inline(always)]
    pub const fn output_state(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Output state."]
    #[inline(always)]
    pub const fn set_output_state(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Outputs {
    #[inline(always)]
    fn default() -> Outputs {
        Outputs(0)
    }
}
impl core::fmt::Debug for Outputs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Outputs")
            .field("output_state", &self.output_state())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Outputs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Outputs {{ output_state: {=u8:?} }}",
            self.output_state()
        )
    }
}
#[doc = "Wakeup interrupt control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WakeintCtrl(pub u32);
impl WakeintCtrl {
    #[doc = "Interrupt mask."]
    #[must_use]
    #[inline(always)]
    pub const fn mask(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Interrupt mask."]
    #[inline(always)]
    pub const fn set_mask(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Filter Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn filter_mode(&self) -> FilterMode {
        let val = (self.0 >> 8usize) & 0x03;
        FilterMode::from_bits(val as u8)
    }
    #[doc = "Filter Mode."]
    #[inline(always)]
    pub const fn set_filter_mode(&mut self, val: FilterMode) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Filter clock select."]
    #[must_use]
    #[inline(always)]
    pub const fn filter_clksel(&self) -> FilterClksel {
        let val = (self.0 >> 10usize) & 0x03;
        FilterClksel::from_bits(val as u8)
    }
    #[doc = "Filter clock select."]
    #[inline(always)]
    pub const fn set_filter_clksel(&mut self, val: FilterClksel) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Latch the interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn latch_enable(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Latch the interrupt."]
    #[inline(always)]
    pub const fn set_latch_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Write to clear wakeint_latched."]
    #[must_use]
    #[inline(always)]
    pub const fn intr_clear(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Write to clear wakeint_latched."]
    #[inline(always)]
    pub const fn set_intr_clear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for WakeintCtrl {
    #[inline(always)]
    fn default() -> WakeintCtrl {
        WakeintCtrl(0)
    }
}
impl core::fmt::Debug for WakeintCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WakeintCtrl")
            .field("mask", &self.mask())
            .field("filter_mode", &self.filter_mode())
            .field("filter_clksel", &self.filter_clksel())
            .field("latch_enable", &self.latch_enable())
            .field("intr_clear", &self.intr_clear())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WakeintCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WakeintCtrl {{ mask: {=u8:?}, filter_mode: {:?}, filter_clksel: {:?}, latch_enable: {=bool:?}, intr_clear: {=bool:?} }}",
            self.mask(),
            self.filter_mode(),
            self.filter_clksel(),
            self.latch_enable(),
            self.intr_clear()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FilterClksel {
    #[doc = "Selects the 1 MHz low-power oscillator as the filter clock."]
    Fro1mhz = 0x0,
    #[doc = "Selects the 12 MHz FRO as the filter clock."]
    Fro12mhz = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl FilterClksel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FilterClksel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FilterClksel {
    #[inline(always)]
    fn from(val: u8) -> FilterClksel {
        FilterClksel::from_bits(val)
    }
}
impl From<FilterClksel> for u8 {
    #[inline(always)]
    fn from(val: FilterClksel) -> u8 {
        FilterClksel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FilterMode {
    #[doc = "Bypass mode."]
    Bypass = 0x0,
    #[doc = "Filter 1 clock period."]
    Filter1clk = 0x01,
    #[doc = "Filter 2 clock period."]
    Filter2clk = 0x02,
    #[doc = "Filter 3 clock period."]
    Filter3clk = 0x03,
}
impl FilterMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FilterMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FilterMode {
    #[inline(always)]
    fn from(val: u8) -> FilterMode {
        FilterMode::from_bits(val)
    }
}
impl From<FilterMode> for u8 {
    #[inline(always)]
    fn from(val: FilterMode) -> u8 {
        FilterMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LuTnInPx {
    #[doc = "PLU primary inputs 0."]
    PluInputs0 = 0x0,
    #[doc = "PLU primary inputs 1."]
    PluInputs1 = 0x01,
    #[doc = "PLU primary inputs 2."]
    PluInputs2 = 0x02,
    #[doc = "PLU primary inputs 3."]
    PluInputs3 = 0x03,
    #[doc = "PLU primary inputs 4."]
    PluInputs4 = 0x04,
    #[doc = "PLU primary inputs 5."]
    PluInputs5 = 0x05,
    #[doc = "Output of LUT0."]
    LutOutputs0 = 0x06,
    #[doc = "Output of LUT1."]
    LutOutputs1 = 0x07,
    #[doc = "Output of LUT2."]
    LutOutputs2 = 0x08,
    #[doc = "Output of LUT3."]
    LutOutputs3 = 0x09,
    #[doc = "Output of LUT4."]
    LutOutputs4 = 0x0a,
    #[doc = "Output of LUT5."]
    LutOutputs5 = 0x0b,
    #[doc = "Output of LUT6."]
    LutOutputs6 = 0x0c,
    #[doc = "Output of LUT7."]
    LutOutputs7 = 0x0d,
    #[doc = "Output of LUT8."]
    LutOutputs8 = 0x0e,
    #[doc = "Output of LUT9."]
    LutOutputs9 = 0x0f,
    #[doc = "Output of LUT10."]
    LutOutputs10 = 0x10,
    #[doc = "Output of LUT11."]
    LutOutputs11 = 0x11,
    #[doc = "Output of LUT12."]
    LutOutputs12 = 0x12,
    #[doc = "Output of LUT13."]
    LutOutputs13 = 0x13,
    #[doc = "Output of LUT14."]
    LutOutputs14 = 0x14,
    #[doc = "Output of LUT15."]
    LutOutputs15 = 0x15,
    #[doc = "Output of LUT16."]
    LutOutputs16 = 0x16,
    #[doc = "Output of LUT17."]
    LutOutputs17 = 0x17,
    #[doc = "Output of LUT18."]
    LutOutputs18 = 0x18,
    #[doc = "Output of LUT19."]
    LutOutputs19 = 0x19,
    #[doc = "Output of LUT20."]
    LutOutputs20 = 0x1a,
    #[doc = "Output of LUT21."]
    LutOutputs21 = 0x1b,
    #[doc = "Output of LUT22."]
    LutOutputs22 = 0x1c,
    #[doc = "Output of LUT23."]
    LutOutputs23 = 0x1d,
    #[doc = "Output of LUT24."]
    LutOutputs24 = 0x1e,
    #[doc = "Output of LUT25."]
    LutOutputs25 = 0x1f,
    #[doc = "State\\[0\\]."]
    State0 = 0x20,
    #[doc = "State\\[1\\]."]
    State1 = 0x21,
    #[doc = "State\\[2\\]."]
    State2 = 0x22,
    #[doc = "State\\[3\\]."]
    State3 = 0x23,
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
impl LuTnInPx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LuTnInPx {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LuTnInPx {
    #[inline(always)]
    fn from(val: u8) -> LuTnInPx {
        LuTnInPx::from_bits(val)
    }
}
impl From<LuTnInPx> for u8 {
    #[inline(always)]
    fn from(val: LuTnInPx) -> u8 {
        LuTnInPx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Output {
    #[doc = "LUT output 0."]
    PluOutput0 = 0x0,
    #[doc = "LUT output 1."]
    PluOutput1 = 0x01,
    #[doc = "LUT output 2."]
    PluOutput2 = 0x02,
    #[doc = "LUT output 3."]
    PluOutput3 = 0x03,
    #[doc = "LUT output 4."]
    PluOutput4 = 0x04,
    #[doc = "LUT output 5."]
    PluOutput5 = 0x05,
    #[doc = "LUT output 6."]
    PluOutput6 = 0x06,
    #[doc = "LUT output 7."]
    PluOutput7 = 0x07,
    #[doc = "LUT output 8."]
    PluOutput8 = 0x08,
    #[doc = "LUT output 9."]
    PluOutput9 = 0x09,
    #[doc = "LUT output 10."]
    PluOutput10 = 0x0a,
    #[doc = "LUT output 11."]
    PluOutput11 = 0x0b,
    #[doc = "LUT output 12."]
    PluOutput12 = 0x0c,
    #[doc = "LUT output 13."]
    PluOutput13 = 0x0d,
    #[doc = "LUT output 14."]
    PluOutput14 = 0x0e,
    #[doc = "LUT output 15."]
    PluOutput15 = 0x0f,
    #[doc = "LUT output 16."]
    PluOutput16 = 0x10,
    #[doc = "LUT output 17."]
    PluOutput17 = 0x11,
    #[doc = "LUT output 18."]
    PluOutput18 = 0x12,
    #[doc = "LUT output 19."]
    PluOutput19 = 0x13,
    #[doc = "LUT output 20."]
    PluOutput20 = 0x14,
    #[doc = "LUT output 21."]
    PluOutput21 = 0x15,
    #[doc = "LUT output 22."]
    PluOutput22 = 0x16,
    #[doc = "LUT output 23."]
    PluOutput23 = 0x17,
    #[doc = "LUT output 24."]
    PluOutput24 = 0x18,
    #[doc = "LUT output 25."]
    PluOutput25 = 0x19,
    #[doc = "State\\[0\\]."]
    State0 = 0x1a,
    #[doc = "State\\[1\\]."]
    State1 = 0x1b,
    #[doc = "State\\[2\\]."]
    State2 = 0x1c,
    #[doc = "State\\[3\\]."]
    State3 = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Output {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Output {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Output {
    #[inline(always)]
    fn from(val: u8) -> Output {
        Output::from_bits(val)
    }
}
impl From<Output> for u8 {
    #[inline(always)]
    fn from(val: Output) -> u8 {
        Output::to_bits(val)
    }
}
