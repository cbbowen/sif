#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "sif_index_niche", repr(transparent))]
#[cfg_attr(
	feature = "sif_index_niche",
	rustc_layout_scalar_valid_range_end(4294967294) // `std::u32::MAX - 1`
)]
pub struct Index(u32);

impl From<usize> for Index {
	#[cfg(feature = "sif_index_niche")]
	fn from(value: usize) -> Self {
		assert!(value < std::u32::MAX as usize);
		unsafe { Index(value as u32) }
	}
	#[cfg(not(feature = "sif_index_niche"))]
	fn from(value: usize) -> Self {
		Index(value as u32)
	}
}

impl Index {
	pub fn index(&self) -> usize {
		self.0 as usize
	}
}

#[cfg(test)]
mod tests {
	#[cfg(feature = "sif_index_niche")]
	#[test]
	fn niche() {
		use std::mem::size_of;
		assert_eq!(size_of::<Option<super::Index>>(), size_of::<super::Index>());
	}
}
