use std::prelude::v1::*;
use bitvec::prelude::*;
use bitvec::indices::Indexable;

//#[test]
pub fn be_u8_range() {
	assert_eq!(Msb0::at::<u8>(0u8.idx()), 7u8.pos());
	assert_eq!(Msb0::at::<u8>(1u8.idx()), 6u8.pos());
	assert_eq!(Msb0::at::<u8>(2u8.idx()), 5u8.pos());
	assert_eq!(Msb0::at::<u8>(3u8.idx()), 4u8.pos());
	assert_eq!(Msb0::at::<u8>(4u8.idx()), 3u8.pos());
	assert_eq!(Msb0::at::<u8>(5u8.idx()), 2u8.pos());
	assert_eq!(Msb0::at::<u8>(6u8.idx()), 1u8.pos());
	assert_eq!(Msb0::at::<u8>(7u8.idx()), 0u8.pos());
}

//#[test]
pub fn be_u16_range() {
	assert_eq!(Msb0::at::<u16>(0u8.idx()), 15u8.pos());
	assert_eq!(Msb0::at::<u16>(1u8.idx()), 14u8.pos());
	assert_eq!(Msb0::at::<u16>(2u8.idx()), 13u8.pos());
	assert_eq!(Msb0::at::<u16>(3u8.idx()), 12u8.pos());
	assert_eq!(Msb0::at::<u16>(4u8.idx()), 11u8.pos());
	assert_eq!(Msb0::at::<u16>(5u8.idx()), 10u8.pos());
	assert_eq!(Msb0::at::<u16>(6u8.idx()), 9u8.pos());
	assert_eq!(Msb0::at::<u16>(7u8.idx()), 8u8.pos());
	assert_eq!(Msb0::at::<u16>(8u8.idx()), 7u8.pos());
	assert_eq!(Msb0::at::<u16>(9u8.idx()), 6u8.pos());
	assert_eq!(Msb0::at::<u16>(10u8.idx()), 5u8.pos());
	assert_eq!(Msb0::at::<u16>(11u8.idx()), 4u8.pos());
	assert_eq!(Msb0::at::<u16>(12u8.idx()), 3u8.pos());
	assert_eq!(Msb0::at::<u16>(13u8.idx()), 2u8.pos());
	assert_eq!(Msb0::at::<u16>(14u8.idx()), 1u8.pos());
	assert_eq!(Msb0::at::<u16>(15u8.idx()), 0u8.pos());
}

