#[doc = "Frequency Measurement (in Read mode)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FreqmectrlR(pub u32);
impl FreqmectrlR {
    #[doc = "Result"]
    #[must_use]
    #[inline(always)]
    pub const fn result(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "Result"]
    #[inline(always)]
    pub const fn set_result(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
    }
    #[doc = "Measure in Progress"]
    #[must_use]
    #[inline(always)]
    pub const fn measure_in_progress(&self) -> super::vals::FreqmectrlRMeasureInProgress {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::FreqmectrlRMeasureInProgress::from_bits(val as u8)
    }
    #[doc = "Measure in Progress"]
    #[inline(always)]
    pub const fn set_measure_in_progress(
        &mut self,
        val: super::vals::FreqmectrlRMeasureInProgress,
    ) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for FreqmectrlR {
    #[inline(always)]
    fn default() -> FreqmectrlR {
        FreqmectrlR(0)
    }
}
impl core::fmt::Debug for FreqmectrlR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FreqmectrlR")
            .field("result", &self.result())
            .field("measure_in_progress", &self.measure_in_progress())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FreqmectrlR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FreqmectrlR {{ result: {=u32:?}, measure_in_progress: {:?} }}",
            self.result(),
            self.measure_in_progress()
        )
    }
}
#[doc = "Freqeuncy Measurement (in Write mode)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FreqmectrlW(pub u32);
impl FreqmectrlW {
    #[doc = "Reference Clock Scaling Factor"]
    #[must_use]
    #[inline(always)]
    pub const fn ref_scale(&self) -> super::vals::RefScale {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::RefScale::from_bits(val as u8)
    }
    #[doc = "Reference Clock Scaling Factor"]
    #[inline(always)]
    pub const fn set_ref_scale(&mut self, val: super::vals::RefScale) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Pulse Width Measurement mode select"]
    #[must_use]
    #[inline(always)]
    pub const fn pulse_mode(&self) -> super::vals::PulseMode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PulseMode::from_bits(val as u8)
    }
    #[doc = "Pulse Width Measurement mode select"]
    #[inline(always)]
    pub const fn set_pulse_mode(&mut self, val: super::vals::PulseMode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Pulse Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn pulse_pol(&self) -> super::vals::PulsePol {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PulsePol::from_bits(val as u8)
    }
    #[doc = "Pulse Polarity"]
    #[inline(always)]
    pub const fn set_pulse_pol(&mut self, val: super::vals::PulsePol) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Measure in Progress"]
    #[must_use]
    #[inline(always)]
    pub const fn measure_in_progress(&self) -> super::vals::FreqmectrlWMeasureInProgress {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::FreqmectrlWMeasureInProgress::from_bits(val as u8)
    }
    #[doc = "Measure in Progress"]
    #[inline(always)]
    pub const fn set_measure_in_progress(
        &mut self,
        val: super::vals::FreqmectrlWMeasureInProgress,
    ) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for FreqmectrlW {
    #[inline(always)]
    fn default() -> FreqmectrlW {
        FreqmectrlW(0)
    }
}
impl core::fmt::Debug for FreqmectrlW {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FreqmectrlW")
            .field("ref_scale", &self.ref_scale())
            .field("pulse_mode", &self.pulse_mode())
            .field("pulse_pol", &self.pulse_pol())
            .field("measure_in_progress", &self.measure_in_progress())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FreqmectrlW {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FreqmectrlW {{ ref_scale: {:?}, pulse_mode: {:?}, pulse_pol: {:?}, measure_in_progress: {:?} }}",
            self.ref_scale(),
            self.pulse_mode(),
            self.pulse_pol(),
            self.measure_in_progress()
        )
    }
}
