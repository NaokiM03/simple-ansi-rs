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
    }
}
