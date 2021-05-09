use cty::{c_int, c_uint};
use fio_ioengine_sys::*;
use std::ptr;

#[no_mangle]
pub unsafe extern "C" fn get_ioengine(ops: *mut *const ioengine_ops) {
    *ops = Box::leak(Box::new(ioengine_ops {
        name: "hello_null\0".as_ptr() as *const i8,
        init: Some(init),
        open_file: Some(open_file),
        queue: Some(queue),
        commit: Some(commit),
        getevents: Some(getevents),
        event: Some(event),
        cleanup: Some(cleanup),
        flags: (FIO_DISKLESSIO | FIO_FAKEIO) as i32,
        ..ioengine_ops::default()
    })) as *const ioengine_ops;
}

struct NullData {
    io_us: Box<[*mut io_u]>,
    queued: c_int,
    events: c_int,
}

impl NullData {
    pub fn with_capacity(capacity: usize) -> Self {
        // Should use `Box::new_zeroed_slice` when it is available on stable.
        let mut io_us = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            io_us.push(ptr::null_mut());
        }
        Self {
            io_us: io_us.into_boxed_slice(),
            queued: 0,
            events: 0,
        }
    }
}

unsafe extern "C" fn init(td: *mut thread_data) -> c_int {
    let io_ops = Box::new(NullData::with_capacity((*td).o.iodepth as usize));
    (*td).io_ops_data = Box::leak(io_ops) as *mut NullData as *mut _;
    0
}

unsafe extern "C" fn open_file(_td: *mut thread_data, _file: *mut fio_file) -> c_int {
    0
}

unsafe extern "C" fn queue(td: *mut thread_data, io_u: *mut io_u) -> fio_q_status {
    // TODO: How to bindgen inline functions?
    // fio_ro_check(td, io_u);
    if ((*(*td).io_ops).flags & FIO_SYNCIO as i32) != 0 {
        return FIO_Q_COMPLETED;
    }
    let nd = (*td).io_ops_data as *mut NullData;
    if (*nd).events != 0 {
        return FIO_Q_BUSY;
    }
    (*nd).io_us[(*nd).queued as usize] = io_u;
    (*nd).queued += 1;
    FIO_Q_QUEUED
}

unsafe extern "C" fn commit(td: *mut thread_data) -> c_int {
    let nd = (*td).io_ops_data as *mut NullData;
    if (*nd).events == 0 {
        (*nd).events = (*nd).queued;
        (*nd).queued = 0;
    }
    0
}

unsafe extern "C" fn getevents(
    td: *mut thread_data,
    min_events: c_uint,
    _max_events: c_uint,
    _time: *const timespec,
) -> c_int {
    let nd = (*td).io_ops_data as *mut NullData;
    let mut ret = 0;
    if min_events != 0 {
        ret = (*nd).events;
        (*nd).events = 0;
    }
    ret
}

unsafe extern "C" fn event(td: *mut thread_data, event: c_int) -> *mut io_u {
    let nd = (*td).io_ops_data as *mut NullData;
    (*nd).io_us[event as usize]
}

unsafe extern "C" fn cleanup(td: *mut thread_data) {
    let nd = (*td).io_ops_data as *mut NullData;
    if !nd.is_null() {
        ptr::drop_in_place(nd)
    }
}
