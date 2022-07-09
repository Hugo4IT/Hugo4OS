use crate::{syscall, ids::SyscallId};

pub unsafe fn stream_create() -> u64 {
    let id = SyscallId::StreamCreate as u64;
    syscall!(id)
}

pub unsafe fn stream_write(stream_id: u64, buffer: u64, len: u64) -> u64 {
    let id = SyscallId::StreamWrite as u64;
    syscall!(id, stream_id, buffer, len)
}

pub unsafe fn stream_read(stream_id: u64, buffer_ptr: u64, count: u64) -> u64 {
    let id = SyscallId::StreamRead as u64;
    syscall!(id, stream_id, buffer_ptr, count)
}