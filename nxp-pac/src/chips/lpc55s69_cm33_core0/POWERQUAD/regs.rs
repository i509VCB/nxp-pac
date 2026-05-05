#[doc = "PowerQuad Control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CONTROL(pub u32);
impl CONTROL {
    #[doc = "opcode specific to decode_machine."]
    #[must_use]
    #[inline(always)]
    pub const fn decode_opcode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "opcode specific to decode_machine."]
    #[inline(always)]
    pub const fn set_decode_opcode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "0 : Coprocessor , 1 : matrix , 2 : fft , 3 : fir , 4 : stat , 5 : cordic , 6 -15 : NA."]
    #[must_use]
    #[inline(always)]
    pub const fn decode_machine(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "0 : Coprocessor , 1 : matrix , 2 : fft , 3 : fir , 4 : stat , 5 : cordic , 6 -15 : NA."]
    #[inline(always)]
    pub const fn set_decode_machine(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Instruction busy signal when high indicates processing is on."]
    #[must_use]
    #[inline(always)]
    pub const fn inst_busy(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Instruction busy signal when high indicates processing is on."]
    #[inline(always)]
    pub const fn set_inst_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CONTROL {
    #[inline(always)]
    fn default() -> CONTROL {
        CONTROL(0)
    }
}
impl core::fmt::Debug for CONTROL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONTROL")
            .field("decode_opcode", &self.decode_opcode())
            .field("decode_machine", &self.decode_machine())
            .field("inst_busy", &self.inst_busy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CONTROL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CONTROL {{ decode_opcode: {=u8:?}, decode_machine: {=u8:?}, inst_busy: {=bool:?} }}",
            self.decode_opcode(),
            self.decode_machine(),
            self.inst_busy()
        )
    }
}
#[doc = "Cordic input X register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CORDIC_X(pub u32);
impl CORDIC_X {
    #[doc = "Cordic input x."]
    #[must_use]
    #[inline(always)]
    pub const fn cordic_x(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Cordic input x."]
    #[inline(always)]
    pub const fn set_cordic_x(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CORDIC_X {
    #[inline(always)]
    fn default() -> CORDIC_X {
        CORDIC_X(0)
    }
}
impl core::fmt::Debug for CORDIC_X {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORDIC_X")
            .field("cordic_x", &self.cordic_x())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CORDIC_X {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CORDIC_X {{ cordic_x: {=u32:?} }}", self.cordic_x())
    }
}
#[doc = "Cordic input Y register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CORDIC_Y(pub u32);
impl CORDIC_Y {
    #[doc = "Cordic input y."]
    #[must_use]
    #[inline(always)]
    pub const fn cordic_y(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Cordic input y."]
    #[inline(always)]
    pub const fn set_cordic_y(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CORDIC_Y {
    #[inline(always)]
    fn default() -> CORDIC_Y {
        CORDIC_Y(0)
    }
}
impl core::fmt::Debug for CORDIC_Y {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORDIC_Y")
            .field("cordic_y", &self.cordic_y())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CORDIC_Y {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CORDIC_Y {{ cordic_y: {=u32:?} }}", self.cordic_y())
    }
}
#[doc = "Cordic input Z register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CORDIC_Z(pub u32);
impl CORDIC_Z {
    #[doc = "Cordic input z."]
    #[must_use]
    #[inline(always)]
    pub const fn cordic_z(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Cordic input z."]
    #[inline(always)]
    pub const fn set_cordic_z(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CORDIC_Z {
    #[inline(always)]
    fn default() -> CORDIC_Z {
        CORDIC_Z(0)
    }
}
impl core::fmt::Debug for CORDIC_Z {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORDIC_Z")
            .field("cordic_z", &self.cordic_z())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CORDIC_Z {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CORDIC_Z {{ cordic_z: {=u32:?} }}", self.cordic_z())
    }
}
#[doc = "Pre-scale register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CPPRE(pub u32);
impl CPPRE {
    #[doc = "co-processor scaling of input."]
    #[must_use]
    #[inline(always)]
    pub const fn cppre_in(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "co-processor scaling of input."]
    #[inline(always)]
    pub const fn set_cppre_in(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "co-processor fixed point output."]
    #[must_use]
    #[inline(always)]
    pub const fn cppre_out(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "co-processor fixed point output."]
    #[inline(always)]
    pub const fn set_cppre_out(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "1 : forces sub-32 bit saturation."]
    #[must_use]
    #[inline(always)]
    pub const fn cppre_sat(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "1 : forces sub-32 bit saturation."]
    #[inline(always)]
    pub const fn set_cppre_sat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "0 = 8bits, 1 = 16bits."]
    #[must_use]
    #[inline(always)]
    pub const fn cppre_sat8(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "0 = 8bits, 1 = 16bits."]
    #[inline(always)]
    pub const fn set_cppre_sat8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for CPPRE {
    #[inline(always)]
    fn default() -> CPPRE {
        CPPRE(0)
    }
}
impl core::fmt::Debug for CPPRE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPPRE")
            .field("cppre_in", &self.cppre_in())
            .field("cppre_out", &self.cppre_out())
            .field("cppre_sat", &self.cppre_sat())
            .field("cppre_sat8", &self.cppre_sat8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CPPRE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CPPRE {{ cppre_in: {=u8:?}, cppre_out: {=u8:?}, cppre_sat: {=bool:?}, cppre_sat8: {=bool:?} }}",
            self.cppre_in(),
            self.cppre_out(),
            self.cppre_sat(),
            self.cppre_sat8()
        )
    }
}
#[doc = "Cursory register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CURSORY(pub u32);
impl CURSORY {
    #[doc = "1 : Enable cursory mode."]
    #[must_use]
    #[inline(always)]
    pub const fn cursory(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "1 : Enable cursory mode."]
    #[inline(always)]
    pub const fn set_cursory(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for CURSORY {
    #[inline(always)]
    fn default() -> CURSORY {
        CURSORY(0)
    }
}
impl core::fmt::Debug for CURSORY {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CURSORY")
            .field("cursory", &self.cursory())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CURSORY {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CURSORY {{ cursory: {=bool:?} }}", self.cursory())
    }
}
#[doc = "Read/Write register where error statuses are captured (sticky)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ERRSTAT(pub u32);
impl ERRSTAT {
    #[doc = "overflow."]
    #[must_use]
    #[inline(always)]
    pub const fn OVERFLOW(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "overflow."]
    #[inline(always)]
    pub const fn set_OVERFLOW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "nan."]
    #[must_use]
    #[inline(always)]
    pub const fn NAN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "nan."]
    #[inline(always)]
    pub const fn set_NAN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "fixed_pt_overflow."]
    #[must_use]
    #[inline(always)]
    pub const fn FIXEDOVERFLOW(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "fixed_pt_overflow."]
    #[inline(always)]
    pub const fn set_FIXEDOVERFLOW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "underflow."]
    #[must_use]
    #[inline(always)]
    pub const fn UNDERFLOW(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "underflow."]
    #[inline(always)]
    pub const fn set_UNDERFLOW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "bus_error."]
    #[must_use]
    #[inline(always)]
    pub const fn BUSERROR(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "bus_error."]
    #[inline(always)]
    pub const fn set_BUSERROR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for ERRSTAT {
    #[inline(always)]
    fn default() -> ERRSTAT {
        ERRSTAT(0)
    }
}
impl core::fmt::Debug for ERRSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ERRSTAT")
            .field("OVERFLOW", &self.OVERFLOW())
            .field("NAN", &self.NAN())
            .field("FIXEDOVERFLOW", &self.FIXEDOVERFLOW())
            .field("UNDERFLOW", &self.UNDERFLOW())
            .field("BUSERROR", &self.BUSERROR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ERRSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ERRSTAT {{ OVERFLOW: {=bool:?}, NAN: {=bool:?}, FIXEDOVERFLOW: {=bool:?}, UNDERFLOW: {=bool:?}, BUSERROR: {=bool:?} }}",
            self.OVERFLOW(),
            self.NAN(),
            self.FIXEDOVERFLOW(),
            self.UNDERFLOW(),
            self.BUSERROR()
        )
    }
}
#[doc = "Event Enable register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EVENTEN(pub u32);
impl EVENTEN {
    #[doc = "1 : Enable event trigger on Floating point overflow."]
    #[must_use]
    #[inline(always)]
    pub const fn event_oflow(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "1 : Enable event trigger on Floating point overflow."]
    #[inline(always)]
    pub const fn set_event_oflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1 : Enable event trigger on Floating point NaN."]
    #[must_use]
    #[inline(always)]
    pub const fn event_nan(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1 : Enable event trigger on Floating point NaN."]
    #[inline(always)]
    pub const fn set_event_nan(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "1: Enable event trigger on Fixed point Overflow."]
    #[must_use]
    #[inline(always)]
    pub const fn event_fixed(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "1: Enable event trigger on Fixed point Overflow."]
    #[inline(always)]
    pub const fn set_event_fixed(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "1 : Enable event trigger on Subnormal truncation."]
    #[must_use]
    #[inline(always)]
    pub const fn event_uflow(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "1 : Enable event trigger on Subnormal truncation."]
    #[inline(always)]
    pub const fn set_event_uflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "1: Enable event trigger on AHBM Buss Error."]
    #[must_use]
    #[inline(always)]
    pub const fn event_berr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "1: Enable event trigger on AHBM Buss Error."]
    #[inline(always)]
    pub const fn set_event_berr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "1: Enable event trigger on instruction completion."]
    #[must_use]
    #[inline(always)]
    pub const fn event_comp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "1: Enable event trigger on instruction completion."]
    #[inline(always)]
    pub const fn set_event_comp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for EVENTEN {
    #[inline(always)]
    fn default() -> EVENTEN {
        EVENTEN(0)
    }
}
impl core::fmt::Debug for EVENTEN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVENTEN")
            .field("event_oflow", &self.event_oflow())
            .field("event_nan", &self.event_nan())
            .field("event_fixed", &self.event_fixed())
            .field("event_uflow", &self.event_uflow())
            .field("event_berr", &self.event_berr())
            .field("event_comp", &self.event_comp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EVENTEN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EVENTEN {{ event_oflow: {=bool:?}, event_nan: {=bool:?}, event_fixed: {=bool:?}, event_uflow: {=bool:?}, event_berr: {=bool:?}, event_comp: {=bool:?} }}",
            self.event_oflow(),
            self.event_nan(),
            self.event_fixed(),
            self.event_uflow(),
            self.event_berr(),
            self.event_comp()
        )
    }
}
#[doc = "Base address register for input A region."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INABASE(pub u32);
impl INABASE {
    #[doc = "Base address register for the input A region."]
    #[must_use]
    #[inline(always)]
    pub const fn inabase(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Base address register for the input A region."]
    #[inline(always)]
    pub const fn set_inabase(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for INABASE {
    #[inline(always)]
    fn default() -> INABASE {
        INABASE(0)
    }
}
impl core::fmt::Debug for INABASE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INABASE")
            .field("inabase", &self.inabase())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INABASE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "INABASE {{ inabase: {=u32:?} }}", self.inabase())
    }
}
#[doc = "Input A format."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INAFORMAT(pub u32);
impl INAFORMAT {
    #[doc = "Input A Internal format (00: q15; 01:q31; 10:float)."]
    #[must_use]
    #[inline(always)]
    pub const fn ina_formatint(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Input A Internal format (00: q15; 01:q31; 10:float)."]
    #[inline(always)]
    pub const fn set_ina_formatint(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Input A External format (00: q15; 01:q31; 10:float)."]
    #[must_use]
    #[inline(always)]
    pub const fn ina_formatext(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Input A External format (00: q15; 01:q31; 10:float)."]
    #[inline(always)]
    pub const fn set_ina_formatext(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Input A Scaler value (for scaled 'q31' formats)."]
    #[must_use]
    #[inline(always)]
    pub const fn ina_scaler(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Input A Scaler value (for scaled 'q31' formats)."]
    #[inline(always)]
    pub const fn set_ina_scaler(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for INAFORMAT {
    #[inline(always)]
    fn default() -> INAFORMAT {
        INAFORMAT(0)
    }
}
impl core::fmt::Debug for INAFORMAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INAFORMAT")
            .field("ina_formatint", &self.ina_formatint())
            .field("ina_formatext", &self.ina_formatext())
            .field("ina_scaler", &self.ina_scaler())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INAFORMAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INAFORMAT {{ ina_formatint: {=u8:?}, ina_formatext: {=u8:?}, ina_scaler: {=u8:?} }}",
            self.ina_formatint(),
            self.ina_formatext(),
            self.ina_scaler()
        )
    }
}
#[doc = "Base address register for input B region."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INBBASE(pub u32);
impl INBBASE {
    #[doc = "Base address register for the input B region."]
    #[must_use]
    #[inline(always)]
    pub const fn inbbase(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Base address register for the input B region."]
    #[inline(always)]
    pub const fn set_inbbase(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for INBBASE {
    #[inline(always)]
    fn default() -> INBBASE {
        INBBASE(0)
    }
}
impl core::fmt::Debug for INBBASE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INBBASE")
            .field("inbbase", &self.inbbase())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INBBASE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "INBBASE {{ inbbase: {=u32:?} }}", self.inbbase())
    }
}
#[doc = "Input B format."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INBFORMAT(pub u32);
impl INBFORMAT {
    #[doc = "Input B Internal format (00: q15; 01:q31; 10:float)."]
    #[must_use]
    #[inline(always)]
    pub const fn inb_formatint(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Input B Internal format (00: q15; 01:q31; 10:float)."]
    #[inline(always)]
    pub const fn set_inb_formatint(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Input B External format (00: q15; 01:q31; 10:float)."]
    #[must_use]
    #[inline(always)]
    pub const fn inb_formatext(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Input B External format (00: q15; 01:q31; 10:float)."]
    #[inline(always)]
    pub const fn set_inb_formatext(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Input B Scaler value (for scaled 'q31' formats)."]
    #[must_use]
    #[inline(always)]
    pub const fn inb_scaler(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Input B Scaler value (for scaled 'q31' formats)."]
    #[inline(always)]
    pub const fn set_inb_scaler(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for INBFORMAT {
    #[inline(always)]
    fn default() -> INBFORMAT {
        INBFORMAT(0)
    }
}
impl core::fmt::Debug for INBFORMAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INBFORMAT")
            .field("inb_formatint", &self.inb_formatint())
            .field("inb_formatext", &self.inb_formatext())
            .field("inb_scaler", &self.inb_scaler())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INBFORMAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INBFORMAT {{ inb_formatint: {=u8:?}, inb_formatext: {=u8:?}, inb_scaler: {=u8:?} }}",
            self.inb_formatint(),
            self.inb_formatext(),
            self.inb_scaler()
        )
    }
}
#[doc = "INTERRUPT enable register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INTREN(pub u32);
impl INTREN {
    #[doc = "1 : Enable interrupt on Floating point overflow."]
    #[must_use]
    #[inline(always)]
    pub const fn intr_oflow(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "1 : Enable interrupt on Floating point overflow."]
    #[inline(always)]
    pub const fn set_intr_oflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1 : Enable interrupt on Floating point NaN."]
    #[must_use]
    #[inline(always)]
    pub const fn intr_nan(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1 : Enable interrupt on Floating point NaN."]
    #[inline(always)]
    pub const fn set_intr_nan(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "1: Enable interrupt on Fixed point Overflow."]
    #[must_use]
    #[inline(always)]
    pub const fn intr_fixed(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "1: Enable interrupt on Fixed point Overflow."]
    #[inline(always)]
    pub const fn set_intr_fixed(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "1 : Enable interrupt on Subnormal truncation."]
    #[must_use]
    #[inline(always)]
    pub const fn intr_uflow(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "1 : Enable interrupt on Subnormal truncation."]
    #[inline(always)]
    pub const fn set_intr_uflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "1: Enable interrupt on AHBM Buss Error."]
    #[must_use]
    #[inline(always)]
    pub const fn intr_berr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "1: Enable interrupt on AHBM Buss Error."]
    #[inline(always)]
    pub const fn set_intr_berr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "1: Enable interrupt on instruction completion."]
    #[must_use]
    #[inline(always)]
    pub const fn intr_comp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "1: Enable interrupt on instruction completion."]
    #[inline(always)]
    pub const fn set_intr_comp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for INTREN {
    #[inline(always)]
    fn default() -> INTREN {
        INTREN(0)
    }
}
impl core::fmt::Debug for INTREN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTREN")
            .field("intr_oflow", &self.intr_oflow())
            .field("intr_nan", &self.intr_nan())
            .field("intr_fixed", &self.intr_fixed())
            .field("intr_uflow", &self.intr_uflow())
            .field("intr_berr", &self.intr_berr())
            .field("intr_comp", &self.intr_comp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INTREN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INTREN {{ intr_oflow: {=bool:?}, intr_nan: {=bool:?}, intr_fixed: {=bool:?}, intr_uflow: {=bool:?}, intr_berr: {=bool:?}, intr_comp: {=bool:?} }}",
            self.intr_oflow(),
            self.intr_nan(),
            self.intr_fixed(),
            self.intr_uflow(),
            self.intr_berr(),
            self.intr_comp()
        )
    }
}
#[doc = "INTERRUPT STATUS register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INTRSTAT(pub u32);
impl INTRSTAT {
    #[doc = "Intr status ( 1 bit to indicate interrupt captured, 0 means no new interrupt), write any value will clear this bit."]
    #[must_use]
    #[inline(always)]
    pub const fn intr_stat(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Intr status ( 1 bit to indicate interrupt captured, 0 means no new interrupt), write any value will clear this bit."]
    #[inline(always)]
    pub const fn set_intr_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for INTRSTAT {
    #[inline(always)]
    fn default() -> INTRSTAT {
        INTRSTAT(0)
    }
}
impl core::fmt::Debug for INTRSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTRSTAT")
            .field("intr_stat", &self.intr_stat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INTRSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "INTRSTAT {{ intr_stat: {=bool:?} }}", self.intr_stat())
    }
}
#[doc = "Length register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LENGTH(pub u32);
impl LENGTH {
    #[doc = "Length register. When FIR : fir_xlength = inst_length\\[15:0\\] , fir_tlength = inst_len\\[31:16\\]. When MTX : rows_a = inst_length\\[4:0\\] , cols_a = inst_length\\[12:8\\] , cols_b = inst_length\\[20:16\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn inst_length(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Length register. When FIR : fir_xlength = inst_length\\[15:0\\] , fir_tlength = inst_len\\[31:16\\]. When MTX : rows_a = inst_length\\[4:0\\] , cols_a = inst_length\\[12:8\\] , cols_b = inst_length\\[20:16\\]."]
    #[inline(always)]
    pub const fn set_inst_length(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for LENGTH {
    #[inline(always)]
    fn default() -> LENGTH {
        LENGTH(0)
    }
}
impl core::fmt::Debug for LENGTH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LENGTH")
            .field("inst_length", &self.inst_length())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LENGTH {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LENGTH {{ inst_length: {=u32:?} }}", self.inst_length())
    }
}
#[doc = "Misc register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MISC(pub u32);
impl MISC {
    #[doc = "Misc register. For Matrix : Used for scale factor."]
    #[must_use]
    #[inline(always)]
    pub const fn inst_misc(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Misc register. For Matrix : Used for scale factor."]
    #[inline(always)]
    pub const fn set_inst_misc(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MISC {
    #[inline(always)]
    fn default() -> MISC {
        MISC(0)
    }
}
impl core::fmt::Debug for MISC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISC")
            .field("inst_misc", &self.inst_misc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MISC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MISC {{ inst_misc: {=u32:?} }}", self.inst_misc())
    }
}
#[doc = "Base address register for output region."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OUTBASE(pub u32);
impl OUTBASE {
    #[doc = "Base address register for the output region."]
    #[must_use]
    #[inline(always)]
    pub const fn outbase(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Base address register for the output region."]
    #[inline(always)]
    pub const fn set_outbase(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for OUTBASE {
    #[inline(always)]
    fn default() -> OUTBASE {
        OUTBASE(0)
    }
}
impl core::fmt::Debug for OUTBASE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUTBASE")
            .field("outbase", &self.outbase())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OUTBASE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "OUTBASE {{ outbase: {=u32:?} }}", self.outbase())
    }
}
#[doc = "Output format."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OUTFORMAT(pub u32);
impl OUTFORMAT {
    #[doc = "Output Internal format (00: q15; 01:q31; 10:float)."]
    #[must_use]
    #[inline(always)]
    pub const fn out_formatint(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Output Internal format (00: q15; 01:q31; 10:float)."]
    #[inline(always)]
    pub const fn set_out_formatint(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Output External format (00: q15; 01:q31; 10:float)."]
    #[must_use]
    #[inline(always)]
    pub const fn out_formatext(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Output External format (00: q15; 01:q31; 10:float)."]
    #[inline(always)]
    pub const fn set_out_formatext(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Output Scaler value (for scaled 'q31' formats)."]
    #[must_use]
    #[inline(always)]
    pub const fn out_scaler(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Output Scaler value (for scaled 'q31' formats)."]
    #[inline(always)]
    pub const fn set_out_scaler(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for OUTFORMAT {
    #[inline(always)]
    fn default() -> OUTFORMAT {
        OUTFORMAT(0)
    }
}
impl core::fmt::Debug for OUTFORMAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUTFORMAT")
            .field("out_formatint", &self.out_formatint())
            .field("out_formatext", &self.out_formatext())
            .field("out_scaler", &self.out_scaler())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OUTFORMAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OUTFORMAT {{ out_formatint: {=u8:?}, out_formatext: {=u8:?}, out_scaler: {=u8:?} }}",
            self.out_formatint(),
            self.out_formatext(),
            self.out_scaler()
        )
    }
}
#[doc = "Base address register for temp region."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TMPBASE(pub u32);
impl TMPBASE {
    #[doc = "Base address register for the temporary region."]
    #[must_use]
    #[inline(always)]
    pub const fn tmpbase(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Base address register for the temporary region."]
    #[inline(always)]
    pub const fn set_tmpbase(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TMPBASE {
    #[inline(always)]
    fn default() -> TMPBASE {
        TMPBASE(0)
    }
}
impl core::fmt::Debug for TMPBASE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TMPBASE")
            .field("tmpbase", &self.tmpbase())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TMPBASE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TMPBASE {{ tmpbase: {=u32:?} }}", self.tmpbase())
    }
}
#[doc = "Temp format."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TMPFORMAT(pub u32);
impl TMPFORMAT {
    #[doc = "Temp Internal format (00: q15; 01:q31; 10:float)."]
    #[must_use]
    #[inline(always)]
    pub const fn tmp_formatint(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Temp Internal format (00: q15; 01:q31; 10:float)."]
    #[inline(always)]
    pub const fn set_tmp_formatint(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Temp External format (00: q15; 01:q31; 10:float)."]
    #[must_use]
    #[inline(always)]
    pub const fn tmp_formatext(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Temp External format (00: q15; 01:q31; 10:float)."]
    #[inline(always)]
    pub const fn set_tmp_formatext(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Temp Scaler value (for scaled 'q31' formats)."]
    #[must_use]
    #[inline(always)]
    pub const fn tmp_scaler(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Temp Scaler value (for scaled 'q31' formats)."]
    #[inline(always)]
    pub const fn set_tmp_scaler(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for TMPFORMAT {
    #[inline(always)]
    fn default() -> TMPFORMAT {
        TMPFORMAT(0)
    }
}
impl core::fmt::Debug for TMPFORMAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TMPFORMAT")
            .field("tmp_formatint", &self.tmp_formatint())
            .field("tmp_formatext", &self.tmp_formatext())
            .field("tmp_scaler", &self.tmp_scaler())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TMPFORMAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TMPFORMAT {{ tmp_formatint: {=u8:?}, tmp_formatext: {=u8:?}, tmp_scaler: {=u8:?} }}",
            self.tmp_formatint(),
            self.tmp_formatext(),
            self.tmp_scaler()
        )
    }
}
#[doc = "Compute register bank."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct compreg(pub u32);
impl compreg {
    #[doc = "Compute register bank."]
    #[must_use]
    #[inline(always)]
    pub const fn compreg(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Compute register bank."]
    #[inline(always)]
    pub const fn set_compreg(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for compreg {
    #[inline(always)]
    fn default() -> compreg {
        compreg(0)
    }
}
impl core::fmt::Debug for compreg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("compreg")
            .field("compreg", &self.compreg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for compreg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "compreg {{ compreg: {=u32:?} }}", self.compreg())
    }
}
#[doc = "General purpose register bank N."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct gpreg(pub u32);
impl gpreg {
    #[doc = "General purpose register bank."]
    #[must_use]
    #[inline(always)]
    pub const fn gpreg(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "General purpose register bank."]
    #[inline(always)]
    pub const fn set_gpreg(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for gpreg {
    #[inline(always)]
    fn default() -> gpreg {
        gpreg(0)
    }
}
impl core::fmt::Debug for gpreg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("gpreg")
            .field("gpreg", &self.gpreg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for gpreg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "gpreg {{ gpreg: {=u32:?} }}", self.gpreg())
    }
}
