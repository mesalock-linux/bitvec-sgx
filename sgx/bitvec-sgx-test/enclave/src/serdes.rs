use std::prelude::v1::*;
use bitvec::prelude::*;
use serde_test::{
	Token,
	assert_ser_tokens,
};
//#[cfg(feature = "alloc")]
use serde_test::assert_de_tokens;

macro_rules! bvtok {
	( s $elts:expr, $head:expr, $bits:expr, $ty:ident $( , $data:expr )* ) => {
		&[
			Token::Struct { name: "BitSet", len: 3, },
			Token::Str("head"), Token::U8( $head ),
			Token::Str("bits"), Token::U64( $bits ),
			Token::Str("data"), Token::Seq { len: Some( $elts ) },
			$( Token:: $ty ( $data ), )*
			Token::SeqEnd,
			Token::StructEnd,
		]
	};
	( d $elts:expr, $head:expr, $bits:expr, $ty:ident $( , $data:expr )* ) => {
		&[
			Token::Struct { name: "BitSet", len: 3, },
			Token::BorrowedStr("head"), Token::U8( $head ),
			Token::BorrowedStr("bits"), Token::U64( $bits ),
			Token::BorrowedStr("data"), Token::Seq { len: Some( $elts ) },
			$( Token:: $ty ( $data ), )*
			Token::SeqEnd,
			Token::StructEnd,
		]
	};
}

//#[test]
pub fn empty() {
	let slice = BitSlice::<Msb0, u8>::empty();

	assert_ser_tokens(&slice, bvtok![s 0, 0, 0, U8]);

	//#[cfg(feature = "alloc")]
	assert_de_tokens(&bitvec![], bvtok![ d 0, 0, 0, U8 ]);
}

//#[cfg(feature = "alloc")]
//#[test]
pub fn small() {
    let bits = 0b1111_1000u8.bits::<Msb0>();
    let bits = &bits[1 .. 5];
    assert_ser_tokens(&bits, bvtok![s 1, 1, 4, U8, 0b1111_1000]);

    let bits = 0b00001111_11111111u16.bits::<Lsb0>();
    let bits = &bits[.. 12];
    assert_ser_tokens(&bits, bvtok![s 1, 0, 12, U16, 0b00001111_11111111]);

    let bits = 0b11_11111111u32.bits::<Local>();
    let bits = &bits[.. 10];
    assert_ser_tokens(&bits, bvtok![s 1, 0, 10, U32, 0x00_00_03_FF]);
}

//#[cfg(feature = "alloc")]
//#[test]
pub fn wide() {
    let src: &[u8] = &[0, !0];
    let bs = src.bits::<Local>();
    assert_ser_tokens(&(&bs[1 .. 15]), bvtok![s 2, 1, 14, U8, 0, !0]);
}

//#[cfg(feature = "alloc")]
//#[test]
//#[cfg(feature = "alloc")]
pub fn deser() {
    let bv = bitvec![Msb0, u8; 0, 1, 1, 0, 1, 0];
    assert_de_tokens(&bv, bvtok![d 1, 0, 6, U8, 0b0110_1000]);
    //  test that the bits outside the bits domain don't matter in deser
    assert_de_tokens(&bv, bvtok![d 1, 0, 6, U8, 0b0110_1001]);
    assert_de_tokens(&bv, bvtok![d 1, 0, 6, U8, 0b0110_1010]);
    assert_de_tokens(&bv, bvtok![d 1, 0, 6, U8, 0b0110_1011]);
}
