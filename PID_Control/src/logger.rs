use colored::{Color, Colorize};

const INFO: &str = "[INFO]";
const OK: &str = "[OK]";
const WARN: &str = "[WARN]";
const ERROR: &str = "[ERROR]";
const DEBUG: &str = "[DEBUG]";

fn print_c_helper(variable: & impl AsRef<str>) -> Color {
    return match variable.as_ref() {
        INFO => Color::Blue,
        OK => Color::Green,
        WARN => Color::Yellow,
        ERROR => Color::Red,
        DEBUG => Color::Magenta,
        _ => "".to_string().into(),
    }

}

fn print_c(category: impl AsRef<str>, subsystem: impl AsRef<str>, msg: impl AsRef<str>) {

    let color = print_c_helper(&category);
    let _category = category.as_ref().color(color);
    let _subsystem = subsystem.as_ref().color(color);

    print!("{} ", _category);

    if !subsystem.as_ref().is_empty() {
        print!("{} ", _subsystem);
    } else {
        print!("")
    }

    println!("{}", msg.as_ref().to_string());
}

pub fn info(subsystem: impl AsRef<str>, msg: impl AsRef<str>) {
    print_c(INFO, subsystem, msg);
}

pub fn success(subsystem: impl AsRef<str>, msg: impl AsRef<str>) {
    print_c(OK, subsystem, msg);
}

pub fn warn(subsystem: impl AsRef<str>, msg: impl AsRef<str>) {
    print_c(WARN, subsystem, msg);
}


pub fn error(subsystem: impl AsRef<str>, msg: impl AsRef<str>) {
    print_c(ERROR, subsystem, msg);
}


pub fn debug(subsystem: impl AsRef<str>, msg: impl AsRef<str>) {
    print_c(DEBUG, subsystem, msg);
}