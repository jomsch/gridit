use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::graphics::{
    Image, size, BlendMode, Color, DrawMode, DrawParam, Drawable, FillOptions, StrokeOptions, Mesh, Rect,
};
use ggez::mint::Point2;

use gridit::{Grid, PositionsEnumerator, Position};

use crate::piece::*;

const BACKGROUND: Color = Color::new(0.8, 0.8, 0.8 ,1.0);

pub struct PickerItem {
    item: Box<dyn Piece>,
    rect: Rect,
}

impl PickerItem {
    fn new(item: Box<dyn Piece>) -> Self {
        Self {
            item,
            rect: Rect::default(),
        }
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }
}

impl Drawable for PickerItem {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult<()> {
        let img_w = self.item.image().width() as f32;
        let img_h = self.item.image().width() as f32;
        let scale_w = self.rect.w / img_w ;
        let scale_h = self.rect.h / img_h;

        self.item
            .image()
            .draw(ctx, 
                DrawParam::new()
                .dest(self.rect.point())
                .scale([scale_w, scale_h])
            )
    }

    fn blend_mode(&self) -> Option<BlendMode> {
        None 
    }

    fn dimensions(&self, ctx: &mut Context) -> Option<Rect> {
        Some(self.rect) 
    }

    fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
        
    }
}

pub struct Picker {
    rect: Rect,
    items: Vec<PickerItem>,
}

impl Picker {
    pub fn new(ctx: &mut Context, rect: Rect) -> Self {
        let mut items = Vec::new(); 
        items.push(PickerItem::new(new_piece(ctx, Name::PAWN, PColor::BLACK)));
        items.push(PickerItem::new(new_piece(ctx, Name::TEST, PColor::BLACK)));
        items.push(PickerItem::new(new_piece(ctx, Name::PAWN, PColor::BLACK)));
        items.push(PickerItem::new(new_piece(ctx, Name::TEST, PColor::BLACK)));
        items.push(PickerItem::new(new_piece(ctx, Name::PAWN, PColor::BLACK)));
        items.push(PickerItem::new(new_piece(ctx, Name::TEST, PColor::BLACK)));
        items.push(PickerItem::new(new_piece(ctx, Name::PAWN, PColor::BLACK)));
        items.push(PickerItem::new(new_piece(ctx, Name::TEST, PColor::BLACK)));
        items.push(PickerItem::new(new_piece(ctx, Name::PAWN, PColor::BLACK)));
        items.push(PickerItem::new(new_piece(ctx, Name::TEST, PColor::BLACK)));
        items.push(PickerItem::new(new_piece(ctx, Name::PAWN, PColor::BLACK)));
        items.push(PickerItem::new(new_piece(ctx, Name::TEST, PColor::BLACK)));

        let mut picker = Self { items, rect };
        picker.update_items_rects();
        picker
    }

    fn update_items_rects(&mut self) {
        let Rect { x, y, w, h } = self.rect;
        let mut wdif = -0.;
        for (i, item) in self.items.iter_mut().enumerate() {
            let padding = 10.;
            let size = w/2.-(padding*3.);
            let m = (i % 2) as f32;
            let ix = x + padding + (m*(size + padding));
            let iw = y + padding + (wdif * ( y + size + padding*2.)); 
            wdif += m;

            let rect = Rect::new(ix, iw, size, size);
            item.set_rect(rect);
        }
    } 

    pub fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
        self.update_items_rects();
    }

    pub fn rect(&self) -> &Rect {
        &self.rect
    }
}

impl Drawable for Picker {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult<()> {
        let mrect = Mesh::new_rectangle(
            ctx,
            DrawMode::Fill(FillOptions::default()),
            self.rect,
            BACKGROUND
        )?;

        mrect.draw(ctx, DrawParam::default())?;

        for item in self.items.iter() {
            item.draw(ctx, DrawParam::default())?;
        }

        Ok(())
    }

    fn blend_mode(&self) -> Option<BlendMode> {
        None 
    }

    fn dimensions(&self, ctx: &mut Context) -> Option<Rect> {
        Some(self.rect) 
    }

    fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
        
    }
}
