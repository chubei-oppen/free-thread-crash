use pyo3::prelude::*;

extern "C" {
    static mut PyOS_InputHook: Option<extern "C" fn() -> std::os::raw::c_int>;
}

extern "C" fn input_hook() -> std::os::raw::c_int {
    println!("In input hook");
    pyo3::prepare_freethreaded_python();
    println!("Out of input hook");
    0
}

#[pymodule]
fn free_thread_crash(_py: Python, _m: &PyModule) -> PyResult<()> {
    unsafe {
        PyOS_InputHook = Some(input_hook);
    }

    Ok(())
}
