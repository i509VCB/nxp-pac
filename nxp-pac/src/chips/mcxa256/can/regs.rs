#[doc = "CAN Bit Timing"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cbt(pub u32);
impl Cbt {
    #[doc = "Extended Phase Segment 2"]
    #[must_use]
    #[inline(always)]
    pub const fn epseg2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Extended Phase Segment 2"]
    #[inline(always)]
    pub const fn set_epseg2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Extended Phase Segment 1"]
    #[must_use]
    #[inline(always)]
    pub const fn epseg1(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x1f;
        val as u8
    }
    #[doc = "Extended Phase Segment 1"]
    #[inline(always)]
    pub const fn set_epseg1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
    }
    #[doc = "Extended Propagation Segment"]
    #[must_use]
    #[inline(always)]
    pub const fn epropseg(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x3f;
        val as u8
    }
    #[doc = "Extended Propagation Segment"]
    #[inline(always)]
    pub const fn set_epropseg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 10usize)) | (((val as u32) & 0x3f) << 10usize);
    }
    #[doc = "Extended Resync Jump Width"]
    #[must_use]
    #[inline(always)]
    pub const fn erjw(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Extended Resync Jump Width"]
    #[inline(always)]
    pub const fn set_erjw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Extended Prescaler Division Factor"]
    #[must_use]
    #[inline(always)]
    pub const fn epresdiv(&self) -> u16 {
        let val = (self.0 >> 21usize) & 0x03ff;
        val as u16
    }
    #[doc = "Extended Prescaler Division Factor"]
    #[inline(always)]
    pub const fn set_epresdiv(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 21usize)) | (((val as u32) & 0x03ff) << 21usize);
    }
    #[doc = "Bit Timing Format Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn btf(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Timing Format Enable"]
    #[inline(always)]
    pub const fn set_btf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Cbt {
    #[inline(always)]
    fn default() -> Cbt {
        Cbt(0)
    }
}
impl core::fmt::Debug for Cbt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cbt")
            .field("epseg2", &self.epseg2())
            .field("epseg1", &self.epseg1())
            .field("epropseg", &self.epropseg())
            .field("erjw", &self.erjw())
            .field("epresdiv", &self.epresdiv())
            .field("btf", &self.btf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cbt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cbt {{ epseg2: {=u8:?}, epseg1: {=u8:?}, epropseg: {=u8:?}, erjw: {=u8:?}, epresdiv: {=u16:?}, btf: {=bool:?} }}",
            self.epseg2(),
            self.epseg1(),
            self.epropseg(),
            self.erjw(),
            self.epresdiv(),
            self.btf()
        )
    }
}
#[doc = "Cyclic Redundancy Check"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crcr(pub u32);
impl Crcr {
    #[doc = "Transmitted CRC value"]
    #[must_use]
    #[inline(always)]
    pub const fn txcrc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Transmitted CRC value"]
    #[inline(always)]
    pub const fn set_txcrc(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
    #[doc = "CRC Message Buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn mbcrc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "CRC Message Buffer"]
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
#[doc = "Message Buffer 0 CS Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs(pub u32);
impl Cs {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    #[must_use]
    #[inline(always)]
    pub const fn time_stamp(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    #[inline(always)]
    pub const fn set_time_stamp(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Length of the data to be stored/transmitted."]
    #[must_use]
    #[inline(always)]
    pub const fn dlc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Length of the data to be stored/transmitted."]
    #[inline(always)]
    pub const fn set_dlc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    #[must_use]
    #[inline(always)]
    pub const fn rtr(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    #[inline(always)]
    pub const fn set_rtr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    #[must_use]
    #[inline(always)]
    pub const fn ide(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    #[inline(always)]
    pub const fn set_ide(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    #[must_use]
    #[inline(always)]
    pub const fn srr(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    #[inline(always)]
    pub const fn set_srr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    #[must_use]
    #[inline(always)]
    pub const fn code(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    #[inline(always)]
    pub const fn set_code(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    #[must_use]
    #[inline(always)]
    pub const fn esi(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    #[inline(always)]
    pub const fn set_esi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    #[must_use]
    #[inline(always)]
    pub const fn brs(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    #[inline(always)]
    pub const fn set_brs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    #[must_use]
    #[inline(always)]
    pub const fn edl(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    #[inline(always)]
    pub const fn set_edl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Cs {
    #[inline(always)]
    fn default() -> Cs {
        Cs(0)
    }
}
impl core::fmt::Debug for Cs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cs")
            .field("time_stamp", &self.time_stamp())
            .field("dlc", &self.dlc())
            .field("rtr", &self.rtr())
            .field("ide", &self.ide())
            .field("srr", &self.srr())
            .field("code", &self.code())
            .field("esi", &self.esi())
            .field("brs", &self.brs())
            .field("edl", &self.edl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cs {{ time_stamp: {=u16:?}, dlc: {=u8:?}, rtr: {=bool:?}, ide: {=bool:?}, srr: {=bool:?}, code: {=u8:?}, esi: {=bool:?}, brs: {=bool:?}, edl: {=bool:?} }}",
            self.time_stamp(),
            self.dlc(),
            self.rtr(),
            self.ide(),
            self.srr(),
            self.code(),
            self.esi(),
            self.brs(),
            self.edl()
        )
    }
}
#[doc = "Control 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl1(pub u32);
impl Ctrl1 {
    #[doc = "Propagation Segment"]
    #[must_use]
    #[inline(always)]
    pub const fn propseg(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Propagation Segment"]
    #[inline(always)]
    pub const fn set_propseg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Listen-Only Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lom(&self) -> super::vals::Lom {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Lom::from_bits(val as u8)
    }
    #[doc = "Listen-Only Mode"]
    #[inline(always)]
    pub const fn set_lom(&mut self, val: super::vals::Lom) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Lowest Buffer Transmitted First"]
    #[must_use]
    #[inline(always)]
    pub const fn lbuf(&self) -> super::vals::Lbuf {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Lbuf::from_bits(val as u8)
    }
    #[doc = "Lowest Buffer Transmitted First"]
    #[inline(always)]
    pub const fn set_lbuf(&mut self, val: super::vals::Lbuf) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Timer Sync"]
    #[must_use]
    #[inline(always)]
    pub const fn tsyn(&self) -> super::vals::Tsyn {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Tsyn::from_bits(val as u8)
    }
    #[doc = "Timer Sync"]
    #[inline(always)]
    pub const fn set_tsyn(&mut self, val: super::vals::Tsyn) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Bus Off Recovery"]
    #[must_use]
    #[inline(always)]
    pub const fn boffrec(&self) -> super::vals::Boffrec {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Boffrec::from_bits(val as u8)
    }
    #[doc = "Bus Off Recovery"]
    #[inline(always)]
    pub const fn set_boffrec(&mut self, val: super::vals::Boffrec) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "CAN Bit Sampling"]
    #[must_use]
    #[inline(always)]
    pub const fn smp(&self) -> super::vals::Smp {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Smp::from_bits(val as u8)
    }
    #[doc = "CAN Bit Sampling"]
    #[inline(always)]
    pub const fn set_smp(&mut self, val: super::vals::Smp) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "RX Warning Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn rwrnmsk(&self) -> super::vals::Rwrnmsk {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Rwrnmsk::from_bits(val as u8)
    }
    #[doc = "RX Warning Interrupt Mask"]
    #[inline(always)]
    pub const fn set_rwrnmsk(&mut self, val: super::vals::Rwrnmsk) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "TX Warning Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn twrnmsk(&self) -> super::vals::Twrnmsk {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Twrnmsk::from_bits(val as u8)
    }
    #[doc = "TX Warning Interrupt Mask"]
    #[inline(always)]
    pub const fn set_twrnmsk(&mut self, val: super::vals::Twrnmsk) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Loopback Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lpb(&self) -> super::vals::Lpb {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Lpb::from_bits(val as u8)
    }
    #[doc = "Loopback Mode"]
    #[inline(always)]
    pub const fn set_lpb(&mut self, val: super::vals::Lpb) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Error Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn errmsk(&self) -> super::vals::Errmsk {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Errmsk::from_bits(val as u8)
    }
    #[doc = "Error Interrupt Mask"]
    #[inline(always)]
    pub const fn set_errmsk(&mut self, val: super::vals::Errmsk) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Bus Off Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn boffmsk(&self) -> super::vals::Boffmsk {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Boffmsk::from_bits(val as u8)
    }
    #[doc = "Bus Off Interrupt Mask"]
    #[inline(always)]
    pub const fn set_boffmsk(&mut self, val: super::vals::Boffmsk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Phase Segment 2"]
    #[must_use]
    #[inline(always)]
    pub const fn pseg2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Phase Segment 2"]
    #[inline(always)]
    pub const fn set_pseg2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Phase Segment 1"]
    #[must_use]
    #[inline(always)]
    pub const fn pseg1(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x07;
        val as u8
    }
    #[doc = "Phase Segment 1"]
    #[inline(always)]
    pub const fn set_pseg1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 19usize)) | (((val as u32) & 0x07) << 19usize);
    }
    #[doc = "Resync Jump Width"]
    #[must_use]
    #[inline(always)]
    pub const fn rjw(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "Resync Jump Width"]
    #[inline(always)]
    pub const fn set_rjw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "Prescaler Division Factor"]
    #[must_use]
    #[inline(always)]
    pub const fn presdiv(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Prescaler Division Factor"]
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
#[doc = "Pretended Networking Control 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl1Pn(pub u32);
impl Ctrl1Pn {
    #[doc = "Filtering Combination Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn fcs(&self) -> super::vals::Fcs {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Fcs::from_bits(val as u8)
    }
    #[doc = "Filtering Combination Selection"]
    #[inline(always)]
    pub const fn set_fcs(&mut self, val: super::vals::Fcs) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "ID Filtering Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn idfs(&self) -> super::vals::Idfs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Idfs::from_bits(val as u8)
    }
    #[doc = "ID Filtering Selection"]
    #[inline(always)]
    pub const fn set_idfs(&mut self, val: super::vals::Idfs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Payload Filtering Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn plfs(&self) -> super::vals::Plfs {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Plfs::from_bits(val as u8)
    }
    #[doc = "Payload Filtering Selection"]
    #[inline(always)]
    pub const fn set_plfs(&mut self, val: super::vals::Plfs) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Number of Messages Matching the Same Filtering Criteria"]
    #[must_use]
    #[inline(always)]
    pub const fn nmatch(&self) -> super::vals::Nmatch {
        let val = (self.0 >> 8usize) & 0xff;
        super::vals::Nmatch::from_bits(val as u8)
    }
    #[doc = "Number of Messages Matching the Same Filtering Criteria"]
    #[inline(always)]
    pub const fn set_nmatch(&mut self, val: super::vals::Nmatch) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u32) & 0xff) << 8usize);
    }
    #[doc = "Wake-up by Matching Flag Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn wumf_msk(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up by Matching Flag Mask"]
    #[inline(always)]
    pub const fn set_wumf_msk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Wake-up by Timeout Flag Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn wtof_msk(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up by Timeout Flag Mask"]
    #[inline(always)]
    pub const fn set_wtof_msk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Ctrl1Pn {
    #[inline(always)]
    fn default() -> Ctrl1Pn {
        Ctrl1Pn(0)
    }
}
impl core::fmt::Debug for Ctrl1Pn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl1Pn")
            .field("fcs", &self.fcs())
            .field("idfs", &self.idfs())
            .field("plfs", &self.plfs())
            .field("nmatch", &self.nmatch())
            .field("wumf_msk", &self.wumf_msk())
            .field("wtof_msk", &self.wtof_msk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl1Pn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl1Pn {{ fcs: {:?}, idfs: {:?}, plfs: {:?}, nmatch: {:?}, wumf_msk: {=bool:?}, wtof_msk: {=bool:?} }}",
            self.fcs(),
            self.idfs(),
            self.plfs(),
            self.nmatch(),
            self.wumf_msk(),
            self.wtof_msk()
        )
    }
}
#[doc = "Control 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl2(pub u32);
impl Ctrl2 {
    #[doc = "Payload Byte and Bit Order Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn pes(&self) -> super::vals::Pes {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pes::from_bits(val as u8)
    }
    #[doc = "Payload Byte and Bit Order Selection"]
    #[inline(always)]
    pub const fn set_pes(&mut self, val: super::vals::Pes) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "ACK Suppression Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn asd(&self) -> super::vals::Asd {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Asd::from_bits(val as u8)
    }
    #[doc = "ACK Suppression Disable"]
    #[inline(always)]
    pub const fn set_asd(&mut self, val: super::vals::Asd) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Edge Filter Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn edfltdis(&self) -> super::vals::Edfltdis {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Edfltdis::from_bits(val as u8)
    }
    #[doc = "Edge Filter Disable"]
    #[inline(always)]
    pub const fn set_edfltdis(&mut self, val: super::vals::Edfltdis) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "ISO CAN FD Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn isocanfden(&self) -> super::vals::Isocanfden {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Isocanfden::from_bits(val as u8)
    }
    #[doc = "ISO CAN FD Enable"]
    #[inline(always)]
    pub const fn set_isocanfden(&mut self, val: super::vals::Isocanfden) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Bit Timing Expansion Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bte(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Timing Expansion Enable"]
    #[inline(always)]
    pub const fn set_bte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Protocol Exception Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn prexcen(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Protocol Exception Enable"]
    #[inline(always)]
    pub const fn set_prexcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Entire Frame Arbitration Field Comparison Enable for RX Message Buffers"]
    #[must_use]
    #[inline(always)]
    pub const fn eacen(&self) -> super::vals::Eacen {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Eacen::from_bits(val as u8)
    }
    #[doc = "Entire Frame Arbitration Field Comparison Enable for RX Message Buffers"]
    #[inline(always)]
    pub const fn set_eacen(&mut self, val: super::vals::Eacen) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Remote Request Storing"]
    #[must_use]
    #[inline(always)]
    pub const fn rrs(&self) -> super::vals::Rrs {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Rrs::from_bits(val as u8)
    }
    #[doc = "Remote Request Storing"]
    #[inline(always)]
    pub const fn set_rrs(&mut self, val: super::vals::Rrs) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Message Buffers Reception Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn mrp(&self) -> super::vals::Mrp {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Mrp::from_bits(val as u8)
    }
    #[doc = "Message Buffers Reception Priority"]
    #[inline(always)]
    pub const fn set_mrp(&mut self, val: super::vals::Mrp) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Transmission Arbitration Start Delay"]
    #[must_use]
    #[inline(always)]
    pub const fn tasd(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "Transmission Arbitration Start Delay"]
    #[inline(always)]
    pub const fn set_tasd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "Number of Legacy Receive FIFO Filters"]
    #[must_use]
    #[inline(always)]
    pub const fn rffn(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Legacy Receive FIFO Filters"]
    #[inline(always)]
    pub const fn set_rffn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Bus Off Done Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn boffdonemsk(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Off Done Interrupt Mask"]
    #[inline(always)]
    pub const fn set_boffdonemsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Error Interrupt Mask for Errors Detected in the Data Phase of Fast CAN FD Frames"]
    #[must_use]
    #[inline(always)]
    pub const fn errmsk_fast(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Error Interrupt Mask for Errors Detected in the Data Phase of Fast CAN FD Frames"]
    #[inline(always)]
    pub const fn set_errmsk_fast(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            .field("pes", &self.pes())
            .field("asd", &self.asd())
            .field("edfltdis", &self.edfltdis())
            .field("isocanfden", &self.isocanfden())
            .field("bte", &self.bte())
            .field("prexcen", &self.prexcen())
            .field("eacen", &self.eacen())
            .field("rrs", &self.rrs())
            .field("mrp", &self.mrp())
            .field("tasd", &self.tasd())
            .field("rffn", &self.rffn())
            .field("boffdonemsk", &self.boffdonemsk())
            .field("errmsk_fast", &self.errmsk_fast())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl2 {{ pes: {:?}, asd: {:?}, edfltdis: {:?}, isocanfden: {:?}, bte: {=bool:?}, prexcen: {=bool:?}, eacen: {:?}, rrs: {:?}, mrp: {:?}, tasd: {=u8:?}, rffn: {=u8:?}, boffdonemsk: {=bool:?}, errmsk_fast: {=bool:?} }}",
            self.pes(),
            self.asd(),
            self.edfltdis(),
            self.isocanfden(),
            self.bte(),
            self.prexcen(),
            self.eacen(),
            self.rrs(),
            self.mrp(),
            self.tasd(),
            self.rffn(),
            self.boffdonemsk(),
            self.errmsk_fast()
        )
    }
}
#[doc = "Pretended Networking Control 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl2Pn(pub u32);
impl Ctrl2Pn {
    #[doc = "Timeout for No Message Matching the Filtering Criteria"]
    #[must_use]
    #[inline(always)]
    pub const fn matchto(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Timeout for No Message Matching the Filtering Criteria"]
    #[inline(always)]
    pub const fn set_matchto(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ctrl2Pn {
    #[inline(always)]
    fn default() -> Ctrl2Pn {
        Ctrl2Pn(0)
    }
}
impl core::fmt::Debug for Ctrl2Pn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl2Pn")
            .field("matchto", &self.matchto())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl2Pn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctrl2Pn {{ matchto: {=u16:?} }}", self.matchto())
    }
}
#[doc = "Error Counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecr(pub u32);
impl Ecr {
    #[doc = "Transmit Error Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn txerrcnt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit Error Counter"]
    #[inline(always)]
    pub const fn set_txerrcnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Receive Error Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn rxerrcnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Receive Error Counter"]
    #[inline(always)]
    pub const fn set_rxerrcnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Transmit Error Counter for Fast Bits"]
    #[must_use]
    #[inline(always)]
    pub const fn txerrcnt_fast(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit Error Counter for Fast Bits"]
    #[inline(always)]
    pub const fn set_txerrcnt_fast(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Receive Error Counter for Fast Bits"]
    #[must_use]
    #[inline(always)]
    pub const fn rxerrcnt_fast(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Receive Error Counter for Fast Bits"]
    #[inline(always)]
    pub const fn set_rxerrcnt_fast(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
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
            .field("txerrcnt", &self.txerrcnt())
            .field("rxerrcnt", &self.rxerrcnt())
            .field("txerrcnt_fast", &self.txerrcnt_fast())
            .field("rxerrcnt_fast", &self.rxerrcnt_fast())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ecr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ecr {{ txerrcnt: {=u8:?}, rxerrcnt: {=u8:?}, txerrcnt_fast: {=u8:?}, rxerrcnt_fast: {=u8:?} }}",
            self.txerrcnt(),
            self.rxerrcnt(),
            self.txerrcnt_fast(),
            self.rxerrcnt_fast()
        )
    }
}
#[doc = "Enhanced Data Phase CAN Bit Timing"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Edcbt(pub u32);
impl Edcbt {
    #[doc = "Data Phase Segment 1"]
    #[must_use]
    #[inline(always)]
    pub const fn dtseg1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Data Phase Segment 1"]
    #[inline(always)]
    pub const fn set_dtseg1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Data Phase Time Segment 2"]
    #[must_use]
    #[inline(always)]
    pub const fn dtseg2(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Data Phase Time Segment 2"]
    #[inline(always)]
    pub const fn set_dtseg2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Data Phase Resynchronization Jump Width"]
    #[must_use]
    #[inline(always)]
    pub const fn drjw(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x0f;
        val as u8
    }
    #[doc = "Data Phase Resynchronization Jump Width"]
    #[inline(always)]
    pub const fn set_drjw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 22usize)) | (((val as u32) & 0x0f) << 22usize);
    }
}
impl Default for Edcbt {
    #[inline(always)]
    fn default() -> Edcbt {
        Edcbt(0)
    }
}
impl core::fmt::Debug for Edcbt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Edcbt")
            .field("dtseg1", &self.dtseg1())
            .field("dtseg2", &self.dtseg2())
            .field("drjw", &self.drjw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Edcbt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Edcbt {{ dtseg1: {=u8:?}, dtseg2: {=u8:?}, drjw: {=u8:?} }}",
            self.dtseg1(),
            self.dtseg2(),
            self.drjw()
        )
    }
}
#[doc = "Enhanced Nominal CAN Bit Timing"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Encbt(pub u32);
impl Encbt {
    #[doc = "Nominal Time Segment 1"]
    #[must_use]
    #[inline(always)]
    pub const fn ntseg1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Nominal Time Segment 1"]
    #[inline(always)]
    pub const fn set_ntseg1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Nominal Time Segment 2"]
    #[must_use]
    #[inline(always)]
    pub const fn ntseg2(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x7f;
        val as u8
    }
    #[doc = "Nominal Time Segment 2"]
    #[inline(always)]
    pub const fn set_ntseg2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 12usize)) | (((val as u32) & 0x7f) << 12usize);
    }
    #[doc = "Nominal Resynchronization Jump Width"]
    #[must_use]
    #[inline(always)]
    pub const fn nrjw(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x7f;
        val as u8
    }
    #[doc = "Nominal Resynchronization Jump Width"]
    #[inline(always)]
    pub const fn set_nrjw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 22usize)) | (((val as u32) & 0x7f) << 22usize);
    }
}
impl Default for Encbt {
    #[inline(always)]
    fn default() -> Encbt {
        Encbt(0)
    }
}
impl core::fmt::Debug for Encbt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Encbt")
            .field("ntseg1", &self.ntseg1())
            .field("ntseg2", &self.ntseg2())
            .field("nrjw", &self.nrjw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Encbt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Encbt {{ ntseg1: {=u8:?}, ntseg2: {=u8:?}, nrjw: {=u8:?} }}",
            self.ntseg1(),
            self.ntseg2(),
            self.nrjw()
        )
    }
}
#[doc = "Enhanced CAN Bit Timing Prescalers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eprs(pub u32);
impl Eprs {
    #[doc = "Extended Nominal Prescaler Division Factor"]
    #[must_use]
    #[inline(always)]
    pub const fn enpresdiv(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Extended Nominal Prescaler Division Factor"]
    #[inline(always)]
    pub const fn set_enpresdiv(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Extended Data Phase Prescaler Division Factor"]
    #[must_use]
    #[inline(always)]
    pub const fn edpresdiv(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Extended Data Phase Prescaler Division Factor"]
    #[inline(always)]
    pub const fn set_edpresdiv(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for Eprs {
    #[inline(always)]
    fn default() -> Eprs {
        Eprs(0)
    }
}
impl core::fmt::Debug for Eprs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eprs")
            .field("enpresdiv", &self.enpresdiv())
            .field("edpresdiv", &self.edpresdiv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eprs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Eprs {{ enpresdiv: {=u16:?}, edpresdiv: {=u16:?} }}",
            self.enpresdiv(),
            self.edpresdiv()
        )
    }
}
#[doc = "Enhanced RX FIFO Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Erfcr(pub u32);
impl Erfcr {
    #[doc = "Enhanced RX FIFO Watermark"]
    #[must_use]
    #[inline(always)]
    pub const fn erfwm(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Enhanced RX FIFO Watermark"]
    #[inline(always)]
    pub const fn set_erfwm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Number of Enhanced RX FIFO Filter Elements"]
    #[must_use]
    #[inline(always)]
    pub const fn nfe(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Number of Enhanced RX FIFO Filter Elements"]
    #[inline(always)]
    pub const fn set_nfe(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Number of Extended ID Filter Elements"]
    #[must_use]
    #[inline(always)]
    pub const fn nexif(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "Number of Extended ID Filter Elements"]
    #[inline(always)]
    pub const fn set_nexif(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[doc = "DMA Last Word"]
    #[must_use]
    #[inline(always)]
    pub const fn dmalw(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x1f;
        val as u8
    }
    #[doc = "DMA Last Word"]
    #[inline(always)]
    pub const fn set_dmalw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 26usize)) | (((val as u32) & 0x1f) << 26usize);
    }
    #[doc = "Enhanced RX FIFO enable"]
    #[must_use]
    #[inline(always)]
    pub const fn erfen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enhanced RX FIFO enable"]
    #[inline(always)]
    pub const fn set_erfen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Erfcr {
    #[inline(always)]
    fn default() -> Erfcr {
        Erfcr(0)
    }
}
impl core::fmt::Debug for Erfcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Erfcr")
            .field("erfwm", &self.erfwm())
            .field("nfe", &self.nfe())
            .field("nexif", &self.nexif())
            .field("dmalw", &self.dmalw())
            .field("erfen", &self.erfen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Erfcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Erfcr {{ erfwm: {=u8:?}, nfe: {=u8:?}, nexif: {=u8:?}, dmalw: {=u8:?}, erfen: {=bool:?} }}",
            self.erfwm(),
            self.nfe(),
            self.nexif(),
            self.dmalw(),
            self.erfen()
        )
    }
}
#[doc = "Enhanced RX FIFO Filter Element"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Erffel(pub u32);
impl Erffel {
    #[doc = "Filter Element Bits"]
    #[must_use]
    #[inline(always)]
    pub const fn fel(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Filter Element Bits"]
    #[inline(always)]
    pub const fn set_fel(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Erffel {
    #[inline(always)]
    fn default() -> Erffel {
        Erffel(0)
    }
}
impl core::fmt::Debug for Erffel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Erffel").field("fel", &self.fel()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Erffel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Erffel {{ fel: {=u32:?} }}", self.fel())
    }
}
#[doc = "Enhanced RX FIFO Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Erfier(pub u32);
impl Erfier {
    #[doc = "Enhanced RX FIFO Data Available Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn erfdaie(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Enhanced RX FIFO Data Available Interrupt Enable"]
    #[inline(always)]
    pub const fn set_erfdaie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Enhanced RX FIFO Watermark Indication Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn erfwmiie(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Enhanced RX FIFO Watermark Indication Interrupt Enable"]
    #[inline(always)]
    pub const fn set_erfwmiie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Enhanced RX FIFO Overflow Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn erfovfie(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Enhanced RX FIFO Overflow Interrupt Enable"]
    #[inline(always)]
    pub const fn set_erfovfie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Enhanced RX FIFO Underflow Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn erfufwie(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enhanced RX FIFO Underflow Interrupt Enable"]
    #[inline(always)]
    pub const fn set_erfufwie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Erfier {
    #[inline(always)]
    fn default() -> Erfier {
        Erfier(0)
    }
}
impl core::fmt::Debug for Erfier {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Erfier")
            .field("erfdaie", &self.erfdaie())
            .field("erfwmiie", &self.erfwmiie())
            .field("erfovfie", &self.erfovfie())
            .field("erfufwie", &self.erfufwie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Erfier {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Erfier {{ erfdaie: {=bool:?}, erfwmiie: {=bool:?}, erfovfie: {=bool:?}, erfufwie: {=bool:?} }}",
            self.erfdaie(),
            self.erfwmiie(),
            self.erfovfie(),
            self.erfufwie()
        )
    }
}
#[doc = "Enhanced RX FIFO Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Erfsr(pub u32);
impl Erfsr {
    #[doc = "Enhanced RX FIFO Elements"]
    #[must_use]
    #[inline(always)]
    pub const fn erfel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Enhanced RX FIFO Elements"]
    #[inline(always)]
    pub const fn set_erfel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Enhanced RX FIFO Full Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn erff(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enhanced RX FIFO Full Flag"]
    #[inline(always)]
    pub const fn set_erff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Enhanced RX FIFO Empty Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn erfe(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Enhanced RX FIFO Empty Flag"]
    #[inline(always)]
    pub const fn set_erfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enhanced RX FIFO Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn erfclr(&self) -> super::vals::Erfclr {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Erfclr::from_bits(val as u8)
    }
    #[doc = "Enhanced RX FIFO Clear"]
    #[inline(always)]
    pub const fn set_erfclr(&mut self, val: super::vals::Erfclr) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Enhanced RX FIFO Data Available Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn erfda(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Enhanced RX FIFO Data Available Flag"]
    #[inline(always)]
    pub const fn set_erfda(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Enhanced RX FIFO Watermark Indication Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn erfwmi(&self) -> super::vals::Erfwmi {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Erfwmi::from_bits(val as u8)
    }
    #[doc = "Enhanced RX FIFO Watermark Indication Flag"]
    #[inline(always)]
    pub const fn set_erfwmi(&mut self, val: super::vals::Erfwmi) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Enhanced RX FIFO Overflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn erfovf(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Enhanced RX FIFO Overflow Flag"]
    #[inline(always)]
    pub const fn set_erfovf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Enhanced RX FIFO Underflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn erfufw(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enhanced RX FIFO Underflow Flag"]
    #[inline(always)]
    pub const fn set_erfufw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Erfsr {
    #[inline(always)]
    fn default() -> Erfsr {
        Erfsr(0)
    }
}
impl core::fmt::Debug for Erfsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Erfsr")
            .field("erfel", &self.erfel())
            .field("erff", &self.erff())
            .field("erfe", &self.erfe())
            .field("erfclr", &self.erfclr())
            .field("erfda", &self.erfda())
            .field("erfwmi", &self.erfwmi())
            .field("erfovf", &self.erfovf())
            .field("erfufw", &self.erfufw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Erfsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Erfsr {{ erfel: {=u8:?}, erff: {=bool:?}, erfe: {=bool:?}, erfclr: {:?}, erfda: {=bool:?}, erfwmi: {:?}, erfovf: {=bool:?}, erfufw: {=bool:?} }}",
            self.erfel(),
            self.erff(),
            self.erfe(),
            self.erfclr(),
            self.erfda(),
            self.erfwmi(),
            self.erfovf(),
            self.erfufw()
        )
    }
}
#[doc = "Error and Status 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Esr1(pub u32);
impl Esr1 {
    #[doc = "Wake-up Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn wakint(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up Interrupt Flag"]
    #[inline(always)]
    pub const fn set_wakint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Error Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn errint(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Error Interrupt Flag"]
    #[inline(always)]
    pub const fn set_errint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Bus Off Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn boffint(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Off Interrupt Flag"]
    #[inline(always)]
    pub const fn set_boffint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "FlexCAN in Reception Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rx(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "FlexCAN in Reception Flag"]
    #[inline(always)]
    pub const fn set_rx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Fault Confinement State"]
    #[must_use]
    #[inline(always)]
    pub const fn fltconf(&self) -> super::vals::Fltconf {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Fltconf::from_bits(val as u8)
    }
    #[doc = "Fault Confinement State"]
    #[inline(always)]
    pub const fn set_fltconf(&mut self, val: super::vals::Fltconf) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "FlexCAN In Transmission"]
    #[must_use]
    #[inline(always)]
    pub const fn tx(&self) -> super::vals::Tx {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Tx::from_bits(val as u8)
    }
    #[doc = "FlexCAN In Transmission"]
    #[inline(always)]
    pub const fn set_tx(&mut self, val: super::vals::Tx) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Idle"]
    #[must_use]
    #[inline(always)]
    pub const fn idle(&self) -> super::vals::Idle {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Idle::from_bits(val as u8)
    }
    #[doc = "Idle"]
    #[inline(always)]
    pub const fn set_idle(&mut self, val: super::vals::Idle) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "RX Error Warning Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rxwrn(&self) -> super::vals::Rxwrn {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Rxwrn::from_bits(val as u8)
    }
    #[doc = "RX Error Warning Flag"]
    #[inline(always)]
    pub const fn set_rxwrn(&mut self, val: super::vals::Rxwrn) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "TX Error Warning Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn txwrn(&self) -> super::vals::Txwrn {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Txwrn::from_bits(val as u8)
    }
    #[doc = "TX Error Warning Flag"]
    #[inline(always)]
    pub const fn set_txwrn(&mut self, val: super::vals::Txwrn) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Stuffing Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn stferr(&self) -> super::vals::Stferr {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Stferr::from_bits(val as u8)
    }
    #[doc = "Stuffing Error Flag"]
    #[inline(always)]
    pub const fn set_stferr(&mut self, val: super::vals::Stferr) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Form Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn frmerr(&self) -> super::vals::Frmerr {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Frmerr::from_bits(val as u8)
    }
    #[doc = "Form Error Flag"]
    #[inline(always)]
    pub const fn set_frmerr(&mut self, val: super::vals::Frmerr) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Cyclic Redundancy Check Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn crcerr(&self) -> super::vals::Crcerr {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Crcerr::from_bits(val as u8)
    }
    #[doc = "Cyclic Redundancy Check Error Flag"]
    #[inline(always)]
    pub const fn set_crcerr(&mut self, val: super::vals::Crcerr) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Acknowledge Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ackerr(&self) -> super::vals::Ackerr {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Ackerr::from_bits(val as u8)
    }
    #[doc = "Acknowledge Error Flag"]
    #[inline(always)]
    pub const fn set_ackerr(&mut self, val: super::vals::Ackerr) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Bit0 Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn bit0err(&self) -> super::vals::Bit0err {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Bit0err::from_bits(val as u8)
    }
    #[doc = "Bit0 Error Flag"]
    #[inline(always)]
    pub const fn set_bit0err(&mut self, val: super::vals::Bit0err) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Bit1 Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn bit1err(&self) -> super::vals::Bit1err {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Bit1err::from_bits(val as u8)
    }
    #[doc = "Bit1 Error Flag"]
    #[inline(always)]
    pub const fn set_bit1err(&mut self, val: super::vals::Bit1err) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "RX Warning Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rwrnint(&self) -> super::vals::Rwrnint {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Rwrnint::from_bits(val as u8)
    }
    #[doc = "RX Warning Interrupt Flag"]
    #[inline(always)]
    pub const fn set_rwrnint(&mut self, val: super::vals::Rwrnint) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "TX Warning Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn twrnint(&self) -> super::vals::Twrnint {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Twrnint::from_bits(val as u8)
    }
    #[doc = "TX Warning Interrupt Flag"]
    #[inline(always)]
    pub const fn set_twrnint(&mut self, val: super::vals::Twrnint) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "CAN Synchronization Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn synch(&self) -> super::vals::Synch {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Synch::from_bits(val as u8)
    }
    #[doc = "CAN Synchronization Status Flag"]
    #[inline(always)]
    pub const fn set_synch(&mut self, val: super::vals::Synch) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Bus Off Done Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn boffdoneint(&self) -> super::vals::Boffdoneint {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Boffdoneint::from_bits(val as u8)
    }
    #[doc = "Bus Off Done Interrupt Flag"]
    #[inline(always)]
    pub const fn set_boffdoneint(&mut self, val: super::vals::Boffdoneint) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Fast Error Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn errint_fast(&self) -> super::vals::ErrintFast {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::ErrintFast::from_bits(val as u8)
    }
    #[doc = "Fast Error Interrupt Flag"]
    #[inline(always)]
    pub const fn set_errint_fast(&mut self, val: super::vals::ErrintFast) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Error Overrun Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn errovr(&self) -> super::vals::Errovr {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Errovr::from_bits(val as u8)
    }
    #[doc = "Error Overrun Flag"]
    #[inline(always)]
    pub const fn set_errovr(&mut self, val: super::vals::Errovr) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Fast Stuffing Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn stferr_fast(&self) -> super::vals::StferrFast {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::StferrFast::from_bits(val as u8)
    }
    #[doc = "Fast Stuffing Error Flag"]
    #[inline(always)]
    pub const fn set_stferr_fast(&mut self, val: super::vals::StferrFast) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Fast Form Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn frmerr_fast(&self) -> super::vals::FrmerrFast {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::FrmerrFast::from_bits(val as u8)
    }
    #[doc = "Fast Form Error Flag"]
    #[inline(always)]
    pub const fn set_frmerr_fast(&mut self, val: super::vals::FrmerrFast) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Fast Cyclic Redundancy Check Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn crcerr_fast(&self) -> super::vals::CrcerrFast {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::CrcerrFast::from_bits(val as u8)
    }
    #[doc = "Fast Cyclic Redundancy Check Error Flag"]
    #[inline(always)]
    pub const fn set_crcerr_fast(&mut self, val: super::vals::CrcerrFast) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Fast Bit0 Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn bit0err_fast(&self) -> super::vals::Bit0errFast {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Bit0errFast::from_bits(val as u8)
    }
    #[doc = "Fast Bit0 Error Flag"]
    #[inline(always)]
    pub const fn set_bit0err_fast(&mut self, val: super::vals::Bit0errFast) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Fast Bit1 Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn bit1err_fast(&self) -> super::vals::Bit1errFast {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Bit1errFast::from_bits(val as u8)
    }
    #[doc = "Fast Bit1 Error Flag"]
    #[inline(always)]
    pub const fn set_bit1err_fast(&mut self, val: super::vals::Bit1errFast) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            .field("boffdoneint", &self.boffdoneint())
            .field("errint_fast", &self.errint_fast())
            .field("errovr", &self.errovr())
            .field("stferr_fast", &self.stferr_fast())
            .field("frmerr_fast", &self.frmerr_fast())
            .field("crcerr_fast", &self.crcerr_fast())
            .field("bit0err_fast", &self.bit0err_fast())
            .field("bit1err_fast", &self.bit1err_fast())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Esr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Esr1 {{ wakint: {=bool:?}, errint: {=bool:?}, boffint: {=bool:?}, rx: {=bool:?}, fltconf: {:?}, tx: {:?}, idle: {:?}, rxwrn: {:?}, txwrn: {:?}, stferr: {:?}, frmerr: {:?}, crcerr: {:?}, ackerr: {:?}, bit0err: {:?}, bit1err: {:?}, rwrnint: {:?}, twrnint: {:?}, synch: {:?}, boffdoneint: {:?}, errint_fast: {:?}, errovr: {:?}, stferr_fast: {:?}, frmerr_fast: {:?}, crcerr_fast: {:?}, bit0err_fast: {:?}, bit1err_fast: {:?} }}",
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
            self.synch(),
            self.boffdoneint(),
            self.errint_fast(),
            self.errovr(),
            self.stferr_fast(),
            self.frmerr_fast(),
            self.crcerr_fast(),
            self.bit0err_fast(),
            self.bit1err_fast()
        )
    }
}
#[doc = "Error and Status 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Esr2(pub u32);
impl Esr2 {
    #[doc = "Inactive Message Buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn imb(&self) -> super::vals::Imb {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Imb::from_bits(val as u8)
    }
    #[doc = "Inactive Message Buffer"]
    #[inline(always)]
    pub const fn set_imb(&mut self, val: super::vals::Imb) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Valid Priority Status"]
    #[must_use]
    #[inline(always)]
    pub const fn vps(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Valid Priority Status"]
    #[inline(always)]
    pub const fn set_vps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Lowest Priority TX Message Buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn lptm(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "Lowest Priority TX Message Buffer"]
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
            "Esr2 {{ imb: {:?}, vps: {=bool:?}, lptm: {=u8:?} }}",
            self.imb(),
            self.vps(),
            self.lptm()
        )
    }
}
#[doc = "Enhanced Transceiver Delay Compensation"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etdc(pub u32);
impl Etdc {
    #[doc = "Enhanced Transceiver Delay Compensation Value"]
    #[must_use]
    #[inline(always)]
    pub const fn etdcval(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Enhanced Transceiver Delay Compensation Value"]
    #[inline(always)]
    pub const fn set_etdcval(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Transceiver Delay Compensation Fail"]
    #[must_use]
    #[inline(always)]
    pub const fn etdcfail(&self) -> super::vals::Etdcfail {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Etdcfail::from_bits(val as u8)
    }
    #[doc = "Transceiver Delay Compensation Fail"]
    #[inline(always)]
    pub const fn set_etdcfail(&mut self, val: super::vals::Etdcfail) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Enhanced Transceiver Delay Compensation Offset"]
    #[must_use]
    #[inline(always)]
    pub const fn etdcoff(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "Enhanced Transceiver Delay Compensation Offset"]
    #[inline(always)]
    pub const fn set_etdcoff(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[doc = "Transceiver Delay Measurement Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn tdmdis(&self) -> super::vals::Tdmdis {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Tdmdis::from_bits(val as u8)
    }
    #[doc = "Transceiver Delay Measurement Disable"]
    #[inline(always)]
    pub const fn set_tdmdis(&mut self, val: super::vals::Tdmdis) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Transceiver Delay Compensation Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn etdcen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Transceiver Delay Compensation Enable"]
    #[inline(always)]
    pub const fn set_etdcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Etdc {
    #[inline(always)]
    fn default() -> Etdc {
        Etdc(0)
    }
}
impl core::fmt::Debug for Etdc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Etdc")
            .field("etdcval", &self.etdcval())
            .field("etdcfail", &self.etdcfail())
            .field("etdcoff", &self.etdcoff())
            .field("tdmdis", &self.tdmdis())
            .field("etdcen", &self.etdcen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etdc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Etdc {{ etdcval: {=u8:?}, etdcfail: {:?}, etdcoff: {=u8:?}, tdmdis: {:?}, etdcen: {=bool:?} }}",
            self.etdcval(),
            self.etdcfail(),
            self.etdcoff(),
            self.tdmdis(),
            self.etdcen()
        )
    }
}
#[doc = "CAN FD Bit Timing"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fdcbt(pub u32);
impl Fdcbt {
    #[doc = "Fast Phase Segment 2"]
    #[must_use]
    #[inline(always)]
    pub const fn fpseg2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Fast Phase Segment 2"]
    #[inline(always)]
    pub const fn set_fpseg2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Fast Phase Segment 1"]
    #[must_use]
    #[inline(always)]
    pub const fn fpseg1(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "Fast Phase Segment 1"]
    #[inline(always)]
    pub const fn set_fpseg1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[doc = "Fast Propagation Segment"]
    #[must_use]
    #[inline(always)]
    pub const fn fpropseg(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x1f;
        val as u8
    }
    #[doc = "Fast Propagation Segment"]
    #[inline(always)]
    pub const fn set_fpropseg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
    }
    #[doc = "Fast Resync Jump Width"]
    #[must_use]
    #[inline(always)]
    pub const fn frjw(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Fast Resync Jump Width"]
    #[inline(always)]
    pub const fn set_frjw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Fast Prescaler Division Factor"]
    #[must_use]
    #[inline(always)]
    pub const fn fpresdiv(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x03ff;
        val as u16
    }
    #[doc = "Fast Prescaler Division Factor"]
    #[inline(always)]
    pub const fn set_fpresdiv(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 20usize)) | (((val as u32) & 0x03ff) << 20usize);
    }
}
impl Default for Fdcbt {
    #[inline(always)]
    fn default() -> Fdcbt {
        Fdcbt(0)
    }
}
impl core::fmt::Debug for Fdcbt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fdcbt")
            .field("fpseg2", &self.fpseg2())
            .field("fpseg1", &self.fpseg1())
            .field("fpropseg", &self.fpropseg())
            .field("frjw", &self.frjw())
            .field("fpresdiv", &self.fpresdiv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fdcbt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fdcbt {{ fpseg2: {=u8:?}, fpseg1: {=u8:?}, fpropseg: {=u8:?}, frjw: {=u8:?}, fpresdiv: {=u16:?} }}",
            self.fpseg2(),
            self.fpseg1(),
            self.fpropseg(),
            self.frjw(),
            self.fpresdiv()
        )
    }
}
#[doc = "CAN FD CRC"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fdcrc(pub u32);
impl Fdcrc {
    #[doc = "Extended Transmitted CRC value"]
    #[must_use]
    #[inline(always)]
    pub const fn fd_txcrc(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x001f_ffff;
        val as u32
    }
    #[doc = "Extended Transmitted CRC value"]
    #[inline(always)]
    pub const fn set_fd_txcrc(&mut self, val: u32) {
        self.0 = (self.0 & !(0x001f_ffff << 0usize)) | (((val as u32) & 0x001f_ffff) << 0usize);
    }
    #[doc = "CRC Message Buffer Number for FD_TXCRC"]
    #[must_use]
    #[inline(always)]
    pub const fn fd_mbcrc(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[doc = "CRC Message Buffer Number for FD_TXCRC"]
    #[inline(always)]
    pub const fn set_fd_mbcrc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
}
impl Default for Fdcrc {
    #[inline(always)]
    fn default() -> Fdcrc {
        Fdcrc(0)
    }
}
impl core::fmt::Debug for Fdcrc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fdcrc")
            .field("fd_txcrc", &self.fd_txcrc())
            .field("fd_mbcrc", &self.fd_mbcrc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fdcrc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fdcrc {{ fd_txcrc: {=u32:?}, fd_mbcrc: {=u8:?} }}",
            self.fd_txcrc(),
            self.fd_mbcrc()
        )
    }
}
#[doc = "CAN FD Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fdctrl(pub u32);
impl Fdctrl {
    #[doc = "Transceiver Delay Compensation Value"]
    #[must_use]
    #[inline(always)]
    pub const fn tdcval(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Transceiver Delay Compensation Value"]
    #[inline(always)]
    pub const fn set_tdcval(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Transceiver Delay Compensation Offset"]
    #[must_use]
    #[inline(always)]
    pub const fn tdcoff(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Transceiver Delay Compensation Offset"]
    #[inline(always)]
    pub const fn set_tdcoff(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Transceiver Delay Compensation Fail"]
    #[must_use]
    #[inline(always)]
    pub const fn tdcfail(&self) -> super::vals::Tdcfail {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Tdcfail::from_bits(val as u8)
    }
    #[doc = "Transceiver Delay Compensation Fail"]
    #[inline(always)]
    pub const fn set_tdcfail(&mut self, val: super::vals::Tdcfail) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Transceiver Delay Compensation Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tdcen(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Transceiver Delay Compensation Enable"]
    #[inline(always)]
    pub const fn set_tdcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Message Buffer Data Size for Region 0"]
    #[must_use]
    #[inline(always)]
    pub const fn mbdsr0(&self) -> super::vals::Mbdsr0 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Mbdsr0::from_bits(val as u8)
    }
    #[doc = "Message Buffer Data Size for Region 0"]
    #[inline(always)]
    pub const fn set_mbdsr0(&mut self, val: super::vals::Mbdsr0) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Bit Rate Switch Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fdrate(&self) -> super::vals::Fdrate {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Fdrate::from_bits(val as u8)
    }
    #[doc = "Bit Rate Switch Enable"]
    #[inline(always)]
    pub const fn set_fdrate(&mut self, val: super::vals::Fdrate) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Fdctrl {
    #[inline(always)]
    fn default() -> Fdctrl {
        Fdctrl(0)
    }
}
impl core::fmt::Debug for Fdctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fdctrl")
            .field("tdcval", &self.tdcval())
            .field("tdcoff", &self.tdcoff())
            .field("tdcfail", &self.tdcfail())
            .field("tdcen", &self.tdcen())
            .field("mbdsr0", &self.mbdsr0())
            .field("fdrate", &self.fdrate())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fdctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fdctrl {{ tdcval: {=u8:?}, tdcoff: {=u8:?}, tdcfail: {:?}, tdcen: {=bool:?}, mbdsr0: {:?}, fdrate: {:?} }}",
            self.tdcval(),
            self.tdcoff(),
            self.tdcfail(),
            self.tdcen(),
            self.mbdsr0(),
            self.fdrate()
        )
    }
}
#[doc = "Pretended Networking Data Length Code (DLC) Filter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FltDlc(pub u32);
impl FltDlc {
    #[doc = "Upper Limit for Length of Data Bytes Filter"]
    #[must_use]
    #[inline(always)]
    pub const fn flt_dlc_hi(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Upper Limit for Length of Data Bytes Filter"]
    #[inline(always)]
    pub const fn set_flt_dlc_hi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Lower Limit for Length of Data Bytes Filter"]
    #[must_use]
    #[inline(always)]
    pub const fn flt_dlc_lo(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Lower Limit for Length of Data Bytes Filter"]
    #[inline(always)]
    pub const fn set_flt_dlc_lo(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for FltDlc {
    #[inline(always)]
    fn default() -> FltDlc {
        FltDlc(0)
    }
}
impl core::fmt::Debug for FltDlc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FltDlc")
            .field("flt_dlc_hi", &self.flt_dlc_hi())
            .field("flt_dlc_lo", &self.flt_dlc_lo())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FltDlc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FltDlc {{ flt_dlc_hi: {=u8:?}, flt_dlc_lo: {=u8:?} }}",
            self.flt_dlc_hi(),
            self.flt_dlc_lo()
        )
    }
}
#[doc = "Pretended Networking ID Filter 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FltId1(pub u32);
impl FltId1 {
    #[doc = "ID Filter 1 for Pretended Networking filtering"]
    #[must_use]
    #[inline(always)]
    pub const fn flt_id1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "ID Filter 1 for Pretended Networking filtering"]
    #[inline(always)]
    pub const fn set_flt_id1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 0usize)) | (((val as u32) & 0x1fff_ffff) << 0usize);
    }
    #[doc = "Remote Transmission Request Filter"]
    #[must_use]
    #[inline(always)]
    pub const fn flt_rtr(&self) -> super::vals::FltRtr {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::FltRtr::from_bits(val as u8)
    }
    #[doc = "Remote Transmission Request Filter"]
    #[inline(always)]
    pub const fn set_flt_rtr(&mut self, val: super::vals::FltRtr) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "ID Extended Filter"]
    #[must_use]
    #[inline(always)]
    pub const fn flt_ide(&self) -> super::vals::FltIde {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::FltIde::from_bits(val as u8)
    }
    #[doc = "ID Extended Filter"]
    #[inline(always)]
    pub const fn set_flt_ide(&mut self, val: super::vals::FltIde) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
}
impl Default for FltId1 {
    #[inline(always)]
    fn default() -> FltId1 {
        FltId1(0)
    }
}
impl core::fmt::Debug for FltId1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FltId1")
            .field("flt_id1", &self.flt_id1())
            .field("flt_rtr", &self.flt_rtr())
            .field("flt_ide", &self.flt_ide())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FltId1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FltId1 {{ flt_id1: {=u32:?}, flt_rtr: {:?}, flt_ide: {:?} }}",
            self.flt_id1(),
            self.flt_rtr(),
            self.flt_ide()
        )
    }
}
#[doc = "Pretended Networking ID Filter 2 or ID Mask"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FltId2Idmask(pub u32);
impl FltId2Idmask {
    #[doc = "ID Filter 2 for Pretended Networking Filtering or ID Mask Bits for Pretended Networking ID Filtering"]
    #[must_use]
    #[inline(always)]
    pub const fn flt_id2_idmask(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "ID Filter 2 for Pretended Networking Filtering or ID Mask Bits for Pretended Networking ID Filtering"]
    #[inline(always)]
    pub const fn set_flt_id2_idmask(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 0usize)) | (((val as u32) & 0x1fff_ffff) << 0usize);
    }
    #[doc = "Remote Transmission Request Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn rtr_msk(&self) -> super::vals::RtrMsk {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::RtrMsk::from_bits(val as u8)
    }
    #[doc = "Remote Transmission Request Mask"]
    #[inline(always)]
    pub const fn set_rtr_msk(&mut self, val: super::vals::RtrMsk) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "ID Extended Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn ide_msk(&self) -> super::vals::IdeMsk {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::IdeMsk::from_bits(val as u8)
    }
    #[doc = "ID Extended Mask"]
    #[inline(always)]
    pub const fn set_ide_msk(&mut self, val: super::vals::IdeMsk) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
}
impl Default for FltId2Idmask {
    #[inline(always)]
    fn default() -> FltId2Idmask {
        FltId2Idmask(0)
    }
}
impl core::fmt::Debug for FltId2Idmask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FltId2Idmask")
            .field("flt_id2_idmask", &self.flt_id2_idmask())
            .field("rtr_msk", &self.rtr_msk())
            .field("ide_msk", &self.ide_msk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FltId2Idmask {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FltId2Idmask {{ flt_id2_idmask: {=u32:?}, rtr_msk: {:?}, ide_msk: {:?} }}",
            self.flt_id2_idmask(),
            self.rtr_msk(),
            self.ide_msk()
        )
    }
}
#[doc = "Message Buffer 0 ID Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id(pub u32);
impl Id {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn ext(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    #[inline(always)]
    pub const fn set_ext(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn std(&self) -> u16 {
        let val = (self.0 >> 18usize) & 0x07ff;
        val as u16
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    #[inline(always)]
    pub const fn set_std(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 18usize)) | (((val as u32) & 0x07ff) << 18usize);
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    #[must_use]
    #[inline(always)]
    pub const fn prio(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    #[inline(always)]
    pub const fn set_prio(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
    }
}
impl Default for Id {
    #[inline(always)]
    fn default() -> Id {
        Id(0)
    }
}
impl core::fmt::Debug for Id {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Id")
            .field("ext", &self.ext())
            .field("std", &self.std())
            .field("prio", &self.prio())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Id {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Id {{ ext: {=u32:?}, std: {=u16:?}, prio: {=u8:?} }}",
            self.ext(),
            self.std(),
            self.prio()
        )
    }
}
#[doc = "Interrupt Flags 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iflag1(pub u32);
impl Iflag1 {
    #[doc = "Buffer MB0 Interrupt or Clear Legacy FIFO bit"]
    #[must_use]
    #[inline(always)]
    pub const fn buf0i(&self) -> super::vals::Buf0i {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Buf0i::from_bits(val as u8)
    }
    #[doc = "Buffer MB0 Interrupt or Clear Legacy FIFO bit"]
    #[inline(always)]
    pub const fn set_buf0i(&mut self, val: super::vals::Buf0i) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Buffer MBi Interrupt or Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn buf4to1i(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x0f;
        val as u8
    }
    #[doc = "Buffer MBi Interrupt or Reserved"]
    #[inline(always)]
    pub const fn set_buf4to1i(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 1usize)) | (((val as u32) & 0x0f) << 1usize);
    }
    #[doc = "Buffer MB5 Interrupt or Frames available in Legacy RX FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn buf5i(&self) -> super::vals::Buf5i {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Buf5i::from_bits(val as u8)
    }
    #[doc = "Buffer MB5 Interrupt or Frames available in Legacy RX FIFO"]
    #[inline(always)]
    pub const fn set_buf5i(&mut self, val: super::vals::Buf5i) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Buffer MB6 Interrupt or Legacy RX FIFO Warning"]
    #[must_use]
    #[inline(always)]
    pub const fn buf6i(&self) -> super::vals::Buf6i {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Buf6i::from_bits(val as u8)
    }
    #[doc = "Buffer MB6 Interrupt or Legacy RX FIFO Warning"]
    #[inline(always)]
    pub const fn set_buf6i(&mut self, val: super::vals::Buf6i) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Buffer MB7 Interrupt or Legacy RX FIFO Overflow"]
    #[must_use]
    #[inline(always)]
    pub const fn buf7i(&self) -> super::vals::Buf7i {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Buf7i::from_bits(val as u8)
    }
    #[doc = "Buffer MB7 Interrupt or Legacy RX FIFO Overflow"]
    #[inline(always)]
    pub const fn set_buf7i(&mut self, val: super::vals::Buf7i) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Buffer MBi Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn buf31to8i(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Buffer MBi Interrupt"]
    #[inline(always)]
    pub const fn set_buf31to8i(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
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
            .field("buf0i", &self.buf0i())
            .field("buf4to1i", &self.buf4to1i())
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
            "Iflag1 {{ buf0i: {:?}, buf4to1i: {=u8:?}, buf5i: {:?}, buf6i: {:?}, buf7i: {:?}, buf31to8i: {=u32:?} }}",
            self.buf0i(),
            self.buf4to1i(),
            self.buf5i(),
            self.buf6i(),
            self.buf7i(),
            self.buf31to8i()
        )
    }
}
#[doc = "Interrupt Masks 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Imask1(pub u32);
impl Imask1 {
    #[doc = "Buffer MBi Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn buf31to0m(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Buffer MBi Mask"]
    #[inline(always)]
    pub const fn set_buf31to0m(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
            .field("buf31to0m", &self.buf31to0m())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Imask1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Imask1 {{ buf31to0m: {=u32:?} }}", self.buf31to0m())
    }
}
#[doc = "Message Buffer 0 CS Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MbbCs(pub u32);
impl MbbCs {
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    #[must_use]
    #[inline(always)]
    pub const fn time_stamp(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    #[inline(always)]
    pub const fn set_time_stamp(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Length of the data to be stored/transmitted."]
    #[must_use]
    #[inline(always)]
    pub const fn dlc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Length of the data to be stored/transmitted."]
    #[inline(always)]
    pub const fn set_dlc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    #[must_use]
    #[inline(always)]
    pub const fn rtr(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Remote Transmission Request. One/zero for remote/data frame."]
    #[inline(always)]
    pub const fn set_rtr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    #[must_use]
    #[inline(always)]
    pub const fn ide(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "ID Extended. One/zero for extended/standard format frame."]
    #[inline(always)]
    pub const fn set_ide(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    #[must_use]
    #[inline(always)]
    pub const fn srr(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Substitute Remote Request. Contains a fixed recessive bit."]
    #[inline(always)]
    pub const fn set_srr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    #[must_use]
    #[inline(always)]
    pub const fn code(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Message Buffer Code. This 4-bit field can be accessed (read or write) by the CPU and by the FlexCAN module itself, as part of the message buffer matching and arbitration process."]
    #[inline(always)]
    pub const fn set_code(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    #[must_use]
    #[inline(always)]
    pub const fn esi(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Error State Indicator. This bit indicates if the transmitting node is error active or error passive."]
    #[inline(always)]
    pub const fn set_esi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    #[must_use]
    #[inline(always)]
    pub const fn brs(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Rate Switch. This bit defines whether the bit rate is switched inside a CAN FD format frame."]
    #[inline(always)]
    pub const fn set_brs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    #[must_use]
    #[inline(always)]
    pub const fn edl(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Extended Data Length. This bit distinguishes between CAN format and CAN FD format frames. The EDL bit must not be set for Message Buffers configured to RANSWER with code field 0b1010."]
    #[inline(always)]
    pub const fn set_edl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for MbbCs {
    #[inline(always)]
    fn default() -> MbbCs {
        MbbCs(0)
    }
}
impl core::fmt::Debug for MbbCs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MbbCs")
            .field("time_stamp", &self.time_stamp())
            .field("dlc", &self.dlc())
            .field("rtr", &self.rtr())
            .field("ide", &self.ide())
            .field("srr", &self.srr())
            .field("code", &self.code())
            .field("esi", &self.esi())
            .field("brs", &self.brs())
            .field("edl", &self.edl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MbbCs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MbbCs {{ time_stamp: {=u16:?}, dlc: {=u8:?}, rtr: {=bool:?}, ide: {=bool:?}, srr: {=bool:?}, code: {=u8:?}, esi: {=bool:?}, brs: {=bool:?}, edl: {=bool:?} }}",
            self.time_stamp(),
            self.dlc(),
            self.rtr(),
            self.ide(),
            self.srr(),
            self.code(),
            self.esi(),
            self.brs(),
            self.edl()
        )
    }
}
#[doc = "Message Buffer 0 ID Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MbbId(pub u32);
impl MbbId {
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn ext(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "Contains extended (LOW word) identifier of message buffer."]
    #[inline(always)]
    pub const fn set_ext(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn std(&self) -> u16 {
        let val = (self.0 >> 18usize) & 0x07ff;
        val as u16
    }
    #[doc = "Contains standard/extended (HIGH word) identifier of message buffer."]
    #[inline(always)]
    pub const fn set_std(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 18usize)) | (((val as u32) & 0x07ff) << 18usize);
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    #[must_use]
    #[inline(always)]
    pub const fn prio(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[doc = "Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    #[inline(always)]
    pub const fn set_prio(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
    }
}
impl Default for MbbId {
    #[inline(always)]
    fn default() -> MbbId {
        MbbId(0)
    }
}
impl core::fmt::Debug for MbbId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MbbId")
            .field("ext", &self.ext())
            .field("std", &self.std())
            .field("prio", &self.prio())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MbbId {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MbbId {{ ext: {=u32:?}, std: {=u16:?}, prio: {=u8:?} }}",
            self.ext(),
            self.std(),
            self.prio()
        )
    }
}
#[doc = "Module Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcr(pub u32);
impl Mcr {
    #[doc = "Number of the Last Message Buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn maxmb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Number of the Last Message Buffer"]
    #[inline(always)]
    pub const fn set_maxmb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "ID Acceptance Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn idam(&self) -> super::vals::Idam {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Idam::from_bits(val as u8)
    }
    #[doc = "ID Acceptance Mode"]
    #[inline(always)]
    pub const fn set_idam(&mut self, val: super::vals::Idam) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "CAN FD Operation Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fden(&self) -> super::vals::Fden {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Fden::from_bits(val as u8)
    }
    #[doc = "CAN FD Operation Enable"]
    #[inline(always)]
    pub const fn set_fden(&mut self, val: super::vals::Fden) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Abort Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn aen(&self) -> super::vals::Aen {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Aen::from_bits(val as u8)
    }
    #[doc = "Abort Enable"]
    #[inline(always)]
    pub const fn set_aen(&mut self, val: super::vals::Aen) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Local Priority Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lprioen(&self) -> super::vals::Lprioen {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Lprioen::from_bits(val as u8)
    }
    #[doc = "Local Priority Enable"]
    #[inline(always)]
    pub const fn set_lprioen(&mut self, val: super::vals::Lprioen) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Pretended Networking Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pnet_en(&self) -> super::vals::PnetEn {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::PnetEn::from_bits(val as u8)
    }
    #[doc = "Pretended Networking Enable"]
    #[inline(always)]
    pub const fn set_pnet_en(&mut self, val: super::vals::PnetEn) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dma(&self) -> super::vals::Dma {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Dma::from_bits(val as u8)
    }
    #[doc = "DMA Enable"]
    #[inline(always)]
    pub const fn set_dma(&mut self, val: super::vals::Dma) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Individual RX Masking and Queue Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn irmq(&self) -> super::vals::Irmq {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Irmq::from_bits(val as u8)
    }
    #[doc = "Individual RX Masking and Queue Enable"]
    #[inline(always)]
    pub const fn set_irmq(&mut self, val: super::vals::Irmq) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Self-Reception Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn srxdis(&self) -> super::vals::Srxdis {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Srxdis::from_bits(val as u8)
    }
    #[doc = "Self-Reception Disable"]
    #[inline(always)]
    pub const fn set_srxdis(&mut self, val: super::vals::Srxdis) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Doze Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn doze(&self) -> super::vals::Doze {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Doze::from_bits(val as u8)
    }
    #[doc = "Doze Mode Enable"]
    #[inline(always)]
    pub const fn set_doze(&mut self, val: super::vals::Doze) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Wake-Up Source"]
    #[must_use]
    #[inline(always)]
    pub const fn waksrc(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-Up Source"]
    #[inline(always)]
    pub const fn set_waksrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Low-Power Mode Acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn lpmack(&self) -> super::vals::Lpmack {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Lpmack::from_bits(val as u8)
    }
    #[doc = "Low-Power Mode Acknowledge"]
    #[inline(always)]
    pub const fn set_lpmack(&mut self, val: super::vals::Lpmack) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Warning Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wrnen(&self) -> super::vals::Wrnen {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Wrnen::from_bits(val as u8)
    }
    #[doc = "Warning Interrupt Enable"]
    #[inline(always)]
    pub const fn set_wrnen(&mut self, val: super::vals::Wrnen) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Self Wake-up"]
    #[must_use]
    #[inline(always)]
    pub const fn slfwak(&self) -> super::vals::Slfwak {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Slfwak::from_bits(val as u8)
    }
    #[doc = "Self Wake-up"]
    #[inline(always)]
    pub const fn set_slfwak(&mut self, val: super::vals::Slfwak) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Supervisor Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn supv(&self) -> super::vals::Supv {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Supv::from_bits(val as u8)
    }
    #[doc = "Supervisor Mode"]
    #[inline(always)]
    pub const fn set_supv(&mut self, val: super::vals::Supv) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Freeze Mode Acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn frzack(&self) -> super::vals::Frzack {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Frzack::from_bits(val as u8)
    }
    #[doc = "Freeze Mode Acknowledge"]
    #[inline(always)]
    pub const fn set_frzack(&mut self, val: super::vals::Frzack) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Soft Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn softrst(&self) -> super::vals::Softrst {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Softrst::from_bits(val as u8)
    }
    #[doc = "Soft Reset"]
    #[inline(always)]
    pub const fn set_softrst(&mut self, val: super::vals::Softrst) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Wake-up Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn wakmsk(&self) -> super::vals::Wakmsk {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Wakmsk::from_bits(val as u8)
    }
    #[doc = "Wake-up Interrupt Mask"]
    #[inline(always)]
    pub const fn set_wakmsk(&mut self, val: super::vals::Wakmsk) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "FlexCAN Not Ready"]
    #[must_use]
    #[inline(always)]
    pub const fn notrdy(&self) -> super::vals::Notrdy {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Notrdy::from_bits(val as u8)
    }
    #[doc = "FlexCAN Not Ready"]
    #[inline(always)]
    pub const fn set_notrdy(&mut self, val: super::vals::Notrdy) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Halt FlexCAN"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::Halt {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Halt::from_bits(val as u8)
    }
    #[doc = "Halt FlexCAN"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::Halt) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Legacy RX FIFO Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rfen(&self) -> super::vals::Rfen {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Rfen::from_bits(val as u8)
    }
    #[doc = "Legacy RX FIFO Enable"]
    #[inline(always)]
    pub const fn set_rfen(&mut self, val: super::vals::Rfen) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Freeze Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn frz(&self) -> super::vals::Frz {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Frz::from_bits(val as u8)
    }
    #[doc = "Freeze Enable"]
    #[inline(always)]
    pub const fn set_frz(&mut self, val: super::vals::Frz) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Module Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn mdis(&self) -> super::vals::Mdis {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Mdis::from_bits(val as u8)
    }
    #[doc = "Module Disable"]
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
            .field("fden", &self.fden())
            .field("aen", &self.aen())
            .field("lprioen", &self.lprioen())
            .field("pnet_en", &self.pnet_en())
            .field("dma", &self.dma())
            .field("irmq", &self.irmq())
            .field("srxdis", &self.srxdis())
            .field("doze", &self.doze())
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
            "Mcr {{ maxmb: {=u8:?}, idam: {:?}, fden: {:?}, aen: {:?}, lprioen: {:?}, pnet_en: {:?}, dma: {:?}, irmq: {:?}, srxdis: {:?}, doze: {:?}, waksrc: {=bool:?}, lpmack: {:?}, wrnen: {:?}, slfwak: {:?}, supv: {:?}, frzack: {:?}, softrst: {:?}, wakmsk: {:?}, notrdy: {:?}, halt: {:?}, rfen: {:?}, frz: {:?}, mdis: {:?} }}",
            self.maxmb(),
            self.idam(),
            self.fden(),
            self.aen(),
            self.lprioen(),
            self.pnet_en(),
            self.dma(),
            self.irmq(),
            self.srxdis(),
            self.doze(),
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
#[doc = "Pretended Networking Payload High Filter 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pl1Hi(pub u32);
impl Pl1Hi {
    #[doc = "Data byte 7"]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_7(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 7"]
    #[inline(always)]
    pub const fn set_data_byte_7(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data byte 6"]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_6(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 6"]
    #[inline(always)]
    pub const fn set_data_byte_6(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data byte 5"]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_5(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 5"]
    #[inline(always)]
    pub const fn set_data_byte_5(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data byte 4"]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_4(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 4"]
    #[inline(always)]
    pub const fn set_data_byte_4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Pl1Hi {
    #[inline(always)]
    fn default() -> Pl1Hi {
        Pl1Hi(0)
    }
}
impl core::fmt::Debug for Pl1Hi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pl1Hi")
            .field("data_byte_7", &self.data_byte_7())
            .field("data_byte_6", &self.data_byte_6())
            .field("data_byte_5", &self.data_byte_5())
            .field("data_byte_4", &self.data_byte_4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pl1Hi {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pl1Hi {{ data_byte_7: {=u8:?}, data_byte_6: {=u8:?}, data_byte_5: {=u8:?}, data_byte_4: {=u8:?} }}",
            self.data_byte_7(),
            self.data_byte_6(),
            self.data_byte_5(),
            self.data_byte_4()
        )
    }
}
#[doc = "Pretended Networking Payload Low Filter 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pl1Lo(pub u32);
impl Pl1Lo {
    #[doc = "Data byte 3"]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_3(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 3"]
    #[inline(always)]
    pub const fn set_data_byte_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data byte 2"]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 2"]
    #[inline(always)]
    pub const fn set_data_byte_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data byte 1"]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 1"]
    #[inline(always)]
    pub const fn set_data_byte_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data byte 0"]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_0(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 0"]
    #[inline(always)]
    pub const fn set_data_byte_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Pl1Lo {
    #[inline(always)]
    fn default() -> Pl1Lo {
        Pl1Lo(0)
    }
}
impl core::fmt::Debug for Pl1Lo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pl1Lo")
            .field("data_byte_3", &self.data_byte_3())
            .field("data_byte_2", &self.data_byte_2())
            .field("data_byte_1", &self.data_byte_1())
            .field("data_byte_0", &self.data_byte_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pl1Lo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pl1Lo {{ data_byte_3: {=u8:?}, data_byte_2: {=u8:?}, data_byte_1: {=u8:?}, data_byte_0: {=u8:?} }}",
            self.data_byte_3(),
            self.data_byte_2(),
            self.data_byte_1(),
            self.data_byte_0()
        )
    }
}
#[doc = "Pretended Networking Payload High Filter 2 and Payload High Mask"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pl2PlmaskHi(pub u32);
impl Pl2PlmaskHi {
    #[doc = "Data Byte 7"]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_7(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 7"]
    #[inline(always)]
    pub const fn set_data_byte_7(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data Byte 6"]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_6(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 6"]
    #[inline(always)]
    pub const fn set_data_byte_6(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data Byte 5"]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_5(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 5"]
    #[inline(always)]
    pub const fn set_data_byte_5(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data Byte 4"]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_4(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 4"]
    #[inline(always)]
    pub const fn set_data_byte_4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Pl2PlmaskHi {
    #[inline(always)]
    fn default() -> Pl2PlmaskHi {
        Pl2PlmaskHi(0)
    }
}
impl core::fmt::Debug for Pl2PlmaskHi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pl2PlmaskHi")
            .field("data_byte_7", &self.data_byte_7())
            .field("data_byte_6", &self.data_byte_6())
            .field("data_byte_5", &self.data_byte_5())
            .field("data_byte_4", &self.data_byte_4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pl2PlmaskHi {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pl2PlmaskHi {{ data_byte_7: {=u8:?}, data_byte_6: {=u8:?}, data_byte_5: {=u8:?}, data_byte_4: {=u8:?} }}",
            self.data_byte_7(),
            self.data_byte_6(),
            self.data_byte_5(),
            self.data_byte_4()
        )
    }
}
#[doc = "Pretended Networking Payload Low Filter 2 and Payload Low Mask"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pl2PlmaskLo(pub u32);
impl Pl2PlmaskLo {
    #[doc = "Data Byte 3"]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_3(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 3"]
    #[inline(always)]
    pub const fn set_data_byte_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data Byte 2"]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 2"]
    #[inline(always)]
    pub const fn set_data_byte_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data Byte 1"]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 1"]
    #[inline(always)]
    pub const fn set_data_byte_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data Byte 0"]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_0(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 0"]
    #[inline(always)]
    pub const fn set_data_byte_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Pl2PlmaskLo {
    #[inline(always)]
    fn default() -> Pl2PlmaskLo {
        Pl2PlmaskLo(0)
    }
}
impl core::fmt::Debug for Pl2PlmaskLo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pl2PlmaskLo")
            .field("data_byte_3", &self.data_byte_3())
            .field("data_byte_2", &self.data_byte_2())
            .field("data_byte_1", &self.data_byte_1())
            .field("data_byte_0", &self.data_byte_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pl2PlmaskLo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pl2PlmaskLo {{ data_byte_3: {=u8:?}, data_byte_2: {=u8:?}, data_byte_1: {=u8:?}, data_byte_0: {=u8:?} }}",
            self.data_byte_3(),
            self.data_byte_2(),
            self.data_byte_1(),
            self.data_byte_0()
        )
    }
}
#[doc = "Receive 14 Mask"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rx14mask(pub u32);
impl Rx14mask {
    #[doc = "RX Buffer 14 Mask Bits"]
    #[must_use]
    #[inline(always)]
    pub const fn rx14m(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "RX Buffer 14 Mask Bits"]
    #[inline(always)]
    pub const fn set_rx14m(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
        defmt::write!(f, "Rx14mask {{ rx14m: {=u32:?} }}", self.rx14m())
    }
}
#[doc = "Receive 15 Mask"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rx15mask(pub u32);
impl Rx15mask {
    #[doc = "RX Buffer 15 Mask Bits"]
    #[must_use]
    #[inline(always)]
    pub const fn rx15m(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "RX Buffer 15 Mask Bits"]
    #[inline(always)]
    pub const fn set_rx15m(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
        defmt::write!(f, "Rx15mask {{ rx15m: {=u32:?} }}", self.rx15m())
    }
}
#[doc = "Legacy RX FIFO Global Mask"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxfgmask(pub u32);
impl Rxfgmask {
    #[doc = "Legacy RX FIFO Global Mask Bits"]
    #[must_use]
    #[inline(always)]
    pub const fn fgm(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Legacy RX FIFO Global Mask Bits"]
    #[inline(always)]
    pub const fn set_fgm(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
        defmt::write!(f, "Rxfgmask {{ fgm: {=u32:?} }}", self.fgm())
    }
}
#[doc = "Legacy RX FIFO Information"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxfir(pub u32);
impl Rxfir {
    #[doc = "Identifier Acceptance Filter Hit Indicator"]
    #[must_use]
    #[inline(always)]
    pub const fn idhit(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Identifier Acceptance Filter Hit Indicator"]
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
#[doc = "Receive Individual Mask"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rximr(pub u32);
impl Rximr {
    #[doc = "Individual Mask Bits"]
    #[must_use]
    #[inline(always)]
    pub const fn mi(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Individual Mask Bits"]
    #[inline(always)]
    pub const fn set_mi(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
        defmt::write!(f, "Rximr {{ mi: {=u32:?} }}", self.mi())
    }
}
#[doc = "RX Message Buffers Global Mask"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxmgmask(pub u32);
impl Rxmgmask {
    #[doc = "Global Mask for RX Message Buffers"]
    #[must_use]
    #[inline(always)]
    pub const fn mg(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Global Mask for RX Message Buffers"]
    #[inline(always)]
    pub const fn set_mg(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
        defmt::write!(f, "Rxmgmask {{ mg: {=u32:?} }}", self.mg())
    }
}
#[doc = "Free-Running Timer"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer(pub u32);
impl Timer {
    #[doc = "Timer Value"]
    #[must_use]
    #[inline(always)]
    pub const fn timer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Timer Value"]
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
#[doc = "Wake-Up Message Buffer"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WmbCs(pub u32);
impl WmbCs {
    #[doc = "Length of Data in Bytes"]
    #[must_use]
    #[inline(always)]
    pub const fn dlc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Length of Data in Bytes"]
    #[inline(always)]
    pub const fn set_dlc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Remote Transmission Request"]
    #[must_use]
    #[inline(always)]
    pub const fn rtr(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Remote Transmission Request"]
    #[inline(always)]
    pub const fn set_rtr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "ID Extended Bit"]
    #[must_use]
    #[inline(always)]
    pub const fn ide(&self) -> super::vals::Ide {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Ide::from_bits(val as u8)
    }
    #[doc = "ID Extended Bit"]
    #[inline(always)]
    pub const fn set_ide(&mut self, val: super::vals::Ide) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Substitute Remote Request"]
    #[must_use]
    #[inline(always)]
    pub const fn srr(&self) -> super::vals::Srr {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Srr::from_bits(val as u8)
    }
    #[doc = "Substitute Remote Request"]
    #[inline(always)]
    pub const fn set_srr(&mut self, val: super::vals::Srr) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
}
impl Default for WmbCs {
    #[inline(always)]
    fn default() -> WmbCs {
        WmbCs(0)
    }
}
impl core::fmt::Debug for WmbCs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WmbCs")
            .field("dlc", &self.dlc())
            .field("rtr", &self.rtr())
            .field("ide", &self.ide())
            .field("srr", &self.srr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WmbCs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WmbCs {{ dlc: {=u8:?}, rtr: {=bool:?}, ide: {:?}, srr: {:?} }}",
            self.dlc(),
            self.rtr(),
            self.ide(),
            self.srr()
        )
    }
}
#[doc = "Wake-Up Message Buffer for Data 0-3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WmbD03(pub u32);
impl WmbD03 {
    #[doc = "Data Byte 3"]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_3(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 3"]
    #[inline(always)]
    pub const fn set_data_byte_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data Byte 2"]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 2"]
    #[inline(always)]
    pub const fn set_data_byte_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data Byte 1"]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 1"]
    #[inline(always)]
    pub const fn set_data_byte_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data Byte 0"]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_0(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 0"]
    #[inline(always)]
    pub const fn set_data_byte_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for WmbD03 {
    #[inline(always)]
    fn default() -> WmbD03 {
        WmbD03(0)
    }
}
impl core::fmt::Debug for WmbD03 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WmbD03")
            .field("data_byte_3", &self.data_byte_3())
            .field("data_byte_2", &self.data_byte_2())
            .field("data_byte_1", &self.data_byte_1())
            .field("data_byte_0", &self.data_byte_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WmbD03 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WmbD03 {{ data_byte_3: {=u8:?}, data_byte_2: {=u8:?}, data_byte_1: {=u8:?}, data_byte_0: {=u8:?} }}",
            self.data_byte_3(),
            self.data_byte_2(),
            self.data_byte_1(),
            self.data_byte_0()
        )
    }
}
#[doc = "Wake-Up Message Buffer Register Data 4-7"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WmbD47(pub u32);
impl WmbD47 {
    #[doc = "Data Byte 7"]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_7(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 7"]
    #[inline(always)]
    pub const fn set_data_byte_7(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data Byte 6"]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_6(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 6"]
    #[inline(always)]
    pub const fn set_data_byte_6(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data Byte 5"]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_5(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 5"]
    #[inline(always)]
    pub const fn set_data_byte_5(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data Byte 4"]
    #[must_use]
    #[inline(always)]
    pub const fn data_byte_4(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 4"]
    #[inline(always)]
    pub const fn set_data_byte_4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for WmbD47 {
    #[inline(always)]
    fn default() -> WmbD47 {
        WmbD47(0)
    }
}
impl core::fmt::Debug for WmbD47 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WmbD47")
            .field("data_byte_7", &self.data_byte_7())
            .field("data_byte_6", &self.data_byte_6())
            .field("data_byte_5", &self.data_byte_5())
            .field("data_byte_4", &self.data_byte_4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WmbD47 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WmbD47 {{ data_byte_7: {=u8:?}, data_byte_6: {=u8:?}, data_byte_5: {=u8:?}, data_byte_4: {=u8:?} }}",
            self.data_byte_7(),
            self.data_byte_6(),
            self.data_byte_5(),
            self.data_byte_4()
        )
    }
}
#[doc = "Wake-Up Message Buffer for ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WmbId(pub u32);
impl WmbId {
    #[doc = "Received ID in Pretended Networking Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn id(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "Received ID in Pretended Networking Mode"]
    #[inline(always)]
    pub const fn set_id(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 0usize)) | (((val as u32) & 0x1fff_ffff) << 0usize);
    }
}
impl Default for WmbId {
    #[inline(always)]
    fn default() -> WmbId {
        WmbId(0)
    }
}
impl core::fmt::Debug for WmbId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WmbId").field("id", &self.id()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WmbId {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "WmbId {{ id: {=u32:?} }}", self.id())
    }
}
#[doc = "Pretended Networking Wake-Up Match"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WuMtc(pub u32);
impl WuMtc {
    #[doc = "Number of Matches in Pretended Networking"]
    #[must_use]
    #[inline(always)]
    pub const fn mcounter(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Number of Matches in Pretended Networking"]
    #[inline(always)]
    pub const fn set_mcounter(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Wake-up by Match Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn wumf(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up by Match Flag"]
    #[inline(always)]
    pub const fn set_wumf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Wake-up by Timeout Flag Bit"]
    #[must_use]
    #[inline(always)]
    pub const fn wtof(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up by Timeout Flag Bit"]
    #[inline(always)]
    pub const fn set_wtof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for WuMtc {
    #[inline(always)]
    fn default() -> WuMtc {
        WuMtc(0)
    }
}
impl core::fmt::Debug for WuMtc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WuMtc")
            .field("mcounter", &self.mcounter())
            .field("wumf", &self.wumf())
            .field("wtof", &self.wtof())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WuMtc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WuMtc {{ mcounter: {=u8:?}, wumf: {=bool:?}, wtof: {=bool:?} }}",
            self.mcounter(),
            self.wumf(),
            self.wtof()
        )
    }
}
