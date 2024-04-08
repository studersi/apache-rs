extern crate apache_rs;

use apache_rs::{null_c_void, null_command_struct, null_module};
use apache_rs::ffi::APR_HOOK_MIDDLE;
use apache_rs::ffi::DECLINED;
use apache_rs::ffi::MODULE_MAGIC_COOKIE;
use apache_rs::ffi::MODULE_MAGIC_NUMBER_MAJOR;
use apache_rs::ffi::MODULE_MAGIC_NUMBER_MINOR;
use apache_rs::ffi::OK;
use apache_rs::ffi::ap_hook_handler;
use apache_rs::ffi::ap_rprintf;
use apache_rs::ffi::ap_set_content_type;
use apache_rs::ffi::apr_pool_t;
use apache_rs::ffi::command_struct;
use apache_rs::ffi::module;
use apache_rs::ffi::request_rec;
use apache_rs::ffi::strcmp;
use std::convert::TryInto;
use std::os::raw::c_int;
use std::os::raw::c_void;


#[allow(unused_unsafe)]
#[no_mangle]
pub static mut hello_module: module =
    module {
        version: MODULE_MAGIC_NUMBER_MAJOR,
        minor_version: MODULE_MAGIC_NUMBER_MINOR,
        module_index: -1,
        name: c"mod_hello".as_ptr(),
        dynamic_load_handle: null_c_void!(),
        next: null_module!(),
        magic: MODULE_MAGIC_COOKIE as u64,
        rewrite_args: None,
        create_dir_config: None,
        merge_dir_config: None,
        create_server_config: None,
        merge_server_config: None,
        flags: 0,
        cmds: null_command_struct!(),
        register_hooks: Some(c_hello_hooks),
    };

extern "C" fn c_hello_hooks(_: *mut apr_pool_t) {
    unsafe {
        ap_hook_handler(Some(c_hello_handler),
            std::ptr::null(),
            std::ptr::null(),
            APR_HOOK_MIDDLE.try_into().unwrap());
    };
}

unsafe extern "C" fn c_hello_handler(r: *mut request_rec) -> c_int {
    /* First off, we need to check if this is a call for the "hello-handler" handler.
     * If it is, we accept it and do our things, if not, we simply return DECLINED,
     * and the server will try somewhere else.
     */
    if (*r).handler == std::ptr::null() || strcmp((*r).handler, c"hello-handler".as_ptr()) != 0 {
        return DECLINED;
    }

    /* Now that we are handling this request, we'll write out "Hello, world!" to the client.
     * To do so, we must first set the appropriate content type, followed by our output.
     */
    ap_set_content_type(r, c"text/html; charset=utf-8".as_ptr());
    ap_rprintf(r, c"Hello world!".as_ptr());

    /* Lastly, we must tell the server that we took care of this request and everything went fine.
     * We do so by simply returning the value OK to the server.
     */
    return OK;
}
