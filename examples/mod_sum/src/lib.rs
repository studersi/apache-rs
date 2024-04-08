extern crate apache_rs;

use apache_rs::ffi::DECLINED;
use apache_rs::ffi::HTTP_FORBIDDEN;
use apache_rs::ffi::HTTP_NOT_FOUND;
use apache_rs::ffi::MODULE_MAGIC_COOKIE;
use apache_rs::ffi::MODULE_MAGIC_NUMBER_MAJOR;
use apache_rs::ffi::MODULE_MAGIC_NUMBER_MINOR;
use apache_rs::ffi::OK;
use apache_rs::ffi::ap_args_to_table;
use apache_rs::ffi::ap_hook_handler;
use apache_rs::ffi::ap_rprintf;
use apache_rs::ffi::ap_set_content_type;
use apache_rs::ffi::apr_pool_t;
use apache_rs::ffi::command_struct;
use apache_rs::ffi::module;
use apache_rs::ffi::request_rec;
use apache_rs::ffi::strcmp;
use apache_rs::ffi::{APR_HOOK_MIDDLE, apr_table_get, apr_table_make};
use apache_rs::{null_c_void, null_command_struct, null_module};
use core::ffi::CStr;
use md5;
use sha1::Digest;
use sha1::Sha1;
use std::convert::TryInto;
use std::ffi::CString;
use std::fs;
use std::os::raw::c_int;
use std::os::raw::c_void;
use std::path::Path;

#[allow(unused_unsafe)]
#[no_mangle]
pub static mut sum_module: module =
    module {
        version: MODULE_MAGIC_NUMBER_MAJOR,
        minor_version: MODULE_MAGIC_NUMBER_MINOR,
        module_index: -1,
        name: c"mod_sum".as_ptr(),
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
        register_hooks: Some(c_sum_hooks),
    };

extern "C" fn c_sum_hooks(_: *mut apr_pool_t) {
    unsafe {
        ap_hook_handler(Some(c_sum_handler),
                        std::ptr::null(),
                        std::ptr::null(),
                        APR_HOOK_MIDDLE.try_into().unwrap());
    };
}

unsafe extern "C" fn c_sum_handler(r: *mut request_rec) -> c_int {

    /* Check that the "example-handler" handler is being called. */
    if (*r).handler == std::ptr::null() || strcmp((*r).handler, c"sum-handler".as_ptr()) != 0 {
        return DECLINED;
    }

    /* Figure out which file is being requested by removing the .sum from it */
    let filename = CStr::from_ptr((*r).filename).to_str().expect("unlikely to fail").to_string();
    let filename_without_sum = filename[..filename.len()-4].to_string(); /* Cut off the last 4 characters. */

    /* Figure out if the file we request a sum on exists and isn't a directory */
    let file_path = Path::new(&filename_without_sum);
    if !file_path.exists() { return HTTP_NOT_FOUND; }
    if !file_path.is_file() { return HTTP_NOT_FOUND; }
    let file_metadata = match fs::metadata(&filename_without_sum) {
        Ok(metadata) => { metadata }
        /* If apr_stat failed, we're probably not allowed to check this file. */
        Err(_) => { return HTTP_FORBIDDEN; }
    };

    /* Parse the GET and, optionally, the POST data sent to us */
    let get = &mut apr_table_make((*r).pool, 0i32);

    ap_args_to_table(r, get);

    // /* Set the appropriate content type */
    ap_set_content_type(r, c"text/html".as_ptr());

    /* Print a title and some general information */
    ap_rprintf(r, CString::new(format!("<h2>Information on {}:</h2>", filename_without_sum)).unwrap().as_ptr());
    ap_rprintf(r, CString::new(format!("<b>Size:</b> {} bytes<br/>", file_metadata.len())).unwrap().as_ptr());

    /* Get the digest type the client wants to see */
    let digest_type_ptr = apr_table_get(*get, c"digest".as_ptr());
    let digest_type = match digest_type_ptr != std::ptr::null() {
        true => CStr::from_ptr(digest_type_ptr),
        false => c"-",
    }.to_str().expect("unlikely to fail");
    ap_rprintf(r, CString::new(format!("<b>digest type:</b> {}<br/>", digest_type)).unwrap().as_ptr());

    let file_content = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let digest = match digest_type.to_lowercase().as_str() {
        "-" => { "no hashing algorithm specified".to_string() }
        "md5" => {
            let digest = md5::compute(file_content.as_bytes());
            format!("{:x}", digest)
        }
        "sha1" => {
            let mut hasher = Sha1::new();
            hasher.update(file_content.as_bytes());
            let result = hasher.finalize();
            format!("{:x}", result)
        }
        &_ => {
            "unknown type, could not calculate has sum".to_string()
        }
    };

    ap_rprintf(r, CString::new(format!("<b>digest:</b> <code>{}</code><br/>", digest)).unwrap().as_ptr());
    ap_rprintf(r, CString::new("<br/><a href='?'>View without hash</a>").unwrap().as_ptr());
    ap_rprintf(r, CString::new("<br/><a href='?digest=md5'>View the MD5 hash</a>").unwrap().as_ptr());
    ap_rprintf(r, CString::new("<br/><a href='?digest=sha1'>View the SHA1 hash</a>").unwrap().as_ptr());
    ap_rprintf(r, CString::new("<br/><a href='?digest=invalid'>View with invalid hash</a>").unwrap().as_ptr());

    /* Let the server know that we responded to this request. */
    return OK;
}
