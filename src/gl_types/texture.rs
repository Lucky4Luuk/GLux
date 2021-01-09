use core::ffi::c_void;

/// 2D texture
pub struct Texture {
    pub id: gl::types::GLuint,
    pub format: gl::types::GLuint,
}

impl Texture {
    pub fn new(size: (i32, i32), data: &[u8], format: gl::types::GLuint) -> Self {
        let mut id: gl::types::GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexImage2D(gl::TEXTURE_2D, 0, format as i32, size.0, size.1, 0, format, gl::UNSIGNED_BYTE, data.as_ptr() as *const c_void);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        Self {
            id: id,
            format: format
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
