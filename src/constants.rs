////////////////////////////////////////////////////////////////////////////////
// async/await                                                                //
////////////////////////////////////////////////////////////////////////////////

pub const MAXIMUM_CONCURRENT_TASKS: usize = 100;

////////////////////////////////////////////////////////////////////////////////
// Memory                                                                     //
////////////////////////////////////////////////////////////////////////////////

// Data sizes
#[allow(non_upper_case_globals)] pub const KiB: usize = 1024;
#[allow(non_upper_case_globals)] pub const MiB: usize = 1024 * KiB;
#[allow(non_upper_case_globals)] pub const GiB: usize = 1024 * MiB;
#[allow(non_upper_case_globals)] pub const TiB: usize = 1024 * GiB;

/// Block sizes to use for FixedSizeBlockAllocator. These **must** be powers
/// of 2, because of how the allocator works.
pub const BLOCK_SIZES: &[usize] = &[8, 16, 32, 64, 128, 256, 512, 1024, 2048];

/// Start address for Dynamic Memory
pub const HEAP_START: usize = 0x4444_4444_0000;
/// Size of Dynamic Memory
pub const HEAP_SIZE: usize = 30 * MiB;


////////////////////////////////////////////////////////////////////////////////
// Graphics                                                                   //
////////////////////////////////////////////////////////////////////////////////

pub const RED: usize = 0;
pub const BG: usize = 1;
pub const FG: usize = 2;
pub static mut COLORS: &mut [u32] = &mut [
    0xffda0037,
    0xff171717,
    0xffd3d3d3,
];

pub static FONT_REGULAR: &[u8] = include_bytes!("../res/fonts/Roboto/Roboto-Regular.ttf");
pub static FONT_NERD_MONO: &[u8] = include_bytes!("../res/fonts/JetBrainsMono/JetBrains Mono Regular Nerd Font Complete Mono.ttf");

pub const COLOR_DIV_LOOKUP_TABLE: &[u8; 131072] = include_bytes!("../res/generated/color-div-lookup-table.bin");
pub const COLOR_MULT_LOOKUP_TABLE: &[u8; 131072] = include_bytes!("../res/generated/color-mult-lookup-table.bin");