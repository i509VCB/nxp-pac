#[doc = "CRC Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crcr(pub u32);
impl Crcr {
    #[doc = "This field indicates the CRC value of the last message transmitted"]
    #[must_use]
    #[inline(always)]
    pub const fn txcrc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "This field indicates the CRC value of the last message transmitted"]
    #[inline(always)]
    pub const fn set_txcrc(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
    #[doc = "This field indicates the number of the Mailbox corresponding to the value in TXCRC field."]
    #[must_use]
    #[inline(always)]
    pub const fn mbcrc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "This field indicates the number of the Mailbox corresponding to the value in TXCRC field."]
    #[inline(always)]
    pub const fn set_mbcrc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
}
impl Default for Crcr {
    #[inline(always)]
    fn default() -> Crcr {
        Crcr(0)
    }
}
impl core::fmt::Debug for Crcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Crcr")
            .field("txcrc", &self.txcrc())
            .field("mbcrc", &self.mbcrc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Crcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Crcr {{ txcrc: {=u16:?}, mbcrc: {=u8:?} }}",
            self.txcrc(),
            self.mbcrc()
        )
    }
}
#[doc = "Control 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl1(pub u32);
impl Ctrl1 {
    #[doc = "This 3-bit field defines the length of the Propagation Segment in the bit time"]
    #[must_use]
    #[inline(always)]
    pub const fn propseg(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "This 3-bit field defines the length of the Propagation Segment in the bit time"]
    #[inline(always)]
    pub const fn set_propseg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "This bit configures FLEXCAN to operate in Listen Only Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lom(&self) -> super::vals::Lom {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Lom::from_bits(val as u8)
    }
    #[doc = "This bit configures FLEXCAN to operate in Listen Only Mode"]
    #[inline(always)]
    pub const fn set_lom(&mut self, val: super::vals::Lom) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "This bit defines the ordering mechanism for Message Buffer transmission"]
    #[must_use]
    #[inline(always)]
    pub const fn lbuf(&self) -> super::vals::Lbuf {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Lbuf::from_bits(val as u8)
    }
    #[doc = "This bit defines the ordering mechanism for Message Buffer transmission"]
    #[inline(always)]
    pub const fn set_lbuf(&mut self, val: super::vals::Lbuf) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "This bit enables a mechanism that resets the free-running timer each time a message is received in Message Buffer 0"]
    #[must_use]
    #[inline(always)]
    pub const fn tsyn(&self) -> super::vals::Tsyn {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Tsyn::from_bits(val as u8)
    }
    #[doc = "This bit enables a mechanism that resets the free-running timer each time a message is received in Message Buffer 0"]
    #[inline(always)]
    pub const fn set_tsyn(&mut self, val: super::vals::Tsyn) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "This bit defines how FLEXCAN recovers from Bus Off state"]
    #[must_use]
    #[inline(always)]
    pub const fn boffrec(&self) -> super::vals::Boffrec {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Boffrec::from_bits(val as u8)
    }
    #[doc = "This bit defines how FLEXCAN recovers from Bus Off state"]
    #[inline(always)]
    pub const fn set_boffrec(&mut self, val: super::vals::Boffrec) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "This bit defines the sampling mode of CAN bits at the FLEXCAN_RX"]
    #[must_use]
    #[inline(always)]
    pub const fn smp(&self) -> super::vals::Smp {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Smp::from_bits(val as u8)
    }
    #[doc = "This bit defines the sampling mode of CAN bits at the FLEXCAN_RX"]
    #[inline(always)]
    pub const fn set_smp(&mut self, val: super::vals::Smp) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "This bit provides a mask for the Rx Warning Interrupt associated with the RWRN_INT flag in the Error and Status Register"]
    #[must_use]
    #[inline(always)]
    pub const fn rwrnmsk(&self) -> super::vals::Rwrnmsk {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Rwrnmsk::from_bits(val as u8)
    }
    #[doc = "This bit provides a mask for the Rx Warning Interrupt associated with the RWRN_INT flag in the Error and Status Register"]
    #[inline(always)]
    pub const fn set_rwrnmsk(&mut self, val: super::vals::Rwrnmsk) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "This bit provides a mask for the Tx Warning Interrupt associated with the TWRN_INT flag in the Error and Status Register"]
    #[must_use]
    #[inline(always)]
    pub const fn twrnmsk(&self) -> super::vals::Twrnmsk {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Twrnmsk::from_bits(val as u8)
    }
    #[doc = "This bit provides a mask for the Tx Warning Interrupt associated with the TWRN_INT flag in the Error and Status Register"]
    #[inline(always)]
    pub const fn set_twrnmsk(&mut self, val: super::vals::Twrnmsk) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "This bit configures FlexCAN to operate in Loop-Back Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lpb(&self) -> super::vals::Lpb {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Lpb::from_bits(val as u8)
    }
    #[doc = "This bit configures FlexCAN to operate in Loop-Back Mode"]
    #[inline(always)]
    pub const fn set_lpb(&mut self, val: super::vals::Lpb) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "This bit provides a mask for the Error Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn errmsk(&self) -> super::vals::Errmsk {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Errmsk::from_bits(val as u8)
    }
    #[doc = "This bit provides a mask for the Error Interrupt."]
    #[inline(always)]
    pub const fn set_errmsk(&mut self, val: super::vals::Errmsk) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "This bit provides a mask for the Bus Off Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn boffmsk(&self) -> super::vals::Boffmsk {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Boffmsk::from_bits(val as u8)
    }
    #[doc = "This bit provides a mask for the Bus Off Interrupt."]
    #[inline(always)]
    pub const fn set_boffmsk(&mut self, val: super::vals::Boffmsk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "This 3-bit field defines the length of Phase Buffer Segment 2 in the bit time"]
    #[must_use]
    #[inline(always)]
    pub const fn pseg2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "This 3-bit field defines the length of Phase Buffer Segment 2 in the bit time"]
    #[inline(always)]
    pub const fn set_pseg2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "This 3-bit field defines the length of Phase Buffer Segment 1 in the bit time"]
    #[must_use]
    #[inline(always)]
    pub const fn pseg1(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x07;
        val as u8
    }
    #[doc = "This 3-bit field defines the length of Phase Buffer Segment 1 in the bit time"]
    #[inline(always)]
    pub const fn set_pseg1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 19usize)) | (((val as u32) & 0x07) << 19usize);
    }
    #[doc = "This 2-bit field defines the maximum number of time quanta One time quantum is equal to the Sclock period"]
    #[must_use]
    #[inline(always)]
    pub const fn rjw(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "This 2-bit field defines the maximum number of time quanta One time quantum is equal to the Sclock period"]
    #[inline(always)]
    pub const fn set_rjw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "This 8-bit field defines the ratio between the PE clock frequency and the Serial Clock (Sclock) frequency"]
    #[must_use]
    #[inline(always)]
    pub const fn presdiv(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "This 8-bit field defines the ratio between the PE clock frequency and the Serial Clock (Sclock) frequency"]
    #[inline(always)]
    pub const fn set_presdiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Ctrl1 {
    #[inline(always)]
    fn default() -> Ctrl1 {
        Ctrl1(0)
    }
}
impl core::fmt::Debug for Ctrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl1")
            .field("propseg", &self.propseg())
            .field("lom", &self.lom())
            .field("lbuf", &self.lbuf())
            .field("tsyn", &self.tsyn())
            .field("boffrec", &self.boffrec())
            .field("smp", &self.smp())
            .field("rwrnmsk", &self.rwrnmsk())
            .field("twrnmsk", &self.twrnmsk())
            .field("lpb", &self.lpb())
            .field("errmsk", &self.errmsk())
            .field("boffmsk", &self.boffmsk())
            .field("pseg2", &self.pseg2())
            .field("pseg1", &self.pseg1())
            .field("rjw", &self.rjw())
            .field("presdiv", &self.presdiv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl1 {{ propseg: {=u8:?}, lom: {:?}, lbuf: {:?}, tsyn: {:?}, boffrec: {:?}, smp: {:?}, rwrnmsk: {:?}, twrnmsk: {:?}, lpb: {:?}, errmsk: {:?}, boffmsk: {:?}, pseg2: {=u8:?}, pseg1: {=u8:?}, rjw: {=u8:?}, presdiv: {=u8:?} }}",
            self.propseg(),
            self.lom(),
            self.lbuf(),
            self.tsyn(),
            self.boffrec(),
            self.smp(),
            self.rwrnmsk(),
            self.twrnmsk(),
            self.lpb(),
            self.errmsk(),
            self.boffmsk(),
            self.pseg2(),
            self.pseg1(),
            self.rjw(),
            self.presdiv()
        )
    }
}
#[doc = "Control 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl2(pub u32);
impl Ctrl2 {
    #[doc = "This bit controls the comparison of IDE and RTR bits within Rx Mailboxes filters with their corresponding bits in the incoming frame by the matching process"]
    #[must_use]
    #[inline(always)]
    pub const fn eacen(&self) -> super::vals::Eacen {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Eacen::from_bits(val as u8)
    }
    #[doc = "This bit controls the comparison of IDE and RTR bits within Rx Mailboxes filters with their corresponding bits in the incoming frame by the matching process"]
    #[inline(always)]
    pub const fn set_eacen(&mut self, val: super::vals::Eacen) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "If this bit is asserted Remote Request Frame is submitted to a matching process and stored in the corresponding Message Buffer in the same fashion of a Data Frame"]
    #[must_use]
    #[inline(always)]
    pub const fn rrs(&self) -> super::vals::Rrs {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Rrs::from_bits(val as u8)
    }
    #[doc = "If this bit is asserted Remote Request Frame is submitted to a matching process and stored in the corresponding Message Buffer in the same fashion of a Data Frame"]
    #[inline(always)]
    pub const fn set_rrs(&mut self, val: super::vals::Rrs) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "If this bit is set the matching process starts from the Mailboxes and if no match occurs the matching continues on the Rx FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn mrp(&self) -> super::vals::Mrp {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Mrp::from_bits(val as u8)
    }
    #[doc = "If this bit is set the matching process starts from the Mailboxes and if no match occurs the matching continues on the Rx FIFO"]
    #[inline(always)]
    pub const fn set_mrp(&mut self, val: super::vals::Mrp) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "This 5-bit field indicates how many CAN bits the Tx arbitration process start point can be delayed from the first bit of CRC field on CAN bus"]
    #[must_use]
    #[inline(always)]
    pub const fn tasd(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "This 5-bit field indicates how many CAN bits the Tx arbitration process start point can be delayed from the first bit of CRC field on CAN bus"]
    #[inline(always)]
    pub const fn set_tasd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "This 4-bit field defines the number of Rx FIFO filters according to"]
    #[must_use]
    #[inline(always)]
    pub const fn rffn(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "This 4-bit field defines the number of Rx FIFO filters according to"]
    #[inline(always)]
    pub const fn set_rffn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Enable unrestricted write access to FlexCAN memory in Freeze mode"]
    #[must_use]
    #[inline(always)]
    pub const fn wrmfrz(&self) -> super::vals::Wrmfrz {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Wrmfrz::from_bits(val as u8)
    }
    #[doc = "Enable unrestricted write access to FlexCAN memory in Freeze mode"]
    #[inline(always)]
    pub const fn set_wrmfrz(&mut self, val: super::vals::Wrmfrz) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
}
impl Default for Ctrl2 {
    #[inline(always)]
    fn default() -> Ctrl2 {
        Ctrl2(0)
    }
}
impl core::fmt::Debug for Ctrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl2")
            .field("eacen", &self.eacen())
            .field("rrs", &self.rrs())
            .field("mrp", &self.mrp())
            .field("tasd", &self.tasd())
            .field("rffn", &self.rffn())
            .field("wrmfrz", &self.wrmfrz())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl2 {{ eacen: {:?}, rrs: {:?}, mrp: {:?}, tasd: {=u8:?}, rffn: {=u8:?}, wrmfrz: {:?} }}",
            self.eacen(),
            self.rrs(),
            self.mrp(),
            self.tasd(),
            self.rffn(),
            self.wrmfrz()
        )
    }
}
#[doc = "Debug 1 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbg1(pub u32);
impl Dbg1 {
    #[doc = "CAN Finite State Machine"]
    #[must_use]
    #[inline(always)]
    pub const fn cfsm(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "CAN Finite State Machine"]
    #[inline(always)]
    pub const fn set_cfsm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "CAN Bit Number"]
    #[must_use]
    #[inline(always)]
    pub const fn cbn(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "CAN Bit Number"]
    #[inline(always)]
    pub const fn set_cbn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
}
impl Default for Dbg1 {
    #[inline(always)]
    fn default() -> Dbg1 {
        Dbg1(0)
    }
}
impl core::fmt::Debug for Dbg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dbg1")
            .field("cfsm", &self.cfsm())
            .field("cbn", &self.cbn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dbg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dbg1 {{ cfsm: {=u8:?}, cbn: {=u8:?} }}",
            self.cfsm(),
            self.cbn()
        )
    }
}
#[doc = "Debug 2 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbg2(pub u32);
impl Dbg2 {
    #[doc = "Rx Matching Pointer"]
    #[must_use]
    #[inline(always)]
    pub const fn rmp(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Rx Matching Pointer"]
    #[inline(always)]
    pub const fn set_rmp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Matching Process in Progress"]
    #[must_use]
    #[inline(always)]
    pub const fn mpp(&self) -> super::vals::Mpp {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Mpp::from_bits(val as u8)
    }
    #[doc = "Matching Process in Progress"]
    #[inline(always)]
    pub const fn set_mpp(&mut self, val: super::vals::Mpp) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Tx Arbitration Pointer"]
    #[must_use]
    #[inline(always)]
    pub const fn tap(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Tx Arbitration Pointer"]
    #[inline(always)]
    pub const fn set_tap(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Arbitration Process in Progress"]
    #[must_use]
    #[inline(always)]
    pub const fn app(&self) -> super::vals::App {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::App::from_bits(val as u8)
    }
    #[doc = "Arbitration Process in Progress"]
    #[inline(always)]
    pub const fn set_app(&mut self, val: super::vals::App) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Dbg2 {
    #[inline(always)]
    fn default() -> Dbg2 {
        Dbg2(0)
    }
}
impl core::fmt::Debug for Dbg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dbg2")
            .field("rmp", &self.rmp())
            .field("mpp", &self.mpp())
            .field("tap", &self.tap())
            .field("app", &self.app())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dbg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dbg2 {{ rmp: {=u8:?}, mpp: {:?}, tap: {=u8:?}, app: {:?} }}",
            self.rmp(),
            self.mpp(),
            self.tap(),
            self.app()
        )
    }
}
#[doc = "Error Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecr(pub u32);
impl Ecr {
    #[doc = "Tx_Err_Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_err_counter(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Tx_Err_Counter"]
    #[inline(always)]
    pub const fn set_tx_err_counter(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Rx_Err_Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_err_counter(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Rx_Err_Counter"]
    #[inline(always)]
    pub const fn set_rx_err_counter(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Ecr {
    #[inline(always)]
    fn default() -> Ecr {
        Ecr(0)
    }
}
impl core::fmt::Debug for Ecr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ecr")
            .field("tx_err_counter", &self.tx_err_counter())
            .field("rx_err_counter", &self.rx_err_counter())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ecr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ecr {{ tx_err_counter: {=u8:?}, rx_err_counter: {=u8:?} }}",
            self.tx_err_counter(),
            self.rx_err_counter()
        )
    }
}
#[doc = "Error and Status 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Esr1(pub u32);
impl Esr1 {
    #[doc = "When FLEXCAN is Stop Mode and a recessive to dominant transition is detected on the CAN bus and if the WAK_MSK bit in the MCR Register is set, an interrupt is generated to the Arm"]
    #[must_use]
    #[inline(always)]
    pub const fn wakint(&self) -> super::vals::Wakint {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Wakint::from_bits(val as u8)
    }
    #[doc = "When FLEXCAN is Stop Mode and a recessive to dominant transition is detected on the CAN bus and if the WAK_MSK bit in the MCR Register is set, an interrupt is generated to the Arm"]
    #[inline(always)]
    pub const fn set_wakint(&mut self, val: super::vals::Wakint) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "This bit indicates that at least one of the Error Bits (bits 15-10) is set"]
    #[must_use]
    #[inline(always)]
    pub const fn errint(&self) -> super::vals::Errint {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Errint::from_bits(val as u8)
    }
    #[doc = "This bit indicates that at least one of the Error Bits (bits 15-10) is set"]
    #[inline(always)]
    pub const fn set_errint(&mut self, val: super::vals::Errint) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "This bit is set when FLEXCAN enters 'Bus Off' state"]
    #[must_use]
    #[inline(always)]
    pub const fn boffint(&self) -> super::vals::Boffint {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Boffint::from_bits(val as u8)
    }
    #[doc = "This bit is set when FLEXCAN enters 'Bus Off' state"]
    #[inline(always)]
    pub const fn set_boffint(&mut self, val: super::vals::Boffint) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit indicates if FlexCAN is receiving a message. Refer to ."]
    #[must_use]
    #[inline(always)]
    pub const fn rx(&self) -> super::vals::Rx {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Rx::from_bits(val as u8)
    }
    #[doc = "This bit indicates if FlexCAN is receiving a message. Refer to ."]
    #[inline(always)]
    pub const fn set_rx(&mut self, val: super::vals::Rx) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "If the LOM bit in the Control Register is asserted, after some delay that depends on the CAN bit timing the FLT_CONF field will indicate \"Error Passive\""]
    #[must_use]
    #[inline(always)]
    pub const fn fltconf(&self) -> super::vals::Fltconf {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Fltconf::from_bits(val as u8)
    }
    #[doc = "If the LOM bit in the Control Register is asserted, after some delay that depends on the CAN bit timing the FLT_CONF field will indicate \"Error Passive\""]
    #[inline(always)]
    pub const fn set_fltconf(&mut self, val: super::vals::Fltconf) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "This bit indicates if FLEXCAN is transmitting a message.Refer to ."]
    #[must_use]
    #[inline(always)]
    pub const fn tx(&self) -> super::vals::Tx {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Tx::from_bits(val as u8)
    }
    #[doc = "This bit indicates if FLEXCAN is transmitting a message.Refer to ."]
    #[inline(always)]
    pub const fn set_tx(&mut self, val: super::vals::Tx) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "This bit indicates when CAN bus is in IDLE state.Refer to ."]
    #[must_use]
    #[inline(always)]
    pub const fn idle(&self) -> super::vals::Idle {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Idle::from_bits(val as u8)
    }
    #[doc = "This bit indicates when CAN bus is in IDLE state.Refer to ."]
    #[inline(always)]
    pub const fn set_idle(&mut self, val: super::vals::Idle) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "This bit indicates when repetitive errors are occurring during message reception."]
    #[must_use]
    #[inline(always)]
    pub const fn rxwrn(&self) -> super::vals::Rxwrn {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Rxwrn::from_bits(val as u8)
    }
    #[doc = "This bit indicates when repetitive errors are occurring during message reception."]
    #[inline(always)]
    pub const fn set_rxwrn(&mut self, val: super::vals::Rxwrn) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "This bit indicates when repetitive errors are occurring during message transmission."]
    #[must_use]
    #[inline(always)]
    pub const fn txwrn(&self) -> super::vals::Txwrn {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Txwrn::from_bits(val as u8)
    }
    #[doc = "This bit indicates when repetitive errors are occurring during message transmission."]
    #[inline(always)]
    pub const fn set_txwrn(&mut self, val: super::vals::Txwrn) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "This bit indicates that a Stuffing Error has been detected."]
    #[must_use]
    #[inline(always)]
    pub const fn stferr(&self) -> super::vals::Stferr {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Stferr::from_bits(val as u8)
    }
    #[doc = "This bit indicates that a Stuffing Error has been detected."]
    #[inline(always)]
    pub const fn set_stferr(&mut self, val: super::vals::Stferr) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "This bit indicates that a Form Error has been detected by the receiver node, i"]
    #[must_use]
    #[inline(always)]
    pub const fn frmerr(&self) -> super::vals::Frmerr {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Frmerr::from_bits(val as u8)
    }
    #[doc = "This bit indicates that a Form Error has been detected by the receiver node, i"]
    #[inline(always)]
    pub const fn set_frmerr(&mut self, val: super::vals::Frmerr) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "This bit indicates that a CRC Error has been detected by the receiver node, i"]
    #[must_use]
    #[inline(always)]
    pub const fn crcerr(&self) -> super::vals::Crcerr {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Crcerr::from_bits(val as u8)
    }
    #[doc = "This bit indicates that a CRC Error has been detected by the receiver node, i"]
    #[inline(always)]
    pub const fn set_crcerr(&mut self, val: super::vals::Crcerr) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "This bit indicates that an Acknowledge Error has been detected by the transmitter node, i"]
    #[must_use]
    #[inline(always)]
    pub const fn ackerr(&self) -> super::vals::Ackerr {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Ackerr::from_bits(val as u8)
    }
    #[doc = "This bit indicates that an Acknowledge Error has been detected by the transmitter node, i"]
    #[inline(always)]
    pub const fn set_ackerr(&mut self, val: super::vals::Ackerr) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "This bit indicates when an inconsistency occurs between the transmitted and the received bit in a message"]
    #[must_use]
    #[inline(always)]
    pub const fn bit0err(&self) -> super::vals::Bit0err {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Bit0err::from_bits(val as u8)
    }
    #[doc = "This bit indicates when an inconsistency occurs between the transmitted and the received bit in a message"]
    #[inline(always)]
    pub const fn set_bit0err(&mut self, val: super::vals::Bit0err) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "This bit indicates when an inconsistency occurs between the transmitted and the received bit in a message"]
    #[must_use]
    #[inline(always)]
    pub const fn bit1err(&self) -> super::vals::Bit1err {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Bit1err::from_bits(val as u8)
    }
    #[doc = "This bit indicates when an inconsistency occurs between the transmitted and the received bit in a message"]
    #[inline(always)]
    pub const fn set_bit1err(&mut self, val: super::vals::Bit1err) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "If the WRN_EN bit in MCR is asserted, the RWRN_INT bit is set when the RX_WRN flag transition from '0' to '1', meaning that the Rx error counters reached 96"]
    #[must_use]
    #[inline(always)]
    pub const fn rwrnint(&self) -> super::vals::Rwrnint {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Rwrnint::from_bits(val as u8)
    }
    #[doc = "If the WRN_EN bit in MCR is asserted, the RWRN_INT bit is set when the RX_WRN flag transition from '0' to '1', meaning that the Rx error counters reached 96"]
    #[inline(always)]
    pub const fn set_rwrnint(&mut self, val: super::vals::Rwrnint) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "If the WRN_EN bit in MCR is asserted, the TWRN_INT bit is set when the TX_WRN flag transition from '0' to '1', meaning that the Tx error counter reached 96"]
    #[must_use]
    #[inline(always)]
    pub const fn twrnint(&self) -> super::vals::Twrnint {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Twrnint::from_bits(val as u8)
    }
    #[doc = "If the WRN_EN bit in MCR is asserted, the TWRN_INT bit is set when the TX_WRN flag transition from '0' to '1', meaning that the Tx error counter reached 96"]
    #[inline(always)]
    pub const fn set_twrnint(&mut self, val: super::vals::Twrnint) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "This read-only flag indicates whether the FlexCAN is synchronized to the CAN bus and able to participate in the communication process"]
    #[must_use]
    #[inline(always)]
    pub const fn synch(&self) -> super::vals::Synch {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Synch::from_bits(val as u8)
    }
    #[doc = "This read-only flag indicates whether the FlexCAN is synchronized to the CAN bus and able to participate in the communication process"]
    #[inline(always)]
    pub const fn set_synch(&mut self, val: super::vals::Synch) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
}
impl Default for Esr1 {
    #[inline(always)]
    fn default() -> Esr1 {
        Esr1(0)
    }
}
impl core::fmt::Debug for Esr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Esr1")
            .field("wakint", &self.wakint())
            .field("errint", &self.errint())
            .field("boffint", &self.boffint())
            .field("rx", &self.rx())
            .field("fltconf", &self.fltconf())
            .field("tx", &self.tx())
            .field("idle", &self.idle())
            .field("rxwrn", &self.rxwrn())
            .field("txwrn", &self.txwrn())
            .field("stferr", &self.stferr())
            .field("frmerr", &self.frmerr())
            .field("crcerr", &self.crcerr())
            .field("ackerr", &self.ackerr())
            .field("bit0err", &self.bit0err())
            .field("bit1err", &self.bit1err())
            .field("rwrnint", &self.rwrnint())
            .field("twrnint", &self.twrnint())
            .field("synch", &self.synch())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Esr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Esr1 {{ wakint: {:?}, errint: {:?}, boffint: {:?}, rx: {:?}, fltconf: {:?}, tx: {:?}, idle: {:?}, rxwrn: {:?}, txwrn: {:?}, stferr: {:?}, frmerr: {:?}, crcerr: {:?}, ackerr: {:?}, bit0err: {:?}, bit1err: {:?}, rwrnint: {:?}, twrnint: {:?}, synch: {:?} }}",
            self.wakint(),
            self.errint(),
            self.boffint(),
            self.rx(),
            self.fltconf(),
            self.tx(),
            self.idle(),
            self.rxwrn(),
            self.txwrn(),
            self.stferr(),
            self.frmerr(),
            self.crcerr(),
            self.ackerr(),
            self.bit0err(),
            self.bit1err(),
            self.rwrnint(),
            self.twrnint(),
            self.synch()
        )
    }
}
#[doc = "Error and Status 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Esr2(pub u32);
impl Esr2 {
    #[doc = "If ESR2\\[VPS\\] is asserted, this bit indicates whether there is any inactive Mailbox (CODE field is either 0b1000 or 0b0000)"]
    #[must_use]
    #[inline(always)]
    pub const fn imb(&self) -> super::vals::Imb {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Imb::from_bits(val as u8)
    }
    #[doc = "If ESR2\\[VPS\\] is asserted, this bit indicates whether there is any inactive Mailbox (CODE field is either 0b1000 or 0b0000)"]
    #[inline(always)]
    pub const fn set_imb(&mut self, val: super::vals::Imb) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "This bit indicates whether IMB and LPTM contents are currently valid or not"]
    #[must_use]
    #[inline(always)]
    pub const fn vps(&self) -> super::vals::Vps {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Vps::from_bits(val as u8)
    }
    #[doc = "This bit indicates whether IMB and LPTM contents are currently valid or not"]
    #[inline(always)]
    pub const fn set_vps(&mut self, val: super::vals::Vps) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "If ESR2\\[VPS\\] is asserted, his 7-bit field indicates the lowest number inactive Mailbox (refer to IMB bit description)"]
    #[must_use]
    #[inline(always)]
    pub const fn lptm(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "If ESR2\\[VPS\\] is asserted, his 7-bit field indicates the lowest number inactive Mailbox (refer to IMB bit description)"]
    #[inline(always)]
    pub const fn set_lptm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
}
impl Default for Esr2 {
    #[inline(always)]
    fn default() -> Esr2 {
        Esr2(0)
    }
}
impl core::fmt::Debug for Esr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Esr2")
            .field("imb", &self.imb())
            .field("vps", &self.vps())
            .field("lptm", &self.lptm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Esr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Esr2 {{ imb: {:?}, vps: {:?}, lptm: {=u8:?} }}",
            self.imb(),
            self.vps(),
            self.lptm()
        )
    }
}
#[doc = "Glitch Filter Width Registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gfwr(pub u32);
impl Gfwr {
    #[doc = "It determines the Glitch Filter Width"]
    #[must_use]
    #[inline(always)]
    pub const fn gfwr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "It determines the Glitch Filter Width"]
    #[inline(always)]
    pub const fn set_gfwr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Gfwr {
    #[inline(always)]
    fn default() -> Gfwr {
        Gfwr(0)
    }
}
impl core::fmt::Debug for Gfwr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gfwr").field("gfwr", &self.gfwr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gfwr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gfwr {{ gfwr: {=u8:?} }}", self.gfwr())
    }
}
#[doc = "Interrupt Flags 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iflag1(pub u32);
impl Iflag1 {
    #[doc = "If the Rx FIFO is not enabled, these bits flag the interrupts for MB0 to MB4"]
    #[must_use]
    #[inline(always)]
    pub const fn buf4to0i(&self) -> super::vals::Buf4to0i {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Buf4to0i::from_bits(val as u8)
    }
    #[doc = "If the Rx FIFO is not enabled, these bits flag the interrupts for MB0 to MB4"]
    #[inline(always)]
    pub const fn set_buf4to0i(&mut self, val: super::vals::Buf4to0i) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "If the Rx FIFO is not enabled, this bit flags the interrupt for MB5"]
    #[must_use]
    #[inline(always)]
    pub const fn buf5i(&self) -> super::vals::Buf5i {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Buf5i::from_bits(val as u8)
    }
    #[doc = "If the Rx FIFO is not enabled, this bit flags the interrupt for MB5"]
    #[inline(always)]
    pub const fn set_buf5i(&mut self, val: super::vals::Buf5i) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "If the Rx FIFO is not enabled, this bit flags the interrupt for MB6"]
    #[must_use]
    #[inline(always)]
    pub const fn buf6i(&self) -> super::vals::Buf6i {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Buf6i::from_bits(val as u8)
    }
    #[doc = "If the Rx FIFO is not enabled, this bit flags the interrupt for MB6"]
    #[inline(always)]
    pub const fn set_buf6i(&mut self, val: super::vals::Buf6i) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "If the Rx FIFO is not enabled, this bit flags the interrupt for MB7"]
    #[must_use]
    #[inline(always)]
    pub const fn buf7i(&self) -> super::vals::Buf7i {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Buf7i::from_bits(val as u8)
    }
    #[doc = "If the Rx FIFO is not enabled, this bit flags the interrupt for MB7"]
    #[inline(always)]
    pub const fn set_buf7i(&mut self, val: super::vals::Buf7i) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Each bit flags the respective FLEXCAN Message Buffer (MB8 to MB31) interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn buf31to8i(&self) -> super::vals::Buf31to8i {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        super::vals::Buf31to8i::from_bits(val as u32)
    }
    #[doc = "Each bit flags the respective FLEXCAN Message Buffer (MB8 to MB31) interrupt."]
    #[inline(always)]
    pub const fn set_buf31to8i(&mut self, val: super::vals::Buf31to8i) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize))
            | (((val.to_bits() as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Iflag1 {
    #[inline(always)]
    fn default() -> Iflag1 {
        Iflag1(0)
    }
}
impl core::fmt::Debug for Iflag1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iflag1")
            .field("buf4to0i", &self.buf4to0i())
            .field("buf5i", &self.buf5i())
            .field("buf6i", &self.buf6i())
            .field("buf7i", &self.buf7i())
            .field("buf31to8i", &self.buf31to8i())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iflag1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Iflag1 {{ buf4to0i: {:?}, buf5i: {:?}, buf6i: {:?}, buf7i: {:?}, buf31to8i: {:?} }}",
            self.buf4to0i(),
            self.buf5i(),
            self.buf6i(),
            self.buf7i(),
            self.buf31to8i()
        )
    }
}
#[doc = "Interrupt Flags 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iflag2(pub u32);
impl Iflag2 {
    #[doc = "Each bit flags the respective FLEXCAN Message Buffer (MB32 to MB63) interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn bufhi(&self) -> super::vals::Bufhi {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Bufhi::from_bits(val as u32)
    }
    #[doc = "Each bit flags the respective FLEXCAN Message Buffer (MB32 to MB63) interrupt."]
    #[inline(always)]
    pub const fn set_bufhi(&mut self, val: super::vals::Bufhi) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Iflag2 {
    #[inline(always)]
    fn default() -> Iflag2 {
        Iflag2(0)
    }
}
impl core::fmt::Debug for Iflag2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iflag2")
            .field("bufhi", &self.bufhi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iflag2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Iflag2 {{ bufhi: {:?} }}", self.bufhi())
    }
}
#[doc = "Interrupt Masks 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Imask1(pub u32);
impl Imask1 {
    #[doc = "Each bit enables or disables the respective FLEXCAN Message Buffer (MB0 to MB31) Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn buflm(&self) -> super::vals::Buflm {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Buflm::from_bits(val as u32)
    }
    #[doc = "Each bit enables or disables the respective FLEXCAN Message Buffer (MB0 to MB31) Interrupt"]
    #[inline(always)]
    pub const fn set_buflm(&mut self, val: super::vals::Buflm) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Imask1 {
    #[inline(always)]
    fn default() -> Imask1 {
        Imask1(0)
    }
}
impl core::fmt::Debug for Imask1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Imask1")
            .field("buflm", &self.buflm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Imask1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Imask1 {{ buflm: {:?} }}", self.buflm())
    }
}
#[doc = "Interrupt Masks 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Imask2(pub u32);
impl Imask2 {
    #[doc = "Each bit enables or disables the respective FLEXCAN Message Buffer (MB32 to MB63) Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn bufhm(&self) -> super::vals::Bufhm {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Bufhm::from_bits(val as u32)
    }
    #[doc = "Each bit enables or disables the respective FLEXCAN Message Buffer (MB32 to MB63) Interrupt"]
    #[inline(always)]
    pub const fn set_bufhm(&mut self, val: super::vals::Bufhm) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Imask2 {
    #[inline(always)]
    fn default() -> Imask2 {
        Imask2(0)
    }
}
impl core::fmt::Debug for Imask2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Imask2")
            .field("bufhm", &self.bufhm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Imask2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Imask2 {{ bufhm: {:?} }}", self.bufhm())
    }
}
#[doc = "Module Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcr(pub u32);
impl Mcr {
    #[doc = "This 7-bit field defines the number of the last Message Buffers that will take part in the matching and arbitration processes"]
    #[must_use]
    #[inline(always)]
    pub const fn maxmb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "This 7-bit field defines the number of the last Message Buffers that will take part in the matching and arbitration processes"]
    #[inline(always)]
    pub const fn set_maxmb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "This 2-bit field identifies the format of the elements of the Rx FIFO filter table, as shown below"]
    #[must_use]
    #[inline(always)]
    pub const fn idam(&self) -> super::vals::Idam {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Idam::from_bits(val as u8)
    }
    #[doc = "This 2-bit field identifies the format of the elements of the Rx FIFO filter table, as shown below"]
    #[inline(always)]
    pub const fn set_idam(&mut self, val: super::vals::Idam) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "This bit is supplied for backwards compatibility reasons"]
    #[must_use]
    #[inline(always)]
    pub const fn aen(&self) -> super::vals::Aen {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Aen::from_bits(val as u8)
    }
    #[doc = "This bit is supplied for backwards compatibility reasons"]
    #[inline(always)]
    pub const fn set_aen(&mut self, val: super::vals::Aen) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "This bit is provided for backwards compatibility reasons"]
    #[must_use]
    #[inline(always)]
    pub const fn lprioen(&self) -> super::vals::Lprioen {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Lprioen::from_bits(val as u8)
    }
    #[doc = "This bit is provided for backwards compatibility reasons"]
    #[inline(always)]
    pub const fn set_lprioen(&mut self, val: super::vals::Lprioen) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "This bit indicates whether Rx matching process will be based either on individual masking and queue or on masking scheme with RXMGMASK, RX14MASK and RX15MASK, RXFGMASK"]
    #[must_use]
    #[inline(always)]
    pub const fn irmq(&self) -> super::vals::Irmq {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Irmq::from_bits(val as u8)
    }
    #[doc = "This bit indicates whether Rx matching process will be based either on individual masking and queue or on masking scheme with RXMGMASK, RX14MASK and RX15MASK, RXFGMASK"]
    #[inline(always)]
    pub const fn set_irmq(&mut self, val: super::vals::Irmq) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "This bit defines whether FlexCAN is allowed to receive frames transmitted by itself"]
    #[must_use]
    #[inline(always)]
    pub const fn srxdis(&self) -> super::vals::Srxdis {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Srxdis::from_bits(val as u8)
    }
    #[doc = "This bit defines whether FlexCAN is allowed to receive frames transmitted by itself"]
    #[inline(always)]
    pub const fn set_srxdis(&mut self, val: super::vals::Srxdis) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "This bit defines whether the integrated low-pass filter is applied to protect the FLEXCAN_RX input from spurious wake up"]
    #[must_use]
    #[inline(always)]
    pub const fn waksrc(&self) -> super::vals::Waksrc {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Waksrc::from_bits(val as u8)
    }
    #[doc = "This bit defines whether the integrated low-pass filter is applied to protect the FLEXCAN_RX input from spurious wake up"]
    #[inline(always)]
    pub const fn set_waksrc(&mut self, val: super::vals::Waksrc) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "This read-only bit indicates that FLEXCAN is either in Disable Mode or Stop Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lpmack(&self) -> super::vals::Lpmack {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Lpmack::from_bits(val as u8)
    }
    #[doc = "This read-only bit indicates that FLEXCAN is either in Disable Mode or Stop Mode"]
    #[inline(always)]
    pub const fn set_lpmack(&mut self, val: super::vals::Lpmack) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "When asserted, this bit enables the generation of the TWRN_INT and RWRN_INT flags in the Error and Status Register"]
    #[must_use]
    #[inline(always)]
    pub const fn wrnen(&self) -> super::vals::Wrnen {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Wrnen::from_bits(val as u8)
    }
    #[doc = "When asserted, this bit enables the generation of the TWRN_INT and RWRN_INT flags in the Error and Status Register"]
    #[inline(always)]
    pub const fn set_wrnen(&mut self, val: super::vals::Wrnen) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "This bit enables the Self Wake Up feature when FLEXCAN is in Stop Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn slfwak(&self) -> super::vals::Slfwak {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Slfwak::from_bits(val as u8)
    }
    #[doc = "This bit enables the Self Wake Up feature when FLEXCAN is in Stop Mode"]
    #[inline(always)]
    pub const fn set_slfwak(&mut self, val: super::vals::Slfwak) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "This bit configures some of the FLEXCAN registers to be either in Supervisor or User Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn supv(&self) -> super::vals::Supv {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Supv::from_bits(val as u8)
    }
    #[doc = "This bit configures some of the FLEXCAN registers to be either in Supervisor or User Mode"]
    #[inline(always)]
    pub const fn set_supv(&mut self, val: super::vals::Supv) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "This read-only bit indicates that FLEXCAN is in Freeze Mode and its prescaler is stopped"]
    #[must_use]
    #[inline(always)]
    pub const fn frzack(&self) -> super::vals::Frzack {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Frzack::from_bits(val as u8)
    }
    #[doc = "This read-only bit indicates that FLEXCAN is in Freeze Mode and its prescaler is stopped"]
    #[inline(always)]
    pub const fn set_frzack(&mut self, val: super::vals::Frzack) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "When this bit is asserted, FlexCAN resets its internal state machines and some of the memory mapped registers"]
    #[must_use]
    #[inline(always)]
    pub const fn softrst(&self) -> super::vals::Softrst {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Softrst::from_bits(val as u8)
    }
    #[doc = "When this bit is asserted, FlexCAN resets its internal state machines and some of the memory mapped registers"]
    #[inline(always)]
    pub const fn set_softrst(&mut self, val: super::vals::Softrst) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "This bit enables the Wake Up Interrupt generation."]
    #[must_use]
    #[inline(always)]
    pub const fn wakmsk(&self) -> super::vals::Wakmsk {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Wakmsk::from_bits(val as u8)
    }
    #[doc = "This bit enables the Wake Up Interrupt generation."]
    #[inline(always)]
    pub const fn set_wakmsk(&mut self, val: super::vals::Wakmsk) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "This read-only bit indicates that FLEXCAN is either in Disable Mode, Stop Mode or Freeze Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn notrdy(&self) -> super::vals::Notrdy {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Notrdy::from_bits(val as u8)
    }
    #[doc = "This read-only bit indicates that FLEXCAN is either in Disable Mode, Stop Mode or Freeze Mode"]
    #[inline(always)]
    pub const fn set_notrdy(&mut self, val: super::vals::Notrdy) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Assertion of this bit puts the FLEXCAN module into Freeze Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::Halt {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Halt::from_bits(val as u8)
    }
    #[doc = "Assertion of this bit puts the FLEXCAN module into Freeze Mode"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::Halt) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "This bit controls whether the Rx FIFO feature is enabled or not"]
    #[must_use]
    #[inline(always)]
    pub const fn rfen(&self) -> super::vals::Rfen {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Rfen::from_bits(val as u8)
    }
    #[doc = "This bit controls whether the Rx FIFO feature is enabled or not"]
    #[inline(always)]
    pub const fn set_rfen(&mut self, val: super::vals::Rfen) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "The FRZ bit specifies the FLEXCAN behavior when the HALT bit in the MCR Register is set or when Debug Mode is requested at Arm level"]
    #[must_use]
    #[inline(always)]
    pub const fn frz(&self) -> super::vals::Frz {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Frz::from_bits(val as u8)
    }
    #[doc = "The FRZ bit specifies the FLEXCAN behavior when the HALT bit in the MCR Register is set or when Debug Mode is requested at Arm level"]
    #[inline(always)]
    pub const fn set_frz(&mut self, val: super::vals::Frz) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "This bit controls whether FLEXCAN is enabled or not"]
    #[must_use]
    #[inline(always)]
    pub const fn mdis(&self) -> super::vals::Mdis {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Mdis::from_bits(val as u8)
    }
    #[doc = "This bit controls whether FLEXCAN is enabled or not"]
    #[inline(always)]
    pub const fn set_mdis(&mut self, val: super::vals::Mdis) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            .field("maxmb", &self.maxmb())
            .field("idam", &self.idam())
            .field("aen", &self.aen())
            .field("lprioen", &self.lprioen())
            .field("irmq", &self.irmq())
            .field("srxdis", &self.srxdis())
            .field("waksrc", &self.waksrc())
            .field("lpmack", &self.lpmack())
            .field("wrnen", &self.wrnen())
            .field("slfwak", &self.slfwak())
            .field("supv", &self.supv())
            .field("frzack", &self.frzack())
            .field("softrst", &self.softrst())
            .field("wakmsk", &self.wakmsk())
            .field("notrdy", &self.notrdy())
            .field("halt", &self.halt())
            .field("rfen", &self.rfen())
            .field("frz", &self.frz())
            .field("mdis", &self.mdis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mcr {{ maxmb: {=u8:?}, idam: {:?}, aen: {:?}, lprioen: {:?}, irmq: {:?}, srxdis: {:?}, waksrc: {:?}, lpmack: {:?}, wrnen: {:?}, slfwak: {:?}, supv: {:?}, frzack: {:?}, softrst: {:?}, wakmsk: {:?}, notrdy: {:?}, halt: {:?}, rfen: {:?}, frz: {:?}, mdis: {:?} }}",
            self.maxmb(),
            self.idam(),
            self.aen(),
            self.lprioen(),
            self.irmq(),
            self.srxdis(),
            self.waksrc(),
            self.lpmack(),
            self.wrnen(),
            self.slfwak(),
            self.supv(),
            self.frzack(),
            self.softrst(),
            self.wakmsk(),
            self.notrdy(),
            self.halt(),
            self.rfen(),
            self.frz(),
            self.mdis()
        )
    }
}
#[doc = "Rx Buffer 14 Mask Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rx14mask(pub u32);
impl Rx14mask {
    #[doc = "These bits mask Mailbox 14 filter bits in the same fashion as RXMGMASK masks other Mailboxes filters (see RXMGMASKRx Mailboxes Global Mask Register )"]
    #[must_use]
    #[inline(always)]
    pub const fn rx14m(&self) -> super::vals::Rx14m {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Rx14m::from_bits(val as u32)
    }
    #[doc = "These bits mask Mailbox 14 filter bits in the same fashion as RXMGMASK masks other Mailboxes filters (see RXMGMASKRx Mailboxes Global Mask Register )"]
    #[inline(always)]
    pub const fn set_rx14m(&mut self, val: super::vals::Rx14m) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Rx14mask {
    #[inline(always)]
    fn default() -> Rx14mask {
        Rx14mask(0)
    }
}
impl core::fmt::Debug for Rx14mask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rx14mask")
            .field("rx14m", &self.rx14m())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rx14mask {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rx14mask {{ rx14m: {:?} }}", self.rx14m())
    }
}
#[doc = "Rx Buffer 15 Mask Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rx15mask(pub u32);
impl Rx15mask {
    #[doc = "These bits mask Mailbox 15 filter bits in the same fashion as RXMGMASK masks other Mailboxes filters (see RXMGMASKRx Mailboxes Global Mask Register )"]
    #[must_use]
    #[inline(always)]
    pub const fn rx15m(&self) -> super::vals::Rx15m {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Rx15m::from_bits(val as u32)
    }
    #[doc = "These bits mask Mailbox 15 filter bits in the same fashion as RXMGMASK masks other Mailboxes filters (see RXMGMASKRx Mailboxes Global Mask Register )"]
    #[inline(always)]
    pub const fn set_rx15m(&mut self, val: super::vals::Rx15m) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Rx15mask {
    #[inline(always)]
    fn default() -> Rx15mask {
        Rx15mask(0)
    }
}
impl core::fmt::Debug for Rx15mask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rx15mask")
            .field("rx15m", &self.rx15m())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rx15mask {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rx15mask {{ rx15m: {:?} }}", self.rx15m())
    }
}
#[doc = "Rx FIFO Global Mask Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxfgmask(pub u32);
impl Rxfgmask {
    #[doc = "These bits mask the ID Filter Table elements bits in a perfect alignment"]
    #[must_use]
    #[inline(always)]
    pub const fn fgm(&self) -> super::vals::Fgm {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Fgm::from_bits(val as u32)
    }
    #[doc = "These bits mask the ID Filter Table elements bits in a perfect alignment"]
    #[inline(always)]
    pub const fn set_fgm(&mut self, val: super::vals::Fgm) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Rxfgmask {
    #[inline(always)]
    fn default() -> Rxfgmask {
        Rxfgmask(0)
    }
}
impl core::fmt::Debug for Rxfgmask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxfgmask")
            .field("fgm", &self.fgm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxfgmask {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rxfgmask {{ fgm: {:?} }}", self.fgm())
    }
}
#[doc = "Rx FIFO Information Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxfir(pub u32);
impl Rxfir {
    #[doc = "This 9-bit field indicates which Identifier Acceptance Filter (see Rx FIFO Structure) was hit by the received message that is in the output of the Rx FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn idhit(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "This 9-bit field indicates which Identifier Acceptance Filter (see Rx FIFO Structure) was hit by the received message that is in the output of the Rx FIFO"]
    #[inline(always)]
    pub const fn set_idhit(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for Rxfir {
    #[inline(always)]
    fn default() -> Rxfir {
        Rxfir(0)
    }
}
impl core::fmt::Debug for Rxfir {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxfir")
            .field("idhit", &self.idhit())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxfir {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rxfir {{ idhit: {=u16:?} }}", self.idhit())
    }
}
#[doc = "Rx Individual Mask Registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rximr(pub u32);
impl Rximr {
    #[doc = "These bits mask both Mailbox filter and Rx FIFO ID Filter Table element in distinct ways"]
    #[must_use]
    #[inline(always)]
    pub const fn mi(&self) -> super::vals::Mi {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Mi::from_bits(val as u32)
    }
    #[doc = "These bits mask both Mailbox filter and Rx FIFO ID Filter Table element in distinct ways"]
    #[inline(always)]
    pub const fn set_mi(&mut self, val: super::vals::Mi) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Rximr {
    #[inline(always)]
    fn default() -> Rximr {
        Rximr(0)
    }
}
impl core::fmt::Debug for Rximr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rximr").field("mi", &self.mi()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rximr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rximr {{ mi: {:?} }}", self.mi())
    }
}
#[doc = "Rx Mailboxes Global Mask Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxmgmask(pub u32);
impl Rxmgmask {
    #[doc = "These bits mask the Mailbox filter bits as shown in the figure above"]
    #[must_use]
    #[inline(always)]
    pub const fn mg(&self) -> super::vals::Mg {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Mg::from_bits(val as u32)
    }
    #[doc = "These bits mask the Mailbox filter bits as shown in the figure above"]
    #[inline(always)]
    pub const fn set_mg(&mut self, val: super::vals::Mg) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Rxmgmask {
    #[inline(always)]
    fn default() -> Rxmgmask {
        Rxmgmask(0)
    }
}
impl core::fmt::Debug for Rxmgmask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxmgmask").field("mg", &self.mg()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxmgmask {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rxmgmask {{ mg: {:?} }}", self.mg())
    }
}
#[doc = "Free Running Timer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer(pub u32);
impl Timer {
    #[doc = "TIMER"]
    #[must_use]
    #[inline(always)]
    pub const fn timer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "TIMER"]
    #[inline(always)]
    pub const fn set_timer(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Timer {
    #[inline(always)]
    fn default() -> Timer {
        Timer(0)
    }
}
impl core::fmt::Debug for Timer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timer")
            .field("timer", &self.timer())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Timer {{ timer: {=u16:?} }}", self.timer())
    }
}
