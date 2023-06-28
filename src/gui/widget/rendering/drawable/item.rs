use rust_graphics::{draw_command::DrawCommand, rect::Rect, vec::Vec2};

use crate::{
    gui::{
        app::app::FontManager,
        widget::{
            rendering::{
                align::Align2D,
                anchor::{Anchor, Anchor2D},
            },
            size_policy::SizePolicy2D,
            state::observable::Observer,
            theme::theme::Theme,
        },
    },
    prelude::{AlignH, AlignV, SizePolicy},
};

use super::Drawable;

pub struct DrawItem {
    width: Observer<SizePolicy>,
    height: Observer<SizePolicy>,
    offset_x: Observer<SizePolicy>,
    offset_y: Observer<SizePolicy>,
    anchor_h: Observer<Anchor>,
    anchor_v: Observer<Anchor>,
    alignment_h: Observer<AlignH>,
    alignment_v: Observer<AlignV>,
    drawable: Box<dyn Drawable>,
}

impl DrawItem {
    pub fn new<T>(drawable: T) -> Self
    where
        T: Drawable + 'static,
    {
        Self {
            width: Observer::value(SizePolicy::default()),
            height: Observer::value(SizePolicy::default()),
            offset_x: Observer::value(SizePolicy::zero()),
            offset_y: Observer::value(SizePolicy::zero()),
            anchor_h: Observer::value(Anchor::default()),
            anchor_v: Observer::value(Anchor::default()),
            alignment_h: Observer::value(AlignH::default()),
            alignment_v: Observer::value(AlignV::default()),
            drawable: Box::new(drawable),
        }
    }

    pub fn set_size(
        &mut self,
        size: (
            impl Into<Observer<SizePolicy>>,
            impl Into<Observer<SizePolicy>>,
        ),
    ) -> &mut Self {
        self.width = size.0.into();
        self.height = size.1.into();
        self
    }

    pub fn set_width(&mut self, width: impl Into<Observer<SizePolicy>>) -> &mut Self {
        self.width = width.into();
        self
    }

    pub fn set_height(&mut self, height: impl Into<Observer<SizePolicy>>) -> &mut Self {
        self.height = height.into();
        self
    }

    pub fn set_offset(
        &mut self,
        offset: (
            impl Into<Observer<SizePolicy>>,
            impl Into<Observer<SizePolicy>>,
        ),
    ) -> &mut Self {
        self.offset_x = offset.0.into();
        self.offset_y = offset.1.into();
        self
    }

    pub fn set_offset_x(&mut self, offset_x: impl Into<Observer<SizePolicy>>) -> &mut Self {
        self.offset_x = offset_x.into();
        self
    }

    pub fn set_offset_y(&mut self, offset_y: impl Into<Observer<SizePolicy>>) -> &mut Self {
        self.offset_y = offset_y.into();
        self
    }

    pub fn set_anchor(
        &mut self,
        anchor: (impl Into<Observer<Anchor>>, impl Into<Observer<Anchor>>),
    ) -> &mut Self {
        self.anchor_h = anchor.0.into();
        self.anchor_v = anchor.1.into();
        self
    }

    pub fn set_anchor_h(&mut self, anchor_h: impl Into<Observer<Anchor>>) -> &mut Self {
        self.anchor_h = anchor_h.into();
        self
    }

    pub fn set_anchor_v(&mut self, anchor_v: impl Into<Observer<Anchor>>) -> &mut Self {
        self.anchor_v = anchor_v.into();
        self
    }

    pub fn set_alignment(
        &mut self,
        alignment: (impl Into<Observer<AlignH>>, impl Into<Observer<AlignV>>),
    ) -> &mut Self {
        self.alignment_h = alignment.0.into();
        self.alignment_v = alignment.1.into();
        self
    }

    pub fn set_alignment_h(&mut self, alignment_h: impl Into<Observer<AlignH>>) -> &mut Self {
        self.alignment_h = alignment_h.into();
        self
    }

    pub fn set_alignment_v(&mut self, alignment_v: impl Into<Observer<AlignV>>) -> &mut Self {
        self.alignment_v = alignment_v.into();
        self
    }

    pub fn calculate_rect(&self, parent_rect: &Rect) -> Rect {
        let Some(width) = self.width.get() else {
            println!("Failed to observe width of draw base!");
            return Rect::default();
        };
        let Some(height) = self.height.get() else {
            println!("Failed to observe height of draw base!");
            return Rect::default();
        };
        let Some(offset_x) = self.offset_x.get() else {
            println!("Failed to observe offset X of draw base!");
            return Rect::default();
        };
        let Some(offset_y) = self.offset_y.get() else {
            println!("Failed to observe offset Y of draw base!");
            return Rect::default();
        };
        let Some(anchor_h) = self.anchor_h.get() else {
            println!("Failed to observe anchor H of draw base!");
            return Rect::default();
        };
        let Some(anchor_v) = self.anchor_v.get() else {
            println!("Failed to observe anchor V of draw base!");
            return Rect::default();
        };
        let Some(alignment_h) = self.alignment_h.get() else {
            println!("Failed to observe alignment H of draw base!");
            return Rect::default();
        };
        let Some(alignment_v) = self.alignment_v.get() else {
            println!("Failed to observe alignment V of draw base!");
            return Rect::default();
        };

        let size = SizePolicy2D::from((width, height));
        let offset = SizePolicy2D::from((offset_x, offset_y));
        let anchor = Anchor2D::from((anchor_h, anchor_v));
        let alignment = Align2D::from((alignment_h, alignment_v));

        let size = size.calculate_size(parent_rect.size());
        let position_offset = offset.calculate_size(parent_rect.size());
        let align_point = Vec2::from((
            match alignment.horizontal {
                AlignH::Left => parent_rect.left,
                AlignH::Center => parent_rect.center().x,
                AlignH::Right => parent_rect.right,
            },
            match alignment.vertical {
                AlignV::Top => parent_rect.top,
                AlignV::Center => parent_rect.center().y,
                AlignV::Bottom => parent_rect.bottom,
            },
        ));
        let anchor_offset = Vec2::from((
            match anchor.horizontal {
                Anchor::Leading => 0.0,
                Anchor::Center => size.x / 2.0,
                Anchor::Trailing => size.x,
            },
            match anchor.vertical {
                Anchor::Leading => 0.0,
                Anchor::Center => size.y / 2.0,
                Anchor::Trailing => size.y,
            },
        ));

        let pos = align_point - anchor_offset + position_offset;
        Rect::new_from_xy(pos.x, pos.y, size.x, size.y)
    }

    pub fn get_draw_command(
        &self,
        parent_rect: &Rect,
        font_manager: &FontManager,
        theme: &Theme,
    ) -> DrawCommand {
        let rect = self.calculate_rect(parent_rect);
        self.drawable.draw(rect, font_manager, theme)
    }
}
