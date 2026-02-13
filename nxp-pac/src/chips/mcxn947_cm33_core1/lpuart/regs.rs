#[doc = "Baud Rate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Baud(pub u32);
impl Baud {
    #[doc = "Baud Rate Modulo Divisor"]
    #[must_use]
    #[inline(always)]
    pub const fn sbr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "Baud Rate Modulo Divisor"]
    #[inline(always)]
    pub const fn set_sbr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
    #[doc = "Stop Bit Number Select"]
    #[must_use]
    #[inline(always)]
    pub const fn sbns(&self) -> super::vals::Sbns {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Sbns::from_bits(val as u8)
    }
    #[doc = "Stop Bit Number Select"]
    #[inline(always)]
    pub const fn set_sbns(&mut self, val: super::vals::Sbns) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "RX Input Active Edge Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rxedgie(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "RX Input Active Edge Interrupt Enable"]
    #[inline(always)]
    pub const fn set_rxedgie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "LIN Break Detect Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lbkdie(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "LIN Break Detect Interrupt Enable"]
    #[inline(always)]
    pub const fn set_lbkdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Resynchronization Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn resyncdis(&self) -> super::vals::Resyncdis {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Resyncdis::from_bits(val as u8)
    }
    #[doc = "Resynchronization Disable"]
    #[inline(always)]
    pub const fn set_resyncdis(&mut self, val: super::vals::Resyncdis) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Both Edge Sampling"]
    #[must_use]
    #[inline(always)]
    pub const fn bothedge(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Both Edge Sampling"]
    #[inline(always)]
    pub const fn set_bothedge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Match Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn matcfg(&self) -> super::vals::Matcfg {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::Matcfg::from_bits(val as u8)
    }
    #[doc = "Match Configuration"]
    #[inline(always)]
    pub const fn set_matcfg(&mut self, val: super::vals::Matcfg) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Receiver Idle DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ridmae(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver Idle DMA Enable"]
    #[inline(always)]
    pub const fn set_ridmae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Receiver Full DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rdmae(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver Full DMA Enable"]
    #[inline(always)]
    pub const fn set_rdmae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Transmitter DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tdmae(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter DMA Enable"]
    #[inline(always)]
    pub const fn set_tdmae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Oversampling Ratio"]
    #[must_use]
    #[inline(always)]
    pub const fn osr(&self) -> super::vals::Osr {
        let val = (self.0 >> 24usize) & 0x1f;
        super::vals::Osr::from_bits(val as u8)
    }
    #[doc = "Oversampling Ratio"]
    #[inline(always)]
    pub const fn set_osr(&mut self, val: super::vals::Osr) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val.to_bits() as u32) & 0x1f) << 24usize);
    }
    #[doc = "10-Bit Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn m10(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "10-Bit Mode Select"]
    #[inline(always)]
    pub const fn set_m10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Match Address Mode Enable 2"]
    #[must_use]
    #[inline(always)]
    pub const fn maen2(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Match Address Mode Enable 2"]
    #[inline(always)]
    pub const fn set_maen2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Match Address Mode Enable 1"]
    #[must_use]
    #[inline(always)]
    pub const fn maen1(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Match Address Mode Enable 1"]
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
            "Baud {{ sbr: {=u16:?}, sbns: {:?}, rxedgie: {=bool:?}, lbkdie: {=bool:?}, resyncdis: {:?}, bothedge: {=bool:?}, matcfg: {:?}, ridmae: {=bool:?}, rdmae: {=bool:?}, tdmae: {=bool:?}, osr: {:?}, m10: {=bool:?}, maen2: {=bool:?}, maen1: {=bool:?} }}",
            self.sbr(),
            self.sbns(),
            self.rxedgie(),
            self.lbkdie(),
            self.resyncdis(),
            self.bothedge(),
            self.matcfg(),
            self.ridmae(),
            self.rdmae(),
            self.tdmae(),
            self.osr(),
            self.m10(),
            self.maen2(),
            self.maen1()
        )
    }
}
#[doc = "Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Parity Type"]
    #[must_use]
    #[inline(always)]
    pub const fn pt(&self) -> super::vals::Pt {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pt::from_bits(val as u8)
    }
    #[doc = "Parity Type"]
    #[inline(always)]
    pub const fn set_pt(&mut self, val: super::vals::Pt) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Parity Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Parity Enable"]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Idle Line Type Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ilt(&self) -> super::vals::Ilt {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Ilt::from_bits(val as u8)
    }
    #[doc = "Idle Line Type Select"]
    #[inline(always)]
    pub const fn set_ilt(&mut self, val: super::vals::Ilt) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Receiver Wake-Up Method Select"]
    #[must_use]
    #[inline(always)]
    pub const fn wake(&self) -> super::vals::Wake {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Wake::from_bits(val as u8)
    }
    #[doc = "Receiver Wake-Up Method Select"]
    #[inline(always)]
    pub const fn set_wake(&mut self, val: super::vals::Wake) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "9-Bit Or 8-Bit Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn m(&self) -> super::vals::M {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::M::from_bits(val as u8)
    }
    #[doc = "9-Bit Or 8-Bit Mode Select"]
    #[inline(always)]
    pub const fn set_m(&mut self, val: super::vals::M) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Receiver Source Select"]
    #[must_use]
    #[inline(always)]
    pub const fn rsrc(&self) -> super::vals::Rsrc {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Rsrc::from_bits(val as u8)
    }
    #[doc = "Receiver Source Select"]
    #[inline(always)]
    pub const fn set_rsrc(&mut self, val: super::vals::Rsrc) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Doze Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn dozeen(&self) -> super::vals::Dozeen {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dozeen::from_bits(val as u8)
    }
    #[doc = "Doze Mode"]
    #[inline(always)]
    pub const fn set_dozeen(&mut self, val: super::vals::Dozeen) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Loop Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn loops(&self) -> super::vals::Loops {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Loops::from_bits(val as u8)
    }
    #[doc = "Loop Mode Select"]
    #[inline(always)]
    pub const fn set_loops(&mut self, val: super::vals::Loops) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Idle Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn idlecfg(&self) -> super::vals::Idlecfg {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Idlecfg::from_bits(val as u8)
    }
    #[doc = "Idle Configuration"]
    #[inline(always)]
    pub const fn set_idlecfg(&mut self, val: super::vals::Idlecfg) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "7-Bit Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn m7(&self) -> super::vals::M7 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::M7::from_bits(val as u8)
    }
    #[doc = "7-Bit Mode Select"]
    #[inline(always)]
    pub const fn set_m7(&mut self, val: super::vals::M7) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Match 2 (MA2F) Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ma2ie(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Match 2 (MA2F) Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ma2ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Match 1 (MA1F) Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ma1ie(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Match 1 (MA1F) Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ma1ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Send Break"]
    #[must_use]
    #[inline(always)]
    pub const fn sbk(&self) -> super::vals::Sbk {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Sbk::from_bits(val as u8)
    }
    #[doc = "Send Break"]
    #[inline(always)]
    pub const fn set_sbk(&mut self, val: super::vals::Sbk) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Receiver Wake-Up Control"]
    #[must_use]
    #[inline(always)]
    pub const fn rwu(&self) -> super::vals::Rwu {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Rwu::from_bits(val as u8)
    }
    #[doc = "Receiver Wake-Up Control"]
    #[inline(always)]
    pub const fn set_rwu(&mut self, val: super::vals::Rwu) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Receiver Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn re(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver Enable"]
    #[inline(always)]
    pub const fn set_re(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Transmitter Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn te(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter Enable"]
    #[inline(always)]
    pub const fn set_te(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Idle Line Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ilie(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Idle Line Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ilie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Receiver Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rie(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver Interrupt Enable"]
    #[inline(always)]
    pub const fn set_rie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Transmission Complete Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tcie(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission Complete Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tcie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Transmit Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tie(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Parity Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn peie(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Parity Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_peie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Framing Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn feie(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Framing Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_feie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Noise Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn neie(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Noise Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_neie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Overrun Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn orie(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun Interrupt Enable"]
    #[inline(always)]
    pub const fn set_orie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Transmit Data Inversion"]
    #[must_use]
    #[inline(always)]
    pub const fn txinv(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Data Inversion"]
    #[inline(always)]
    pub const fn set_txinv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "TXD Pin Direction in Single-Wire Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn txdir(&self) -> super::vals::Txdir {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Txdir::from_bits(val as u8)
    }
    #[doc = "TXD Pin Direction in Single-Wire Mode"]
    #[inline(always)]
    pub const fn set_txdir(&mut self, val: super::vals::Txdir) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Receive Bit 9 Transmit Bit 8"]
    #[must_use]
    #[inline(always)]
    pub const fn r9t8(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Bit 9 Transmit Bit 8"]
    #[inline(always)]
    pub const fn set_r9t8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Receive Bit 8 Transmit Bit 9"]
    #[must_use]
    #[inline(always)]
    pub const fn r8t9(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Bit 8 Transmit Bit 9"]
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
            "Ctrl {{ pt: {:?}, pe: {=bool:?}, ilt: {:?}, wake: {:?}, m: {:?}, rsrc: {:?}, dozeen: {:?}, loops: {:?}, idlecfg: {:?}, m7: {:?}, ma2ie: {=bool:?}, ma1ie: {=bool:?}, sbk: {:?}, rwu: {:?}, re: {=bool:?}, te: {=bool:?}, ilie: {=bool:?}, rie: {=bool:?}, tcie: {=bool:?}, tie: {=bool:?}, peie: {=bool:?}, feie: {=bool:?}, neie: {=bool:?}, orie: {=bool:?}, txinv: {=bool:?}, txdir: {:?}, r9t8: {=bool:?}, r8t9: {=bool:?} }}",
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
#[doc = "Data"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Data(pub u32);
impl Data {
    #[doc = "Read receive FIFO bit 0 or write transmit FIFO bit 0"]
    #[must_use]
    #[inline(always)]
    pub const fn r0t0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read receive FIFO bit 0 or write transmit FIFO bit 0"]
    #[inline(always)]
    pub const fn set_r0t0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Read receive FIFO bit 1 or write transmit FIFO bit 1"]
    #[must_use]
    #[inline(always)]
    pub const fn r1t1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Read receive FIFO bit 1 or write transmit FIFO bit 1"]
    #[inline(always)]
    pub const fn set_r1t1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Read receive FIFO bit 2 or write transmit FIFO bit 2"]
    #[must_use]
    #[inline(always)]
    pub const fn r2t2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Read receive FIFO bit 2 or write transmit FIFO bit 2"]
    #[inline(always)]
    pub const fn set_r2t2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Read receive FIFO bit 3 or write transmit FIFO bit 3"]
    #[must_use]
    #[inline(always)]
    pub const fn r3t3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Read receive FIFO bit 3 or write transmit FIFO bit 3"]
    #[inline(always)]
    pub const fn set_r3t3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Read receive FIFO bit 4 or write transmit FIFO bit 4"]
    #[must_use]
    #[inline(always)]
    pub const fn r4t4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Read receive FIFO bit 4 or write transmit FIFO bit 4"]
    #[inline(always)]
    pub const fn set_r4t4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Read receive FIFO bit 5 or write transmit FIFO bit 5"]
    #[must_use]
    #[inline(always)]
    pub const fn r5t5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Read receive FIFO bit 5 or write transmit FIFO bit 5"]
    #[inline(always)]
    pub const fn set_r5t5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Read receive FIFO bit 6 or write transmit FIFO bit 6"]
    #[must_use]
    #[inline(always)]
    pub const fn r6t6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Read receive FIFO bit 6 or write transmit FIFO bit 6"]
    #[inline(always)]
    pub const fn set_r6t6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Read receive FIFO bit 7 or write transmit FIFO bit 7"]
    #[must_use]
    #[inline(always)]
    pub const fn r7t7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Read receive FIFO bit 7 or write transmit FIFO bit 7"]
    #[inline(always)]
    pub const fn set_r7t7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Read receive FIFO bit 8 or write transmit FIFO bit 8"]
    #[must_use]
    #[inline(always)]
    pub const fn r8t8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Read receive FIFO bit 8 or write transmit FIFO bit 8"]
    #[inline(always)]
    pub const fn set_r8t8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Read receive FIFO bit 9 or write transmit FIFO bit 9"]
    #[must_use]
    #[inline(always)]
    pub const fn r9t9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Read receive FIFO bit 9 or write transmit FIFO bit 9"]
    #[inline(always)]
    pub const fn set_r9t9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "LIN Break"]
    #[must_use]
    #[inline(always)]
    pub const fn linbrk(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "LIN Break"]
    #[inline(always)]
    pub const fn set_linbrk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Idle Line"]
    #[must_use]
    #[inline(always)]
    pub const fn idline(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Idle Line"]
    #[inline(always)]
    pub const fn set_idline(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Receive Buffer Empty"]
    #[must_use]
    #[inline(always)]
    pub const fn rxempt(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Buffer Empty"]
    #[inline(always)]
    pub const fn set_rxempt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Frame Error Transmit Special Character"]
    #[must_use]
    #[inline(always)]
    pub const fn fretsc(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Frame Error Transmit Special Character"]
    #[inline(always)]
    pub const fn set_fretsc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Parity Error"]
    #[must_use]
    #[inline(always)]
    pub const fn paritye(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Parity Error"]
    #[inline(always)]
    pub const fn set_paritye(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Noisy Data Received"]
    #[must_use]
    #[inline(always)]
    pub const fn noisy(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Noisy Data Received"]
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
#[doc = "Data Read-Only"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dataro(pub u32);
impl Dataro {
    #[doc = "Receive Data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Receive Data"]
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
#[doc = "FIFO"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifo(pub u32);
impl Fifo {
    #[doc = "Receive FIFO Buffer Depth"]
    #[must_use]
    #[inline(always)]
    pub const fn rxfifosize(&self) -> super::vals::Rxfifosize {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Rxfifosize::from_bits(val as u8)
    }
    #[doc = "Receive FIFO Buffer Depth"]
    #[inline(always)]
    pub const fn set_rxfifosize(&mut self, val: super::vals::Rxfifosize) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Receive FIFO Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rxfe(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO Enable"]
    #[inline(always)]
    pub const fn set_rxfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Transmit FIFO Buffer Depth"]
    #[must_use]
    #[inline(always)]
    pub const fn txfifosize(&self) -> super::vals::Txfifosize {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Txfifosize::from_bits(val as u8)
    }
    #[doc = "Transmit FIFO Buffer Depth"]
    #[inline(always)]
    pub const fn set_txfifosize(&mut self, val: super::vals::Txfifosize) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Transmit FIFO Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn txfe(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO Enable"]
    #[inline(always)]
    pub const fn set_txfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Receive FIFO Underflow Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rxufe(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO Underflow Interrupt Enable"]
    #[inline(always)]
    pub const fn set_rxufe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Transmit FIFO Overflow Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn txofe(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO Overflow Interrupt Enable"]
    #[inline(always)]
    pub const fn set_txofe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Receiver Idle Empty Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rxiden(&self) -> super::vals::Rxiden {
        let val = (self.0 >> 10usize) & 0x07;
        super::vals::Rxiden::from_bits(val as u8)
    }
    #[doc = "Receiver Idle Empty Enable"]
    #[inline(always)]
    pub const fn set_rxiden(&mut self, val: super::vals::Rxiden) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val.to_bits() as u32) & 0x07) << 10usize);
    }
    #[doc = "Receive FIFO Flush"]
    #[must_use]
    #[inline(always)]
    pub const fn rxflush(&self) -> super::vals::Rxflush {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Rxflush::from_bits(val as u8)
    }
    #[doc = "Receive FIFO Flush"]
    #[inline(always)]
    pub const fn set_rxflush(&mut self, val: super::vals::Rxflush) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Transmit FIFO Flush"]
    #[must_use]
    #[inline(always)]
    pub const fn txflush(&self) -> super::vals::Txflush {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Txflush::from_bits(val as u8)
    }
    #[doc = "Transmit FIFO Flush"]
    #[inline(always)]
    pub const fn set_txflush(&mut self, val: super::vals::Txflush) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Receiver FIFO Underflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rxuf(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver FIFO Underflow Flag"]
    #[inline(always)]
    pub const fn set_rxuf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Transmitter FIFO Overflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn txof(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter FIFO Overflow Flag"]
    #[inline(always)]
    pub const fn set_txof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Receive FIFO Or Buffer Empty"]
    #[must_use]
    #[inline(always)]
    pub const fn rxempt(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO Or Buffer Empty"]
    #[inline(always)]
    pub const fn set_rxempt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Transmit FIFO Or Buffer Empty"]
    #[must_use]
    #[inline(always)]
    pub const fn txempt(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO Or Buffer Empty"]
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
#[doc = "Global"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Global(pub u32);
impl Global {
    #[doc = "Software Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn rst(&self) -> super::vals::Rst {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Rst::from_bits(val as u8)
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn set_rst(&mut self, val: super::vals::Rst) {
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
#[doc = "Half Duplex Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hdcr(pub u32);
impl Hdcr {
    #[doc = "Transmit Stall"]
    #[must_use]
    #[inline(always)]
    pub const fn txstall(&self) -> super::vals::Txstall {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Txstall::from_bits(val as u8)
    }
    #[doc = "Transmit Stall"]
    #[inline(always)]
    pub const fn set_txstall(&mut self, val: super::vals::Txstall) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive Select"]
    #[must_use]
    #[inline(always)]
    pub const fn rxsel(&self) -> super::vals::Rxsel {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Rxsel::from_bits(val as u8)
    }
    #[doc = "Receive Select"]
    #[inline(always)]
    pub const fn set_rxsel(&mut self, val: super::vals::Rxsel) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Receive FIFO Write Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn rxwrmsk(&self) -> super::vals::Rxwrmsk {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Rxwrmsk::from_bits(val as u8)
    }
    #[doc = "Receive FIFO Write Mask"]
    #[inline(always)]
    pub const fn set_rxwrmsk(&mut self, val: super::vals::Rxwrmsk) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Receive Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn rxmsk(&self) -> super::vals::Rxmsk {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Rxmsk::from_bits(val as u8)
    }
    #[doc = "Receive Mask"]
    #[inline(always)]
    pub const fn set_rxmsk(&mut self, val: super::vals::Rxmsk) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "RTS Extended"]
    #[must_use]
    #[inline(always)]
    pub const fn rtsext(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "RTS Extended"]
    #[inline(always)]
    pub const fn set_rtsext(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Hdcr {
    #[inline(always)]
    fn default() -> Hdcr {
        Hdcr(0)
    }
}
impl core::fmt::Debug for Hdcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hdcr")
            .field("txstall", &self.txstall())
            .field("rxsel", &self.rxsel())
            .field("rxwrmsk", &self.rxwrmsk())
            .field("rxmsk", &self.rxmsk())
            .field("rtsext", &self.rtsext())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hdcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hdcr {{ txstall: {:?}, rxsel: {:?}, rxwrmsk: {:?}, rxmsk: {:?}, rtsext: {=u8:?} }}",
            self.txstall(),
            self.rxsel(),
            self.rxwrmsk(),
            self.rxmsk(),
            self.rtsext()
        )
    }
}
#[doc = "Match Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match(pub u32);
impl Match {
    #[doc = "Match Address 1"]
    #[must_use]
    #[inline(always)]
    pub const fn ma1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Match Address 1"]
    #[inline(always)]
    pub const fn set_ma1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Match Address 2"]
    #[must_use]
    #[inline(always)]
    pub const fn ma2(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Match Address 2"]
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
#[doc = "MODEM Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcr(pub u32);
impl Mcr {
    #[doc = "Clear To Send"]
    #[must_use]
    #[inline(always)]
    pub const fn cts(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clear To Send"]
    #[inline(always)]
    pub const fn set_cts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Data Set Ready"]
    #[must_use]
    #[inline(always)]
    pub const fn dsr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Data Set Ready"]
    #[inline(always)]
    pub const fn set_dsr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Ring Indicator"]
    #[must_use]
    #[inline(always)]
    pub const fn rin(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Ring Indicator"]
    #[inline(always)]
    pub const fn set_rin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Data Carrier Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn dcd(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Data Carrier Detect"]
    #[inline(always)]
    pub const fn set_dcd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Data Terminal Ready"]
    #[must_use]
    #[inline(always)]
    pub const fn dtr(&self) -> super::vals::Dtr {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Dtr::from_bits(val as u8)
    }
    #[doc = "Data Terminal Ready"]
    #[inline(always)]
    pub const fn set_dtr(&mut self, val: super::vals::Dtr) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Request To Send"]
    #[must_use]
    #[inline(always)]
    pub const fn rts(&self) -> super::vals::Rts {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Rts::from_bits(val as u8)
    }
    #[doc = "Request To Send"]
    #[inline(always)]
    pub const fn set_rts(&mut self, val: super::vals::Rts) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Mcr {
    #[inline(always)]
    fn default() -> Mcr {
        Mcr(0)
    }
}
impl core::fmt::Debug for Mcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mcr")
            .field("cts", &self.cts())
            .field("dsr", &self.dsr())
            .field("rin", &self.rin())
            .field("dcd", &self.dcd())
            .field("dtr", &self.dtr())
            .field("rts", &self.rts())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mcr {{ cts: {=bool:?}, dsr: {=bool:?}, rin: {=bool:?}, dcd: {=bool:?}, dtr: {:?}, rts: {:?} }}",
            self.cts(),
            self.dsr(),
            self.rin(),
            self.dcd(),
            self.dtr(),
            self.rts()
        )
    }
}
#[doc = "MODEM IrDA"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Modir(pub u32);
impl Modir {
    #[doc = "Transmitter CTS Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn txctse(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter CTS Enable"]
    #[inline(always)]
    pub const fn set_txctse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmitter RTS Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn txrtse(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter RTS Enable"]
    #[inline(always)]
    pub const fn set_txrtse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmitter RTS Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn txrtspol(&self) -> super::vals::Txrtspol {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Txrtspol::from_bits(val as u8)
    }
    #[doc = "Transmitter RTS Polarity"]
    #[inline(always)]
    pub const fn set_txrtspol(&mut self, val: super::vals::Txrtspol) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Receiver RTS Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rxrtse(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver RTS Enable"]
    #[inline(always)]
    pub const fn set_rxrtse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Transmit CTS Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn txctsc(&self) -> super::vals::Txctsc {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Txctsc::from_bits(val as u8)
    }
    #[doc = "Transmit CTS Configuration"]
    #[inline(always)]
    pub const fn set_txctsc(&mut self, val: super::vals::Txctsc) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Transmit CTS Source"]
    #[must_use]
    #[inline(always)]
    pub const fn txctssrc(&self) -> super::vals::Txctssrc {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Txctssrc::from_bits(val as u8)
    }
    #[doc = "Transmit CTS Source"]
    #[inline(always)]
    pub const fn set_txctssrc(&mut self, val: super::vals::Txctssrc) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Receive RTS Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn rtswater(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Receive RTS Configuration"]
    #[inline(always)]
    pub const fn set_rtswater(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "Transmitter Narrow Pulse"]
    #[must_use]
    #[inline(always)]
    pub const fn tnp(&self) -> super::vals::Tnp {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Tnp::from_bits(val as u8)
    }
    #[doc = "Transmitter Narrow Pulse"]
    #[inline(always)]
    pub const fn set_tnp(&mut self, val: super::vals::Tnp) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "IR Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn iren(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "IR Enable"]
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
#[doc = "MODEM Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msr(pub u32);
impl Msr {
    #[doc = "Delta Clear To Send"]
    #[must_use]
    #[inline(always)]
    pub const fn dcts(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Delta Clear To Send"]
    #[inline(always)]
    pub const fn set_dcts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Delta Data Set Ready"]
    #[must_use]
    #[inline(always)]
    pub const fn ddsr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Delta Data Set Ready"]
    #[inline(always)]
    pub const fn set_ddsr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Delta Ring Indicator"]
    #[must_use]
    #[inline(always)]
    pub const fn dri(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Delta Ring Indicator"]
    #[inline(always)]
    pub const fn set_dri(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Delta Data Carrier Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn ddcd(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Delta Data Carrier Detect"]
    #[inline(always)]
    pub const fn set_ddcd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Clear To Send"]
    #[must_use]
    #[inline(always)]
    pub const fn cts(&self) -> super::vals::MsrCts {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::MsrCts::from_bits(val as u8)
    }
    #[doc = "Clear To Send"]
    #[inline(always)]
    pub const fn set_cts(&mut self, val: super::vals::MsrCts) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Data Set Ready"]
    #[must_use]
    #[inline(always)]
    pub const fn dsr(&self) -> super::vals::MsrDsr {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::MsrDsr::from_bits(val as u8)
    }
    #[doc = "Data Set Ready"]
    #[inline(always)]
    pub const fn set_dsr(&mut self, val: super::vals::MsrDsr) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Ring Indicator"]
    #[must_use]
    #[inline(always)]
    pub const fn rin(&self) -> super::vals::MsrRin {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::MsrRin::from_bits(val as u8)
    }
    #[doc = "Ring Indicator"]
    #[inline(always)]
    pub const fn set_rin(&mut self, val: super::vals::MsrRin) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Data Carrier Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn dcd(&self) -> super::vals::MsrDcd {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::MsrDcd::from_bits(val as u8)
    }
    #[doc = "Data Carrier Detect"]
    #[inline(always)]
    pub const fn set_dcd(&mut self, val: super::vals::MsrDcd) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for Msr {
    #[inline(always)]
    fn default() -> Msr {
        Msr(0)
    }
}
impl core::fmt::Debug for Msr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Msr")
            .field("dcts", &self.dcts())
            .field("ddsr", &self.ddsr())
            .field("dri", &self.dri())
            .field("ddcd", &self.ddcd())
            .field("cts", &self.cts())
            .field("dsr", &self.dsr())
            .field("rin", &self.rin())
            .field("dcd", &self.dcd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Msr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Msr {{ dcts: {=bool:?}, ddsr: {=bool:?}, dri: {=bool:?}, ddcd: {=bool:?}, cts: {:?}, dsr: {:?}, rin: {:?}, dcd: {:?} }}",
            self.dcts(),
            self.ddsr(),
            self.dri(),
            self.ddcd(),
            self.cts(),
            self.dsr(),
            self.rin(),
            self.dcd()
        )
    }
}
#[doc = "Parameter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "Transmit FIFO Size"]
    #[must_use]
    #[inline(always)]
    pub const fn txfifo(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit FIFO Size"]
    #[inline(always)]
    pub const fn set_txfifo(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Receive FIFO Size"]
    #[must_use]
    #[inline(always)]
    pub const fn rxfifo(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Receive FIFO Size"]
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
#[doc = "Pin Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pincfg(pub u32);
impl Pincfg {
    #[doc = "Trigger Select"]
    #[must_use]
    #[inline(always)]
    pub const fn trgsel(&self) -> super::vals::Trgsel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Trgsel::from_bits(val as u8)
    }
    #[doc = "Trigger Select"]
    #[inline(always)]
    pub const fn set_trgsel(&mut self, val: super::vals::Trgsel) {
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
#[doc = "Receiver Extended Idle"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reir(pub u32);
impl Reir {
    #[doc = "Idle Time"]
    #[must_use]
    #[inline(always)]
    pub const fn idtime(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "Idle Time"]
    #[inline(always)]
    pub const fn set_idtime(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
}
impl Default for Reir {
    #[inline(always)]
    fn default() -> Reir {
        Reir(0)
    }
}
impl core::fmt::Debug for Reir {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Reir")
            .field("idtime", &self.idtime())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Reir {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Reir {{ idtime: {=u16:?} }}", self.idtime())
    }
}
#[doc = "Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat(pub u32);
impl Stat {
    #[doc = "LIN Break Flag Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lbkfe(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LIN Break Flag Enable"]
    #[inline(always)]
    pub const fn set_lbkfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Address Mark Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ame(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Address Mark Enable"]
    #[inline(always)]
    pub const fn set_ame(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "MODEM Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn msf(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "MODEM Status Flag"]
    #[inline(always)]
    pub const fn set_msf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Timeout Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tsf(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout Status Flag"]
    #[inline(always)]
    pub const fn set_tsf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Match 2 Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ma2f(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Match 2 Flag"]
    #[inline(always)]
    pub const fn set_ma2f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Match 1 Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ma1f(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Match 1 Flag"]
    #[inline(always)]
    pub const fn set_ma1f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Parity Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn pf(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Parity Error Flag"]
    #[inline(always)]
    pub const fn set_pf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Framing Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn fe(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Framing Error Flag"]
    #[inline(always)]
    pub const fn set_fe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Noise Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn nf(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Noise Flag"]
    #[inline(always)]
    pub const fn set_nf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Receiver Overrun Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn or(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver Overrun Flag"]
    #[inline(always)]
    pub const fn set_or(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Idle Line Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn idle(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Idle Line Flag"]
    #[inline(always)]
    pub const fn set_idle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Receive Data Register Full Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rdrf(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data Register Full Flag"]
    #[inline(always)]
    pub const fn set_rdrf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Transmission Complete Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tc(&self) -> super::vals::Tc {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Tc::from_bits(val as u8)
    }
    #[doc = "Transmission Complete Flag"]
    #[inline(always)]
    pub const fn set_tc(&mut self, val: super::vals::Tc) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Transmit Data Register Empty Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tdre(&self) -> super::vals::Tdre {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Tdre::from_bits(val as u8)
    }
    #[doc = "Transmit Data Register Empty Flag"]
    #[inline(always)]
    pub const fn set_tdre(&mut self, val: super::vals::Tdre) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Receiver Active Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn raf(&self) -> super::vals::Raf {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Raf::from_bits(val as u8)
    }
    #[doc = "Receiver Active Flag"]
    #[inline(always)]
    pub const fn set_raf(&mut self, val: super::vals::Raf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "LIN Break Detection Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lbkde(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "LIN Break Detection Enable"]
    #[inline(always)]
    pub const fn set_lbkde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Break Character Generation Length"]
    #[must_use]
    #[inline(always)]
    pub const fn brk13(&self) -> super::vals::Brk13 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Brk13::from_bits(val as u8)
    }
    #[doc = "Break Character Generation Length"]
    #[inline(always)]
    pub const fn set_brk13(&mut self, val: super::vals::Brk13) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Receive Wake Up Idle Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn rwuid(&self) -> super::vals::Rwuid {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Rwuid::from_bits(val as u8)
    }
    #[doc = "Receive Wake Up Idle Detect"]
    #[inline(always)]
    pub const fn set_rwuid(&mut self, val: super::vals::Rwuid) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Receive Data Inversion"]
    #[must_use]
    #[inline(always)]
    pub const fn rxinv(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data Inversion"]
    #[inline(always)]
    pub const fn set_rxinv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "MSB First"]
    #[must_use]
    #[inline(always)]
    pub const fn msbf(&self) -> super::vals::Msbf {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Msbf::from_bits(val as u8)
    }
    #[doc = "MSB First"]
    #[inline(always)]
    pub const fn set_msbf(&mut self, val: super::vals::Msbf) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "RXD Pin Active Edge Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rxedgif(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "RXD Pin Active Edge Interrupt Flag"]
    #[inline(always)]
    pub const fn set_rxedgif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "LIN Break Detect Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn lbkdif(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LIN Break Detect Interrupt Flag"]
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
            .field("msf", &self.msf())
            .field("tsf", &self.tsf())
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
            "Stat {{ lbkfe: {=bool:?}, ame: {=bool:?}, msf: {=bool:?}, tsf: {=bool:?}, ma2f: {=bool:?}, ma1f: {=bool:?}, pf: {=bool:?}, fe: {=bool:?}, nf: {=bool:?}, or: {=bool:?}, idle: {=bool:?}, rdrf: {=bool:?}, tc: {:?}, tdre: {:?}, raf: {:?}, lbkde: {=bool:?}, brk13: {:?}, rwuid: {:?}, rxinv: {=bool:?}, msbf: {:?}, rxedgif: {=bool:?}, lbkdif: {=bool:?} }}",
            self.lbkfe(),
            self.ame(),
            self.msf(),
            self.tsf(),
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
#[doc = "Transmit Command Burst"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcbr(pub u32);
impl Tcbr {
    #[doc = "Data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Tcbr {
    #[inline(always)]
    fn default() -> Tcbr {
        Tcbr(0)
    }
}
impl core::fmt::Debug for Tcbr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcbr").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcbr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcbr {{ data: {=u16:?} }}", self.data())
    }
}
#[doc = "Transmit Data Burst"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdbr(pub u32);
impl Tdbr {
    #[doc = "Data0"]
    #[must_use]
    #[inline(always)]
    pub const fn data0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data0"]
    #[inline(always)]
    pub const fn set_data0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data1"]
    #[must_use]
    #[inline(always)]
    pub const fn data1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data1"]
    #[inline(always)]
    pub const fn set_data1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data2"]
    #[must_use]
    #[inline(always)]
    pub const fn data2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data2"]
    #[inline(always)]
    pub const fn set_data2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data3"]
    #[must_use]
    #[inline(always)]
    pub const fn data3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data3"]
    #[inline(always)]
    pub const fn set_data3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Tdbr {
    #[inline(always)]
    fn default() -> Tdbr {
        Tdbr(0)
    }
}
impl core::fmt::Debug for Tdbr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tdbr")
            .field("data0", &self.data0())
            .field("data1", &self.data1())
            .field("data2", &self.data2())
            .field("data3", &self.data3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tdbr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tdbr {{ data0: {=u8:?}, data1: {=u8:?}, data2: {=u8:?}, data3: {=u8:?} }}",
            self.data0(),
            self.data1(),
            self.data2(),
            self.data3()
        )
    }
}
#[doc = "Transmitter Extended Idle"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Teir(pub u32);
impl Teir {
    #[doc = "Idle Time"]
    #[must_use]
    #[inline(always)]
    pub const fn idtime(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "Idle Time"]
    #[inline(always)]
    pub const fn set_idtime(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
}
impl Default for Teir {
    #[inline(always)]
    fn default() -> Teir {
        Teir(0)
    }
}
impl core::fmt::Debug for Teir {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Teir")
            .field("idtime", &self.idtime())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Teir {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Teir {{ idtime: {=u16:?} }}", self.idtime())
    }
}
#[doc = "Timeout N"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timeout(pub u32);
impl Timeout {
    #[doc = "Timeout Value"]
    #[must_use]
    #[inline(always)]
    pub const fn timeout(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "Timeout Value"]
    #[inline(always)]
    pub const fn set_timeout(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[doc = "Idle Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn cfg(&self) -> super::vals::Cfg {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::Cfg::from_bits(val as u8)
    }
    #[doc = "Idle Configuration"]
    #[inline(always)]
    pub const fn set_cfg(&mut self, val: super::vals::Cfg) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for Timeout {
    #[inline(always)]
    fn default() -> Timeout {
        Timeout(0)
    }
}
impl core::fmt::Debug for Timeout {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timeout")
            .field("timeout", &self.timeout())
            .field("cfg", &self.cfg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timeout {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Timeout {{ timeout: {=u16:?}, cfg: {:?} }}",
            self.timeout(),
            self.cfg()
        )
    }
}
#[doc = "Timeout Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tocr(pub u32);
impl Tocr {
    #[doc = "Timeout Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn toen(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Timeout Enable"]
    #[inline(always)]
    pub const fn set_toen(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Timeout Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn toie(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Timeout Interrupt Enable"]
    #[inline(always)]
    pub const fn set_toie(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
}
impl Default for Tocr {
    #[inline(always)]
    fn default() -> Tocr {
        Tocr(0)
    }
}
impl core::fmt::Debug for Tocr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tocr")
            .field("toen", &self.toen())
            .field("toie", &self.toie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tocr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tocr {{ toen: {=u8:?}, toie: {=u8:?} }}",
            self.toen(),
            self.toie()
        )
    }
}
#[doc = "Timeout Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tosr(pub u32);
impl Tosr {
    #[doc = "Timeout Zero"]
    #[must_use]
    #[inline(always)]
    pub const fn toz(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Timeout Zero"]
    #[inline(always)]
    pub const fn set_toz(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Timeout Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tof(&self) -> super::vals::Tof {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Tof::from_bits(val as u8)
    }
    #[doc = "Timeout Flag"]
    #[inline(always)]
    pub const fn set_tof(&mut self, val: super::vals::Tof) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
}
impl Default for Tosr {
    #[inline(always)]
    fn default() -> Tosr {
        Tosr(0)
    }
}
impl core::fmt::Debug for Tosr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tosr")
            .field("toz", &self.toz())
            .field("tof", &self.tof())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tosr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tosr {{ toz: {=u8:?}, tof: {:?} }}",
            self.toz(),
            self.tof()
        )
    }
}
#[doc = "Version ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "Feature Identification Number"]
    #[must_use]
    #[inline(always)]
    pub const fn feature(&self) -> super::vals::Feature {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Feature::from_bits(val as u16)
    }
    #[doc = "Feature Identification Number"]
    #[inline(always)]
    pub const fn set_feature(&mut self, val: super::vals::Feature) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
    #[doc = "Minor Version Number"]
    #[must_use]
    #[inline(always)]
    pub const fn minor(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Minor Version Number"]
    #[inline(always)]
    pub const fn set_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Major Version Number"]
    #[must_use]
    #[inline(always)]
    pub const fn major(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Major Version Number"]
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
#[doc = "Watermark"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Water(pub u32);
impl Water {
    #[doc = "Transmit Watermark"]
    #[must_use]
    #[inline(always)]
    pub const fn txwater(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Transmit Watermark"]
    #[inline(always)]
    pub const fn set_txwater(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Transmit Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn txcount(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Transmit Counter"]
    #[inline(always)]
    pub const fn set_txcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Receive Watermark"]
    #[must_use]
    #[inline(always)]
    pub const fn rxwater(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Receive Watermark"]
    #[inline(always)]
    pub const fn set_rxwater(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Receive Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn rxcount(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Receive Counter"]
    #[inline(always)]
    pub const fn set_rxcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
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
