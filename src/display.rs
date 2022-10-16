use crate::args::{parse_args, Args};
use crate::entry::Entry;

use walkdir::{WalkDir, DirEntry};
use atty::Stream;
use terminal_size::{terminal_size, Width};
use term_grid::{Grid, GridOptions, Direction, Filling, Cell};

use std::cmp::{max, Ordering};

// sort by directory contents count
fn more_contents_last(a: &'_ DirEntry, b: &'_ DirEntry) -> Ordering {
    let Args { depth, .. } = parse_args();

    // contents count for entry a
    let a_count = WalkDir::new(a.path())
        .max_depth(depth.saturating_sub(a.depth()))
        .into_iter()
        .count();

    // contents count for entry b
    let b_count = WalkDir::new(b.path())
        .max_depth(depth.saturating_sub(b.depth()))
        .into_iter()
        .count();

    a_count.cmp(&b_count)
}

// sort by showing directories first
fn dirs_first(a: &'_ DirEntry, b: &'_ DirEntry) -> Ordering {
    (b.file_type().is_dir() as u8)
        .cmp(&(a.file_type().is_dir() as u8))
}

// display files
pub fn display_files() {
    let args = parse_args();

    // walkdir from args and convert to vec
    let entries: Vec<DirEntry> = WalkDir::new(&args.path)
        .min_depth(if args.tree { 0 } else { 1 })
        .max_depth(args.depth)
        .sort_by(more_contents_last)
        .sort_by(dirs_first)
        .into_iter()
        .map(|i| i.unwrap())
        .collect();

    if !atty::is(Stream::Stdout) {
        // print plain entries
        for entry in entries.iter() {
            println!("{}", entry.file_name().to_str().unwrap());
        }
    } else if args.tree {
        // tree mode
        for (i, entry) in entries.iter().enumerate() {
            let entry = Entry::new(entry);
            // show tree entry
            entry.tree_entry(&entries, i);
        }
    } else {
        // grid for entries
        let mut grid = Grid::new(GridOptions {
            filling:   Filling::Spaces(3),
            direction: Direction::LeftToRight,
        });

        let mut longest = 0;

        for entry in entries.iter() {
            let entry = Entry::new(entry);

            // get entry width
            let entry_width = entry.raw_display().len() + 3;
            longest = max(entry_width, longest);

            let mut cell = Cell::from(entry.display());
            cell.width = entry_width;

            // add entry to grid
            grid.add(cell);
        }

        // make sure longest is not zero
        longest = max(longest, 1);

        // display grid
        let (Width(width), _) = terminal_size().unwrap();
        println!("{}", grid.fit_into_columns(width as usize / longest));
    }
}
