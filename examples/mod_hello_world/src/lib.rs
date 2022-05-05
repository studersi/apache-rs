extern crate apache_rs;

use apache_rs::APR_HOOK_MIDDLE;
use apache_rs::MODULE_MAGIC_COOKIE;
use apache_rs::MODULE_MAGIC_NUMBER_MAJOR;
use apache_rs::MODULE_MAGIC_NUMBER_MINOR;
use apache_rs::OK;
use apache_rs::ap_hook_handler;
use apache_rs::ap_rprintf;
use apache_rs::ap_set_content_type;
use apache_rs::apr_pool_t;
use apache_rs::command_struct;
use apache_rs::module;
use apache_rs::request_rec;
use std::convert::TryInto;
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::os::raw::c_void;


#[allow(unused_unsafe)]
#[no_mangle]
pub static mut hello_world_module: module =
    module {
        version: MODULE_MAGIC_NUMBER_MAJOR as i32,
        minor_version: MODULE_MAGIC_NUMBER_MINOR as i32,
        module_index: -1,
        name: b"mod_hello_world\x00" as *const u8 as *const c_char,
        dynamic_load_handle: 0 as *mut c_void,
        next: 0 as *mut module,
        magic: MODULE_MAGIC_COOKIE as u64,
        rewrite_args: None,
        create_dir_config: None,
        merge_dir_config: None,
        create_server_config: None,
        merge_server_config: None,
        flags: 0,
        cmds: 0 as *mut command_struct,
        register_hooks: Some(c_hello_world_hooks),
    };

extern "C" fn c_hello_world_hooks(_: *mut apr_pool_t) {
    unsafe {
        ap_hook_handler(Some(c_hello_world_handler),
            std::ptr::null(),
            std::ptr::null(),
            APR_HOOK_MIDDLE.try_into().unwrap());
    };
}

unsafe extern "C" fn c_hello_world_handler(r: *mut request_rec) -> c_int {
    ap_set_content_type(r, b"text/html; charset=utf-8\x00" as *const u8 as *const c_char);
    ap_rprintf(r, b"Hello world!\x00" as *const u8 as *const c_char);
    OK.try_into().unwrap()
}
