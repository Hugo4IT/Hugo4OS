use alloc::vec::Vec;

// Rendering

#[test_case]
fn check_framebuffer_exists() {
    assert!(unsafe { cpu_renderer::FRAMEBUFFER.is_some() });
}

#[test_case]
fn check_set_background() {
    unsafe {
        cpu_renderer::clear_background();
        for y in 0..cpu_renderer::FB_HEIGHT {
            for x in 0..cpu_renderer::FB_WIDTH {
                assert!(cpu_renderer::get_pixel(x, y) == 0x17171717);
            }
        }
    }
}

#[test_case]
fn check_set_rect() {
    unsafe {
        cpu_renderer::fill_rect(constants::COLORS[constants::RED], 10, 10, 100, 100);
        for y in 10..110 {
            for x in 10..110 {
                assert!(cpu_renderer::get_pixel(x, y) == constants::COLORS[constants::RED]);
            }
        }
    };
}

#[test_case]
fn large_vec() {
    let n = 1000;
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(i);
    }
    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
}