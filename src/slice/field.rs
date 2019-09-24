/*! Bitfield-style access to `BitSlice` memory regions.

The rest of the crate is built around the concept of single-bit access, and
multiple bits are written into or read out of a slice through iteration. This is
safe and fully general, but is not efficient for a common use case wanted by
what the crate provides: memory access on units smaller than an element, and/or
misaligned from the element boundaries.

Bitfield access cannot be safely generalized across all cursors. It requires
some concept of contiguity in the underlying memory, which each cursor must
implement themselves.

This module provides a trait, `BitField`, which permits single-element
load/store access on arbitrary `BitSlice`s, and implementations of it on the
two cursors `BigEndian` and `LittleEndian` provided by the crate.
!*/

use crate::{
	cursor::{
		BigEndian,
		LittleEndian,
	},
	domain::{
		BitDomain,
		BitDomainMut,
	},
	slice::BitSlice,
	store::{
		BitAccess,
		BitStore,
	},
};

/** Permit a specific `BitSlice` to be used for C-style bitfield access.

Cursors that permit batched access to regions of memory are enabled to load data
from a `BitSlice` and store data to a `BitSlice` with faster behavior than the
default bit-by-bit traversal.

This trait transfers data between a `BitSlice` and an element. The trait
functions always place the live bit region against the least significant bit
edge of the transfer element (return value for `load`, argument for `store`).

Implementations are encouraged to preserve in-memory bit ordering, so that call
sites can provide a value pattern that the user can clearly see matches what
they expect for memory ordering. These methods merely move data from a fixed
location in an element to a variable location in the slice.

Methods should be called as `bits[start .. end].load_or_store()`, where the
range subslice selects up to but no more than the `T::BITS` width.

Without access to the `=` transfer operator, this trait cannot provide native
syntax for access, and must resort to named methods. This is admittedly less
convenient than the equivalent C++ library implementation, but probably a safer
language choice overall.
**/
pub trait BitField<T>
where T: BitStore {
	/// Loads a sequence of bits from `self` into the least-significant bits of
	/// an element.
	///
	/// # Parameters
	///
	/// - `&self`: A read reference to some bits in memory. This slice must have
	///   already been cut down to no more than the width of `T`, using range
	///   indexing from a parent slice to retarget as needed.
	///
	/// # Returns
	///
	/// If `self` has a length greater than zero, and not greater than the width
	/// of `T` in memory, then this returns an element whose least `self.len()`
	/// significant bits are filled with the bits of `self`.
	///
	/// If `self` is the empty slice, or wider than an element, then this
	/// returns `None`.
	fn load(&self) -> Option<T>;

	/// Stores a sequence of bits from the user into the domain of `self`.
	///
	/// # Parameters
	///
	/// - `&mut self`: A write reference to some bits in memory. This slice must
	///   have already been cut down to no more than the width of `T`, using
	///   range indexing from a parent slice to retarget as needed.
	/// - `value`: A user-provided value whose `self.len()` least significant
	///   bits will be stored into `self`.
	///
	/// # Behavior
	///
	/// Implementations are permitted to either exit silently, or panic, when
	/// `self` is either the empty slice or wider than an element.
	fn store(&mut self, value: T);
}

