use crate::{
    gui::{svg::svg::Svg, widget::widget::WidgetMouseState},
    prelude::{BuildContext, BuildResult, State, ToInstance, Widget, WidgetInstance},
};

pub struct Icon {
    icon: Svg,
}

impl Icon {
    pub fn new(icon: IconType) -> WidgetInstance {
        let svg = icon.to_svg();

        Self { icon: svg }.instance()
    }
}

impl Widget for Icon {
    fn build(&mut self, _ctx: &mut BuildContext, _: &State<WidgetMouseState>) -> BuildResult {
        let mut res = BuildResult::default();
        res.draw_svg(self.icon.clone());
        res
    }
}

pub enum IconType {
    ArrowRight,
    ArrowLeft,
    Blocked,
    CheckCircle,
    Checkmark,

    Delete,
    Favorite,
    Star,

    FileCopy,
    FileDownload,
    FileOpen,

    Home,
    Menu,
    Redo,
    Undo,
    Refresh,
    Search,
    Sync,

    ZoomIn,
    ZoomOut,
}

impl IconType {
    pub fn to_svg(&self) -> Svg {
        match self {
            IconType::Checkmark => Svg::load_from_str(include_str!("icons/checkmark.svg")),
            IconType::Home => Svg::load_from_str(include_str!("icons/home.svg")),
            IconType::Menu => Svg::load_from_str(include_str!("icons/menu.svg")),
            IconType::Redo => Svg::load_from_str(include_str!("icons/redo.svg")),
            IconType::Undo => Svg::load_from_str(include_str!("icons/undo.svg")),
            IconType::Refresh => Svg::load_from_str(include_str!("icons/refresh.svg")),
            IconType::Search => Svg::load_from_str(include_str!("icons/search.svg")),
            IconType::Sync => Svg::load_from_str(include_str!("icons/sync.svg")),
            IconType::ZoomIn => Svg::load_from_str(include_str!("icons/zoom_in.svg")),
            IconType::ZoomOut => Svg::load_from_str(include_str!("icons/zoom_out.svg")),
            IconType::ArrowRight => Svg::load_from_str(include_str!("icons/arrow_right.svg")),
            IconType::ArrowLeft => Svg::load_from_str(include_str!("icons/arrow_left.svg")),
            IconType::Blocked => Svg::load_from_str(include_str!("icons/block.svg")),
            IconType::CheckCircle => Svg::load_from_str(include_str!("icons/check_circle.svg")),
            IconType::Delete => Svg::load_from_str(include_str!("icons/delete.svg")),
            IconType::Favorite => Svg::load_from_str(include_str!("icons/favorite.svg")),
            IconType::Star => Svg::load_from_str(include_str!("icons/star.svg")),
            IconType::FileCopy => Svg::load_from_str(include_str!("icons/file_copy.svg")),
            IconType::FileDownload => Svg::load_from_str(include_str!("icons/file_download.svg")),
            IconType::FileOpen => Svg::load_from_str(include_str!("icons/file_open.svg")),
        }
        .map_err(|e| {
            println!("Could not load svg: {e}");
        })
        .unwrap()
    }
}
