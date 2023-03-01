// TODO:
// implement next command
// implement prev command
// implement viewer for quickfixes
// implement command runner for quickfix population
// implement picker-to-quicklist
use std::path::PathBuf;

// TODO: Do we want these structs to be public?

/// A structure suitable for formatting the new position in the list for the user.
pub struct NewPosition {
    /// The entry in the list we've switched to.
    pub entry: Entry,

    /// The human-readable (1-indexed) position in the list.
    pub number: usize,

    /// The total number of entries in the list.
    pub total: usize,
}

#[derive(Default, Debug, Clone)]
pub struct Location {
    pub path: PathBuf,
    pub line: usize,
    // col: Option<usize>,
}

#[derive(Default, Debug, Clone)]
pub struct Entry {
    // TODO: I would love to use ui::FileLocation here, but that abstraction lives
    //       in helix-term. Should it?
    pub location: Location,
    pub text: String,
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
    /// of the list already. If there are no entries, None is returned.
    pub fn next(&mut self) -> Option<NewPosition> {
        match self.current_index {
            Some(index) => {
                let new_index = if index == self.entries.len() - 1 {
                    0
                } else {
                    index + 1
                };

                self.current_index = Some(new_index);

                Some(NewPosition {
                    entry: self.entries[new_index].clone(),
                    number: new_index + 1,
                    total: self.entries.len(),
                })
            }
            None if self.entries.is_empty() => None,
            None => {
                self.current_index = Some(0);
                Some(NewPosition {
                    entry: self.entries[0].clone(),
                    number: 1,
                    total: self.entries.len(),
                })
            }
        }
    }

    /// Rewind the quickfix list one entry, wrapping around to the end if at
    /// the beginning already. If there are no entries, None is returned.
    pub fn prev(&mut self) -> Option<NewPosition> {
        match self.current_index {
            Some(index) => {
                let new_index = if index == 0 {
                    self.entries.len() - 1
                } else {
                    index - 1
                };

                self.current_index = Some(new_index);

                Some(NewPosition {
                    entry: self.entries[new_index].clone(),
                    number: new_index + 1,
                    total: self.entries.len(),
                })
            }
            None if self.entries.is_empty() => None,
            None => {
                self.current_index = Some(0);
                Some(NewPosition {
                    entry: self.entries[0].clone(),
                    number: 1,
                    total: self.entries.len(),
                })
            }
        }
    }

    pub fn reset(&mut self) {
        self.current_index = None;
        self.entries = Vec::new();
    }

    pub fn add(&mut self, entry: Entry) {
        self.entries.push(entry);
    }
}