//#[test]
pub fn be_u32_range() {
	assert_eq!(Msb0::at::<u32>(0u8.idx()), 31u8.pos());
	assert_eq!(Msb0::at::<u32>(1u8.idx()), 30u8.pos());
	assert_eq!(Msb0::at::<u32>(2u8.idx()), 29u8.pos());
	assert_eq!(Msb0::at::<u32>(3u8.idx()), 28u8.pos());
	assert_eq!(Msb0::at::<u32>(4u8.idx()), 27u8.pos());
	assert_eq!(Msb0::at::<u32>(5u8.idx()), 26u8.pos());
	assert_eq!(Msb0::at::<u32>(6u8.idx()), 25u8.pos());
	assert_eq!(Msb0::at::<u32>(7u8.idx()), 24u8.pos());
	assert_eq!(Msb0::at::<u32>(8u8.idx()), 23u8.pos());
	assert_eq!(Msb0::at::<u32>(9u8.idx()), 22u8.pos());
	assert_eq!(Msb0::at::<u32>(10u8.idx()), 21u8.pos());
	assert_eq!(Msb0::at::<u32>(11u8.idx()), 20u8.pos());
	assert_eq!(Msb0::at::<u32>(12u8.idx()), 19u8.pos());
	assert_eq!(Msb0::at::<u32>(13u8.idx()), 18u8.pos());
	assert_eq!(Msb0::at::<u32>(14u8.idx()), 17u8.pos());
	assert_eq!(Msb0::at::<u32>(15u8.idx()), 16u8.pos());
	assert_eq!(Msb0::at::<u32>(16u8.idx()), 15u8.pos());
	assert_eq!(Msb0::at::<u32>(17u8.idx()), 14u8.pos());
	assert_eq!(Msb0::at::<u32>(18u8.idx()), 13u8.pos());
	assert_eq!(Msb0::at::<u32>(19u8.idx()), 12u8.pos());
	assert_eq!(Msb0::at::<u32>(20u8.idx()), 11u8.pos());
	assert_eq!(Msb0::at::<u32>(21u8.idx()), 10u8.pos());
	assert_eq!(Msb0::at::<u32>(22u8.idx()), 9u8.pos());
	assert_eq!(Msb0::at::<u32>(23u8.idx()), 8u8.pos());
	assert_eq!(Msb0::at::<u32>(24u8.idx()), 7u8.pos());
	assert_eq!(Msb0::at::<u32>(25u8.idx()), 6u8.pos());
	assert_eq!(Msb0::at::<u32>(26u8.idx()), 5u8.pos());
	assert_eq!(Msb0::at::<u32>(27u8.idx()), 4u8.pos());
	assert_eq!(Msb0::at::<u32>(28u8.idx()), 3u8.pos());
	assert_eq!(Msb0::at::<u32>(29u8.idx()), 2u8.pos());
	assert_eq!(Msb0::at::<u32>(30u8.idx()), 1u8.pos());
	assert_eq!(Msb0::at::<u32>(31u8.idx()), 0u8.pos());
}

