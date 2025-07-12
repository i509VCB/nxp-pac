#[doc = "GPIO data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dr(pub u32);
impl Dr {
    #[must_use]
    #[inline(always)]
    pub const fn dr(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_dr(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Dr {
    #[inline(always)]
    fn default() -> Dr {
        Dr(0)
    }
}
impl core::fmt::Debug for Dr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dr")
            .field("dr[0]", &self.dr(0usize))
            .field("dr[1]", &self.dr(1usize))
            .field("dr[2]", &self.dr(2usize))
            .field("dr[3]", &self.dr(3usize))
            .field("dr[4]", &self.dr(4usize))
            .field("dr[5]", &self.dr(5usize))
            .field("dr[6]", &self.dr(6usize))
            .field("dr[7]", &self.dr(7usize))
            .field("dr[8]", &self.dr(8usize))
            .field("dr[9]", &self.dr(9usize))
            .field("dr[10]", &self.dr(10usize))
            .field("dr[11]", &self.dr(11usize))
            .field("dr[12]", &self.dr(12usize))
            .field("dr[13]", &self.dr(13usize))
            .field("dr[14]", &self.dr(14usize))
            .field("dr[15]", &self.dr(15usize))
            .field("dr[16]", &self.dr(16usize))
            .field("dr[17]", &self.dr(17usize))
            .field("dr[18]", &self.dr(18usize))
            .field("dr[19]", &self.dr(19usize))
            .field("dr[20]", &self.dr(20usize))
            .field("dr[21]", &self.dr(21usize))
            .field("dr[22]", &self.dr(22usize))
            .field("dr[23]", &self.dr(23usize))
            .field("dr[24]", &self.dr(24usize))
            .field("dr[25]", &self.dr(25usize))
            .field("dr[26]", &self.dr(26usize))
            .field("dr[27]", &self.dr(27usize))
            .field("dr[28]", &self.dr(28usize))
            .field("dr[29]", &self.dr(29usize))
            .field("dr[30]", &self.dr(30usize))
            .field("dr[31]", &self.dr(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dr {{ dr[0]: {=bool:?}, dr[1]: {=bool:?}, dr[2]: {=bool:?}, dr[3]: {=bool:?}, dr[4]: {=bool:?}, dr[5]: {=bool:?}, dr[6]: {=bool:?}, dr[7]: {=bool:?}, dr[8]: {=bool:?}, dr[9]: {=bool:?}, dr[10]: {=bool:?}, dr[11]: {=bool:?}, dr[12]: {=bool:?}, dr[13]: {=bool:?}, dr[14]: {=bool:?}, dr[15]: {=bool:?}, dr[16]: {=bool:?}, dr[17]: {=bool:?}, dr[18]: {=bool:?}, dr[19]: {=bool:?}, dr[20]: {=bool:?}, dr[21]: {=bool:?}, dr[22]: {=bool:?}, dr[23]: {=bool:?}, dr[24]: {=bool:?}, dr[25]: {=bool:?}, dr[26]: {=bool:?}, dr[27]: {=bool:?}, dr[28]: {=bool:?}, dr[29]: {=bool:?}, dr[30]: {=bool:?}, dr[31]: {=bool:?} }}",
            self.dr(0usize),
            self.dr(1usize),
            self.dr(2usize),
            self.dr(3usize),
            self.dr(4usize),
            self.dr(5usize),
            self.dr(6usize),
            self.dr(7usize),
            self.dr(8usize),
            self.dr(9usize),
            self.dr(10usize),
            self.dr(11usize),
            self.dr(12usize),
            self.dr(13usize),
            self.dr(14usize),
            self.dr(15usize),
            self.dr(16usize),
            self.dr(17usize),
            self.dr(18usize),
            self.dr(19usize),
            self.dr(20usize),
            self.dr(21usize),
            self.dr(22usize),
            self.dr(23usize),
            self.dr(24usize),
            self.dr(25usize),
            self.dr(26usize),
            self.dr(27usize),
            self.dr(28usize),
            self.dr(29usize),
            self.dr(30usize),
            self.dr(31usize)
        )
    }
}
#[doc = "GPIO data register CLEAR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DrClear(pub u32);
impl DrClear {
    #[must_use]
    #[inline(always)]
    pub const fn dr_clear(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_dr_clear(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for DrClear {
    #[inline(always)]
    fn default() -> DrClear {
        DrClear(0)
    }
}
impl core::fmt::Debug for DrClear {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DrClear")
            .field("dr_clear[0]", &self.dr_clear(0usize))
            .field("dr_clear[1]", &self.dr_clear(1usize))
            .field("dr_clear[2]", &self.dr_clear(2usize))
            .field("dr_clear[3]", &self.dr_clear(3usize))
            .field("dr_clear[4]", &self.dr_clear(4usize))
            .field("dr_clear[5]", &self.dr_clear(5usize))
            .field("dr_clear[6]", &self.dr_clear(6usize))
            .field("dr_clear[7]", &self.dr_clear(7usize))
            .field("dr_clear[8]", &self.dr_clear(8usize))
            .field("dr_clear[9]", &self.dr_clear(9usize))
            .field("dr_clear[10]", &self.dr_clear(10usize))
            .field("dr_clear[11]", &self.dr_clear(11usize))
            .field("dr_clear[12]", &self.dr_clear(12usize))
            .field("dr_clear[13]", &self.dr_clear(13usize))
            .field("dr_clear[14]", &self.dr_clear(14usize))
            .field("dr_clear[15]", &self.dr_clear(15usize))
            .field("dr_clear[16]", &self.dr_clear(16usize))
            .field("dr_clear[17]", &self.dr_clear(17usize))
            .field("dr_clear[18]", &self.dr_clear(18usize))
            .field("dr_clear[19]", &self.dr_clear(19usize))
            .field("dr_clear[20]", &self.dr_clear(20usize))
            .field("dr_clear[21]", &self.dr_clear(21usize))
            .field("dr_clear[22]", &self.dr_clear(22usize))
            .field("dr_clear[23]", &self.dr_clear(23usize))
            .field("dr_clear[24]", &self.dr_clear(24usize))
            .field("dr_clear[25]", &self.dr_clear(25usize))
            .field("dr_clear[26]", &self.dr_clear(26usize))
            .field("dr_clear[27]", &self.dr_clear(27usize))
            .field("dr_clear[28]", &self.dr_clear(28usize))
            .field("dr_clear[29]", &self.dr_clear(29usize))
            .field("dr_clear[30]", &self.dr_clear(30usize))
            .field("dr_clear[31]", &self.dr_clear(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DrClear {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DrClear {{ dr_clear[0]: {=bool:?}, dr_clear[1]: {=bool:?}, dr_clear[2]: {=bool:?}, dr_clear[3]: {=bool:?}, dr_clear[4]: {=bool:?}, dr_clear[5]: {=bool:?}, dr_clear[6]: {=bool:?}, dr_clear[7]: {=bool:?}, dr_clear[8]: {=bool:?}, dr_clear[9]: {=bool:?}, dr_clear[10]: {=bool:?}, dr_clear[11]: {=bool:?}, dr_clear[12]: {=bool:?}, dr_clear[13]: {=bool:?}, dr_clear[14]: {=bool:?}, dr_clear[15]: {=bool:?}, dr_clear[16]: {=bool:?}, dr_clear[17]: {=bool:?}, dr_clear[18]: {=bool:?}, dr_clear[19]: {=bool:?}, dr_clear[20]: {=bool:?}, dr_clear[21]: {=bool:?}, dr_clear[22]: {=bool:?}, dr_clear[23]: {=bool:?}, dr_clear[24]: {=bool:?}, dr_clear[25]: {=bool:?}, dr_clear[26]: {=bool:?}, dr_clear[27]: {=bool:?}, dr_clear[28]: {=bool:?}, dr_clear[29]: {=bool:?}, dr_clear[30]: {=bool:?}, dr_clear[31]: {=bool:?} }}",
            self.dr_clear(0usize),
            self.dr_clear(1usize),
            self.dr_clear(2usize),
            self.dr_clear(3usize),
            self.dr_clear(4usize),
            self.dr_clear(5usize),
            self.dr_clear(6usize),
            self.dr_clear(7usize),
            self.dr_clear(8usize),
            self.dr_clear(9usize),
            self.dr_clear(10usize),
            self.dr_clear(11usize),
            self.dr_clear(12usize),
            self.dr_clear(13usize),
            self.dr_clear(14usize),
            self.dr_clear(15usize),
            self.dr_clear(16usize),
            self.dr_clear(17usize),
            self.dr_clear(18usize),
            self.dr_clear(19usize),
            self.dr_clear(20usize),
            self.dr_clear(21usize),
            self.dr_clear(22usize),
            self.dr_clear(23usize),
            self.dr_clear(24usize),
            self.dr_clear(25usize),
            self.dr_clear(26usize),
            self.dr_clear(27usize),
            self.dr_clear(28usize),
            self.dr_clear(29usize),
            self.dr_clear(30usize),
            self.dr_clear(31usize)
        )
    }
}
#[doc = "GPIO data register SET"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DrSet(pub u32);
impl DrSet {
    #[must_use]
    #[inline(always)]
    pub const fn dr_set(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_dr_set(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for DrSet {
    #[inline(always)]
    fn default() -> DrSet {
        DrSet(0)
    }
}
impl core::fmt::Debug for DrSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DrSet")
            .field("dr_set[0]", &self.dr_set(0usize))
            .field("dr_set[1]", &self.dr_set(1usize))
            .field("dr_set[2]", &self.dr_set(2usize))
            .field("dr_set[3]", &self.dr_set(3usize))
            .field("dr_set[4]", &self.dr_set(4usize))
            .field("dr_set[5]", &self.dr_set(5usize))
            .field("dr_set[6]", &self.dr_set(6usize))
            .field("dr_set[7]", &self.dr_set(7usize))
            .field("dr_set[8]", &self.dr_set(8usize))
            .field("dr_set[9]", &self.dr_set(9usize))
            .field("dr_set[10]", &self.dr_set(10usize))
            .field("dr_set[11]", &self.dr_set(11usize))
            .field("dr_set[12]", &self.dr_set(12usize))
            .field("dr_set[13]", &self.dr_set(13usize))
            .field("dr_set[14]", &self.dr_set(14usize))
            .field("dr_set[15]", &self.dr_set(15usize))
            .field("dr_set[16]", &self.dr_set(16usize))
            .field("dr_set[17]", &self.dr_set(17usize))
            .field("dr_set[18]", &self.dr_set(18usize))
            .field("dr_set[19]", &self.dr_set(19usize))
            .field("dr_set[20]", &self.dr_set(20usize))
            .field("dr_set[21]", &self.dr_set(21usize))
            .field("dr_set[22]", &self.dr_set(22usize))
            .field("dr_set[23]", &self.dr_set(23usize))
            .field("dr_set[24]", &self.dr_set(24usize))
            .field("dr_set[25]", &self.dr_set(25usize))
            .field("dr_set[26]", &self.dr_set(26usize))
            .field("dr_set[27]", &self.dr_set(27usize))
            .field("dr_set[28]", &self.dr_set(28usize))
            .field("dr_set[29]", &self.dr_set(29usize))
            .field("dr_set[30]", &self.dr_set(30usize))
            .field("dr_set[31]", &self.dr_set(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DrSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DrSet {{ dr_set[0]: {=bool:?}, dr_set[1]: {=bool:?}, dr_set[2]: {=bool:?}, dr_set[3]: {=bool:?}, dr_set[4]: {=bool:?}, dr_set[5]: {=bool:?}, dr_set[6]: {=bool:?}, dr_set[7]: {=bool:?}, dr_set[8]: {=bool:?}, dr_set[9]: {=bool:?}, dr_set[10]: {=bool:?}, dr_set[11]: {=bool:?}, dr_set[12]: {=bool:?}, dr_set[13]: {=bool:?}, dr_set[14]: {=bool:?}, dr_set[15]: {=bool:?}, dr_set[16]: {=bool:?}, dr_set[17]: {=bool:?}, dr_set[18]: {=bool:?}, dr_set[19]: {=bool:?}, dr_set[20]: {=bool:?}, dr_set[21]: {=bool:?}, dr_set[22]: {=bool:?}, dr_set[23]: {=bool:?}, dr_set[24]: {=bool:?}, dr_set[25]: {=bool:?}, dr_set[26]: {=bool:?}, dr_set[27]: {=bool:?}, dr_set[28]: {=bool:?}, dr_set[29]: {=bool:?}, dr_set[30]: {=bool:?}, dr_set[31]: {=bool:?} }}",
            self.dr_set(0usize),
            self.dr_set(1usize),
            self.dr_set(2usize),
            self.dr_set(3usize),
            self.dr_set(4usize),
            self.dr_set(5usize),
            self.dr_set(6usize),
            self.dr_set(7usize),
            self.dr_set(8usize),
            self.dr_set(9usize),
            self.dr_set(10usize),
            self.dr_set(11usize),
            self.dr_set(12usize),
            self.dr_set(13usize),
            self.dr_set(14usize),
            self.dr_set(15usize),
            self.dr_set(16usize),
            self.dr_set(17usize),
            self.dr_set(18usize),
            self.dr_set(19usize),
            self.dr_set(20usize),
            self.dr_set(21usize),
            self.dr_set(22usize),
            self.dr_set(23usize),
            self.dr_set(24usize),
            self.dr_set(25usize),
            self.dr_set(26usize),
            self.dr_set(27usize),
            self.dr_set(28usize),
            self.dr_set(29usize),
            self.dr_set(30usize),
            self.dr_set(31usize)
        )
    }
}
#[doc = "GPIO data register TOGGLE"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DrToggle(pub u32);
impl DrToggle {
    #[must_use]
    #[inline(always)]
    pub const fn dr_toggle(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_dr_toggle(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for DrToggle {
    #[inline(always)]
    fn default() -> DrToggle {
        DrToggle(0)
    }
}
impl core::fmt::Debug for DrToggle {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DrToggle")
            .field("dr_toggle[0]", &self.dr_toggle(0usize))
            .field("dr_toggle[1]", &self.dr_toggle(1usize))
            .field("dr_toggle[2]", &self.dr_toggle(2usize))
            .field("dr_toggle[3]", &self.dr_toggle(3usize))
            .field("dr_toggle[4]", &self.dr_toggle(4usize))
            .field("dr_toggle[5]", &self.dr_toggle(5usize))
            .field("dr_toggle[6]", &self.dr_toggle(6usize))
            .field("dr_toggle[7]", &self.dr_toggle(7usize))
            .field("dr_toggle[8]", &self.dr_toggle(8usize))
            .field("dr_toggle[9]", &self.dr_toggle(9usize))
            .field("dr_toggle[10]", &self.dr_toggle(10usize))
            .field("dr_toggle[11]", &self.dr_toggle(11usize))
            .field("dr_toggle[12]", &self.dr_toggle(12usize))
            .field("dr_toggle[13]", &self.dr_toggle(13usize))
            .field("dr_toggle[14]", &self.dr_toggle(14usize))
            .field("dr_toggle[15]", &self.dr_toggle(15usize))
            .field("dr_toggle[16]", &self.dr_toggle(16usize))
            .field("dr_toggle[17]", &self.dr_toggle(17usize))
            .field("dr_toggle[18]", &self.dr_toggle(18usize))
            .field("dr_toggle[19]", &self.dr_toggle(19usize))
            .field("dr_toggle[20]", &self.dr_toggle(20usize))
            .field("dr_toggle[21]", &self.dr_toggle(21usize))
            .field("dr_toggle[22]", &self.dr_toggle(22usize))
            .field("dr_toggle[23]", &self.dr_toggle(23usize))
            .field("dr_toggle[24]", &self.dr_toggle(24usize))
            .field("dr_toggle[25]", &self.dr_toggle(25usize))
            .field("dr_toggle[26]", &self.dr_toggle(26usize))
            .field("dr_toggle[27]", &self.dr_toggle(27usize))
            .field("dr_toggle[28]", &self.dr_toggle(28usize))
            .field("dr_toggle[29]", &self.dr_toggle(29usize))
            .field("dr_toggle[30]", &self.dr_toggle(30usize))
            .field("dr_toggle[31]", &self.dr_toggle(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DrToggle {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DrToggle {{ dr_toggle[0]: {=bool:?}, dr_toggle[1]: {=bool:?}, dr_toggle[2]: {=bool:?}, dr_toggle[3]: {=bool:?}, dr_toggle[4]: {=bool:?}, dr_toggle[5]: {=bool:?}, dr_toggle[6]: {=bool:?}, dr_toggle[7]: {=bool:?}, dr_toggle[8]: {=bool:?}, dr_toggle[9]: {=bool:?}, dr_toggle[10]: {=bool:?}, dr_toggle[11]: {=bool:?}, dr_toggle[12]: {=bool:?}, dr_toggle[13]: {=bool:?}, dr_toggle[14]: {=bool:?}, dr_toggle[15]: {=bool:?}, dr_toggle[16]: {=bool:?}, dr_toggle[17]: {=bool:?}, dr_toggle[18]: {=bool:?}, dr_toggle[19]: {=bool:?}, dr_toggle[20]: {=bool:?}, dr_toggle[21]: {=bool:?}, dr_toggle[22]: {=bool:?}, dr_toggle[23]: {=bool:?}, dr_toggle[24]: {=bool:?}, dr_toggle[25]: {=bool:?}, dr_toggle[26]: {=bool:?}, dr_toggle[27]: {=bool:?}, dr_toggle[28]: {=bool:?}, dr_toggle[29]: {=bool:?}, dr_toggle[30]: {=bool:?}, dr_toggle[31]: {=bool:?} }}",
            self.dr_toggle(0usize),
            self.dr_toggle(1usize),
            self.dr_toggle(2usize),
            self.dr_toggle(3usize),
            self.dr_toggle(4usize),
            self.dr_toggle(5usize),
            self.dr_toggle(6usize),
            self.dr_toggle(7usize),
            self.dr_toggle(8usize),
            self.dr_toggle(9usize),
            self.dr_toggle(10usize),
            self.dr_toggle(11usize),
            self.dr_toggle(12usize),
            self.dr_toggle(13usize),
            self.dr_toggle(14usize),
            self.dr_toggle(15usize),
            self.dr_toggle(16usize),
            self.dr_toggle(17usize),
            self.dr_toggle(18usize),
            self.dr_toggle(19usize),
            self.dr_toggle(20usize),
            self.dr_toggle(21usize),
            self.dr_toggle(22usize),
            self.dr_toggle(23usize),
            self.dr_toggle(24usize),
            self.dr_toggle(25usize),
            self.dr_toggle(26usize),
            self.dr_toggle(27usize),
            self.dr_toggle(28usize),
            self.dr_toggle(29usize),
            self.dr_toggle(30usize),
            self.dr_toggle(31usize)
        )
    }
}
#[doc = "GPIO edge select register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EdgeSel(pub u32);
impl EdgeSel {
    #[must_use]
    #[inline(always)]
    pub const fn edge_sel(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_edge_sel(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for EdgeSel {
    #[inline(always)]
    fn default() -> EdgeSel {
        EdgeSel(0)
    }
}
impl core::fmt::Debug for EdgeSel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EdgeSel")
            .field("edge_sel[0]", &self.edge_sel(0usize))
            .field("edge_sel[1]", &self.edge_sel(1usize))
            .field("edge_sel[2]", &self.edge_sel(2usize))
            .field("edge_sel[3]", &self.edge_sel(3usize))
            .field("edge_sel[4]", &self.edge_sel(4usize))
            .field("edge_sel[5]", &self.edge_sel(5usize))
            .field("edge_sel[6]", &self.edge_sel(6usize))
            .field("edge_sel[7]", &self.edge_sel(7usize))
            .field("edge_sel[8]", &self.edge_sel(8usize))
            .field("edge_sel[9]", &self.edge_sel(9usize))
            .field("edge_sel[10]", &self.edge_sel(10usize))
            .field("edge_sel[11]", &self.edge_sel(11usize))
            .field("edge_sel[12]", &self.edge_sel(12usize))
            .field("edge_sel[13]", &self.edge_sel(13usize))
            .field("edge_sel[14]", &self.edge_sel(14usize))
            .field("edge_sel[15]", &self.edge_sel(15usize))
            .field("edge_sel[16]", &self.edge_sel(16usize))
            .field("edge_sel[17]", &self.edge_sel(17usize))
            .field("edge_sel[18]", &self.edge_sel(18usize))
            .field("edge_sel[19]", &self.edge_sel(19usize))
            .field("edge_sel[20]", &self.edge_sel(20usize))
            .field("edge_sel[21]", &self.edge_sel(21usize))
            .field("edge_sel[22]", &self.edge_sel(22usize))
            .field("edge_sel[23]", &self.edge_sel(23usize))
            .field("edge_sel[24]", &self.edge_sel(24usize))
            .field("edge_sel[25]", &self.edge_sel(25usize))
            .field("edge_sel[26]", &self.edge_sel(26usize))
            .field("edge_sel[27]", &self.edge_sel(27usize))
            .field("edge_sel[28]", &self.edge_sel(28usize))
            .field("edge_sel[29]", &self.edge_sel(29usize))
            .field("edge_sel[30]", &self.edge_sel(30usize))
            .field("edge_sel[31]", &self.edge_sel(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EdgeSel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EdgeSel {{ edge_sel[0]: {=bool:?}, edge_sel[1]: {=bool:?}, edge_sel[2]: {=bool:?}, edge_sel[3]: {=bool:?}, edge_sel[4]: {=bool:?}, edge_sel[5]: {=bool:?}, edge_sel[6]: {=bool:?}, edge_sel[7]: {=bool:?}, edge_sel[8]: {=bool:?}, edge_sel[9]: {=bool:?}, edge_sel[10]: {=bool:?}, edge_sel[11]: {=bool:?}, edge_sel[12]: {=bool:?}, edge_sel[13]: {=bool:?}, edge_sel[14]: {=bool:?}, edge_sel[15]: {=bool:?}, edge_sel[16]: {=bool:?}, edge_sel[17]: {=bool:?}, edge_sel[18]: {=bool:?}, edge_sel[19]: {=bool:?}, edge_sel[20]: {=bool:?}, edge_sel[21]: {=bool:?}, edge_sel[22]: {=bool:?}, edge_sel[23]: {=bool:?}, edge_sel[24]: {=bool:?}, edge_sel[25]: {=bool:?}, edge_sel[26]: {=bool:?}, edge_sel[27]: {=bool:?}, edge_sel[28]: {=bool:?}, edge_sel[29]: {=bool:?}, edge_sel[30]: {=bool:?}, edge_sel[31]: {=bool:?} }}",
            self.edge_sel(0usize),
            self.edge_sel(1usize),
            self.edge_sel(2usize),
            self.edge_sel(3usize),
            self.edge_sel(4usize),
            self.edge_sel(5usize),
            self.edge_sel(6usize),
            self.edge_sel(7usize),
            self.edge_sel(8usize),
            self.edge_sel(9usize),
            self.edge_sel(10usize),
            self.edge_sel(11usize),
            self.edge_sel(12usize),
            self.edge_sel(13usize),
            self.edge_sel(14usize),
            self.edge_sel(15usize),
            self.edge_sel(16usize),
            self.edge_sel(17usize),
            self.edge_sel(18usize),
            self.edge_sel(19usize),
            self.edge_sel(20usize),
            self.edge_sel(21usize),
            self.edge_sel(22usize),
            self.edge_sel(23usize),
            self.edge_sel(24usize),
            self.edge_sel(25usize),
            self.edge_sel(26usize),
            self.edge_sel(27usize),
            self.edge_sel(28usize),
            self.edge_sel(29usize),
            self.edge_sel(30usize),
            self.edge_sel(31usize)
        )
    }
}
#[doc = "GPIO direction register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gdir(pub u32);
impl Gdir {
    #[must_use]
    #[inline(always)]
    pub const fn gdir(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_gdir(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Gdir {
    #[inline(always)]
    fn default() -> Gdir {
        Gdir(0)
    }
}
impl core::fmt::Debug for Gdir {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gdir")
            .field("gdir[0]", &self.gdir(0usize))
            .field("gdir[1]", &self.gdir(1usize))
            .field("gdir[2]", &self.gdir(2usize))
            .field("gdir[3]", &self.gdir(3usize))
            .field("gdir[4]", &self.gdir(4usize))
            .field("gdir[5]", &self.gdir(5usize))
            .field("gdir[6]", &self.gdir(6usize))
            .field("gdir[7]", &self.gdir(7usize))
            .field("gdir[8]", &self.gdir(8usize))
            .field("gdir[9]", &self.gdir(9usize))
            .field("gdir[10]", &self.gdir(10usize))
            .field("gdir[11]", &self.gdir(11usize))
            .field("gdir[12]", &self.gdir(12usize))
            .field("gdir[13]", &self.gdir(13usize))
            .field("gdir[14]", &self.gdir(14usize))
            .field("gdir[15]", &self.gdir(15usize))
            .field("gdir[16]", &self.gdir(16usize))
            .field("gdir[17]", &self.gdir(17usize))
            .field("gdir[18]", &self.gdir(18usize))
            .field("gdir[19]", &self.gdir(19usize))
            .field("gdir[20]", &self.gdir(20usize))
            .field("gdir[21]", &self.gdir(21usize))
            .field("gdir[22]", &self.gdir(22usize))
            .field("gdir[23]", &self.gdir(23usize))
            .field("gdir[24]", &self.gdir(24usize))
            .field("gdir[25]", &self.gdir(25usize))
            .field("gdir[26]", &self.gdir(26usize))
            .field("gdir[27]", &self.gdir(27usize))
            .field("gdir[28]", &self.gdir(28usize))
            .field("gdir[29]", &self.gdir(29usize))
            .field("gdir[30]", &self.gdir(30usize))
            .field("gdir[31]", &self.gdir(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gdir {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gdir {{ gdir[0]: {=bool:?}, gdir[1]: {=bool:?}, gdir[2]: {=bool:?}, gdir[3]: {=bool:?}, gdir[4]: {=bool:?}, gdir[5]: {=bool:?}, gdir[6]: {=bool:?}, gdir[7]: {=bool:?}, gdir[8]: {=bool:?}, gdir[9]: {=bool:?}, gdir[10]: {=bool:?}, gdir[11]: {=bool:?}, gdir[12]: {=bool:?}, gdir[13]: {=bool:?}, gdir[14]: {=bool:?}, gdir[15]: {=bool:?}, gdir[16]: {=bool:?}, gdir[17]: {=bool:?}, gdir[18]: {=bool:?}, gdir[19]: {=bool:?}, gdir[20]: {=bool:?}, gdir[21]: {=bool:?}, gdir[22]: {=bool:?}, gdir[23]: {=bool:?}, gdir[24]: {=bool:?}, gdir[25]: {=bool:?}, gdir[26]: {=bool:?}, gdir[27]: {=bool:?}, gdir[28]: {=bool:?}, gdir[29]: {=bool:?}, gdir[30]: {=bool:?}, gdir[31]: {=bool:?} }}",
            self.gdir(0usize),
            self.gdir(1usize),
            self.gdir(2usize),
            self.gdir(3usize),
            self.gdir(4usize),
            self.gdir(5usize),
            self.gdir(6usize),
            self.gdir(7usize),
            self.gdir(8usize),
            self.gdir(9usize),
            self.gdir(10usize),
            self.gdir(11usize),
            self.gdir(12usize),
            self.gdir(13usize),
            self.gdir(14usize),
            self.gdir(15usize),
            self.gdir(16usize),
            self.gdir(17usize),
            self.gdir(18usize),
            self.gdir(19usize),
            self.gdir(20usize),
            self.gdir(21usize),
            self.gdir(22usize),
            self.gdir(23usize),
            self.gdir(24usize),
            self.gdir(25usize),
            self.gdir(26usize),
            self.gdir(27usize),
            self.gdir(28usize),
            self.gdir(29usize),
            self.gdir(30usize),
            self.gdir(31usize)
        )
    }
}
#[doc = "GPIO interrupt configuration register1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr(pub u32);
impl Icr {
    #[doc = "Interrupt configuration field for GPIO interrupt 0"]
    #[must_use]
    #[inline(always)]
    pub const fn pin(&self, n: usize) -> super::vals::Icr {
        assert!(n < 16usize);
        let offs = 0usize + n * 2usize;
        let val = (self.0 >> offs) & 0x03;
        super::vals::Icr::from_bits(val as u8)
    }
    #[doc = "Interrupt configuration field for GPIO interrupt 0"]
    #[inline(always)]
    pub const fn set_pin(&mut self, n: usize, val: super::vals::Icr) {
        assert!(n < 16usize);
        let offs = 0usize + n * 2usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for Icr {
    #[inline(always)]
    fn default() -> Icr {
        Icr(0)
    }
}
impl core::fmt::Debug for Icr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr")
            .field("pin[0]", &self.pin(0usize))
            .field("pin[1]", &self.pin(1usize))
            .field("pin[2]", &self.pin(2usize))
            .field("pin[3]", &self.pin(3usize))
            .field("pin[4]", &self.pin(4usize))
            .field("pin[5]", &self.pin(5usize))
            .field("pin[6]", &self.pin(6usize))
            .field("pin[7]", &self.pin(7usize))
            .field("pin[8]", &self.pin(8usize))
            .field("pin[9]", &self.pin(9usize))
            .field("pin[10]", &self.pin(10usize))
            .field("pin[11]", &self.pin(11usize))
            .field("pin[12]", &self.pin(12usize))
            .field("pin[13]", &self.pin(13usize))
            .field("pin[14]", &self.pin(14usize))
            .field("pin[15]", &self.pin(15usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icr {{ pin[0]: {:?}, pin[1]: {:?}, pin[2]: {:?}, pin[3]: {:?}, pin[4]: {:?}, pin[5]: {:?}, pin[6]: {:?}, pin[7]: {:?}, pin[8]: {:?}, pin[9]: {:?}, pin[10]: {:?}, pin[11]: {:?}, pin[12]: {:?}, pin[13]: {:?}, pin[14]: {:?}, pin[15]: {:?} }}",
            self.pin(0usize),
            self.pin(1usize),
            self.pin(2usize),
            self.pin(3usize),
            self.pin(4usize),
            self.pin(5usize),
            self.pin(6usize),
            self.pin(7usize),
            self.pin(8usize),
            self.pin(9usize),
            self.pin(10usize),
            self.pin(11usize),
            self.pin(12usize),
            self.pin(13usize),
            self.pin(14usize),
            self.pin(15usize)
        )
    }
}
#[doc = "GPIO interrupt mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Imr(pub u32);
impl Imr {
    #[must_use]
    #[inline(always)]
    pub const fn imr(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_imr(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Imr {
    #[inline(always)]
    fn default() -> Imr {
        Imr(0)
    }
}
impl core::fmt::Debug for Imr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Imr")
            .field("imr[0]", &self.imr(0usize))
            .field("imr[1]", &self.imr(1usize))
            .field("imr[2]", &self.imr(2usize))
            .field("imr[3]", &self.imr(3usize))
            .field("imr[4]", &self.imr(4usize))
            .field("imr[5]", &self.imr(5usize))
            .field("imr[6]", &self.imr(6usize))
            .field("imr[7]", &self.imr(7usize))
            .field("imr[8]", &self.imr(8usize))
            .field("imr[9]", &self.imr(9usize))
            .field("imr[10]", &self.imr(10usize))
            .field("imr[11]", &self.imr(11usize))
            .field("imr[12]", &self.imr(12usize))
            .field("imr[13]", &self.imr(13usize))
            .field("imr[14]", &self.imr(14usize))
            .field("imr[15]", &self.imr(15usize))
            .field("imr[16]", &self.imr(16usize))
            .field("imr[17]", &self.imr(17usize))
            .field("imr[18]", &self.imr(18usize))
            .field("imr[19]", &self.imr(19usize))
            .field("imr[20]", &self.imr(20usize))
            .field("imr[21]", &self.imr(21usize))
            .field("imr[22]", &self.imr(22usize))
            .field("imr[23]", &self.imr(23usize))
            .field("imr[24]", &self.imr(24usize))
            .field("imr[25]", &self.imr(25usize))
            .field("imr[26]", &self.imr(26usize))
            .field("imr[27]", &self.imr(27usize))
            .field("imr[28]", &self.imr(28usize))
            .field("imr[29]", &self.imr(29usize))
            .field("imr[30]", &self.imr(30usize))
            .field("imr[31]", &self.imr(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Imr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Imr {{ imr[0]: {=bool:?}, imr[1]: {=bool:?}, imr[2]: {=bool:?}, imr[3]: {=bool:?}, imr[4]: {=bool:?}, imr[5]: {=bool:?}, imr[6]: {=bool:?}, imr[7]: {=bool:?}, imr[8]: {=bool:?}, imr[9]: {=bool:?}, imr[10]: {=bool:?}, imr[11]: {=bool:?}, imr[12]: {=bool:?}, imr[13]: {=bool:?}, imr[14]: {=bool:?}, imr[15]: {=bool:?}, imr[16]: {=bool:?}, imr[17]: {=bool:?}, imr[18]: {=bool:?}, imr[19]: {=bool:?}, imr[20]: {=bool:?}, imr[21]: {=bool:?}, imr[22]: {=bool:?}, imr[23]: {=bool:?}, imr[24]: {=bool:?}, imr[25]: {=bool:?}, imr[26]: {=bool:?}, imr[27]: {=bool:?}, imr[28]: {=bool:?}, imr[29]: {=bool:?}, imr[30]: {=bool:?}, imr[31]: {=bool:?} }}",
            self.imr(0usize),
            self.imr(1usize),
            self.imr(2usize),
            self.imr(3usize),
            self.imr(4usize),
            self.imr(5usize),
            self.imr(6usize),
            self.imr(7usize),
            self.imr(8usize),
            self.imr(9usize),
            self.imr(10usize),
            self.imr(11usize),
            self.imr(12usize),
            self.imr(13usize),
            self.imr(14usize),
            self.imr(15usize),
            self.imr(16usize),
            self.imr(17usize),
            self.imr(18usize),
            self.imr(19usize),
            self.imr(20usize),
            self.imr(21usize),
            self.imr(22usize),
            self.imr(23usize),
            self.imr(24usize),
            self.imr(25usize),
            self.imr(26usize),
            self.imr(27usize),
            self.imr(28usize),
            self.imr(29usize),
            self.imr(30usize),
            self.imr(31usize)
        )
    }
}
#[doc = "GPIO interrupt status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isr(pub u32);
impl Isr {
    #[must_use]
    #[inline(always)]
    pub const fn isr(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_isr(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Isr {
    #[inline(always)]
    fn default() -> Isr {
        Isr(0)
    }
}
impl core::fmt::Debug for Isr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isr")
            .field("isr[0]", &self.isr(0usize))
            .field("isr[1]", &self.isr(1usize))
            .field("isr[2]", &self.isr(2usize))
            .field("isr[3]", &self.isr(3usize))
            .field("isr[4]", &self.isr(4usize))
            .field("isr[5]", &self.isr(5usize))
            .field("isr[6]", &self.isr(6usize))
            .field("isr[7]", &self.isr(7usize))
            .field("isr[8]", &self.isr(8usize))
            .field("isr[9]", &self.isr(9usize))
            .field("isr[10]", &self.isr(10usize))
            .field("isr[11]", &self.isr(11usize))
            .field("isr[12]", &self.isr(12usize))
            .field("isr[13]", &self.isr(13usize))
            .field("isr[14]", &self.isr(14usize))
            .field("isr[15]", &self.isr(15usize))
            .field("isr[16]", &self.isr(16usize))
            .field("isr[17]", &self.isr(17usize))
            .field("isr[18]", &self.isr(18usize))
            .field("isr[19]", &self.isr(19usize))
            .field("isr[20]", &self.isr(20usize))
            .field("isr[21]", &self.isr(21usize))
            .field("isr[22]", &self.isr(22usize))
            .field("isr[23]", &self.isr(23usize))
            .field("isr[24]", &self.isr(24usize))
            .field("isr[25]", &self.isr(25usize))
            .field("isr[26]", &self.isr(26usize))
            .field("isr[27]", &self.isr(27usize))
            .field("isr[28]", &self.isr(28usize))
            .field("isr[29]", &self.isr(29usize))
            .field("isr[30]", &self.isr(30usize))
            .field("isr[31]", &self.isr(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Isr {{ isr[0]: {=bool:?}, isr[1]: {=bool:?}, isr[2]: {=bool:?}, isr[3]: {=bool:?}, isr[4]: {=bool:?}, isr[5]: {=bool:?}, isr[6]: {=bool:?}, isr[7]: {=bool:?}, isr[8]: {=bool:?}, isr[9]: {=bool:?}, isr[10]: {=bool:?}, isr[11]: {=bool:?}, isr[12]: {=bool:?}, isr[13]: {=bool:?}, isr[14]: {=bool:?}, isr[15]: {=bool:?}, isr[16]: {=bool:?}, isr[17]: {=bool:?}, isr[18]: {=bool:?}, isr[19]: {=bool:?}, isr[20]: {=bool:?}, isr[21]: {=bool:?}, isr[22]: {=bool:?}, isr[23]: {=bool:?}, isr[24]: {=bool:?}, isr[25]: {=bool:?}, isr[26]: {=bool:?}, isr[27]: {=bool:?}, isr[28]: {=bool:?}, isr[29]: {=bool:?}, isr[30]: {=bool:?}, isr[31]: {=bool:?} }}",
            self.isr(0usize),
            self.isr(1usize),
            self.isr(2usize),
            self.isr(3usize),
            self.isr(4usize),
            self.isr(5usize),
            self.isr(6usize),
            self.isr(7usize),
            self.isr(8usize),
            self.isr(9usize),
            self.isr(10usize),
            self.isr(11usize),
            self.isr(12usize),
            self.isr(13usize),
            self.isr(14usize),
            self.isr(15usize),
            self.isr(16usize),
            self.isr(17usize),
            self.isr(18usize),
            self.isr(19usize),
            self.isr(20usize),
            self.isr(21usize),
            self.isr(22usize),
            self.isr(23usize),
            self.isr(24usize),
            self.isr(25usize),
            self.isr(26usize),
            self.isr(27usize),
            self.isr(28usize),
            self.isr(29usize),
            self.isr(30usize),
            self.isr(31usize)
        )
    }
}
#[doc = "GPIO pad status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psr(pub u32);
impl Psr {
    #[must_use]
    #[inline(always)]
    pub const fn psr(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_psr(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Psr {
    #[inline(always)]
    fn default() -> Psr {
        Psr(0)
    }
}
impl core::fmt::Debug for Psr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Psr")
            .field("psr[0]", &self.psr(0usize))
            .field("psr[1]", &self.psr(1usize))
            .field("psr[2]", &self.psr(2usize))
            .field("psr[3]", &self.psr(3usize))
            .field("psr[4]", &self.psr(4usize))
            .field("psr[5]", &self.psr(5usize))
            .field("psr[6]", &self.psr(6usize))
            .field("psr[7]", &self.psr(7usize))
            .field("psr[8]", &self.psr(8usize))
            .field("psr[9]", &self.psr(9usize))
            .field("psr[10]", &self.psr(10usize))
            .field("psr[11]", &self.psr(11usize))
            .field("psr[12]", &self.psr(12usize))
            .field("psr[13]", &self.psr(13usize))
            .field("psr[14]", &self.psr(14usize))
            .field("psr[15]", &self.psr(15usize))
            .field("psr[16]", &self.psr(16usize))
            .field("psr[17]", &self.psr(17usize))
            .field("psr[18]", &self.psr(18usize))
            .field("psr[19]", &self.psr(19usize))
            .field("psr[20]", &self.psr(20usize))
            .field("psr[21]", &self.psr(21usize))
            .field("psr[22]", &self.psr(22usize))
            .field("psr[23]", &self.psr(23usize))
            .field("psr[24]", &self.psr(24usize))
            .field("psr[25]", &self.psr(25usize))
            .field("psr[26]", &self.psr(26usize))
            .field("psr[27]", &self.psr(27usize))
            .field("psr[28]", &self.psr(28usize))
            .field("psr[29]", &self.psr(29usize))
            .field("psr[30]", &self.psr(30usize))
            .field("psr[31]", &self.psr(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Psr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Psr {{ psr[0]: {=bool:?}, psr[1]: {=bool:?}, psr[2]: {=bool:?}, psr[3]: {=bool:?}, psr[4]: {=bool:?}, psr[5]: {=bool:?}, psr[6]: {=bool:?}, psr[7]: {=bool:?}, psr[8]: {=bool:?}, psr[9]: {=bool:?}, psr[10]: {=bool:?}, psr[11]: {=bool:?}, psr[12]: {=bool:?}, psr[13]: {=bool:?}, psr[14]: {=bool:?}, psr[15]: {=bool:?}, psr[16]: {=bool:?}, psr[17]: {=bool:?}, psr[18]: {=bool:?}, psr[19]: {=bool:?}, psr[20]: {=bool:?}, psr[21]: {=bool:?}, psr[22]: {=bool:?}, psr[23]: {=bool:?}, psr[24]: {=bool:?}, psr[25]: {=bool:?}, psr[26]: {=bool:?}, psr[27]: {=bool:?}, psr[28]: {=bool:?}, psr[29]: {=bool:?}, psr[30]: {=bool:?}, psr[31]: {=bool:?} }}",
            self.psr(0usize),
            self.psr(1usize),
            self.psr(2usize),
            self.psr(3usize),
            self.psr(4usize),
            self.psr(5usize),
            self.psr(6usize),
            self.psr(7usize),
            self.psr(8usize),
            self.psr(9usize),
            self.psr(10usize),
            self.psr(11usize),
            self.psr(12usize),
            self.psr(13usize),
            self.psr(14usize),
            self.psr(15usize),
            self.psr(16usize),
            self.psr(17usize),
            self.psr(18usize),
            self.psr(19usize),
            self.psr(20usize),
            self.psr(21usize),
            self.psr(22usize),
            self.psr(23usize),
            self.psr(24usize),
            self.psr(25usize),
            self.psr(26usize),
            self.psr(27usize),
            self.psr(28usize),
            self.psr(29usize),
            self.psr(30usize),
            self.psr(31usize)
        )
    }
}
