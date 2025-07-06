#[doc = "Bus (AXI) Master Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bmcr0(pub u32);
impl Bmcr0 {
    #[doc = "Weight of QOS"]
    #[must_use]
    #[inline(always)]
    pub const fn wqos(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Weight of QOS"]
    #[inline(always)]
    pub const fn set_wqos(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Weight of AGE"]
    #[must_use]
    #[inline(always)]
    pub const fn wage(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Weight of AGE"]
    #[inline(always)]
    pub const fn set_wage(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Weight of Slave Hit without read/write switch"]
    #[must_use]
    #[inline(always)]
    pub const fn wsh(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Weight of Slave Hit without read/write switch"]
    #[inline(always)]
    pub const fn set_wsh(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Weight of slave hit with Read/Write Switch"]
    #[must_use]
    #[inline(always)]
    pub const fn wrws(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Weight of slave hit with Read/Write Switch"]
    #[inline(always)]
    pub const fn set_wrws(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Bmcr0 {
    #[inline(always)]
    fn default() -> Bmcr0 {
        Bmcr0(0)
    }
}
impl core::fmt::Debug for Bmcr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bmcr0")
            .field("wqos", &self.wqos())
            .field("wage", &self.wage())
            .field("wsh", &self.wsh())
            .field("wrws", &self.wrws())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bmcr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bmcr0 {{ wqos: {=u8:?}, wage: {=u8:?}, wsh: {=u8:?}, wrws: {=u8:?} }}",
            self.wqos(),
            self.wage(),
            self.wsh(),
            self.wrws()
        )
    }
}
#[doc = "Bus (AXI) Master Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bmcr1(pub u32);
impl Bmcr1 {
    #[doc = "Weight of QOS"]
    #[must_use]
    #[inline(always)]
    pub const fn wqos(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Weight of QOS"]
    #[inline(always)]
    pub const fn set_wqos(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Weight of AGE"]
    #[must_use]
    #[inline(always)]
    pub const fn wage(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Weight of AGE"]
    #[inline(always)]
    pub const fn set_wage(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Weight of Page Hit"]
    #[must_use]
    #[inline(always)]
    pub const fn wph(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Weight of Page Hit"]
    #[inline(always)]
    pub const fn set_wph(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Weight of slave hit without Read/Write Switch"]
    #[must_use]
    #[inline(always)]
    pub const fn wrws(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Weight of slave hit without Read/Write Switch"]
    #[inline(always)]
    pub const fn set_wrws(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Weight of Bank Rotation"]
    #[must_use]
    #[inline(always)]
    pub const fn wbr(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Weight of Bank Rotation"]
    #[inline(always)]
    pub const fn set_wbr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Bmcr1 {
    #[inline(always)]
    fn default() -> Bmcr1 {
        Bmcr1(0)
    }
}
impl core::fmt::Debug for Bmcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bmcr1")
            .field("wqos", &self.wqos())
            .field("wage", &self.wage())
            .field("wph", &self.wph())
            .field("wrws", &self.wrws())
            .field("wbr", &self.wbr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bmcr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bmcr1 {{ wqos: {=u8:?}, wage: {=u8:?}, wph: {=u8:?}, wrws: {=u8:?}, wbr: {=u8:?} }}",
            self.wqos(),
            self.wage(),
            self.wph(),
            self.wrws(),
            self.wbr()
        )
    }
}
#[doc = "Base Register n"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Br(pub u32);
impl Br {
    #[doc = "Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn vld(&self) -> super::vals::Vld {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Vld::from_bits(val as u8)
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub const fn set_vld(&mut self, val: super::vals::Vld) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Memory size"]
    #[must_use]
    #[inline(always)]
    pub const fn ms(&self) -> super::vals::Ms {
        let val = (self.0 >> 1usize) & 0x1f;
        super::vals::Ms::from_bits(val as u8)
    }
    #[doc = "Memory size"]
    #[inline(always)]
    pub const fn set_ms(&mut self, val: super::vals::Ms) {
        self.0 = (self.0 & !(0x1f << 1usize)) | (((val.to_bits() as u32) & 0x1f) << 1usize);
    }
    #[doc = "Base Address"]
    #[must_use]
    #[inline(always)]
    pub const fn ba(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Base Address"]
    #[inline(always)]
    pub const fn set_ba(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Br {
    #[inline(always)]
    fn default() -> Br {
        Br(0)
    }
}
impl core::fmt::Debug for Br {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Br")
            .field("vld", &self.vld())
            .field("ms", &self.ms())
            .field("ba", &self.ba())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Br {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Br {{ vld: {:?}, ms: {:?}, ba: {=u32:?} }}",
            self.vld(),
            self.ms(),
            self.ba()
        )
    }
}
#[doc = "DBI-B Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbicr0(pub u32);
impl Dbicr0 {
    #[doc = "Port Size"]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> super::vals::Dbicr0Ps {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Dbicr0Ps::from_bits(val as u8)
    }
    #[doc = "Port Size"]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: super::vals::Dbicr0Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Burst Length"]
    #[must_use]
    #[inline(always)]
    pub const fn bl(&self) -> super::vals::Dbicr0Bl {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Dbicr0Bl::from_bits(val as u8)
    }
    #[doc = "Burst Length"]
    #[inline(always)]
    pub const fn set_bl(&mut self, val: super::vals::Dbicr0Bl) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Column Address bit width"]
    #[must_use]
    #[inline(always)]
    pub const fn col(&self) -> super::vals::Dbicr0Col {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Dbicr0Col::from_bits(val as u8)
    }
    #[doc = "Column Address bit width"]
    #[inline(always)]
    pub const fn set_col(&mut self, val: super::vals::Dbicr0Col) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
}
impl Default for Dbicr0 {
    #[inline(always)]
    fn default() -> Dbicr0 {
        Dbicr0(0)
    }
}
impl core::fmt::Debug for Dbicr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dbicr0")
            .field("ps", &self.ps())
            .field("bl", &self.bl())
            .field("col", &self.col())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dbicr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dbicr0 {{ ps: {:?}, bl: {:?}, col: {:?} }}",
            self.ps(),
            self.bl(),
            self.col()
        )
    }
}
#[doc = "DBI-B Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbicr1(pub u32);
impl Dbicr1 {
    #[doc = "CSX Setup Time"]
    #[must_use]
    #[inline(always)]
    pub const fn ces(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "CSX Setup Time"]
    #[inline(always)]
    pub const fn set_ces(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "CSX Hold Time"]
    #[must_use]
    #[inline(always)]
    pub const fn ceh(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "CSX Hold Time"]
    #[inline(always)]
    pub const fn set_ceh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "WRX Low Time"]
    #[must_use]
    #[inline(always)]
    pub const fn wel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "WRX Low Time"]
    #[inline(always)]
    pub const fn set_wel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "WRX High Time"]
    #[must_use]
    #[inline(always)]
    pub const fn weh(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "WRX High Time"]
    #[inline(always)]
    pub const fn set_weh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "RDX Low Time bit \\[3:0\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn rel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "RDX Low Time bit \\[3:0\\]"]
    #[inline(always)]
    pub const fn set_rel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "RDX High Time bit \\[3:0\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn reh(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "RDX High Time bit \\[3:0\\]"]
    #[inline(always)]
    pub const fn set_reh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "CSX interval time"]
    #[must_use]
    #[inline(always)]
    pub const fn ceitv(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "CSX interval time"]
    #[inline(always)]
    pub const fn set_ceitv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "RDX Low Time bit \\[5:4\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn rel2(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "RDX Low Time bit \\[5:4\\]"]
    #[inline(always)]
    pub const fn set_rel2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "RDX High Time bit \\[5:4\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn reh2(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "RDX High Time bit \\[5:4\\]"]
    #[inline(always)]
    pub const fn set_reh2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Dbicr1 {
    #[inline(always)]
    fn default() -> Dbicr1 {
        Dbicr1(0)
    }
}
impl core::fmt::Debug for Dbicr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dbicr1")
            .field("ces", &self.ces())
            .field("ceh", &self.ceh())
            .field("wel", &self.wel())
            .field("weh", &self.weh())
            .field("rel", &self.rel())
            .field("reh", &self.reh())
            .field("ceitv", &self.ceitv())
            .field("rel2", &self.rel2())
            .field("reh2", &self.reh2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dbicr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dbicr1 {{ ces: {=u8:?}, ceh: {=u8:?}, wel: {=u8:?}, weh: {=u8:?}, rel: {=u8:?}, reh: {=u8:?}, ceitv: {=u8:?}, rel2: {=u8:?}, reh2: {=u8:?} }}",
            self.ces(),
            self.ceh(),
            self.wel(),
            self.weh(),
            self.rel(),
            self.reh(),
            self.ceitv(),
            self.rel2(),
            self.reh2()
        )
    }
}
#[doc = "DLL Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dllcr(pub u32);
impl Dllcr {
    #[doc = "DLL calibration enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dllen(&self) -> super::vals::Dllen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Dllen::from_bits(val as u8)
    }
    #[doc = "DLL calibration enable"]
    #[inline(always)]
    pub const fn set_dllen(&mut self, val: super::vals::Dllen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "DLL Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn dllreset(&self) -> super::vals::Dllreset {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Dllreset::from_bits(val as u8)
    }
    #[doc = "DLL Reset"]
    #[inline(always)]
    pub const fn set_dllreset(&mut self, val: super::vals::Dllreset) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Delay Target for Slave"]
    #[must_use]
    #[inline(always)]
    pub const fn slvdlytarget(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x0f;
        val as u8
    }
    #[doc = "Delay Target for Slave"]
    #[inline(always)]
    pub const fn set_slvdlytarget(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
    }
    #[doc = "Override Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ovrden(&self) -> super::vals::Ovrden {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Ovrden::from_bits(val as u8)
    }
    #[doc = "Override Enable"]
    #[inline(always)]
    pub const fn set_ovrden(&mut self, val: super::vals::Ovrden) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Override Value"]
    #[must_use]
    #[inline(always)]
    pub const fn ovrdval(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x3f;
        val as u8
    }
    #[doc = "Override Value"]
    #[inline(always)]
    pub const fn set_ovrdval(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 9usize)) | (((val as u32) & 0x3f) << 9usize);
    }
}
impl Default for Dllcr {
    #[inline(always)]
    fn default() -> Dllcr {
        Dllcr(0)
    }
}
impl core::fmt::Debug for Dllcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dllcr")
            .field("dllen", &self.dllen())
            .field("dllreset", &self.dllreset())
            .field("slvdlytarget", &self.slvdlytarget())
            .field("ovrden", &self.ovrden())
            .field("ovrdval", &self.ovrdval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dllcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dllcr {{ dllen: {:?}, dllreset: {:?}, slvdlytarget: {=u8:?}, ovrden: {:?}, ovrdval: {=u8:?} }}",
            self.dllen(),
            self.dllreset(),
            self.slvdlytarget(),
            self.ovrden(),
            self.ovrdval()
        )
    }
}
#[doc = "Interrupt Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "IP command done interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmddoneen(&self) -> super::vals::Ipcmddoneen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ipcmddoneen::from_bits(val as u8)
    }
    #[doc = "IP command done interrupt enable"]
    #[inline(always)]
    pub const fn set_ipcmddoneen(&mut self, val: super::vals::Ipcmddoneen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "IP command error interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmderren(&self) -> super::vals::Ipcmderren {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ipcmderren::from_bits(val as u8)
    }
    #[doc = "IP command error interrupt enable"]
    #[inline(always)]
    pub const fn set_ipcmderren(&mut self, val: super::vals::Ipcmderren) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "AXI command error interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn axicmderren(&self) -> super::vals::Axicmderren {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Axicmderren::from_bits(val as u8)
    }
    #[doc = "AXI command error interrupt enable"]
    #[inline(always)]
    pub const fn set_axicmderren(&mut self, val: super::vals::Axicmderren) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "AXI bus error interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn axibuserren(&self) -> super::vals::Axibuserren {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Axibuserren::from_bits(val as u8)
    }
    #[doc = "AXI bus error interrupt enable"]
    #[inline(always)]
    pub const fn set_axibuserren(&mut self, val: super::vals::Axibuserren) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "NAND page end interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ndpageenden(&self) -> super::vals::Ndpageenden {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Ndpageenden::from_bits(val as u8)
    }
    #[doc = "NAND page end interrupt enable"]
    #[inline(always)]
    pub const fn set_ndpageenden(&mut self, val: super::vals::Ndpageenden) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "NAND no pending AXI access interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ndnopenden(&self) -> super::vals::Ndnopenden {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Ndnopenden::from_bits(val as u8)
    }
    #[doc = "NAND no pending AXI access interrupt enable"]
    #[inline(always)]
    pub const fn set_ndnopenden(&mut self, val: super::vals::Ndnopenden) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
