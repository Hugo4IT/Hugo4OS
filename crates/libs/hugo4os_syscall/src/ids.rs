#[derive(Debug, Clone, Copy)]
#[repr(u64)]
pub enum SyscallId {
    StreamCreate = 0,
    StreamWrite,
    StreamRead,
    StreamFlush,
}