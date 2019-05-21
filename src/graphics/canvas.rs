use crate::graphics::gpu::{self, texture, Gpu};
use crate::graphics::{Target, IntoQuad};
use crate::load::Task;
use crate::Result;

/// An off-screen rendering target.
///
/// It can be used both as a [`Target`] and as a resource.
///
/// [`Target`]: struct.Target.html
#[derive(Clone)]
pub struct Canvas {
    drawable: texture::Drawable,
}

impl Canvas {
    /// Create a new [`Canvas`] with the given size.
    ///
    /// [`Canvas`]: struct.Canvas.html
    pub fn new(gpu: &mut Gpu, width: u16, height: u16) -> Result<Canvas> {
        Ok(Canvas {
            drawable: gpu.create_drawable_texture(width, height),
        })
    }

    /// Create a [`Task`] that produces a new [`Canvas`] with the given size.
    ///
    /// [`Task`]: ../load/struct.Task.html
    /// [`Canvas`]: struct.Canvas.html
    pub fn load(width: u16, height: u16) -> Task<Canvas> {
        Task::using_gpu(move |gpu| Canvas::new(gpu, width, height))
    }

    /// Get the width of the [`Canvas`].
    ///
    /// [`Canvas`]: struct.Canvas.html
    pub fn width(&self) -> u16 {
        self.drawable.texture().width()
    }

    /// Get the height of the [`Canvas`].
    ///
    /// [`Canvas`]: struct.Canvas.html
    pub fn height(&self) -> u16 {
        self.drawable.texture().height()
    }

    /// View the [`Canvas`] as a [`Target`].
    ///
    /// [`Canvas`]: struct.Canvas.html
    /// [`Target`]: struct.Target.html
    pub fn as_target<'a>(&mut self, gpu: &'a mut Gpu) -> Target<'a> {
        let texture = self.drawable.texture();

        Target::with_transformation(
            gpu,
            self.drawable.target().clone(),
            texture.width() as f32,
            texture.height() as f32,
            texture::Drawable::render_transformation(),
        )
    }

    /// Render the [`Canvas`] on the given [`Target`].
    ///
    /// [`Canvas`]: struct.Canvas.html
    /// [`Target`]: struct.Target.html
    pub fn draw<T: IntoQuad>(&self, quad: T, target: &mut Target, x_unit: f32, y_unit: f32) {
        target.draw_texture_quads(
            &self.drawable.texture(),
            &[gpu::Instance::from(quad.into_quad(x_unit, y_unit))],
        );
    }
}

impl std::fmt::Debug for Canvas {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Canvas {{ width: {}, height: {} }}",
            self.width(),
            self.height()
        )
    }
}
