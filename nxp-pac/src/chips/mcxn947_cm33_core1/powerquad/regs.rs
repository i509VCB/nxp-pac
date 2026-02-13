#[doc = "Compute Register Bank n"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Compreg(pub u32);
impl Compreg {
    #[doc = "Compute bank"]
    #[must_use]
    #[inline(always)]
    pub const fn compreg(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Compute bank"]
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
#[doc = "Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Control(pub u32);
impl Control {
    #[doc = "Decode Opcode"]
    #[must_use]
    #[inline(always)]
    pub const fn decode_opcode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Decode Opcode"]
    #[inline(always)]
    pub const fn set_decode_opcode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Decode Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn decode_machine(&self) -> super::vals::DecodeMachine {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::DecodeMachine::from_bits(val as u8)
    }
    #[doc = "Decode Machine"]
    #[inline(always)]
    pub const fn set_decode_machine(&mut self, val: super::vals::DecodeMachine) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "Instruction Busy"]
    #[must_use]
    #[inline(always)]
    pub const fn inst_busy(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Instruction Busy"]
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
            "Control {{ decode_opcode: {=u8:?}, decode_machine: {:?}, inst_busy: {=bool:?} }}",
            self.decode_opcode(),
            self.decode_machine(),
            self.inst_busy()
        )
    }
}
#[doc = "CORDIC Input X"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CordicX(pub u32);
impl CordicX {
    #[doc = "CORDIC Input X"]
    #[must_use]
    #[inline(always)]
    pub const fn cordic_x(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "CORDIC Input X"]
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
#[doc = "CORDIC Input Y"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CordicY(pub u32);
impl CordicY {
    #[doc = "CORDIC Input Y"]
    #[must_use]
    #[inline(always)]
    pub const fn cordic_y(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "CORDIC Input Y"]
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
#[doc = "CORDIC Input Z"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CordicZ(pub u32);
impl CordicZ {
    #[doc = "CORDIC Input Z"]
    #[must_use]
    #[inline(always)]
    pub const fn cordic_z(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "CORDIC Input Z"]
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
#[doc = "Coprocessor Prescale"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cppre(pub u32);
impl Cppre {
    #[doc = "Prescaling Input"]
    #[must_use]
    #[inline(always)]
    pub const fn cppre_in(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Prescaling Input"]
    #[inline(always)]
    pub const fn set_cppre_in(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Postscaling Output"]
    #[must_use]
    #[inline(always)]
    pub const fn cppre_out(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Postscaling Output"]
    #[inline(always)]
    pub const fn set_cppre_out(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Saturation"]
    #[must_use]
    #[inline(always)]
    pub const fn cppre_sat(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Saturation"]
    #[inline(always)]
    pub const fn set_cppre_sat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Saturation 8"]
    #[must_use]
    #[inline(always)]
    pub const fn cppre_sat8(&self) -> super::vals::CppreSat8 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::CppreSat8::from_bits(val as u8)
    }
    #[doc = "Saturation 8"]
    #[inline(always)]
    pub const fn set_cppre_sat8(&mut self, val: super::vals::CppreSat8) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
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
            "Cppre {{ cppre_in: {=u8:?}, cppre_out: {=u8:?}, cppre_sat: {=bool:?}, cppre_sat8: {:?} }}",
            self.cppre_in(),
            self.cppre_out(),
            self.cppre_sat(),
            self.cppre_sat8()
        )
    }
}
#[doc = "Cursory"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cursory(pub u32);
impl Cursory {
    #[doc = "Cursory Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn cursory(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Cursory Mode"]
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
#[doc = "Error Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Errstat(pub u32);
impl Errstat {
    #[doc = "Floating-point Overflow"]
    #[must_use]
    #[inline(always)]
    pub const fn overflow(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Floating-point Overflow"]
    #[inline(always)]
    pub const fn set_overflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Floating-Point Not-a-Number (NaN)"]
    #[must_use]
    #[inline(always)]
    pub const fn nan(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Floating-Point Not-a-Number (NaN)"]
    #[inline(always)]
    pub const fn set_nan(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Fixed-point Overflow"]
    #[must_use]
    #[inline(always)]
    pub const fn fixedoverflow(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Fixed-point Overflow"]
    #[inline(always)]
    pub const fn set_fixedoverflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Underflow"]
    #[must_use]
    #[inline(always)]
    pub const fn underflow(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Underflow"]
    #[inline(always)]
    pub const fn set_underflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Bus Error"]
    #[must_use]
    #[inline(always)]
    pub const fn buserror(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Error"]
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
#[doc = "Event Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eventen(pub u32);
impl Eventen {
    #[doc = "Event Trigger on Floating-point Overflow"]
    #[must_use]
    #[inline(always)]
    pub const fn event_oflow(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Event Trigger on Floating-point Overflow"]
    #[inline(always)]
    pub const fn set_event_oflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Event Trigger on Floating-Point NaN"]
    #[must_use]
    #[inline(always)]
    pub const fn event_nan(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Event Trigger on Floating-Point NaN"]
    #[inline(always)]
    pub const fn set_event_nan(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Event Trigger on Fixed-point Overflow"]
    #[must_use]
    #[inline(always)]
    pub const fn event_fixed(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Event Trigger on Fixed-point Overflow"]
    #[inline(always)]
    pub const fn set_event_fixed(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Event Trigger on Underflow"]
    #[must_use]
    #[inline(always)]
    pub const fn event_uflow(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Event Trigger on Underflow"]
    #[inline(always)]
    pub const fn set_event_uflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Event Trigger on AHBM Bus Error"]
    #[must_use]
    #[inline(always)]
    pub const fn event_berr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Event Trigger on AHBM Bus Error"]
    #[inline(always)]
    pub const fn set_event_berr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Event Trigger on Instruction Completion"]
    #[must_use]
    #[inline(always)]
    pub const fn event_comp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Event Trigger on Instruction Completion"]
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
#[doc = "General Purpose Register Bank n"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpreg(pub u32);
impl Gpreg {
    #[doc = "General Purpose Bank"]
    #[must_use]
    #[inline(always)]
    pub const fn gpreg(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "General Purpose Bank"]
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
#[doc = "Input A Base"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inabase(pub u32);
impl Inabase {
    #[doc = "Input A Base"]
    #[must_use]
    #[inline(always)]
    pub const fn inabase(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input A Base"]
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
#[doc = "Input A Format"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inaformat(pub u32);
impl Inaformat {
    #[doc = "Input A Internal Format"]
    #[must_use]
    #[inline(always)]
    pub const fn ina_formatint(&self) -> super::vals::InaFormatint {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::InaFormatint::from_bits(val as u8)
    }
    #[doc = "Input A Internal Format"]
    #[inline(always)]
    pub const fn set_ina_formatint(&mut self, val: super::vals::InaFormatint) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Input A External Format"]
    #[must_use]
    #[inline(always)]
    pub const fn ina_formatext(&self) -> super::vals::InaFormatext {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::InaFormatext::from_bits(val as u8)
    }
    #[doc = "Input A External Format"]
    #[inline(always)]
    pub const fn set_ina_formatext(&mut self, val: super::vals::InaFormatext) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Input A Scaler Value"]
    #[must_use]
    #[inline(always)]
    pub const fn ina_scaler(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Input A Scaler Value"]
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
            "Inaformat {{ ina_formatint: {:?}, ina_formatext: {:?}, ina_scaler: {=u8:?} }}",
            self.ina_formatint(),
            self.ina_formatext(),
            self.ina_scaler()
        )
    }
}
#[doc = "Input B Base"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inbbase(pub u32);
impl Inbbase {
    #[doc = "Input B Base"]
    #[must_use]
    #[inline(always)]
    pub const fn inbbase(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input B Base"]
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
#[doc = "Input B Format"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inbformat(pub u32);
impl Inbformat {
    #[doc = "Input B Internal Format"]
    #[must_use]
    #[inline(always)]
    pub const fn inb_formatint(&self) -> super::vals::InbFormatint {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::InbFormatint::from_bits(val as u8)
    }
    #[doc = "Input B Internal Format"]
    #[inline(always)]
    pub const fn set_inb_formatint(&mut self, val: super::vals::InbFormatint) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Input B External Format"]
    #[must_use]
    #[inline(always)]
    pub const fn inb_formatext(&self) -> super::vals::InbFormatext {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::InbFormatext::from_bits(val as u8)
    }
    #[doc = "Input B External Format"]
    #[inline(always)]
    pub const fn set_inb_formatext(&mut self, val: super::vals::InbFormatext) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Input B Scaler"]
    #[must_use]
    #[inline(always)]
    pub const fn inb_scaler(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Input B Scaler"]
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
            "Inbformat {{ inb_formatint: {:?}, inb_formatext: {:?}, inb_scaler: {=u8:?} }}",
            self.inb_formatint(),
            self.inb_formatext(),
            self.inb_scaler()
        )
    }
}
#[doc = "Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intren(pub u32);
impl Intren {
    #[doc = "Interrupt Floating-point Overflow"]
    #[must_use]
    #[inline(always)]
    pub const fn intr_oflow(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Floating-point Overflow"]
    #[inline(always)]
    pub const fn set_intr_oflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt Floating-point NaN"]
    #[must_use]
    #[inline(always)]
    pub const fn intr_nan(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Floating-point NaN"]
    #[inline(always)]
    pub const fn set_intr_nan(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Interrupt on Fixed-point Overflow"]
    #[must_use]
    #[inline(always)]
    pub const fn intr_fixed(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt on Fixed-point Overflow"]
    #[inline(always)]
    pub const fn set_intr_fixed(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Interrupt on Underflow"]
    #[must_use]
    #[inline(always)]
    pub const fn intr_uflow(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt on Underflow"]
    #[inline(always)]
    pub const fn set_intr_uflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Interrupt on AHBM Bus Error"]
    #[must_use]
    #[inline(always)]
    pub const fn intr_berr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt on AHBM Bus Error"]
    #[inline(always)]
    pub const fn set_intr_berr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Interrupt on Instruction Completion"]
    #[must_use]
    #[inline(always)]
    pub const fn intr_comp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt on Instruction Completion"]
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
#[doc = "Interrupt Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intrstat(pub u32);
impl Intrstat {
    #[doc = "Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn intr_stat(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Status"]
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
#[doc = "Length"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Length(pub u32);
impl Length {
    #[doc = "Instruction length"]
    #[must_use]
    #[inline(always)]
    pub const fn inst_length(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Instruction length"]
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
#[doc = "Miscellaneous"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Misc(pub u32);
impl Misc {
    #[doc = "Scaling Factor"]
    #[must_use]
    #[inline(always)]
    pub const fn inst_misc(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Scaling Factor"]
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
#[doc = "Output Base"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Outbase(pub u32);
impl Outbase {
    #[doc = "Output Region Base Address"]
    #[must_use]
    #[inline(always)]
    pub const fn outbase(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Output Region Base Address"]
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
#[doc = "Output Format"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Outformat(pub u32);
impl Outformat {
    #[doc = "Output Internal Format"]
    #[must_use]
    #[inline(always)]
    pub const fn out_formatint(&self) -> super::vals::OutFormatint {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::OutFormatint::from_bits(val as u8)
    }
    #[doc = "Output Internal Format"]
    #[inline(always)]
    pub const fn set_out_formatint(&mut self, val: super::vals::OutFormatint) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Output External Format"]
    #[must_use]
    #[inline(always)]
    pub const fn out_formatext(&self) -> super::vals::OutFormatext {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::OutFormatext::from_bits(val as u8)
    }
    #[doc = "Output External Format"]
    #[inline(always)]
    pub const fn set_out_formatext(&mut self, val: super::vals::OutFormatext) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "8-bit Scaling Value for Result Data"]
    #[must_use]
    #[inline(always)]
    pub const fn out_scaler(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "8-bit Scaling Value for Result Data"]
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
            "Outformat {{ out_formatint: {:?}, out_formatext: {:?}, out_scaler: {=u8:?} }}",
            self.out_formatint(),
            self.out_formatext(),
            self.out_scaler()
        )
    }
}
#[doc = "Temporary Base"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmpbase(pub u32);
impl Tmpbase {
    #[doc = "Base Address for the Temporary Region"]
    #[must_use]
    #[inline(always)]
    pub const fn tmpbase(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Base Address for the Temporary Region"]
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
#[doc = "Temporary Format"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmpformat(pub u32);
impl Tmpformat {
    #[doc = "Temporary Internal Format"]
    #[must_use]
    #[inline(always)]
    pub const fn tmp_formatint(&self) -> super::vals::TmpFormatint {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::TmpFormatint::from_bits(val as u8)
    }
    #[doc = "Temporary Internal Format"]
    #[inline(always)]
    pub const fn set_tmp_formatint(&mut self, val: super::vals::TmpFormatint) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Temporary External Format"]
    #[must_use]
    #[inline(always)]
    pub const fn tmp_formatext(&self) -> super::vals::TmpFormatext {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::TmpFormatext::from_bits(val as u8)
    }
    #[doc = "Temporary External Format"]
    #[inline(always)]
    pub const fn set_tmp_formatext(&mut self, val: super::vals::TmpFormatext) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Scaling Value for Temporary Data."]
    #[must_use]
    #[inline(always)]
    pub const fn tmp_scaler(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Scaling Value for Temporary Data."]
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
            "Tmpformat {{ tmp_formatint: {:?}, tmp_formatext: {:?}, tmp_scaler: {=u8:?} }}",
            self.tmp_formatint(),
            self.tmp_formatext(),
            self.tmp_scaler()
        )
    }
}
