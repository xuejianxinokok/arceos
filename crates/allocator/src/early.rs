use crate::{AllocError, AllocResult, BaseAllocator, PageAllocator,ByteAllocator};
use core::alloc::Layout;
use core::ptr::NonNull;
/// 题目要求 https://oslearning365.github.io/oscamp_unikernel/ch1-3.html
pub struct EarlyAllocator<const PAGE_SIZE: usize> {
    start: usize,
    byte_pos: usize,
    page_pos: usize,
    end:usize,
    size:usize,
    total_pages: usize,
    used_pages: usize,
}

impl<const PAGE_SIZE: usize> EarlyAllocator<PAGE_SIZE> {
    /// Creates a new empty `EarlyAllocator`.
    pub const fn new() -> Self {
        Self {
            start: 0,
            byte_pos: 0,
            page_pos: 0,
            end:0,
            size:0,
            total_pages:0,
            used_pages:0,
        }
    }
}

impl<const PAGE_SIZE: usize> BaseAllocator for EarlyAllocator<PAGE_SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        assert!(PAGE_SIZE.is_power_of_two());
        //按页对齐
        let end   = super::align_down(start + size, PAGE_SIZE);
        let start = super::align_up(start, PAGE_SIZE);
        self.start = start;
        self.end=end;
        self.byte_pos=start;
        self.page_pos=end;
        self.total_pages = (end - start) / PAGE_SIZE;
        self.size=end-start;
    }

    fn add_memory(&mut self, _start: usize,_size: usize) -> AllocResult {
        Err(AllocError::NoMemory) // unsupported
        //self.init(start, size);
        //Ok(())
    }
}

/// 页分配器
impl<const PAGE_SIZE: usize> PageAllocator for EarlyAllocator<PAGE_SIZE> {
    const PAGE_SIZE: usize = PAGE_SIZE;

    fn alloc_pages(&mut self, num_pages: usize, align_pow2: usize) -> AllocResult<usize> {
        if align_pow2 % PAGE_SIZE != 0 {
            return Err(AllocError::InvalidParam);
        }
        let align_pow2 = align_pow2 / PAGE_SIZE;
        if !align_pow2.is_power_of_two() {
            return Err(AllocError::InvalidParam);
        }

        if self.page_pos>self.byte_pos{
           self.used_pages += num_pages;
           self.page_pos-=num_pages*PAGE_SIZE;
           Ok(self.page_pos)
        }else {
           Err(AllocError::NoMemory)
        }
        
    }

    fn dealloc_pages(&mut self, pos: usize, num_pages: usize) {
        //为了简单,只有与最后位置相同的位置才释放,先不管中间空隙
        if pos==self.page_pos{
          self.used_pages -= num_pages;
          self.page_pos+=num_pages*PAGE_SIZE;
        }
    }

    fn total_pages(&self) -> usize {
        self.total_pages
    }

    fn used_pages(&self) -> usize {
        self.used_pages
    }

    fn available_pages(&self) -> usize {
        (self.page_pos - self.byte_pos)/PAGE_SIZE
    }
}

/// 字节分配器
impl<const PAGE_SIZE: usize> ByteAllocator for  EarlyAllocator<PAGE_SIZE> {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        if self.byte_pos<self.page_pos {
            let old_pos=self.byte_pos;
            self.byte_pos+=layout.size();
            unsafe { Ok( NonNull::new_unchecked(old_pos as *mut u8) )}
        }else {
            Err(AllocError::NoMemory)
        }
    }
 
    fn dealloc(&mut self, pos: NonNull<u8>, layout: Layout) {
        let dealloc_pos= pos.as_ptr() as usize;
        //为了简单,只有与最后位置相同的位置才释放,先不管中间空隙
        if self.byte_pos>self.start && dealloc_pos==self.byte_pos{
            self.byte_pos-=layout.size();
        }
    }

    fn total_bytes(&self) -> usize {
        self.size
    }

    fn used_bytes(&self) -> usize {
        self.byte_pos-self.start
    }

    fn available_bytes(&self) -> usize {
        self.page_pos- self.byte_pos
    }
}
