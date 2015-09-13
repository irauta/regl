
macro_rules! glcall {
    ($call:expr) => ({
        use ::gl::*;
        let result = unsafe { $call };
        let error = unsafe { ::gl::GetError() };
        if error != 0 {
            let error_type = match error {
                INVALID_ENUM => "INVALID_ENUM",
                INVALID_VALUE => "INVALID_VALUE",
                INVALID_OPERATION => "INVALID_OPERATION",
                INVALID_FRAMEBUFFER_OPERATION => "INVALID_FRAMEBUFFER_OPERATION",
                OUT_OF_MEMORY => "OUT_OF_MEMORY",
                //gl::STACK_UNDERFLOW => "STACK_UNDERFLOW",
                //gl::STACK_OVERFLOW => "STACK_OVERFLOW",
                _ => "Unrecognized error",
            };
            // Could panic too - except probably not a good idea within drop()
            println!("OpenGL error: {} ({}) caused by {}", error_type, error, stringify!($call));
        }
        // println!("OpenGL call {:?}", stringify!($call));
        result
    })
}