#[cfg(target_pointer_width = "64")]
//#[test]
pub fn be_u64_range() {
	assert_eq!(Msb0::at::<u64>(0u8.idx()), 63u8.pos());
	assert_eq!(Msb0::at::<u64>(1u8.idx()), 62u8.pos());
	assert_eq!(Msb0::at::<u64>(2u8.idx()), 61u8.pos());
	assert_eq!(Msb0::at::<u64>(3u8.idx()), 60u8.pos());
	assert_eq!(Msb0::at::<u64>(4u8.idx()), 59u8.pos());
	assert_eq!(Msb0::at::<u64>(5u8.idx()), 58u8.pos());
	assert_eq!(Msb0::at::<u64>(6u8.idx()), 57u8.pos());
	assert_eq!(Msb0::at::<u64>(7u8.idx()), 56u8.pos());
	assert_eq!(Msb0::at::<u64>(8u8.idx()), 55u8.pos());
	assert_eq!(Msb0::at::<u64>(9u8.idx()), 54u8.pos());
	assert_eq!(Msb0::at::<u64>(10u8.idx()), 53u8.pos());
	assert_eq!(Msb0::at::<u64>(11u8.idx()), 52u8.pos());
	assert_eq!(Msb0::at::<u64>(12u8.idx()), 51u8.pos());
	assert_eq!(Msb0::at::<u64>(13u8.idx()), 50u8.pos());
	assert_eq!(Msb0::at::<u64>(14u8.idx()), 49u8.pos());
	assert_eq!(Msb0::at::<u64>(15u8.idx()), 48u8.pos());
	assert_eq!(Msb0::at::<u64>(16u8.idx()), 47u8.pos());
	assert_eq!(Msb0::at::<u64>(17u8.idx()), 46u8.pos());
	assert_eq!(Msb0::at::<u64>(18u8.idx()), 45u8.pos());
	assert_eq!(Msb0::at::<u64>(19u8.idx()), 44u8.pos());
	assert_eq!(Msb0::at::<u64>(20u8.idx()), 43u8.pos());
	assert_eq!(Msb0::at::<u64>(21u8.idx()), 42u8.pos());
	assert_eq!(Msb0::at::<u64>(22u8.idx()), 41u8.pos());
	assert_eq!(Msb0::at::<u64>(23u8.idx()), 40u8.pos());
	assert_eq!(Msb0::at::<u64>(24u8.idx()), 39u8.pos());
	assert_eq!(Msb0::at::<u64>(25u8.idx()), 38u8.pos());
	assert_eq!(Msb0::at::<u64>(26u8.idx()), 37u8.pos());
	assert_eq!(Msb0::at::<u64>(27u8.idx()), 36u8.pos());
	assert_eq!(Msb0::at::<u64>(28u8.idx()), 35u8.pos());
	assert_eq!(Msb0::at::<u64>(29u8.idx()), 34u8.pos());
	assert_eq!(Msb0::at::<u64>(30u8.idx()), 33u8.pos());
	assert_eq!(Msb0::at::<u64>(31u8.idx()), 32u8.pos());
	assert_eq!(Msb0::at::<u64>(32u8.idx()), 31u8.pos());
	assert_eq!(Msb0::at::<u64>(33u8.idx()), 30u8.pos());
	assert_eq!(Msb0::at::<u64>(34u8.idx()), 29u8.pos());
	assert_eq!(Msb0::at::<u64>(35u8.idx()), 28u8.pos());
	assert_eq!(Msb0::at::<u64>(36u8.idx()), 27u8.pos());
	assert_eq!(Msb0::at::<u64>(37u8.idx()), 26u8.pos());
	assert_eq!(Msb0::at::<u64>(38u8.idx()), 25u8.pos());
	assert_eq!(Msb0::at::<u64>(39u8.idx()), 24u8.pos());
	assert_eq!(Msb0::at::<u64>(40u8.idx()), 23u8.pos());
	assert_eq!(Msb0::at::<u64>(41u8.idx()), 22u8.pos());
	assert_eq!(Msb0::at::<u64>(42u8.idx()), 21u8.pos());
	assert_eq!(Msb0::at::<u64>(43u8.idx()), 20u8.pos());
	assert_eq!(Msb0::at::<u64>(44u8.idx()), 19u8.pos());
	assert_eq!(Msb0::at::<u64>(45u8.idx()), 18u8.pos());
	assert_eq!(Msb0::at::<u64>(46u8.idx()), 17u8.pos());
	assert_eq!(Msb0::at::<u64>(47u8.idx()), 16u8.pos());
	assert_eq!(Msb0::at::<u64>(48u8.idx()), 15u8.pos());
	assert_eq!(Msb0::at::<u64>(49u8.idx()), 14u8.pos());
	assert_eq!(Msb0::at::<u64>(50u8.idx()), 13u8.pos());
	assert_eq!(Msb0::at::<u64>(51u8.idx()), 12u8.pos());
	assert_eq!(Msb0::at::<u64>(52u8.idx()), 11u8.pos());
	assert_eq!(Msb0::at::<u64>(53u8.idx()), 10u8.pos());
	assert_eq!(Msb0::at::<u64>(54u8.idx()), 9u8.pos());
	assert_eq!(Msb0::at::<u64>(55u8.idx()), 8u8.pos());
	assert_eq!(Msb0::at::<u64>(56u8.idx()), 7u8.pos());
	assert_eq!(Msb0::at::<u64>(57u8.idx()), 6u8.pos());
	assert_eq!(Msb0::at::<u64>(58u8.idx()), 5u8.pos());
	assert_eq!(Msb0::at::<u64>(59u8.idx()), 4u8.pos());
	assert_eq!(Msb0::at::<u64>(60u8.idx()), 3u8.pos());
	assert_eq!(Msb0::at::<u64>(61u8.idx()), 2u8.pos());
	assert_eq!(Msb0::at::<u64>(62u8.idx()), 1u8.pos());
	assert_eq!(Msb0::at::<u64>(63u8.idx()), 0u8.pos());
}

