use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bookmark {
    name: String,
    path: PathBuf,
}

impl Bookmark {
    pub fn new(
        name: impl Into<String>,
        path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

#[derive(Debug, Default)]
pub struct BookmarkManager {
    bookmarks: HashMap<String, Bookmark>,
}

impl BookmarkManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(
        &mut self,
        name: impl Into<String>,
        path: impl Into<PathBuf>,
    ) -> Option<Bookmark> {
        let bookmark = Bookmark::new(name, path);
        self.bookmarks
            .insert(bookmark.name.clone(), bookmark)
    }

    pub fn remove(&mut self, name: &str) -> Option<Bookmark> {
        self.bookmarks.remove(name)
    }

    pub fn get(&self, name: &str) -> Option<&Bookmark> {
        self.bookmarks.get(name)
    }

    pub fn contains(&self, name: &str) -> bool {
        self.bookmarks.contains_key(name)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Bookmark> {
        self.bookmarks.values()
    }

    pub fn len(&self) -> usize {
        self.bookmarks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bookmarks.is_empty()
    }

    pub fn clear(&mut self) {
        self.bookmarks.clear();
    }

    pub fn find_by_path(
        &self,
        path: &Path,
    ) -> Option<&Bookmark> {
        self.bookmarks
            .values()
            .find(|bookmark| bookmark.path() == path)
    }
}
