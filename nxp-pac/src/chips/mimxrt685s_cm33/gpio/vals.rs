#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Edge(u32);
impl Edge {
    #[doc = "level."]
    pub const Level: Self = Self(0x0);
    #[doc = "edge."]
    pub const Edge: Self = Self(0x01);
}
impl Edge {
    pub const fn from_bits(val: u32) -> Edge {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Edge {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Level"),
            0x01 => f.write_str("Edge"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Edge {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Level"),
            0x01 => defmt::write!(f, "Edge"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Edge {
    #[inline(always)]
    fn from(val: u32) -> Edge {
        Edge::from_bits(val)
    }
}
impl From<Edge> for u32 {
    #[inline(always)]
    fn from(val: Edge) -> u32 {
        Edge::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct PolCtl(u32);
impl PolCtl {
    #[doc = "interrupt when gpio high."]
    pub const Hihg: Self = Self(0x0);
    #[doc = "interrupt when gpio low."]
    pub const Low: Self = Self(0x01);
}
impl PolCtl {
    pub const fn from_bits(val: u32) -> PolCtl {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for PolCtl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Hihg"),
            0x01 => f.write_str("Low"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PolCtl {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Hihg"),
            0x01 => defmt::write!(f, "Low"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for PolCtl {
    #[inline(always)]
    fn from(val: u32) -> PolCtl {
        PolCtl::from_bits(val)
    }
}
impl From<PolCtl> for u32 {
    #[inline(always)]
    fn from(val: PolCtl) -> u32 {
        PolCtl::to_bits(val)
    }
}
