use raylib::prelude::*;

use crate::rendering::Drawable;

#[derive(Default)]
pub struct Button<'a> {
    rect: Rectangle,
    text_position: Vector2,
    color: Color,
    color_hover: Color,
    bg_color: Color,
    bg_color_hover: Color,
    font_size: f32,
    font: Option<&'a Font>,
    pub text: String,
}

impl<'a> Button<'a> {
    pub fn new(
        rect: Rectangle,
        text: impl ToString,
        color: Color,
        color_hover: Color,
        background_color: Color,
        background_color_hover: Color,
        font: &'a Font,
        font_size: f32,
    ) -> Self {
        let text = text.to_string();
        let measurements = font.measure_text(&text, font_size, 0.0);

        let text_position = Vector2::new(
            rect.x + rect.width / 2.0 - measurements.x / 2.0,
            rect.y + rect.height / 2.0 - measurements.y / 2.0,
        );

        Self {
            color,
            color_hover,
            bg_color: background_color,
            bg_color_hover: background_color_hover,
            font_size,
            text,
            rect,
            font: Some(font),
            text_position,
        }
    }

    pub fn is_hovered(&self, d: &RaylibDrawHandle) -> bool {
        self.rect
            .check_collision_point_rec(Vector2::new(d.get_mouse_x() as f32, d.get_mouse_y() as f32))
    }
}

impl Drawable for Button<'_> {
    /// * `_position`: This parameter is ignored since a button's position is static
    fn draw(
        &self,
        _position: (i32, i32),
        d: &mut RaylibDrawHandle,
        _textures: &std::collections::HashMap<String, Texture2D>,
    ) {
        let hovered = self.is_hovered(d);
        let (text_color, bg_color) = match hovered {
            false => (self.color, self.bg_color),
            true => (self.color_hover, self.bg_color_hover),
        };

        d.draw_rectangle_rec(self.rect, bg_color);
        d.draw_text_ex(
            self.font.unwrap(),
            &self.text,
            self.text_position,
            self.font_size,
            0.0,
            text_color,
        );
    }
}