impl<T> BitField<T> for BitSlice<LittleEndian, T>
where T: BitStore {
	/// Reads data out of `self`. See trait documentation.
	///
	/// # Examples
	///
	/// ```rust
	/// use bitvec::prelude::*;
	///
	/// let base = 0xABCDu16;
	/// let bits = base.bits::<LittleEndian>();
	///
	/// assert_eq!(bits[.. 12].load(), Some(0x0BCDu16));
	/// ```
	fn load(&self) -> Option<T> {
		let len = self.len();
		if len == 0 || len > T::BITS as usize {
			return None;
		}

		match self.bitptr().domain() {
			//  The live region of `self` is in the interior of a single
			//  element, touching neither edge.
			BitDomain::Minor(head, elt, _) => {
				//  Load the entire element, shift to align with LSbit, and mask
				Some((elt.load() >> *head) & mask_for(len))
			},

			/* The live region of self is in the interior of a two-element span,
			touching neither outer edge but crossing the interior edge. Because
			of the `len` check above, the middle span is known to not exist, and
			the partially-filled `left` and `right` elements are adjacent.

			Consider this memory layout:

			  [ 7 6 5 4 3 2 1 0 ]
			          v- tail
			1 [ _ _ _ _ h g f e ] <- right
			0 [ d c b a _ _ _ _ ] <- left
			          ^- head

			The load produces the value `0b_hgfedcba` by putting the high bits
			of `left` in the low bits of the output, and the low bits of `right`
			in the middle/high bits of the output.
			*/
			BitDomain::Major(head, left, _, right, _) => {
				//  Get the number of live bits in `left`.
				let mid = T::BITS - *head;
				//  Pull the high bits of `left` out, and put them at LSedge.
				let low = left.load() >> *head;
				//  Pull the low bits of `right`, and lift them above `low`.
				let high = right.load() << mid;
				Some((high | low) & mask_for(len))
			},

			//  This is equivalent to the `Minor` case, except that the live
			//  region is at the MSbit, so no bits require masking after the
			//  downshift.
			BitDomain::PartialHead(head, elt, _) => Some(elt.load() >> *head),

			//  This is equivalent to the `Minor` case, except that the live
			//  region is at the LSbit, so only the mask is required.
			BitDomain::PartialTail(_, elt, _) => Some(elt.load() & mask_for(len)),

			//  `self` fills an element, so that element is just copied.
			BitDomain::Spanning(body) => Some(body[0]),

			//  This is statically unreachable, since the empty case already
			//  returned up above.
			BitDomain::Empty => None,
		}
	}

	/// Writes data into `self`. See trait documentation.
	///
	/// In debug mode, this panics on empty or too-wide slices; in release mode,
	/// it exits without effect.
	///
	/// # Examples
	///
	/// ```rust
	/// use bitvec::prelude::*;
	///
	/// let mut base = 0u16;
	/// let bits = base.bits_mut::<LittleEndian>();
	///
	/// bits[ .. 12].store(0xFABC);
	/// assert_eq!(base, 0x0ABC);
	/// ```
	fn store(&mut self, value: T) {
		let len = self.len();
		if len == 0 || len > T::BITS as usize {
			//  Panic in debug mode.
			#[cfg(debug_assertions)]
			panic!("Cannot store {} bits in a {}-bit region", T::BITS, len);

			//  Exit silently in release mode.
			#[cfg(not(debug_assertions))]
			return;
		}

		//  Mask away any unusable bits in `value`.
		let value = value & mask_for(len);

		match self.bitptr().domain_mut() {
			BitDomainMut::Minor(head, elt, _) => {
				//  Erase the storage region.
				elt.erase_bits(!(mask_for::<T>(len) << *head));
				//  Write the value into the storage region.
				elt.write_bits(value << *head);
			},

			//  See `load` for the memory model in effect here.
			BitDomainMut::Major(head, left, _, right, _) => {
				//  Split the value at `T::BITS - *head`.
				let mid = T::BITS - *head;

				//  Separate the [.. mid] and [mid ..] halves of `value`.
				let low = value & !(T::bits(true) << mid);
				let high = value >> mid;

				//  Erase the high `mid` bits of the left element,
				left.erase_bits(T::bits(true) >> mid);
				//  Then write the low `mid` bits of the value into that slot.
				left.write_bits(low << *head);

				//  Erase the low `len - mid` bits of the right element,
				right.erase_bits(T::bits(true) << (len as u8 - mid));
				//  Then write the high bits of the value into that slot.
				right.write_bits(high);
			},

			//  The live region runs from MSbit to `*head`.
			BitDomainMut::PartialHead(head, elt, _) => {
				elt.erase_bits(T::bits(true) >> len as u8);
				elt.write_bits(value << *head);
			}

			//  The live region is directly at LSbit.
			BitDomainMut::PartialTail(_, elt, _) => {
				elt.erase_bits(!mask_for::<T>(len));
				elt.write_bits(value);
			},

			BitDomainMut::Spanning(body) => body[0] = value,

			BitDomainMut::Empty => return,
		}
	}
}

