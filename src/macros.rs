
macro_rules! glcall {
    ($gl_func:ident($($param:expr),*)) => ({
        use ::gl::*;

        // Debug print the call:
        // println!("{}", stringify!($gl_func));
        // $( println!("\t{} = {:?}", stringify!($param), $param); )*

        let result = unsafe { $gl_func($($param),*) };
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
            print!("OpenGL error: {} ({}) caused by {}", error_type, error, stringify!($gl_func));
            $( println!("\t{:?} ", $param); )*
            println!("at {}:{}", file!(), line!());
        }
        // println!("OpenGL call {:?}", stringify!($call));
        result
    })
}
