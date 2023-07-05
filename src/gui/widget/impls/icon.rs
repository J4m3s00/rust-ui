use crate::{
    gui::{svg::svg::Svg, widget::rendering::drawable::draw_svg::DrawSvg},
    prelude::{BuildContext, BuildResult, ToInstance, Widget, WidgetInstance},
};

pub struct Icon {
    icon: Svg,
}

impl Icon {
    pub fn new(icon: IconType) -> WidgetInstance {
        let svg = icon.to_svg();
        println!("svg: {:?}", svg);

        Self { icon: svg }.instance()
    }
}

impl Widget for Icon {
    fn build(&mut self, _ctx: &mut BuildContext) -> BuildResult {
        let mut res = BuildResult::default();
        res.draw_svg(DrawSvg::new(self.icon.clone()));
        res
    }
}

pub enum IconType {
    Checkmark,
}

impl IconType {
    pub fn to_svg(&self) -> Svg {
        match self {
            IconType::Checkmark => Svg::load_from_str(include_str!("icons/test.svg"))
                .map_err(|e| {
                    println!("Could not load svg: {}", e);
                })
                .unwrap(),
        }
    }
}