//#[test]
pub fn le_u8_range() {
	assert_eq!(Lsb0::at::<u8>(0u8.idx()), 0u8.pos());
	assert_eq!(Lsb0::at::<u8>(1u8.idx()), 1u8.pos());
	assert_eq!(Lsb0::at::<u8>(2u8.idx()), 2u8.pos());
	assert_eq!(Lsb0::at::<u8>(3u8.idx()), 3u8.pos());
	assert_eq!(Lsb0::at::<u8>(4u8.idx()), 4u8.pos());
	assert_eq!(Lsb0::at::<u8>(5u8.idx()), 5u8.pos());
	assert_eq!(Lsb0::at::<u8>(6u8.idx()), 6u8.pos());
	assert_eq!(Lsb0::at::<u8>(7u8.idx()), 7u8.pos());
}

//#[test]
pub fn le_u16_range() {
	assert_eq!(Lsb0::at::<u16>(0u8.idx()), 0u8.pos());
	assert_eq!(Lsb0::at::<u16>(1u8.idx()), 1u8.pos());
	assert_eq!(Lsb0::at::<u16>(2u8.idx()), 2u8.pos());
	assert_eq!(Lsb0::at::<u16>(3u8.idx()), 3u8.pos());
	assert_eq!(Lsb0::at::<u16>(4u8.idx()), 4u8.pos());
	assert_eq!(Lsb0::at::<u16>(5u8.idx()), 5u8.pos());
	assert_eq!(Lsb0::at::<u16>(6u8.idx()), 6u8.pos());
	assert_eq!(Lsb0::at::<u16>(7u8.idx()), 7u8.pos());
	assert_eq!(Lsb0::at::<u16>(8u8.idx()), 8u8.pos());
	assert_eq!(Lsb0::at::<u16>(9u8.idx()), 9u8.pos());
	assert_eq!(Lsb0::at::<u16>(10u8.idx()), 10u8.pos());
	assert_eq!(Lsb0::at::<u16>(11u8.idx()), 11u8.pos());
	assert_eq!(Lsb0::at::<u16>(12u8.idx()), 12u8.pos());
	assert_eq!(Lsb0::at::<u16>(13u8.idx()), 13u8.pos());
	assert_eq!(Lsb0::at::<u16>(14u8.idx()), 14u8.pos());
	assert_eq!(Lsb0::at::<u16>(15u8.idx()), 15u8.pos());
}

//#[test]
pub fn le_u32_range() {
	assert_eq!(Lsb0::at::<u32>(0u8.idx()), 0u8.pos());
	assert_eq!(Lsb0::at::<u32>(1u8.idx()), 1u8.pos());
	assert_eq!(Lsb0::at::<u32>(2u8.idx()), 2u8.pos());
	assert_eq!(Lsb0::at::<u32>(3u8.idx()), 3u8.pos());
	assert_eq!(Lsb0::at::<u32>(4u8.idx()), 4u8.pos());
	assert_eq!(Lsb0::at::<u32>(5u8.idx()), 5u8.pos());
	assert_eq!(Lsb0::at::<u32>(6u8.idx()), 6u8.pos());
	assert_eq!(Lsb0::at::<u32>(7u8.idx()), 7u8.pos());
	assert_eq!(Lsb0::at::<u32>(8u8.idx()), 8u8.pos());
	assert_eq!(Lsb0::at::<u32>(9u8.idx()), 9u8.pos());
	assert_eq!(Lsb0::at::<u32>(10u8.idx()), 10u8.pos());
	assert_eq!(Lsb0::at::<u32>(11u8.idx()), 11u8.pos());
	assert_eq!(Lsb0::at::<u32>(12u8.idx()), 12u8.pos());
	assert_eq!(Lsb0::at::<u32>(13u8.idx()), 13u8.pos());
	assert_eq!(Lsb0::at::<u32>(14u8.idx()), 14u8.pos());
	assert_eq!(Lsb0::at::<u32>(15u8.idx()), 15u8.pos());
	assert_eq!(Lsb0::at::<u32>(16u8.idx()), 16u8.pos());
	assert_eq!(Lsb0::at::<u32>(17u8.idx()), 17u8.pos());
	assert_eq!(Lsb0::at::<u32>(18u8.idx()), 18u8.pos());
	assert_eq!(Lsb0::at::<u32>(19u8.idx()), 19u8.pos());
	assert_eq!(Lsb0::at::<u32>(20u8.idx()), 20u8.pos());
	assert_eq!(Lsb0::at::<u32>(21u8.idx()), 21u8.pos());
	assert_eq!(Lsb0::at::<u32>(22u8.idx()), 22u8.pos());
	assert_eq!(Lsb0::at::<u32>(23u8.idx()), 23u8.pos());
	assert_eq!(Lsb0::at::<u32>(24u8.idx()), 24u8.pos());
	assert_eq!(Lsb0::at::<u32>(25u8.idx()), 25u8.pos());
	assert_eq!(Lsb0::at::<u32>(26u8.idx()), 26u8.pos());
	assert_eq!(Lsb0::at::<u32>(27u8.idx()), 27u8.pos());
	assert_eq!(Lsb0::at::<u32>(28u8.idx()), 28u8.pos());
	assert_eq!(Lsb0::at::<u32>(29u8.idx()), 29u8.pos());
	assert_eq!(Lsb0::at::<u32>(30u8.idx()), 30u8.pos());
	assert_eq!(Lsb0::at::<u32>(31u8.idx()), 31u8.pos());
}

