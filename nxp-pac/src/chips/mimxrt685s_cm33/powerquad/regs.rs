#[doc = "Compute register bank"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Compreg(pub u32);
impl Compreg {
    #[doc = "Compute register bank"]
    #[must_use]
    #[inline(always)]
    pub const fn compreg(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Compute register bank"]
    #[inline(always)]
    pub const fn set_compreg(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Compreg {
    #[inline(always)]
    fn default() -> Compreg {
        Compreg(0)
    }
}
impl core::fmt::Debug for Compreg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Compreg")
            .field("compreg", &self.compreg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Compreg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Compreg {{ compreg: {=u32:?} }}", self.compreg())
    }
}
#[doc = "PowerQuad Control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Control(pub u32);
impl Control {
    #[doc = "opcode specific to decode_machine"]
    #[must_use]
    #[inline(always)]
    pub const fn decode_opcode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "opcode specific to decode_machine"]
    #[inline(always)]
    pub const fn set_decode_opcode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "0 : Coprocessor , 1 : matrix , 2 : fft , 3 : fir , 4 : stat , 5 : cordic , 6 -15 : NA"]
    #[must_use]
    #[inline(always)]
    pub const fn decode_machine(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "0 : Coprocessor , 1 : matrix , 2 : fft , 3 : fir , 4 : stat , 5 : cordic , 6 -15 : NA"]
    #[inline(always)]
    pub const fn set_decode_machine(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Instruction busy signal when high indicates processing is on"]
    #[must_use]
    #[inline(always)]
    pub const fn inst_busy(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Instruction busy signal when high indicates processing is on"]
    #[inline(always)]
    pub const fn set_inst_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Control {
    #[inline(always)]
    fn default() -> Control {
        Control(0)
    }
}
impl core::fmt::Debug for Control {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Control")
            .field("decode_opcode", &self.decode_opcode())
            .field("decode_machine", &self.decode_machine())
            .field("inst_busy", &self.inst_busy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Control {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Control {{ decode_opcode: {=u8:?}, decode_machine: {=u8:?}, inst_busy: {=bool:?} }}",
            self.decode_opcode(),
            self.decode_machine(),
            self.inst_busy()
        )
    }
}
#[doc = "Cordic input X register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CordicX(pub u32);
impl CordicX {
    #[doc = "Cordic input x"]
    #[must_use]
    #[inline(always)]
    pub const fn cordic_x(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Cordic input x"]
    #[inline(always)]
    pub const fn set_cordic_x(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CordicX {
    #[inline(always)]
    fn default() -> CordicX {
        CordicX(0)
    }
}
impl core::fmt::Debug for CordicX {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CordicX")
            .field("cordic_x", &self.cordic_x())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CordicX {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CordicX {{ cordic_x: {=u32:?} }}", self.cordic_x())
    }
}
#[doc = "Cordic input Y register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CordicY(pub u32);
impl CordicY {
    #[doc = "Cordic input y"]
    #[must_use]
    #[inline(always)]
    pub const fn cordic_y(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Cordic input y"]
    #[inline(always)]
    pub const fn set_cordic_y(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CordicY {
    #[inline(always)]
    fn default() -> CordicY {
        CordicY(0)
    }
}
impl core::fmt::Debug for CordicY {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CordicY")
            .field("cordic_y", &self.cordic_y())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CordicY {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CordicY {{ cordic_y: {=u32:?} }}", self.cordic_y())
    }
}
#[doc = "Cordic input Z register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CordicZ(pub u32);
impl CordicZ {
    #[doc = "Cordic input z"]
    #[must_use]
    #[inline(always)]
    pub const fn cordic_z(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Cordic input z"]
    #[inline(always)]
    pub const fn set_cordic_z(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CordicZ {
    #[inline(always)]
    fn default() -> CordicZ {
        CordicZ(0)
    }
}
impl core::fmt::Debug for CordicZ {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CordicZ")
            .field("cordic_z", &self.cordic_z())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CordicZ {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CordicZ {{ cordic_z: {=u32:?} }}", self.cordic_z())
    }
}
#[doc = "Pre-scale register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cppre(pub u32);
impl Cppre {
    #[doc = "co-processor scaling of input"]
    #[must_use]
    #[inline(always)]
    pub const fn cppre_in(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "co-processor scaling of input"]
    #[inline(always)]
    pub const fn set_cppre_in(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "co-processor fixed point output"]
    #[must_use]
    #[inline(always)]
    pub const fn cppre_out(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "co-processor fixed point output"]
    #[inline(always)]
    pub const fn set_cppre_out(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "1 : forces sub-32 bit saturation"]
    #[must_use]
    #[inline(always)]
    pub const fn cppre_sat(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "1 : forces sub-32 bit saturation"]
    #[inline(always)]
    pub const fn set_cppre_sat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "0 = 8bits, 1 = 16bits"]
    #[must_use]
    #[inline(always)]
    pub const fn cppre_sat8(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "0 = 8bits, 1 = 16bits"]
    #[inline(always)]
    pub const fn set_cppre_sat8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Cppre {
    #[inline(always)]
    fn default() -> Cppre {
        Cppre(0)
    }
}
impl core::fmt::Debug for Cppre {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cppre")
            .field("cppre_in", &self.cppre_in())
            .field("cppre_out", &self.cppre_out())
            .field("cppre_sat", &self.cppre_sat())
            .field("cppre_sat8", &self.cppre_sat8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cppre {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cppre {{ cppre_in: {=u8:?}, cppre_out: {=u8:?}, cppre_sat: {=bool:?}, cppre_sat8: {=bool:?} }}",
            self.cppre_in(),
            self.cppre_out(),
            self.cppre_sat(),
            self.cppre_sat8()
        )
    }
}
#[doc = "Cursory register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cursory(pub u32);
impl Cursory {
    #[doc = "1 : Enable cursory mode"]
    #[must_use]
    #[inline(always)]
    pub const fn cursory(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "1 : Enable cursory mode"]
    #[inline(always)]
    pub const fn set_cursory(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Cursory {
    #[inline(always)]
    fn default() -> Cursory {
        Cursory(0)
    }
}
impl core::fmt::Debug for Cursory {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cursory")
            .field("cursory", &self.cursory())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cursory {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cursory {{ cursory: {=bool:?} }}", self.cursory())
    }
}
#[doc = "Read/Write register where error statuses are captured (sticky)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Errstat(pub u32);
impl Errstat {
    #[doc = "overflow"]
    #[must_use]
    #[inline(always)]
    pub const fn overflow(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "overflow"]
    #[inline(always)]
    pub const fn set_overflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "nan"]
    #[must_use]
    #[inline(always)]
    pub const fn nan(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "nan"]
    #[inline(always)]
    pub const fn set_nan(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "fixed_pt_overflow"]
    #[must_use]
    #[inline(always)]
    pub const fn fixedoverflow(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "fixed_pt_overflow"]
    #[inline(always)]
    pub const fn set_fixedoverflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "underflow"]
    #[must_use]
    #[inline(always)]
    pub const fn underflow(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "underflow"]
    #[inline(always)]
    pub const fn set_underflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "bus_error"]
    #[must_use]
    #[inline(always)]
    pub const fn buserror(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "bus_error"]
    #[inline(always)]
    pub const fn set_buserror(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Errstat {
    #[inline(always)]
    fn default() -> Errstat {
        Errstat(0)
    }
}
impl core::fmt::Debug for Errstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Errstat")
            .field("overflow", &self.overflow())
            .field("nan", &self.nan())
            .field("fixedoverflow", &self.fixedoverflow())
            .field("underflow", &self.underflow())
            .field("buserror", &self.buserror())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Errstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Errstat {{ overflow: {=bool:?}, nan: {=bool:?}, fixedoverflow: {=bool:?}, underflow: {=bool:?}, buserror: {=bool:?} }}",
            self.overflow(),
            self.nan(),
            self.fixedoverflow(),
            self.underflow(),
            self.buserror()
        )
    }
}
#[doc = "Event Enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eventen(pub u32);
impl Eventen {
    #[doc = "1 : Enable event trigger on Floating point overflow"]
    #[must_use]
    #[inline(always)]
    pub const fn event_oflow(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "1 : Enable event trigger on Floating point overflow"]
    #[inline(always)]
    pub const fn set_event_oflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1 : Enable event trigger on Floating point NaN"]
    #[must_use]
    #[inline(always)]
    pub const fn event_nan(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1 : Enable event trigger on Floating point NaN"]
    #[inline(always)]
    pub const fn set_event_nan(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "1: Enable event trigger on Fixed point Overflow"]
    #[must_use]
    #[inline(always)]
    pub const fn event_fixed(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "1: Enable event trigger on Fixed point Overflow"]
    #[inline(always)]
    pub const fn set_event_fixed(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "1 : Enable event trigger on Subnormal truncation"]
    #[must_use]
    #[inline(always)]
    pub const fn event_uflow(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "1 : Enable event trigger on Subnormal truncation"]
    #[inline(always)]
    pub const fn set_event_uflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "1: Enable event trigger on AHBM Buss Error"]
    #[must_use]
    #[inline(always)]
    pub const fn event_berr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "1: Enable event trigger on AHBM Buss Error"]
    #[inline(always)]
    pub const fn set_event_berr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "1: Enable event trigger on instruction completion"]
    #[must_use]
    #[inline(always)]
    pub const fn event_comp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "1: Enable event trigger on instruction completion"]
    #[inline(always)]
    pub const fn set_event_comp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Eventen {
    #[inline(always)]
    fn default() -> Eventen {
        Eventen(0)
    }
}
impl core::fmt::Debug for Eventen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eventen")
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
impl defmt::Format for Eventen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Eventen {{ event_oflow: {=bool:?}, event_nan: {=bool:?}, event_fixed: {=bool:?}, event_uflow: {=bool:?}, event_berr: {=bool:?}, event_comp: {=bool:?} }}",
            self.event_oflow(),
            self.event_nan(),
            self.event_fixed(),
            self.event_uflow(),
            self.event_berr(),
            self.event_comp()
        )
    }
}
#[doc = "General purpose register bank N."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpreg(pub u32);
impl Gpreg {
    #[doc = "General purpose register bank"]
    #[must_use]
    #[inline(always)]
    pub const fn gpreg(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "General purpose register bank"]
    #[inline(always)]
    pub const fn set_gpreg(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gpreg {
    #[inline(always)]
    fn default() -> Gpreg {
        Gpreg(0)
    }
}
impl core::fmt::Debug for Gpreg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpreg")
            .field("gpreg", &self.gpreg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpreg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gpreg {{ gpreg: {=u32:?} }}", self.gpreg())
    }
}
#[doc = "Base address register for input A region"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inabase(pub u32);
impl Inabase {
    #[doc = "Base address register for the input A region"]
    #[must_use]
    #[inline(always)]
    pub const fn inabase(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Base address register for the input A region"]
    #[inline(always)]
    pub const fn set_inabase(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Inabase {
    #[inline(always)]
    fn default() -> Inabase {
        Inabase(0)
    }
}
impl core::fmt::Debug for Inabase {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Inabase")
            .field("inabase", &self.inabase())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Inabase {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Inabase {{ inabase: {=u32:?} }}", self.inabase())
    }
}
#[doc = "Input A format"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inaformat(pub u32);
impl Inaformat {
    #[doc = "Input A Internal format (00: q15; 01:q31; 10:float)"]
    #[must_use]
    #[inline(always)]
    pub const fn ina_formatint(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Input A Internal format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub const fn set_ina_formatint(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Input A External format (00: q15; 01:q31; 10:float)"]
    #[must_use]
    #[inline(always)]
    pub const fn ina_formatext(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Input A External format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub const fn set_ina_formatext(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Input A Scaler value (for scaled 'q31' formats)"]
    #[must_use]
    #[inline(always)]
    pub const fn ina_scaler(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Input A Scaler value (for scaled 'q31' formats)"]
    #[inline(always)]
    pub const fn set_ina_scaler(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Inaformat {
    #[inline(always)]
    fn default() -> Inaformat {
        Inaformat(0)
    }
}
impl core::fmt::Debug for Inaformat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Inaformat")
            .field("ina_formatint", &self.ina_formatint())
            .field("ina_formatext", &self.ina_formatext())
            .field("ina_scaler", &self.ina_scaler())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Inaformat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Inaformat {{ ina_formatint: {=u8:?}, ina_formatext: {=u8:?}, ina_scaler: {=u8:?} }}",
            self.ina_formatint(),
            self.ina_formatext(),
            self.ina_scaler()
        )
    }
}
#[doc = "Base address register for input B region"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inbbase(pub u32);
impl Inbbase {
    #[doc = "Base address register for the input B region"]
    #[must_use]
    #[inline(always)]
    pub const fn inbbase(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Base address register for the input B region"]
    #[inline(always)]
    pub const fn set_inbbase(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Inbbase {
    #[inline(always)]
    fn default() -> Inbbase {
        Inbbase(0)
    }
}
impl core::fmt::Debug for Inbbase {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Inbbase")
            .field("inbbase", &self.inbbase())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Inbbase {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Inbbase {{ inbbase: {=u32:?} }}", self.inbbase())
    }
}
#[doc = "Input B format"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inbformat(pub u32);
impl Inbformat {
    #[doc = "Input B Internal format (00: q15; 01:q31; 10:float)"]
    #[must_use]
    #[inline(always)]
    pub const fn inb_formatint(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Input B Internal format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub const fn set_inb_formatint(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Input B External format (00: q15; 01:q31; 10:float)"]
    #[must_use]
    #[inline(always)]
    pub const fn inb_formatext(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Input B External format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub const fn set_inb_formatext(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Input B Scaler value (for scaled 'q31' formats)"]
    #[must_use]
    #[inline(always)]
    pub const fn inb_scaler(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Input B Scaler value (for scaled 'q31' formats)"]
    #[inline(always)]
    pub const fn set_inb_scaler(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Inbformat {
    #[inline(always)]
    fn default() -> Inbformat {
        Inbformat(0)
    }
}
impl core::fmt::Debug for Inbformat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Inbformat")
            .field("inb_formatint", &self.inb_formatint())
            .field("inb_formatext", &self.inb_formatext())
            .field("inb_scaler", &self.inb_scaler())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Inbformat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Inbformat {{ inb_formatint: {=u8:?}, inb_formatext: {=u8:?}, inb_scaler: {=u8:?} }}",
            self.inb_formatint(),
            self.inb_formatext(),
            self.inb_scaler()
        )
    }
}
#[doc = "INTERRUPT enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intren(pub u32);
impl Intren {
    #[doc = "1 : Enable interrupt on Floating point overflow"]
    #[must_use]
    #[inline(always)]
    pub const fn intr_oflow(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "1 : Enable interrupt on Floating point overflow"]
    #[inline(always)]
    pub const fn set_intr_oflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1 : Enable interrupt on Floating point NaN"]
    #[must_use]
    #[inline(always)]
    pub const fn intr_nan(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1 : Enable interrupt on Floating point NaN"]
    #[inline(always)]
    pub const fn set_intr_nan(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "1: Enable interrupt on Fixed point Overflow"]
    #[must_use]
    #[inline(always)]
    pub const fn intr_fixed(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "1: Enable interrupt on Fixed point Overflow"]
    #[inline(always)]
    pub const fn set_intr_fixed(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "1 : Enable interrupt on Subnormal truncation"]
    #[must_use]
    #[inline(always)]
    pub const fn intr_uflow(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "1 : Enable interrupt on Subnormal truncation"]
    #[inline(always)]
    pub const fn set_intr_uflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "1: Enable interrupt on AHBM Buss Error"]
    #[must_use]
    #[inline(always)]
    pub const fn intr_berr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "1: Enable interrupt on AHBM Buss Error"]
    #[inline(always)]
    pub const fn set_intr_berr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "1: Enable interrupt on instruction completion"]
    #[must_use]
    #[inline(always)]
    pub const fn intr_comp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "1: Enable interrupt on instruction completion"]
    #[inline(always)]
    pub const fn set_intr_comp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Intren {
    #[inline(always)]
    fn default() -> Intren {
        Intren(0)
    }
}
impl core::fmt::Debug for Intren {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intren")
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
impl defmt::Format for Intren {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intren {{ intr_oflow: {=bool:?}, intr_nan: {=bool:?}, intr_fixed: {=bool:?}, intr_uflow: {=bool:?}, intr_berr: {=bool:?}, intr_comp: {=bool:?} }}",
            self.intr_oflow(),
            self.intr_nan(),
            self.intr_fixed(),
            self.intr_uflow(),
            self.intr_berr(),
            self.intr_comp()
        )
    }
}
#[doc = "INTERRUPT STATUS register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intrstat(pub u32);
impl Intrstat {
    #[doc = "Intr status ( 1 bit to indicate interrupt captured, 0 means no new interrupt), write any value will clear this bit"]
    #[must_use]
    #[inline(always)]
    pub const fn intr_stat(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Intr status ( 1 bit to indicate interrupt captured, 0 means no new interrupt), write any value will clear this bit"]
    #[inline(always)]
    pub const fn set_intr_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Intrstat {
    #[inline(always)]
    fn default() -> Intrstat {
        Intrstat(0)
    }
}
impl core::fmt::Debug for Intrstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intrstat")
            .field("intr_stat", &self.intr_stat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intrstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Intrstat {{ intr_stat: {=bool:?} }}", self.intr_stat())
    }
}
#[doc = "Length register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Length(pub u32);
impl Length {
    #[doc = "Length register. When FIR : fir_xlength = inst_length\\[15:0\\] , fir_tlength = inst_len\\[31:16\\]. When MTX : rows_a = inst_length\\[4:0\\] , cols_a = inst_length\\[12:8\\] , cols_b = inst_length\\[20:16\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn inst_length(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Length register. When FIR : fir_xlength = inst_length\\[15:0\\] , fir_tlength = inst_len\\[31:16\\]. When MTX : rows_a = inst_length\\[4:0\\] , cols_a = inst_length\\[12:8\\] , cols_b = inst_length\\[20:16\\]"]
    #[inline(always)]
    pub const fn set_inst_length(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Length {
    #[inline(always)]
    fn default() -> Length {
        Length(0)
    }
}
impl core::fmt::Debug for Length {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Length")
            .field("inst_length", &self.inst_length())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Length {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Length {{ inst_length: {=u32:?} }}", self.inst_length())
    }
}
#[doc = "Misc register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Misc(pub u32);
impl Misc {
    #[doc = "Misc register. For Matrix : Used for scale factor"]
    #[must_use]
    #[inline(always)]
    pub const fn inst_misc(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Misc register. For Matrix : Used for scale factor"]
    #[inline(always)]
    pub const fn set_inst_misc(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Misc {
    #[inline(always)]
    fn default() -> Misc {
        Misc(0)
    }
}
impl core::fmt::Debug for Misc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Misc")
            .field("inst_misc", &self.inst_misc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Misc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Misc {{ inst_misc: {=u32:?} }}", self.inst_misc())
    }
}
#[doc = "Base address register for output region"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Outbase(pub u32);
impl Outbase {
    #[doc = "Base address register for the output region"]
    #[must_use]
    #[inline(always)]
    pub const fn outbase(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Base address register for the output region"]
    #[inline(always)]
    pub const fn set_outbase(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Outbase {
    #[inline(always)]
    fn default() -> Outbase {
        Outbase(0)
    }
}
impl core::fmt::Debug for Outbase {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Outbase")
            .field("outbase", &self.outbase())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Outbase {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Outbase {{ outbase: {=u32:?} }}", self.outbase())
    }
}
#[doc = "Output format"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Outformat(pub u32);
impl Outformat {
    #[doc = "Output Internal format (00: q15; 01:q31; 10:float)"]
    #[must_use]
    #[inline(always)]
    pub const fn out_formatint(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Output Internal format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub const fn set_out_formatint(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Output External format (00: q15; 01:q31; 10:float)"]
    #[must_use]
    #[inline(always)]
    pub const fn out_formatext(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Output External format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub const fn set_out_formatext(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Output Scaler value (for scaled 'q31' formats)"]
    #[must_use]
    #[inline(always)]
    pub const fn out_scaler(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Output Scaler value (for scaled 'q31' formats)"]
    #[inline(always)]
    pub const fn set_out_scaler(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Outformat {
    #[inline(always)]
    fn default() -> Outformat {
        Outformat(0)
    }
}
impl core::fmt::Debug for Outformat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Outformat")
            .field("out_formatint", &self.out_formatint())
            .field("out_formatext", &self.out_formatext())
            .field("out_scaler", &self.out_scaler())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Outformat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Outformat {{ out_formatint: {=u8:?}, out_formatext: {=u8:?}, out_scaler: {=u8:?} }}",
            self.out_formatint(),
            self.out_formatext(),
            self.out_scaler()
        )
    }
}
#[doc = "Base address register for temp region"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmpbase(pub u32);
impl Tmpbase {
    #[doc = "Base address register for the temporary region"]
    #[must_use]
    #[inline(always)]
    pub const fn tmpbase(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Base address register for the temporary region"]
    #[inline(always)]
    pub const fn set_tmpbase(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tmpbase {
    #[inline(always)]
    fn default() -> Tmpbase {
        Tmpbase(0)
    }
}
impl core::fmt::Debug for Tmpbase {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tmpbase")
            .field("tmpbase", &self.tmpbase())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tmpbase {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tmpbase {{ tmpbase: {=u32:?} }}", self.tmpbase())
    }
}
#[doc = "Temp format"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmpformat(pub u32);
impl Tmpformat {
    #[doc = "Temp Internal format (00: q15; 01:q31; 10:float)"]
    #[must_use]
    #[inline(always)]
    pub const fn tmp_formatint(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Temp Internal format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub const fn set_tmp_formatint(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Temp External format (00: q15; 01:q31; 10:float)"]
    #[must_use]
    #[inline(always)]
    pub const fn tmp_formatext(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Temp External format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub const fn set_tmp_formatext(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Temp Scaler value (for scaled 'q31' formats)"]
    #[must_use]
    #[inline(always)]
    pub const fn tmp_scaler(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Temp Scaler value (for scaled 'q31' formats)"]
    #[inline(always)]
    pub const fn set_tmp_scaler(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Tmpformat {
    #[inline(always)]
    fn default() -> Tmpformat {
        Tmpformat(0)
    }
}
impl core::fmt::Debug for Tmpformat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tmpformat")
            .field("tmp_formatint", &self.tmp_formatint())
            .field("tmp_formatext", &self.tmp_formatext())
            .field("tmp_scaler", &self.tmp_scaler())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tmpformat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tmpformat {{ tmp_formatint: {=u8:?}, tmp_formatext: {=u8:?}, tmp_scaler: {=u8:?} }}",
            self.tmp_formatint(),
            self.tmp_formatext(),
            self.tmp_scaler()
        )
    }
}
