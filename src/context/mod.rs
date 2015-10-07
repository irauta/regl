
pub mod shared;

use std::rc::Rc;
use ::gl::types::{GLenum,GLint,GLsizei,GLvoid};
use self::shared::{SharedContext,new_shared_context};
use ::id::{Id,IdGenerator,GenerateId};
use ::options::{self,RenderOption};
use ::resource::ResourceCreationSupport;
use ::buffer::BufferCreationSupport;
use ::framebuffer::{self,Framebuffer,FramebufferInternal};
use ::vertex_array::{self,VertexArray,VertexArrayInternal};
use ::program::{Program,ProgramCreationSupport,ProgramInternal};
use ::shader::ShaderCreationSupport;

#[derive(Debug,Clone,Copy)]
pub enum PrimitiveMode {
    Triangles,
}

#[derive(Debug,Clone,Copy)]
pub enum IndexType {
    UByte,
    UShort,
    UInt,
}

pub struct Context {
    id_gen: IdGenerator,
    shared_context: Rc<SharedContext>,
    default_framebuffer: Framebuffer,
    default_vertex_array: Rc<VertexArray>,
    validate_shaders: bool,
}

impl Context {
    pub fn new() -> Context {
        let mut booter = ContextBooter {
            id_gen: IdGenerator::new(),
            shared_context: Rc::new(new_shared_context()),
        };
        let default_framebuffer = framebuffer::create_default_framebuffer(&mut booter);
        let default_vertex_array = Rc::new(vertex_array::create_default_vertex_array(&mut booter).unwrap());

        Context {
            id_gen: booter.id_gen,
            shared_context: booter.shared_context,
            default_framebuffer: default_framebuffer,
            default_vertex_array: default_vertex_array,
            validate_shaders: true,
        }
    }

    pub fn default_framebuffer(&self) -> &Framebuffer {
        &self.default_framebuffer
    }

    pub fn set_option(&self, option: RenderOption) {
        options::set_option(option)
    }

    pub fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        glcall!(Viewport(x, y, width, height));
    }

    pub fn draw(
            &self,
            program: &Program,
            target: &Framebuffer,
            vertex_array: &VertexArray,
            mode: PrimitiveMode, first_vertex: u32, count: u32,
        ) {
        program.bind();
        target.bind();
        vertex_array.bind();
        glcall!(DrawArrays(gl_mode(mode), first_vertex as GLint, count as GLsizei));
    }

    pub fn draw_indexed(
            &self,
            program: &Program,
            target: &Framebuffer,
            vertex_array: &VertexArray,
            mode: PrimitiveMode, index_type: IndexType, base_vertex: u32, count: u32, index_offset: usize
        ) {
        program.bind();
        target.bind();
        vertex_array.bind();
        glcall!(DrawElementsBaseVertex(
            gl_mode(mode),
            count as GLsizei,
            gl_type(index_type),
            index_offset as *const GLvoid,
            base_vertex as GLint
        ));
    }
}

fn gl_mode(mode: PrimitiveMode) -> GLenum {
    match mode {
        PrimitiveMode::Triangles => ::gl::TRIANGLES,
    }
}

fn gl_type(index_type: IndexType) -> GLenum {
    match index_type {
        IndexType::UByte => ::gl::UNSIGNED_BYTE,
        IndexType::UShort => ::gl::UNSIGNED_SHORT,
        IndexType::UInt => ::gl::UNSIGNED_INT,
    }
}

impl GenerateId for Context {
    fn generate_id(&mut self) -> Id {
        self.id_gen.generate_id()
    }
}

impl ResourceCreationSupport for Context {
    fn get_shared_context(&mut self) -> Rc<SharedContext> {
        self.shared_context.clone()
    }
}

impl BufferCreationSupport for Context {
    fn get_default_vertex_array(&mut self) -> Rc<VertexArray> {
        self.default_vertex_array.clone()
    }
}

impl ProgramCreationSupport for Context {
    fn validate_after_linking(&self) -> bool {
        self.validate_shaders
    }
}

impl ShaderCreationSupport for Context {
    fn validate_after_compilation(&self) -> bool {
        self.validate_shaders
    }
}

struct ContextBooter {
    id_gen: IdGenerator,
    shared_context: Rc<SharedContext>,
}

impl GenerateId for ContextBooter {
    fn generate_id(&mut self) -> Id {
        self.id_gen.generate_id()
    }
}

impl ResourceCreationSupport for ContextBooter {
    fn get_shared_context(&mut self) -> Rc<SharedContext> {
        self.shared_context.clone()
    }
}