impl<T> BitField<T> for BitSlice<BigEndian, T>
where T: BitStore {
	/// Reads data out of `self`. See trait documentation.
	///
	/// # Examples
	///
	/// ```rust
	/// use bitvec::prelude::*;
	///
	/// let base = 0xABCDu16;
	/// let bits = base.bits::<BigEndian>();
	///
	/// assert_eq!(bits[.. 12].load(), Some(0x0ABC));
	/// ```
	fn load(&self) -> Option<T> {
		let len = self.len();
		if len == 0 || len > T::BITS as usize {
			return None;
		}

		match self.bitptr().domain() {
			BitDomain::Minor(_, elt, tail) => {
				Some((elt.load() >> T::BITS - *tail) & mask_for(len))
			},

			/* The live region of self is in the interior of a two-element span,
			touching neither outer edge but crossing the interior edge. Because
			of the `len` check above, the middle span is known to not exist, and
			the partially-filled `left` and `right` elements are adjacent.

			Consider this memory layout:

			  [ 0 1 2 3 4 5 6 7 ]
			            v- tail
			1 [ e f g h _ _ _ _ ] <- right
			0 [ _ _ _ _ a b c d ] <- left
			            ^- head

			The load produces the value `0b_abcdefgh` by putting the high bits
			of `right` in the low bits of the output, and the low bits of `left`
			in the middle/high bits of the output.
			*/
			BitDomain::Major(head, left, _, right, tail) => {
				//  There are `width - head` live bits in the LSedge of `left`,
				let left_bits = T::BITS - *head;
				//  And `len - left` live bits in the MSedge of `right`.
				let right_bits = len as u8 - left_bits;

				//  Move `left` from LSedge towards MSedge by the number of live
				//  bits in `right`.
				let high = left.load() << right_bits;
				//  Move `right` from MSedge down to LSedge by the number of
				//  dead bits in `right`.
				let low = right.load() >> T::BITS - *tail;
				//  Recombine and mask.
				Some((high | low) & mask_for(len))
			},

			//  This is equivalent to the `Minor` case, except that the live
			//  region is at the LSbit, so only the mask is required.
			BitDomain::PartialHead(_, elt, _) => Some(elt.load() & mask_for(len)),

			//  This is equivalent to the `Minor` case, except that the live
			//  region is at the MSbit, so no bits require masking after the
			//  downshift.
			BitDomain::PartialTail(_, elt, tail) => {
				Some(elt.load() >> (T::BITS - *tail))
			},

			BitDomain::Spanning(body) => Some(body[0]),

			BitDomain::Empty => None,
		}
	}

	/// Writes data into `self`. See trait documentation.
	///
	/// In debug mode, this panics on empty or too-wide slices; in release mode,
	/// it exits without effect.
	///
	/// # Examples
	///
	/// ```rust
	/// use bitvec::prelude::*;
	///
	/// let mut base = 0u16;
	/// let bits = base.bits_mut::<BigEndian>();
	///
	/// bits[.. 12].store(0xFABC);
	/// assert_eq!(base, 0xABC0);
	/// ```
	fn store(&mut self, value: T) {
		let len = self.len();
		if len == 0 || len > T::BITS as usize {
			//  Panic in debug mode.
			#[cfg(debug_assertions)]
			panic!("Cannot store {} bits in a {}-bit region", T::BITS, len);

			//  Exit silently in release mode.
			#[cfg(not(debug_assertions))]
			return;
		}

		//  Mask away any unusable bits in `value`.
		let value = value & mask_for(len);

		match self.bitptr().domain_mut() {
			BitDomainMut::Minor(_, elt, tail) => {
				//  Compute the distance between LSedge and the live region.
				let ls_distance = T::BITS - *tail;

				//  Move the value mask away from LSedge to cover the region,
				//  and erase the region bits.
				elt.erase_bits(!(mask_for::<T>(len) << ls_distance));
				//  Move the value away from LSedge and write into the region.
				elt.write_bits(value << ls_distance);
			},

			//  See `load` for the memory model in use here.
			BitDomainMut::Major(head, left, _, right, tail) => {
				//  The left element must erase from its LSedge.
				left.erase_bits(!(T::bits(true) >> *head));
				//  The bottom `*tail` bits of the value go in `right`; the
				//  remaining bits are written into the LSedge of `left`.
				left.write_bits(value >> *tail);

				//  The right element must erase from its MSedge.
				right.erase_bits(T::bits(true) >> *tail);
				//  Then upshift the value by the number of dead bits in
				//  `right`, and write into the element.
				right.write_bits(value << (T::BITS - *tail));
			},

			//  The live region is directly at LSbit.
			BitDomainMut::PartialHead(_, elt, _) => {
				elt.erase_bits(!mask_for::<T>(len));
				elt.write_bits(value);
			},

			//  The live region runs from MSbit to `len`.
			BitDomainMut::PartialTail(_, elt, _) => {
				elt.erase_bits(T::bits(true) >> len as u8);
				elt.write_bits(value << (T::BITS - len as u8));
			},

			BitDomainMut::Spanning(body) => body[0] = value,

			BitDomainMut::Empty => return,
		}
	}
}

