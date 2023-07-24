#[derive(Debug, Clone, Copy, Default)]
pub enum PageRoute {
    #[default]
    Home,
    BlankPage,
    EmptyPage,
}

impl PageRoute {
    pub fn path(&self) -> &'static str {
        match self {
            Self::Home => "/",
            Self::BlankPage => "/page/blank",
            Self::EmptyPage => "/page/empty",
        }
    }
}
