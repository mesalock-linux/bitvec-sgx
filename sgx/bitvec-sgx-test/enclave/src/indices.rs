use bitvec::prelude::*;
use bitvec::indices::*;

//#[test]
pub fn jump_far_up() {
	//  isize::max_value() is 0x7f...ff, so the result bit will be one less
	//  than the start bit.
	for n in 1 .. 8 {
		let (elt, bit) = n.idx::<u8>().offset(isize::max_value());
		assert_eq!(elt, (isize::max_value() >> u8::INDX) + 1);
		assert_eq!(*bit, n - 1);
	}
	let (elt, bit) = 0u8.idx::<u8>().offset(isize::max_value());
	assert_eq!(elt, isize::max_value() >> u8::INDX);
	assert_eq!(*bit, 7);
}

//#[test]
pub fn jump_far_down() {
	//  isize::min_value() is 0x80...00, so the result bit will be equal to
	//  the start bit
	for n in 0 .. 8 {
		let (elt, bit) = n.idx::<u8>().offset(isize::min_value());
		assert_eq!(elt, isize::min_value() >> u8::INDX);
		assert_eq!(*bit, n);
	}
}
