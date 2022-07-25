use ext_trait::extension;
#[extension(trait SimpleAnsi)]
impl str {
    fn bold(&self) -> String {
        format!("\u{1b}[1m{}\u{1b}[0m", self)
    }
    fn dimmed(&self) -> String {
        format!("\u{1b}[2m{}\u{1b}[0m", self)
    }
    fn italic(&self) -> String {
        format!("\u{1b}[3m{}\u{1b}[0m", self)
    }
    fn underline(&self) -> String {
        format!("\u{1b}[4m{}\u{1b}[0m", self)
    }
    fn blink(&self) -> String {
        format!("\u{1b}[5m{}\u{1b}[0m", self)
    }
    fn reverse(&self) -> String {
        format!("\u{1b}[7m{}\u{1b}[0m", self)
    }
    fn hidden(&self) -> String {
        format!("\u{1b}[8m{}\u{1b}[0m", self)
    }
    fn strikethrough(&self) -> String {
        format!("\u{1b}[9m{}\u{1b}[0m", self)
    }

    fn black(&self) -> String {
        format!("\u{1b}[30m{}\u{1b}[0m", self)
    }
    fn red(&self) -> String {
        format!("\u{1b}[31m{}\u{1b}[0m", self)
    }
    fn green(&self) -> String {
        format!("\u{1b}[32m{}\u{1b}[0m", self)
    }
    fn yellow(&self) -> String {
        format!("\u{1b}[33m{}\u{1b}[0m", self)
    }
    fn blue(&self) -> String {
        format!("\u{1b}[34m{}\u{1b}[0m", self)
    }
    fn magenta(&self) -> String {
        format!("\u{1b}[35m{}\u{1b}[0m", self)
    }
    fn cyan(&self) -> String {
        format!("\u{1b}[36m{}\u{1b}[0m", self)
    }
    fn white(&self) -> String {
        format!("\u{1b}[37m{}\u{1b}[0m", self)
    }
    fn bright_black(&self) -> String {
        format!("\u{1b}[90m{}\u{1b}[0m", self)
    }
    fn bright_red(&self) -> String {
        format!("\u{1b}[91m{}\u{1b}[0m", self)
    }
    fn bright_green(&self) -> String {
        format!("\u{1b}[92m{}\u{1b}[0m", self)
    }
    fn bright_yellow(&self) -> String {
        format!("\u{1b}[93m{}\u{1b}[0m", self)
    }
    fn bright_blue(&self) -> String {
        format!("\u{1b}[94m{}\u{1b}[0m", self)
    }
    fn bright_magenta(&self) -> String {
        format!("\u{1b}[95m{}\u{1b}[0m", self)
    }
    fn bright_cyan(&self) -> String {
        format!("\u{1b}[96m{}\u{1b}[0m", self)
    }
    fn bright_white(&self) -> String {
        format!("\u{1b}[97m{}\u{1b}[0m", self)
    }

    fn on_black(&self) -> String {
        format!("\u{1b}[40m{}\u{1b}[0m", self)
    }
    fn on_red(&self) -> String {
        format!("\u{1b}[41m{}\u{1b}[0m", self)
    }
    fn on_green(&self) -> String {
        format!("\u{1b}[42m{}\u{1b}[0m", self)
    }
    fn on_yellow(&self) -> String {
        format!("\u{1b}[43m{}\u{1b}[0m", self)
    }
    fn on_blue(&self) -> String {
        format!("\u{1b}[44m{}\u{1b}[0m", self)
    }
    fn on_magenta(&self) -> String {
        format!("\u{1b}[45m{}\u{1b}[0m", self)
    }
    fn on_cyan(&self) -> String {
        format!("\u{1b}[46m{}\u{1b}[0m", self)
    }
    fn on_white(&self) -> String {
        format!("\u{1b}[47m{}\u{1b}[0m", self)
    }
    fn on_bright_black(&self) -> String {
        format!("\u{1b}[100m{}\u{1b}[0m", self)
    }
    fn on_bright_red(&self) -> String {
        format!("\u{1b}[101m{}\u{1b}[0m", self)
    }
    fn on_bright_green(&self) -> String {
        format!("\u{1b}[102m{}\u{1b}[0m", self)
    }
    fn on_bright_yellow(&self) -> String {
        format!("\u{1b}[103m{}\u{1b}[0m", self)
    }
    fn on_bright_blue(&self) -> String {
        format!("\u{1b}[104m{}\u{1b}[0m", self)
    }
    fn on_bright_magenta(&self) -> String {
        format!("\u{1b}[105m{}\u{1b}[0m", self)
    }
    fn on_bright_cyan(&self) -> String {
        format!("\u{1b}[106m{}\u{1b}[0m", self)
    }
    fn on_bright_white(&self) -> String {
        format!("\u{1b}[107m{}\u{1b}[0m", self)
    }
}

#[cfg(test)]
mod tests {
    use crate::SimpleAnsi;

    #[test]
    fn test_show() {
        println!("{}", "bold".bold());
        println!("{}", "dimmed".dimmed());
        println!("{}", "italic".italic());
        println!("{}", "underline".underline());
        println!("{}", "blink".blink());
        println!("{}", "reverse".reverse());
        println!("{}", "hidden".hidden());
        println!("{}", "strikethrough".strikethrough());

        println!("{}", "black".black());
        println!("{}", "red".red());
        println!("{}", "green".green());
        println!("{}", "yellow".yellow());
        println!("{}", "blue".blue());
        println!("{}", "magenta".magenta());
        println!("{}", "cyan".cyan());
        println!("{}", "white".white());
        println!("{}", "bright_black".bright_black());
        println!("{}", "bright_red".bright_red());
        println!("{}", "bright_green".bright_green());
        println!("{}", "bright_yellow".bright_yellow());
        println!("{}", "bright_blue".bright_blue());
        println!("{}", "bright_magenta".bright_magenta());
        println!("{}", "bright_cyan".bright_cyan());
        println!("{}", "bright_white".bright_white());

        println!("{}", "on black".on_black());
        println!("{}", "on red".on_red());
        println!("{}", "on green".on_green());
        println!("{}", "on yellow".on_yellow());
        println!("{}", "on blue".on_blue());
        println!("{}", "on magenta".on_magenta());
        println!("{}", "on cyan".on_cyan());
        println!("{}", "on white".on_white());
        println!("{}", "on bright_black".on_bright_black());
        println!("{}", "on bright_red".on_bright_red());
        println!("{}", "on bright_green".on_bright_green());
        println!("{}", "on bright_yellow".on_bright_yellow());
        println!("{}", "on bright_blue".on_bright_blue());
        println!("{}", "on bright_magenta".on_bright_magenta());
        println!("{}", "on bright_cyan".on_bright_cyan());
        println!("{}", "on bright_white".on_bright_white());

        println!("{}", "italic yellow on blue".italic().yellow().on_blue());
    }
}
