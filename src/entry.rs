use crate::colors::Color;
use crate::filename::Filename;
use crate::icons::Icon;

use walkdir::DirEntry;

// entry for displaying
pub struct Entry {
    // filename
    filename: Filename,

    // filetype icon
    icon: Icon,

    // file color
    color: Color,
}

impl Entry {
    // create new entry
    pub fn new(entry: &DirEntry) -> Self {
        let path = entry.path();

        let filename = Filename::new(entry);
        let icon  = Icon::new(entry.file_type(), path);
        let color = Color::new(path);

        Self {
            filename,
            icon,
            color,
        }
    }

    // display entry
    pub fn display(&self) -> String {
        self.color.paint(format!(
            "{}{}",
            self.icon.display(),
            self.filename.display(),
        ))
    }

    // display entry without color
    pub fn raw_display(&self) -> String {
        format!(
            "{}{}",
            self.icon.display(),
            self.filename.display(),
        )
    }

    // file entries for tree
    pub fn tree_entry(&self, entries: &[DirEntry], idx: usize) {
        let entry = entries.get(idx).unwrap();
        let depth = entry.depth();

        // bars to display
        let mut bars = String::new();

        for d in 1..depth {
            let mut bar = "";

            for e in idx..entries.len() {
                let ent = entries.get(e).unwrap();

                // don't show bar if entry is lower depth
                if ent.depth() < d {
                    break;
                }

                // show bar if entry has same depth
                if ent.depth() == d {
                    bar = "│";
                    break;
                }
            }

            bars = format!("{}{}   ", bars, bar);
        }

        if idx != 0 {
            // head for entries
            let mut head = "";

            for e in idx + 1..entries.len() {
                let ent = entries.get(e).unwrap();

                // show ending if entry is lower depth
                if ent.depth() < depth {
                    head = "└──";
                    break;
                }

                // show continuing if entry has same depth
                if ent.depth() == depth {
                    head = "├──";
                    break;
                }
            }

            println!(
                "{}{} {}",
                bars,
                head,
                self.display(),
            );
        } else {
            // show no bars and head for root
            println!("{}", self.display());
        }
    }
}
