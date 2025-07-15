#[doc = "Extended IBI Data 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ibiext1(pub u32);
impl Ibiext1 {
    #[doc = "Count"]
    #[must_use]
    #[inline(always)]
    pub const fn cnt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Count"]
    #[inline(always)]
    pub const fn set_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Maximum"]
    #[must_use]
    #[inline(always)]
    pub const fn max(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Maximum"]
    #[inline(always)]
    pub const fn set_max(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Extra Byte 1"]
    #[must_use]
    #[inline(always)]
    pub const fn ext1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Extra Byte 1"]
    #[inline(always)]
    pub const fn set_ext1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Extra Byte 2"]
    #[must_use]
    #[inline(always)]
    pub const fn ext2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Extra Byte 2"]
    #[inline(always)]
    pub const fn set_ext2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Extra Byte 3"]
    #[must_use]
    #[inline(always)]
    pub const fn ext3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Extra Byte 3"]
    #[inline(always)]
    pub const fn set_ext3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Ibiext1 {
    #[inline(always)]
    fn default() -> Ibiext1 {
        Ibiext1(0)
    }
}
impl core::fmt::Debug for Ibiext1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ibiext1")
            .field("cnt", &self.cnt())
            .field("max", &self.max())
            .field("ext1", &self.ext1())
            .field("ext2", &self.ext2())
            .field("ext3", &self.ext3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ibiext1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ibiext1 {{ cnt: {=u8:?}, max: {=u8:?}, ext1: {=u8:?}, ext2: {=u8:?}, ext3: {=u8:?} }}",
            self.cnt(),
            self.max(),
            self.ext1(),
            self.ext2(),
            self.ext3()
        )
    }
}
#[doc = "Extended IBI Data 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ibiext2(pub u32);
impl Ibiext2 {
    #[doc = "Extra Byte 4"]
    #[must_use]
    #[inline(always)]
    pub const fn ext4(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Extra Byte 4"]
    #[inline(always)]
    pub const fn set_ext4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Extra Byte 5"]
    #[must_use]
    #[inline(always)]
    pub const fn ext5(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Extra Byte 5"]
    #[inline(always)]
    pub const fn set_ext5(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Extra Byte 6"]
    #[must_use]
    #[inline(always)]
    pub const fn ext6(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Extra Byte 6"]
    #[inline(always)]
    pub const fn set_ext6(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Extra Byte 7"]
    #[must_use]
    #[inline(always)]
    pub const fn ext7(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Extra Byte 7"]
    #[inline(always)]
    pub const fn set_ext7(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Ibiext2 {
    #[inline(always)]
    fn default() -> Ibiext2 {
        Ibiext2(0)
    }
}
impl core::fmt::Debug for Ibiext2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ibiext2")
            .field("ext4", &self.ext4())
            .field("ext5", &self.ext5())
            .field("ext6", &self.ext6())
            .field("ext7", &self.ext7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ibiext2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ibiext2 {{ ext4: {=u8:?}, ext5: {=u8:?}, ext6: {=u8:?}, ext7: {=u8:?} }}",
            self.ext4(),
            self.ext5(),
            self.ext6(),
            self.ext7()
        )
    }
}
#[doc = "Controller Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mconfig(pub u32);
impl Mconfig {
    #[doc = "Controller Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mstena(&self) -> super::vals::Mstena {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Mstena::from_bits(val as u8)
    }
    #[doc = "Controller Enable"]
    #[inline(always)]
    pub const fn set_mstena(&mut self, val: super::vals::Mstena) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Disable Timeout"]
    #[must_use]
    #[inline(always)]
    pub const fn disto(&self) -> super::vals::Disto {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Disto::from_bits(val as u8)
    }
    #[doc = "Disable Timeout"]
    #[inline(always)]
    pub const fn set_disto(&mut self, val: super::vals::Disto) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "High-Keeper"]
    #[must_use]
    #[inline(always)]
    pub const fn hkeep(&self) -> super::vals::Hkeep {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Hkeep::from_bits(val as u8)
    }
    #[doc = "High-Keeper"]
    #[inline(always)]
    pub const fn set_hkeep(&mut self, val: super::vals::Hkeep) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Open Drain Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn odstop(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Open Drain Stop"]
    #[inline(always)]
    pub const fn set_odstop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Push-Pull Baud Rate"]
    #[must_use]
    #[inline(always)]
    pub const fn ppbaud(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Push-Pull Baud Rate"]
    #[inline(always)]
    pub const fn set_ppbaud(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Push-Pull Low"]
    #[must_use]
    #[inline(always)]
    pub const fn pplow(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Push-Pull Low"]
    #[inline(always)]
    pub const fn set_pplow(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Open Drain Baud Rate"]
    #[must_use]
    #[inline(always)]
    pub const fn odbaud(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Open Drain Baud Rate"]
    #[inline(always)]
    pub const fn set_odbaud(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Open Drain High Push-Pull"]
    #[must_use]
    #[inline(always)]
    pub const fn odhpp(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Open Drain High Push-Pull"]
    #[inline(always)]
    pub const fn set_odhpp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Skew"]
    #[must_use]
    #[inline(always)]
    pub const fn skew(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x07;
        val as u8
    }
    #[doc = "Skew"]
    #[inline(always)]
    pub const fn set_skew(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 25usize)) | (((val as u32) & 0x07) << 25usize);
    }
    #[doc = "I2C Baud Rate"]
    #[must_use]
    #[inline(always)]
    pub const fn i2cbaud(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "I2C Baud Rate"]
    #[inline(always)]
    pub const fn set_i2cbaud(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Mconfig {
    #[inline(always)]
    fn default() -> Mconfig {
        Mconfig(0)
    }
}
impl core::fmt::Debug for Mconfig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mconfig")
            .field("mstena", &self.mstena())
            .field("disto", &self.disto())
            .field("hkeep", &self.hkeep())
            .field("odstop", &self.odstop())
            .field("ppbaud", &self.ppbaud())
            .field("pplow", &self.pplow())
            .field("odbaud", &self.odbaud())
            .field("odhpp", &self.odhpp())
            .field("skew", &self.skew())
            .field("i2cbaud", &self.i2cbaud())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mconfig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mconfig {{ mstena: {:?}, disto: {:?}, hkeep: {:?}, odstop: {=bool:?}, ppbaud: {=u8:?}, pplow: {=u8:?}, odbaud: {=u8:?}, odhpp: {=bool:?}, skew: {=u8:?}, i2cbaud: {=u8:?} }}",
            self.mstena(),
            self.disto(),
            self.hkeep(),
            self.odstop(),
            self.ppbaud(),
            self.pplow(),
            self.odbaud(),
            self.odhpp(),
            self.skew(),
            self.i2cbaud()
        )
    }
}
#[doc = "Controller Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mctrl(pub u32);
impl Mctrl {
    #[doc = "Request"]
    #[must_use]
    #[inline(always)]
    pub const fn request(&self) -> super::vals::Request {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Request::from_bits(val as u8)
    }
    #[doc = "Request"]
    #[inline(always)]
    pub const fn set_request(&mut self, val: super::vals::Request) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Bus Type with EmitStartAddr"]
    #[must_use]
    #[inline(always)]
    pub const fn type_(&self) -> super::vals::Type {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Type::from_bits(val as u8)
    }
    #[doc = "Bus Type with EmitStartAddr"]
    #[inline(always)]
    pub const fn set_type_(&mut self, val: super::vals::Type) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "In-Band Interrupt Response"]
    #[must_use]
    #[inline(always)]
    pub const fn ibiresp(&self) -> super::vals::Ibiresp {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Ibiresp::from_bits(val as u8)
    }
    #[doc = "In-Band Interrupt Response"]
    #[inline(always)]
    pub const fn set_ibiresp(&mut self, val: super::vals::Ibiresp) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn dir(&self) -> super::vals::MctrlDir {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::MctrlDir::from_bits(val as u8)
    }
    #[doc = "Direction"]
    #[inline(always)]
    pub const fn set_dir(&mut self, val: super::vals::MctrlDir) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Address"]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x7f;
        val as u8
    }
    #[doc = "Address"]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u32) & 0x7f) << 9usize);
    }
    #[doc = "Read Terminate Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn rdterm(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Read Terminate Counter"]
    #[inline(always)]
    pub const fn set_rdterm(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Mctrl {
    #[inline(always)]
    fn default() -> Mctrl {
        Mctrl(0)
    }
}
impl core::fmt::Debug for Mctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mctrl")
            .field("request", &self.request())
            .field("type_", &self.type_())
            .field("ibiresp", &self.ibiresp())
            .field("dir", &self.dir())
            .field("addr", &self.addr())
            .field("rdterm", &self.rdterm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mctrl {{ request: {:?}, type_: {:?}, ibiresp: {:?}, dir: {:?}, addr: {=u8:?}, rdterm: {=u8:?} }}",
            self.request(),
            self.type_(),
            self.ibiresp(),
            self.dir(),
            self.addr(),
            self.rdterm()
        )
    }
}
#[doc = "Controller Data Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mdatactrl(pub u32);
impl Mdatactrl {
    #[doc = "Flush To-Bus Buffer or FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn flushtb(&self) -> super::vals::Flushtb {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flushtb::from_bits(val as u8)
    }
    #[doc = "Flush To-Bus Buffer or FIFO"]
    #[inline(always)]
    pub const fn set_flushtb(&mut self, val: super::vals::Flushtb) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Flush From-Bus Buffer or FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn flushfb(&self) -> super::vals::Flushfb {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Flushfb::from_bits(val as u8)
    }
    #[doc = "Flush From-Bus Buffer or FIFO"]
    #[inline(always)]
    pub const fn set_flushfb(&mut self, val: super::vals::Flushfb) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Unlock"]
    #[must_use]
    #[inline(always)]
    pub const fn unlock(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Unlock"]
    #[inline(always)]
    pub const fn set_unlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Transmit Trigger Level"]
    #[must_use]
    #[inline(always)]
    pub const fn txtrig(&self) -> super::vals::MdatactrlTxtrig {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::MdatactrlTxtrig::from_bits(val as u8)
    }
    #[doc = "Transmit Trigger Level"]
    #[inline(always)]
    pub const fn set_txtrig(&mut self, val: super::vals::MdatactrlTxtrig) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Receive Trigger Level"]
    #[must_use]
    #[inline(always)]
    pub const fn rxtrig(&self) -> super::vals::MdatactrlRxtrig {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::MdatactrlRxtrig::from_bits(val as u8)
    }
    #[doc = "Receive Trigger Level"]
    #[inline(always)]
    pub const fn set_rxtrig(&mut self, val: super::vals::MdatactrlRxtrig) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Transmit Byte Count"]
    #[must_use]
    #[inline(always)]
    pub const fn txcount(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Transmit Byte Count"]
    #[inline(always)]
    pub const fn set_txcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Receive Byte Count"]
    #[must_use]
    #[inline(always)]
    pub const fn rxcount(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "Receive Byte Count"]
    #[inline(always)]
    pub const fn set_rxcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "Transmit is Full"]
    #[must_use]
    #[inline(always)]
    pub const fn txfull(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit is Full"]
    #[inline(always)]
    pub const fn set_txfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Receive is Empty"]
    #[must_use]
    #[inline(always)]
    pub const fn rxempty(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Receive is Empty"]
    #[inline(always)]
    pub const fn set_rxempty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Mdatactrl {
    #[inline(always)]
    fn default() -> Mdatactrl {
        Mdatactrl(0)
    }
}
impl core::fmt::Debug for Mdatactrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mdatactrl")
            .field("flushtb", &self.flushtb())
            .field("flushfb", &self.flushfb())
            .field("unlock", &self.unlock())
            .field("txtrig", &self.txtrig())
            .field("rxtrig", &self.rxtrig())
            .field("txcount", &self.txcount())
            .field("rxcount", &self.rxcount())
            .field("txfull", &self.txfull())
            .field("rxempty", &self.rxempty())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mdatactrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mdatactrl {{ flushtb: {:?}, flushfb: {:?}, unlock: {=bool:?}, txtrig: {:?}, rxtrig: {:?}, txcount: {=u8:?}, rxcount: {=u8:?}, txfull: {=bool:?}, rxempty: {=bool:?} }}",
            self.flushtb(),
            self.flushfb(),
            self.unlock(),
            self.txtrig(),
            self.rxtrig(),
            self.txcount(),
            self.rxcount(),
            self.txfull(),
            self.rxempty()
        )
    }
}
#[doc = "Controller DMA Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mdmactrl(pub u32);
impl Mdmactrl {
    #[doc = "DMA from Bus"]
    #[must_use]
    #[inline(always)]
    pub const fn dmafb(&self) -> super::vals::MdmactrlDmafb {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::MdmactrlDmafb::from_bits(val as u8)
    }
    #[doc = "DMA from Bus"]
    #[inline(always)]
    pub const fn set_dmafb(&mut self, val: super::vals::MdmactrlDmafb) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "DMA to Bus"]
    #[must_use]
    #[inline(always)]
    pub const fn dmatb(&self) -> super::vals::MdmactrlDmatb {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::MdmactrlDmatb::from_bits(val as u8)
    }
    #[doc = "DMA to Bus"]
    #[inline(always)]
    pub const fn set_dmatb(&mut self, val: super::vals::MdmactrlDmatb) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "DMA Width"]
    #[must_use]
    #[inline(always)]
    pub const fn dmawidth(&self) -> super::vals::MdmactrlDmawidth {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::MdmactrlDmawidth::from_bits(val as u8)
    }
    #[doc = "DMA Width"]
    #[inline(always)]
    pub const fn set_dmawidth(&mut self, val: super::vals::MdmactrlDmawidth) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
}
impl Default for Mdmactrl {
    #[inline(always)]
    fn default() -> Mdmactrl {
        Mdmactrl(0)
    }
}
impl core::fmt::Debug for Mdmactrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mdmactrl")
            .field("dmafb", &self.dmafb())
            .field("dmatb", &self.dmatb())
            .field("dmawidth", &self.dmawidth())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mdmactrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mdmactrl {{ dmafb: {:?}, dmatb: {:?}, dmawidth: {:?} }}",
            self.dmafb(),
            self.dmatb(),
            self.dmawidth()
        )
    }
}
#[doc = "Controller Dynamic Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mdynaddr(pub u32);
impl Mdynaddr {
    #[doc = "Dynamic Address Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn davalid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Dynamic Address Valid"]
    #[inline(always)]
    pub const fn set_davalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Dynamic Address"]
    #[must_use]
    #[inline(always)]
    pub const fn daddr(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Dynamic Address"]
    #[inline(always)]
    pub const fn set_daddr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
}
impl Default for Mdynaddr {
    #[inline(always)]
    fn default() -> Mdynaddr {
        Mdynaddr(0)
    }
}
impl core::fmt::Debug for Mdynaddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mdynaddr")
            .field("davalid", &self.davalid())
            .field("daddr", &self.daddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mdynaddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mdynaddr {{ davalid: {=bool:?}, daddr: {=u8:?} }}",
            self.davalid(),
            self.daddr()
        )
    }
}
#[doc = "Controller Errors and Warnings"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Merrwarn(pub u32);
impl Merrwarn {
    #[doc = "Not Acknowledge Error"]
    #[must_use]
    #[inline(always)]
    pub const fn nack(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Not Acknowledge Error"]
    #[inline(always)]
    pub const fn set_nack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write Abort Error"]
    #[must_use]
    #[inline(always)]
    pub const fn wrabt(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Write Abort Error"]
    #[inline(always)]
    pub const fn set_wrabt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "High Data Rate Parity"]
    #[must_use]
    #[inline(always)]
    pub const fn hpar(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "High Data Rate Parity"]
    #[inline(always)]
    pub const fn set_hpar(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "High Data Rate CRC Error"]
    #[must_use]
    #[inline(always)]
    pub const fn hcrc(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "High Data Rate CRC Error"]
    #[inline(always)]
    pub const fn set_hcrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Overread Error"]
    #[must_use]
    #[inline(always)]
    pub const fn oread(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Overread Error"]
    #[inline(always)]
    pub const fn set_oread(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Overwrite Error"]
    #[must_use]
    #[inline(always)]
    pub const fn owrite(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Overwrite Error"]
    #[inline(always)]
    pub const fn set_owrite(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Message Error"]
    #[must_use]
    #[inline(always)]
    pub const fn msgerr(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Message Error"]
    #[inline(always)]
    pub const fn set_msgerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Invalid Request Error"]
    #[must_use]
    #[inline(always)]
    pub const fn invreq(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Invalid Request Error"]
    #[inline(always)]
    pub const fn set_invreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Timeout Error"]
    #[must_use]
    #[inline(always)]
    pub const fn timeout(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout Error"]
    #[inline(always)]
    pub const fn set_timeout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for Merrwarn {
    #[inline(always)]
    fn default() -> Merrwarn {
        Merrwarn(0)
    }
}
impl core::fmt::Debug for Merrwarn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Merrwarn")
            .field("nack", &self.nack())
            .field("wrabt", &self.wrabt())
            .field("hpar", &self.hpar())
            .field("hcrc", &self.hcrc())
            .field("oread", &self.oread())
            .field("owrite", &self.owrite())
            .field("msgerr", &self.msgerr())
            .field("invreq", &self.invreq())
            .field("timeout", &self.timeout())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Merrwarn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Merrwarn {{ nack: {=bool:?}, wrabt: {=bool:?}, hpar: {=bool:?}, hcrc: {=bool:?}, oread: {=bool:?}, owrite: {=bool:?}, msgerr: {=bool:?}, invreq: {=bool:?}, timeout: {=bool:?} }}",
            self.nack(),
            self.wrabt(),
            self.hpar(),
            self.hcrc(),
            self.oread(),
            self.owrite(),
            self.msgerr(),
            self.invreq(),
            self.timeout()
        )
    }
}
#[doc = "Controller In-band Interrupt Registry and Rules"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mibirules(pub u32);
impl Mibirules {
    #[doc = "ADDR0"]
    #[must_use]
    #[inline(always)]
    pub const fn addr0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "ADDR0"]
    #[inline(always)]
    pub const fn set_addr0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "ADDR1"]
    #[must_use]
    #[inline(always)]
    pub const fn addr1(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x3f;
        val as u8
    }
    #[doc = "ADDR1"]
    #[inline(always)]
    pub const fn set_addr1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 6usize)) | (((val as u32) & 0x3f) << 6usize);
    }
    #[doc = "ADDR2"]
    #[must_use]
    #[inline(always)]
    pub const fn addr2(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x3f;
        val as u8
    }
    #[doc = "ADDR2"]
    #[inline(always)]
    pub const fn set_addr2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 12usize)) | (((val as u32) & 0x3f) << 12usize);
    }
    #[doc = "ADDR3"]
    #[must_use]
    #[inline(always)]
    pub const fn addr3(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x3f;
        val as u8
    }
    #[doc = "ADDR3"]
    #[inline(always)]
    pub const fn set_addr3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 18usize)) | (((val as u32) & 0x3f) << 18usize);
    }
    #[doc = "ADDR4"]
    #[must_use]
    #[inline(always)]
    pub const fn addr4(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "ADDR4"]
    #[inline(always)]
    pub const fn set_addr4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
    #[doc = "Most Significant Address Bit is 0"]
    #[must_use]
    #[inline(always)]
    pub const fn msb0(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Most Significant Address Bit is 0"]
    #[inline(always)]
    pub const fn set_msb0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "No IBI byte"]
    #[must_use]
    #[inline(always)]
    pub const fn nobyte(&self) -> super::vals::Nobyte {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Nobyte::from_bits(val as u8)
    }
    #[doc = "No IBI byte"]
    #[inline(always)]
    pub const fn set_nobyte(&mut self, val: super::vals::Nobyte) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mibirules {
    #[inline(always)]
    fn default() -> Mibirules {
        Mibirules(0)
    }
}
impl core::fmt::Debug for Mibirules {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mibirules")
            .field("addr0", &self.addr0())
            .field("addr1", &self.addr1())
            .field("addr2", &self.addr2())
            .field("addr3", &self.addr3())
            .field("addr4", &self.addr4())
            .field("msb0", &self.msb0())
            .field("nobyte", &self.nobyte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mibirules {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mibirules {{ addr0: {=u8:?}, addr1: {=u8:?}, addr2: {=u8:?}, addr3: {=u8:?}, addr4: {=u8:?}, msb0: {=bool:?}, nobyte: {:?} }}",
            self.addr0(),
            self.addr1(),
            self.addr2(),
            self.addr3(),
            self.addr4(),
            self.msb0(),
            self.nobyte()
        )
    }
}
#[doc = "Controller Interrupt Clear"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mintclr(pub u32);
impl Mintclr {
    #[doc = "SLVSTART Interrupt Enable Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn slvstart(&self) -> super::vals::MintclrSlvstart {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::MintclrSlvstart::from_bits(val as u8)
    }
    #[doc = "SLVSTART Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn set_slvstart(&mut self, val: super::vals::MintclrSlvstart) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "MCTRLDONE Interrupt Enable Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn mctrldone(&self) -> super::vals::MintclrMctrldone {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::MintclrMctrldone::from_bits(val as u8)
    }
    #[doc = "MCTRLDONE Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn set_mctrldone(&mut self, val: super::vals::MintclrMctrldone) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "COMPLETE Interrupt Enable Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn complete(&self) -> super::vals::MintclrComplete {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::MintclrComplete::from_bits(val as u8)
    }
    #[doc = "COMPLETE Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn set_complete(&mut self, val: super::vals::MintclrComplete) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "RXPEND Interrupt Enable Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpend(&self) -> super::vals::MintclrRxpend {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::MintclrRxpend::from_bits(val as u8)
    }
    #[doc = "RXPEND Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn set_rxpend(&mut self, val: super::vals::MintclrRxpend) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "TXNOTFULL Interrupt Enable Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn txnotfull(&self) -> super::vals::MintclrTxnotfull {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::MintclrTxnotfull::from_bits(val as u8)
    }
    #[doc = "TXNOTFULL Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn set_txnotfull(&mut self, val: super::vals::MintclrTxnotfull) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "IBIWON Interrupt Enable Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn ibiwon(&self) -> super::vals::MintclrIbiwon {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::MintclrIbiwon::from_bits(val as u8)
    }
    #[doc = "IBIWON Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn set_ibiwon(&mut self, val: super::vals::MintclrIbiwon) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "ERRWARN Interrupt Enable Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn errwarn(&self) -> super::vals::MintclrErrwarn {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::MintclrErrwarn::from_bits(val as u8)
    }
    #[doc = "ERRWARN Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn set_errwarn(&mut self, val: super::vals::MintclrErrwarn) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "NOWCONTROLLER Interrupt Enable Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn nowmaster(&self) -> super::vals::MintclrNowmaster {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::MintclrNowmaster::from_bits(val as u8)
    }
    #[doc = "NOWCONTROLLER Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn set_nowmaster(&mut self, val: super::vals::MintclrNowmaster) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
}
impl Default for Mintclr {
    #[inline(always)]
    fn default() -> Mintclr {
        Mintclr(0)
    }
}
impl core::fmt::Debug for Mintclr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mintclr")
            .field("slvstart", &self.slvstart())
            .field("mctrldone", &self.mctrldone())
            .field("complete", &self.complete())
            .field("rxpend", &self.rxpend())
            .field("txnotfull", &self.txnotfull())
            .field("ibiwon", &self.ibiwon())
            .field("errwarn", &self.errwarn())
            .field("nowmaster", &self.nowmaster())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mintclr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mintclr {{ slvstart: {:?}, mctrldone: {:?}, complete: {:?}, rxpend: {:?}, txnotfull: {:?}, ibiwon: {:?}, errwarn: {:?}, nowmaster: {:?} }}",
            self.slvstart(),
            self.mctrldone(),
            self.complete(),
            self.rxpend(),
            self.txnotfull(),
            self.ibiwon(),
            self.errwarn(),
            self.nowmaster()
        )
    }
}
#[doc = "Controller Interrupt Mask"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mintmasked(pub u32);
impl Mintmasked {
    #[doc = "SLVSTART Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn slvstart(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SLVSTART Interrupt Mask"]
    #[inline(always)]
    pub const fn set_slvstart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "MCTRLDONE Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn mctrldone(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "MCTRLDONE Interrupt Mask"]
    #[inline(always)]
    pub const fn set_mctrldone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "COMPLETE Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn complete(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "COMPLETE Interrupt Mask"]
    #[inline(always)]
    pub const fn set_complete(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "RXPEND Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpend(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "RXPEND Interrupt Mask"]
    #[inline(always)]
    pub const fn set_rxpend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "TXNOTFULL Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn txnotfull(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "TXNOTFULL Interrupt Mask"]
    #[inline(always)]
    pub const fn set_txnotfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "IBIWON Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn ibiwon(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "IBIWON Interrupt Mask"]
    #[inline(always)]
    pub const fn set_ibiwon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "ERRWARN Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn errwarn(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "ERRWARN Interrupt Mask"]
    #[inline(always)]
    pub const fn set_errwarn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "NOWCONTROLLER Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn nowmaster(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "NOWCONTROLLER Interrupt Mask"]
    #[inline(always)]
    pub const fn set_nowmaster(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for Mintmasked {
    #[inline(always)]
    fn default() -> Mintmasked {
        Mintmasked(0)
    }
}
impl core::fmt::Debug for Mintmasked {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mintmasked")
            .field("slvstart", &self.slvstart())
            .field("mctrldone", &self.mctrldone())
            .field("complete", &self.complete())
            .field("rxpend", &self.rxpend())
            .field("txnotfull", &self.txnotfull())
            .field("ibiwon", &self.ibiwon())
            .field("errwarn", &self.errwarn())
            .field("nowmaster", &self.nowmaster())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mintmasked {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mintmasked {{ slvstart: {=bool:?}, mctrldone: {=bool:?}, complete: {=bool:?}, rxpend: {=bool:?}, txnotfull: {=bool:?}, ibiwon: {=bool:?}, errwarn: {=bool:?}, nowmaster: {=bool:?} }}",
            self.slvstart(),
            self.mctrldone(),
            self.complete(),
            self.rxpend(),
            self.txnotfull(),
            self.ibiwon(),
            self.errwarn(),
            self.nowmaster()
        )
    }
}
#[doc = "Controller Interrupt Set"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mintset(pub u32);
impl Mintset {
    #[doc = "Target Start Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn slvstart(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Target Start Interrupt Enable"]
    #[inline(always)]
    pub const fn set_slvstart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Controller Control Done Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mctrldone(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Controller Control Done Interrupt Enable"]
    #[inline(always)]
    pub const fn set_mctrldone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Completed Message Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn complete(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Completed Message Interrupt Enable"]
    #[inline(always)]
    pub const fn set_complete(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Receive Pending Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpend(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Pending Interrupt Enable"]
    #[inline(always)]
    pub const fn set_rxpend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Transmit Buffer/FIFO Not Full Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn txnotfull(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Buffer/FIFO Not Full Interrupt Enable"]
    #[inline(always)]
    pub const fn set_txnotfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "IBI Won Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ibiwon(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "IBI Won Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ibiwon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Error or Warning (ERRWARN) Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn errwarn(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Error or Warning (ERRWARN) Interrupt Enable"]
    #[inline(always)]
    pub const fn set_errwarn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Now Controller Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nowmaster(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Now Controller Interrupt Enable"]
    #[inline(always)]
    pub const fn set_nowmaster(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for Mintset {
    #[inline(always)]
    fn default() -> Mintset {
        Mintset(0)
    }
}
impl core::fmt::Debug for Mintset {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mintset")
            .field("slvstart", &self.slvstart())
            .field("mctrldone", &self.mctrldone())
            .field("complete", &self.complete())
            .field("rxpend", &self.rxpend())
            .field("txnotfull", &self.txnotfull())
            .field("ibiwon", &self.ibiwon())
            .field("errwarn", &self.errwarn())
            .field("nowmaster", &self.nowmaster())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mintset {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mintset {{ slvstart: {=bool:?}, mctrldone: {=bool:?}, complete: {=bool:?}, rxpend: {=bool:?}, txnotfull: {=bool:?}, ibiwon: {=bool:?}, errwarn: {=bool:?}, nowmaster: {=bool:?} }}",
            self.slvstart(),
            self.mctrldone(),
            self.complete(),
            self.rxpend(),
            self.txnotfull(),
            self.ibiwon(),
            self.errwarn(),
            self.nowmaster()
        )
    }
}
#[doc = "Controller Read Data Byte"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrdatab(pub u32);
impl Mrdatab {
    #[doc = "Value"]
    #[must_use]
    #[inline(always)]
    pub const fn value(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Value"]
    #[inline(always)]
    pub const fn set_value(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Mrdatab {
    #[inline(always)]
    fn default() -> Mrdatab {
        Mrdatab(0)
    }
}
impl core::fmt::Debug for Mrdatab {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mrdatab")
            .field("value", &self.value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mrdatab {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mrdatab {{ value: {=u8:?} }}", self.value())
    }
}
#[doc = "Controller Read Data Halfword"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrdatah(pub u32);
impl Mrdatah {
    #[doc = "Low Byte"]
    #[must_use]
    #[inline(always)]
    pub const fn lsb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Low Byte"]
    #[inline(always)]
    pub const fn set_lsb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "High Byte"]
    #[must_use]
    #[inline(always)]
    pub const fn msb(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "High Byte"]
    #[inline(always)]
    pub const fn set_msb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Mrdatah {
    #[inline(always)]
    fn default() -> Mrdatah {
        Mrdatah(0)
    }
}
impl core::fmt::Debug for Mrdatah {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mrdatah")
            .field("lsb", &self.lsb())
            .field("msb", &self.msb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mrdatah {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mrdatah {{ lsb: {=u8:?}, msb: {=u8:?} }}",
            self.lsb(),
            self.msb()
        )
    }
}
#[doc = "Controller Read Message in DDR mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrmsgDdr(pub u32);
impl MrmsgDdr {
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
impl Default for MrmsgDdr {
    #[inline(always)]
    fn default() -> MrmsgDdr {
        MrmsgDdr(0)
    }
}
impl core::fmt::Debug for MrmsgDdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrmsgDdr")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrmsgDdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrmsgDdr {{ data: {=u16:?} }}", self.data())
    }
}
#[doc = "Controller Read Message in SDR mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrmsgSdr(pub u32);
impl MrmsgSdr {
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
impl Default for MrmsgSdr {
    #[inline(always)]
    fn default() -> MrmsgSdr {
        MrmsgSdr(0)
    }
}
impl core::fmt::Debug for MrmsgSdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MrmsgSdr")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MrmsgSdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MrmsgSdr {{ data: {=u16:?} }}", self.data())
    }
}
#[doc = "Controller Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mstatus(pub u32);
impl Mstatus {
    #[doc = "State of the Controller"]
    #[must_use]
    #[inline(always)]
    pub const fn state(&self) -> super::vals::State {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::State::from_bits(val as u8)
    }
    #[doc = "State of the Controller"]
    #[inline(always)]
    pub const fn set_state(&mut self, val: super::vals::State) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Between"]
    #[must_use]
    #[inline(always)]
    pub const fn between(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Between"]
    #[inline(always)]
    pub const fn set_between(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Not Acknowledged"]
    #[must_use]
    #[inline(always)]
    pub const fn nacked(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Not Acknowledged"]
    #[inline(always)]
    pub const fn set_nacked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "In-Band Interrupt (IBI) Type"]
    #[must_use]
    #[inline(always)]
    pub const fn ibitype(&self) -> super::vals::Ibitype {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Ibitype::from_bits(val as u8)
    }
    #[doc = "In-Band Interrupt (IBI) Type"]
    #[inline(always)]
    pub const fn set_ibitype(&mut self, val: super::vals::Ibitype) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Target Start"]
    #[must_use]
    #[inline(always)]
    pub const fn slvstart(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Target Start"]
    #[inline(always)]
    pub const fn set_slvstart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Controller Control Done"]
    #[must_use]
    #[inline(always)]
    pub const fn mctrldone(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Controller Control Done"]
    #[inline(always)]
    pub const fn set_mctrldone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Complete"]
    #[must_use]
    #[inline(always)]
    pub const fn complete(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Complete"]
    #[inline(always)]
    pub const fn set_complete(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "RXPEND"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpend(&self) -> super::vals::MstatusRxpend {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::MstatusRxpend::from_bits(val as u8)
    }
    #[doc = "RXPEND"]
    #[inline(always)]
    pub const fn set_rxpend(&mut self, val: super::vals::MstatusRxpend) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "TX Buffer or FIFO Not Full"]
    #[must_use]
    #[inline(always)]
    pub const fn txnotfull(&self) -> super::vals::MstatusTxnotfull {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::MstatusTxnotfull::from_bits(val as u8)
    }
    #[doc = "TX Buffer or FIFO Not Full"]
    #[inline(always)]
    pub const fn set_txnotfull(&mut self, val: super::vals::MstatusTxnotfull) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "In-Band Interrupt (IBI) Won"]
    #[must_use]
    #[inline(always)]
    pub const fn ibiwon(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "In-Band Interrupt (IBI) Won"]
    #[inline(always)]
    pub const fn set_ibiwon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Error or Warning"]
    #[must_use]
    #[inline(always)]
    pub const fn errwarn(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Error or Warning"]
    #[inline(always)]
    pub const fn set_errwarn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Module is now Controller"]
    #[must_use]
    #[inline(always)]
    pub const fn nowmaster(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Module is now Controller"]
    #[inline(always)]
    pub const fn set_nowmaster(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "IBI Address"]
    #[must_use]
    #[inline(always)]
    pub const fn ibiaddr(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[doc = "IBI Address"]
    #[inline(always)]
    pub const fn set_ibiaddr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
}
impl Default for Mstatus {
    #[inline(always)]
    fn default() -> Mstatus {
        Mstatus(0)
    }
}
impl core::fmt::Debug for Mstatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mstatus")
            .field("state", &self.state())
            .field("between", &self.between())
            .field("nacked", &self.nacked())
            .field("ibitype", &self.ibitype())
            .field("slvstart", &self.slvstart())
            .field("mctrldone", &self.mctrldone())
            .field("complete", &self.complete())
            .field("rxpend", &self.rxpend())
            .field("txnotfull", &self.txnotfull())
            .field("ibiwon", &self.ibiwon())
            .field("errwarn", &self.errwarn())
            .field("nowmaster", &self.nowmaster())
            .field("ibiaddr", &self.ibiaddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mstatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mstatus {{ state: {:?}, between: {=bool:?}, nacked: {=bool:?}, ibitype: {:?}, slvstart: {=bool:?}, mctrldone: {=bool:?}, complete: {=bool:?}, rxpend: {:?}, txnotfull: {:?}, ibiwon: {=bool:?}, errwarn: {=bool:?}, nowmaster: {=bool:?}, ibiaddr: {=u8:?} }}",
            self.state(),
            self.between(),
            self.nacked(),
            self.ibitype(),
            self.slvstart(),
            self.mctrldone(),
            self.complete(),
            self.rxpend(),
            self.txnotfull(),
            self.ibiwon(),
            self.errwarn(),
            self.nowmaster(),
            self.ibiaddr()
        )
    }
}
#[doc = "Controller Write Data Byte"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mwdatab(pub u32);
impl Mwdatab {
    #[doc = "Data Byte"]
    #[must_use]
    #[inline(always)]
    pub const fn value(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte"]
    #[inline(always)]
    pub const fn set_value(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "End of Message"]
    #[must_use]
    #[inline(always)]
    pub const fn end(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "End of Message"]
    #[inline(always)]
    pub const fn set_end(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "End of Message ALSO"]
    #[must_use]
    #[inline(always)]
    pub const fn end_also(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "End of Message ALSO"]
    #[inline(always)]
    pub const fn set_end_also(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Mwdatab {
    #[inline(always)]
    fn default() -> Mwdatab {
        Mwdatab(0)
    }
}
impl core::fmt::Debug for Mwdatab {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mwdatab")
            .field("value", &self.value())
            .field("end", &self.end())
            .field("end_also", &self.end_also())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mwdatab {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mwdatab {{ value: {=u8:?}, end: {=bool:?}, end_also: {=bool:?} }}",
            self.value(),
            self.end(),
            self.end_also()
        )
    }
}
#[doc = "Controller Write Byte Data 1 (to Bus)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mwdatab1(pub u32);
impl Mwdatab1 {
    #[doc = "Value"]
    #[must_use]
    #[inline(always)]
    pub const fn value(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Value"]
    #[inline(always)]
    pub const fn set_value(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Mwdatab1 {
    #[inline(always)]
    fn default() -> Mwdatab1 {
        Mwdatab1(0)
    }
}
impl core::fmt::Debug for Mwdatab1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mwdatab1")
            .field("value", &self.value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mwdatab1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mwdatab1 {{ value: {=u8:?} }}", self.value())
    }
}
#[doc = "Controller Write Data Byte End"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mwdatabe(pub u32);
impl Mwdatabe {
    #[doc = "Data"]
    #[must_use]
    #[inline(always)]
    pub const fn value(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data"]
    #[inline(always)]
    pub const fn set_value(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Mwdatabe {
    #[inline(always)]
    fn default() -> Mwdatabe {
        Mwdatabe(0)
    }
}
impl core::fmt::Debug for Mwdatabe {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mwdatabe")
            .field("value", &self.value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mwdatabe {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mwdatabe {{ value: {=u8:?} }}", self.value())
    }
}
#[doc = "Controller Write Data Halfword"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mwdatah(pub u32);
impl Mwdatah {
    #[doc = "Data Byte 0"]
    #[must_use]
    #[inline(always)]
    pub const fn data0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 0"]
    #[inline(always)]
    pub const fn set_data0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data Byte 1"]
    #[must_use]
    #[inline(always)]
    pub const fn data1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 1"]
    #[inline(always)]
    pub const fn set_data1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "End of Message"]
    #[must_use]
    #[inline(always)]
    pub const fn end(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "End of Message"]
    #[inline(always)]
    pub const fn set_end(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Mwdatah {
    #[inline(always)]
    fn default() -> Mwdatah {
        Mwdatah(0)
    }
}
impl core::fmt::Debug for Mwdatah {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mwdatah")
            .field("data0", &self.data0())
            .field("data1", &self.data1())
            .field("end", &self.end())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mwdatah {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mwdatah {{ data0: {=u8:?}, data1: {=u8:?}, end: {=bool:?} }}",
            self.data0(),
            self.data1(),
            self.end()
        )
    }
}
#[doc = "Controller Write Data Halfword End"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mwdatahe(pub u32);
impl Mwdatahe {
    #[doc = "Data Byte 0"]
    #[must_use]
    #[inline(always)]
    pub const fn data0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 0"]
    #[inline(always)]
    pub const fn set_data0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data Byte 1"]
    #[must_use]
    #[inline(always)]
    pub const fn data1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data Byte 1"]
    #[inline(always)]
    pub const fn set_data1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Mwdatahe {
    #[inline(always)]
    fn default() -> Mwdatahe {
        Mwdatahe(0)
    }
}
impl core::fmt::Debug for Mwdatahe {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mwdatahe")
            .field("data0", &self.data0())
            .field("data1", &self.data1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mwdatahe {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mwdatahe {{ data0: {=u8:?}, data1: {=u8:?} }}",
            self.data0(),
            self.data1()
        )
    }
}
#[doc = "Controller Write Message in DDR mode: First Control Word"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MwmsgDdrControl(pub u32);
impl MwmsgDdrControl {
    #[doc = "Address Command"]
    #[must_use]
    #[inline(always)]
    pub const fn addrcmd(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Address Command"]
    #[inline(always)]
    pub const fn set_addrcmd(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for MwmsgDdrControl {
    #[inline(always)]
    fn default() -> MwmsgDdrControl {
        MwmsgDdrControl(0)
    }
}
impl core::fmt::Debug for MwmsgDdrControl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MwmsgDdrControl")
            .field("addrcmd", &self.addrcmd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MwmsgDdrControl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MwmsgDdrControl {{ addrcmd: {=u16:?} }}", self.addrcmd())
    }
}
#[doc = "Controller Write Message in DDR Mode Control 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MwmsgDdrControl2(pub u32);
impl MwmsgDdrControl2 {
    #[doc = "Length of Message"]
    #[must_use]
    #[inline(always)]
    pub const fn len(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Length of Message"]
    #[inline(always)]
    pub const fn set_len(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "End of Message"]
    #[must_use]
    #[inline(always)]
    pub const fn end(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "End of Message"]
    #[inline(always)]
    pub const fn set_end(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for MwmsgDdrControl2 {
    #[inline(always)]
    fn default() -> MwmsgDdrControl2 {
        MwmsgDdrControl2(0)
    }
}
impl core::fmt::Debug for MwmsgDdrControl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MwmsgDdrControl2")
            .field("len", &self.len())
            .field("end", &self.end())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MwmsgDdrControl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MwmsgDdrControl2 {{ len: {=u16:?}, end: {=bool:?} }}",
            self.len(),
            self.end()
        )
    }
}
#[doc = "Controller Write Message Data in DDR mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MwmsgDdrData(pub u32);
impl MwmsgDdrData {
    #[doc = "Data"]
    #[must_use]
    #[inline(always)]
    pub const fn data16b(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Data"]
    #[inline(always)]
    pub const fn set_data16b(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for MwmsgDdrData {
    #[inline(always)]
    fn default() -> MwmsgDdrData {
        MwmsgDdrData(0)
    }
}
impl core::fmt::Debug for MwmsgDdrData {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MwmsgDdrData")
            .field("data16b", &self.data16b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MwmsgDdrData {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MwmsgDdrData {{ data16b: {=u16:?} }}", self.data16b())
    }
}
#[doc = "Controller Write Message Control in SDR mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MwmsgSdrControl(pub u32);
impl MwmsgSdrControl {
    #[doc = "Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn dir(&self) -> super::vals::MwmsgSdrControlDir {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::MwmsgSdrControlDir::from_bits(val as u8)
    }
    #[doc = "Direction"]
    #[inline(always)]
    pub const fn set_dir(&mut self, val: super::vals::MwmsgSdrControlDir) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Address"]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Address"]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
    #[doc = "End of SDR Message"]
    #[must_use]
    #[inline(always)]
    pub const fn end(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "End of SDR Message"]
    #[inline(always)]
    pub const fn set_end(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "I2C"]
    #[must_use]
    #[inline(always)]
    pub const fn i2c(&self) -> super::vals::I2c {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::I2c::from_bits(val as u8)
    }
    #[doc = "I2C"]
    #[inline(always)]
    pub const fn set_i2c(&mut self, val: super::vals::I2c) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Length"]
    #[must_use]
    #[inline(always)]
    pub const fn len(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Length"]
    #[inline(always)]
    pub const fn set_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
    }
}
impl Default for MwmsgSdrControl {
    #[inline(always)]
    fn default() -> MwmsgSdrControl {
        MwmsgSdrControl(0)
    }
}
impl core::fmt::Debug for MwmsgSdrControl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MwmsgSdrControl")
            .field("dir", &self.dir())
            .field("addr", &self.addr())
            .field("end", &self.end())
            .field("i2c", &self.i2c())
            .field("len", &self.len())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MwmsgSdrControl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MwmsgSdrControl {{ dir: {:?}, addr: {=u8:?}, end: {=bool:?}, i2c: {:?}, len: {=u8:?} }}",
            self.dir(),
            self.addr(),
            self.end(),
            self.i2c(),
            self.len()
        )
    }
}
#[doc = "Controller Write Message Data in SDR mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MwmsgSdrData(pub u32);
impl MwmsgSdrData {
    #[doc = "Data"]
    #[must_use]
    #[inline(always)]
    pub const fn data16b(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Data"]
    #[inline(always)]
    pub const fn set_data16b(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for MwmsgSdrData {
    #[inline(always)]
    fn default() -> MwmsgSdrData {
        MwmsgSdrData(0)
    }
}
impl core::fmt::Debug for MwmsgSdrData {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MwmsgSdrData")
            .field("data16b", &self.data16b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MwmsgSdrData {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MwmsgSdrData {{ data16b: {=u16:?} }}", self.data16b())
    }
}
#[doc = "Target Capabilities"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scapabilities(pub u32);
impl Scapabilities {
    #[doc = "ID 48b Handler"]
    #[must_use]
    #[inline(always)]
    pub const fn idena(&self) -> super::vals::Idena {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Idena::from_bits(val as u8)
    }
    #[doc = "ID 48b Handler"]
    #[inline(always)]
    pub const fn set_idena(&mut self, val: super::vals::Idena) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "ID Register"]
    #[must_use]
    #[inline(always)]
    pub const fn idreg(&self) -> super::vals::Idreg {
        let val = (self.0 >> 2usize) & 0x0f;
        super::vals::Idreg::from_bits(val as u8)
    }
    #[doc = "ID Register"]
    #[inline(always)]
    pub const fn set_idreg(&mut self, val: super::vals::Idreg) {
        self.0 = (self.0 & !(0x0f << 2usize)) | (((val.to_bits() as u32) & 0x0f) << 2usize);
    }
    #[doc = "High Data Rate Support"]
    #[must_use]
    #[inline(always)]
    pub const fn hdrsupp(&self) -> super::vals::Hdrsupp {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Hdrsupp::from_bits(val as u8)
    }
    #[doc = "High Data Rate Support"]
    #[inline(always)]
    pub const fn set_hdrsupp(&mut self, val: super::vals::Hdrsupp) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Controller"]
    #[must_use]
    #[inline(always)]
    pub const fn master(&self) -> super::vals::Master {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Master::from_bits(val as u8)
    }
    #[doc = "Controller"]
    #[inline(always)]
    pub const fn set_master(&mut self, val: super::vals::Master) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Static Address"]
    #[must_use]
    #[inline(always)]
    pub const fn saddr(&self) -> super::vals::Saddr {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Saddr::from_bits(val as u8)
    }
    #[doc = "Static Address"]
    #[inline(always)]
    pub const fn set_saddr(&mut self, val: super::vals::Saddr) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Common Command Codes Handling"]
    #[must_use]
    #[inline(always)]
    pub const fn ccchandle(&self) -> super::vals::Ccchandle {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Ccchandle::from_bits(val as u8)
    }
    #[doc = "Common Command Codes Handling"]
    #[inline(always)]
    pub const fn set_ccchandle(&mut self, val: super::vals::Ccchandle) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "In-Band Interrupts, Controller Requests, Hot-Join Events"]
    #[must_use]
    #[inline(always)]
    pub const fn ibi_mr_hj(&self) -> super::vals::IbiMrHj {
        let val = (self.0 >> 16usize) & 0x1f;
        super::vals::IbiMrHj::from_bits(val as u8)
    }
    #[doc = "In-Band Interrupts, Controller Requests, Hot-Join Events"]
    #[inline(always)]
    pub const fn set_ibi_mr_hj(&mut self, val: super::vals::IbiMrHj) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val.to_bits() as u32) & 0x1f) << 16usize);
    }
    #[doc = "Time Control"]
    #[must_use]
    #[inline(always)]
    pub const fn timectrl(&self) -> super::vals::ScapabilitiesTimectrl {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::ScapabilitiesTimectrl::from_bits(val as u8)
    }
    #[doc = "Time Control"]
    #[inline(always)]
    pub const fn set_timectrl(&mut self, val: super::vals::ScapabilitiesTimectrl) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "External FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn extfifo(&self) -> super::vals::Extfifo {
        let val = (self.0 >> 23usize) & 0x07;
        super::vals::Extfifo::from_bits(val as u8)
    }
    #[doc = "External FIFO"]
    #[inline(always)]
    pub const fn set_extfifo(&mut self, val: super::vals::Extfifo) {
        self.0 = (self.0 & !(0x07 << 23usize)) | (((val.to_bits() as u32) & 0x07) << 23usize);
    }
    #[doc = "FIFO Transmit"]
    #[must_use]
    #[inline(always)]
    pub const fn fifotx(&self) -> super::vals::Fifotx {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Fifotx::from_bits(val as u8)
    }
    #[doc = "FIFO Transmit"]
    #[inline(always)]
    pub const fn set_fifotx(&mut self, val: super::vals::Fifotx) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "FIFO Receive"]
    #[must_use]
    #[inline(always)]
    pub const fn fiforx(&self) -> super::vals::Fiforx {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Fiforx::from_bits(val as u8)
    }
    #[doc = "FIFO Receive"]
    #[inline(always)]
    pub const fn set_fiforx(&mut self, val: super::vals::Fiforx) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Interrupts"]
    #[must_use]
    #[inline(always)]
    pub const fn int(&self) -> super::vals::Int {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Int::from_bits(val as u8)
    }
    #[doc = "Interrupts"]
    #[inline(always)]
    pub const fn set_int(&mut self, val: super::vals::Int) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Direct Memory Access"]
    #[must_use]
    #[inline(always)]
    pub const fn dma(&self) -> super::vals::Dma {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Dma::from_bits(val as u8)
    }
    #[doc = "Direct Memory Access"]
    #[inline(always)]
    pub const fn set_dma(&mut self, val: super::vals::Dma) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Scapabilities {
    #[inline(always)]
    fn default() -> Scapabilities {
        Scapabilities(0)
    }
}
impl core::fmt::Debug for Scapabilities {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scapabilities")
            .field("idena", &self.idena())
            .field("idreg", &self.idreg())
            .field("hdrsupp", &self.hdrsupp())
            .field("master", &self.master())
            .field("saddr", &self.saddr())
            .field("ccchandle", &self.ccchandle())
            .field("ibi_mr_hj", &self.ibi_mr_hj())
            .field("timectrl", &self.timectrl())
            .field("extfifo", &self.extfifo())
            .field("fifotx", &self.fifotx())
            .field("fiforx", &self.fiforx())
            .field("int", &self.int())
            .field("dma", &self.dma())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scapabilities {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scapabilities {{ idena: {:?}, idreg: {:?}, hdrsupp: {:?}, master: {:?}, saddr: {:?}, ccchandle: {:?}, ibi_mr_hj: {:?}, timectrl: {:?}, extfifo: {:?}, fifotx: {:?}, fiforx: {:?}, int: {:?}, dma: {:?} }}",
            self.idena(),
            self.idreg(),
            self.hdrsupp(),
            self.master(),
            self.saddr(),
            self.ccchandle(),
            self.ibi_mr_hj(),
            self.timectrl(),
            self.extfifo(),
            self.fifotx(),
            self.fiforx(),
            self.int(),
            self.dma()
        )
    }
}
#[doc = "Target Capabilities 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scapabilities2(pub u32);
impl Scapabilities2 {
    #[doc = "Map Count"]
    #[must_use]
    #[inline(always)]
    pub const fn mapcnt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Map Count"]
    #[inline(always)]
    pub const fn set_mapcnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "I2C 10-bit Address"]
    #[must_use]
    #[inline(always)]
    pub const fn i2c10b(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "I2C 10-bit Address"]
    #[inline(always)]
    pub const fn set_i2c10b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "I2C Software Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn i2crst(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "I2C Software Reset"]
    #[inline(always)]
    pub const fn set_i2crst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "I2C Device ID"]
    #[must_use]
    #[inline(always)]
    pub const fn i2cdevid(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "I2C Device ID"]
    #[inline(always)]
    pub const fn set_i2cdevid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "In-Band Interrupt EXTDATA"]
    #[must_use]
    #[inline(always)]
    pub const fn ibiext(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "In-Band Interrupt EXTDATA"]
    #[inline(always)]
    pub const fn set_ibiext(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "In-Band Interrupt Extended Register"]
    #[must_use]
    #[inline(always)]
    pub const fn ibixreg(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "In-Band Interrupt Extended Register"]
    #[inline(always)]
    pub const fn set_ibixreg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Target Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn slvrst(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Target Reset"]
    #[inline(always)]
    pub const fn set_slvrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Group"]
    #[must_use]
    #[inline(always)]
    pub const fn group(&self) -> super::vals::Group {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::Group::from_bits(val as u8)
    }
    #[doc = "Group"]
    #[inline(always)]
    pub const fn set_group(&mut self, val: super::vals::Group) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "SETAASA"]
    #[must_use]
    #[inline(always)]
    pub const fn aasa(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "SETAASA"]
    #[inline(always)]
    pub const fn set_aasa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Target-Target(s)-Tunnel Subscriber Capable"]
    #[must_use]
    #[inline(always)]
    pub const fn sstsub(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Target-Target(s)-Tunnel Subscriber Capable"]
    #[inline(always)]
    pub const fn set_sstsub(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Target-Target(s)-Tunnel Write Capable"]
    #[must_use]
    #[inline(always)]
    pub const fn sstwr(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Target-Target(s)-Tunnel Write Capable"]
    #[inline(always)]
    pub const fn set_sstwr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Scapabilities2 {
    #[inline(always)]
    fn default() -> Scapabilities2 {
        Scapabilities2(0)
    }
}
impl core::fmt::Debug for Scapabilities2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scapabilities2")
            .field("mapcnt", &self.mapcnt())
            .field("i2c10b", &self.i2c10b())
            .field("i2crst", &self.i2crst())
            .field("i2cdevid", &self.i2cdevid())
            .field("ibiext", &self.ibiext())
            .field("ibixreg", &self.ibixreg())
            .field("slvrst", &self.slvrst())
            .field("group", &self.group())
            .field("aasa", &self.aasa())
            .field("sstsub", &self.sstsub())
            .field("sstwr", &self.sstwr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scapabilities2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scapabilities2 {{ mapcnt: {=u8:?}, i2c10b: {=bool:?}, i2crst: {=bool:?}, i2cdevid: {=bool:?}, ibiext: {=bool:?}, ibixreg: {=bool:?}, slvrst: {=bool:?}, group: {:?}, aasa: {=bool:?}, sstsub: {=bool:?}, sstwr: {=bool:?} }}",
            self.mapcnt(),
            self.i2c10b(),
            self.i2crst(),
            self.i2cdevid(),
            self.ibiext(),
            self.ibixreg(),
            self.slvrst(),
            self.group(),
            self.aasa(),
            self.sstsub(),
            self.sstwr()
        )
    }
}
#[doc = "Target Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sconfig(pub u32);
impl Sconfig {
    #[doc = "Target Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn slvena(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Target Enable"]
    #[inline(always)]
    pub const fn set_slvena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Not Acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn nack(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Not Acknowledge"]
    #[inline(always)]
    pub const fn set_nack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Match Start or Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn matchss(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Match Start or Stop"]
    #[inline(always)]
    pub const fn set_matchss(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Ignore TE0 or TE1 Errors"]
    #[must_use]
    #[inline(always)]
    pub const fn s0ignore(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Ignore TE0 or TE1 Errors"]
    #[inline(always)]
    pub const fn set_s0ignore(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Double Data Rate OK"]
    #[must_use]
    #[inline(always)]
    pub const fn ddrok(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Double Data Rate OK"]
    #[inline(always)]
    pub const fn set_ddrok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "ID random"]
    #[must_use]
    #[inline(always)]
    pub const fn idrand(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "ID random"]
    #[inline(always)]
    pub const fn set_idrand(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Offline"]
    #[must_use]
    #[inline(always)]
    pub const fn offline(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Offline"]
    #[inline(always)]
    pub const fn set_offline(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Bus Available Match"]
    #[must_use]
    #[inline(always)]
    pub const fn bamatch(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Bus Available Match"]
    #[inline(always)]
    pub const fn set_bamatch(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Static Address"]
    #[must_use]
    #[inline(always)]
    pub const fn saddr(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x7f;
        val as u8
    }
    #[doc = "Static Address"]
    #[inline(always)]
    pub const fn set_saddr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
    }
}
impl Default for Sconfig {
    #[inline(always)]
    fn default() -> Sconfig {
        Sconfig(0)
    }
}
impl core::fmt::Debug for Sconfig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sconfig")
            .field("slvena", &self.slvena())
            .field("nack", &self.nack())
            .field("matchss", &self.matchss())
            .field("s0ignore", &self.s0ignore())
            .field("ddrok", &self.ddrok())
            .field("idrand", &self.idrand())
            .field("offline", &self.offline())
            .field("bamatch", &self.bamatch())
            .field("saddr", &self.saddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sconfig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sconfig {{ slvena: {=bool:?}, nack: {=bool:?}, matchss: {=bool:?}, s0ignore: {=bool:?}, ddrok: {=bool:?}, idrand: {=bool:?}, offline: {=bool:?}, bamatch: {=u8:?}, saddr: {=u8:?} }}",
            self.slvena(),
            self.nack(),
            self.matchss(),
            self.s0ignore(),
            self.ddrok(),
            self.idrand(),
            self.offline(),
            self.bamatch(),
            self.saddr()
        )
    }
}
#[doc = "Target Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sctrl(pub u32);
impl Sctrl {
    #[doc = "Event"]
    #[must_use]
    #[inline(always)]
    pub const fn event(&self) -> super::vals::SctrlEvent {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SctrlEvent::from_bits(val as u8)
    }
    #[doc = "Event"]
    #[inline(always)]
    pub const fn set_event(&mut self, val: super::vals::SctrlEvent) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Extended Data"]
    #[must_use]
    #[inline(always)]
    pub const fn extdata(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Extended Data"]
    #[inline(always)]
    pub const fn set_extdata(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "In-Band Interrupt Data"]
    #[must_use]
    #[inline(always)]
    pub const fn ibidata(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "In-Band Interrupt Data"]
    #[inline(always)]
    pub const fn set_ibidata(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Pending Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn pendint(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Pending Interrupt"]
    #[inline(always)]
    pub const fn set_pendint(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Activity State of Target"]
    #[must_use]
    #[inline(always)]
    pub const fn actstate(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Activity State of Target"]
    #[inline(always)]
    pub const fn set_actstate(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "Vendor Information"]
    #[must_use]
    #[inline(always)]
    pub const fn vendinfo(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Vendor Information"]
    #[inline(always)]
    pub const fn set_vendinfo(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Sctrl {
    #[inline(always)]
    fn default() -> Sctrl {
        Sctrl(0)
    }
}
impl core::fmt::Debug for Sctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sctrl")
            .field("event", &self.event())
            .field("extdata", &self.extdata())
            .field("ibidata", &self.ibidata())
            .field("pendint", &self.pendint())
            .field("actstate", &self.actstate())
            .field("vendinfo", &self.vendinfo())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sctrl {{ event: {:?}, extdata: {=bool:?}, ibidata: {=u8:?}, pendint: {=u8:?}, actstate: {=u8:?}, vendinfo: {=u8:?} }}",
            self.event(),
            self.extdata(),
            self.ibidata(),
            self.pendint(),
            self.actstate(),
            self.vendinfo()
        )
    }
}
#[doc = "Target Data Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdatactrl(pub u32);
impl Sdatactrl {
    #[doc = "Flush To-Bus Buffer or FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn flushtb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Flush To-Bus Buffer or FIFO"]
    #[inline(always)]
    pub const fn set_flushtb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Flush From-Bus Buffer or FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn flushfb(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Flush From-Bus Buffer or FIFO"]
    #[inline(always)]
    pub const fn set_flushfb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Unlock"]
    #[must_use]
    #[inline(always)]
    pub const fn unlock(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Unlock"]
    #[inline(always)]
    pub const fn set_unlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Transmit Trigger Level"]
    #[must_use]
    #[inline(always)]
    pub const fn txtrig(&self) -> super::vals::SdatactrlTxtrig {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SdatactrlTxtrig::from_bits(val as u8)
    }
    #[doc = "Transmit Trigger Level"]
    #[inline(always)]
    pub const fn set_txtrig(&mut self, val: super::vals::SdatactrlTxtrig) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Receive Trigger Level"]
    #[must_use]
    #[inline(always)]
    pub const fn rxtrig(&self) -> super::vals::SdatactrlRxtrig {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::SdatactrlRxtrig::from_bits(val as u8)
    }
    #[doc = "Receive Trigger Level"]
    #[inline(always)]
    pub const fn set_rxtrig(&mut self, val: super::vals::SdatactrlRxtrig) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Count of Bytes in Transmit"]
    #[must_use]
    #[inline(always)]
    pub const fn txcount(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Count of Bytes in Transmit"]
    #[inline(always)]
    pub const fn set_txcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Count of Bytes in Receive"]
    #[must_use]
    #[inline(always)]
    pub const fn rxcount(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "Count of Bytes in Receive"]
    #[inline(always)]
    pub const fn set_rxcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "Transmit is Full"]
    #[must_use]
    #[inline(always)]
    pub const fn txfull(&self) -> super::vals::SdatactrlTxfull {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::SdatactrlTxfull::from_bits(val as u8)
    }
    #[doc = "Transmit is Full"]
    #[inline(always)]
    pub const fn set_txfull(&mut self, val: super::vals::SdatactrlTxfull) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Receive is Empty"]
    #[must_use]
    #[inline(always)]
    pub const fn rxempty(&self) -> super::vals::SdatactrlRxempty {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SdatactrlRxempty::from_bits(val as u8)
    }
    #[doc = "Receive is Empty"]
    #[inline(always)]
    pub const fn set_rxempty(&mut self, val: super::vals::SdatactrlRxempty) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Sdatactrl {
    #[inline(always)]
    fn default() -> Sdatactrl {
        Sdatactrl(0)
    }
}
impl core::fmt::Debug for Sdatactrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sdatactrl")
            .field("flushtb", &self.flushtb())
            .field("flushfb", &self.flushfb())
            .field("unlock", &self.unlock())
            .field("txtrig", &self.txtrig())
            .field("rxtrig", &self.rxtrig())
            .field("txcount", &self.txcount())
            .field("rxcount", &self.rxcount())
            .field("txfull", &self.txfull())
            .field("rxempty", &self.rxempty())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sdatactrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sdatactrl {{ flushtb: {=bool:?}, flushfb: {=bool:?}, unlock: {=bool:?}, txtrig: {:?}, rxtrig: {:?}, txcount: {=u8:?}, rxcount: {=u8:?}, txfull: {:?}, rxempty: {:?} }}",
            self.flushtb(),
            self.flushfb(),
            self.unlock(),
            self.txtrig(),
            self.rxtrig(),
            self.txcount(),
            self.rxcount(),
            self.txfull(),
            self.rxempty()
        )
    }
}
#[doc = "Target DMA Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdmactrl(pub u32);
impl Sdmactrl {
    #[doc = "DMA Read (From-Bus) Trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn dmafb(&self) -> super::vals::SdmactrlDmafb {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SdmactrlDmafb::from_bits(val as u8)
    }
    #[doc = "DMA Read (From-Bus) Trigger"]
    #[inline(always)]
    pub const fn set_dmafb(&mut self, val: super::vals::SdmactrlDmafb) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "DMA Write (To-Bus) Trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn dmatb(&self) -> super::vals::SdmactrlDmatb {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SdmactrlDmatb::from_bits(val as u8)
    }
    #[doc = "DMA Write (To-Bus) Trigger"]
    #[inline(always)]
    pub const fn set_dmatb(&mut self, val: super::vals::SdmactrlDmatb) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Width of DMA Operations"]
    #[must_use]
    #[inline(always)]
    pub const fn dmawidth(&self) -> super::vals::SdmactrlDmawidth {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SdmactrlDmawidth::from_bits(val as u8)
    }
    #[doc = "Width of DMA Operations"]
    #[inline(always)]
    pub const fn set_dmawidth(&mut self, val: super::vals::SdmactrlDmawidth) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
}
impl Default for Sdmactrl {
    #[inline(always)]
    fn default() -> Sdmactrl {
        Sdmactrl(0)
    }
}
impl core::fmt::Debug for Sdmactrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sdmactrl")
            .field("dmafb", &self.dmafb())
            .field("dmatb", &self.dmatb())
            .field("dmawidth", &self.dmawidth())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sdmactrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sdmactrl {{ dmafb: {:?}, dmatb: {:?}, dmawidth: {:?} }}",
            self.dmafb(),
            self.dmatb(),
            self.dmawidth()
        )
    }
}
#[doc = "Target Dynamic Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdynaddr(pub u32);
impl Sdynaddr {
    #[doc = "Dynamic Address Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn davalid(&self) -> super::vals::SdynaddrDavalid {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SdynaddrDavalid::from_bits(val as u8)
    }
    #[doc = "Dynamic Address Valid"]
    #[inline(always)]
    pub const fn set_davalid(&mut self, val: super::vals::SdynaddrDavalid) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Dynamic Address"]
    #[must_use]
    #[inline(always)]
    pub const fn daddr(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Dynamic Address"]
    #[inline(always)]
    pub const fn set_daddr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
    #[doc = "Map a Static Address"]
    #[must_use]
    #[inline(always)]
    pub const fn mapsa(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Map a Static Address"]
    #[inline(always)]
    pub const fn set_mapsa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "10-Bit Static Address"]
    #[must_use]
    #[inline(always)]
    pub const fn sa10b(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "10-Bit Static Address"]
    #[inline(always)]
    pub const fn set_sa10b(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "Key"]
    #[must_use]
    #[inline(always)]
    pub const fn key(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Key"]
    #[inline(always)]
    pub const fn set_key(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Sdynaddr {
    #[inline(always)]
    fn default() -> Sdynaddr {
        Sdynaddr(0)
    }
}
impl core::fmt::Debug for Sdynaddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sdynaddr")
            .field("davalid", &self.davalid())
            .field("daddr", &self.daddr())
            .field("mapsa", &self.mapsa())
            .field("sa10b", &self.sa10b())
            .field("key", &self.key())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sdynaddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sdynaddr {{ davalid: {:?}, daddr: {=u8:?}, mapsa: {=bool:?}, sa10b: {=u8:?}, key: {=u16:?} }}",
            self.davalid(),
            self.daddr(),
            self.mapsa(),
            self.sa10b(),
            self.key()
        )
    }
}
#[doc = "Target Errors and Warnings"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Serrwarn(pub u32);
impl Serrwarn {
    #[doc = "Overrun Error"]
    #[must_use]
    #[inline(always)]
    pub const fn orun(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun Error"]
    #[inline(always)]
    pub const fn set_orun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Underrun Error"]
    #[must_use]
    #[inline(always)]
    pub const fn urun(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Underrun Error"]
    #[inline(always)]
    pub const fn set_urun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Underrun and Not Acknowledged (NACKed) Error"]
    #[must_use]
    #[inline(always)]
    pub const fn urunnack(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Underrun and Not Acknowledged (NACKed) Error"]
    #[inline(always)]
    pub const fn set_urunnack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Terminated Error"]
    #[must_use]
    #[inline(always)]
    pub const fn term(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Terminated Error"]
    #[inline(always)]
    pub const fn set_term(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Invalid Start Error"]
    #[must_use]
    #[inline(always)]
    pub const fn invstart(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Invalid Start Error"]
    #[inline(always)]
    pub const fn set_invstart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "SDR Parity Error"]
    #[must_use]
    #[inline(always)]
    pub const fn spar(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SDR Parity Error"]
    #[inline(always)]
    pub const fn set_spar(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "HDR Parity Error"]
    #[must_use]
    #[inline(always)]
    pub const fn hpar(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "HDR Parity Error"]
    #[inline(always)]
    pub const fn set_hpar(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "HDR-DDR CRC Error"]
    #[must_use]
    #[inline(always)]
    pub const fn hcrc(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "HDR-DDR CRC Error"]
    #[inline(always)]
    pub const fn set_hcrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "TE0 or TE1 Error"]
    #[must_use]
    #[inline(always)]
    pub const fn s0s1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "TE0 or TE1 Error"]
    #[inline(always)]
    pub const fn set_s0s1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Over-Read Error"]
    #[must_use]
    #[inline(always)]
    pub const fn oread(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Over-Read Error"]
    #[inline(always)]
    pub const fn set_oread(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Over-Write Error"]
    #[must_use]
    #[inline(always)]
    pub const fn owrite(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Over-Write Error"]
    #[inline(always)]
    pub const fn set_owrite(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Serrwarn {
    #[inline(always)]
    fn default() -> Serrwarn {
        Serrwarn(0)
    }
}
impl core::fmt::Debug for Serrwarn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Serrwarn")
            .field("orun", &self.orun())
            .field("urun", &self.urun())
            .field("urunnack", &self.urunnack())
            .field("term", &self.term())
            .field("invstart", &self.invstart())
            .field("spar", &self.spar())
            .field("hpar", &self.hpar())
            .field("hcrc", &self.hcrc())
            .field("s0s1", &self.s0s1())
            .field("oread", &self.oread())
            .field("owrite", &self.owrite())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Serrwarn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Serrwarn {{ orun: {=bool:?}, urun: {=bool:?}, urunnack: {=bool:?}, term: {=bool:?}, invstart: {=bool:?}, spar: {=bool:?}, hpar: {=bool:?}, hcrc: {=bool:?}, s0s1: {=bool:?}, oread: {=bool:?}, owrite: {=bool:?} }}",
            self.orun(),
            self.urun(),
            self.urunnack(),
            self.term(),
            self.invstart(),
            self.spar(),
            self.hpar(),
            self.hcrc(),
            self.s0s1(),
            self.oread(),
            self.owrite()
        )
    }
}
#[doc = "Target Module ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sid(pub u32);
impl Sid {
    #[doc = "ID"]
    #[must_use]
    #[inline(always)]
    pub const fn id(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "ID"]
    #[inline(always)]
    pub const fn set_id(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Sid {
    #[inline(always)]
    fn default() -> Sid {
        Sid(0)
    }
}
impl core::fmt::Debug for Sid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sid").field("id", &self.id()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sid {{ id: {=u32:?} }}", self.id())
    }
}
#[doc = "Target ID Extension"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sidext(pub u32);
impl Sidext {
    #[doc = "Device Characteristic Register"]
    #[must_use]
    #[inline(always)]
    pub const fn dcr(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Device Characteristic Register"]
    #[inline(always)]
    pub const fn set_dcr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Bus Characteristics Register"]
    #[must_use]
    #[inline(always)]
    pub const fn bcr(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Bus Characteristics Register"]
    #[inline(always)]
    pub const fn set_bcr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Sidext {
    #[inline(always)]
    fn default() -> Sidext {
        Sidext(0)
    }
}
impl core::fmt::Debug for Sidext {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sidext")
            .field("dcr", &self.dcr())
            .field("bcr", &self.bcr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sidext {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sidext {{ dcr: {=u8:?}, bcr: {=u8:?} }}",
            self.dcr(),
            self.bcr()
        )
    }
}
#[doc = "Target ID Part Number"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sidpartno(pub u32);
impl Sidpartno {
    #[doc = "Part Number"]
    #[must_use]
    #[inline(always)]
    pub const fn partno(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Part Number"]
    #[inline(always)]
    pub const fn set_partno(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Sidpartno {
    #[inline(always)]
    fn default() -> Sidpartno {
        Sidpartno(0)
    }
}
impl core::fmt::Debug for Sidpartno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sidpartno")
            .field("partno", &self.partno())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sidpartno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sidpartno {{ partno: {=u32:?} }}", self.partno())
    }
}
#[doc = "Target Interrupt Clear"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sintclr(pub u32);
impl Sintclr {
    #[doc = "START Interrupt Enable Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "START Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Matched Interrupt Enable Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn matched(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Matched Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn set_matched(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "STOP Interrupt Enable Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn stop(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "STOP Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn set_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "RXPEND Interrupt Enable Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpend(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "RXPEND Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn set_rxpend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "TXSEND Interrupt Enable Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn txsend(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "TXSEND Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn set_txsend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DACHG Interrupt Enable Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn dachg(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "DACHG Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn set_dachg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "CCC Interrupt Enable Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn ccc(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "CCC Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn set_ccc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "ERRWARN Interrupt Enable Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn errwarn(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "ERRWARN Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn set_errwarn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "DDRMATCHED Interrupt Enable Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn ddrmatched(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "DDRMATCHED Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn set_ddrmatched(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "CHANDLED Interrupt Enable Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn chandled(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "CHANDLED Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn set_chandled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "EVENT Interrupt Enable Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn event(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "EVENT Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn set_event(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for Sintclr {
    #[inline(always)]
    fn default() -> Sintclr {
        Sintclr(0)
    }
}
impl core::fmt::Debug for Sintclr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sintclr")
            .field("start", &self.start())
            .field("matched", &self.matched())
            .field("stop", &self.stop())
            .field("rxpend", &self.rxpend())
            .field("txsend", &self.txsend())
            .field("dachg", &self.dachg())
            .field("ccc", &self.ccc())
            .field("errwarn", &self.errwarn())
            .field("ddrmatched", &self.ddrmatched())
            .field("chandled", &self.chandled())
            .field("event", &self.event())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sintclr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sintclr {{ start: {=bool:?}, matched: {=bool:?}, stop: {=bool:?}, rxpend: {=bool:?}, txsend: {=bool:?}, dachg: {=bool:?}, ccc: {=bool:?}, errwarn: {=bool:?}, ddrmatched: {=bool:?}, chandled: {=bool:?}, event: {=bool:?} }}",
            self.start(),
            self.matched(),
            self.stop(),
            self.rxpend(),
            self.txsend(),
            self.dachg(),
            self.ccc(),
            self.errwarn(),
            self.ddrmatched(),
            self.chandled(),
            self.event()
        )
    }
}
#[doc = "Target Interrupt Mask"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sintmasked(pub u32);
impl Sintmasked {
    #[doc = "START Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "START Interrupt Mask"]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "MATCHED Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn matched(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "MATCHED Interrupt Mask"]
    #[inline(always)]
    pub const fn set_matched(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "STOP Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn stop(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "STOP Interrupt Mask"]
    #[inline(always)]
    pub const fn set_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "RXPEND Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpend(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "RXPEND Interrupt Mask"]
    #[inline(always)]
    pub const fn set_rxpend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "TXSEND Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn txsend(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "TXSEND Interrupt Mask"]
    #[inline(always)]
    pub const fn set_txsend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DACHG Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn dachg(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "DACHG Interrupt Mask"]
    #[inline(always)]
    pub const fn set_dachg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "CCC Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn ccc(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "CCC Interrupt Mask"]
    #[inline(always)]
    pub const fn set_ccc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "ERRWARN Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn errwarn(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "ERRWARN Interrupt Mask"]
    #[inline(always)]
    pub const fn set_errwarn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "DDRMATCHED Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn ddrmatched(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "DDRMATCHED Interrupt Mask"]
    #[inline(always)]
    pub const fn set_ddrmatched(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "CHANDLED Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn chandled(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "CHANDLED Interrupt Mask"]
    #[inline(always)]
    pub const fn set_chandled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "EVENT Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn event(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "EVENT Interrupt Mask"]
    #[inline(always)]
    pub const fn set_event(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for Sintmasked {
    #[inline(always)]
    fn default() -> Sintmasked {
        Sintmasked(0)
    }
}
impl core::fmt::Debug for Sintmasked {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sintmasked")
            .field("start", &self.start())
            .field("matched", &self.matched())
            .field("stop", &self.stop())
            .field("rxpend", &self.rxpend())
            .field("txsend", &self.txsend())
            .field("dachg", &self.dachg())
            .field("ccc", &self.ccc())
            .field("errwarn", &self.errwarn())
            .field("ddrmatched", &self.ddrmatched())
            .field("chandled", &self.chandled())
            .field("event", &self.event())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sintmasked {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sintmasked {{ start: {=bool:?}, matched: {=bool:?}, stop: {=bool:?}, rxpend: {=bool:?}, txsend: {=bool:?}, dachg: {=bool:?}, ccc: {=bool:?}, errwarn: {=bool:?}, ddrmatched: {=bool:?}, chandled: {=bool:?}, event: {=bool:?} }}",
            self.start(),
            self.matched(),
            self.stop(),
            self.rxpend(),
            self.txsend(),
            self.dachg(),
            self.ccc(),
            self.errwarn(),
            self.ddrmatched(),
            self.chandled(),
            self.event()
        )
    }
}
#[doc = "Target Interrupt Set"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sintset(pub u32);
impl Sintset {
    #[doc = "Start Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Start Interrupt Enable"]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Match Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn matched(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Match Interrupt Enable"]
    #[inline(always)]
    pub const fn set_matched(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Stop Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn stop(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Stop Interrupt Enable"]
    #[inline(always)]
    pub const fn set_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Receive Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpend(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Interrupt Enable"]
    #[inline(always)]
    pub const fn set_rxpend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Transmit Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn txsend(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Interrupt Enable"]
    #[inline(always)]
    pub const fn set_txsend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Dynamic Address Change Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dachg(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Dynamic Address Change Interrupt Enable"]
    #[inline(always)]
    pub const fn set_dachg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Common Command Code (CCC) Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ccc(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Common Command Code (CCC) Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ccc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Error or Warning Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn errwarn(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Error or Warning Interrupt Enable"]
    #[inline(always)]
    pub const fn set_errwarn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Double Data Rate Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ddrmatched(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Double Data Rate Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ddrmatched(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Common Command Code (CCC) Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn chandled(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Common Command Code (CCC) Interrupt Enable"]
    #[inline(always)]
    pub const fn set_chandled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Event Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn event(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Event Interrupt Enable"]
    #[inline(always)]
    pub const fn set_event(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for Sintset {
    #[inline(always)]
    fn default() -> Sintset {
        Sintset(0)
    }
}
impl core::fmt::Debug for Sintset {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sintset")
            .field("start", &self.start())
            .field("matched", &self.matched())
            .field("stop", &self.stop())
            .field("rxpend", &self.rxpend())
            .field("txsend", &self.txsend())
            .field("dachg", &self.dachg())
            .field("ccc", &self.ccc())
            .field("errwarn", &self.errwarn())
            .field("ddrmatched", &self.ddrmatched())
            .field("chandled", &self.chandled())
            .field("event", &self.event())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sintset {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sintset {{ start: {=bool:?}, matched: {=bool:?}, stop: {=bool:?}, rxpend: {=bool:?}, txsend: {=bool:?}, dachg: {=bool:?}, ccc: {=bool:?}, errwarn: {=bool:?}, ddrmatched: {=bool:?}, chandled: {=bool:?}, event: {=bool:?} }}",
            self.start(),
            self.matched(),
            self.stop(),
            self.rxpend(),
            self.txsend(),
            self.dachg(),
            self.ccc(),
            self.errwarn(),
            self.ddrmatched(),
            self.chandled(),
            self.event()
        )
    }
}
#[doc = "Map Feature Control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smapctrl0(pub u32);
impl Smapctrl0 {
    #[doc = "Enable Primary Dynamic Address"]
    #[must_use]
    #[inline(always)]
    pub const fn ena(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Primary Dynamic Address"]
    #[inline(always)]
    pub const fn set_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Dynamic Address"]
    #[must_use]
    #[inline(always)]
    pub const fn da(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Dynamic Address"]
    #[inline(always)]
    pub const fn set_da(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
    #[doc = "Cause"]
    #[must_use]
    #[inline(always)]
    pub const fn cause(&self) -> super::vals::Cause {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Cause::from_bits(val as u8)
    }
    #[doc = "Cause"]
    #[inline(always)]
    pub const fn set_cause(&mut self, val: super::vals::Cause) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
}
impl Default for Smapctrl0 {
    #[inline(always)]
    fn default() -> Smapctrl0 {
        Smapctrl0(0)
    }
}
impl core::fmt::Debug for Smapctrl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smapctrl0")
            .field("ena", &self.ena())
            .field("da", &self.da())
            .field("cause", &self.cause())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smapctrl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Smapctrl0 {{ ena: {=bool:?}, da: {=u8:?}, cause: {:?} }}",
            self.ena(),
            self.da(),
            self.cause()
        )
    }
}
#[doc = "Target Maximum Limits"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smaxlimits(pub u32);
impl Smaxlimits {
    #[doc = "Maximum Read Length"]
    #[must_use]
    #[inline(always)]
    pub const fn maxrd(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Maximum Read Length"]
    #[inline(always)]
    pub const fn set_maxrd(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Maximum Write Length"]
    #[must_use]
    #[inline(always)]
    pub const fn maxwr(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Maximum Write Length"]
    #[inline(always)]
    pub const fn set_maxwr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Smaxlimits {
    #[inline(always)]
    fn default() -> Smaxlimits {
        Smaxlimits(0)
    }
}
impl core::fmt::Debug for Smaxlimits {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smaxlimits")
            .field("maxrd", &self.maxrd())
            .field("maxwr", &self.maxwr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smaxlimits {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Smaxlimits {{ maxrd: {=u16:?}, maxwr: {=u16:?} }}",
            self.maxrd(),
            self.maxwr()
        )
    }
}
#[doc = "Target Message Map Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smsgmapaddr(pub u32);
impl Smsgmapaddr {
    #[doc = "Matched Address Index"]
    #[must_use]
    #[inline(always)]
    pub const fn maplast(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Matched Address Index"]
    #[inline(always)]
    pub const fn set_maplast(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Last Static Address Matched"]
    #[must_use]
    #[inline(always)]
    pub const fn laststatic(&self) -> super::vals::Laststatic {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Laststatic::from_bits(val as u8)
    }
    #[doc = "Last Static Address Matched"]
    #[inline(always)]
    pub const fn set_laststatic(&mut self, val: super::vals::Laststatic) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Matched Previous Address Index 1"]
    #[must_use]
    #[inline(always)]
    pub const fn maplastm1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Matched Previous Address Index 1"]
    #[inline(always)]
    pub const fn set_maplastm1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Matched Previous Index 2"]
    #[must_use]
    #[inline(always)]
    pub const fn maplastm2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Matched Previous Index 2"]
    #[inline(always)]
    pub const fn set_maplastm2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Smsgmapaddr {
    #[inline(always)]
    fn default() -> Smsgmapaddr {
        Smsgmapaddr(0)
    }
}
impl core::fmt::Debug for Smsgmapaddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smsgmapaddr")
            .field("maplast", &self.maplast())
            .field("laststatic", &self.laststatic())
            .field("maplastm1", &self.maplastm1())
            .field("maplastm2", &self.maplastm2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smsgmapaddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Smsgmapaddr {{ maplast: {=u8:?}, laststatic: {:?}, maplastm1: {=u8:?}, maplastm2: {=u8:?} }}",
            self.maplast(),
            self.laststatic(),
            self.maplastm1(),
            self.maplastm2()
        )
    }
}
#[doc = "Target Read Data Byte"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srdatab(pub u32);
impl Srdatab {
    #[doc = "Data 0"]
    #[must_use]
    #[inline(always)]
    pub const fn data0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data 0"]
    #[inline(always)]
    pub const fn set_data0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Srdatab {
    #[inline(always)]
    fn default() -> Srdatab {
        Srdatab(0)
    }
}
impl core::fmt::Debug for Srdatab {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Srdatab")
            .field("data0", &self.data0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Srdatab {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Srdatab {{ data0: {=u8:?} }}", self.data0())
    }
}
#[doc = "Target Read Data Halfword"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srdatah(pub u32);
impl Srdatah {
    #[doc = "Low Byte"]
    #[must_use]
    #[inline(always)]
    pub const fn lsb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Low Byte"]
    #[inline(always)]
    pub const fn set_lsb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "High Byte"]
    #[must_use]
    #[inline(always)]
    pub const fn msb(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "High Byte"]
    #[inline(always)]
    pub const fn set_msb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Srdatah {
    #[inline(always)]
    fn default() -> Srdatah {
        Srdatah(0)
    }
}
impl core::fmt::Debug for Srdatah {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Srdatah")
            .field("lsb", &self.lsb())
            .field("msb", &self.msb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Srdatah {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Srdatah {{ lsb: {=u8:?}, msb: {=u8:?} }}",
            self.lsb(),
            self.msb()
        )
    }
}
#[doc = "Target Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sstatus(pub u32);
impl Sstatus {
    #[doc = "Status not Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn stnotstop(&self) -> super::vals::Stnotstop {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Stnotstop::from_bits(val as u8)
    }
    #[doc = "Status not Stop"]
    #[inline(always)]
    pub const fn set_stnotstop(&mut self, val: super::vals::Stnotstop) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Status Message"]
    #[must_use]
    #[inline(always)]
    pub const fn stmsg(&self) -> super::vals::Stmsg {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Stmsg::from_bits(val as u8)
    }
    #[doc = "Status Message"]
    #[inline(always)]
    pub const fn set_stmsg(&mut self, val: super::vals::Stmsg) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Status Common Command Code Handler"]
    #[must_use]
    #[inline(always)]
    pub const fn stccch(&self) -> super::vals::Stccch {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Stccch::from_bits(val as u8)
    }
    #[doc = "Status Common Command Code Handler"]
    #[inline(always)]
    pub const fn set_stccch(&mut self, val: super::vals::Stccch) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Status Request Read"]
    #[must_use]
    #[inline(always)]
    pub const fn streqrd(&self) -> super::vals::Streqrd {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Streqrd::from_bits(val as u8)
    }
    #[doc = "Status Request Read"]
    #[inline(always)]
    pub const fn set_streqrd(&mut self, val: super::vals::Streqrd) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Status Request Write"]
    #[must_use]
    #[inline(always)]
    pub const fn streqwr(&self) -> super::vals::Streqwr {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Streqwr::from_bits(val as u8)
    }
    #[doc = "Status Request Write"]
    #[inline(always)]
    pub const fn set_streqwr(&mut self, val: super::vals::Streqwr) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Status Dynamic Address Assignment"]
    #[must_use]
    #[inline(always)]
    pub const fn stdaa(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Status Dynamic Address Assignment"]
    #[inline(always)]
    pub const fn set_stdaa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Status High Data Rate"]
    #[must_use]
    #[inline(always)]
    pub const fn sthdr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Status High Data Rate"]
    #[inline(always)]
    pub const fn set_sthdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Start"]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> super::vals::SstatusStart {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::SstatusStart::from_bits(val as u8)
    }
    #[doc = "Start"]
    #[inline(always)]
    pub const fn set_start(&mut self, val: super::vals::SstatusStart) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Matched"]
    #[must_use]
    #[inline(always)]
    pub const fn matched(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Matched"]
    #[inline(always)]
    pub const fn set_matched(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn stop(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Stop"]
    #[inline(always)]
    pub const fn set_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Received Message Pending"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_pend(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Received Message Pending"]
    #[inline(always)]
    pub const fn set_rx_pend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Transmit Buffer Not Full"]
    #[must_use]
    #[inline(always)]
    pub const fn txnotfull(&self) -> super::vals::SstatusTxnotfull {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::SstatusTxnotfull::from_bits(val as u8)
    }
    #[doc = "Transmit Buffer Not Full"]
    #[inline(always)]
    pub const fn set_txnotfull(&mut self, val: super::vals::SstatusTxnotfull) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Dynamic Address Change"]
    #[must_use]
    #[inline(always)]
    pub const fn dachg(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Dynamic Address Change"]
    #[inline(always)]
    pub const fn set_dachg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Common Command Code"]
    #[must_use]
    #[inline(always)]
    pub const fn ccc(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Common Command Code"]
    #[inline(always)]
    pub const fn set_ccc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Error Warning"]
    #[must_use]
    #[inline(always)]
    pub const fn errwarn(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Error Warning"]
    #[inline(always)]
    pub const fn set_errwarn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "High Data Rate Command Match"]
    #[must_use]
    #[inline(always)]
    pub const fn hdrmatch(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "High Data Rate Command Match"]
    #[inline(always)]
    pub const fn set_hdrmatch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Common Command Code Handled"]
    #[must_use]
    #[inline(always)]
    pub const fn chandled(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Common Command Code Handled"]
    #[inline(always)]
    pub const fn set_chandled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Event"]
    #[must_use]
    #[inline(always)]
    pub const fn event(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Event"]
    #[inline(always)]
    pub const fn set_event(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Event Details"]
    #[must_use]
    #[inline(always)]
    pub const fn evdet(&self) -> super::vals::Evdet {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Evdet::from_bits(val as u8)
    }
    #[doc = "Event Details"]
    #[inline(always)]
    pub const fn set_evdet(&mut self, val: super::vals::Evdet) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "In-Band Interrupts Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn ibidis(&self) -> super::vals::Ibidis {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Ibidis::from_bits(val as u8)
    }
    #[doc = "In-Band Interrupts Disable"]
    #[inline(always)]
    pub const fn set_ibidis(&mut self, val: super::vals::Ibidis) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Controller Requests Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn mrdis(&self) -> super::vals::Mrdis {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Mrdis::from_bits(val as u8)
    }
    #[doc = "Controller Requests Disable"]
    #[inline(always)]
    pub const fn set_mrdis(&mut self, val: super::vals::Mrdis) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Hot-Join Disabled"]
    #[must_use]
    #[inline(always)]
    pub const fn hjdis(&self) -> super::vals::Hjdis {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Hjdis::from_bits(val as u8)
    }
    #[doc = "Hot-Join Disabled"]
    #[inline(always)]
    pub const fn set_hjdis(&mut self, val: super::vals::Hjdis) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Activity State from Common Command Codes (CCC)"]
    #[must_use]
    #[inline(always)]
    pub const fn actstate(&self) -> super::vals::Actstate {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Actstate::from_bits(val as u8)
    }
    #[doc = "Activity State from Common Command Codes (CCC)"]
    #[inline(always)]
    pub const fn set_actstate(&mut self, val: super::vals::Actstate) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Time Control"]
    #[must_use]
    #[inline(always)]
    pub const fn timectrl(&self) -> super::vals::SstatusTimectrl {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::SstatusTimectrl::from_bits(val as u8)
    }
    #[doc = "Time Control"]
    #[inline(always)]
    pub const fn set_timectrl(&mut self, val: super::vals::SstatusTimectrl) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for Sstatus {
    #[inline(always)]
    fn default() -> Sstatus {
        Sstatus(0)
    }
}
impl core::fmt::Debug for Sstatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sstatus")
            .field("stnotstop", &self.stnotstop())
            .field("stmsg", &self.stmsg())
            .field("stccch", &self.stccch())
            .field("streqrd", &self.streqrd())
            .field("streqwr", &self.streqwr())
            .field("stdaa", &self.stdaa())
            .field("sthdr", &self.sthdr())
            .field("start", &self.start())
            .field("matched", &self.matched())
            .field("stop", &self.stop())
            .field("rx_pend", &self.rx_pend())
            .field("txnotfull", &self.txnotfull())
            .field("dachg", &self.dachg())
            .field("ccc", &self.ccc())
            .field("errwarn", &self.errwarn())
            .field("hdrmatch", &self.hdrmatch())
            .field("chandled", &self.chandled())
            .field("event", &self.event())
            .field("evdet", &self.evdet())
            .field("ibidis", &self.ibidis())
            .field("mrdis", &self.mrdis())
            .field("hjdis", &self.hjdis())
            .field("actstate", &self.actstate())
            .field("timectrl", &self.timectrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sstatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sstatus {{ stnotstop: {:?}, stmsg: {:?}, stccch: {:?}, streqrd: {:?}, streqwr: {:?}, stdaa: {=bool:?}, sthdr: {=bool:?}, start: {:?}, matched: {=bool:?}, stop: {=bool:?}, rx_pend: {=bool:?}, txnotfull: {:?}, dachg: {=bool:?}, ccc: {=bool:?}, errwarn: {=bool:?}, hdrmatch: {=bool:?}, chandled: {=bool:?}, event: {=bool:?}, evdet: {:?}, ibidis: {:?}, mrdis: {:?}, hjdis: {:?}, actstate: {:?}, timectrl: {:?} }}",
            self.stnotstop(),
            self.stmsg(),
            self.stccch(),
            self.streqrd(),
            self.streqwr(),
            self.stdaa(),
            self.sthdr(),
            self.start(),
            self.matched(),
            self.stop(),
            self.rx_pend(),
            self.txnotfull(),
            self.dachg(),
            self.ccc(),
            self.errwarn(),
            self.hdrmatch(),
            self.chandled(),
            self.event(),
            self.evdet(),
            self.ibidis(),
            self.mrdis(),
            self.hjdis(),
            self.actstate(),
            self.timectrl()
        )
    }
}
#[doc = "Target Time Control Clock"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stcclock(pub u32);
impl Stcclock {
    #[doc = "Clock Accuracy"]
    #[must_use]
    #[inline(always)]
    pub const fn accuracy(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Accuracy"]
    #[inline(always)]
    pub const fn set_accuracy(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Clock Frequency"]
    #[must_use]
    #[inline(always)]
    pub const fn freq(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Frequency"]
    #[inline(always)]
    pub const fn set_freq(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Stcclock {
    #[inline(always)]
    fn default() -> Stcclock {
        Stcclock(0)
    }
}
impl core::fmt::Debug for Stcclock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stcclock")
            .field("accuracy", &self.accuracy())
            .field("freq", &self.freq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stcclock {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Stcclock {{ accuracy: {=u8:?}, freq: {=u8:?} }}",
            self.accuracy(),
            self.freq()
        )
    }
}
#[doc = "Target Vendor ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Svendorid(pub u32);
impl Svendorid {
    #[doc = "Vendor ID"]
    #[must_use]
    #[inline(always)]
    pub const fn vid(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Vendor ID"]
    #[inline(always)]
    pub const fn set_vid(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
}
impl Default for Svendorid {
    #[inline(always)]
    fn default() -> Svendorid {
        Svendorid(0)
    }
}
impl core::fmt::Debug for Svendorid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Svendorid")
            .field("vid", &self.vid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Svendorid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Svendorid {{ vid: {=u16:?} }}", self.vid())
    }
}
#[doc = "Target Write Data Byte"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swdatab(pub u32);
impl Swdatab {
    #[doc = "Data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "End"]
    #[must_use]
    #[inline(always)]
    pub const fn end(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "End"]
    #[inline(always)]
    pub const fn set_end(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "End Also"]
    #[must_use]
    #[inline(always)]
    pub const fn end_also(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "End Also"]
    #[inline(always)]
    pub const fn set_end_also(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Swdatab {
    #[inline(always)]
    fn default() -> Swdatab {
        Swdatab(0)
    }
}
impl core::fmt::Debug for Swdatab {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Swdatab")
            .field("data", &self.data())
            .field("end", &self.end())
            .field("end_also", &self.end_also())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swdatab {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Swdatab {{ data: {=u8:?}, end: {=bool:?}, end_also: {=bool:?} }}",
            self.data(),
            self.end(),
            self.end_also()
        )
    }
}
#[doc = "Target Write Data Byte"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swdatab1(pub u32);
impl Swdatab1 {
    #[doc = "Data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Swdatab1 {
    #[inline(always)]
    fn default() -> Swdatab1 {
        Swdatab1(0)
    }
}
impl core::fmt::Debug for Swdatab1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Swdatab1")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swdatab1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Swdatab1 {{ data: {=u8:?} }}", self.data())
    }
}
#[doc = "Target Write Data Byte End"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swdatabe(pub u32);
impl Swdatabe {
    #[doc = "Data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Swdatabe {
    #[inline(always)]
    fn default() -> Swdatabe {
        Swdatabe(0)
    }
}
impl core::fmt::Debug for Swdatabe {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Swdatabe")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swdatabe {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Swdatabe {{ data: {=u8:?} }}", self.data())
    }
}
#[doc = "Target Write Data Halfword"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swdatah(pub u32);
impl Swdatah {
    #[doc = "Data 0"]
    #[must_use]
    #[inline(always)]
    pub const fn data0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data 0"]
    #[inline(always)]
    pub const fn set_data0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data 1"]
    #[must_use]
    #[inline(always)]
    pub const fn data1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data 1"]
    #[inline(always)]
    pub const fn set_data1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "End of Message"]
    #[must_use]
    #[inline(always)]
    pub const fn end(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "End of Message"]
    #[inline(always)]
    pub const fn set_end(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Swdatah {
    #[inline(always)]
    fn default() -> Swdatah {
        Swdatah(0)
    }
}
impl core::fmt::Debug for Swdatah {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Swdatah")
            .field("data0", &self.data0())
            .field("data1", &self.data1())
            .field("end", &self.end())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swdatah {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Swdatah {{ data0: {=u8:?}, data1: {=u8:?}, end: {=bool:?} }}",
            self.data0(),
            self.data1(),
            self.end()
        )
    }
}
#[doc = "Target Write Data Halfword End"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swdatahe(pub u32);
impl Swdatahe {
    #[doc = "Data 0"]
    #[must_use]
    #[inline(always)]
    pub const fn data0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data 0"]
    #[inline(always)]
    pub const fn set_data0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data 1"]
    #[must_use]
    #[inline(always)]
    pub const fn data1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data 1"]
    #[inline(always)]
    pub const fn set_data1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Swdatahe {
    #[inline(always)]
    fn default() -> Swdatahe {
        Swdatahe(0)
    }
}
impl core::fmt::Debug for Swdatahe {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Swdatahe")
            .field("data0", &self.data0())
            .field("data1", &self.data1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swdatahe {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Swdatahe {{ data0: {=u8:?}, data1: {=u8:?} }}",
            self.data0(),
            self.data1()
        )
    }
}
