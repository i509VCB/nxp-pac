#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adcen {
    #[doc = "ADC is disabled."]
    ADCEN_0 = 0x0,
    #[doc = "ADC is enabled."]
    ADCEN_1 = 0x01,
}
impl Adcen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adcen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adcen {
    #[inline(always)]
    fn from(val: u8) -> Adcen {
        Adcen::from_bits(val)
    }
}
impl From<Adcen> for u8 {
    #[inline(always)]
    fn from(val: Adcen) -> u8 {
        Adcen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Calofsi {
    #[doc = "Offset calibration and offset trimming not implemented."]
    CALOFSI_0 = 0x0,
    #[doc = "Offset calibration and offset trimming implemented."]
    CALOFSI_1 = 0x01,
}
impl Calofsi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Calofsi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Calofsi {
    #[inline(always)]
    fn from(val: u8) -> Calofsi {
        Calofsi::from_bits(val)
    }
}
impl From<Calofsi> for u8 {
    #[inline(always)]
    fn from(val: Calofsi) -> u8 {
        Calofsi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdact {
    #[doc = "No command is currently in progress."]
    CMDACT_0 = 0x0,
    #[doc = "Command 1 currently being executed."]
    CMDACT_1 = 0x01,
    #[doc = "Command 2 currently being executed."]
    CMDACT_2 = 0x02,
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
    #[doc = "Command 15 currently being executed."]
    CMDACT_15 = 0x0f,
}
impl Cmdact {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdact {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdact {
    #[inline(always)]
    fn from(val: u8) -> Cmdact {
        Cmdact::from_bits(val)
    }
}
impl From<Cmdact> for u8 {
    #[inline(always)]
    fn from(val: Cmdact) -> u8 {
        Cmdact::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmdhAvgs {
    #[doc = "Single conversion."]
    AVGS_0 = 0x0,
    #[doc = "2 conversions averaged."]
    AVGS_1 = 0x01,
    #[doc = "4 conversions averaged."]
    AVGS_2 = 0x02,
    #[doc = "8 conversions averaged."]
    AVGS_3 = 0x03,
    #[doc = "16 conversions averaged."]
    AVGS_4 = 0x04,
    #[doc = "32 conversions averaged."]
    AVGS_5 = 0x05,
    #[doc = "64 conversions averaged."]
    AVGS_6 = 0x06,
    #[doc = "128 conversions averaged."]
    AVGS_7 = 0x07,
}
impl CmdhAvgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmdhAvgs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmdhAvgs {
    #[inline(always)]
    fn from(val: u8) -> CmdhAvgs {
        CmdhAvgs::from_bits(val)
    }
}
impl From<CmdhAvgs> for u8 {
    #[inline(always)]
    fn from(val: CmdhAvgs) -> u8 {
        CmdhAvgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmdhCmpen {
    #[doc = "Compare disabled."]
    CMPEN_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Compare enabled. Store on true."]
    CMPEN_2 = 0x02,
    #[doc = "Compare enabled. Repeat channel acquisition (sample/convert/compare) until true."]
    CMPEN_3 = 0x03,
}
impl CmdhCmpen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmdhCmpen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmdhCmpen {
    #[inline(always)]
    fn from(val: u8) -> CmdhCmpen {
        CmdhCmpen::from_bits(val)
    }
}
impl From<CmdhCmpen> for u8 {
    #[inline(always)]
    fn from(val: CmdhCmpen) -> u8 {
        CmdhCmpen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmdhLoop {
    #[doc = "Looping not enabled. Command executes 1 time."]
    LOOP_0 = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    LOOP_1 = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    LOOP_2 = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    LOOP_15 = 0x0f,
}
impl CmdhLoop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmdhLoop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmdhLoop {
    #[inline(always)]
    fn from(val: u8) -> CmdhLoop {
        CmdhLoop::from_bits(val)
    }
}
impl From<CmdhLoop> for u8 {
    #[inline(always)]
    fn from(val: CmdhLoop) -> u8 {
        CmdhLoop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmdhLwi {
    #[doc = "Auto channel increment disabled"]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled"]
    LWI_1 = 0x01,
}
impl CmdhLwi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmdhLwi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmdhLwi {
    #[inline(always)]
    fn from(val: u8) -> CmdhLwi {
        CmdhLwi::from_bits(val)
    }
}
impl From<CmdhLwi> for u8 {
    #[inline(always)]
    fn from(val: CmdhLwi) -> u8 {
        CmdhLwi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmdhNext {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl CmdhNext {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmdhNext {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmdhNext {
    #[inline(always)]
    fn from(val: u8) -> CmdhNext {
        CmdhNext::from_bits(val)
    }
}
impl From<CmdhNext> for u8 {
    #[inline(always)]
    fn from(val: CmdhNext) -> u8 {
        CmdhNext::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmdhSts {
    #[doc = "Minimum sample time of 3 ADCK cycles."]
    STS_0 = 0x0,
    #[doc = "3 + 2^1 ADCK cycles; 5 ADCK cycles total sample time."]
    STS_1 = 0x01,
    #[doc = "3 + 2^2 ADCK cycles; 7 ADCK cycles total sample time."]
    STS_2 = 0x02,
    #[doc = "3 + 2^3 ADCK cycles; 11 ADCK cycles total sample time."]
    STS_3 = 0x03,
    #[doc = "3 + 2^4 ADCK cycles; 19 ADCK cycles total sample time."]
    STS_4 = 0x04,
    #[doc = "3 + 2^5 ADCK cycles; 35 ADCK cycles total sample time."]
    STS_5 = 0x05,
    #[doc = "3 + 2^6 ADCK cycles; 67 ADCK cycles total sample time."]
    STS_6 = 0x06,
    #[doc = "3 + 2^7 ADCK cycles; 131 ADCK cycles total sample time."]
    STS_7 = 0x07,
}
impl CmdhSts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmdhSts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmdhSts {
    #[inline(always)]
    fn from(val: u8) -> CmdhSts {
        CmdhSts::from_bits(val)
    }
}
impl From<CmdhSts> for u8 {
    #[inline(always)]
    fn from(val: CmdhSts) -> u8 {
        CmdhSts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmdlAbsel {
    #[doc = "When DIFF=0b0, the associated A-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnA-CHnB)."]
    ABSEL_0 = 0x0,
    #[doc = "When DIFF=0b0, the associated B-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnB-CHnA)."]
    ABSEL_1 = 0x01,
}
impl CmdlAbsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmdlAbsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmdlAbsel {
    #[inline(always)]
    fn from(val: u8) -> CmdlAbsel {
        CmdlAbsel::from_bits(val)
    }
}
impl From<CmdlAbsel> for u8 {
    #[inline(always)]
    fn from(val: CmdlAbsel) -> u8 {
        CmdlAbsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmdlAdch {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    ADCH_0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    ADCH_1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    ADCH_2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    ADCH_3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_9 = 0x09,
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
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    ADCH_30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    ADCH_31 = 0x1f,
}
impl CmdlAdch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmdlAdch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmdlAdch {
    #[inline(always)]
    fn from(val: u8) -> CmdlAdch {
        CmdlAdch::from_bits(val)
    }
}
impl From<CmdlAdch> for u8 {
    #[inline(always)]
    fn from(val: CmdlAdch) -> u8 {
        CmdlAdch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmdlCscale {
    #[doc = "Scale selected analog channel (Factor of 30/64)"]
    CSCALE_0 = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    CSCALE_1 = 0x01,
}
impl CmdlCscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmdlCscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmdlCscale {
    #[inline(always)]
    fn from(val: u8) -> CmdlCscale {
        CmdlCscale::from_bits(val)
    }
}
impl From<CmdlCscale> for u8 {
    #[inline(always)]
    fn from(val: CmdlCscale) -> u8 {
        CmdlCscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmdlDiff {
    #[doc = "Single-ended mode."]
    DIFF_0 = 0x0,
    #[doc = "Differential mode."]
    DIFF_1 = 0x01,
}
impl CmdlDiff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmdlDiff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmdlDiff {
    #[inline(always)]
    fn from(val: u8) -> CmdlDiff {
        CmdlDiff::from_bits(val)
    }
}
impl From<CmdlDiff> for u8 {
    #[inline(always)]
    fn from(val: CmdlDiff) -> u8 {
        CmdlDiff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdsrc {
    #[doc = "Not a valid value CMDSRC value for a dataword in RESFIFO. 0x0 is only found in initial FIFO state prior to an ADC conversion result dataword being stored to a RESFIFO buffer."]
    CMDSRC_0 = 0x0,
    #[doc = "CMD1 buffer used as control settings for this conversion."]
    CMDSRC_1 = 0x01,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CMDSRC_2 = 0x02,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CMDSRC_3 = 0x03,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CMDSRC_4 = 0x04,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CMDSRC_5 = 0x05,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CMDSRC_6 = 0x06,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CMDSRC_7 = 0x07,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CMDSRC_8 = 0x08,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CMDSRC_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "CMD15 buffer used as control settings for this conversion."]
    CMDSRC_15 = 0x0f,
}
impl Cmdsrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdsrc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdsrc {
    #[inline(always)]
    fn from(val: u8) -> Cmdsrc {
        Cmdsrc::from_bits(val)
    }
}
impl From<Cmdsrc> for u8 {
    #[inline(always)]
    fn from(val: Cmdsrc) -> u8 {
        Cmdsrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csw {
    #[doc = "Channel scaling not supported."]
    CSW_0 = 0x0,
    #[doc = "Channel scaling supported. 1-bit CSCALE control field."]
    CSW_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "Channel scaling supported. 6-bit CSCALE control field."]
    CSW_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Csw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csw {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csw {
    #[inline(always)]
    fn from(val: u8) -> Csw {
        Csw::from_bits(val)
    }
}
impl From<Csw> for u8 {
    #[inline(always)]
    fn from(val: Csw) -> u8 {
        Csw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Diffen {
    #[doc = "Differential operation not supported."]
    DIFFEN_0 = 0x0,
    #[doc = "Differential operation supported. CMDLa\\[DIFF\\] and CMDLa\\[ABSEL\\] control fields implemented."]
    DIFFEN_1 = 0x01,
}
impl Diffen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Diffen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Diffen {
    #[inline(always)]
    fn from(val: u8) -> Diffen {
        Diffen::from_bits(val)
    }
}
impl From<Diffen> for u8 {
    #[inline(always)]
    fn from(val: Diffen) -> u8 {
        Diffen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dozen {
    #[doc = "ADC is enabled in Doze mode."]
    DOZEN_0 = 0x0,
    #[doc = "ADC is disabled in Doze mode."]
    DOZEN_1 = 0x01,
}
impl Dozen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dozen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dozen {
    #[inline(always)]
    fn from(val: u8) -> Dozen {
        Dozen::from_bits(val)
    }
}
impl From<Dozen> for u8 {
    #[inline(always)]
    fn from(val: Dozen) -> u8 {
        Dozen::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Fifosize(u8);
impl Fifosize {
    #[doc = "Result FIFO depth = 1 dataword."]
    pub const FIFOSIZE_1: Self = Self(0x01);
    #[doc = "Result FIFO depth = 4 datawords."]
    pub const FIFOSIZE_4: Self = Self(0x04);
    #[doc = "Result FIFO depth = 8 datawords."]
    pub const FIFOSIZE_8: Self = Self(0x08);
    #[doc = "Result FIFO depth = 16 datawords."]
    pub const FIFOSIZE_16: Self = Self(0x10);
    #[doc = "Result FIFO depth = 32 datawords."]
    pub const FIFOSIZE_32: Self = Self(0x20);
    #[doc = "Result FIFO depth = 64 datawords."]
    pub const FIFOSIZE_64: Self = Self(0x40);
}
impl Fifosize {
    pub const fn from_bits(val: u8) -> Fifosize {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Fifosize {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("FIFOSIZE_1"),
            0x04 => f.write_str("FIFOSIZE_4"),
            0x08 => f.write_str("FIFOSIZE_8"),
            0x10 => f.write_str("FIFOSIZE_16"),
            0x20 => f.write_str("FIFOSIZE_32"),
            0x40 => f.write_str("FIFOSIZE_64"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fifosize {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "FIFOSIZE_1"),
            0x04 => defmt::write!(f, "FIFOSIZE_4"),
            0x08 => defmt::write!(f, "FIFOSIZE_8"),
            0x10 => defmt::write!(f, "FIFOSIZE_16"),
            0x20 => defmt::write!(f, "FIFOSIZE_32"),
            0x40 => defmt::write!(f, "FIFOSIZE_64"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Fifosize {
    #[inline(always)]
    fn from(val: u8) -> Fifosize {
        Fifosize::from_bits(val)
    }
}
impl From<Fifosize> for u8 {
    #[inline(always)]
    fn from(val: Fifosize) -> u8 {
        Fifosize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fof {
    #[doc = "No result FIFO overflow has occurred since the last time the flag was cleared."]
    FOF_0 = 0x0,
    #[doc = "At least one result FIFO overflow has occurred since the last time the flag was cleared."]
    FOF_1 = 0x01,
}
impl Fof {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fof {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fof {
    #[inline(always)]
    fn from(val: u8) -> Fof {
        Fof::from_bits(val)
    }
}
impl From<Fof> for u8 {
    #[inline(always)]
    fn from(val: Fof) -> u8 {
        Fof::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fofie {
    #[doc = "FIFO overflow interrupts are not enabled."]
    FOFIE_0 = 0x0,
    #[doc = "FIFO overflow interrupts are enabled."]
    FOFIE_1 = 0x01,
}
impl Fofie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fofie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fofie {
    #[inline(always)]
    fn from(val: u8) -> Fofie {
        Fofie::from_bits(val)
    }
}
impl From<Fofie> for u8 {
    #[inline(always)]
    fn from(val: Fofie) -> u8 {
        Fofie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fwmde {
    #[doc = "DMA request disabled."]
    FWMDE_0 = 0x0,
    #[doc = "DMA request enabled."]
    FWMDE_1 = 0x01,
}
impl Fwmde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fwmde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fwmde {
    #[inline(always)]
    fn from(val: u8) -> Fwmde {
        Fwmde::from_bits(val)
    }
}
impl From<Fwmde> for u8 {
    #[inline(always)]
    fn from(val: Fwmde) -> u8 {
        Fwmde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fwmie {
    #[doc = "FIFO watermark interrupts are not enabled."]
    FWMIE_0 = 0x0,
    #[doc = "FIFO watermark interrupts are enabled."]
    FWMIE_1 = 0x01,
}
impl Fwmie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fwmie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fwmie {
    #[inline(always)]
    fn from(val: u8) -> Fwmie {
        Fwmie::from_bits(val)
    }
}
impl From<Fwmie> for u8 {
    #[inline(always)]
    fn from(val: Fwmie) -> u8 {
        Fwmie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hten {
    #[doc = "Hardware trigger source disabled"]
    HTEN_0 = 0x0,
    #[doc = "Hardware trigger source enabled"]
    HTEN_1 = 0x01,
}
impl Hten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hten {
    #[inline(always)]
    fn from(val: u8) -> Hten {
        Hten::from_bits(val)
    }
}
impl From<Hten> for u8 {
    #[inline(always)]
    fn from(val: Hten) -> u8 {
        Hten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iadcki {
    #[doc = "Internal clock source not implemented."]
    IADCKI_0 = 0x0,
    #[doc = "Internal clock source (and CFG\\[ADCKEN\\]) implemented."]
    IADCKI_1 = 0x01,
}
impl Iadcki {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iadcki {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iadcki {
    #[inline(always)]
    fn from(val: u8) -> Iadcki {
        Iadcki::from_bits(val)
    }
}
impl From<Iadcki> for u8 {
    #[inline(always)]
    fn from(val: Iadcki) -> u8 {
        Iadcki::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Loopcnt {
    #[doc = "Result is from initial conversion in command."]
    LOOPCNT_0 = 0x0,
    #[doc = "Result is from second conversion in command."]
    LOOPCNT_1 = 0x01,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    LOOPCNT_2 = 0x02,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    LOOPCNT_3 = 0x03,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    LOOPCNT_4 = 0x04,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    LOOPCNT_5 = 0x05,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    LOOPCNT_6 = 0x06,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    LOOPCNT_7 = 0x07,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    LOOPCNT_8 = 0x08,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    LOOPCNT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Result is from 16th conversion in command."]
    LOOPCNT_15 = 0x0f,
}
impl Loopcnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Loopcnt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Loopcnt {
    #[inline(always)]
    fn from(val: u8) -> Loopcnt {
        Loopcnt::from_bits(val)
    }
}
impl From<Loopcnt> for u8 {
    #[inline(always)]
    fn from(val: Loopcnt) -> u8 {
        Loopcnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mvi {
    #[doc = "Single voltage reference high (VREFH) input supported."]
    MVI_0 = 0x0,
    #[doc = "Multiple voltage reference high (VREFH) inputs supported."]
    MVI_1 = 0x01,
}
impl Mvi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mvi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mvi {
    #[inline(always)]
    fn from(val: u8) -> Mvi {
        Mvi::from_bits(val)
    }
}
impl From<Mvi> for u8 {
    #[inline(always)]
    fn from(val: Mvi) -> u8 {
        Mvi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pauseen {
    #[doc = "Pause operation disabled"]
    PAUSEEN_0 = 0x0,
    #[doc = "Pause operation enabled"]
    PAUSEEN_1 = 0x01,
}
impl Pauseen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pauseen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pauseen {
    #[inline(always)]
    fn from(val: u8) -> Pauseen {
        Pauseen::from_bits(val)
    }
}
impl From<Pauseen> for u8 {
    #[inline(always)]
    fn from(val: Pauseen) -> u8 {
        Pauseen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwren {
    #[doc = "ADC analog circuits are only enabled while conversions are active. Performance is affected due to analog startup delays."]
    PWREN_0 = 0x0,
    #[doc = "ADC analog circuits are pre-enabled and ready to execute conversions without startup delays (at the cost of higher DC current consumption). When PWREN is set, the power up delay is enforced such that any detected trigger does not begin ADC operation until the power up delay time has passed."]
    PWREN_1 = 0x01,
}
impl Pwren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwren {
    #[inline(always)]
    fn from(val: u8) -> Pwren {
        Pwren::from_bits(val)
    }
}
impl From<Pwren> for u8 {
    #[inline(always)]
    fn from(val: Pwren) -> u8 {
        Pwren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwrsel {
    #[doc = "Lowest power setting."]
    PWRSEL_0 = 0x0,
    #[doc = "Next lowest power setting."]
    PWRSEL_1 = 0x01,
    #[doc = "...."]
    PWRSEL_2 = 0x02,
    #[doc = "Highest power setting."]
    PWRSEL_3 = 0x03,
}
impl Pwrsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwrsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwrsel {
    #[inline(always)]
    fn from(val: u8) -> Pwrsel {
        Pwrsel::from_bits(val)
    }
}
impl From<Pwrsel> for u8 {
    #[inline(always)]
    fn from(val: Pwrsel) -> u8 {
        Pwrsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rdy {
    #[doc = "Result FIFO data level not above watermark level."]
    RDY_0 = 0x0,
    #[doc = "Result FIFO holding data above watermark level."]
    RDY_1 = 0x01,
}
impl Rdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rdy {
    #[inline(always)]
    fn from(val: u8) -> Rdy {
        Rdy::from_bits(val)
    }
}
impl From<Rdy> for u8 {
    #[inline(always)]
    fn from(val: Rdy) -> u8 {
        Rdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Refsel {
    #[doc = "(Default) Option 1 setting."]
    REFSEL_0 = 0x0,
    #[doc = "Option 2 setting."]
    REFSEL_1 = 0x01,
    #[doc = "Option 3 setting."]
    REFSEL_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Refsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Refsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Refsel {
    #[inline(always)]
    fn from(val: u8) -> Refsel {
        Refsel::from_bits(val)
    }
}
impl From<Refsel> for u8 {
    #[inline(always)]
    fn from(val: Refsel) -> u8 {
        Refsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Res {
    #[doc = "Up to 13-bit differential/12-bit single ended resolution supported."]
    RES_0 = 0x0,
    #[doc = "Up to 16-bit differential/15-bit single ended resolution supported."]
    RES_1 = 0x01,
}
impl Res {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Res {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Res {
    #[inline(always)]
    fn from(val: u8) -> Res {
        Res::from_bits(val)
    }
}
impl From<Res> for u8 {
    #[inline(always)]
    fn from(val: Res) -> u8 {
        Res::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rst {
    #[doc = "ADC logic is not reset."]
    RST_0 = 0x0,
    #[doc = "ADC logic is reset."]
    RST_1 = 0x01,
}
impl Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rst {
    #[inline(always)]
    fn from(val: u8) -> Rst {
        Rst::from_bits(val)
    }
}
impl From<Rst> for u8 {
    #[inline(always)]
    fn from(val: Rst) -> u8 {
        Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rstfifo {
    #[doc = "No effect."]
    RSTFIFO_0 = 0x0,
    #[doc = "FIFO is reset."]
    RSTFIFO_1 = 0x01,
}
impl Rstfifo {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rstfifo {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rstfifo {
    #[inline(always)]
    fn from(val: u8) -> Rstfifo {
        Rstfifo::from_bits(val)
    }
}
impl From<Rstfifo> for u8 {
    #[inline(always)]
    fn from(val: Rstfifo) -> u8 {
        Rstfifo::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt0 {
    #[doc = "No trigger 0 event generated."]
    SWT0_0 = 0x0,
    #[doc = "Trigger 0 event generated."]
    SWT0_1 = 0x01,
}
impl Swt0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt0 {
    #[inline(always)]
    fn from(val: u8) -> Swt0 {
        Swt0::from_bits(val)
    }
}
impl From<Swt0> for u8 {
    #[inline(always)]
    fn from(val: Swt0) -> u8 {
        Swt0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt1 {
    #[doc = "No trigger 1 event generated."]
    SWT1_0 = 0x0,
    #[doc = "Trigger 1 event generated."]
    SWT1_1 = 0x01,
}
impl Swt1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt1 {
    #[inline(always)]
    fn from(val: u8) -> Swt1 {
        Swt1::from_bits(val)
    }
}
impl From<Swt1> for u8 {
    #[inline(always)]
    fn from(val: Swt1) -> u8 {
        Swt1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt10 {
    #[doc = "No trigger 10 event generated."]
    SWT10_0 = 0x0,
    #[doc = "Trigger 10 event generated."]
    SWT10_1 = 0x01,
}
impl Swt10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt10 {
    #[inline(always)]
    fn from(val: u8) -> Swt10 {
        Swt10::from_bits(val)
    }
}
impl From<Swt10> for u8 {
    #[inline(always)]
    fn from(val: Swt10) -> u8 {
        Swt10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt11 {
    #[doc = "No trigger 11 event generated."]
    SWT11_0 = 0x0,
    #[doc = "Trigger 11 event generated."]
    SWT11_1 = 0x01,
}
impl Swt11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt11 {
    #[inline(always)]
    fn from(val: u8) -> Swt11 {
        Swt11::from_bits(val)
    }
}
impl From<Swt11> for u8 {
    #[inline(always)]
    fn from(val: Swt11) -> u8 {
        Swt11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt12 {
    #[doc = "No trigger 12 event generated."]
    SWT12_0 = 0x0,
    #[doc = "Trigger 12 event generated."]
    SWT12_1 = 0x01,
}
impl Swt12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt12 {
    #[inline(always)]
    fn from(val: u8) -> Swt12 {
        Swt12::from_bits(val)
    }
}
impl From<Swt12> for u8 {
    #[inline(always)]
    fn from(val: Swt12) -> u8 {
        Swt12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt13 {
    #[doc = "No trigger 13 event generated."]
    SWT13_0 = 0x0,
    #[doc = "Trigger 13 event generated."]
    SWT13_1 = 0x01,
}
impl Swt13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt13 {
    #[inline(always)]
    fn from(val: u8) -> Swt13 {
        Swt13::from_bits(val)
    }
}
impl From<Swt13> for u8 {
    #[inline(always)]
    fn from(val: Swt13) -> u8 {
        Swt13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt14 {
    #[doc = "No trigger 14 event generated."]
    SWT14_0 = 0x0,
    #[doc = "Trigger 14 event generated."]
    SWT14_1 = 0x01,
}
impl Swt14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt14 {
    #[inline(always)]
    fn from(val: u8) -> Swt14 {
        Swt14::from_bits(val)
    }
}
impl From<Swt14> for u8 {
    #[inline(always)]
    fn from(val: Swt14) -> u8 {
        Swt14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt15 {
    #[doc = "No trigger 15 event generated."]
    SWT15_0 = 0x0,
    #[doc = "Trigger 15 event generated."]
    SWT15_1 = 0x01,
}
impl Swt15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt15 {
    #[inline(always)]
    fn from(val: u8) -> Swt15 {
        Swt15::from_bits(val)
    }
}
impl From<Swt15> for u8 {
    #[inline(always)]
    fn from(val: Swt15) -> u8 {
        Swt15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt2 {
    #[doc = "No trigger 2 event generated."]
    SWT2_0 = 0x0,
    #[doc = "Trigger 2 event generated."]
    SWT2_1 = 0x01,
}
impl Swt2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt2 {
    #[inline(always)]
    fn from(val: u8) -> Swt2 {
        Swt2::from_bits(val)
    }
}
impl From<Swt2> for u8 {
    #[inline(always)]
    fn from(val: Swt2) -> u8 {
        Swt2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt3 {
    #[doc = "No trigger 3 event generated."]
    SWT3_0 = 0x0,
    #[doc = "Trigger 3 event generated."]
    SWT3_1 = 0x01,
}
impl Swt3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt3 {
    #[inline(always)]
    fn from(val: u8) -> Swt3 {
        Swt3::from_bits(val)
    }
}
impl From<Swt3> for u8 {
    #[inline(always)]
    fn from(val: Swt3) -> u8 {
        Swt3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt4 {
    #[doc = "No trigger 4 event generated."]
    SWT4_0 = 0x0,
    #[doc = "Trigger 4 event generated."]
    SWT4_1 = 0x01,
}
impl Swt4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt4 {
    #[inline(always)]
    fn from(val: u8) -> Swt4 {
        Swt4::from_bits(val)
    }
}
impl From<Swt4> for u8 {
    #[inline(always)]
    fn from(val: Swt4) -> u8 {
        Swt4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt5 {
    #[doc = "No trigger 5 event generated."]
    SWT5_0 = 0x0,
    #[doc = "Trigger 5 event generated."]
    SWT5_1 = 0x01,
}
impl Swt5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt5 {
    #[inline(always)]
    fn from(val: u8) -> Swt5 {
        Swt5::from_bits(val)
    }
}
impl From<Swt5> for u8 {
    #[inline(always)]
    fn from(val: Swt5) -> u8 {
        Swt5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt6 {
    #[doc = "No trigger 6 event generated."]
    SWT6_0 = 0x0,
    #[doc = "Trigger 6 event generated."]
    SWT6_1 = 0x01,
}
impl Swt6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt6 {
    #[inline(always)]
    fn from(val: u8) -> Swt6 {
        Swt6::from_bits(val)
    }
}
impl From<Swt6> for u8 {
    #[inline(always)]
    fn from(val: Swt6) -> u8 {
        Swt6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt7 {
    #[doc = "No trigger 7 event generated."]
    SWT7_0 = 0x0,
    #[doc = "Trigger 7 event generated."]
    SWT7_1 = 0x01,
}
impl Swt7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt7 {
    #[inline(always)]
    fn from(val: u8) -> Swt7 {
        Swt7::from_bits(val)
    }
}
impl From<Swt7> for u8 {
    #[inline(always)]
    fn from(val: Swt7) -> u8 {
        Swt7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt8 {
    #[doc = "No trigger 8 event generated."]
    SWT8_0 = 0x0,
    #[doc = "Trigger 8 event generated."]
    SWT8_1 = 0x01,
}
impl Swt8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt8 {
    #[inline(always)]
    fn from(val: u8) -> Swt8 {
        Swt8::from_bits(val)
    }
}
impl From<Swt8> for u8 {
    #[inline(always)]
    fn from(val: Swt8) -> u8 {
        Swt8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swt9 {
    #[doc = "No trigger 9 event generated."]
    SWT9_0 = 0x0,
    #[doc = "Trigger 9 event generated."]
    SWT9_1 = 0x01,
}
impl Swt9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt9 {
    #[inline(always)]
    fn from(val: u8) -> Swt9 {
        Swt9::from_bits(val)
    }
}
impl From<Swt9> for u8 {
    #[inline(always)]
    fn from(val: Swt9) -> u8 {
        Swt9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcmd {
    #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
    TCMD_0 = 0x0,
    #[doc = "CMD1 is executed"]
    TCMD_1 = 0x01,
    #[doc = "Corresponding CMD is executed"]
    TCMD_2 = 0x02,
    #[doc = "Corresponding CMD is executed"]
    TCMD_3 = 0x03,
    #[doc = "Corresponding CMD is executed"]
    TCMD_4 = 0x04,
    #[doc = "Corresponding CMD is executed"]
    TCMD_5 = 0x05,
    #[doc = "Corresponding CMD is executed"]
    TCMD_6 = 0x06,
    #[doc = "Corresponding CMD is executed"]
    TCMD_7 = 0x07,
    #[doc = "Corresponding CMD is executed"]
    TCMD_8 = 0x08,
    #[doc = "Corresponding CMD is executed"]
    TCMD_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "CMD15 is executed"]
    TCMD_15 = 0x0f,
}
impl Tcmd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcmd {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcmd {
    #[inline(always)]
    fn from(val: u8) -> Tcmd {
        Tcmd::from_bits(val)
    }
}
impl From<Tcmd> for u8 {
    #[inline(always)]
    fn from(val: Tcmd) -> u8 {
        Tcmd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpri {
    #[doc = "Set to highest priority, Level 1"]
    TPRI_0 = 0x0,
    #[doc = "Set to corresponding priority level"]
    TPRI_1 = 0x01,
    #[doc = "Set to corresponding priority level"]
    TPRI_2 = 0x02,
    #[doc = "Set to corresponding priority level"]
    TPRI_3 = 0x03,
    #[doc = "Set to corresponding priority level"]
    TPRI_4 = 0x04,
    #[doc = "Set to corresponding priority level"]
    TPRI_5 = 0x05,
    #[doc = "Set to corresponding priority level"]
    TPRI_6 = 0x06,
    #[doc = "Set to corresponding priority level"]
    TPRI_7 = 0x07,
    #[doc = "Set to corresponding priority level"]
    TPRI_8 = 0x08,
    #[doc = "Set to corresponding priority level"]
    TPRI_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Set to lowest priority, Level 16"]
    TPRI_15 = 0x0f,
}
impl Tpri {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpri {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpri {
    #[inline(always)]
    fn from(val: u8) -> Tpri {
        Tpri::from_bits(val)
    }
}
impl From<Tpri> for u8 {
    #[inline(always)]
    fn from(val: Tpri) -> u8 {
        Tpri::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tprictrl {
    #[doc = "If a higher priority trigger is detected during command processing, the current conversion is aborted and the new command specified by the trigger is started."]
    TPRICTRL_0 = 0x0,
    #[doc = "If a higher priority trigger is received during command processing, the current conversion is completed (including averaging iterations and compare function if enabled) and stored to the RESFIFO before the higher priority trigger/command is initiated."]
    TPRICTRL_1 = 0x01,
}
impl Tprictrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tprictrl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tprictrl {
    #[inline(always)]
    fn from(val: u8) -> Tprictrl {
        Tprictrl::from_bits(val)
    }
}
impl From<Tprictrl> for u8 {
    #[inline(always)]
    fn from(val: Tprictrl) -> u8 {
        Tprictrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trgact {
    #[doc = "Command (sequence) associated with Trigger 0 currently being executed."]
    TRGACT_0 = 0x0,
    #[doc = "Command (sequence) associated with Trigger 1 currently being executed."]
    TRGACT_1 = 0x01,
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
    #[doc = "Command (sequence) associated with Trigger 15 currently being executed."]
    TRGACT_15 = 0x0f,
}
impl Trgact {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trgact {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trgact {
    #[inline(always)]
    fn from(val: u8) -> Trgact {
        Trgact::from_bits(val)
    }
}
impl From<Trgact> for u8 {
    #[inline(always)]
    fn from(val: Trgact) -> u8 {
        Trgact::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsrc {
    #[doc = "Trigger source 0 initiated this conversion."]
    TSRC_0 = 0x0,
    #[doc = "Trigger source 1 initiated this conversion."]
    TSRC_1 = 0x01,
    #[doc = "Corresponding trigger source initiated this conversion."]
    TSRC_2 = 0x02,
    #[doc = "Corresponding trigger source initiated this conversion."]
    TSRC_3 = 0x03,
    #[doc = "Corresponding trigger source initiated this conversion."]
    TSRC_4 = 0x04,
    #[doc = "Corresponding trigger source initiated this conversion."]
    TSRC_5 = 0x05,
    #[doc = "Corresponding trigger source initiated this conversion."]
    TSRC_6 = 0x06,
    #[doc = "Corresponding trigger source initiated this conversion."]
    TSRC_7 = 0x07,
    #[doc = "Corresponding trigger source initiated this conversion."]
    TSRC_8 = 0x08,
    #[doc = "Corresponding trigger source initiated this conversion."]
    TSRC_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Trigger source 15 initiated this conversion."]
    TSRC_15 = 0x0f,
}
impl Tsrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsrc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsrc {
    #[inline(always)]
    fn from(val: u8) -> Tsrc {
        Tsrc::from_bits(val)
    }
}
impl From<Tsrc> for u8 {
    #[inline(always)]
    fn from(val: Tsrc) -> u8 {
        Tsrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Valid {
    #[doc = "FIFO is empty. Discard any read from RESFIFO."]
    VALID_0 = 0x0,
    #[doc = "FIFO record read from RESFIFO is valid."]
    VALID_1 = 0x01,
}
impl Valid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Valid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Valid {
    #[inline(always)]
    fn from(val: u8) -> Valid {
        Valid::from_bits(val)
    }
}
impl From<Valid> for u8 {
    #[inline(always)]
    fn from(val: Valid) -> u8 {
        Valid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vr1rngi {
    #[doc = "Range control not required. CFG\\[VREF1RNG\\] is not implemented."]
    VR1RNGI_0 = 0x0,
    #[doc = "Range control required. CFG\\[VREF1RNG\\] is implemented."]
    VR1RNGI_1 = 0x01,
}
impl Vr1rngi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vr1rngi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vr1rngi {
    #[inline(always)]
    fn from(val: u8) -> Vr1rngi {
        Vr1rngi::from_bits(val)
    }
}
impl From<Vr1rngi> for u8 {
    #[inline(always)]
    fn from(val: Vr1rngi) -> u8 {
        Vr1rngi::to_bits(val)
    }
}
