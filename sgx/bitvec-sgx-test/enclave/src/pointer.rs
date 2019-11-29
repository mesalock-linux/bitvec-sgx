use bitvec::indices::Indexable;
use bitvec::pointer::*;

//#[test]
pub fn associated_consts_u8() {
	assert_eq!(BitPtr::<u8>::PTR_HEAD_BITS, 0);

	assert_eq!(BitPtr::<u8>::PTR_DATA_MASK, !0);
	assert_eq!(BitPtr::<u8>::PTR_HEAD_MASK, 0);
}

//#[test]
pub fn associated_consts_u16() {
	assert_eq!(BitPtr::<u16>::PTR_HEAD_BITS, 1);

	assert_eq!(BitPtr::<u16>::PTR_DATA_MASK, !0 << 1);
	assert_eq!(BitPtr::<u16>::PTR_HEAD_MASK, 1);
}

//#[test]
pub fn associated_consts_u32() {
	assert_eq!(BitPtr::<u32>::PTR_HEAD_BITS, 2);

	assert_eq!(BitPtr::<u32>::PTR_DATA_MASK, !0 << 2);
	assert_eq!(BitPtr::<u32>::PTR_HEAD_MASK, 3);
}

//#[cfg(target_pointer_width = "64")]
//#[test]
pub fn associated_consts_u64() {
	assert_eq!(BitPtr::<u64>::PTR_HEAD_BITS, 3);

	assert_eq!(BitPtr::<u64>::PTR_DATA_MASK, !0 << 3);
	assert_eq!(BitPtr::<u64>::PTR_HEAD_MASK, 7);
}

//#[test]
pub fn ctors() {
	let data: [u32; 4] = [0; 4];
	let bp = BitPtr::<u32>::new(&data as *const u32, 0u8.idx(), 32 * 4);
	assert_eq!(bp.pointer().r(), &data as *const u32);
	assert_eq!(bp.elements(), 4);
	assert_eq!(*bp.head(), 0);
	assert_eq!(*bp.tail(), 32);
}

//#[test]
pub fn empty() {
	let data = [0u8; 4];
	//  anything with 0 bits is unconditionally empty
	let bp = BitPtr::<u8>::new(&data as *const u8, 2u8.idx(), 0);

	assert_eq!(bp.len(), 0);
	assert_eq!(*bp.head(), 2);
	assert_eq!(*bp.tail(), 2);
}

//#[cfg(not(miri))]
//#[test]
//#[should_panic]
pub fn overfull() {
	BitPtr::<u32>::new(
		8 as *const u32,
		1u8.idx(),
		BitPtr::<u32>::MAX_BITS + 1,
	);
}
