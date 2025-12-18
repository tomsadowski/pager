// pager/src/view

#[derive(Clone, Debug)]
pub enum View {
    Tab,
    History,
    Bookmarks,
    Quit,
}
#[derive(Clone, Debug)]
pub enum ViewMsg {
    None,
    Go(String),
    Switch(View),
}
