#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "PowerQuad."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Powerquad {
    ptr: *mut u8,
}
unsafe impl Send for Powerquad {}
unsafe impl Sync for Powerquad {}
impl Powerquad {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Output Base."]
    #[inline(always)]
    pub const fn outbase(self) -> crate::pac::common::Reg<Outbase, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Output Format."]
    #[inline(always)]
    pub const fn outformat(self) -> crate::pac::common::Reg<Outformat, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Temporary Base."]
    #[inline(always)]
    pub const fn tmpbase(self) -> crate::pac::common::Reg<Tmpbase, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Temporary Format."]
    #[inline(always)]
    pub const fn tmpformat(self) -> crate::pac::common::Reg<Tmpformat, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Input A Base."]
    #[inline(always)]
    pub const fn inabase(self) -> crate::pac::common::Reg<Inabase, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Input A Format."]
    #[inline(always)]
    pub const fn inaformat(self) -> crate::pac::common::Reg<Inaformat, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Input B Base."]
    #[inline(always)]
    pub const fn inbbase(self) -> crate::pac::common::Reg<Inbbase, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Input B Format."]
    #[inline(always)]
    pub const fn inbformat(self) -> crate::pac::common::Reg<Inbformat, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Control."]
    #[inline(always)]
    pub const fn control(self) -> crate::pac::common::Reg<Control, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Length."]
    #[inline(always)]
    pub const fn length(self) -> crate::pac::common::Reg<Length, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Coprocessor Prescale."]
    #[inline(always)]
    pub const fn cppre(self) -> crate::pac::common::Reg<Cppre, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Miscellaneous."]
    #[inline(always)]
    pub const fn misc(self) -> crate::pac::common::Reg<Misc, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Cursory."]
    #[inline(always)]
    pub const fn cursory(self) -> crate::pac::common::Reg<Cursory, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "CORDIC Input X."]
    #[inline(always)]
    pub const fn cordic_x(self) -> crate::pac::common::Reg<CordicX, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "CORDIC Input Y."]
    #[inline(always)]
    pub const fn cordic_y(self) -> crate::pac::common::Reg<CordicY, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "CORDIC Input Z."]
    #[inline(always)]
    pub const fn cordic_z(self) -> crate::pac::common::Reg<CordicZ, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "Error Status."]
    #[inline(always)]
    pub const fn errstat(self) -> crate::pac::common::Reg<Errstat, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "Interrupt Enable."]
    #[inline(always)]
    pub const fn intren(self) -> crate::pac::common::Reg<Intren, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "Event Enable."]
    #[inline(always)]
    pub const fn eventen(self) -> crate::pac::common::Reg<Eventen, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "Interrupt Status."]
    #[inline(always)]
    pub const fn intrstat(self) -> crate::pac::common::Reg<Intrstat, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "General Purpose Register Bank n."]
    #[inline(always)]
    pub const fn gpreg(self, n: usize) -> crate::pac::common::Reg<Gpreg, crate::pac::common::RW> {
        assert!(n < 16usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 4usize) as _)
        }
    }
    #[doc = "Compute Register Bank n."]
    #[inline(always)]
    pub const fn compreg(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Compreg, crate::pac::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize + n * 4usize) as _)
        }
    }
}
#[doc = "Compute Register Bank n."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Compreg(pub u32);
impl Compreg {
    #[doc = "Compute bank."]
    #[must_use]
    #[inline(always)]
    pub const fn compreg(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Compute bank."]
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
#[doc = "Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Control(pub u32);
impl Control {
    #[doc = "Decode Opcode."]
    #[must_use]
    #[inline(always)]
    pub const fn decode_opcode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Decode Opcode."]
    #[inline(always)]
    pub const fn set_decode_opcode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Decode Machine."]
    #[must_use]
    #[inline(always)]
    pub const fn decode_machine(&self) -> DecodeMachine {
        let val = (self.0 >> 4usize) & 0x0f;
        DecodeMachine::from_bits(val as u8)
    }
    #[doc = "Decode Machine."]
    #[inline(always)]
    pub const fn set_decode_machine(&mut self, val: DecodeMachine) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "Instruction Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn inst_busy(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Instruction Busy."]
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
#[doc = "CORDIC Input X."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CordicX(pub u32);
impl CordicX {
    #[doc = "CORDIC Input X."]
    #[must_use]
    #[inline(always)]
    pub const fn cordic_x(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "CORDIC Input X."]
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
#[doc = "CORDIC Input Y."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CordicY(pub u32);
impl CordicY {
    #[doc = "CORDIC Input Y."]
    #[must_use]
    #[inline(always)]
    pub const fn cordic_y(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "CORDIC Input Y."]
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
#[doc = "CORDIC Input Z."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CordicZ(pub u32);
impl CordicZ {
    #[doc = "CORDIC Input Z."]
    #[must_use]
    #[inline(always)]
    pub const fn cordic_z(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "CORDIC Input Z."]
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
#[doc = "Coprocessor Prescale."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cppre(pub u32);
impl Cppre {
    #[doc = "Prescaling Input."]
    #[must_use]
    #[inline(always)]
    pub const fn cppre_in(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Prescaling Input."]
    #[inline(always)]
    pub const fn set_cppre_in(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Postscaling Output."]
    #[must_use]
    #[inline(always)]
    pub const fn cppre_out(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Postscaling Output."]
    #[inline(always)]
    pub const fn set_cppre_out(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Saturation."]
    #[must_use]
    #[inline(always)]
    pub const fn cppre_sat(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Saturation."]
    #[inline(always)]
    pub const fn set_cppre_sat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Saturation 8."]
    #[must_use]
    #[inline(always)]
    pub const fn cppre_sat8(&self) -> CppreSat8 {
        let val = (self.0 >> 17usize) & 0x01;
        CppreSat8::from_bits(val as u8)
    }
    #[doc = "Saturation 8."]
    #[inline(always)]
    pub const fn set_cppre_sat8(&mut self, val: CppreSat8) {
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
#[doc = "Cursory."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cursory(pub u32);
impl Cursory {
    #[doc = "Cursory Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn cursory(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Cursory Mode."]
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
#[doc = "Error Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Errstat(pub u32);
impl Errstat {
    #[doc = "Floating-point Overflow."]
    #[must_use]
    #[inline(always)]
    pub const fn overflow(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Floating-point Overflow."]
    #[inline(always)]
    pub const fn set_overflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Floating-Point Not-a-Number (NaN)."]
    #[must_use]
    #[inline(always)]
    pub const fn nan(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Floating-Point Not-a-Number (NaN)."]
    #[inline(always)]
    pub const fn set_nan(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Fixed-point Overflow."]
    #[must_use]
    #[inline(always)]
    pub const fn fixedoverflow(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Fixed-point Overflow."]
    #[inline(always)]
    pub const fn set_fixedoverflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Underflow."]
    #[must_use]
    #[inline(always)]
    pub const fn underflow(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Underflow."]
    #[inline(always)]
    pub const fn set_underflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Bus Error."]
    #[must_use]
    #[inline(always)]
    pub const fn buserror(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Error."]
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
#[doc = "Event Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eventen(pub u32);
impl Eventen {
    #[doc = "Event Trigger on Floating-point Overflow."]
    #[must_use]
    #[inline(always)]
    pub const fn event_oflow(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Event Trigger on Floating-point Overflow."]
    #[inline(always)]
    pub const fn set_event_oflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Event Trigger on Floating-Point NaN."]
    #[must_use]
    #[inline(always)]
    pub const fn event_nan(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Event Trigger on Floating-Point NaN."]
    #[inline(always)]
    pub const fn set_event_nan(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Event Trigger on Fixed-point Overflow."]
    #[must_use]
    #[inline(always)]
    pub const fn event_fixed(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Event Trigger on Fixed-point Overflow."]
    #[inline(always)]
    pub const fn set_event_fixed(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Event Trigger on Underflow."]
    #[must_use]
    #[inline(always)]
    pub const fn event_uflow(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Event Trigger on Underflow."]
    #[inline(always)]
    pub const fn set_event_uflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Event Trigger on AHBM Bus Error."]
    #[must_use]
    #[inline(always)]
    pub const fn event_berr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Event Trigger on AHBM Bus Error."]
    #[inline(always)]
    pub const fn set_event_berr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Event Trigger on Instruction Completion."]
    #[must_use]
    #[inline(always)]
    pub const fn event_comp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Event Trigger on Instruction Completion."]
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
#[doc = "General Purpose Register Bank n."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpreg(pub u32);
impl Gpreg {
    #[doc = "General Purpose Bank."]
    #[must_use]
    #[inline(always)]
    pub const fn gpreg(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "General Purpose Bank."]
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
#[doc = "Input A Base."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inabase(pub u32);
impl Inabase {
    #[doc = "Input A Base."]
    #[must_use]
    #[inline(always)]
    pub const fn inabase(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input A Base."]
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
#[doc = "Input A Format."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inaformat(pub u32);
impl Inaformat {
    #[doc = "Input A Internal Format."]
    #[must_use]
    #[inline(always)]
    pub const fn ina_formatint(&self) -> InaFormatint {
        let val = (self.0 >> 0usize) & 0x03;
        InaFormatint::from_bits(val as u8)
    }
    #[doc = "Input A Internal Format."]
    #[inline(always)]
    pub const fn set_ina_formatint(&mut self, val: InaFormatint) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Input A External Format."]
    #[must_use]
    #[inline(always)]
    pub const fn ina_formatext(&self) -> InaFormatext {
        let val = (self.0 >> 4usize) & 0x03;
        InaFormatext::from_bits(val as u8)
    }
    #[doc = "Input A External Format."]
    #[inline(always)]
    pub const fn set_ina_formatext(&mut self, val: InaFormatext) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Input A Scaler Value."]
    #[must_use]
    #[inline(always)]
    pub const fn ina_scaler(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Input A Scaler Value."]
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
#[doc = "Input B Base."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inbbase(pub u32);
impl Inbbase {
    #[doc = "Input B Base."]
    #[must_use]
    #[inline(always)]
    pub const fn inbbase(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Input B Base."]
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
#[doc = "Input B Format."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inbformat(pub u32);
impl Inbformat {
    #[doc = "Input B Internal Format."]
    #[must_use]
    #[inline(always)]
    pub const fn inb_formatint(&self) -> InbFormatint {
        let val = (self.0 >> 0usize) & 0x03;
        InbFormatint::from_bits(val as u8)
    }
    #[doc = "Input B Internal Format."]
    #[inline(always)]
    pub const fn set_inb_formatint(&mut self, val: InbFormatint) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Input B External Format."]
    #[must_use]
    #[inline(always)]
    pub const fn inb_formatext(&self) -> InbFormatext {
        let val = (self.0 >> 4usize) & 0x03;
        InbFormatext::from_bits(val as u8)
    }
    #[doc = "Input B External Format."]
    #[inline(always)]
    pub const fn set_inb_formatext(&mut self, val: InbFormatext) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Input B Scaler."]
    #[must_use]
    #[inline(always)]
    pub const fn inb_scaler(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Input B Scaler."]
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
#[doc = "Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intren(pub u32);
impl Intren {
    #[doc = "Interrupt Floating-point Overflow."]
    #[must_use]
    #[inline(always)]
    pub const fn intr_oflow(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Floating-point Overflow."]
    #[inline(always)]
    pub const fn set_intr_oflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt Floating-point NaN."]
    #[must_use]
    #[inline(always)]
    pub const fn intr_nan(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Floating-point NaN."]
    #[inline(always)]
    pub const fn set_intr_nan(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Interrupt on Fixed-point Overflow."]
    #[must_use]
    #[inline(always)]
    pub const fn intr_fixed(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt on Fixed-point Overflow."]
    #[inline(always)]
    pub const fn set_intr_fixed(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Interrupt on Underflow."]
    #[must_use]
    #[inline(always)]
    pub const fn intr_uflow(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt on Underflow."]
    #[inline(always)]
    pub const fn set_intr_uflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Interrupt on AHBM Bus Error."]
    #[must_use]
    #[inline(always)]
    pub const fn intr_berr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt on AHBM Bus Error."]
    #[inline(always)]
    pub const fn set_intr_berr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Interrupt on Instruction Completion."]
    #[must_use]
    #[inline(always)]
    pub const fn intr_comp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt on Instruction Completion."]
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
#[doc = "Interrupt Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intrstat(pub u32);
impl Intrstat {
    #[doc = "Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn intr_stat(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Status."]
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
#[doc = "Length."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Length(pub u32);
impl Length {
    #[doc = "Instruction length."]
    #[must_use]
    #[inline(always)]
    pub const fn inst_length(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Instruction length."]
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
#[doc = "Miscellaneous."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Misc(pub u32);
impl Misc {
    #[doc = "Scaling Factor."]
    #[must_use]
    #[inline(always)]
    pub const fn inst_misc(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Scaling Factor."]
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
#[doc = "Output Base."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Outbase(pub u32);
impl Outbase {
    #[doc = "Output Region Base Address."]
    #[must_use]
    #[inline(always)]
    pub const fn outbase(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Output Region Base Address."]
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
#[doc = "Output Format."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Outformat(pub u32);
impl Outformat {
    #[doc = "Output Internal Format."]
    #[must_use]
    #[inline(always)]
    pub const fn out_formatint(&self) -> OutFormatint {
        let val = (self.0 >> 0usize) & 0x03;
        OutFormatint::from_bits(val as u8)
    }
    #[doc = "Output Internal Format."]
    #[inline(always)]
    pub const fn set_out_formatint(&mut self, val: OutFormatint) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Output External Format."]
    #[must_use]
    #[inline(always)]
    pub const fn out_formatext(&self) -> OutFormatext {
        let val = (self.0 >> 4usize) & 0x03;
        OutFormatext::from_bits(val as u8)
    }
    #[doc = "Output External Format."]
    #[inline(always)]
    pub const fn set_out_formatext(&mut self, val: OutFormatext) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "8-bit Scaling Value for Result Data."]
    #[must_use]
    #[inline(always)]
    pub const fn out_scaler(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "8-bit Scaling Value for Result Data."]
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
#[doc = "Temporary Base."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmpbase(pub u32);
impl Tmpbase {
    #[doc = "Base Address for the Temporary Region."]
    #[must_use]
    #[inline(always)]
    pub const fn tmpbase(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Base Address for the Temporary Region."]
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
#[doc = "Temporary Format."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmpformat(pub u32);
impl Tmpformat {
    #[doc = "Temporary Internal Format."]
    #[must_use]
    #[inline(always)]
    pub const fn tmp_formatint(&self) -> TmpFormatint {
        let val = (self.0 >> 0usize) & 0x03;
        TmpFormatint::from_bits(val as u8)
    }
    #[doc = "Temporary Internal Format."]
    #[inline(always)]
    pub const fn set_tmp_formatint(&mut self, val: TmpFormatint) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Temporary External Format."]
    #[must_use]
    #[inline(always)]
    pub const fn tmp_formatext(&self) -> TmpFormatext {
        let val = (self.0 >> 4usize) & 0x03;
        TmpFormatext::from_bits(val as u8)
    }
    #[doc = "Temporary External Format."]
    #[inline(always)]
    pub const fn set_tmp_formatext(&mut self, val: TmpFormatext) {
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CppreSat8 {
    #[doc = "8 bits."]
    Sat8Bits = 0x0,
    #[doc = "16 bits."]
    Sat16Bits = 0x01,
}
impl CppreSat8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CppreSat8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CppreSat8 {
    #[inline(always)]
    fn from(val: u8) -> CppreSat8 {
        CppreSat8::from_bits(val)
    }
}
impl From<CppreSat8> for u8 {
    #[inline(always)]
    fn from(val: CppreSat8) -> u8 {
        CppreSat8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DecodeMachine {
    #[doc = "Coprocessor."]
    Coprocessor = 0x0,
    #[doc = "Matrix engine."]
    Matrix = 0x01,
    #[doc = "Transform engine."]
    Transform = 0x02,
    #[doc = "Filter engine."]
    Filter = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CORDIC engine."]
    Cordic = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl DecodeMachine {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DecodeMachine {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DecodeMachine {
    #[inline(always)]
    fn from(val: u8) -> DecodeMachine {
        DecodeMachine::from_bits(val)
    }
}
impl From<DecodeMachine> for u8 {
    #[inline(always)]
    fn from(val: DecodeMachine) -> u8 {
        DecodeMachine::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InaFormatext {
    #[doc = "Q15 16-bit fixed-point integer."]
    Q15 = 0x0,
    #[doc = "Q31 32-bit fixed-point integer."]
    Q31 = 0x01,
    #[doc = "F32 32-bit floating-point format."]
    Float = 0x02,
    _RESERVED_3 = 0x03,
}
impl InaFormatext {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InaFormatext {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InaFormatext {
    #[inline(always)]
    fn from(val: u8) -> InaFormatext {
        InaFormatext::from_bits(val)
    }
}
impl From<InaFormatext> for u8 {
    #[inline(always)]
    fn from(val: InaFormatext) -> u8 {
        InaFormatext::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InaFormatint {
    #[doc = "Q15 16-bit fixed-point integer."]
    Q15 = 0x0,
    #[doc = "Q31 32-bit fixed-point integer."]
    Q31 = 0x01,
    #[doc = "F32 32-bit floating-point format."]
    Float = 0x02,
    _RESERVED_3 = 0x03,
}
impl InaFormatint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InaFormatint {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InaFormatint {
    #[inline(always)]
    fn from(val: u8) -> InaFormatint {
        InaFormatint::from_bits(val)
    }
}
impl From<InaFormatint> for u8 {
    #[inline(always)]
    fn from(val: InaFormatint) -> u8 {
        InaFormatint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InbFormatext {
    #[doc = "Q15 16-bit fixed-point integer."]
    Q15 = 0x0,
    #[doc = "Q31 32-bit fixed-point integer."]
    Q31 = 0x01,
    #[doc = "F32 32-bit floating-point format."]
    Float = 0x02,
    _RESERVED_3 = 0x03,
}
impl InbFormatext {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InbFormatext {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InbFormatext {
    #[inline(always)]
    fn from(val: u8) -> InbFormatext {
        InbFormatext::from_bits(val)
    }
}
impl From<InbFormatext> for u8 {
    #[inline(always)]
    fn from(val: InbFormatext) -> u8 {
        InbFormatext::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InbFormatint {
    #[doc = "Q15 16-bit fixed-point integer."]
    Q15 = 0x0,
    #[doc = "Q31 32-bit fixed-point integer."]
    Q31 = 0x01,
    #[doc = "F32 32-bit floating-point format."]
    Float = 0x02,
    _RESERVED_3 = 0x03,
}
impl InbFormatint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InbFormatint {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InbFormatint {
    #[inline(always)]
    fn from(val: u8) -> InbFormatint {
        InbFormatint::from_bits(val)
    }
}
impl From<InbFormatint> for u8 {
    #[inline(always)]
    fn from(val: InbFormatint) -> u8 {
        InbFormatint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OutFormatext {
    #[doc = "Q15 16-bit fixed-point integer."]
    Q15 = 0x0,
    #[doc = "Q31 32-bit fixed-point integer."]
    Q31 = 0x01,
    #[doc = "F32 32-bit floating-point format."]
    Float = 0x02,
    _RESERVED_3 = 0x03,
}
impl OutFormatext {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutFormatext {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutFormatext {
    #[inline(always)]
    fn from(val: u8) -> OutFormatext {
        OutFormatext::from_bits(val)
    }
}
impl From<OutFormatext> for u8 {
    #[inline(always)]
    fn from(val: OutFormatext) -> u8 {
        OutFormatext::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OutFormatint {
    #[doc = "Q15 16-bit fixed-point integer."]
    Q15 = 0x0,
    #[doc = "Q31 32-bit fixed-point integer."]
    Q31 = 0x01,
    #[doc = "F32 32-bit floating-point format."]
    Float = 0x02,
    _RESERVED_3 = 0x03,
}
impl OutFormatint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutFormatint {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutFormatint {
    #[inline(always)]
    fn from(val: u8) -> OutFormatint {
        OutFormatint::from_bits(val)
    }
}
impl From<OutFormatint> for u8 {
    #[inline(always)]
    fn from(val: OutFormatint) -> u8 {
        OutFormatint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TmpFormatext {
    #[doc = "Q15 16-bit fixed-point integer."]
    Q15 = 0x0,
    #[doc = "Q31 32-bit fixed-point integer."]
    Q31 = 0x01,
    #[doc = "F32 32-bit floating-point format."]
    Float = 0x02,
    _RESERVED_3 = 0x03,
}
impl TmpFormatext {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TmpFormatext {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TmpFormatext {
    #[inline(always)]
    fn from(val: u8) -> TmpFormatext {
        TmpFormatext::from_bits(val)
    }
}
impl From<TmpFormatext> for u8 {
    #[inline(always)]
    fn from(val: TmpFormatext) -> u8 {
        TmpFormatext::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TmpFormatint {
    #[doc = "Q15 16-bit fixed-point integer."]
    Q15 = 0x0,
    #[doc = "Q31 32-bit fixed-point integer."]
    Q31 = 0x01,
    #[doc = "F32 32-bit floating-point format."]
    Float = 0x02,
    _RESERVED_3 = 0x03,
}
impl TmpFormatint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TmpFormatint {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TmpFormatint {
    #[inline(always)]
    fn from(val: u8) -> TmpFormatint {
        TmpFormatint::from_bits(val)
    }
}
impl From<TmpFormatint> for u8 {
    #[inline(always)]
    fn from(val: TmpFormatint) -> u8 {
        TmpFormatint::to_bits(val)
    }
}
