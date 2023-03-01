pub mod quickfix {
    use crate::{DocumentId, Editor};

    #[derive(Default)]
    pub struct Entry {
        file: String,
        doc_id: Option<DocumentId>,
    }

    #[derive(Default)]
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

        // TODO: How?
        pub fn populate_from_picker(&mut self) {
            self.reset()
        }

        fn reset(&mut self) {
            self.current_index = None;
            self.entries = Vec::new();
        }
    }
}
