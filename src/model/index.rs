#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(sif_index_niche, repr(transparent))]
#[cfg_attr(sif_index_niche, rustc_layout_scalar_valid_range_end(4294967294))] // `std::u32::MAX - 1`
pub struct Index(u32);

impl From<usize> for Index {
	#[cfg(sif_index_niche)]
	fn from(value: usize) -> Self {
		if value >= std::u32::MAX as usize {
			panic!("index out of range");
		}
		unsafe { Key(value as u32) }
	}
	#[cfg(not(sif_index_niche))]
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
	#[cfg(sif_index_niche)]
	#[test]
	fn niche() {
		use std::mem::sizeof;
		assert_eq!(sizeof::<Option<super::index>>(), sizeof::<super::Index>());
	}
}
