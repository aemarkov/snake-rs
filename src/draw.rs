use log;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};

use crate::coord::Coord;
use crate::field::{CellType, Field};


/// Drawing context to store temporal variables
/// Internal helper struct to reduce amount of arguments passed down throug call stack
struct DrawingCtx<'a> {
    field: &'a Field,
    cell_width: f64,
    cell_height: f64,
}

/// Drawer object
/// Draws game field with HTML canvas
pub struct Draw {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    width: f64,
    height: f64
}

impl Draw {
    /// Create new Draw with given HTML Canvas id
    pub fn new(canvas_id: &str) -> Draw {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("Window has no document");

        let canvas = document.get_element_by_id(canvas_id)
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .expect("Canvas element is not canvas");
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let draw = Draw {
            canvas, ctx,
            // FIXME: don't use hardcoded size
            width: 512.0, height: 512.0
        };

        // Initial drawing
        draw.canvas.set_width(draw.width as u32);
        draw.canvas.set_height(draw.height as u32);
        draw.ctx.set_fill_style(&"#fff".into());
        draw.ctx.fill_rect(0.0, 0.0, draw.width, draw.height);

        log::info!("Draw initialized");
        return draw;
    }

    /// Draw game field
    pub fn draw(&self, field: &Field) {
        let ctx = DrawingCtx {
            field,
            cell_width: self.width / field.width as f64,
            cell_height: self.height / field.height as f64
        };

        // clear
        self.ctx.set_fill_style(&"#fff".into());
        self.ctx.set_stroke_style(&"#ccc".into());
        self.ctx.fill_rect(0.0, 0.0, self.width, self.height);

        self.draw_field(&ctx);
        self.draw_grid(&ctx);
    }

    fn draw_field(&self, ctx: &DrawingCtx) {
        for row in 0..ctx.field.width {
            for col in 0..ctx.field.height {
                self.draw_cell(ctx, row as i32, col as i32)
            }
        }
    }

    fn draw_cell(&self, ctx: &DrawingCtx, row: i32, col: i32) {
        match Draw::get_cell_style(ctx.field[Coord::new(row, col)]) {
            None => return,
            Some(style) => {
                self.ctx.set_fill_style(&style.into());
                self.ctx.fill_rect(
                    (row as f64) * ctx.cell_width, (col as f64) * ctx.cell_height,
                    ctx.cell_width, ctx.cell_height);
            }
        };
    }

    fn get_cell_style(cell_type: CellType) -> Option<&'static str> {
        match cell_type {
            CellType::Empty => None,
            CellType::Food => Some("#f00"),
            CellType::Snake => Some("#000"),
            CellType::Head => Some("#555"),
        }
    }

    fn draw_grid(&self, ctx: &DrawingCtx) {
        for row in 0..ctx.field.width {
            self.ctx.move_to((row as f64) * ctx.cell_width, 0.0);
            self.ctx.line_to((row as f64) * ctx.cell_width, self.height);
        }
        for col in 0..ctx.field.height {
            self.ctx.move_to(0.0, (col as f64) * ctx.cell_height);
            self.ctx.line_to(self.width, (col as f64) * ctx.cell_height);
        }
        self.ctx.stroke();
    }
}
