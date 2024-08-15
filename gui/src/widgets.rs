
pub mod letter_box {
  // For now, to implement a custom native widget you will need to add
  // `iced_native` and `iced_wgpu` to your dependencies.
  //
  // Then, you simply need to define your widget type and implement the
  // `iced_native::Widget` trait with the `iced_wgpu::Renderer`.
  //
  // Of course, you can choose to make the implementation renderer-agnostic,
  // if you wish to, by creating your own `Renderer` trait, which could be
  // implemented by `iced_wgpu` and other renderers.
  use iced::advanced::layout::{self, Layout};
  use iced::advanced::renderer;
  use iced::advanced::widget::{self, Widget};
  use iced::mouse;
  use iced::{Color, Element, Length, Rectangle, Size};

  pub enum BoxColor {
    Grey,
    Yellow,
    Green
  }

  pub struct LetterBox {
      color: BoxColor,
      letter: Option<char>
  }

  impl LetterBox {
      pub fn new() -> Self {
          Self {
            color: BoxColor::Grey,
            letter: None
          }
      }

      pub fn set_color(self: &mut Self, color: BoxColor) {
        self.color = color;
      }
  }

  pub fn letter_box() -> LetterBox {
    LetterBox::new()
  }

  impl<Message, Renderer> Widget<Message, Renderer> for LetterBox
  where
      Renderer: renderer::Renderer,
  {
      fn width(&self) -> Length {
          Length::Shrink
      }

      fn height(&self) -> Length {
          Length::Shrink
      }

      fn layout(
          &self,
          _renderer: &Renderer,
          _limits: &layout::Limits,
      ) -> layout::Node {
          layout::Node::new(Size::new(25.0, 25.0))
      }

      fn draw(
          &self,
          _state: &widget::Tree,
          renderer: &mut Renderer,
          _theme: &Renderer::Theme,
          _style: &renderer::Style,
          layout: Layout<'_>,
          _cursor: mouse::Cursor,
          _viewport: &Rectangle,
      ) {
          let render_color = match self.color {
            BoxColor::Grey => Color::from_rgb8(0x3a, 0x3a, 0x3c), // #3a3a3c
            BoxColor::Yellow => Color::from_rgb8(0xb5, 0x9f, 0x3b), // #b59f3b
            BoxColor::Green => Color::from_rgb8(0x53, 0x8d, 0x4e), // #538d4e
          };

          renderer.fill_quad(
              renderer::Quad {
                  bounds: layout.bounds(),
                  border_radius: 5.0.into(),
                  border_width: 0.0,
                  border_color: Color::TRANSPARENT,
              },
              render_color,
          );
      }
  }

  impl<'a, Message, Renderer> From<LetterBox> for Element<'a, Message, Renderer>
  where
      Renderer: renderer::Renderer,
  {
      fn from(letter_box: LetterBox) -> Self {
          Self::new(letter_box)
      }
  }
}