//! Write a graphics backend.
use crate::core::image;
use crate::core::svg;
use crate::core::Size;
use crate::Mesh;

use std::borrow::Cow;

/// The graphics backend of a [`Renderer`].
///
/// [`Renderer`]: crate::Renderer
pub trait Backend {
    /// The compositor of this [`Backend`].
    type Compositor;

    /// The custom kind of primitives this [`Backend`] supports.
    type Primitive: TryFrom<Mesh, Error = &'static str>;
}

/// A graphics backend that supports text rendering.
pub trait Text {
    /// Loads a font from its bytes.
    fn load_font(&mut self, font: Cow<'static, [u8]>);
}

/// A graphics backend that supports image rendering.
pub trait Image {
    /// Returns the dimensions of the provided image.
    fn dimensions(&self, handle: &image::Handle) -> Size<u32>;
}

/// A graphics backend that supports SVG rendering.
pub trait Svg {
    /// Returns the viewport dimensions of the provided SVG.
    fn viewport_dimensions(&self, handle: &svg::Handle) -> Size<u32>;
}
