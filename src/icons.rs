use crate::args::{parse_args, Args};

use std::fs::FileType;
use std::path::Path;

use std::collections::HashMap;

// icon for entry
pub struct Icon {
    // icon as string
    icon: String,
}

impl Icon {
    // create icon from filetype
    pub fn new(filetype: FileType, path: &Path) -> Self {
        let Args { icons, .. } = parse_args();

        if !icons {
            // return empty icon
            return Self { icon: String::new() };
        }

        let icon = String::from(
            if filetype.is_dir() {
                // folder icon for directories
                ""
            } else if let Some(extension) = path.extension() {
                let extension = extension.to_str().unwrap();

                // get file icon from extension icons
                if let Some(icon) = Self::extension_icons().get(extension) {
                    icon
                } else {
                    // default file icon
                    ""
                }
            } else {
                // default file icon
                ""
            }
        );

        Self { icon }
    }

    // display icon
    pub fn display(&self) -> String {
        if self.icon.is_empty() {
            String::new()
        } else {
            format!("{} ", self.icon)
        }
    }

    // extension icons for file entries
    fn extension_icons() -> HashMap<&'static str, &'static str> {
        let mut icons = HashMap::new();

        icons.insert("7z", "\u{f410}");             // icon: 
        icons.insert("ai", "\u{e7b4}");             // icon: 
        icons.insert("apk", "\u{e70e}");            // icon: 
        icons.insert("avi", "\u{f03d}");            // icon: 
        icons.insert("avro", "\u{e60b}");           // icon: 
        icons.insert("awk", "\u{e795}");            // icon: 
        icons.insert("bash", "\u{e795}");           // icon: 
        icons.insert("bash_history", "\u{e795}");   // icon: 
        icons.insert("bash_profile", "\u{e795}");   // icon: 
        icons.insert("bashrc", "\u{e795}");         // icon: 
        icons.insert("bat", "\u{f17a}");            // icon: 
        icons.insert("bmp", "\u{f1c5}");            // icon: 
        icons.insert("bz2", "\u{f410}");            // icon: 
        icons.insert("c", "\u{e61e}");              // icon: 
        icons.insert("c++", "\u{e61d}");            // icon: 
        icons.insert("cc", "\u{e61d}");             // icon: 
        icons.insert("cfg", "\u{e615}");            // icon: 
        icons.insert("clj", "\u{e768}");            // icon: 
        icons.insert("cljs", "\u{e76a}");           // icon: 
        icons.insert("cls", "\u{e600}");            // icon: 
        icons.insert("coffee", "\u{e61b}");         // icon: 
        icons.insert("conf", "\u{e615}");           // icon: 
        icons.insert("cp", "\u{e61d}");             // icon: 
        icons.insert("cpp", "\u{e61d}");            // icon: 
        icons.insert("cs", "\u{f81a}");             // icon: 
        icons.insert("cshtml", "\u{f1fa}");         // icon: 
        icons.insert("csproj", "\u{f81a}");         // icon: 
        icons.insert("csx", "\u{f81a}");            // icon: 
        icons.insert("csh", "\u{e795}");            // icon: 
        icons.insert("css", "\u{e749}");            // icon: 
        icons.insert("csv", "\u{f1c3}");            // icon: 
        icons.insert("cxx", "\u{e61d}");            // icon: 
        icons.insert("d", "\u{e7af}");              // icon: 
        icons.insert("dart", "\u{e798}");           // icon: 
        icons.insert("db", "\u{f1c0}");             // icon: 
        icons.insert("diff", "\u{f440}");           // icon: 
        icons.insert("doc", "\u{f1c2}");            // icon: 
        icons.insert("docx", "\u{f1c2}");           // icon: 
        icons.insert("ds_store", "\u{f179}");       // icon: 
        icons.insert("dump", "\u{f1c0}");           // icon: 
        icons.insert("ebook", "\u{e28b}");          // icon: 
        icons.insert("editorconfig", "\u{e615}");   // icon: 
        icons.insert("ejs", "\u{e618}");            // icon: 
        icons.insert("elm", "\u{e62c}");            // icon: 
        icons.insert("env", "\u{f462}");            // icon: 
        icons.insert("eot", "\u{f031}");            // icon: 
        icons.insert("epub", "\u{e28a}");           // icon: 
        icons.insert("erb", "\u{e73b}");            // icon: 
        icons.insert("erl", "\u{e7b1}");            // icon: 
        icons.insert("exe", "\u{f17a}");            // icon: 
        icons.insert("ex", "\u{e62d}");             // icon: 
        icons.insert("exs", "\u{e62d}");            // icon: 
        icons.insert("fish", "\u{e795}");           // icon: 
        icons.insert("flac", "\u{f001}");           // icon: 
        icons.insert("flv", "\u{f03d}");            // icon: 
        icons.insert("font", "\u{f031}");           // icon: 
        icons.insert("fpl", "\u{f910}");            // icon: 蘿
        icons.insert("fs", "\u{e7a7}");             // icon: 
        icons.insert("fsx", "\u{e7a7}");            // icon: 
        icons.insert("fsi", "\u{e7a7}");            // icon: 
        icons.insert("gdoc", "\u{f1c2}");           // icon: 
        icons.insert("gemfile", "\u{e791}");        // icon: 
        icons.insert("gemspec", "\u{e791}");        // icon: 
        icons.insert("gform", "\u{f298}");          // icon: 
        icons.insert("gif", "\u{f1c5}");            // icon: 
        icons.insert("git", "\u{e702}");            // icon: 
        icons.insert("go", "\u{e627}");             // icon: 
        icons.insert("gradle", "\u{e70e}");         // icon: 
        icons.insert("gsheet", "\u{f1c3}");         // icon: 
        icons.insert("gslides", "\u{f1c4}");        // icon: 
        icons.insert("guardfile", "\u{e791}");      // icon: 
        icons.insert("gz", "\u{f410}");             // icon: 
        icons.insert("h", "\u{f0fd}");              // icon: 
        icons.insert("hbs", "\u{e60f}");            // icon: 
        icons.insert("hpp", "\u{f0fd}");            // icon: 
        icons.insert("hs", "\u{e777}");             // icon: 
        icons.insert("htm", "\u{f13b}");            // icon: 
        icons.insert("html", "\u{f13b}");           // icon: 
        icons.insert("hxx", "\u{f0fd}");            // icon: 
        icons.insert("ico", "\u{f1c5}");            // icon: 
        icons.insert("image", "\u{f1c5}");          // icon: 
        icons.insert("iml", "\u{e7b5}");            // icon: 
        icons.insert("ini", "\u{e615}");            // icon: 
        icons.insert("ipynb", "\u{e606}");          // icon: 
        icons.insert("jar", "\u{e256}");            // icon: 
        icons.insert("java", "\u{e256}");           // icon: 
        icons.insert("jpeg", "\u{f1c5}");           // icon: 
        icons.insert("jpg", "\u{f1c5}");            // icon: 
        icons.insert("js", "\u{e74e}");             // icon: 
        icons.insert("json", "\u{e60b}");           // icon: 
        icons.insert("jsx", "\u{e7ba}");            // icon: 
        icons.insert("jl", "\u{e624}");             // icon: 
        icons.insert("ksh", "\u{e795}");            // icon: 
        icons.insert("less", "\u{e614}");           // icon: 
        icons.insert("lhs", "\u{e777}");            // icon: 
        icons.insert("license", "\u{e609}");        // icon: 
        icons.insert("localized", "\u{f179}");      // icon: 
        icons.insert("lock", "\u{f023}");           // icon: 
        icons.insert("log", "\u{f868}");            // icon: 
        icons.insert("lua", "\u{e620}");            // icon: 
        icons.insert("lz", "\u{f410}");             // icon: 
        icons.insert("m3u", "\u{f910}");            // icon: 蘿
        icons.insert("m3u8", "\u{f910}");           // icon: 蘿
        icons.insert("m4a", "\u{f001}");            // icon: 
        icons.insert("magnet", "\u{f076}");         // icon: 
        icons.insert("markdown", "\u{e609}");       // icon: 
        icons.insert("md", "\u{e609}");             // icon: 
        icons.insert("mjs", "\u{e74e}");            // icon: 
        icons.insert("mkd", "\u{e609}");            // icon: 
        icons.insert("mkv", "\u{f03d}");            // icon: 
        icons.insert("mobi", "\u{e28b}");           // icon: 
        icons.insert("mov", "\u{f03d}");            // icon: 
        icons.insert("mp3", "\u{f001}");            // icon: 
        icons.insert("mp4", "\u{f03d}");            // icon: 
        icons.insert("mustache", "\u{e60f}");       // icon: 
        icons.insert("nix", "\u{f313}");            // icon: 
        icons.insert("npmignore", "\u{e71e}");      // icon: 
        icons.insert("opus", "\u{f001}");           // icon: 
        icons.insert("ogg", "\u{f001}");            // icon: 
        icons.insert("ogv", "\u{f03d}");            // icon: 
        icons.insert("otf", "\u{f031}");            // icon: 
        icons.insert("pdf", "\u{f1c1}");            // icon: 
        icons.insert("php", "\u{e73d}");            // icon: 
        icons.insert("pl", "\u{e769}");             // icon: 
        icons.insert("pls", "\u{f910}");            // icon: 蘿
        icons.insert("pm", "\u{e769}");             // icon: 
        icons.insert("png", "\u{f1c5}");            // icon: 
        icons.insert("ppt", "\u{f1c4}");            // icon: 
        icons.insert("pptx", "\u{f1c4}");           // icon: 
        icons.insert("procfile", "\u{e791}");       // icon: 
        icons.insert("properties", "\u{e60b}");     // icon: 
        icons.insert("ps1", "\u{e795}");            // icon: 
        icons.insert("psd", "\u{e7b8}");            // icon: 
        icons.insert("pxm", "\u{f1c5}");            // icon: 
        icons.insert("py", "\u{e606}");             // icon: 
        icons.insert("pyc", "\u{e606}");            // icon: 
        icons.insert("r", "\u{f25d}");              // icon: 
        icons.insert("rakefile", "\u{e791}");       // icon: 
        icons.insert("rar", "\u{f410}");            // icon: 
        icons.insert("razor", "\u{f1fa}");          // icon: 
        icons.insert("rb", "\u{e791}");             // icon: 
        icons.insert("rdata", "\u{f25d}");          // icon: 
        icons.insert("rdb", "\u{e76d}");            // icon: 
        icons.insert("rdoc", "\u{e609}");           // icon: 
        icons.insert("rds", "\u{f25d}");            // icon: 
        icons.insert("readme", "\u{e609}");         // icon: 
        icons.insert("rlib", "\u{e7a8}");           // icon: 
        icons.insert("rmd", "\u{e609}");            // icon: 
        icons.insert("rs", "\u{e7a8}");             // icon: 
        icons.insert("rspec", "\u{e791}");          // icon: 
        icons.insert("rspec_parallel", "\u{e791}"); // icon: 
        icons.insert("rspec_status", "\u{e791}");   // icon: 
        icons.insert("rss", "\u{f09e}");            // icon: 
        icons.insert("ru", "\u{e791}");             // icon: 
        icons.insert("rubydoc", "\u{e73b}");        // icon: 
        icons.insert("sass", "\u{e603}");           // icon: 
        icons.insert("scala", "\u{e737}");          // icon: 
        icons.insert("scpt", "\u{f302}");           // icon: 
        icons.insert("scss", "\u{e749}");           // icon: 
        icons.insert("sh", "\u{e795}");             // icon: 
        icons.insert("shell", "\u{e795}");          // icon: 
        icons.insert("slim", "\u{e73b}");           // icon: 
        icons.insert("sln", "\u{e70c}");            // icon: 
        icons.insert("sql", "\u{f1c0}");            // icon: 
        icons.insert("sqlite3", "\u{e7c4}");        // icon: 
        icons.insert("styl", "\u{e600}");           // icon: 
        icons.insert("stylus", "\u{e600}");         // icon: 
        icons.insert("svg", "\u{f1c5}");            // icon: 
        icons.insert("swift", "\u{e755}");          // icon: 
        icons.insert("t", "\u{e769}");              // icon: 
        icons.insert("tar", "\u{f410}");            // icon: 
        icons.insert("tex", "\u{e600}");            // icon: 
        icons.insert("tiff", "\u{f1c5}");           // icon: 
        icons.insert("toml", "\u{f669}");           // icon: 
        icons.insert("ts", "\u{e628}");             // icon: 
        icons.insert("tsx", "\u{e7ba}");            // icon: 
        icons.insert("ttc", "\u{f031}");            // icon: 
        icons.insert("ttf", "\u{f031}");            // icon: 
        icons.insert("twig", "\u{e61c}");           // icon: 
        icons.insert("txt", "\u{f15c}");            // icon: 
        icons.insert("video", "\u{f03d}");          // icon: 
        icons.insert("vim", "\u{e62b}");            // icon: 
        icons.insert("vue", "\u{fd42}");            // icon: ﵂
        icons.insert("wav", "\u{f001}");            // icon: 
        icons.insert("webm", "\u{f03d}");           // icon: 
        icons.insert("webp", "\u{f1c5}");           // icon: 
        icons.insert("windows", "\u{f17a}");        // icon: 
        icons.insert("wma", "\u{f001}");            // icon: 
        icons.insert("wmv", "\u{f03d}");            // icon: 
        icons.insert("wpl", "\u{f910}");            // icon: 蘿
        icons.insert("woff", "\u{f031}");           // icon: 
        icons.insert("woff2", "\u{f031}");          // icon: 
        icons.insert("xls", "\u{f1c3}");            // icon: 
        icons.insert("xlsx", "\u{f1c3}");           // icon: 
        icons.insert("xml", "\u{e619}");            // icon: 
        icons.insert("xul", "\u{e619}");            // icon: 
        icons.insert("xz", "\u{f410}");             // icon: 
        icons.insert("yaml", "\u{e60b}");           // icon: 
        icons.insert("yml", "\u{e60b}");            // icon: 
        icons.insert("zip", "\u{f410}");            // icon: 
        icons.insert("zsh", "\u{e795}");            // icon: 
        icons.insert("zsh-theme", "\u{e795}");      // icon: 
        icons.insert("zshrc", "\u{e795}");          // icon: 

        icons
    }
}