/// Safely computes an LSedge bitmask for a value of some length.
///
/// Shifting a value for `T::BITS` or more causes overflow panics, which is
/// not ideal when shifting for exactly `T::BITS` is a thing that occasionally
/// happens.
///
/// # Parameters
///
/// - `len`: The width in bits of the value stored in an element `T`.
///
/// # Type Parameters
///
/// - `T`: The `BitStore` type for which the mask is computed.
///
/// # Returns
///
/// An LSedge-aligned bitmask of `len` bits. All bits after the `len`th are
/// zero.
#[inline]
fn mask_for<T>(len: usize) -> T
where T: BitStore {
	let len = len as u8;
	if len >= T::BITS {
		T::bits(true)
	}
	else {
		!(T::bits(true) << len)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::bits::*;

	#[cfg(feature = "std")]
	#[test]
	fn empty() {
		assert!(BitSlice::<BigEndian, u64>::empty().load().is_none());
		assert!(BitSlice::<LittleEndian, u64>::empty().load().is_none());

		//  hit the early-exit branches
		assert!(std::panic::catch_unwind(|| {
			BitSlice::<BigEndian, u32>::empty_mut().store(0);
		}).is_err());
		assert!(std::panic::catch_unwind(|| {
			BitSlice::<LittleEndian, u32>::empty_mut().store(0);
		}).is_err());
	}

	#[test]
	fn bitfields_minor() {
		let mut base = 0u8;
		let bits = base.bits_mut::<LittleEndian>();

		bits[3 .. 7].store(255);
		assert_eq!(bits[3 .. 7].load(), Some(15));

		bits[3 .. 7].store(9);
		assert_eq!(bits[3 .. 7].load(), Some(9));

		assert_eq!(bits.as_slice(), &[0x48]);

		bits.store(0);
		let bits = base.bits_mut::<BigEndian>();

		bits[3 .. 7].store(255);
		assert_eq!(bits[3 .. 7].load(), Some(15));

		bits[3 .. 7].store(6);
		assert_eq!(bits[3 .. 7].load(), Some(6));
	}

	#[test]
	fn bitfields_major() {
		let mut base: [u8; 2] = [0, 0];
		let bits = base.bits_mut::<LittleEndian>();

		bits[5 .. 11].store(255);
		assert_eq!(bits[5 .. 11].load(), Some(63));
		assert_eq!(bits.as_slice(), &[0xE0, 0x07]);

		let bits = base.bits_mut::<BigEndian>();

		bits[5 .. 11].store(63);
		assert_eq!(bits[5 .. 11].load(), Some(63));
		assert_eq!(bits.as_slice(), &[0xE7, 0xE7]);
		bits[.. 3].store(0);
		assert_eq!(bits.as_slice(), &[0x07, 0xE7]);
	}

	#[test]
	fn bitfields_partial_head() {
		let mut base = 0u8;
		let bits = base.bits_mut::<LittleEndian>();

		bits[3 ..].store(255);
		assert_eq!(bits[3 ..].load(), Some(31));
		assert_eq!(bits.as_slice(), &[0xF8]);

		bits.store(0);
		let bits = base.bits_mut::<BigEndian>();

		bits[3 ..].store(255);
		assert_eq!(bits[3 ..].load(), Some(31));
		assert_eq!(bits.as_slice(), &[0x1F]);
	}

	#[test]
	fn bitfields_partial_tail() {
		let mut base = 0u8;
		let bits = base.bits_mut::<LittleEndian>();

		bits[.. 6].store(255);
		assert_eq!(bits[.. 6].load(), Some(63));
		assert_eq!(bits.as_slice(), &[0x3F]);

		bits.store(0);
		let bits = base.bits_mut::<BigEndian>();

		bits[.. 6].store(255);
		assert_eq!(bits[.. 6].load(), Some(63));
		assert_eq!(bits.as_slice(), &[0xFC]);
	}

	#[test]
	fn bitfields_spanning() {
		let mut base = 0u64;

		let bits = base.bits_mut::<LittleEndian>();
		bits.store(0x01234567_89abcdef);
		assert_eq!(bits.load(), Some(0x01234567_89abcdef));

		let bits = base.bits_mut::<BigEndian>();
		bits.store(0x01234567_89abcdef);
		assert_eq!(bits.load(), Some(0x01234567_89abcdef));
	}
}
