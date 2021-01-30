use core::ffi::c_void;

/// 2D texture
pub struct Texture {
    pub id: gl::types::GLuint,
    pub format: gl::types::GLint,
    pub internal_format: gl::types::GLuint,
}

impl Texture {
    pub fn new(size: (i32, i32), data: &[u8], format: gl::types::GLint, internal_format: gl::types::GLuint) -> Self {
        Self::from_ptr(size, data.as_ptr() as *const c_void, format, internal_format)
    }

    pub fn from_ptr(size: (i32, i32), data: *const c_void, format: gl::types::GLint, internal_format: gl::types::GLuint) -> Self {
        let mut id: gl::types::GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexImage2D(gl::TEXTURE_2D, 0, format, size.0, size.1, 0, internal_format, gl::FLOAT, data);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32); //possibly make the filtering linear?
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        Self {
            id: id,
            format: format,
            internal_format: internal_format,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &mut self.id);
        }
    }
}
