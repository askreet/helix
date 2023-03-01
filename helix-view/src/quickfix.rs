// TODO:
// implement next command
// implement prev command
// implement viewer for quickfixes
// implement command runner for quickfix population
// implement picker-to-quicklist
pub mod quickfix {
    use std::path::PathBuf;

    use crate::Editor;

    #[derive(Default, Debug)]
    pub struct Location {
        path: PathBuf,
        line: usize,
        // col: Option<usize>,
    }

    #[derive(Default, Debug)]
    pub struct Entry {
        // TODO: I would love to use ui::FileLocation here, but that abstraction lives
        //       in helix-term. Should it?
        location: Location,
        text: String,
    }

    #[derive(Default, Debug)]
    pub struct List {
        current_index: Option<usize>,
        entries: Vec<Entry>,
    }

    impl List {
        pub fn new() -> Self {
            Self::default()
        }

        /// Advance the quickfix list one entry, wrapping around if at the end
        /// of the list already.
        pub fn next(&mut self, editor: &mut Editor) {}

        /// Rewind the quickfix list one entry, wrapping around to the end if at
        /// the beginning already.
        pub fn prev(&mut self, editor: &mut Editor) {}

        fn reset(&mut self) {
            self.current_index = None;
            self.entries = Vec::new();
        }
    }
}
