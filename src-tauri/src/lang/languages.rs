use super::Language;

use regex::Regex;

macro_rules! l {
    ($ext:literal, $name:literal, $id:literal) => {
        (Regex::new($ext).unwrap(), Language($name, $id))
    };
}

// The array is sorted by the extension name
lazy_static! {
    pub static ref LANGUAGES: [(Regex, Language<'static>); 60] = [
        l!(r"bat", "Batch", "batch"),
        l!(r"c", "C", "c"),
        l!(r"cc", "C++", "cpp"),
        l!(r"cl", "Common Lisp", "common-lisp"),
        l!(r"clj", "Clojure", "clojure"),
        l!(r"comp", "GLSL", "glsl"),
        l!(r"cpp", "C++", "cpp"),
        l!(r"cs", "C#", "csharp"),
        l!(r"css", "CSS", "css"),
        l!(r"cxx", "C++", "cpp"),
        l!(r"dart", "Dart", "dart"),
        l!(r"frag", "GLSL", "glsl"),
        l!(r"geom", "GLSL", "glsl"),
        l!(r"glsl", "GLSL", "glsl"),
        l!(r"go", "Go", "go"),
        l!(r"h", "C", "c"),
        l!(r"haml", "Haml", "haml"),
        l!(r"handlebars", "Handlebars", "handlebars"),
        l!(r"hbs", "Handlebars", "handlebars"),
        l!(r"hlsl", "HLSL", "HLSL"),
        l!(r"hpp", "C++", "cpp"),
        l!(r"hh", "C++", "cpp"),
        l!(r"html", "HTML", "html"),
        l!(r"htm", "HTML", "html"),
        l!(r"hxx", "C++", "cpp"),
        l!(r"ini", "INI", "ini"),
        l!(r"java", "Java", "java"),
        l!(r"jinja", "Jinja", "jinja"),
        l!(r"jinja2", "Jinja", "jinja"),
        l!(r"js", "JavaScript", "javascript"),
        l!(r"json", "JSON", "json"),
        l!(r"jsonc", "JSON with Comments", "jsonc"),
        l!(r"kt", "Kotlin", "kotlin"),
        l!(r"less", "Less", "less"),
        l!(r"lua", "Lua", "lua"),
        l!(r"md", "Markdown", "markdown"),
        l!(r"pl", "Perl", "perl"),
        l!(r"py", "Python", "python"),
        l!(r"pyc", "Python", "python"),
        l!(r"pyo", "Python", "python"),
        l!(r"rb", "Ruby", "ruby"),
        l!(r"rkt", "Racket", "racket"),
        l!(r"rs", "Rust", "rust"),
        l!(r"sass", "SASS", "sass"),
        l!(r"sc", "Scala", "scala"),
        l!(r"scala", "Scala", "scala"),
        l!(r"scss", "SCSS", "scss"),
        l!(r"sh", "Shell", "shell"),
        l!(r"sql", "SQL", "sql"),
        l!(r"swift", "Swift", "swift"),
        l!(r"tesc", "GLSL", "glsl"),
        l!(r"tese", "GLSL", "glsl"),
        l!(r"tex", "TeX", "tex"),
        l!(r"toml", "TOML", "toml"),
        l!(r"ts", "TypeScript", "typescript"),
        l!(r"vert", "GLSL", "glsl"),
        l!(r"xhtml", "XHTML", "xhtml"),
        l!(r"xml", "XML", "xml"),
        l!(r"yaml", "YAML", "yaml"),
        l!(r"yml", "YAML", "yaml"),
    ];
}
