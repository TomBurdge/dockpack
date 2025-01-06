use pyo3::prelude::*;
use core_dockpack::cmd_processes::pull::unpack_files::unpack_files_from_image;
use pyo3::exceptions::PyException;


#[pyfunction]
fn hello_from_bin() -> String {
    "Hello from pydockpack!".to_string()
}

#[pyfunction]
fn pull_docker_image(image: String, directory: String) -> PyResult<String> {
    match unpack_files_from_image(&image, &directory) {
        Ok(path) => Ok(path),
        Err(message) => Err(PyException::new_err(message))
    }
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello_from_bin, m)?)?;
    m.add_function(wrap_pyfunction!(pull_docker_image, m)?)?;
    Ok(())
}
