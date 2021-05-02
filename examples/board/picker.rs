use ggez::graphics::{
    BlendMode, Color, DrawMode, DrawParam, Drawable, FillOptions, Image, Mesh, Rect,
};
use ggez::mint::Point2;
use ggez::{Context, GameResult};

use crate::piece::*;

const BACKGROUND: Color = Color::new(0.8, 0.8, 0.8, 1.0);

pub struct PickerItem {
    item: Box<dyn Piece>,
    name: Name,
    pcolor: PColor,
    rect: Rect,
}

impl PickerItem {
    fn new(ctx: &mut Context, name: Name, pcolor: PColor) -> Self {
        Self {
            item: new_piece(ctx, name, pcolor),
            rect: Rect::default(),
            name,
            pcolor,
        }
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }
}

impl Drawable for PickerItem {
    fn draw(&self, ctx: &mut Context, _param: DrawParam) -> GameResult<()> {
        let img_w = self.item.image().width() as f32;
        let img_h = self.item.image().width() as f32;
        let scale_w = self.rect.w / img_w;
        let scale_h = self.rect.h / img_h;

        self.item.image().draw(
            ctx,
            DrawParam::new()
                .dest(self.rect.point())
                .scale([scale_w, scale_h]),
        )
    }

    fn blend_mode(&self) -> Option<BlendMode> {
        None
    }

    fn dimensions(&self, _ctx: &mut Context) -> Option<Rect> {
        Some(self.rect)
    }

    fn set_blend_mode(&mut self, _mode: Option<BlendMode>) {}
}

pub struct Picker {
    rect: Rect,
    items: Vec<PickerItem>,
}

impl Picker {
    pub fn new(ctx: &mut Context, rect: Rect) -> Self {
        let mut items = Vec::new();
        items.push(PickerItem::new(ctx, Name::PAWN, PColor::BLACK));
        items.push(PickerItem::new(ctx, Name::PAWN, PColor::WHITE));
        items.push(PickerItem::new(ctx, Name::ROOK, PColor::BLACK));
        items.push(PickerItem::new(ctx, Name::ROOK, PColor::WHITE));
        items.push(PickerItem::new(ctx, Name::BISHOP, PColor::BLACK));
        items.push(PickerItem::new(ctx, Name::BISHOP, PColor::WHITE));
        items.push(PickerItem::new(ctx, Name::KNIGHT, PColor::BLACK));
        items.push(PickerItem::new(ctx, Name::KNIGHT, PColor::WHITE));
        items.push(PickerItem::new(ctx, Name::QUEEN, PColor::BLACK));
        items.push(PickerItem::new(ctx, Name::QUEEN, PColor::WHITE));
        items.push(PickerItem::new(ctx, Name::KING, PColor::BLACK));
        items.push(PickerItem::new(ctx, Name::KING, PColor::WHITE));

        let mut picker = Self { items, rect };
        picker.update_items_rects();
        picker
    }

    fn update_items_rects(&mut self) {
        let Rect { x, y, w, .. } = self.rect;
        let mut wdif = -0.;
        for (i, item) in self.items.iter_mut().enumerate() {
            let padding = 10.;
            let size = w / 2. - (padding * 3.);
            let m = (i % 2) as f32;
            let ix = x + padding + (m * (size + padding * 3.));
            let iy = y + padding + (wdif * (y + size + padding));
            wdif += m;

            let rect = Rect::new(ix, iy, size, size);
            item.set_rect(rect);
        }
    }

    pub fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
        self.update_items_rects();
    }

    pub fn contains_point(&self, point: Point2<f32>) -> bool {
        self.rect.contains(point)
    }

    pub fn on_dragable(&self, point: Point2<f32>) -> bool {
        self.items
            .iter()
            .filter(|i| i.rect.contains(point))
            .next()
            .is_some()
    }

    pub fn get_item_at(&self, at: Point2<f32>) -> (Image, Name, PColor) {
        let item = self
            .items
            .iter()
            .filter(|i| i.rect.contains(at))
            .next()
            .unwrap();
        (item.item.image().clone(), item.name, item.pcolor)
    }
}

impl Drawable for Picker {
    fn draw(&self, ctx: &mut Context, _param: DrawParam) -> GameResult<()> {
        let mrect = Mesh::new_rectangle(
            ctx,
            DrawMode::Fill(FillOptions::default()),
            self.rect,
            BACKGROUND,
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

    fn dimensions(&self, _ctx: &mut Context) -> Option<Rect> {
        Some(self.rect)
    }

    fn set_blend_mode(&mut self, _mode: Option<BlendMode>) {}
}
