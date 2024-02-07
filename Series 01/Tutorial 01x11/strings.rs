
fn main() {
    // newline character
    let n = "String that will be printed\nin several lines";

    // tab character
    let t = "S\tt\tr\ti\tn\tg";

    // other escapes
    let e = "\"\`\x78\u{211D}";

    // multi-line string with a line break escaped
    let s = "This is a 
    multi-line string
    where this line \
    is not broken";

    // raw string
    let r = r"this is a raw string where literals such as \ and ' and \xFF are shown as they are written here";

    // concatenation
    let a = "A";
    let b = "B";
    let c = "A".to_string() + b;
}