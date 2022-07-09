use crate::config::Config;

pub const CONFIG: Config = Config {
    map_physical_memory: true,
    map_page_table_recursively: false,
    map_framebuffer: true,
    aslr: false,
    kernel_stack_size: None,
    physical_memory_offset: None,
    recursive_index: None,
    kernel_stack_address: None,
    boot_info_address: None,
    framebuffer_address: None,
    minimum_framebuffer_height: None,
    minimum_framebuffer_width: None,
};