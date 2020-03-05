/*! Memory description.

This module defines the properties of bare chunks of memory. It deals only with
register-type widths, and has no information on the means by which addressed
memory is accessed.
!*/

use crate::{
	index::BitIdx,
	order::BitOrder,
	store::BitStore,
};

use core::mem;

use funty::IsUnsigned;

use radium::marker::BitOps;

/** Describes properties of register types.

This trait describes raw memory, without any access modifiers. It provides
information about the width of a memory element and useful constants.
**/
pub trait BitMemory: IsUnsigned + BitOps {
	/// The width, in bits, of the memory element.
	const BITS: u8 = mem::size_of::<Self>() as u8 * 8;
	/// The number of bits required to hold a bit index into the element.
	const INDX: u8 = Self::BITS.trailing_zeros() as u8;
	/// The maximum value of a bit index for the element.
	const MASK: u8 = Self::BITS - 1;

	/// The element value with only the least significant bit high.
	const ONE: Self;
	/// The element value with all bits high.
	const ALL: Self;

	/// The element’s name.
	const TYPENAME: &'static str;

	/// Gets a specific bit in an element.
	///
	/// # Safety
	///
	/// This method cannot be called from within an `&BitSlice` context; it may
	/// only be called by construction of an `&Self` reference from a `Self`
	/// element directly.
	///
	/// # Parameters
	///
	/// - `&self`
	/// - `place`: A bit index in the element. The bit under this index, as
	///   governed by the `O` `BitOrder`, will be retrieved as a `bool`.
	///
	/// # Returns
	///
	/// The value of the bit under `place`.
	///
	/// # Type Parameters
	///
	/// - `O`: A `BitOrder` implementation to translate the index into a
	///   position.
	fn get<O>(&self, place: BitIdx<Self>) -> bool
	where O: BitOrder {
		*self & *O::select(place) != Self::ZERO
	}

	/// Sets a specific bit in an element to a given value.
	///
	/// # Safety
	///
	/// This method cannot be called from within an `&mut BitSlice` context; it
	/// may only be called by construction of an `&mut Self` reference from a
	/// `Self` element directly.
	///
	/// # Parameters
	///
	/// - `place`: A bit index in the element. The bit under this index, as
	///   governed by the `O` `BitOrder`, will be set according to `value`.
	///
	/// # Type Parameters
	///
	/// - `O`: A `BitOrder` implementation to translate the index into a
	///   position.
	fn set<O>(&mut self, place: BitIdx<Self>, value: bool)
	where O: BitOrder {
		let sel = *O::select(place);
		if value {
			*self |= sel;
		}
		else {
			*self &= !sel;
		}
	}

	/// Computes the number of elements of `Self` required to hold some bits.
	///
	/// # Parameters
	///
	/// - `bits`: The number of bits to store in an array of `[Self]`.
	///
	/// # Returns
	///
	/// The number of elements of `Self` required to hold the requested bits.
	fn elts(bits: usize) -> usize {
		crate::mem::elts::<Self>(bits)
	}

	#[doc(hidden)]
	fn retype<T>(self) -> T::Mem
	where T: BitStore {
		unsafe { *(&self as *const _ as *const _) }
	}
}

/** Computes the number of elements required to store a number of bits.

# Parameters

- `bits`: The number of bits to store in an element `T` array.

# Returns

The number of elements `T` required to store `bits`.

Because this is a const function, when `bits` is a const-expr, this function can
be used in array types `[T; elts(len)]`.
**/
#[doc(hidden)]
pub const fn elts<T>(bits: usize) -> usize {
	let width = mem::size_of::<T>() * 8;
	bits / width + (bits % width != 0) as usize
}

macro_rules! memory {
	($($t:ty),* $(,)?) => { $(
		impl BitMemory for $t {
			const ONE: Self = 1;
			const ALL: Self = !0;

			const TYPENAME: &'static str = stringify!($t);
		}
	)* };
}

memory!(u8, u16, u32, usize);

#[cfg(target_pointer_width = "64")]
memory!(u64);
