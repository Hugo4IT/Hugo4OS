use core::{alloc::{GlobalAlloc, Layout}, ptr::{self, NonNull}, mem};

use bootloader::boot_info::{MemoryRegions, MemoryRegionKind};
use x86_64::{structures::paging::{PageTable, OffsetPageTable, FrameAllocator, Size4KiB, PhysFrame, mapper::MapToError, Mapper, Page, PageTableFlags}, VirtAddr, PhysAddr};

use crate::{ALLOCATOR, constants::{HEAP_SIZE, HEAP_START, BLOCK_SIZES}, println_verbose};

pub fn init(physical_memory_offset: u64, memory_regions: &'static MemoryRegions) {
    let phys_mem_offset = VirtAddr::new(physical_memory_offset);
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::new(memory_regions) };
    let mut mapper = unsafe { new_page_table(phys_mem_offset) };
    
    // Initialize dynamic managed memory
    init_heap(&mut mapper, &mut frame_allocator).unwrap();
}

/// Wrapper to add trait implementation support.
pub struct Locked<A>(spin::Mutex<A>);
impl<A> Locked<A> {
    pub const fn new(inner: A) -> Locked<A> { Locked(spin::Mutex::new(inner)) }
    fn lock(&self) -> spin::MutexGuard<A> { self.0.lock() }
}

/// Initialize a new OffsetPageTable.
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
pub unsafe fn new_page_table(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);
    
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

/// Returns a mutable reference to the active level 4 table.
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
pub unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();
    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr
}

////////////////////////////////////////////////////////////////////////////////
// FrameAllocator                                                             //
////////////////////////////////////////////////////////////////////////////////

/// A FrameAllocator that always returns `None`.
pub struct EmptyFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        None
    }
}

/// A FrameAllocator that returns usable frames from the bootloader's memory map.
pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryRegions,
    next: usize,
}

impl BootInfoFrameAllocator {
    /// Create a FrameAllocator from the passed memory map.
    ///
    /// This function is unsafe because the caller must guarantee that the passed
    /// memory map is valid. The main requirement is that all frames that are marked
    /// as `USABLE` in it are really unused.
    pub unsafe fn new(memory_map: &'static MemoryRegions) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    /// Returns an iterator over the usable frames specified in the memory map.
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        // get usable regions from memory map
        let regions = self.memory_map.iter();
        let usable_regions = regions
            .filter(|r| r.kind == MemoryRegionKind::Usable);
        // map each region to its address range
        let addr_ranges = usable_regions
            .map(|r| r.start..r.end);
        // transform to an iterator of frame start addresses
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
        // create `PhysFrame` types from the start addresses
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}

pub fn init_heap<M, A>(mapper: &mut M, frame_allocator: &mut A) -> Result<(), MapToError<Size4KiB>>
where
    M: Mapper<Size4KiB>,
    A: FrameAllocator<Size4KiB>,
{
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        let frame = frame_allocator.allocate_frame().ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe {
            mapper.map_to(page, frame, flags, frame_allocator)?.flush();
        };
    }

    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }

    Ok(())
}


////////////////////////////////////////////////////////////////////////////////
// FixedSizeBlockAllocator                                                    //
////////////////////////////////////////////////////////////////////////////////

fn list_index(layout: &Layout) -> Option<usize> {
    let required_block_size = layout.size().max(layout.align());
    BLOCK_SIZES.iter().position(|&s| s >= required_block_size)
}

struct ListNode {
    next: Option<&'static mut ListNode>,
}

pub struct FixedSizeBlockAllocator {
    list_heads: [Option<&'static mut ListNode>; BLOCK_SIZES.len()],
    fallback_allocator: linked_list_allocator::Heap,
}

impl FixedSizeBlockAllocator {
    pub const fn new() -> FixedSizeBlockAllocator {
        const EMPTY: Option<&'static mut ListNode> = None;
        FixedSizeBlockAllocator {
            list_heads: [EMPTY; BLOCK_SIZES.len()],
            fallback_allocator: linked_list_allocator::Heap::empty(),
        }
    }

    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.fallback_allocator.init(heap_start, heap_size)
    }

    fn fallback_alloc(&mut self, layout: Layout) -> *mut u8 {
        match self.fallback_allocator.allocate_first_fit(layout) {
            Ok(ptr) => ptr.as_ptr(),
            Err(_) => ptr::null_mut(),
        }
    }
}

unsafe impl GlobalAlloc for Locked<FixedSizeBlockAllocator> {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let mut allocator = self.lock();
        match list_index(&layout) {
            Some(index) => {
                println_verbose!("Allocating {} bytes using block of size {}.", layout.size(), BLOCK_SIZES[index]);

                match allocator.list_heads[index].take() {
                    Some(node) => {
                        allocator.list_heads[index] = node.next.take();
                        node as *mut ListNode as *mut u8
                    }
                    None => { // If no block of the requested size exists, create it
                        println_verbose!("Block doesn't exist yet, creating one.");

                        let block_size = BLOCK_SIZES[index];
                        let block_align = block_size; // Only works for powers of two
                        let layout = Layout::from_size_align(block_size, block_align).unwrap();
                        allocator.fallback_alloc(layout)
                    }
                }
            }
            None => {
                println_verbose!("Allocating {} bytes using fallback allocator.", layout.size());
                println_verbose!("Heap size: {}, free: {}, after alloc: {}", allocator.fallback_allocator.size(), allocator.fallback_allocator.free(), allocator.fallback_allocator.free() - layout.size());
                allocator.fallback_alloc(layout)
            }
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        let mut allocator = self.lock();
        match list_index(&layout) {
            Some(index) => {
                println_verbose!("Deallocating block of size {} (used: {})", BLOCK_SIZES[index], layout.size());

                let new_node = ListNode {
                    next: allocator.list_heads[index].take(),
                };

                debug_assert!(mem::size_of::<ListNode>() <= BLOCK_SIZES[index]);
                debug_assert!(mem::align_of::<ListNode>() <= BLOCK_SIZES[index]);

                let new_node_ptr = ptr as *mut ListNode;
                new_node_ptr.write(new_node);
                allocator.list_heads[index] = Some(&mut *new_node_ptr);
            }
            None => {
                println_verbose!("Deallocating {} bytes using fallback allocator", layout.size());

                let ptr = NonNull::new(ptr).unwrap();
                allocator.fallback_allocator.deallocate(ptr, layout);
            }
        }
    }
}