use core_dockpack::cmd_processes::pull::unpack_files::unpack_files_from_image;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Unpacks the files from a Docker image into a directory.
///
/// # Arguments
/// * `image` - The name of the Docker image to unpack.
/// * `directory` - The directory to unpack the Docker image into.
///
/// # Returns
/// A C string with the path to the directory where the Docker image files are stored.
/// On error, returns a null pointer.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub async extern "C" fn unpack_files_from_image_c(
    image: *const c_char,
    directory: *const c_char,
) -> *const c_char {
    // Convert C strings to Rust strings
    let image = unsafe { CStr::from_ptr(image).to_string_lossy().into_owned() };
    let directory = unsafe { CStr::from_ptr(directory).to_string_lossy().into_owned() };

    match unpack_files_from_image(&image, &directory).await {
        Ok(path) => {
            let c_string = CString::new(path).unwrap();
            c_string.into_raw() // Return the C string
        }
        Err(err) => {
            eprintln!("Error unpacking image: {}", err);
            std::ptr::null() // Return null on error
        }
    }
}
