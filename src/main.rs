//! https://www.thespatula.io/rust/rust_io_uring_bindings/

use liburing::{
    io_uring, io_uring_cqe, io_uring_cqe_seen, io_uring_get_sqe, io_uring_prep_nop,
    io_uring_queue_exit, io_uring_queue_init, io_uring_submit, io_uring_wait_cqe,
};
use std::{io, mem::zeroed, ptr::null_mut};

type Result<T = ()> = std::result::Result<T, io::Error>;

fn main() -> Result {
    let queue_depth = 1;
    let mut ring = setup_io_uring(queue_depth)?;

    println!("submit noop operation");
    submit_noop(&mut ring)?;

    println!("submit noop operation");
    wait_for_completion(&mut ring)?;

    unsafe {
        io_uring_queue_exit(&mut ring);
    }

    Ok(())
}

fn setup_io_uring(queue_len: u32) -> Result<io_uring> {
    unsafe {
        let mut ring: io_uring = zeroed();
        let ret = io_uring_queue_init(queue_len, &mut ring, 0);

        if ret < 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(ring)
    }
}

fn submit_noop(ring: &mut io_uring) -> Result {
    unsafe {
        let sqe = io_uring_get_sqe(ring);
        if sqe.is_null() {
            return Err(io::Error::new(io::ErrorKind::Other, "failed to get SQE"));
        }

        io_uring_prep_nop(sqe);
        (*sqe).user_data = 0x7b;

        let ret = io_uring_submit(ring);
        if ret < 0 {
            return Err(io::Error::last_os_error());
        }
    }

    Ok(())
}

fn wait_for_completion(ring: &mut io_uring) -> Result {
    let mut cqe: *mut io_uring_cqe = null_mut();
    let ret = unsafe { io_uring_wait_cqe(ring, &mut cqe) };
    if ret < 0 {
        return Err(io::Error::last_os_error());
    }

    unsafe {
        println!("NOP completed with result: {}", (*cqe).res);
        println!("User data: 0x{:x}", (*cqe).user_data);
        io_uring_cqe_seen(ring, cqe);
    }

    Ok(())
}
