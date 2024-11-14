#![no_std]

pub mod extensions;
pub mod instructions;
pub mod state;

pinocchio_pubkey::declare_id!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");

use core::mem::MaybeUninit;

const UNINIT_BYTE: MaybeUninit<u8> = MaybeUninit::<u8>::uninit();

pub unsafe fn from_bytes<T: Clone + Copy>(data: &[u8]) -> T {
    assert_eq!(data.len(), core::mem::size_of::<T>());
    *(data.as_ptr() as *const T)
}

#[inline(always)]
fn write_bytes(destination: &mut [MaybeUninit<u8>], source: &[u8]) {
    for (d, s) in destination.iter_mut().zip(source.iter()) {
        d.write(*s);
    }
}