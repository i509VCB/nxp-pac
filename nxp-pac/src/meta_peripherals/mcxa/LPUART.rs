#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "LPUART."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart {
    ptr: *mut u8,
}
unsafe impl Send for Lpuart {}
unsafe impl Sync for Lpuart {}
impl Lpuart {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Version ID."]
    #[inline(always)]
    pub const fn verid(self) -> crate::pac::common::Reg<Verid, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Parameter."]
    #[inline(always)]
    pub const fn param(self) -> crate::pac::common::Reg<Param, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Global."]
    #[inline(always)]
    pub const fn global(self) -> crate::pac::common::Reg<Global, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Pin Configuration."]
    #[inline(always)]
    pub const fn pincfg(self) -> crate::pac::common::Reg<Pincfg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Baud Rate."]
    #[inline(always)]
    pub const fn baud(self) -> crate::pac::common::Reg<Baud, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Status."]
    #[inline(always)]
    pub const fn stat(self) -> crate::pac::common::Reg<Stat, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Control."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::pac::common::Reg<Ctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Data."]
    #[inline(always)]
    pub const fn data(self) -> crate::pac::common::Reg<Data, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Match Address."]
    #[inline(always)]
    pub const fn match_(self) -> crate::pac::common::Reg<Match, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "MODEM IrDA."]
    #[inline(always)]
    pub const fn modir(self) -> crate::pac::common::Reg<Modir, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "FIFO."]
    #[inline(always)]
    pub const fn fifo(self) -> crate::pac::common::Reg<Fifo, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Watermark."]
    #[inline(always)]
    pub const fn water(self) -> crate::pac::common::Reg<Water, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Data Read-Only."]
    #[inline(always)]
    pub const fn dataro(self) -> crate::pac::common::Reg<Dataro, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
}
#[doc = "Baud Rate."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Baud(pub u32);
impl Baud {
    #[doc = "Baud Rate Modulo Divisor."]
    #[must_use]
    #[inline(always)]
    pub const fn sbr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "Baud Rate Modulo Divisor."]
    #[inline(always)]
    pub const fn set_sbr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
    #[doc = "Stop Bit Number Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sbns(&self) -> Sbns {
        let val = (self.0 >> 13usize) & 0x01;
        Sbns::from_bits(val as u8)
    }
    #[doc = "Stop Bit Number Select."]
    #[inline(always)]
    pub const fn set_sbns(&mut self, val: Sbns) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "RX Input Active Edge Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rxedgie(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "RX Input Active Edge Interrupt Enable."]
    #[inline(always)]
    pub const fn set_rxedgie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "LIN Break Detect Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lbkdie(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "LIN Break Detect Interrupt Enable."]
    #[inline(always)]
    pub const fn set_lbkdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Resynchronization Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn resyncdis(&self) -> Resyncdis {
        let val = (self.0 >> 16usize) & 0x01;
        Resyncdis::from_bits(val as u8)
    }
    #[doc = "Resynchronization Disable."]
    #[inline(always)]
    pub const fn set_resyncdis(&mut self, val: Resyncdis) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Both Edge Sampling."]
    #[must_use]
    #[inline(always)]
    pub const fn bothedge(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Both Edge Sampling."]
    #[inline(always)]
    pub const fn set_bothedge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Match Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn matcfg(&self) -> Matcfg {
        let val = (self.0 >> 18usize) & 0x03;
        Matcfg::from_bits(val as u8)
    }
    #[doc = "Match Configuration."]
    #[inline(always)]
    pub const fn set_matcfg(&mut self, val: Matcfg) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Receiver Idle DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ridmae(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver Idle DMA Enable."]
    #[inline(always)]
    pub const fn set_ridmae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Receiver Full DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rdmae(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver Full DMA Enable."]
    #[inline(always)]
    pub const fn set_rdmae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Reduced Oversample Mode Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn hlfm(&self) -> Hlfm {
        let val = (self.0 >> 22usize) & 0x01;
        Hlfm::from_bits(val as u8)
    }
    #[doc = "Reduced Oversample Mode Selection."]
    #[inline(always)]
    pub const fn set_hlfm(&mut self, val: Hlfm) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Transmitter DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tdmae(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter DMA Enable."]
    #[inline(always)]
    pub const fn set_tdmae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Oversampling Ratio."]
    #[must_use]
    #[inline(always)]
    pub const fn osr(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "Oversampling Ratio."]
    #[inline(always)]
    pub const fn set_osr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "10-Bit Mode Select."]
    #[must_use]
    #[inline(always)]
    pub const fn m10(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "10-Bit Mode Select."]
    #[inline(always)]
    pub const fn set_m10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Match Address Mode Enable 2."]
    #[must_use]
    #[inline(always)]
    pub const fn maen2(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Match Address Mode Enable 2."]
    #[inline(always)]
    pub const fn set_maen2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Match Address Mode Enable 1."]
    #[must_use]
    #[inline(always)]
    pub const fn maen1(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Match Address Mode Enable 1."]
    #[inline(always)]
    pub const fn set_maen1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Baud {
    #[inline(always)]
    fn default() -> Baud {
        Baud(0)
    }
}
impl core::fmt::Debug for Baud {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Baud")
            .field("sbr", &self.sbr())
            .field("sbns", &self.sbns())
            .field("rxedgie", &self.rxedgie())
            .field("lbkdie", &self.lbkdie())
            .field("resyncdis", &self.resyncdis())
            .field("bothedge", &self.bothedge())
            .field("matcfg", &self.matcfg())
            .field("ridmae", &self.ridmae())
            .field("rdmae", &self.rdmae())
            .field("hlfm", &self.hlfm())
            .field("tdmae", &self.tdmae())
            .field("osr", &self.osr())
            .field("m10", &self.m10())
            .field("maen2", &self.maen2())
            .field("maen1", &self.maen1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Baud {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Baud {{ sbr: {=u16:?}, sbns: {:?}, rxedgie: {=bool:?}, lbkdie: {=bool:?}, resyncdis: {:?}, bothedge: {=bool:?}, matcfg: {:?}, ridmae: {=bool:?}, rdmae: {=bool:?}, hlfm: {:?}, tdmae: {=bool:?}, osr: {=u8:?}, m10: {=bool:?}, maen2: {=bool:?}, maen1: {=bool:?} }}",
            self.sbr(),
            self.sbns(),
            self.rxedgie(),
            self.lbkdie(),
            self.resyncdis(),
            self.bothedge(),
            self.matcfg(),
            self.ridmae(),
            self.rdmae(),
            self.hlfm(),
            self.tdmae(),
            self.osr(),
            self.m10(),
            self.maen2(),
            self.maen1()
        )
    }
}
#[doc = "Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Parity Type."]
    #[must_use]
    #[inline(always)]
    pub const fn pt(&self) -> Pt {
        let val = (self.0 >> 0usize) & 0x01;
        Pt::from_bits(val as u8)
    }
    #[doc = "Parity Type."]
    #[inline(always)]
    pub const fn set_pt(&mut self, val: Pt) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Parity Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Parity Enable."]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Idle Line Type Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ilt(&self) -> Ilt {
        let val = (self.0 >> 2usize) & 0x01;
        Ilt::from_bits(val as u8)
    }
    #[doc = "Idle Line Type Select."]
    #[inline(always)]
    pub const fn set_ilt(&mut self, val: Ilt) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Receiver Wake-Up Method Select."]
    #[must_use]
    #[inline(always)]
    pub const fn wake(&self) -> Wake {
        let val = (self.0 >> 3usize) & 0x01;
        Wake::from_bits(val as u8)
    }
    #[doc = "Receiver Wake-Up Method Select."]
    #[inline(always)]
    pub const fn set_wake(&mut self, val: Wake) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "9-Bit Or 8-Bit Mode Select."]
    #[must_use]
    #[inline(always)]
    pub const fn m(&self) -> M {
        let val = (self.0 >> 4usize) & 0x01;
        M::from_bits(val as u8)
    }
    #[doc = "9-Bit Or 8-Bit Mode Select."]
    #[inline(always)]
    pub const fn set_m(&mut self, val: M) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Receiver Source Select."]
    #[must_use]
    #[inline(always)]
    pub const fn rsrc(&self) -> Rsrc {
        let val = (self.0 >> 5usize) & 0x01;
        Rsrc::from_bits(val as u8)
    }
    #[doc = "Receiver Source Select."]
    #[inline(always)]
    pub const fn set_rsrc(&mut self, val: Rsrc) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Doze Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn dozeen(&self) -> Dozeen {
        let val = (self.0 >> 6usize) & 0x01;
        Dozeen::from_bits(val as u8)
    }
    #[doc = "Doze Mode."]
    #[inline(always)]
    pub const fn set_dozeen(&mut self, val: Dozeen) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Loop Mode Select."]
    #[must_use]
    #[inline(always)]
    pub const fn loops(&self) -> Loops {
        let val = (self.0 >> 7usize) & 0x01;
        Loops::from_bits(val as u8)
    }
    #[doc = "Loop Mode Select."]
    #[inline(always)]
    pub const fn set_loops(&mut self, val: Loops) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Idle Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn idlecfg(&self) -> Idlecfg {
        let val = (self.0 >> 8usize) & 0x07;
        Idlecfg::from_bits(val as u8)
    }
    #[doc = "Idle Configuration."]
    #[inline(always)]
    pub const fn set_idlecfg(&mut self, val: Idlecfg) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "7-Bit Mode Select."]
    #[must_use]
    #[inline(always)]
    pub const fn m7(&self) -> M7 {
        let val = (self.0 >> 11usize) & 0x01;
        M7::from_bits(val as u8)
    }
    #[doc = "7-Bit Mode Select."]
    #[inline(always)]
    pub const fn set_m7(&mut self, val: M7) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "TXD and RXD Pin Swap."]
    #[must_use]
    #[inline(always)]
    pub const fn swap(&self) -> Swap {
        let val = (self.0 >> 12usize) & 0x01;
        Swap::from_bits(val as u8)
    }
    #[doc = "TXD and RXD Pin Swap."]
    #[inline(always)]
    pub const fn set_swap(&mut self, val: Swap) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Early Sample Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn erlsmp(&self) -> Erlsmp {
        let val = (self.0 >> 13usize) & 0x01;
        Erlsmp::from_bits(val as u8)
    }
    #[doc = "Early Sample Selection."]
    #[inline(always)]
    pub const fn set_erlsmp(&mut self, val: Erlsmp) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Match 2 (MA2F) Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ma2ie(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Match 2 (MA2F) Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ma2ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Match 1 (MA1F) Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ma1ie(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Match 1 (MA1F) Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ma1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Send Break."]
    #[must_use]
    #[inline(always)]
    pub const fn sbk(&self) -> Sbk {
        let val = (self.0 >> 16usize) & 0x01;
        Sbk::from_bits(val as u8)
    }
    #[doc = "Send Break."]
    #[inline(always)]
    pub const fn set_sbk(&mut self, val: Sbk) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Receiver Wake-Up Control."]
    #[must_use]
    #[inline(always)]
    pub const fn rwu(&self) -> Rwu {
        let val = (self.0 >> 17usize) & 0x01;
        Rwu::from_bits(val as u8)
    }
    #[doc = "Receiver Wake-Up Control."]
    #[inline(always)]
    pub const fn set_rwu(&mut self, val: Rwu) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Receiver Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn re(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver Enable."]
    #[inline(always)]
    pub const fn set_re(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Transmitter Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn te(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter Enable."]
    #[inline(always)]
    pub const fn set_te(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Idle Line Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ilie(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Idle Line Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ilie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Receiver Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rie(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver Interrupt Enable."]
    #[inline(always)]
    pub const fn set_rie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Transmission Complete Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tcie(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission Complete Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tcie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Transmit Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tie(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Parity Error Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn peie(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Parity Error Interrupt Enable."]
    #[inline(always)]
    pub const fn set_peie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Framing Error Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn feie(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Framing Error Interrupt Enable."]
    #[inline(always)]
    pub const fn set_feie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Noise Error Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn neie(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Noise Error Interrupt Enable."]
    #[inline(always)]
    pub const fn set_neie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Overrun Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn orie(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun Interrupt Enable."]
    #[inline(always)]
    pub const fn set_orie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Transmit Data Inversion."]
    #[must_use]
    #[inline(always)]
    pub const fn txinv(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Data Inversion."]
    #[inline(always)]
    pub const fn set_txinv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "TXD Pin Direction in Single-Wire Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn txdir(&self) -> Txdir {
        let val = (self.0 >> 29usize) & 0x01;
        Txdir::from_bits(val as u8)
    }
    #[doc = "TXD Pin Direction in Single-Wire Mode."]
    #[inline(always)]
    pub const fn set_txdir(&mut self, val: Txdir) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Receive Bit 9 Transmit Bit 8."]
    #[must_use]
    #[inline(always)]
    pub const fn r9t8(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Bit 9 Transmit Bit 8."]
    #[inline(always)]
    pub const fn set_r9t8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Receive Bit 8 Transmit Bit 9."]
    #[must_use]
    #[inline(always)]
    pub const fn r8t9(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Bit 8 Transmit Bit 9."]
    #[inline(always)]
    pub const fn set_r8t9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
impl core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl")
            .field("pt", &self.pt())
            .field("pe", &self.pe())
            .field("ilt", &self.ilt())
            .field("wake", &self.wake())
            .field("m", &self.m())
            .field("rsrc", &self.rsrc())
            .field("dozeen", &self.dozeen())
            .field("loops", &self.loops())
            .field("idlecfg", &self.idlecfg())
            .field("m7", &self.m7())
            .field("swap", &self.swap())
            .field("erlsmp", &self.erlsmp())
            .field("ma2ie", &self.ma2ie())
            .field("ma1ie", &self.ma1ie())
            .field("sbk", &self.sbk())
            .field("rwu", &self.rwu())
            .field("re", &self.re())
            .field("te", &self.te())
            .field("ilie", &self.ilie())
            .field("rie", &self.rie())
            .field("tcie", &self.tcie())
            .field("tie", &self.tie())
            .field("peie", &self.peie())
            .field("feie", &self.feie())
            .field("neie", &self.neie())
            .field("orie", &self.orie())
            .field("txinv", &self.txinv())
            .field("txdir", &self.txdir())
            .field("r9t8", &self.r9t8())
            .field("r8t9", &self.r8t9())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ pt: {:?}, pe: {=bool:?}, ilt: {:?}, wake: {:?}, m: {:?}, rsrc: {:?}, dozeen: {:?}, loops: {:?}, idlecfg: {:?}, m7: {:?}, swap: {:?}, erlsmp: {:?}, ma2ie: {=bool:?}, ma1ie: {=bool:?}, sbk: {:?}, rwu: {:?}, re: {=bool:?}, te: {=bool:?}, ilie: {=bool:?}, rie: {=bool:?}, tcie: {=bool:?}, tie: {=bool:?}, peie: {=bool:?}, feie: {=bool:?}, neie: {=bool:?}, orie: {=bool:?}, txinv: {=bool:?}, txdir: {:?}, r9t8: {=bool:?}, r8t9: {=bool:?} }}",
            self.pt(),
            self.pe(),
            self.ilt(),
            self.wake(),
            self.m(),
            self.rsrc(),
            self.dozeen(),
            self.loops(),
            self.idlecfg(),
            self.m7(),
            self.swap(),
            self.erlsmp(),
            self.ma2ie(),
            self.ma1ie(),
            self.sbk(),
            self.rwu(),
            self.re(),
            self.te(),
            self.ilie(),
            self.rie(),
            self.tcie(),
            self.tie(),
            self.peie(),
            self.feie(),
            self.neie(),
            self.orie(),
            self.txinv(),
            self.txdir(),
            self.r9t8(),
            self.r8t9()
        )
    }
}
#[doc = "Data."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Data(pub u32);
impl Data {
    #[doc = "Read receive FIFO bit 0 or write transmit FIFO bit 0."]
    #[must_use]
    #[inline(always)]
    pub const fn r0t0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read receive FIFO bit 0 or write transmit FIFO bit 0."]
    #[inline(always)]
    pub const fn set_r0t0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Read receive FIFO bit 1 or write transmit FIFO bit 1."]
    #[must_use]
    #[inline(always)]
    pub const fn r1t1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Read receive FIFO bit 1 or write transmit FIFO bit 1."]
    #[inline(always)]
    pub const fn set_r1t1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Read receive FIFO bit 2 or write transmit FIFO bit 2."]
    #[must_use]
    #[inline(always)]
    pub const fn r2t2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Read receive FIFO bit 2 or write transmit FIFO bit 2."]
    #[inline(always)]
    pub const fn set_r2t2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Read receive FIFO bit 3 or write transmit FIFO bit 3."]
    #[must_use]
    #[inline(always)]
    pub const fn r3t3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Read receive FIFO bit 3 or write transmit FIFO bit 3."]
    #[inline(always)]
    pub const fn set_r3t3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Read receive FIFO bit 4 or write transmit FIFO bit 4."]
    #[must_use]
    #[inline(always)]
    pub const fn r4t4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Read receive FIFO bit 4 or write transmit FIFO bit 4."]
    #[inline(always)]
    pub const fn set_r4t4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Read receive FIFO bit 5 or write transmit FIFO bit 5."]
    #[must_use]
    #[inline(always)]
    pub const fn r5t5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Read receive FIFO bit 5 or write transmit FIFO bit 5."]
    #[inline(always)]
    pub const fn set_r5t5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Read receive FIFO bit 6 or write transmit FIFO bit 6."]
    #[must_use]
    #[inline(always)]
    pub const fn r6t6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Read receive FIFO bit 6 or write transmit FIFO bit 6."]
    #[inline(always)]
    pub const fn set_r6t6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Read receive FIFO bit 7 or write transmit FIFO bit 7."]
    #[must_use]
    #[inline(always)]
    pub const fn r7t7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Read receive FIFO bit 7 or write transmit FIFO bit 7."]
    #[inline(always)]
    pub const fn set_r7t7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Read receive FIFO bit 8 or write transmit FIFO bit 8."]
    #[must_use]
    #[inline(always)]
    pub const fn r8t8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Read receive FIFO bit 8 or write transmit FIFO bit 8."]
    #[inline(always)]
    pub const fn set_r8t8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Read receive FIFO bit 9 or write transmit FIFO bit 9."]
    #[must_use]
    #[inline(always)]
    pub const fn r9t9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Read receive FIFO bit 9 or write transmit FIFO bit 9."]
    #[inline(always)]
    pub const fn set_r9t9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "LIN Break."]
    #[must_use]
    #[inline(always)]
    pub const fn linbrk(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "LIN Break."]
    #[inline(always)]
    pub const fn set_linbrk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Idle Line."]
    #[must_use]
    #[inline(always)]
    pub const fn idline(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Idle Line."]
    #[inline(always)]
    pub const fn set_idline(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Receive Buffer Empty."]
    #[must_use]
    #[inline(always)]
    pub const fn rxempt(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Buffer Empty."]
    #[inline(always)]
    pub const fn set_rxempt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Frame Error Transmit Special Character."]
    #[must_use]
    #[inline(always)]
    pub const fn fretsc(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Frame Error Transmit Special Character."]
    #[inline(always)]
    pub const fn set_fretsc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Parity Error."]
    #[must_use]
    #[inline(always)]
    pub const fn paritye(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Parity Error."]
    #[inline(always)]
    pub const fn set_paritye(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Noisy Data Received."]
    #[must_use]
    #[inline(always)]
    pub const fn noisy(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Noisy Data Received."]
    #[inline(always)]
    pub const fn set_noisy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Data {
    #[inline(always)]
    fn default() -> Data {
        Data(0)
    }
}
impl core::fmt::Debug for Data {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Data")
            .field("r0t0", &self.r0t0())
            .field("r1t1", &self.r1t1())
            .field("r2t2", &self.r2t2())
            .field("r3t3", &self.r3t3())
            .field("r4t4", &self.r4t4())
            .field("r5t5", &self.r5t5())
            .field("r6t6", &self.r6t6())
            .field("r7t7", &self.r7t7())
            .field("r8t8", &self.r8t8())
            .field("r9t9", &self.r9t9())
            .field("linbrk", &self.linbrk())
            .field("idline", &self.idline())
            .field("rxempt", &self.rxempt())
            .field("fretsc", &self.fretsc())
            .field("paritye", &self.paritye())
            .field("noisy", &self.noisy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Data {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Data {{ r0t0: {=bool:?}, r1t1: {=bool:?}, r2t2: {=bool:?}, r3t3: {=bool:?}, r4t4: {=bool:?}, r5t5: {=bool:?}, r6t6: {=bool:?}, r7t7: {=bool:?}, r8t8: {=bool:?}, r9t9: {=bool:?}, linbrk: {=bool:?}, idline: {=bool:?}, rxempt: {=bool:?}, fretsc: {=bool:?}, paritye: {=bool:?}, noisy: {=bool:?} }}",
            self.r0t0(),
            self.r1t1(),
            self.r2t2(),
            self.r3t3(),
            self.r4t4(),
            self.r5t5(),
            self.r6t6(),
            self.r7t7(),
            self.r8t8(),
            self.r9t9(),
            self.linbrk(),
            self.idline(),
            self.rxempt(),
            self.fretsc(),
            self.paritye(),
            self.noisy()
        )
    }
}
#[doc = "Data Read-Only."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dataro(pub u32);
impl Dataro {
    #[doc = "Receive Data."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Receive Data."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Dataro {
    #[inline(always)]
    fn default() -> Dataro {
        Dataro(0)
    }
}
impl core::fmt::Debug for Dataro {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dataro")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dataro {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dataro {{ data: {=u16:?} }}", self.data())
    }
}
#[doc = "FIFO."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifo(pub u32);
impl Fifo {
    #[doc = "Receive FIFO Buffer Depth."]
    #[must_use]
    #[inline(always)]
    pub const fn rxfifosize(&self) -> Rxfifosize {
        let val = (self.0 >> 0usize) & 0x07;
        Rxfifosize::from_bits(val as u8)
    }
    #[doc = "Receive FIFO Buffer Depth."]
    #[inline(always)]
    pub const fn set_rxfifosize(&mut self, val: Rxfifosize) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Receive FIFO Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rxfe(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO Enable."]
    #[inline(always)]
    pub const fn set_rxfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Transmit FIFO Buffer Depth."]
    #[must_use]
    #[inline(always)]
    pub const fn txfifosize(&self) -> Txfifosize {
        let val = (self.0 >> 4usize) & 0x07;
        Txfifosize::from_bits(val as u8)
    }
    #[doc = "Transmit FIFO Buffer Depth."]
    #[inline(always)]
    pub const fn set_txfifosize(&mut self, val: Txfifosize) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Transmit FIFO Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn txfe(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO Enable."]
    #[inline(always)]
    pub const fn set_txfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Receive FIFO Underflow Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rxufe(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO Underflow Interrupt Enable."]
    #[inline(always)]
    pub const fn set_rxufe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Transmit FIFO Overflow Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn txofe(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO Overflow Interrupt Enable."]
    #[inline(always)]
    pub const fn set_txofe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Receiver Idle Empty Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rxiden(&self) -> Rxiden {
        let val = (self.0 >> 10usize) & 0x07;
        Rxiden::from_bits(val as u8)
    }
    #[doc = "Receiver Idle Empty Enable."]
    #[inline(always)]
    pub const fn set_rxiden(&mut self, val: Rxiden) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val.to_bits() as u32) & 0x07) << 10usize);
    }
    #[doc = "Receive FIFO Flush."]
    #[must_use]
    #[inline(always)]
    pub const fn rxflush(&self) -> Rxflush {
        let val = (self.0 >> 14usize) & 0x01;
        Rxflush::from_bits(val as u8)
    }
    #[doc = "Receive FIFO Flush."]
    #[inline(always)]
    pub const fn set_rxflush(&mut self, val: Rxflush) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Transmit FIFO Flush."]
    #[must_use]
    #[inline(always)]
    pub const fn txflush(&self) -> Txflush {
        let val = (self.0 >> 15usize) & 0x01;
        Txflush::from_bits(val as u8)
    }
    #[doc = "Transmit FIFO Flush."]
    #[inline(always)]
    pub const fn set_txflush(&mut self, val: Txflush) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Receiver FIFO Underflow Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rxuf(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver FIFO Underflow Flag."]
    #[inline(always)]
    pub const fn set_rxuf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Transmitter FIFO Overflow Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn txof(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter FIFO Overflow Flag."]
    #[inline(always)]
    pub const fn set_txof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Receive FIFO Or Buffer Empty."]
    #[must_use]
    #[inline(always)]
    pub const fn rxempt(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO Or Buffer Empty."]
    #[inline(always)]
    pub const fn set_rxempt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Transmit FIFO Or Buffer Empty."]
    #[must_use]
    #[inline(always)]
    pub const fn txempt(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO Or Buffer Empty."]
    #[inline(always)]
    pub const fn set_txempt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Fifo {
    #[inline(always)]
    fn default() -> Fifo {
        Fifo(0)
    }
}
impl core::fmt::Debug for Fifo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fifo")
            .field("rxfifosize", &self.rxfifosize())
            .field("rxfe", &self.rxfe())
            .field("txfifosize", &self.txfifosize())
            .field("txfe", &self.txfe())
            .field("rxufe", &self.rxufe())
            .field("txofe", &self.txofe())
            .field("rxiden", &self.rxiden())
            .field("rxflush", &self.rxflush())
            .field("txflush", &self.txflush())
            .field("rxuf", &self.rxuf())
            .field("txof", &self.txof())
            .field("rxempt", &self.rxempt())
            .field("txempt", &self.txempt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fifo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fifo {{ rxfifosize: {:?}, rxfe: {=bool:?}, txfifosize: {:?}, txfe: {=bool:?}, rxufe: {=bool:?}, txofe: {=bool:?}, rxiden: {:?}, rxflush: {:?}, txflush: {:?}, rxuf: {=bool:?}, txof: {=bool:?}, rxempt: {=bool:?}, txempt: {=bool:?} }}",
            self.rxfifosize(),
            self.rxfe(),
            self.txfifosize(),
            self.txfe(),
            self.rxufe(),
            self.txofe(),
            self.rxiden(),
            self.rxflush(),
            self.txflush(),
            self.rxuf(),
            self.txof(),
            self.rxempt(),
            self.txempt()
        )
    }
}
#[doc = "Global."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Global(pub u32);
impl Global {
    #[doc = "Software Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn rst(&self) -> Rst {
        let val = (self.0 >> 1usize) & 0x01;
        Rst::from_bits(val as u8)
    }
    #[doc = "Software Reset."]
    #[inline(always)]
    pub const fn set_rst(&mut self, val: Rst) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for Global {
    #[inline(always)]
    fn default() -> Global {
        Global(0)
    }
}
impl core::fmt::Debug for Global {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Global").field("rst", &self.rst()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Global {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Global {{ rst: {:?} }}", self.rst())
    }
}
#[doc = "Match Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match(pub u32);
impl Match {
    #[doc = "Match Address 1."]
    #[must_use]
    #[inline(always)]
    pub const fn ma1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Match Address 1."]
    #[inline(always)]
    pub const fn set_ma1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Match Address 2."]
    #[must_use]
    #[inline(always)]
    pub const fn ma2(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Match Address 2."]
    #[inline(always)]
    pub const fn set_ma2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for Match {
    #[inline(always)]
    fn default() -> Match {
        Match(0)
    }
}
impl core::fmt::Debug for Match {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Match")
            .field("ma1", &self.ma1())
            .field("ma2", &self.ma2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Match {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Match {{ ma1: {=u16:?}, ma2: {=u16:?} }}",
            self.ma1(),
            self.ma2()
        )
    }
}
#[doc = "MODEM IrDA."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Modir(pub u32);
impl Modir {
    #[doc = "Transmitter CTS Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn txctse(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter CTS Enable."]
    #[inline(always)]
    pub const fn set_txctse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmitter RTS Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn txrtse(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter RTS Enable."]
    #[inline(always)]
    pub const fn set_txrtse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmitter RTS Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn txrtspol(&self) -> Txrtspol {
        let val = (self.0 >> 2usize) & 0x01;
        Txrtspol::from_bits(val as u8)
    }
    #[doc = "Transmitter RTS Polarity."]
    #[inline(always)]
    pub const fn set_txrtspol(&mut self, val: Txrtspol) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Receiver RTS Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rxrtse(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver RTS Enable."]
    #[inline(always)]
    pub const fn set_rxrtse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Transmit CTS Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn txctsc(&self) -> Txctsc {
        let val = (self.0 >> 4usize) & 0x01;
        Txctsc::from_bits(val as u8)
    }
    #[doc = "Transmit CTS Configuration."]
    #[inline(always)]
    pub const fn set_txctsc(&mut self, val: Txctsc) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Transmit CTS Source."]
    #[must_use]
    #[inline(always)]
    pub const fn txctssrc(&self) -> Txctssrc {
        let val = (self.0 >> 5usize) & 0x01;
        Txctssrc::from_bits(val as u8)
    }
    #[doc = "Transmit CTS Source."]
    #[inline(always)]
    pub const fn set_txctssrc(&mut self, val: Txctssrc) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Receive RTS Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn rtswater(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Receive RTS Configuration."]
    #[inline(always)]
    pub const fn set_rtswater(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Transmitter Narrow Pulse."]
    #[must_use]
    #[inline(always)]
    pub const fn tnp(&self) -> Tnp {
        let val = (self.0 >> 16usize) & 0x03;
        Tnp::from_bits(val as u8)
    }
    #[doc = "Transmitter Narrow Pulse."]
    #[inline(always)]
    pub const fn set_tnp(&mut self, val: Tnp) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "IR Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn iren(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "IR Enable."]
    #[inline(always)]
    pub const fn set_iren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for Modir {
    #[inline(always)]
    fn default() -> Modir {
        Modir(0)
    }
}
impl core::fmt::Debug for Modir {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Modir")
            .field("txctse", &self.txctse())
            .field("txrtse", &self.txrtse())
            .field("txrtspol", &self.txrtspol())
            .field("rxrtse", &self.rxrtse())
            .field("txctsc", &self.txctsc())
            .field("txctssrc", &self.txctssrc())
            .field("rtswater", &self.rtswater())
            .field("tnp", &self.tnp())
            .field("iren", &self.iren())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Modir {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Modir {{ txctse: {=bool:?}, txrtse: {=bool:?}, txrtspol: {:?}, rxrtse: {=bool:?}, txctsc: {:?}, txctssrc: {:?}, rtswater: {=u8:?}, tnp: {:?}, iren: {=bool:?} }}",
            self.txctse(),
            self.txrtse(),
            self.txrtspol(),
            self.rxrtse(),
            self.txctsc(),
            self.txctssrc(),
            self.rtswater(),
            self.tnp(),
            self.iren()
        )
    }
}
#[doc = "Parameter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "Transmit FIFO Size."]
    #[must_use]
    #[inline(always)]
    pub const fn txfifo(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit FIFO Size."]
    #[inline(always)]
    pub const fn set_txfifo(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Receive FIFO Size."]
    #[must_use]
    #[inline(always)]
    pub const fn rxfifo(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Receive FIFO Size."]
    #[inline(always)]
    pub const fn set_rxfifo(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Param {
    #[inline(always)]
    fn default() -> Param {
        Param(0)
    }
}
impl core::fmt::Debug for Param {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Param")
            .field("txfifo", &self.txfifo())
            .field("rxfifo", &self.rxfifo())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Param {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Param {{ txfifo: {=u8:?}, rxfifo: {=u8:?} }}",
            self.txfifo(),
            self.rxfifo()
        )
    }
}
#[doc = "Pin Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pincfg(pub u32);
impl Pincfg {
    #[doc = "Trigger Select."]
    #[must_use]
    #[inline(always)]
    pub const fn trgsel(&self) -> Trgsel {
        let val = (self.0 >> 0usize) & 0x03;
        Trgsel::from_bits(val as u8)
    }
    #[doc = "Trigger Select."]
    #[inline(always)]
    pub const fn set_trgsel(&mut self, val: Trgsel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Pincfg {
    #[inline(always)]
    fn default() -> Pincfg {
        Pincfg(0)
    }
}
impl core::fmt::Debug for Pincfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pincfg")
            .field("trgsel", &self.trgsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pincfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pincfg {{ trgsel: {:?} }}", self.trgsel())
    }
}
#[doc = "Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat(pub u32);
impl Stat {
    #[doc = "LIN Break Flag Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lbkfe(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LIN Break Flag Enable."]
    #[inline(always)]
    pub const fn set_lbkfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Address Mark Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ame(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Address Mark Enable."]
    #[inline(always)]
    pub const fn set_ame(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Match 2 Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ma2f(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Match 2 Flag."]
    #[inline(always)]
    pub const fn set_ma2f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Match 1 Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ma1f(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Match 1 Flag."]
    #[inline(always)]
    pub const fn set_ma1f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Parity Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn pf(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Parity Error Flag."]
    #[inline(always)]
    pub const fn set_pf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Framing Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn fe(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Framing Error Flag."]
    #[inline(always)]
    pub const fn set_fe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Noise Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn nf(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Noise Flag."]
    #[inline(always)]
    pub const fn set_nf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Receiver Overrun Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn or(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver Overrun Flag."]
    #[inline(always)]
    pub const fn set_or(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Idle Line Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn idle(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Idle Line Flag."]
    #[inline(always)]
    pub const fn set_idle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Receive Data Register Full Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rdrf(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data Register Full Flag."]
    #[inline(always)]
    pub const fn set_rdrf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Transmission Complete Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tc(&self) -> Tc {
        let val = (self.0 >> 22usize) & 0x01;
        Tc::from_bits(val as u8)
    }
    #[doc = "Transmission Complete Flag."]
    #[inline(always)]
    pub const fn set_tc(&mut self, val: Tc) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Transmit Data Register Empty Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tdre(&self) -> Tdre {
        let val = (self.0 >> 23usize) & 0x01;
        Tdre::from_bits(val as u8)
    }
    #[doc = "Transmit Data Register Empty Flag."]
    #[inline(always)]
    pub const fn set_tdre(&mut self, val: Tdre) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Receiver Active Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn raf(&self) -> Raf {
        let val = (self.0 >> 24usize) & 0x01;
        Raf::from_bits(val as u8)
    }
    #[doc = "Receiver Active Flag."]
    #[inline(always)]
    pub const fn set_raf(&mut self, val: Raf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "LIN Break Detection Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lbkde(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "LIN Break Detection Enable."]
    #[inline(always)]
    pub const fn set_lbkde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Break Character Generation Length."]
    #[must_use]
    #[inline(always)]
    pub const fn brk13(&self) -> Brk13 {
        let val = (self.0 >> 26usize) & 0x01;
        Brk13::from_bits(val as u8)
    }
    #[doc = "Break Character Generation Length."]
    #[inline(always)]
    pub const fn set_brk13(&mut self, val: Brk13) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Receive Wake Up Idle Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn rwuid(&self) -> Rwuid {
        let val = (self.0 >> 27usize) & 0x01;
        Rwuid::from_bits(val as u8)
    }
    #[doc = "Receive Wake Up Idle Detect."]
    #[inline(always)]
    pub const fn set_rwuid(&mut self, val: Rwuid) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Receive Data Inversion."]
    #[must_use]
    #[inline(always)]
    pub const fn rxinv(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data Inversion."]
    #[inline(always)]
    pub const fn set_rxinv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "MSB First."]
    #[must_use]
    #[inline(always)]
    pub const fn msbf(&self) -> Msbf {
        let val = (self.0 >> 29usize) & 0x01;
        Msbf::from_bits(val as u8)
    }
    #[doc = "MSB First."]
    #[inline(always)]
    pub const fn set_msbf(&mut self, val: Msbf) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "RXD Pin Active Edge Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rxedgif(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "RXD Pin Active Edge Interrupt Flag."]
    #[inline(always)]
    pub const fn set_rxedgif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "LIN Break Detect Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn lbkdif(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LIN Break Detect Interrupt Flag."]
    #[inline(always)]
    pub const fn set_lbkdif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Stat {
    #[inline(always)]
    fn default() -> Stat {
        Stat(0)
    }
}
impl core::fmt::Debug for Stat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stat")
            .field("lbkfe", &self.lbkfe())
            .field("ame", &self.ame())
            .field("ma2f", &self.ma2f())
            .field("ma1f", &self.ma1f())
            .field("pf", &self.pf())
            .field("fe", &self.fe())
            .field("nf", &self.nf())
            .field("or", &self.or())
            .field("idle", &self.idle())
            .field("rdrf", &self.rdrf())
            .field("tc", &self.tc())
            .field("tdre", &self.tdre())
            .field("raf", &self.raf())
            .field("lbkde", &self.lbkde())
            .field("brk13", &self.brk13())
            .field("rwuid", &self.rwuid())
            .field("rxinv", &self.rxinv())
            .field("msbf", &self.msbf())
            .field("rxedgif", &self.rxedgif())
            .field("lbkdif", &self.lbkdif())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Stat {{ lbkfe: {=bool:?}, ame: {=bool:?}, ma2f: {=bool:?}, ma1f: {=bool:?}, pf: {=bool:?}, fe: {=bool:?}, nf: {=bool:?}, or: {=bool:?}, idle: {=bool:?}, rdrf: {=bool:?}, tc: {:?}, tdre: {:?}, raf: {:?}, lbkde: {=bool:?}, brk13: {:?}, rwuid: {:?}, rxinv: {=bool:?}, msbf: {:?}, rxedgif: {=bool:?}, lbkdif: {=bool:?} }}",
            self.lbkfe(),
            self.ame(),
            self.ma2f(),
            self.ma1f(),
            self.pf(),
            self.fe(),
            self.nf(),
            self.or(),
            self.idle(),
            self.rdrf(),
            self.tc(),
            self.tdre(),
            self.raf(),
            self.lbkde(),
            self.brk13(),
            self.rwuid(),
            self.rxinv(),
            self.msbf(),
            self.rxedgif(),
            self.lbkdif()
        )
    }
}
#[doc = "Version ID."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "Feature Identification Number."]
    #[must_use]
    #[inline(always)]
    pub const fn feature(&self) -> Feature {
        let val = (self.0 >> 0usize) & 0xffff;
        Feature::from_bits(val as u16)
    }
    #[doc = "Feature Identification Number."]
    #[inline(always)]
    pub const fn set_feature(&mut self, val: Feature) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
    #[doc = "Minor Version Number."]
    #[must_use]
    #[inline(always)]
    pub const fn minor(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Minor Version Number."]
    #[inline(always)]
    pub const fn set_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Major Version Number."]
    #[must_use]
    #[inline(always)]
    pub const fn major(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Major Version Number."]
    #[inline(always)]
    pub const fn set_major(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Verid {
    #[inline(always)]
    fn default() -> Verid {
        Verid(0)
    }
}
impl core::fmt::Debug for Verid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Verid")
            .field("feature", &self.feature())
            .field("minor", &self.minor())
            .field("major", &self.major())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Verid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Verid {{ feature: {:?}, minor: {=u8:?}, major: {=u8:?} }}",
            self.feature(),
            self.minor(),
            self.major()
        )
    }
}
#[doc = "Watermark."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Water(pub u32);
impl Water {
    #[doc = "Transmit Watermark."]
    #[must_use]
    #[inline(always)]
    pub const fn txwater(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Transmit Watermark."]
    #[inline(always)]
    pub const fn set_txwater(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Transmit Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn txcount(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Transmit Counter."]
    #[inline(always)]
    pub const fn set_txcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "Receive Watermark."]
    #[must_use]
    #[inline(always)]
    pub const fn rxwater(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Receive Watermark."]
    #[inline(always)]
    pub const fn set_rxwater(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Receive Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn rxcount(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Receive Counter."]
    #[inline(always)]
    pub const fn set_rxcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
}
impl Default for Water {
    #[inline(always)]
    fn default() -> Water {
        Water(0)
    }
}
impl core::fmt::Debug for Water {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Water")
            .field("txwater", &self.txwater())
            .field("txcount", &self.txcount())
            .field("rxwater", &self.rxwater())
            .field("rxcount", &self.rxcount())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Water {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Water {{ txwater: {=u8:?}, txcount: {=u8:?}, rxwater: {=u8:?}, rxcount: {=u8:?} }}",
            self.txwater(),
            self.txcount(),
            self.rxwater(),
            self.rxcount()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Brk13 {
    #[doc = "9 to 13 bit times."]
    Short = 0x0,
    #[doc = "12 to 15 bit times."]
    Long = 0x01,
}
impl Brk13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Brk13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Brk13 {
    #[inline(always)]
    fn from(val: u8) -> Brk13 {
        Brk13::from_bits(val)
    }
}
impl From<Brk13> for u8 {
    #[inline(always)]
    fn from(val: Brk13) -> u8 {
        Brk13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dozeen {
    #[doc = "Enable."]
    Enabled = 0x0,
    #[doc = "Disable."]
    Disabled = 0x01,
}
impl Dozeen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dozeen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dozeen {
    #[inline(always)]
    fn from(val: u8) -> Dozeen {
        Dozeen::from_bits(val)
    }
}
impl From<Dozeen> for u8 {
    #[inline(always)]
    fn from(val: Dozeen) -> u8 {
        Dozeen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Erlsmp {
    #[doc = "Regular midpoint bit sampling."]
    Midpoint = 0x0,
    #[doc = "Early bit sampling at 1/2 duration of bit timing."]
    Reduced = 0x01,
}
impl Erlsmp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Erlsmp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Erlsmp {
    #[inline(always)]
    fn from(val: u8) -> Erlsmp {
        Erlsmp::from_bits(val)
    }
}
impl From<Erlsmp> for u8 {
    #[inline(always)]
    fn from(val: Erlsmp) -> u8 {
        Erlsmp::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Feature(u16);
impl Feature {
    #[doc = "Standard feature set."]
    pub const Standard: Self = Self(0x01);
    #[doc = "Standard feature set with MODEM and IrDA support."]
    pub const Modem: Self = Self(0x03);
}
impl Feature {
    pub const fn from_bits(val: u16) -> Feature {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Feature {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("Standard"),
            0x03 => f.write_str("Modem"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Feature {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "Standard"),
            0x03 => defmt::write!(f, "Modem"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Feature {
    #[inline(always)]
    fn from(val: u16) -> Feature {
        Feature::from_bits(val)
    }
}
impl From<Feature> for u16 {
    #[inline(always)]
    fn from(val: Feature) -> u16 {
        Feature::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hlfm {
    #[doc = "Regular Oversampling."]
    Regular = 0x0,
    #[doc = "Reduced Oversampling by 0.5."]
    Reduced = 0x01,
}
impl Hlfm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hlfm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hlfm {
    #[inline(always)]
    fn from(val: u8) -> Hlfm {
        Hlfm::from_bits(val)
    }
}
impl From<Hlfm> for u8 {
    #[inline(always)]
    fn from(val: Hlfm) -> u8 {
        Hlfm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Idlecfg {
    #[doc = "1."]
    Idle1 = 0x0,
    #[doc = "2."]
    Idle2 = 0x01,
    #[doc = "4."]
    Idle4 = 0x02,
    #[doc = "8."]
    Idle8 = 0x03,
    #[doc = "16."]
    Idle16 = 0x04,
    #[doc = "32."]
    Idle32 = 0x05,
    #[doc = "64."]
    Idle64 = 0x06,
    #[doc = "128."]
    Idle128 = 0x07,
}
impl Idlecfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Idlecfg {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Idlecfg {
    #[inline(always)]
    fn from(val: u8) -> Idlecfg {
        Idlecfg::from_bits(val)
    }
}
impl From<Idlecfg> for u8 {
    #[inline(always)]
    fn from(val: Idlecfg) -> u8 {
        Idlecfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ilt {
    #[doc = "After the start bit."]
    FromStart = 0x0,
    #[doc = "After the stop bit."]
    FromStop = 0x01,
}
impl Ilt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ilt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ilt {
    #[inline(always)]
    fn from(val: u8) -> Ilt {
        Ilt::from_bits(val)
    }
}
impl From<Ilt> for u8 {
    #[inline(always)]
    fn from(val: Ilt) -> u8 {
        Ilt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Loops {
    #[doc = "Normal operation: RXD and TXD use separate pins."]
    Noffect = 0x0,
    #[doc = "Loop mode or Single-Wire mode."]
    Loopback = 0x01,
}
impl Loops {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Loops {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Loops {
    #[inline(always)]
    fn from(val: u8) -> Loops {
        Loops::from_bits(val)
    }
}
impl From<Loops> for u8 {
    #[inline(always)]
    fn from(val: Loops) -> u8 {
        Loops::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M {
    #[doc = "8-bit."]
    Data8 = 0x0,
    #[doc = "9-bit."]
    Data9 = 0x01,
}
impl M {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M {
    #[inline(always)]
    fn from(val: u8) -> M {
        M::from_bits(val)
    }
}
impl From<M> for u8 {
    #[inline(always)]
    fn from(val: M) -> u8 {
        M::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum M7 {
    #[doc = "8-bit to 10-bit."]
    NoEffect = 0x0,
    #[doc = "7-bit."]
    Data7 = 0x01,
}
impl M7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> M7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for M7 {
    #[inline(always)]
    fn from(val: u8) -> M7 {
        M7::from_bits(val)
    }
}
impl From<M7> for u8 {
    #[inline(always)]
    fn from(val: M7) -> u8 {
        M7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Matcfg {
    #[doc = "Address match wake-up."]
    AddrMatch = 0x0,
    #[doc = "Idle match wake-up."]
    IdleMatch = 0x01,
    #[doc = "Match on and match off."]
    OnoffMatch = 0x02,
    #[doc = "Enables RWU on data match and match on or off for the transmitter CTS input."]
    RwuMatch = 0x03,
}
impl Matcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Matcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Matcfg {
    #[inline(always)]
    fn from(val: u8) -> Matcfg {
        Matcfg::from_bits(val)
    }
}
impl From<Matcfg> for u8 {
    #[inline(always)]
    fn from(val: Matcfg) -> u8 {
        Matcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Msbf {
    #[doc = "LSB."]
    LsbFirst = 0x0,
    #[doc = "MSB."]
    MsbFirst = 0x01,
}
impl Msbf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Msbf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Msbf {
    #[inline(always)]
    fn from(val: u8) -> Msbf {
        Msbf::from_bits(val)
    }
}
impl From<Msbf> for u8 {
    #[inline(always)]
    fn from(val: Msbf) -> u8 {
        Msbf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pt {
    #[doc = "Even parity."]
    Even = 0x0,
    #[doc = "Odd parity."]
    Odd = 0x01,
}
impl Pt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pt {
    #[inline(always)]
    fn from(val: u8) -> Pt {
        Pt::from_bits(val)
    }
}
impl From<Pt> for u8 {
    #[inline(always)]
    fn from(val: Pt) -> u8 {
        Pt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Raf {
    #[doc = "Idle, waiting for a start bit."]
    Idle = 0x0,
    #[doc = "Receiver active (RXD pin input not idle)."]
    Active = 0x01,
}
impl Raf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Raf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Raf {
    #[inline(always)]
    fn from(val: u8) -> Raf {
        Raf::from_bits(val)
    }
}
impl From<Raf> for u8 {
    #[inline(always)]
    fn from(val: Raf) -> u8 {
        Raf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Resyncdis {
    #[doc = "Enable."]
    Resync = 0x0,
    #[doc = "Disable."]
    NoResync = 0x01,
}
impl Resyncdis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Resyncdis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Resyncdis {
    #[inline(always)]
    fn from(val: u8) -> Resyncdis {
        Resyncdis::from_bits(val)
    }
}
impl From<Resyncdis> for u8 {
    #[inline(always)]
    fn from(val: Resyncdis) -> u8 {
        Resyncdis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rsrc {
    #[doc = "Internal Loopback mode."]
    NoEffect = 0x0,
    #[doc = "Single-wire mode."]
    Onewire = 0x01,
}
impl Rsrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rsrc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rsrc {
    #[inline(always)]
    fn from(val: u8) -> Rsrc {
        Rsrc::from_bits(val)
    }
}
impl From<Rsrc> for u8 {
    #[inline(always)]
    fn from(val: Rsrc) -> u8 {
        Rsrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rst {
    #[doc = "Not reset."]
    NoEffect = 0x0,
    #[doc = "Reset."]
    Reset = 0x01,
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
pub enum Rwu {
    #[doc = "Normal receiver operation."]
    NoEffect = 0x0,
    #[doc = "LPUART receiver in standby, waiting for a wake-up condition."]
    RxWakeup = 0x01,
}
impl Rwu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rwu {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rwu {
    #[inline(always)]
    fn from(val: u8) -> Rwu {
        Rwu::from_bits(val)
    }
}
impl From<Rwu> for u8 {
    #[inline(always)]
    fn from(val: Rwu) -> u8 {
        Rwu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rwuid {
    #[doc = "STAT\\[IDLE\\] does not become 1."]
    IdleNotset = 0x0,
    #[doc = "STAT\\[IDLE\\] becomes 1."]
    IdleSet = 0x01,
}
impl Rwuid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rwuid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rwuid {
    #[inline(always)]
    fn from(val: u8) -> Rwuid {
        Rwuid::from_bits(val)
    }
}
impl From<Rwuid> for u8 {
    #[inline(always)]
    fn from(val: Rwuid) -> u8 {
        Rwuid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxfifosize {
    #[doc = "1."]
    Fifo1 = 0x0,
    #[doc = "4."]
    Fifo4 = 0x01,
    #[doc = "8."]
    Fifo8 = 0x02,
    #[doc = "16."]
    Fifo16 = 0x03,
    #[doc = "32."]
    Fifo32 = 0x04,
    #[doc = "64."]
    Fifo64 = 0x05,
    #[doc = "128."]
    Fifo128 = 0x06,
    #[doc = "256."]
    Fifo256 = 0x07,
}
impl Rxfifosize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxfifosize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxfifosize {
    #[inline(always)]
    fn from(val: u8) -> Rxfifosize {
        Rxfifosize::from_bits(val)
    }
}
impl From<Rxfifosize> for u8 {
    #[inline(always)]
    fn from(val: Rxfifosize) -> u8 {
        Rxfifosize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxflush {
    #[doc = "No effect."]
    NoEffect = 0x0,
    #[doc = "All data flushed out."]
    RxfifoRst = 0x01,
}
impl Rxflush {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxflush {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxflush {
    #[inline(always)]
    fn from(val: u8) -> Rxflush {
        Rxflush::from_bits(val)
    }
}
impl From<Rxflush> for u8 {
    #[inline(always)]
    fn from(val: Rxflush) -> u8 {
        Rxflush::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxiden {
    #[doc = "Disable STAT\\[RDRF\\] to become 1 because of partially filled FIFO when the receiver is idle."]
    Disabled = 0x0,
    #[doc = "Enable STAT\\[RDRF\\] to become 1 because of partially filled FIFO when the receiver is idle for one character."]
    Idle1 = 0x01,
    #[doc = "Enable STAT\\[RDRF\\] to become 1 because of partially filled FIFO when the receiver is idle for two characters."]
    Idle2 = 0x02,
    #[doc = "Enable STAT\\[RDRF\\] to become 1 because of partially filled FIFO when the receiver is idle for four characters."]
    Idle4 = 0x03,
    #[doc = "Enable STAT\\[RDRF\\] to become 1 because of partially filled FIFO when the receiver is idle for eight characters."]
    Idle8 = 0x04,
    #[doc = "Enable STAT\\[RDRF\\] to become 1 because of partially filled FIFO when the receiver is idle for 16 characters."]
    Idle16 = 0x05,
    #[doc = "Enable STAT\\[RDRF\\] to become 1 because of partially filled FIFO when the receiver is idle for 32 characters."]
    Idle32 = 0x06,
    #[doc = "Enable STAT\\[RDRF\\] to become 1 because of partially filled FIFO when the receiver is idle for 64 characters."]
    Idle64 = 0x07,
}
impl Rxiden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxiden {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxiden {
    #[inline(always)]
    fn from(val: u8) -> Rxiden {
        Rxiden::from_bits(val)
    }
}
impl From<Rxiden> for u8 {
    #[inline(always)]
    fn from(val: Rxiden) -> u8 {
        Rxiden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sbk {
    #[doc = "Normal transmitter operation."]
    NoEffect = 0x0,
    #[doc = "Queue break character(s) to be sent."]
    TxBreak = 0x01,
}
impl Sbk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sbk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sbk {
    #[inline(always)]
    fn from(val: u8) -> Sbk {
        Sbk::from_bits(val)
    }
}
impl From<Sbk> for u8 {
    #[inline(always)]
    fn from(val: Sbk) -> u8 {
        Sbk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sbns {
    #[doc = "One stop bit."]
    One = 0x0,
    #[doc = "Two stop bits."]
    Two = 0x01,
}
impl Sbns {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sbns {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sbns {
    #[inline(always)]
    fn from(val: u8) -> Sbns {
        Sbns::from_bits(val)
    }
}
impl From<Sbns> for u8 {
    #[inline(always)]
    fn from(val: Sbns) -> u8 {
        Sbns::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swap {
    #[doc = "Use the standard way."]
    Standard = 0x0,
    #[doc = "Swap."]
    Swap = 0x01,
}
impl Swap {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swap {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swap {
    #[inline(always)]
    fn from(val: u8) -> Swap {
        Swap::from_bits(val)
    }
}
impl From<Swap> for u8 {
    #[inline(always)]
    fn from(val: Swap) -> u8 {
        Swap::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tc {
    #[doc = "Transmitter active."]
    Active = 0x0,
    #[doc = "Transmitter idle."]
    Complete = 0x01,
}
impl Tc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tc {
    #[inline(always)]
    fn from(val: u8) -> Tc {
        Tc::from_bits(val)
    }
}
impl From<Tc> for u8 {
    #[inline(always)]
    fn from(val: Tc) -> u8 {
        Tc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tdre {
    #[doc = "Greater than watermark."]
    Txdata = 0x0,
    #[doc = "Equal to or less than watermark."]
    NoTxdata = 0x01,
}
impl Tdre {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tdre {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tdre {
    #[inline(always)]
    fn from(val: u8) -> Tdre {
        Tdre::from_bits(val)
    }
}
impl From<Tdre> for u8 {
    #[inline(always)]
    fn from(val: Tdre) -> u8 {
        Tdre::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tnp {
    #[doc = "1 / OSR."]
    OneSample = 0x0,
    #[doc = "2 / OSR."]
    TwoSample = 0x01,
    #[doc = "3 / OSR."]
    ThreeSample = 0x02,
    #[doc = "4 / OSR."]
    FourSample = 0x03,
}
impl Tnp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tnp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tnp {
    #[inline(always)]
    fn from(val: u8) -> Tnp {
        Tnp::from_bits(val)
    }
}
impl From<Tnp> for u8 {
    #[inline(always)]
    fn from(val: Tnp) -> u8 {
        Tnp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trgsel {
    #[doc = "Input trigger disabled."]
    Disabled = 0x0,
    #[doc = "Input trigger used instead of the RXD pin input."]
    TrgRxd = 0x01,
    #[doc = "Input trigger used instead of the CTS_B pin input."]
    TrgCts = 0x02,
    #[doc = "Input trigger used to modulate the TXD pin output, which (after TXINV configuration) is internally ANDed with the input trigger."]
    TrgTxd = 0x03,
}
impl Trgsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trgsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trgsel {
    #[inline(always)]
    fn from(val: u8) -> Trgsel {
        Trgsel::from_bits(val)
    }
}
impl From<Trgsel> for u8 {
    #[inline(always)]
    fn from(val: Trgsel) -> u8 {
        Trgsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txctsc {
    #[doc = "Sampled at the start of each character."]
    Start = 0x0,
    #[doc = "Sampled when the transmitter is idle."]
    Idle = 0x01,
}
impl Txctsc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txctsc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txctsc {
    #[inline(always)]
    fn from(val: u8) -> Txctsc {
        Txctsc::from_bits(val)
    }
}
impl From<Txctsc> for u8 {
    #[inline(always)]
    fn from(val: Txctsc) -> u8 {
        Txctsc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txctssrc {
    #[doc = "The CTS_B pin."]
    Cts = 0x0,
    #[doc = "An internal connection to the receiver address match result."]
    Match = 0x01,
}
impl Txctssrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txctssrc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txctssrc {
    #[inline(always)]
    fn from(val: u8) -> Txctssrc {
        Txctssrc::from_bits(val)
    }
}
impl From<Txctssrc> for u8 {
    #[inline(always)]
    fn from(val: Txctssrc) -> u8 {
        Txctssrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txdir {
    #[doc = "Input."]
    TxInput = 0x0,
    #[doc = "Output."]
    TxOutput = 0x01,
}
impl Txdir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txdir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txdir {
    #[inline(always)]
    fn from(val: u8) -> Txdir {
        Txdir::from_bits(val)
    }
}
impl From<Txdir> for u8 {
    #[inline(always)]
    fn from(val: Txdir) -> u8 {
        Txdir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txfifosize {
    #[doc = "1."]
    Fifo1 = 0x0,
    #[doc = "4."]
    Fifo4 = 0x01,
    #[doc = "8."]
    Fifo8 = 0x02,
    #[doc = "16."]
    Fifo16 = 0x03,
    #[doc = "32."]
    Fifo32 = 0x04,
    #[doc = "64."]
    Fifo64 = 0x05,
    #[doc = "128."]
    Fifo128 = 0x06,
    #[doc = "256."]
    Fifo256 = 0x07,
}
impl Txfifosize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txfifosize {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txfifosize {
    #[inline(always)]
    fn from(val: u8) -> Txfifosize {
        Txfifosize::from_bits(val)
    }
}
impl From<Txfifosize> for u8 {
    #[inline(always)]
    fn from(val: Txfifosize) -> u8 {
        Txfifosize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txflush {
    #[doc = "No effect."]
    NoEffect = 0x0,
    #[doc = "All data flushed out."]
    TxfifoRst = 0x01,
}
impl Txflush {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txflush {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txflush {
    #[inline(always)]
    fn from(val: u8) -> Txflush {
        Txflush::from_bits(val)
    }
}
impl From<Txflush> for u8 {
    #[inline(always)]
    fn from(val: Txflush) -> u8 {
        Txflush::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txrtspol {
    #[doc = "Active low."]
    Low = 0x0,
    #[doc = "Active high."]
    High = 0x01,
}
impl Txrtspol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txrtspol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txrtspol {
    #[inline(always)]
    fn from(val: u8) -> Txrtspol {
        Txrtspol::from_bits(val)
    }
}
impl From<Txrtspol> for u8 {
    #[inline(always)]
    fn from(val: Txrtspol) -> u8 {
        Txrtspol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wake {
    #[doc = "Idle."]
    Idle = 0x0,
    #[doc = "Mark."]
    Mark = 0x01,
}
impl Wake {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wake {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wake {
    #[inline(always)]
    fn from(val: u8) -> Wake {
        Wake::from_bits(val)
    }
}
impl From<Wake> for u8 {
    #[inline(always)]
    fn from(val: Wake) -> u8 {
        Wake::to_bits(val)
    }
}
