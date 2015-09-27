
use ::gl::types::GLenum;

/// Rendering options.
#[derive(Debug,Copy,Clone)]
pub enum RenderOption {
    /// GL_DEPTH_TEST
    DepthTest(bool),
    /// GL_CULL_FACE
    CullingEnabled(bool)
}

pub fn set_option(option: RenderOption) {
    match option {
        RenderOption::DepthTest(enable) => set_capability(::gl::DEPTH_TEST, enable),
        RenderOption::CullingEnabled(enable) => set_capability(::gl::CULL_FACE, enable),
    }
}

fn set_capability(cap: GLenum, enable: bool) {
    if enable {
        glcall!(Enable(cap));
    }
    else {
        glcall!(Disable(cap));
    }
}
