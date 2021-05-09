use std::{env, path::PathBuf};

use bindgen::MacroTypeVariation;

fn main() {
    let (fio_path, src) = match env::var("FIO_PATH") {
        Ok(fio) => (PathBuf::from(fio), "FIO_PATH"),
        _ if cfg!(feature = "vendor") => (PathBuf::from("vendor"), "vendored fio source code"),
        _ => {
            let fallback = PathBuf::from("/usr/local/src/fio");
            if !fallback.exists() {
                panic!("FIO_PATH must be specified when not using the `vendor` feature");
            }
            (fallback, "/usr/local/src/fio")
        }
    };

    if !fio_path.join("fio.h").exists() {
        panic!("{} does not contain `fio.h`", src);
    }
    if !fio_path.join("optgroup.h").exists() {
        panic!("{} does not contain `optgroup.h`", src);
    }

    let bindings = bindgen::builder()
        .clang_arg(format!("-I{}", fio_path.to_string_lossy()))
        .header("src/wrapper.h")
        .rustfmt_bindings(true)
        .allowlist_type("thread_data")
        .allowlist_type("io_u")
        .allowlist_type("timespec")
        .allowlist_type("fio_file")
        .allowlist_type("zbd_zoned_model")
        .allowlist_type("zbd_zone")
        .allowlist_type("ioengine_ops")
        .allowlist_type("fio_option")
        .allowlist_type("fio_q_status")
        .allowlist_type("fio_ioengine_flags")
        .allowlist_function("fio_ro_check")
        .default_macro_constant_type(MacroTypeVariation::Signed)
        .constified_enum("fio_ioengine_flags")
        .constified_enum("fio_q_status")
        .prepend_enum_name(false)
        .allowlist_var("FIO_IOOPS_VERSION")
        .allowlist_var("FIO_OPT_C_ENGINE")
        .use_core()
        .ctypes_prefix("cty")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("write bindings");
}
