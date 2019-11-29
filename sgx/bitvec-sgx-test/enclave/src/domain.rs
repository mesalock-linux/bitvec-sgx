use std::prelude::v1::*;
use bitvec::pointer::*;
use bitvec::indices::Indexable;

//#[test]
pub fn minor() {
	let data: u8 = 0u8;
	let bp = BitPtr::new(&data, 1u8.idx(), 6);

	assert!(bp.domain().is_minor());
}

//#[test]
pub fn major() {
	let data: &[u16] = &[0u16, !0u16];
	let bp = BitPtr::new(&data[0], 1u8.idx(), 28);

	assert!(bp.domain().is_major());
}

//#[test]
pub fn partial_head() {
	let data: u32 = 0u32;
	let bp = BitPtr::new(&data, 4u8.idx(), 28);

	assert!(bp.domain().is_partial_head());

	let data: &[u32] = &[0u32, !0u32];
	let bp = BitPtr::new(&data[0], 4u8.idx(), 60);

	assert!(bp.domain().is_partial_head());
}

//#[test]
pub fn partial_tail() {
	let data: u32 = 0u32;
	let bp = BitPtr::new(&data, 0u8.idx(), 60);

	assert!(bp.domain().is_partial_tail());

	let data: &[u32] = &[0u32, !0u32];
	let bp = BitPtr::new(&data[0], 0u8.idx(), 60);

	assert!(bp.domain().is_partial_tail());
}

//#[test]
pub fn spanning() {
	let data: u8 = 0u8;
	let bp = BitPtr::new(&data, 0u8.idx(), 8);

	assert!(bp.domain().is_spanning());

	let data: &[u16] = &[0u16, !0u16];
	let bp = BitPtr::new(&data[0], 0u8.idx(), 32);

	assert!(bp.domain().is_spanning());
}
