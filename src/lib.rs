#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

use core::ptr;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl Default for ioengine_ops {
    fn default() -> Self {
        Self {
            list: flist_head {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            name: ptr::null(),
            version: FIO_IOOPS_VERSION as i32,
            flags: 0,
            dlhandle: ptr::null_mut(),
            setup: None,
            init: None,
            post_init: None,
            prep: None,
            queue: None,
            commit: None,
            getevents: None,
            event: None,
            errdetails: None,
            cancel: None,
            cleanup: None,
            open_file: None,
            close_file: None,
            invalidate: None,
            unlink_file: None,
            get_file_size: None,
            prepopulate_file: None,
            terminate: None,
            iomem_alloc: None,
            iomem_free: None,
            io_u_init: None,
            io_u_free: None,
            get_zoned_model: None,
            report_zones: None,
            reset_wp: None,
            option_struct_size: 0,
            options: ptr::null_mut(),
        }
    }
}
