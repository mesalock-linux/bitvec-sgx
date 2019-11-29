use bitvec::prelude::*;

#[allow(unused_imports)]
use bitvec::order::{
	Lsb0,
	Msb0,
};

//#[test]
//#[cfg(feature = "alloc")]
pub fn compile_bits_macros() {
	bits![0, 1];
	bits![Msb0; 0, 1];
	bits![Lsb0; 0, 1];
	bits![Msb0, u8; 0, 1];
	bits![Lsb0, u8; 0, 1];
	bits![Msb0, u16; 0, 1];
	bits![Lsb0, u16; 0, 1];
	bits![Msb0, u32; 0, 1];
	bits![Lsb0, u32; 0, 1];

	//#[cfg(target_pointer_width = "64")]
	{
		bits![Msb0, u64; 0, 1];
		bits![Lsb0, u64; 0, 1];
	}

	bits![1; 70];
	bits![Msb0; 0; 70];
	bits![Lsb0; 1; 70];
	bits![Msb0, u8; 0; 70];
	bits![Lsb0, u8; 1; 70];
	bits![Msb0, u16; 0; 70];
	bits![Lsb0, u16; 1; 70];
	bits![Msb0, u32; 0; 70];
	bits![Lsb0, u32; 1; 70];

	//#[cfg(target_pointer_width = "64")]
	{
		bits![Msb0, u64; 0; 70];
		bits![Lsb0, u64; 1; 70];
	}
}

//#[test]
//#[cfg(feature = "alloc")]
pub fn compile_bitvec_macros() {
	bitvec![0, 1];
	bitvec![Msb0; 0, 1];
	bitvec![Lsb0; 0, 1];
	bitvec![Msb0, u8; 0, 1];
	bitvec![Lsb0, u8; 0, 1];
	bitvec![Msb0, u16; 0, 1];
	bitvec![Lsb0, u16; 0, 1];
	bitvec![Msb0, u32; 0, 1];
	bitvec![Lsb0, u32; 0, 1];

	//#[cfg(target_pointer_width = "64")]
	{
		bitvec![Msb0, u64; 0, 1];
		bitvec![Lsb0, u64; 0, 1];
	}

	bitvec![1; 70];
	bitvec![Msb0; 0; 70];
	bitvec![Lsb0; 1; 70];
	bitvec![Msb0, u8; 0; 70];
	bitvec![Lsb0, u8; 1; 70];
	bitvec![Msb0, u16; 0; 70];
	bitvec![Lsb0, u16; 1; 70];
	bitvec![Msb0, u32; 0; 70];
	bitvec![Lsb0, u32; 1; 70];

	//#[cfg(target_pointer_width = "64")]
	{
		bitvec![Msb0, u64; 0; 70];
		bitvec![Lsb0, u64; 1; 70];
	}
}

//#[test]
//#[cfg(feature = "alloc")]
pub fn compile_bitbox_macros() {
	bitbox![0, 1];
	bitbox![Msb0; 0, 1];
	bitbox![Lsb0; 0, 1];
	bitbox![Msb0, u8; 0, 1];
	bitbox![Lsb0, u8; 0, 1];
	bitbox![Msb0, u16; 0, 1];
	bitbox![Lsb0, u16; 0, 1];
	bitbox![Msb0, u32; 0, 1];
	bitbox![Lsb0, u32; 0, 1];

	//#[cfg(target_pointer_width = "64")]
	{
		bitbox![Msb0, u64; 0, 1];
		bitbox![Lsb0, u64; 0, 1];
	}

	bitbox![1; 70];
	bitbox![Msb0; 0; 70];
	bitbox![Lsb0; 1; 70];
	bitbox![Msb0, u8; 0; 70];
	bitbox![Lsb0, u8; 1; 70];
	bitbox![Msb0, u16; 0; 70];
	bitbox![Lsb0, u16; 1; 70];
	bitbox![Msb0, u32; 0; 70];
	bitbox![Lsb0, u32; 1; 70];

	//#[cfg(target_pointer_width = "64")]
	{
		bitbox![Msb0, u64; 0; 70];
		bitbox![Lsb0, u64; 1; 70];
	}
}
