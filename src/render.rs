use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;

pub fn render(canvas: &mut WindowCanvas, color: Color) {
	canvas.set_draw_color(color);
	canvas.clear();
	canvas.present();
}