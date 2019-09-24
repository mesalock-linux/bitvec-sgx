/*!

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
	/// If `self` is the empty slice, or wider than an element, then this exits
	/// without effect.
	fn store(&mut self, value: T);
}

impl<T> BitField<T> for BitSlice<LittleEndian, T>
where T: BitStore {
	fn load(&self) -> Option<T> {
		let len = self.len();
		if len == 0 || len > T::BITS as usize {
			return None;
		}

		//  Lazily produce a bitmask with the lowest `len` bits set high.
		let low_mask = || !(T::bits(true) << len as u8);

		match self.bitptr().domain() {
			BitDomain::Empty => None,

			//  The live region of `self` is in the interior of a single
			//  element, touching neither edge.
			BitDomain::Minor(head, elt, _) => {
				//  Load the entire element, shift to align with LSbit, and mask
				Some((elt.load() >> *head) & low_mask())
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
				Some((high | low) & low_mask())
			},

			//  This is equivalent to the `Minor` case, except that the live
			//  region is at the MSbit, so no bits require masking after the
			//  downshift.
			BitDomain::PartialHead(head, elt, _) => Some(elt.load() >> *head),

			//  This is equivalent to the `Minor` case, except that the live
			//  region is at the LSbit, so only the mask is required.
			BitDomain::PartialTail(_, elt, _) => Some(elt.load() & low_mask()),

			//  `self` fills an element, so that element is just copied.
			BitDomain::Spanning(body) => Some(body[0]),
		}
	}

	fn store(&mut self, value: T) {
		let len = self.len();
		if len == 0 || len > T::BITS as usize {
			return;
		}

		//  Produce a mask of the `len` least significant bits in `T`.
		let mask = !(T::bits(true) << len as u8);

		//  Mask away any unusable bits in `value`.
		let value = value & mask;

		match self.bitptr().domain_mut() {
			BitDomainMut::Empty => return,

			BitDomainMut::Minor(head, elt, _) => {
				//  Erase the storage region.
				elt.erase_bits(!(mask << *head));
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
				elt.erase_bits(!mask);
				elt.write_bits(value);
			},

			BitDomainMut::Spanning(body) => body[0] = value,
		}
	}
}

impl<T> BitField<T> for BitSlice<BigEndian, T>
where T: BitStore {
	fn load(&self) -> Option<T> {
		let len = self.len();
		if len == 0 || len > T::BITS as usize {
			return None;
		}

		//  Lazily produce a bitmask with the lowest `len` bits set high.
		let low_mask = || !(T::bits(true) << len as u8);

		match self.bitptr().domain() {
			BitDomain::Empty => None,

			BitDomain::Minor(_, elt, tail) => {
				Some((elt.load() >> T::BITS - *tail) & low_mask())
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
				Some((high | low) & low_mask())
			},

			//  This is equivalent to the `Minor` case, except that the live
			//  region is at the LSbit, so only the mask is required.
			BitDomain::PartialHead(_, elt, _) => Some(elt.load() & low_mask()),

			//  This is equivalent to the `Minor` case, except that the live
			//  region is at the MSbit, so no bits require masking after the
			//  downshift.
			BitDomain::PartialTail(_, elt, tail) => {
				Some(elt.load() >> (T::BITS - *tail))
			},

			BitDomain::Spanning(body) => Some(body[0]),
		}
	}

	fn store(&mut self, value: T) {
		let len = self.len();
		if len == 0 || len > T::BITS as usize {
			return;
		}

		//  Produce a mask of the `len` least significant bits in `T`.
		let mask = !(T::bits(true) << len as u8);

		//  Mask away any unusable bits in `value`.
		let value = value & mask;

		match self.bitptr().domain_mut() {
			BitDomainMut::Empty => return,

			BitDomainMut::Minor(_, elt, tail) => {
				//  Compute the distance between LSedge and the live region.
				let ls_distance = T::BITS - *tail;

				//  Move the value mask away from LSedge to cover the region,
				//  and erase the region bits.
				elt.erase_bits(!(mask << ls_distance));
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
				elt.erase_bits(T::bits(true) << len as u8);
				elt.write_bits(value);
			},

			//  The live region runs from MSbit to `len`.
			BitDomainMut::PartialTail(_, elt, _) => {
				elt.erase_bits(T::bits(true) >> len as u8);
				elt.write_bits(value << (T::BITS - len as u8));
			},

			BitDomainMut::Spanning(body) => body[0] = value,
		}
	}
}