impl core::fmt::Debug for Inten {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Inten")
            .field("ipcmddoneen", &self.ipcmddoneen())
            .field("ipcmderren", &self.ipcmderren())
            .field("axicmderren", &self.axicmderren())
            .field("axibuserren", &self.axibuserren())
            .field("ndpageenden", &self.ndpageenden())
            .field("ndnopenden", &self.ndnopenden())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Inten {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Inten {{ ipcmddoneen: {:?}, ipcmderren: {:?}, axicmderren: {:?}, axibuserren: {:?}, ndpageenden: {:?}, ndnopenden: {:?} }}",
            self.ipcmddoneen(),
            self.ipcmderren(),
            self.axicmderren(),
            self.axibuserren(),
            self.ndpageenden(),
            self.ndnopenden()
        )
    }
}
#[doc = "Interrupt Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intr(pub u32);
impl Intr {
    #[doc = "IP command normal done interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmddone(&self) -> super::vals::Ipcmddone {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ipcmddone::from_bits(val as u8)
    }
    #[doc = "IP command normal done interrupt"]
    #[inline(always)]
    pub const fn set_ipcmddone(&mut self, val: super::vals::Ipcmddone) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "IP command error done interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmderr(&self) -> super::vals::Ipcmderr {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ipcmderr::from_bits(val as u8)
    }
    #[doc = "IP command error done interrupt"]
    #[inline(always)]
    pub const fn set_ipcmderr(&mut self, val: super::vals::Ipcmderr) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "AXI command error interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn axicmderr(&self) -> super::vals::Axicmderr {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Axicmderr::from_bits(val as u8)
    }
    #[doc = "AXI command error interrupt"]
    #[inline(always)]
    pub const fn set_axicmderr(&mut self, val: super::vals::Axicmderr) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "AXI bus error interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn axibuserr(&self) -> super::vals::Axibuserr {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Axibuserr::from_bits(val as u8)
    }
    #[doc = "AXI bus error interrupt"]
    #[inline(always)]
    pub const fn set_axibuserr(&mut self, val: super::vals::Axibuserr) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "NAND page end interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn ndpageend(&self) -> super::vals::Ndpageend {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Ndpageend::from_bits(val as u8)
    }
    #[doc = "NAND page end interrupt"]
    #[inline(always)]
    pub const fn set_ndpageend(&mut self, val: super::vals::Ndpageend) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "NAND no pending AXI write transaction interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn ndnopend(&self) -> super::vals::Ndnopend {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Ndnopend::from_bits(val as u8)
    }
    #[doc = "NAND no pending AXI write transaction interrupt"]
    #[inline(always)]
    pub const fn set_ndnopend(&mut self, val: super::vals::Ndnopend) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
}
impl Default for Intr {
    #[inline(always)]
    fn default() -> Intr {
        Intr(0)
    }
}
impl core::fmt::Debug for Intr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intr")
            .field("ipcmddone", &self.ipcmddone())
            .field("ipcmderr", &self.ipcmderr())
            .field("axicmderr", &self.axicmderr())
            .field("axibuserr", &self.axibuserr())
            .field("ndpageend", &self.ndpageend())
            .field("ndnopend", &self.ndnopend())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intr {{ ipcmddone: {:?}, ipcmderr: {:?}, axicmderr: {:?}, axibuserr: {:?}, ndpageend: {:?}, ndnopend: {:?} }}",
            self.ipcmddone(),
            self.ipcmderr(),
            self.axicmderr(),
            self.axibuserr(),
            self.ndpageend(),
            self.ndnopend()
        )
    }
}
#[doc = "IO MUX Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iocr(pub u32);
impl Iocr {
    #[doc = "SEMC_ADDR08 output selection"]
    #[must_use]
    #[inline(always)]
    pub const fn mux_a8(&self) -> super::vals::MuxA8 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::MuxA8::from_bits(val as u8)
    }
    #[doc = "SEMC_ADDR08 output selection"]
    #[inline(always)]
    pub const fn set_mux_a8(&mut self, val: super::vals::MuxA8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "SEMC_CSX0 output selection"]
    #[must_use]
    #[inline(always)]
    pub const fn mux_csx0(&self) -> super::vals::MuxCsx0 {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::MuxCsx0::from_bits(val as u8)
    }
    #[doc = "SEMC_CSX0 output selection"]
    #[inline(always)]
    pub const fn set_mux_csx0(&mut self, val: super::vals::MuxCsx0) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
    }
    #[doc = "SEMC_CSX1 output selection"]
    #[must_use]
    #[inline(always)]
    pub const fn mux_csx1(&self) -> super::vals::MuxCsx1 {
        let val = (self.0 >> 6usize) & 0x07;
        super::vals::MuxCsx1::from_bits(val as u8)
    }
    #[doc = "SEMC_CSX1 output selection"]
    #[inline(always)]
    pub const fn set_mux_csx1(&mut self, val: super::vals::MuxCsx1) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val.to_bits() as u32) & 0x07) << 6usize);
    }
    #[doc = "SEMC_CSX2 output selection"]
    #[must_use]
    #[inline(always)]
    pub const fn mux_csx2(&self) -> super::vals::MuxCsx2 {
        let val = (self.0 >> 9usize) & 0x07;
        super::vals::MuxCsx2::from_bits(val as u8)
    }
    #[doc = "SEMC_CSX2 output selection"]
    #[inline(always)]
    pub const fn set_mux_csx2(&mut self, val: super::vals::MuxCsx2) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val.to_bits() as u32) & 0x07) << 9usize);
    }
    #[doc = "SEMC_CSX3 output selection"]
    #[must_use]
    #[inline(always)]
    pub const fn mux_csx3(&self) -> super::vals::MuxCsx3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::MuxCsx3::from_bits(val as u8)
    }
    #[doc = "SEMC_CSX3 output selection"]
    #[inline(always)]
    pub const fn set_mux_csx3(&mut self, val: super::vals::MuxCsx3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "SEMC_RDY function selection"]
    #[must_use]
    #[inline(always)]
    pub const fn mux_rdy(&self) -> super::vals::MuxRdy {
        let val = (self.0 >> 15usize) & 0x07;
        super::vals::MuxRdy::from_bits(val as u8)
    }
    #[doc = "SEMC_RDY function selection"]
    #[inline(always)]
    pub const fn set_mux_rdy(&mut self, val: super::vals::MuxRdy) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val.to_bits() as u32) & 0x07) << 15usize);
    }
    #[doc = "SEMC_CLKX0 function selection"]
    #[must_use]
    #[inline(always)]
    pub const fn mux_clkx0(&self) -> super::vals::MuxClkx0 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::MuxClkx0::from_bits(val as u8)
    }
    #[doc = "SEMC_CLKX0 function selection"]
    #[inline(always)]
    pub const fn set_mux_clkx0(&mut self, val: super::vals::MuxClkx0) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "SEMC_CLKX1 function selection"]
    #[must_use]
    #[inline(always)]
    pub const fn mux_clkx1(&self) -> super::vals::MuxClkx1 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::MuxClkx1::from_bits(val as u8)
    }
    #[doc = "SEMC_CLKX1 function selection"]
    #[inline(always)]
    pub const fn set_mux_clkx1(&mut self, val: super::vals::MuxClkx1) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
}
impl Default for Iocr {
    #[inline(always)]
    fn default() -> Iocr {
        Iocr(0)
    }
}
impl core::fmt::Debug for Iocr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iocr")
            .field("mux_a8", &self.mux_a8())
            .field("mux_csx0", &self.mux_csx0())
            .field("mux_csx1", &self.mux_csx1())
            .field("mux_csx2", &self.mux_csx2())
            .field("mux_csx3", &self.mux_csx3())
            .field("mux_rdy", &self.mux_rdy())
            .field("mux_clkx0", &self.mux_clkx0())
            .field("mux_clkx1", &self.mux_clkx1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iocr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Iocr {{ mux_a8: {:?}, mux_csx0: {:?}, mux_csx1: {:?}, mux_csx2: {:?}, mux_csx3: {:?}, mux_rdy: {:?}, mux_clkx0: {:?}, mux_clkx1: {:?} }}",
            self.mux_a8(),
            self.mux_csx0(),
            self.mux_csx1(),
            self.mux_csx2(),
            self.mux_csx3(),
            self.mux_rdy(),
            self.mux_clkx0(),
            self.mux_clkx1()
        )
    }
}
#[doc = "IP Command Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipcmd(pub u32);
impl Ipcmd {
    #[doc = "SDRAM Commands: 0x8: Read 0x9: Write 0xA: Mode Register Set 0xB: Active 0xC: Auto Refresh 0xD: Self Refresh 0xE: Precharge 0xF: Precharge All Others: Reserved Self Refresh is sent to all SDRAM devices because they share the same SEMC_CLK pin"]
    #[must_use]
    #[inline(always)]
    pub const fn cmd(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "SDRAM Commands: 0x8: Read 0x9: Write 0xA: Mode Register Set 0xB: Active 0xC: Auto Refresh 0xD: Self Refresh 0xE: Precharge 0xF: Precharge All Others: Reserved Self Refresh is sent to all SDRAM devices because they share the same SEMC_CLK pin"]
    #[inline(always)]
    pub const fn set_cmd(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "This field should be written with 0xA55A when trigging an IP command for all device types"]
    #[must_use]
    #[inline(always)]
    pub const fn key(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "This field should be written with 0xA55A when trigging an IP command for all device types"]
    #[inline(always)]
    pub const fn set_key(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Ipcmd {
    #[inline(always)]
    fn default() -> Ipcmd {
        Ipcmd(0)
    }
}
impl core::fmt::Debug for Ipcmd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipcmd")
            .field("cmd", &self.cmd())
            .field("key", &self.key())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipcmd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipcmd {{ cmd: {=u16:?}, key: {=u16:?} }}",
            self.cmd(),
            self.key()
        )
    }
}
#[doc = "IP Command Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipcr0(pub u32);
impl Ipcr0 {
    #[doc = "Slave address"]
    #[must_use]
    #[inline(always)]
    pub const fn sa(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Slave address"]
    #[inline(always)]
    pub const fn set_sa(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipcr0 {
    #[inline(always)]
    fn default() -> Ipcr0 {
        Ipcr0(0)
    }
}
impl core::fmt::Debug for Ipcr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipcr0").field("sa", &self.sa()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipcr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ipcr0 {{ sa: {=u32:?} }}", self.sa())
    }
}
#[doc = "IP Command Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipcr1(pub u32);
impl Ipcr1 {
    #[doc = "Data Size in Byte"]
    #[must_use]
    #[inline(always)]
    pub const fn datsz(&self) -> super::vals::Datsz {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Datsz::from_bits(val as u8)
    }
    #[doc = "Data Size in Byte"]
    #[inline(always)]
    pub const fn set_datsz(&mut self, val: super::vals::Datsz) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "NAND Extended Address"]
    #[must_use]
    #[inline(always)]
    pub const fn nand_ext_addr(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "NAND Extended Address"]
    #[inline(always)]
    pub const fn set_nand_ext_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Ipcr1 {
    #[inline(always)]
    fn default() -> Ipcr1 {
        Ipcr1(0)
    }
}
impl core::fmt::Debug for Ipcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipcr1")
            .field("datsz", &self.datsz())
            .field("nand_ext_addr", &self.nand_ext_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipcr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipcr1 {{ datsz: {:?}, nand_ext_addr: {=u8:?} }}",
            self.datsz(),
            self.nand_ext_addr()
        )
    }
}
#[doc = "IP Command Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipcr2(pub u32);
impl Ipcr2 {
    #[doc = "Byte Mask for Byte 0 (IPTXDAT bit 7:0)"]
    #[must_use]
    #[inline(always)]
    pub const fn bm0(&self) -> super::vals::Bm0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Bm0::from_bits(val as u8)
    }
    #[doc = "Byte Mask for Byte 0 (IPTXDAT bit 7:0)"]
    #[inline(always)]
    pub const fn set_bm0(&mut self, val: super::vals::Bm0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Byte Mask for Byte 1 (IPTXDAT bit 15:8)"]
    #[must_use]
    #[inline(always)]
    pub const fn bm1(&self) -> super::vals::Bm1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Bm1::from_bits(val as u8)
    }
    #[doc = "Byte Mask for Byte 1 (IPTXDAT bit 15:8)"]
    #[inline(always)]
    pub const fn set_bm1(&mut self, val: super::vals::Bm1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Byte Mask for Byte 2 (IPTXDAT bit 23:16)"]
    #[must_use]
    #[inline(always)]
    pub const fn bm2(&self) -> super::vals::Bm2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Bm2::from_bits(val as u8)
    }
    #[doc = "Byte Mask for Byte 2 (IPTXDAT bit 23:16)"]
    #[inline(always)]
    pub const fn set_bm2(&mut self, val: super::vals::Bm2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Byte Mask for Byte 3 (IPTXDAT bit 31:24)"]
    #[must_use]
    #[inline(always)]
    pub const fn bm3(&self) -> super::vals::Bm3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Bm3::from_bits(val as u8)
    }
    #[doc = "Byte Mask for Byte 3 (IPTXDAT bit 31:24)"]
    #[inline(always)]
    pub const fn set_bm3(&mut self, val: super::vals::Bm3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for Ipcr2 {
    #[inline(always)]
    fn default() -> Ipcr2 {
        Ipcr2(0)
    }
}
impl core::fmt::Debug for Ipcr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipcr2")
            .field("bm0", &self.bm0())
            .field("bm1", &self.bm1())
            .field("bm2", &self.bm2())
            .field("bm3", &self.bm3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipcr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipcr2 {{ bm0: {:?}, bm1: {:?}, bm2: {:?}, bm3: {:?} }}",
            self.bm0(),
            self.bm1(),
            self.bm2(),
            self.bm3()
        )
    }
}
#[doc = "RX DATA Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iprxdat(pub u32);
impl Iprxdat {
    #[doc = "Data returned by device for an IP read command."]
    #[must_use]
    #[inline(always)]
    pub const fn dat(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data returned by device for an IP read command."]
    #[inline(always)]
    pub const fn set_dat(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Iprxdat {
    #[inline(always)]
    fn default() -> Iprxdat {
        Iprxdat(0)
    }
}
impl core::fmt::Debug for Iprxdat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iprxdat").field("dat", &self.dat()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iprxdat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Iprxdat {{ dat: {=u32:?} }}", self.dat())
    }
}
#[doc = "TX DATA Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iptxdat(pub u32);
impl Iptxdat {
    #[doc = "Data value to use for an IP write command"]
    #[must_use]
    #[inline(always)]
    pub const fn dat(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data value to use for an IP write command"]
    #[inline(always)]
    pub const fn set_dat(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Iptxdat {
    #[inline(always)]
    fn default() -> Iptxdat {
        Iptxdat(0)
    }
}
impl core::fmt::Debug for Iptxdat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iptxdat").field("dat", &self.dat()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iptxdat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Iptxdat {{ dat: {=u32:?} }}", self.dat())
    }
}
#[doc = "Module Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcr(pub u32);
impl Mcr {
    #[doc = "Software Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn swrst(&self) -> super::vals::Swrst {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Swrst::from_bits(val as u8)
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn set_swrst(&mut self, val: super::vals::Swrst) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Module Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn mdis(&self) -> super::vals::Mdis {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Mdis::from_bits(val as u8)
    }
    #[doc = "Module Disable"]
    #[inline(always)]
    pub const fn set_mdis(&mut self, val: super::vals::Mdis) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "DQS (read strobe) mode"]
    #[must_use]
    #[inline(always)]
    pub const fn dqsmd(&self) -> super::vals::Dqsmd {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Dqsmd::from_bits(val as u8)
    }
    #[doc = "DQS (read strobe) mode"]
    #[inline(always)]
    pub const fn set_dqsmd(&mut self, val: super::vals::Dqsmd) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "WAIT/RDY polarity for SRAM/NOR"]
    #[must_use]
    #[inline(always)]
    pub const fn wpol0(&self) -> super::vals::Wpol0 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Wpol0::from_bits(val as u8)
    }
    #[doc = "WAIT/RDY polarity for SRAM/NOR"]
    #[inline(always)]
    pub const fn set_wpol0(&mut self, val: super::vals::Wpol0) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "R/B# polarity for NAND device"]
    #[must_use]
    #[inline(always)]
    pub const fn wpol1(&self) -> super::vals::Wpol1 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Wpol1::from_bits(val as u8)
    }
    #[doc = "R/B# polarity for NAND device"]
    #[inline(always)]
    pub const fn set_wpol1(&mut self, val: super::vals::Wpol1) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Command Execution timeout cycles"]
    #[must_use]
    #[inline(always)]
    pub const fn cto(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Command Execution timeout cycles"]
    #[inline(always)]
    pub const fn set_cto(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Bus timeout cycles"]
    #[must_use]
    #[inline(always)]
    pub const fn bto(&self) -> super::vals::Bto {
        let val = (self.0 >> 24usize) & 0x1f;
        super::vals::Bto::from_bits(val as u8)
    }
    #[doc = "Bus timeout cycles"]
    #[inline(always)]
    pub const fn set_bto(&mut self, val: super::vals::Bto) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val.to_bits() as u32) & 0x1f) << 24usize);
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
            .field("swrst", &self.swrst())
            .field("mdis", &self.mdis())
            .field("dqsmd", &self.dqsmd())
            .field("wpol0", &self.wpol0())
            .field("wpol1", &self.wpol1())
            .field("cto", &self.cto())
            .field("bto", &self.bto())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mcr {{ swrst: {:?}, mdis: {:?}, dqsmd: {:?}, wpol0: {:?}, wpol1: {:?}, cto: {=u8:?}, bto: {:?} }}",
            self.swrst(),
            self.mdis(),
            self.dqsmd(),
            self.wpol0(),
            self.wpol1(),
            self.cto(),
            self.bto()
        )
    }
}
#[doc = "NAND Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nandcr0(pub u32);
impl Nandcr0 {
    #[doc = "Port Size"]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> super::vals::Nandcr0Ps {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Nandcr0Ps::from_bits(val as u8)
    }
    #[doc = "Port Size"]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: super::vals::Nandcr0Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Synchronous Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn syncen(&self) -> super::vals::Nandcr0Syncen {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Nandcr0Syncen::from_bits(val as u8)
    }
    #[doc = "Synchronous Mode Enable"]
    #[inline(always)]
    pub const fn set_syncen(&mut self, val: super::vals::Nandcr0Syncen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Burst Length"]
    #[must_use]
    #[inline(always)]
    pub const fn bl(&self) -> super::vals::Nandcr0Bl {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Nandcr0Bl::from_bits(val as u8)
    }
    #[doc = "Burst Length"]
    #[inline(always)]
    pub const fn set_bl(&mut self, val: super::vals::Nandcr0Bl) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "EDO mode enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn edo(&self) -> super::vals::Edo {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Edo::from_bits(val as u8)
    }
    #[doc = "EDO mode enabled"]
    #[inline(always)]
    pub const fn set_edo(&mut self, val: super::vals::Edo) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Column address bit number"]
    #[must_use]
    #[inline(always)]
    pub const fn col(&self) -> super::vals::Nandcr0Col {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Nandcr0Col::from_bits(val as u8)
    }
    #[doc = "Column address bit number"]
    #[inline(always)]
    pub const fn set_col(&mut self, val: super::vals::Nandcr0Col) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
}
impl Default for Nandcr0 {
    #[inline(always)]
    fn default() -> Nandcr0 {
        Nandcr0(0)
    }
}
impl core::fmt::Debug for Nandcr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nandcr0")
            .field("ps", &self.ps())
            .field("syncen", &self.syncen())
            .field("bl", &self.bl())
            .field("edo", &self.edo())
            .field("col", &self.col())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nandcr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Nandcr0 {{ ps: {:?}, syncen: {:?}, bl: {:?}, edo: {:?}, col: {:?} }}",
            self.ps(),
            self.syncen(),
            self.bl(),
            self.edo(),
            self.col()
        )
    }
}
#[doc = "NAND Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nandcr1(pub u32);
impl Nandcr1 {
    #[doc = "CE# setup time"]
    #[must_use]
    #[inline(always)]
    pub const fn ces(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "CE# setup time"]
    #[inline(always)]
    pub const fn set_ces(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "CE# hold time"]
    #[must_use]
    #[inline(always)]
    pub const fn ceh(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "CE# hold time"]
    #[inline(always)]
    pub const fn set_ceh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "WE# low time"]
    #[must_use]
    #[inline(always)]
    pub const fn wel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "WE# low time"]
    #[inline(always)]
    pub const fn set_wel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "WE# high time"]
    #[must_use]
    #[inline(always)]
    pub const fn weh(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "WE# high time"]
    #[inline(always)]
    pub const fn set_weh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "RE# low time"]
    #[must_use]
    #[inline(always)]
    pub const fn rel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "RE# low time"]
    #[inline(always)]
    pub const fn set_rel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "RE# high time"]
    #[must_use]
    #[inline(always)]
    pub const fn reh(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "RE# high time"]
    #[inline(always)]
    pub const fn set_reh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Turnaround time"]
    #[must_use]
    #[inline(always)]
    pub const fn ta(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Turnaround time"]
    #[inline(always)]
    pub const fn set_ta(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "CE# interval time"]
    #[must_use]
    #[inline(always)]
    pub const fn ceitv(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "CE# interval time"]
    #[inline(always)]
    pub const fn set_ceitv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Nandcr1 {
    #[inline(always)]
    fn default() -> Nandcr1 {
        Nandcr1(0)
    }
}
impl core::fmt::Debug for Nandcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nandcr1")
            .field("ces", &self.ces())
            .field("ceh", &self.ceh())
            .field("wel", &self.wel())
            .field("weh", &self.weh())
            .field("rel", &self.rel())
            .field("reh", &self.reh())
            .field("ta", &self.ta())
            .field("ceitv", &self.ceitv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nandcr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Nandcr1 {{ ces: {=u8:?}, ceh: {=u8:?}, wel: {=u8:?}, weh: {=u8:?}, rel: {=u8:?}, reh: {=u8:?}, ta: {=u8:?}, ceitv: {=u8:?} }}",
            self.ces(),
            self.ceh(),
            self.wel(),
            self.weh(),
            self.rel(),
            self.reh(),
            self.ta(),
            self.ceitv()
        )
    }
}
#[doc = "NAND Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nandcr2(pub u32);
impl Nandcr2 {
    #[doc = "WE# high to RE# low time"]
    #[must_use]
    #[inline(always)]
    pub const fn twhr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "WE# high to RE# low time"]
    #[inline(always)]
    pub const fn set_twhr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "RE# high to WE# low time"]
    #[must_use]
    #[inline(always)]
    pub const fn trhw(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x3f;
        val as u8
    }
    #[doc = "RE# high to WE# low time"]
    #[inline(always)]
    pub const fn set_trhw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 6usize)) | (((val as u32) & 0x3f) << 6usize);
    }
    #[doc = "Address cycle to data loading time"]
    #[must_use]
    #[inline(always)]
    pub const fn tadl(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x3f;
        val as u8
    }
    #[doc = "Address cycle to data loading time"]
    #[inline(always)]
    pub const fn set_tadl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 12usize)) | (((val as u32) & 0x3f) << 12usize);
    }
    #[doc = "Ready to RE# low time"]
    #[must_use]
    #[inline(always)]
    pub const fn trr(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x3f;
        val as u8
    }
    #[doc = "Ready to RE# low time"]
    #[inline(always)]
    pub const fn set_trr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 18usize)) | (((val as u32) & 0x3f) << 18usize);
    }
    #[doc = "WE# high to busy time"]
    #[must_use]
    #[inline(always)]
    pub const fn twb(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "WE# high to busy time"]
    #[inline(always)]
    pub const fn set_twb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for Nandcr2 {
    #[inline(always)]
    fn default() -> Nandcr2 {
        Nandcr2(0)
    }
}
impl core::fmt::Debug for Nandcr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nandcr2")
            .field("twhr", &self.twhr())
            .field("trhw", &self.trhw())
            .field("tadl", &self.tadl())
            .field("trr", &self.trr())
            .field("twb", &self.twb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nandcr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Nandcr2 {{ twhr: {=u8:?}, trhw: {=u8:?}, tadl: {=u8:?}, trr: {=u8:?}, twb: {=u8:?} }}",
            self.twhr(),
            self.trhw(),
            self.tadl(),
            self.trr(),
            self.twb()
        )
    }
}
#[doc = "NAND Control Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nandcr3(pub u32);
impl Nandcr3 {
    #[doc = "NAND option bit 1"]
    #[must_use]
    #[inline(always)]
    pub const fn ndopt1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "NAND option bit 1"]
    #[inline(always)]
    pub const fn set_ndopt1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "NAND option bit 2"]
    #[must_use]
    #[inline(always)]
    pub const fn ndopt2(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "NAND option bit 2"]
    #[inline(always)]
    pub const fn set_ndopt2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "NAND option bit 3"]
    #[must_use]
    #[inline(always)]
    pub const fn ndopt3(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "NAND option bit 3"]
    #[inline(always)]
    pub const fn set_ndopt3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Read Data Setup time"]
    #[must_use]
    #[inline(always)]
    pub const fn rds(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Read Data Setup time"]
    #[inline(always)]
    pub const fn set_rds(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Read Data Hold time"]
    #[must_use]
    #[inline(always)]
    pub const fn rdh(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Read Data Hold time"]
    #[inline(always)]
    pub const fn set_rdh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Write Data Setup time"]
    #[must_use]
    #[inline(always)]
    pub const fn wds(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Write Data Setup time"]
    #[inline(always)]
    pub const fn set_wds(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Write Data Hold time"]
    #[must_use]
    #[inline(always)]
    pub const fn wdh(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Write Data Hold time"]
    #[inline(always)]
    pub const fn set_wdh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Nandcr3 {
    #[inline(always)]
    fn default() -> Nandcr3 {
        Nandcr3(0)
    }
}
impl core::fmt::Debug for Nandcr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nandcr3")
            .field("ndopt1", &self.ndopt1())
            .field("ndopt2", &self.ndopt2())
            .field("ndopt3", &self.ndopt3())
            .field("rds", &self.rds())
            .field("rdh", &self.rdh())
            .field("wds", &self.wds())
            .field("wdh", &self.wdh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nandcr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Nandcr3 {{ ndopt1: {=bool:?}, ndopt2: {=bool:?}, ndopt3: {=bool:?}, rds: {=u8:?}, rdh: {=u8:?}, wds: {=u8:?}, wdh: {=u8:?} }}",
            self.ndopt1(),
            self.ndopt2(),
            self.ndopt3(),
            self.rds(),
            self.rdh(),
            self.wds(),
            self.wdh()
        )
    }
}
#[doc = "NOR Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Norcr0(pub u32);
impl Norcr0 {
    #[doc = "Port Size"]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> super::vals::Norcr0Ps {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Norcr0Ps::from_bits(val as u8)
    }
    #[doc = "Port Size"]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: super::vals::Norcr0Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Synchronous Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn syncen(&self) -> super::vals::Norcr0Syncen {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Norcr0Syncen::from_bits(val as u8)
    }
    #[doc = "Synchronous Mode Enable"]
    #[inline(always)]
    pub const fn set_syncen(&mut self, val: super::vals::Norcr0Syncen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Burst Length"]
    #[must_use]
    #[inline(always)]
    pub const fn bl(&self) -> super::vals::Norcr0Bl {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Norcr0Bl::from_bits(val as u8)
    }
    #[doc = "Burst Length"]
    #[inline(always)]
    pub const fn set_bl(&mut self, val: super::vals::Norcr0Bl) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Address Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> super::vals::Norcr0Am {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Norcr0Am::from_bits(val as u8)
    }
    #[doc = "Address Mode"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: super::vals::Norcr0Am) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "ADV# Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn advp(&self) -> super::vals::Norcr0Advp {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Norcr0Advp::from_bits(val as u8)
    }
    #[doc = "ADV# Polarity"]
    #[inline(always)]
    pub const fn set_advp(&mut self, val: super::vals::Norcr0Advp) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "ADV# level control during address hold state"]
    #[must_use]
    #[inline(always)]
    pub const fn advh(&self) -> super::vals::Norcr0Advh {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Norcr0Advh::from_bits(val as u8)
    }
    #[doc = "ADV# level control during address hold state"]
    #[inline(always)]
    pub const fn set_advh(&mut self, val: super::vals::Norcr0Advh) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Column Address bit width"]
    #[must_use]
    #[inline(always)]
    pub const fn col(&self) -> super::vals::Norcr0Col {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Norcr0Col::from_bits(val as u8)
    }
    #[doc = "Column Address bit width"]
    #[inline(always)]
    pub const fn set_col(&mut self, val: super::vals::Norcr0Col) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
}
impl Default for Norcr0 {
    #[inline(always)]
    fn default() -> Norcr0 {
        Norcr0(0)
    }
}
impl core::fmt::Debug for Norcr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Norcr0")
            .field("ps", &self.ps())
            .field("syncen", &self.syncen())
            .field("bl", &self.bl())
            .field("am", &self.am())
            .field("advp", &self.advp())
            .field("advh", &self.advh())
            .field("col", &self.col())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Norcr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Norcr0 {{ ps: {:?}, syncen: {:?}, bl: {:?}, am: {:?}, advp: {:?}, advh: {:?}, col: {:?} }}",
            self.ps(),
            self.syncen(),
            self.bl(),
            self.am(),
            self.advp(),
            self.advh(),
            self.col()
        )
    }
}
#[doc = "NOR Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Norcr1(pub u32);
impl Norcr1 {
    #[doc = "CE setup time"]
    #[must_use]
    #[inline(always)]
    pub const fn ces(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "CE setup time"]
    #[inline(always)]
    pub const fn set_ces(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "CE hold time"]
    #[must_use]
    #[inline(always)]
    pub const fn ceh(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "CE hold time"]
    #[inline(always)]
    pub const fn set_ceh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Address setup time"]
    #[must_use]
    #[inline(always)]
    pub const fn as_(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Address setup time"]
    #[inline(always)]
    pub const fn set_as_(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Address hold time"]
    #[must_use]
    #[inline(always)]
    pub const fn ah(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Address hold time"]
    #[inline(always)]
    pub const fn set_ah(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "WE low time"]
    #[must_use]
    #[inline(always)]
    pub const fn wel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "WE low time"]
    #[inline(always)]
    pub const fn set_wel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "WE high time"]
    #[must_use]
    #[inline(always)]
    pub const fn weh(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "WE high time"]
    #[inline(always)]
    pub const fn set_weh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "RE low time"]
    #[must_use]
    #[inline(always)]
    pub const fn rel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "RE low time"]
    #[inline(always)]
    pub const fn set_rel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "RE high time"]
    #[must_use]
    #[inline(always)]
    pub const fn reh(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "RE high time"]
    #[inline(always)]
    pub const fn set_reh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Norcr1 {
    #[inline(always)]
    fn default() -> Norcr1 {
        Norcr1(0)
    }
}
impl core::fmt::Debug for Norcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Norcr1")
            .field("ces", &self.ces())
            .field("ceh", &self.ceh())
            .field("as_", &self.as_())
            .field("ah", &self.ah())
            .field("wel", &self.wel())
            .field("weh", &self.weh())
            .field("rel", &self.rel())
            .field("reh", &self.reh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Norcr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Norcr1 {{ ces: {=u8:?}, ceh: {=u8:?}, as_: {=u8:?}, ah: {=u8:?}, wel: {=u8:?}, weh: {=u8:?}, rel: {=u8:?}, reh: {=u8:?} }}",
            self.ces(),
            self.ceh(),
            self.as_(),
            self.ah(),
            self.wel(),
            self.weh(),
            self.rel(),
            self.reh()
        )
    }
}
#[doc = "NOR Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Norcr2(pub u32);
impl Norcr2 {
    #[doc = "Turnaround time"]
    #[must_use]
    #[inline(always)]
    pub const fn ta(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Turnaround time"]
    #[inline(always)]
    pub const fn set_ta(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Address to write data hold time"]
    #[must_use]
    #[inline(always)]
    pub const fn awdh(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Address to write data hold time"]
    #[inline(always)]
    pub const fn set_awdh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Latency count"]
    #[must_use]
    #[inline(always)]
    pub const fn lc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Latency count"]
    #[inline(always)]
    pub const fn set_lc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Read time"]
    #[must_use]
    #[inline(always)]
    pub const fn rd(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Read time"]
    #[inline(always)]
    pub const fn set_rd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "CE# interval time"]
    #[must_use]
    #[inline(always)]
    pub const fn ceitv(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "CE# interval time"]
    #[inline(always)]
    pub const fn set_ceitv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Read hold time"]
    #[must_use]
    #[inline(always)]
    pub const fn rdh(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Read hold time"]
    #[inline(always)]
    pub const fn set_rdh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Norcr2 {
    #[inline(always)]
    fn default() -> Norcr2 {
        Norcr2(0)
    }
}
impl core::fmt::Debug for Norcr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Norcr2")
            .field("ta", &self.ta())
            .field("awdh", &self.awdh())
            .field("lc", &self.lc())
            .field("rd", &self.rd())
            .field("ceitv", &self.ceitv())
            .field("rdh", &self.rdh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Norcr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Norcr2 {{ ta: {=u8:?}, awdh: {=u8:?}, lc: {=u8:?}, rd: {=u8:?}, ceitv: {=u8:?}, rdh: {=u8:?} }}",
            self.ta(),
            self.awdh(),
            self.lc(),
            self.rd(),
            self.ceitv(),
            self.rdh()
        )
    }
}
#[doc = "NOR Control Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Norcr3(pub u32);
impl Norcr3 {
    #[doc = "Address setup time for SYNC read"]
    #[must_use]
    #[inline(always)]
    pub const fn assr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Address setup time for SYNC read"]
    #[inline(always)]
    pub const fn set_assr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Address hold time for SYNC read"]
    #[must_use]
    #[inline(always)]
    pub const fn ahsr(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Address hold time for SYNC read"]
    #[inline(always)]
    pub const fn set_ahsr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Norcr3 {
    #[inline(always)]
    fn default() -> Norcr3 {
        Norcr3(0)
    }
}
impl core::fmt::Debug for Norcr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Norcr3")
            .field("assr", &self.assr())
            .field("ahsr", &self.ahsr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Norcr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Norcr3 {{ assr: {=u8:?}, ahsr: {=u8:?} }}",
            self.assr(),
            self.ahsr()
        )
    }
}
#[doc = "SDRAM Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdramcr0(pub u32);
impl Sdramcr0 {
    #[doc = "Port Size"]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> super::vals::Sdramcr0Ps {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sdramcr0Ps::from_bits(val as u8)
    }
    #[doc = "Port Size"]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: super::vals::Sdramcr0Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Burst Length"]
    #[must_use]
    #[inline(always)]
    pub const fn bl(&self) -> super::vals::Sdramcr0Bl {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Sdramcr0Bl::from_bits(val as u8)
    }
    #[doc = "Burst Length"]
    #[inline(always)]
    pub const fn set_bl(&mut self, val: super::vals::Sdramcr0Bl) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Column 8 selection"]
    #[must_use]
    #[inline(always)]
    pub const fn col8(&self) -> super::vals::Col8 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Col8::from_bits(val as u8)
    }
    #[doc = "Column 8 selection"]
    #[inline(always)]
    pub const fn set_col8(&mut self, val: super::vals::Col8) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Column address bit number"]
    #[must_use]
    #[inline(always)]
    pub const fn col(&self) -> super::vals::Sdramcr0Col {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Sdramcr0Col::from_bits(val as u8)
    }
    #[doc = "Column address bit number"]
    #[inline(always)]
    pub const fn set_col(&mut self, val: super::vals::Sdramcr0Col) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "CAS Latency"]
    #[must_use]
    #[inline(always)]
    pub const fn cl(&self) -> super::vals::Cl {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Cl::from_bits(val as u8)
    }
    #[doc = "CAS Latency"]
    #[inline(always)]
    pub const fn set_cl(&mut self, val: super::vals::Cl) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "2 Bank selection bit"]
    #[must_use]
    #[inline(always)]
    pub const fn bank2(&self) -> super::vals::Bank2 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Bank2::from_bits(val as u8)
    }
    #[doc = "2 Bank selection bit"]
    #[inline(always)]
    pub const fn set_bank2(&mut self, val: super::vals::Bank2) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
}
impl Default for Sdramcr0 {
    #[inline(always)]
    fn default() -> Sdramcr0 {
        Sdramcr0(0)
    }
}
impl core::fmt::Debug for Sdramcr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sdramcr0")
            .field("ps", &self.ps())
            .field("bl", &self.bl())
            .field("col8", &self.col8())
            .field("col", &self.col())
            .field("cl", &self.cl())
            .field("bank2", &self.bank2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sdramcr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sdramcr0 {{ ps: {:?}, bl: {:?}, col8: {:?}, col: {:?}, cl: {:?}, bank2: {:?} }}",
            self.ps(),
            self.bl(),
            self.col8(),
            self.col(),
            self.cl(),
            self.bank2()
        )
    }
}
#[doc = "SDRAM Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdramcr1(pub u32);
impl Sdramcr1 {
    #[doc = "PRECHARGE to ACTIVE/REFRESH command wait time"]
    #[must_use]
    #[inline(always)]
    pub const fn pre2act(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "PRECHARGE to ACTIVE/REFRESH command wait time"]
    #[inline(always)]
    pub const fn set_pre2act(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "ACTIVE to READ/WRITE delay"]
    #[must_use]
    #[inline(always)]
    pub const fn act2rw(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "ACTIVE to READ/WRITE delay"]
    #[inline(always)]
    pub const fn set_act2rw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "REFRESH recovery time"]
    #[must_use]
    #[inline(always)]
    pub const fn rfrc(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "REFRESH recovery time"]
    #[inline(always)]
    pub const fn set_rfrc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "WRITE recovery time"]
    #[must_use]
    #[inline(always)]
    pub const fn wrc(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "WRITE recovery time"]
    #[inline(always)]
    pub const fn set_wrc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "CKE off minimum time"]
    #[must_use]
    #[inline(always)]
    pub const fn ckeoff(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "CKE off minimum time"]
    #[inline(always)]
    pub const fn set_ckeoff(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "ACTIVE to PRECHARGE minimum time"]
    #[must_use]
    #[inline(always)]
    pub const fn act2pre(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "ACTIVE to PRECHARGE minimum time"]
    #[inline(always)]
    pub const fn set_act2pre(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
}
impl Default for Sdramcr1 {
    #[inline(always)]
    fn default() -> Sdramcr1 {
        Sdramcr1(0)
    }
}
impl core::fmt::Debug for Sdramcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sdramcr1")
            .field("pre2act", &self.pre2act())
            .field("act2rw", &self.act2rw())
            .field("rfrc", &self.rfrc())
            .field("wrc", &self.wrc())
            .field("ckeoff", &self.ckeoff())
            .field("act2pre", &self.act2pre())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sdramcr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sdramcr1 {{ pre2act: {=u8:?}, act2rw: {=u8:?}, rfrc: {=u8:?}, wrc: {=u8:?}, ckeoff: {=u8:?}, act2pre: {=u8:?} }}",
            self.pre2act(),
            self.act2rw(),
            self.rfrc(),
            self.wrc(),
            self.ckeoff(),
            self.act2pre()
        )
    }
}
#[doc = "SDRAM Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdramcr2(pub u32);
impl Sdramcr2 {
    #[doc = "SELF REFRESH recovery time"]
    #[must_use]
    #[inline(always)]
    pub const fn srrc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SELF REFRESH recovery time"]
    #[inline(always)]
    pub const fn set_srrc(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "REFRESH to REFRESH delay"]
    #[must_use]
    #[inline(always)]
    pub const fn ref2ref(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "REFRESH to REFRESH delay"]
    #[inline(always)]
    pub const fn set_ref2ref(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "ACTIVE to ACTIVE delay"]
    #[must_use]
    #[inline(always)]
    pub const fn act2act(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "ACTIVE to ACTIVE delay"]
    #[inline(always)]
    pub const fn set_act2act(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "SDRAM idle timeout"]
    #[must_use]
    #[inline(always)]
    pub const fn ito(&self) -> super::vals::Ito {
        let val = (self.0 >> 24usize) & 0xff;
        super::vals::Ito::from_bits(val as u8)
    }
    #[doc = "SDRAM idle timeout"]
    #[inline(always)]
    pub const fn set_ito(&mut self, val: super::vals::Ito) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val.to_bits() as u32) & 0xff) << 24usize);
    }
}
impl Default for Sdramcr2 {
    #[inline(always)]
    fn default() -> Sdramcr2 {
        Sdramcr2(0)
    }
}
impl core::fmt::Debug for Sdramcr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sdramcr2")
            .field("srrc", &self.srrc())
            .field("ref2ref", &self.ref2ref())
            .field("act2act", &self.act2act())
            .field("ito", &self.ito())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sdramcr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sdramcr2 {{ srrc: {=u8:?}, ref2ref: {=u8:?}, act2act: {=u8:?}, ito: {:?} }}",
            self.srrc(),
            self.ref2ref(),
            self.act2act(),
            self.ito()
        )
    }
}
#[doc = "SDRAM Control Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdramcr3(pub u32);
impl Sdramcr3 {
    #[doc = "Refresh enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ren(&self) -> super::vals::Ren {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ren::from_bits(val as u8)
    }
    #[doc = "Refresh enable"]
    #[inline(always)]
    pub const fn set_ren(&mut self, val: super::vals::Ren) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Refresh burst length"]
    #[must_use]
    #[inline(always)]
    pub const fn rebl(&self) -> super::vals::Rebl {
        let val = (self.0 >> 1usize) & 0x07;
        super::vals::Rebl::from_bits(val as u8)
    }
    #[doc = "Refresh burst length"]
    #[inline(always)]
    pub const fn set_rebl(&mut self, val: super::vals::Rebl) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u32) & 0x07) << 1usize);
    }
    #[doc = "Prescaler period"]
    #[must_use]
    #[inline(always)]
    pub const fn prescale(&self) -> super::vals::Prescale {
        let val = (self.0 >> 8usize) & 0xff;
        super::vals::Prescale::from_bits(val as u8)
    }
    #[doc = "Prescaler period"]
    #[inline(always)]
    pub const fn set_prescale(&mut self, val: super::vals::Prescale) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u32) & 0xff) << 8usize);
    }
    #[doc = "Refresh timer period"]
    #[must_use]
    #[inline(always)]
    pub const fn rt(&self) -> super::vals::Rt {
        let val = (self.0 >> 16usize) & 0xff;
        super::vals::Rt::from_bits(val as u8)
    }
    #[doc = "Refresh timer period"]
    #[inline(always)]
    pub const fn set_rt(&mut self, val: super::vals::Rt) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
    }
    #[doc = "Urgent refresh threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn ut(&self) -> super::vals::Ut {
        let val = (self.0 >> 24usize) & 0xff;
        super::vals::Ut::from_bits(val as u8)
    }
    #[doc = "Urgent refresh threshold"]
    #[inline(always)]
    pub const fn set_ut(&mut self, val: super::vals::Ut) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val.to_bits() as u32) & 0xff) << 24usize);
    }
}
impl Default for Sdramcr3 {
    #[inline(always)]
    fn default() -> Sdramcr3 {
        Sdramcr3(0)
    }
}
impl core::fmt::Debug for Sdramcr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sdramcr3")
            .field("ren", &self.ren())
            .field("rebl", &self.rebl())
            .field("prescale", &self.prescale())
            .field("rt", &self.rt())
            .field("ut", &self.ut())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sdramcr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sdramcr3 {{ ren: {:?}, rebl: {:?}, prescale: {:?}, rt: {:?}, ut: {:?} }}",
            self.ren(),
            self.rebl(),
            self.prescale(),
            self.rt(),
            self.ut()
        )
    }
}
#[doc = "SRAM Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramcr0(pub u32);
impl Sramcr0 {
    #[doc = "Port Size"]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> super::vals::Sramcr0Ps {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sramcr0Ps::from_bits(val as u8)
    }
    #[doc = "Port Size"]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: super::vals::Sramcr0Ps) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Synchronous Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn syncen(&self) -> super::vals::Sramcr0Syncen {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sramcr0Syncen::from_bits(val as u8)
    }
    #[doc = "Synchronous Mode Enable"]
    #[inline(always)]
    pub const fn set_syncen(&mut self, val: super::vals::Sramcr0Syncen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Burst Length"]
    #[must_use]
    #[inline(always)]
    pub const fn bl(&self) -> super::vals::Sramcr0Bl {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Sramcr0Bl::from_bits(val as u8)
    }
    #[doc = "Burst Length"]
    #[inline(always)]
    pub const fn set_bl(&mut self, val: super::vals::Sramcr0Bl) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Address Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> super::vals::Sramcr0Am {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Sramcr0Am::from_bits(val as u8)
    }
    #[doc = "Address Mode"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: super::vals::Sramcr0Am) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "ADV# polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn advp(&self) -> super::vals::Sramcr0Advp {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Sramcr0Advp::from_bits(val as u8)
    }
    #[doc = "ADV# polarity"]
    #[inline(always)]
    pub const fn set_advp(&mut self, val: super::vals::Sramcr0Advp) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "ADV# level control during address hold state"]
    #[must_use]
    #[inline(always)]
    pub const fn advh(&self) -> super::vals::Sramcr0Advh {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Sramcr0Advh::from_bits(val as u8)
    }
    #[doc = "ADV# level control during address hold state"]
    #[inline(always)]
    pub const fn set_advh(&mut self, val: super::vals::Sramcr0Advh) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Column Address bit width"]
    #[must_use]
    #[inline(always)]
    pub const fn col(&self) -> super::vals::Sramcr0Col {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Sramcr0Col::from_bits(val as u8)
    }
    #[doc = "Column Address bit width"]
    #[inline(always)]
    pub const fn set_col(&mut self, val: super::vals::Sramcr0Col) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
}
impl Default for Sramcr0 {
    #[inline(always)]
    fn default() -> Sramcr0 {
        Sramcr0(0)
    }
}
impl core::fmt::Debug for Sramcr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sramcr0")
            .field("ps", &self.ps())
            .field("syncen", &self.syncen())
            .field("bl", &self.bl())
            .field("am", &self.am())
            .field("advp", &self.advp())
            .field("advh", &self.advh())
            .field("col", &self.col())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sramcr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sramcr0 {{ ps: {:?}, syncen: {:?}, bl: {:?}, am: {:?}, advp: {:?}, advh: {:?}, col: {:?} }}",
            self.ps(),
            self.syncen(),
            self.bl(),
            self.am(),
            self.advp(),
            self.advh(),
            self.col()
        )
    }
}
#[doc = "SRAM Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramcr1(pub u32);
impl Sramcr1 {
    #[doc = "CE setup time"]
    #[must_use]
    #[inline(always)]
    pub const fn ces(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "CE setup time"]
    #[inline(always)]
    pub const fn set_ces(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "CE hold time"]
    #[must_use]
    #[inline(always)]
    pub const fn ceh(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "CE hold time"]
    #[inline(always)]
    pub const fn set_ceh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Address setup time"]
    #[must_use]
    #[inline(always)]
    pub const fn as_(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Address setup time"]
    #[inline(always)]
    pub const fn set_as_(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Address hold time"]
    #[must_use]
    #[inline(always)]
    pub const fn ah(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Address hold time"]
    #[inline(always)]
    pub const fn set_ah(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "WE low time"]
    #[must_use]
    #[inline(always)]
    pub const fn wel(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "WE low time"]
    #[inline(always)]
    pub const fn set_wel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "WE high time"]
    #[must_use]
    #[inline(always)]
    pub const fn weh(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "WE high time"]
    #[inline(always)]
    pub const fn set_weh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "RE low time"]
    #[must_use]
    #[inline(always)]
    pub const fn rel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "RE low time"]
    #[inline(always)]
    pub const fn set_rel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "RE high time"]
    #[must_use]
    #[inline(always)]
    pub const fn reh(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "RE high time"]
    #[inline(always)]
    pub const fn set_reh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Sramcr1 {
    #[inline(always)]
    fn default() -> Sramcr1 {
        Sramcr1(0)
    }
}
impl core::fmt::Debug for Sramcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sramcr1")
            .field("ces", &self.ces())
            .field("ceh", &self.ceh())
            .field("as_", &self.as_())
            .field("ah", &self.ah())
            .field("wel", &self.wel())
            .field("weh", &self.weh())
            .field("rel", &self.rel())
            .field("reh", &self.reh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sramcr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sramcr1 {{ ces: {=u8:?}, ceh: {=u8:?}, as_: {=u8:?}, ah: {=u8:?}, wel: {=u8:?}, weh: {=u8:?}, rel: {=u8:?}, reh: {=u8:?} }}",
            self.ces(),
            self.ceh(),
            self.as_(),
            self.ah(),
            self.wel(),
            self.weh(),
            self.rel(),
            self.reh()
        )
    }
}
#[doc = "SRAM Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramcr2(pub u32);
impl Sramcr2 {
    #[doc = "Write Data setup time"]
    #[must_use]
    #[inline(always)]
    pub const fn wds(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Write Data setup time"]
    #[inline(always)]
    pub const fn set_wds(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Write Data hold time"]
    #[must_use]
    #[inline(always)]
    pub const fn wdh(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Write Data hold time"]
    #[inline(always)]
    pub const fn set_wdh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Turnaround time"]
    #[must_use]
    #[inline(always)]
    pub const fn ta(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Turnaround time"]
    #[inline(always)]
    pub const fn set_ta(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Address to write data hold time"]
    #[must_use]
    #[inline(always)]
    pub const fn awdh(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Address to write data hold time"]
    #[inline(always)]
    pub const fn set_awdh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Latency count"]
    #[must_use]
    #[inline(always)]
    pub const fn lc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Latency count"]
    #[inline(always)]
    pub const fn set_lc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Read time"]
    #[must_use]
    #[inline(always)]
    pub const fn rd(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Read time"]
    #[inline(always)]
    pub const fn set_rd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "CE# interval time"]
    #[must_use]
    #[inline(always)]
    pub const fn ceitv(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "CE# interval time"]
    #[inline(always)]
    pub const fn set_ceitv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Read hold time"]
    #[must_use]
    #[inline(always)]
    pub const fn rdh(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Read hold time"]
    #[inline(always)]
    pub const fn set_rdh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Sramcr2 {
    #[inline(always)]
    fn default() -> Sramcr2 {
        Sramcr2(0)
    }
}
impl core::fmt::Debug for Sramcr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sramcr2")
            .field("wds", &self.wds())
            .field("wdh", &self.wdh())
            .field("ta", &self.ta())
            .field("awdh", &self.awdh())
            .field("lc", &self.lc())
            .field("rd", &self.rd())
            .field("ceitv", &self.ceitv())
            .field("rdh", &self.rdh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sramcr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sramcr2 {{ wds: {=u8:?}, wdh: {=u8:?}, ta: {=u8:?}, awdh: {=u8:?}, lc: {=u8:?}, rd: {=u8:?}, ceitv: {=u8:?}, rdh: {=u8:?} }}",
            self.wds(),
            self.wdh(),
            self.ta(),
            self.awdh(),
            self.lc(),
            self.rd(),
            self.ceitv(),
            self.rdh()
        )
    }
}
#[doc = "Status Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sts0(pub u32);
impl Sts0 {
    #[doc = "Indicating whether the SEMC is in idle state."]
    #[must_use]
    #[inline(always)]
    pub const fn idle(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicating whether the SEMC is in idle state."]
    #[inline(always)]
    pub const fn set_idle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicating NAND device Ready/WAIT# pin level."]
    #[must_use]
    #[inline(always)]
    pub const fn nardy(&self) -> super::vals::Nardy {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Nardy::from_bits(val as u8)
    }
    #[doc = "Indicating NAND device Ready/WAIT# pin level."]
    #[inline(always)]
    pub const fn set_nardy(&mut self, val: super::vals::Nardy) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for Sts0 {
    #[inline(always)]
    fn default() -> Sts0 {
        Sts0(0)
    }
}
impl core::fmt::Debug for Sts0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sts0")
            .field("idle", &self.idle())
            .field("nardy", &self.nardy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sts0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sts0 {{ idle: {=bool:?}, nardy: {:?} }}",
            self.idle(),
            self.nardy()
        )
    }
}
#[doc = "Status Register 12"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sts12(pub u32);
impl Sts12 {
    #[doc = "This field indicating the last write address (AXI command) to NAND device (without base address in SEMC_BR4)."]
    #[must_use]
    #[inline(always)]
    pub const fn ndaddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "This field indicating the last write address (AXI command) to NAND device (without base address in SEMC_BR4)."]
    #[inline(always)]
    pub const fn set_ndaddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Sts12 {
    #[inline(always)]
    fn default() -> Sts12 {
        Sts12(0)
    }
}
impl core::fmt::Debug for Sts12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sts12")
            .field("ndaddr", &self.ndaddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sts12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sts12 {{ ndaddr: {=u32:?} }}", self.ndaddr())
    }
}
#[doc = "Status Register 13"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sts13(pub u32);
impl Sts13 {
    #[doc = "Sample clock slave delay line locked."]
    #[must_use]
    #[inline(always)]
    pub const fn slvlock(&self) -> super::vals::Slvlock {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Slvlock::from_bits(val as u8)
    }
    #[doc = "Sample clock slave delay line locked."]
    #[inline(always)]
    pub const fn set_slvlock(&mut self, val: super::vals::Slvlock) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Sample clock reference delay line locked."]
    #[must_use]
    #[inline(always)]
    pub const fn reflock(&self) -> super::vals::Reflock {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Reflock::from_bits(val as u8)
    }
    #[doc = "Sample clock reference delay line locked."]
    #[inline(always)]
    pub const fn set_reflock(&mut self, val: super::vals::Reflock) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Sample clock slave delay line delay cell number selection."]
    #[must_use]
    #[inline(always)]
    pub const fn slvsel(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[doc = "Sample clock slave delay line delay cell number selection."]
    #[inline(always)]
    pub const fn set_slvsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u32) & 0x3f) << 2usize);
    }
    #[doc = "Sample clock reference delay line delay cell number selection."]
    #[must_use]
    #[inline(always)]
    pub const fn refsel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Sample clock reference delay line delay cell number selection."]
    #[inline(always)]
    pub const fn set_refsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
}
impl Default for Sts13 {
    #[inline(always)]
    fn default() -> Sts13 {
        Sts13(0)
    }
}
impl core::fmt::Debug for Sts13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sts13")
            .field("slvlock", &self.slvlock())
            .field("reflock", &self.reflock())
            .field("slvsel", &self.slvsel())
            .field("refsel", &self.refsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sts13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sts13 {{ slvlock: {:?}, reflock: {:?}, slvsel: {=u8:?}, refsel: {=u8:?} }}",
            self.slvlock(),
            self.reflock(),
            self.slvsel(),
            self.refsel()
        )
    }
}
#[doc = "Status Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sts2(pub u32);
impl Sts2 {
    #[doc = "This field indicating whether there is pending AXI command (write) to NAND device."]
    #[must_use]
    #[inline(always)]
    pub const fn ndwrpend(&self) -> super::vals::Ndwrpend {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Ndwrpend::from_bits(val as u8)
    }
    #[doc = "This field indicating whether there is pending AXI command (write) to NAND device."]
    #[inline(always)]
    pub const fn set_ndwrpend(&mut self, val: super::vals::Ndwrpend) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for Sts2 {
    #[inline(always)]
    fn default() -> Sts2 {
        Sts2(0)
    }
}
impl core::fmt::Debug for Sts2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sts2")
            .field("ndwrpend", &self.ndwrpend())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sts2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sts2 {{ ndwrpend: {:?} }}", self.ndwrpend())
    }
}
