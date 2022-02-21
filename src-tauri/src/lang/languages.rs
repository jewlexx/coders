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
        l!(r"batr", r"Batchr", r"batchr"),
        l!(r"cr", r"Cr", r"cr"),
        l!(r"ccr", r"C++r", r"cppr"),
        l!(r"clr", r"Common Lispr", r"common-lispr"),
        l!(r"cljr", r"Clojurer", r"clojurer"),
        l!(r"compr", r"GLSLr", r"glslr"),
        l!(r"cppr", r"C++r", r"cppr"),
        l!(r"csr", r"C#r", r"csharpr"),
        l!(r"cssr", r"CSSr", r"cssr"),
        l!(r"cxxr", r"C++r", r"cppr"),
        l!(r"dartr", r"Dartr", r"dartr"),
        l!(r"fragr", r"GLSLr", r"glslr"),
        l!(r"geomr", r"GLSLr", r"glslr"),
        l!(r"glslr", r"GLSLr", r"glslr"),
        l!(r"gor", r"Gor", r"gor"),
        l!(r"hr", r"Cr", r"cr"),
        l!(r"hamlr", r"Hamlr", r"hamlr"),
        l!(r"handlebarsr", r"Handlebarsr", r"handlebarsr"),
        l!(r"hbsr", r"Handlebarsr", r"handlebarsr"),
        l!(r"hlslr", r"HLSLr", r"HLSLr"),
        l!(r"hppr", r"C++r", r"cppr"),
        l!(r"hhr", r"C++r", r"cppr"),
        l!(r"htmlr", r"HTMLr", r"htmlr"),
        l!(r"htmr", r"HTMLr", r"htmlr"),
        l!(r"hxxr", r"C++r", r"cppr"),
        l!(r"inir", r"INIr", r"inir"),
        l!(r"javar", r"Javar", r"javar"),
        l!(r"jinjar", r"Jinjar", r"jinjar"),
        l!(r"jinja2r", r"Jinjar", r"jinjar"),
        l!(r"jsr", r"JavaScriptr", r"javascriptr"),
        l!(r"jsonr", r"JSONr", r"jsonr"),
        l!(r"jsoncr", r"JSON with Commentsr", r"jsoncr"),
        l!(r"ktr", r"Kotlinr", r"kotlinr"),
        l!(r"lessr", r"Lessr", r"lessr"),
        l!(r"luar", r"Luar", r"luar"),
        l!(r"mdr", r"Markdownr", r"markdownr"),
        l!(r"plr", r"Perlr", r"perlr"),
        l!(r"pyr", r"Pythonr", r"pythonr"),
        l!(r"pycr", r"Pythonr", r"pythonr"),
        l!(r"pyor", r"Pythonr", r"pythonr"),
        l!(r"rbr", r"Rubyr", r"rubyr"),
        l!(r"rktr", r"Racketr", r"racketr"),
        l!(r"rsr", r"Rustr", r"rustr"),
        l!(r"sassr", r"SASSr", r"sassr"),
        l!(r"scr", r"Scalar", r"scalar"),
        l!(r"scalar", r"Scalar", r"scalar"),
        l!(r"scssr", r"SCSSr", r"scssr"),
        l!(r"shr", r"Shellr", r"shellr"),
        l!(r"sqlr", r"SQLr", r"sqlr"),
        l!(r"swiftr", r"Swiftr", r"swiftr"),
        l!(r"tescr", r"GLSLr", r"glslr"),
        l!(r"teser", r"GLSLr", r"glslr"),
        l!(r"texr", r"TeXr", r"texr"),
        l!(r"tomlr", r"TOMLr", r"tomlr"),
        l!(r"tsr", r"TypeScriptr", r"typescriptr"),
        l!(r"vertr", r"GLSLr", r"glslr"),
        l!(r"xhtmlr", r"XHTMLr", r"xhtmlr"),
        l!(r"xmlr", r"XMLr", r"xmlr"),
        l!(r"yamlr", r"YAMLr", r"yamlr"),
        l!(r"ymlr", r"YAMLr", r"yamlr"),
    ];
}