//#[cfg(target_pointer_width = "64")]
//#[test]
pub fn le_u64_range() {
	assert_eq!(Lsb0::at::<u64>(0u8.idx()), 0u8.pos());
	assert_eq!(Lsb0::at::<u64>(1u8.idx()), 1u8.pos());
	assert_eq!(Lsb0::at::<u64>(2u8.idx()), 2u8.pos());
	assert_eq!(Lsb0::at::<u64>(3u8.idx()), 3u8.pos());
	assert_eq!(Lsb0::at::<u64>(4u8.idx()), 4u8.pos());
	assert_eq!(Lsb0::at::<u64>(5u8.idx()), 5u8.pos());
	assert_eq!(Lsb0::at::<u64>(6u8.idx()), 6u8.pos());
	assert_eq!(Lsb0::at::<u64>(7u8.idx()), 7u8.pos());
	assert_eq!(Lsb0::at::<u64>(8u8.idx()), 8u8.pos());
	assert_eq!(Lsb0::at::<u64>(9u8.idx()), 9u8.pos());
	assert_eq!(Lsb0::at::<u64>(10u8.idx()), 10u8.pos());
	assert_eq!(Lsb0::at::<u64>(11u8.idx()), 11u8.pos());
	assert_eq!(Lsb0::at::<u64>(12u8.idx()), 12u8.pos());
	assert_eq!(Lsb0::at::<u64>(13u8.idx()), 13u8.pos());
	assert_eq!(Lsb0::at::<u64>(14u8.idx()), 14u8.pos());
	assert_eq!(Lsb0::at::<u64>(15u8.idx()), 15u8.pos());
	assert_eq!(Lsb0::at::<u64>(16u8.idx()), 16u8.pos());
	assert_eq!(Lsb0::at::<u64>(17u8.idx()), 17u8.pos());
	assert_eq!(Lsb0::at::<u64>(18u8.idx()), 18u8.pos());
	assert_eq!(Lsb0::at::<u64>(19u8.idx()), 19u8.pos());
	assert_eq!(Lsb0::at::<u64>(20u8.idx()), 20u8.pos());
	assert_eq!(Lsb0::at::<u64>(21u8.idx()), 21u8.pos());
	assert_eq!(Lsb0::at::<u64>(22u8.idx()), 22u8.pos());
	assert_eq!(Lsb0::at::<u64>(23u8.idx()), 23u8.pos());
	assert_eq!(Lsb0::at::<u64>(24u8.idx()), 24u8.pos());
	assert_eq!(Lsb0::at::<u64>(25u8.idx()), 25u8.pos());
	assert_eq!(Lsb0::at::<u64>(26u8.idx()), 26u8.pos());
	assert_eq!(Lsb0::at::<u64>(27u8.idx()), 27u8.pos());
	assert_eq!(Lsb0::at::<u64>(28u8.idx()), 28u8.pos());
	assert_eq!(Lsb0::at::<u64>(29u8.idx()), 29u8.pos());
	assert_eq!(Lsb0::at::<u64>(30u8.idx()), 30u8.pos());
	assert_eq!(Lsb0::at::<u64>(31u8.idx()), 31u8.pos());
	assert_eq!(Lsb0::at::<u64>(32u8.idx()), 32u8.pos());
	assert_eq!(Lsb0::at::<u64>(33u8.idx()), 33u8.pos());
	assert_eq!(Lsb0::at::<u64>(34u8.idx()), 34u8.pos());
	assert_eq!(Lsb0::at::<u64>(35u8.idx()), 35u8.pos());
	assert_eq!(Lsb0::at::<u64>(36u8.idx()), 36u8.pos());
	assert_eq!(Lsb0::at::<u64>(37u8.idx()), 37u8.pos());
	assert_eq!(Lsb0::at::<u64>(38u8.idx()), 38u8.pos());
	assert_eq!(Lsb0::at::<u64>(39u8.idx()), 39u8.pos());
	assert_eq!(Lsb0::at::<u64>(40u8.idx()), 40u8.pos());
	assert_eq!(Lsb0::at::<u64>(41u8.idx()), 41u8.pos());
	assert_eq!(Lsb0::at::<u64>(42u8.idx()), 42u8.pos());
	assert_eq!(Lsb0::at::<u64>(43u8.idx()), 43u8.pos());
	assert_eq!(Lsb0::at::<u64>(44u8.idx()), 44u8.pos());
	assert_eq!(Lsb0::at::<u64>(45u8.idx()), 45u8.pos());
	assert_eq!(Lsb0::at::<u64>(46u8.idx()), 46u8.pos());
	assert_eq!(Lsb0::at::<u64>(47u8.idx()), 47u8.pos());
	assert_eq!(Lsb0::at::<u64>(48u8.idx()), 48u8.pos());
	assert_eq!(Lsb0::at::<u64>(49u8.idx()), 49u8.pos());
	assert_eq!(Lsb0::at::<u64>(50u8.idx()), 50u8.pos());
	assert_eq!(Lsb0::at::<u64>(51u8.idx()), 51u8.pos());
	assert_eq!(Lsb0::at::<u64>(52u8.idx()), 52u8.pos());
	assert_eq!(Lsb0::at::<u64>(53u8.idx()), 53u8.pos());
	assert_eq!(Lsb0::at::<u64>(54u8.idx()), 54u8.pos());
	assert_eq!(Lsb0::at::<u64>(55u8.idx()), 55u8.pos());
	assert_eq!(Lsb0::at::<u64>(56u8.idx()), 56u8.pos());
	assert_eq!(Lsb0::at::<u64>(57u8.idx()), 57u8.pos());
	assert_eq!(Lsb0::at::<u64>(58u8.idx()), 58u8.pos());
	assert_eq!(Lsb0::at::<u64>(59u8.idx()), 59u8.pos());
	assert_eq!(Lsb0::at::<u64>(60u8.idx()), 60u8.pos());
	assert_eq!(Lsb0::at::<u64>(61u8.idx()), 61u8.pos());
	assert_eq!(Lsb0::at::<u64>(62u8.idx()), 62u8.pos());
	assert_eq!(Lsb0::at::<u64>(63u8.idx()), 63u8.pos());
}
