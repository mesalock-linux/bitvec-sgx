// Copyright (C) 2017-2018 Baidu, Inc. All Rights Reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
//  * Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
//  * Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in
//    the documentation and/or other materials provided with the
//    distribution.
//  * Neither the name of Baidu, Inc., nor the names of its
//    contributors may be used to endorse or promote products derived
//    from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

#![crate_name = "helloworldsampleenclave"]
#![crate_type = "staticlib"]

#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

extern crate sgx_types;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;
extern crate sgx_tunittest;

use sgx_types::*;
use std::string::String;
use std::vec::Vec;
use std::io::{self, Write};
use std::slice;
use std::panic;
use sgx_tunittest::*;

extern crate bitvec;
extern crate serde_test;
extern crate serde;

mod serdes;
mod domain;
mod indices;
mod order;
mod macros;
mod pointer;
mod fields;
mod slice_test;

#[no_mangle]
pub extern "C" fn say_something(some_string: *const u8, some_len: usize) -> sgx_status_t {

    let str_slice = unsafe { slice::from_raw_parts(some_string, some_len) };
    let _ = io::stdout().write(str_slice);

    // A sample &'static string
    let rust_raw_string = "This is a in-Enclave ";
    // An array
    let word:[u8;4] = [82, 117, 115, 116];
    // An vector
    let word_vec:Vec<u8> = vec![32, 115, 116, 114, 105, 110, 103, 33];

    // Construct a string from &'static string
    let mut hello_string = String::from(rust_raw_string);

    // Iterate on word array
    for c in word.iter() {
        hello_string.push(*c as char);
    }

    // Rust style convertion
    hello_string += String::from_utf8(word_vec).expect("Invalid UTF-8")
                                               .as_str();

    // Ocall to normal world for output
    println!("{}", &hello_string);

    rsgx_unit_tests!(
serdes::empty,
serdes::small,
serdes::wide,
serdes::deser,
domain::minor,
domain::major,
domain::partial_head,
domain::partial_tail,
domain::spanning,
indices::jump_far_up,
indices::jump_far_down,
//store::incr,
//store::incr_tail,
//store::decr,
//store::decr_tail,
//store::bits,
order::be_u8_range,
order::be_u16_range,
order::be_u32_range,
order::be_u64_range,
order::le_u8_range,
order::le_u16_range,
order::le_u32_range,
order::le_u64_range,
//|| should_panic!(order::be_u8_ovf()),
//|| should_panic!(order::be_u16_ovf()),
//|| should_panic!(order::be_u32_ovf()),
//|| should_panic!(order::be_u64_ovf()),
//|| should_panic!(order::le_u8_ovf()),
//|| should_panic!(order::le_u16_ovf()),
//|| should_panic!(order::le_u32_ovf()),
macros::compile_bits_macros,
macros::compile_bitvec_macros,
macros::compile_bitbox_macros,
pointer::associated_consts_u8,
pointer::associated_consts_u16,
pointer::associated_consts_u32,
pointer::associated_consts_u64,
pointer::ctors,
pointer::empty,
|| should_panic!(pointer::overfull()),
fields::check_mask,
fields::check_resize,
|| should_panic!(fields::bsl08_ll08_empty()),
|| should_panic!(fields::bsl08_ll16_empty()),
|| should_panic!(fields::bsl08_ll32_empty()),
|| should_panic!(fields::bsl08_ll64_empty()),
|| should_panic!(fields::bsl08_lb08_empty()),
|| should_panic!(fields::bsl08_lb16_empty()),
|| should_panic!(fields::bsl08_lb32_empty()),
|| should_panic!(fields::bsl08_lb64_empty()),
|| should_panic!(fields::bsl16_ll08_empty()),
|| should_panic!(fields::bsl16_ll16_empty()),
|| should_panic!(fields::bsl16_ll32_empty()),
|| should_panic!(fields::bsl16_ll64_empty()),
|| should_panic!(fields::bsl16_lb08_empty()),
|| should_panic!(fields::bsl16_lb16_empty()),
|| should_panic!(fields::bsl16_lb32_empty()),
|| should_panic!(fields::bsl16_lb64_empty()),
|| should_panic!(fields::bsl32_ll08_empty()),
|| should_panic!(fields::bsl32_ll16_empty()),
|| should_panic!(fields::bsl32_ll32_empty()),
|| should_panic!(fields::bsl32_ll64_empty()),
|| should_panic!(fields::bsl32_lb08_empty()),
|| should_panic!(fields::bsl32_lb16_empty()),
|| should_panic!(fields::bsl32_lb32_empty()),
|| should_panic!(fields::bsl32_lb64_empty()),
|| should_panic!(fields::bsl64_ll08_empty()),
|| should_panic!(fields::bsl64_ll16_empty()),
|| should_panic!(fields::bsl64_ll32_empty()),
|| should_panic!(fields::bsl64_ll64_empty()),
|| should_panic!(fields::bsl64_lb08_empty()),
|| should_panic!(fields::bsl64_lb16_empty()),
|| should_panic!(fields::bsl64_lb32_empty()),
|| should_panic!(fields::bsl64_lb64_empty()),
|| should_panic!(fields::bsm08_ll08_empty()),
|| should_panic!(fields::bsm08_ll16_empty()),
|| should_panic!(fields::bsm08_ll32_empty()),
|| should_panic!(fields::bsm08_ll64_empty()),
|| should_panic!(fields::bsm08_lb08_empty()),
|| should_panic!(fields::bsm08_lb16_empty()),
|| should_panic!(fields::bsm08_lb32_empty()),
|| should_panic!(fields::bsm08_lb64_empty()),
|| should_panic!(fields::bsm16_ll08_empty()),
|| should_panic!(fields::bsm16_ll16_empty()),
|| should_panic!(fields::bsm16_ll32_empty()),
|| should_panic!(fields::bsm16_ll64_empty()),
|| should_panic!(fields::bsm16_lb08_empty()),
|| should_panic!(fields::bsm16_lb16_empty()),
|| should_panic!(fields::bsm16_lb32_empty()),
|| should_panic!(fields::bsm16_lb64_empty()),
|| should_panic!(fields::bsm32_ll08_empty()),
|| should_panic!(fields::bsm32_ll16_empty()),
|| should_panic!(fields::bsm32_ll32_empty()),
|| should_panic!(fields::bsm32_ll64_empty()),
|| should_panic!(fields::bsm32_lb08_empty()),
|| should_panic!(fields::bsm32_lb16_empty()),
|| should_panic!(fields::bsm32_lb32_empty()),
|| should_panic!(fields::bsm32_lb64_empty()),
|| should_panic!(fields::bsm64_ll08_empty()),
|| should_panic!(fields::bsm64_ll16_empty()),
|| should_panic!(fields::bsm64_ll32_empty()),
|| should_panic!(fields::bsm64_ll64_empty()),
|| should_panic!(fields::bsm64_lb08_empty()),
|| should_panic!(fields::bsm64_lb16_empty()),
|| should_panic!(fields::bsm64_lb32_empty()),
|| should_panic!(fields::bsm64_lb64_empty()),
|| should_panic!(fields::bsl08_ll08_full()),
|| should_panic!(fields::bsl08_ll16_full()),
|| should_panic!(fields::bsl08_ll32_full()),
|| should_panic!(fields::bsl08_ll64_full()),
|| should_panic!(fields::bsl08_lb08_full()),
|| should_panic!(fields::bsl08_lb16_full()),
|| should_panic!(fields::bsl08_lb32_full()),
|| should_panic!(fields::bsl08_lb64_full()),
|| should_panic!(fields::bsl16_ll08_full()),
|| should_panic!(fields::bsl16_ll16_full()),
|| should_panic!(fields::bsl16_ll32_full()),
|| should_panic!(fields::bsl16_ll64_full()),
|| should_panic!(fields::bsl16_lb08_full()),
|| should_panic!(fields::bsl16_lb16_full()),
|| should_panic!(fields::bsl16_lb32_full()),
|| should_panic!(fields::bsl16_lb64_full()),
|| should_panic!(fields::bsl32_ll08_full()),
|| should_panic!(fields::bsl32_ll16_full()),
|| should_panic!(fields::bsl32_ll32_full()),
|| should_panic!(fields::bsl32_ll64_full()),
|| should_panic!(fields::bsl32_lb08_full()),
|| should_panic!(fields::bsl32_lb16_full()),
|| should_panic!(fields::bsl32_lb32_full()),
|| should_panic!(fields::bsl32_lb64_full()),
|| should_panic!(fields::bsl64_ll08_full()),
|| should_panic!(fields::bsl64_ll16_full()),
|| should_panic!(fields::bsl64_ll32_full()),
|| should_panic!(fields::bsl64_ll64_full()),
|| should_panic!(fields::bsl64_lb08_full()),
|| should_panic!(fields::bsl64_lb16_full()),
|| should_panic!(fields::bsl64_lb32_full()),
|| should_panic!(fields::bsl64_lb64_full()),
|| should_panic!(fields::bsm08_ll08_full()),
|| should_panic!(fields::bsm08_ll16_full()),
|| should_panic!(fields::bsm08_ll32_full()),
|| should_panic!(fields::bsm08_ll64_full()),
|| should_panic!(fields::bsm08_lb08_full()),
|| should_panic!(fields::bsm08_lb16_full()),
|| should_panic!(fields::bsm08_lb32_full()),
|| should_panic!(fields::bsm08_lb64_full()),
|| should_panic!(fields::bsm16_ll08_full()),
|| should_panic!(fields::bsm16_ll16_full()),
|| should_panic!(fields::bsm16_ll32_full()),
|| should_panic!(fields::bsm16_ll64_full()),
|| should_panic!(fields::bsm16_lb08_full()),
|| should_panic!(fields::bsm16_lb16_full()),
|| should_panic!(fields::bsm16_lb32_full()),
|| should_panic!(fields::bsm16_lb64_full()),
|| should_panic!(fields::bsm32_ll08_full()),
|| should_panic!(fields::bsm32_ll16_full()),
|| should_panic!(fields::bsm32_ll32_full()),
|| should_panic!(fields::bsm32_ll64_full()),
|| should_panic!(fields::bsm32_lb08_full()),
|| should_panic!(fields::bsm32_lb16_full()),
|| should_panic!(fields::bsm32_lb32_full()),
|| should_panic!(fields::bsm32_lb64_full()),
|| should_panic!(fields::bsm64_ll08_full()),
|| should_panic!(fields::bsm64_ll16_full()),
|| should_panic!(fields::bsm64_ll32_full()),
|| should_panic!(fields::bsm64_ll64_full()),
|| should_panic!(fields::bsm64_lb08_full()),
|| should_panic!(fields::bsm64_lb16_full()),
|| should_panic!(fields::bsm64_lb32_full()),
|| should_panic!(fields::bsm64_lb64_full()),
|| should_panic!(fields::bsl08_sl08_empty()),
|| should_panic!(fields::bsl08_sl16_empty()),
|| should_panic!(fields::bsl08_sl32_empty()),
|| should_panic!(fields::bsl08_sl64_empty()),
|| should_panic!(fields::bsl08_sb08_empty()),
|| should_panic!(fields::bsl08_sb16_empty()),
|| should_panic!(fields::bsl08_sb32_empty()),
|| should_panic!(fields::bsl08_sb64_empty()),
|| should_panic!(fields::bsl16_sl08_empty()),
|| should_panic!(fields::bsl16_sl16_empty()),
|| should_panic!(fields::bsl16_sl32_empty()),
|| should_panic!(fields::bsl16_sl64_empty()),
|| should_panic!(fields::bsl16_sb08_empty()),
|| should_panic!(fields::bsl16_sb16_empty()),
|| should_panic!(fields::bsl16_sb32_empty()),
|| should_panic!(fields::bsl16_sb64_empty()),
|| should_panic!(fields::bsl32_sl08_empty()),
|| should_panic!(fields::bsl32_sl16_empty()),
|| should_panic!(fields::bsl32_sl32_empty()),
|| should_panic!(fields::bsl32_sl64_empty()),
|| should_panic!(fields::bsl32_sb08_empty()),
|| should_panic!(fields::bsl32_sb16_empty()),
|| should_panic!(fields::bsl32_sb32_empty()),
|| should_panic!(fields::bsl32_sb64_empty()),
|| should_panic!(fields::bsl64_sl08_empty()),
|| should_panic!(fields::bsl64_sl16_empty()),
|| should_panic!(fields::bsl64_sl32_empty()),
|| should_panic!(fields::bsl64_sl64_empty()),
|| should_panic!(fields::bsl64_sb08_empty()),
|| should_panic!(fields::bsl64_sb16_empty()),
|| should_panic!(fields::bsl64_sb32_empty()),
|| should_panic!(fields::bsl64_sb64_empty()),
|| should_panic!(fields::bsm08_sl08_empty()),
|| should_panic!(fields::bsm08_sl16_empty()),
|| should_panic!(fields::bsm08_sl32_empty()),
|| should_panic!(fields::bsm08_sl64_empty()),
|| should_panic!(fields::bsm08_sb08_empty()),
|| should_panic!(fields::bsm08_sb16_empty()),
|| should_panic!(fields::bsm08_sb32_empty()),
|| should_panic!(fields::bsm08_sb64_empty()),
|| should_panic!(fields::bsm16_sl08_empty()),
|| should_panic!(fields::bsm16_sl16_empty()),
|| should_panic!(fields::bsm16_sl32_empty()),
|| should_panic!(fields::bsm16_sl64_empty()),
|| should_panic!(fields::bsm16_sb08_empty()),
|| should_panic!(fields::bsm16_sb16_empty()),
|| should_panic!(fields::bsm16_sb32_empty()),
|| should_panic!(fields::bsm16_sb64_empty()),
|| should_panic!(fields::bsm32_sl08_empty()),
|| should_panic!(fields::bsm32_sl16_empty()),
|| should_panic!(fields::bsm32_sl32_empty()),
|| should_panic!(fields::bsm32_sl64_empty()),
|| should_panic!(fields::bsm32_sb08_empty()),
|| should_panic!(fields::bsm32_sb16_empty()),
|| should_panic!(fields::bsm32_sb32_empty()),
|| should_panic!(fields::bsm32_sb64_empty()),
|| should_panic!(fields::bsm64_sl08_empty()),
|| should_panic!(fields::bsm64_sl16_empty()),
|| should_panic!(fields::bsm64_sl32_empty()),
|| should_panic!(fields::bsm64_sl64_empty()),
|| should_panic!(fields::bsm64_sb08_empty()),
|| should_panic!(fields::bsm64_sb16_empty()),
|| should_panic!(fields::bsm64_sb32_empty()),
|| should_panic!(fields::bsm64_sb64_empty()),
|| should_panic!(fields::bsl08_sl08_full()),
|| should_panic!(fields::bsl08_sl16_full()),
|| should_panic!(fields::bsl08_sl32_full()),
|| should_panic!(fields::bsl08_sl64_full()),
|| should_panic!(fields::bsl08_sb08_full()),
|| should_panic!(fields::bsl08_sb16_full()),
|| should_panic!(fields::bsl08_sb32_full()),
|| should_panic!(fields::bsl08_sb64_full()),
|| should_panic!(fields::bsl16_sl08_full()),
|| should_panic!(fields::bsl16_sl16_full()),
|| should_panic!(fields::bsl16_sl32_full()),
|| should_panic!(fields::bsl16_sl64_full()),
|| should_panic!(fields::bsl16_sb08_full()),
|| should_panic!(fields::bsl16_sb16_full()),
|| should_panic!(fields::bsl16_sb32_full()),
|| should_panic!(fields::bsl16_sb64_full()),
|| should_panic!(fields::bsl32_sl08_full()),
|| should_panic!(fields::bsl32_sl16_full()),
|| should_panic!(fields::bsl32_sl32_full()),
|| should_panic!(fields::bsl32_sl64_full()),
|| should_panic!(fields::bsl32_sb08_full()),
|| should_panic!(fields::bsl32_sb16_full()),
|| should_panic!(fields::bsl32_sb32_full()),
|| should_panic!(fields::bsl32_sb64_full()),
|| should_panic!(fields::bsl64_sl08_full()),
|| should_panic!(fields::bsl64_sl16_full()),
|| should_panic!(fields::bsl64_sl32_full()),
|| should_panic!(fields::bsl64_sl64_full()),
|| should_panic!(fields::bsl64_sb08_full()),
|| should_panic!(fields::bsl64_sb16_full()),
|| should_panic!(fields::bsl64_sb32_full()),
|| should_panic!(fields::bsl64_sb64_full()),
|| should_panic!(fields::bsm08_sl08_full()),
|| should_panic!(fields::bsm08_sl16_full()),
|| should_panic!(fields::bsm08_sl32_full()),
|| should_panic!(fields::bsm08_sl64_full()),
|| should_panic!(fields::bsm08_sb08_full()),
|| should_panic!(fields::bsm08_sb16_full()),
|| should_panic!(fields::bsm08_sb32_full()),
|| should_panic!(fields::bsm08_sb64_full()),
|| should_panic!(fields::bsm16_sl08_full()),
|| should_panic!(fields::bsm16_sl16_full()),
|| should_panic!(fields::bsm16_sl32_full()),
|| should_panic!(fields::bsm16_sl64_full()),
|| should_panic!(fields::bsm16_sb08_full()),
|| should_panic!(fields::bsm16_sb16_full()),
|| should_panic!(fields::bsm16_sb32_full()),
|| should_panic!(fields::bsm16_sb64_full()),
|| should_panic!(fields::bsm32_sl08_full()),
|| should_panic!(fields::bsm32_sl16_full()),
|| should_panic!(fields::bsm32_sl32_full()),
|| should_panic!(fields::bsm32_sl64_full()),
|| should_panic!(fields::bsm32_sb08_full()),
|| should_panic!(fields::bsm32_sb16_full()),
|| should_panic!(fields::bsm32_sb32_full()),
|| should_panic!(fields::bsm32_sb64_full()),
|| should_panic!(fields::bsm64_sl08_full()),
|| should_panic!(fields::bsm64_sl16_full()),
|| should_panic!(fields::bsm64_sl32_full()),
|| should_panic!(fields::bsm64_sl64_full()),
|| should_panic!(fields::bsm64_sb08_full()),
|| should_panic!(fields::bsm64_sb16_full()),
|| should_panic!(fields::bsm64_sb32_full()),
|| should_panic!(fields::bsm64_sb64_full()),
fields::lsb0,
fields::msb0,
slice_test::all,
slice_test::any,
slice_test::count_ones,
slice_test::count_zeros,
slice_test::set_all,
);

    sgx_status_t::SGX_SUCCESS
}
