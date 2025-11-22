use colored::Colorize;

pub fn info(category: impl AsRef<str>, msg: impl AsRef<str>) {
    println!("{} {} {}", "[INFO]".blue(), category.as_ref().blue(), msg.as_ref().to_string())
}

pub fn success(category: impl AsRef<str>, msg: impl AsRef<str>) {
    println!("{} {} {}", "[OK]".green(), category.as_ref().green(), msg.as_ref().to_string());
}

pub fn warn(category: impl AsRef<str>, msg: impl AsRef<str>) {
    println!("{} {} {}", "[WARN]".yellow(), category.as_ref().yellow(), msg.as_ref().to_string());
}


pub fn error(category: impl AsRef<str>, msg: impl AsRef<str>) {
    println!("{} {} {}", "[ERROR]".red().bold(), category.as_ref().red().bold(), msg.as_ref().bold());
}


pub fn debug(category: impl AsRef<str>, msg: impl AsRef<str>) {
    println!("{} {} {}", "[DEBUG]".magenta().dimmed(), category.as_ref().magenta().dimmed(), msg.as_ref().dimmed());
}