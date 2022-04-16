#[derive(Debug, Clone, Copy)]
#[repr(usize)]
pub enum SyscallId {
    StreamCreate = 0,
    StreamWrite,
    StreamRead,
    StreamFlush,
}