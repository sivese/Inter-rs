use std::ptr::{NonNull, write};

pub type BlockPtr = NonNull<u8>;
pub type BlockSize = usize;

trait AllocRaw {
    fn alloc<T> (&self, object : T) -> *const T;
}

pub enum BlockError {
    BadRequest,
    OOM
}

struct Block {
    ptr  : BlockPtr,
    size : BlockSize
}

impl Block {
    pub fn new(size : BlockSize) -> Result<Block, BlockError> {
        if !size.is_power_of_two() { return Err(BlockError::BadRequest); }

        Ok(Block {
            ptr : internal::alloc_block(size)?,
            size
        })
    }

    pub fn as_ptr(&self) -> *const u8 { self.ptr.as_ptr() }
}

mod internal {
    use std::{alloc::{alloc, dealloc, Layout}, ptr::NonNull};

    use crate::{ BlockPtr, BlockSize, BlockError };

    pub fn alloc_block(size : BlockSize) -> Result<BlockPtr, BlockError> {
        unsafe {
            let layout = Layout::from_size_align_unchecked(size, size);

            let ptr = alloc(layout);

            if ptr.is_null() { Err(BlockError::OOM) }
            else { Ok(NonNull::new_unchecked(ptr)) }
        }
    }

    pub fn dealloc_block(ptr : BlockPtr, size : BlockSize) {
        unsafe {
            let layout = Layout::from_size_align_unchecked(size, size);

            dealloc(ptr.as_ptr(), layout);
        }
    }
}

mod constants {
    pub const BLOCK_SIZE_BITS : usize = 15;
    pub const BLOCK_SIZE : usize = 1 << BLOCK_SIZE_BITS;

    pub const LINE_SIZE_BITS : usize = 7;
    pub const LINE_SIZE : usize = 1 << LINE_SIZE_BITS;
//    pub const LINE_COUNT : usize = BLOCK_SIZE / LINE_SIZE;
}

struct BumpBlock {
    cursor  : usize,
    limit   : usize,
    block   : Block,
    meta    : Box<BlockMeta>
}

impl BumpBlock {
    pub fn inner_alloc(&mut self, alloc_size : usize) -> Option<*const u8> {
        let next_bump = self.cursor + alloc_size;

        if next_bump > constants::BLOCK_SIZE {
            None
        }
        else {
            let offset = self.cursor;
            self.cursor = next_bump;
            unsafe { Some(self.block.as_ptr().add(offset) as *const u8) }
        }
    }
}

unsafe fn write<T> (dest : *const u8, object : T) {
    write(dest as *mut T, object);
}

pub struct BlockMeta {
//    line_mark : [bool; ]
}

/*
    reference : https://rust-hosted-langs.github.io/book/chapter-blocks.html

*/
fn main() {

}