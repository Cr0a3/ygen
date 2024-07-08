#![allow(unused_assignments)]

const ANSI_RESET: &str = "\x1b[0m";

/// Color style
pub struct Style {
    /// If the text is bold
    pub bold: bool,
    /// If the text is italic
    pub italic: bool,
    /// If the text is underlined
    pub underlined: bool,
    /// If the text is striketrough
    pub strike: bool,

    /// The background rgb color red channel
    pub bg_r: u8,
    /// The background rgb color green channel
    pub bg_g: u8,
    /// The background rgb color blue channel
    pub bg_b: u8,
    /// If the text has a background color
    pub bg: bool,
}

/// Clorize trait: Colorizes a string
pub trait Colorize {
    /// Makes the string to the specified color
    fn color(&self, r: i16, g: i16, b: i16) -> String;
    /// Makes the string black
    fn black(&self) -> String;
    /// Makes the string red
    fn red(&self) -> String;
    /// Makes the string green
    fn green(&self) -> String;
    /// Makes the string yellow
    fn yellow(&self) -> String;
    /// Makes the string blue
    fn blue(&self) -> String;
    /// Makes the string magenta
    fn magenta(&self) -> String;
    /// Makes the string cyan
    fn cyan(&self) -> String;
    /// Makes the string white
    fn white(&self) -> String;
    /// Makes the string gray
    fn gray(&self) -> String;

    /// Makes the string to a specified background color
    fn bg_color(&self, r: u8, g: u8, b: u8) -> String;
    /// Makes the background of the string black
    fn bg_black(&self) -> String;
    /// Makes the background of the string red
    fn bg_red(&self) -> String;
    /// Makes the background of the string green
    fn bg_green(&self) -> String;
    /// Makes the background of the string yellow
    fn bg_yellow(&self) -> String;
    /// Makes the background of the string blue
    fn bg_blue(&self) -> String;
    /// Makes the background of the string magenta
    fn bg_magenta(&self) -> String;
    /// Makes the background of the string cyan
    fn bg_cyan(&self) -> String;
    /// Makes the background of the string white
    fn bg_white(&self) -> String;
    /// Makes the background of the string gray
    fn bg_gray(&self) -> String;

    /// Makes the string bold
    fn bold(&self) -> String;
    /// Makes the string italitc
    fn italic(&self) -> String;
    /// Makes the string underlined
    fn underline(&self) -> String;
    /// Makes the string striketrough
    fn strike(&self) -> String;
}

impl<'a> Colorize for &'a str {
    fn color(&self, r: i16, g: i16, b: i16) -> String {
        ColoredString::new(
            r,
            g,
            b,
            Style {
                bold: false,
                italic: false,
                underlined: false,
                strike: false,
                bg_r: 0, bg_b: 0, bg_g: 0, bg: false,
            },
            self,
        )
        .to_string()
    }

    fn white(&self) -> String {
        self.color(255, 255, 255)
    }

    fn black(&self) -> String {
        self.color(1, 1, 1)
    }

    fn red(&self) -> String {
        self.color(205, 49, 49)
    }

    fn green(&self) -> String {
        self.color(13, 188, 121)
    }

    fn yellow(&self) -> String {
        self.color(229, 229, 16)
    }

    fn blue(&self) -> String {
        self.color(36, 114, 200)
    }

    fn magenta(&self) -> String {
        self.color(188, 63, 188)
    }

    fn cyan(&self) -> String {
        self.color(17, 168, 205)
    }

    fn gray(&self) -> String {
        self.color(118, 118, 118)
    }

    fn bold(&self) -> String {
        ColoredString::new(
            -1,
            -1,
            -1,
            Style {
                bold: true,
                italic: false,
                underlined: false,
                strike: false,
                bg_r: 0, bg_b: 0, bg_g: 0, bg: false
            },
            self,
        )
        .to_string()
    }

    fn italic(&self) -> String {
        ColoredString::new(
            -1,
            -1,
            -1,
            Style {
                bold: false,
                italic: true,
                underlined: false,
                strike: false,
                bg_r: 0, bg_b: 0, bg_g: 0, bg: false
            },
            self,
        )
        .to_string()
    }

    fn underline(&self) -> String {
        ColoredString::new(
            -1,
            -1,
            -1,
            Style {
                bold: false,
                italic: false,
                underlined: true,
                strike: false,
                bg_r: 0, bg_b: 0, bg_g: 0, bg: false
            },
            self,
        )
        .to_string()
    }

    fn strike(&self) -> String {
        ColoredString::new(
            -1,
            -1,
            -1,
            Style {
                bold: false,
                italic: false,
                underlined: false,
                strike: true,
                bg_r: 0, bg_b: 0, bg_g: 0, bg: false
            },
            self,
        )
        .to_string()
    }
    
    fn bg_color(&self, r: u8, g: u8, b: u8) -> String {
        ColoredString::new(
            -1,
            -1,
            -1,
            Style {
                bold: false,
                italic: false,
                underlined: false,
                strike: false,
                bg_r: r, bg_b: g, bg_g: b, bg: true
            },
            self,
        )
        .to_string()
    }
    
    fn bg_white(&self) -> String {
        self.bg_color(255, 255, 255)
    }

    fn bg_black(&self) -> String {
        self.bg_color(1, 1, 1)
    }

    fn bg_red(&self) -> String {
        self.bg_color(205, 49, 49)
    }

    fn bg_green(&self) -> String {
        self.bg_color(36, 114, 200)
    }

    fn bg_yellow(&self) -> String {
        self.bg_color(229, 229, 16)
    }

    fn bg_blue(&self) -> String {
        self.bg_color(13, 188, 121)
    }

    fn bg_magenta(&self) -> String {
        self.bg_color(188, 63, 188)
    }

    fn bg_cyan(&self) -> String {
        self.bg_color(17, 168, 205)
    }

    fn bg_gray(&self) -> String {
        self.bg_color(118, 118, 118)
    }
}

impl Colorize for String {
    fn color(&self, r: i16, g: i16, b: i16) -> String {
        ColoredString::new(
            r,
            g,
            b,
            Style {
                bold: false,
                italic: false,
                underlined: false,
                strike: false,
                bg_r: 0, bg_b: 0, bg_g: 0, bg: false
            },
            self,
        )
        .to_string()
    }

    fn white(&self) -> String {
        self.color(255, 255, 255)
    }

    fn black(&self) -> String {
        self.color(1, 1, 1)
    }

    fn red(&self) -> String {
        self.color(205, 49, 49)
    }

    fn green(&self) -> String {
        self.color(13, 188, 121)
    }

    fn yellow(&self) -> String {
        self.color(229, 229, 16)
    }

    fn blue(&self) -> String {
        self.color(36, 114, 200)
    }

    fn magenta(&self) -> String {
        self.color(188, 63, 188)
    }

    fn cyan(&self) -> String {
        self.color(17, 168, 205)
    }

    fn gray(&self) -> String {
        self.color(118, 118, 118)
    }

    fn bold(&self) -> String {
        ColoredString::new(
            -1,
            -1,
            -1,
            Style {
                bold: true,
                italic: false,
                underlined: false,
                strike: false,
                bg_r: 0, bg_b: 0, bg_g: 0, bg: false
            },
            self,
        )
        .to_string()
    }

    fn italic(&self) -> String {
        ColoredString::new(
            -1,
            -1,
            -1,
            Style {
                bold: false,
                italic: true,
                underlined: false,
                strike: false,
                bg_r: 0, bg_b: 0, bg_g: 0, bg: false
            },
            self,
        )
        .to_string()
    }

    fn underline(&self) -> String {
        ColoredString::new(
            -1,
            -1,
            -1,
            Style {
                bold: false,
                italic: false,
                underlined: true,
                strike: false,
                bg_r: 0, bg_b: 0, bg_g: 0, bg: false
            },
            self,
        )
        .to_string()
    }

    fn strike(&self) -> String {
        ColoredString::new(
            -1,
            -1,
            -1,
            Style {
                bold: false,
                italic: false,
                underlined: false,
                strike: true,
                bg_r: 0, bg_b: 0, bg_g: 0, bg: false
            },
            self,
        )
        .to_string()
    }

    fn bg_color(&self, r: u8, g: u8, b: u8) -> String {
        ColoredString::new(
            -1,
            -1,
            -1,
            Style {
                bold: false,
                italic: false,
                underlined: false,
                strike: false,
                bg_r: r, bg_b: g, bg_g: b, bg: true
            },
            self,
        )
        .to_string()
    }
    
    fn bg_white(&self) -> String {
        self.bg_color(255, 255, 255)
    }

    fn bg_black(&self) -> String {
        self.bg_color(1, 1, 1)
    }

    fn bg_red(&self) -> String {
        self.bg_color(205, 49, 49)
    }

    fn bg_green(&self) -> String {
        self.bg_color(13, 188, 121)
    }

    fn bg_yellow(&self) -> String {
        self.bg_color(229, 229, 16)
    }

    fn bg_blue(&self) -> String {
        self.bg_color(36, 114, 200)
    }

    fn bg_magenta(&self) -> String {
        self.bg_color(188, 63, 188)
    }

    fn bg_cyan(&self) -> String {
        self.bg_color(17, 168, 205)
    }

    fn bg_gray(&self) -> String {
        self.bg_color(118, 118, 118)
    }
}

pub(crate) struct ColoredString {
    r: i16,
    g: i16,
    b: i16,

    attr: Style,
    str: String,
}

impl ColoredString {
    pub fn new(_r: i16, _g: i16, _b: i16, _attr: Style, _str: &str) -> Self {
        Self {
            r: _r,
            g: _g,
            b: _b,
            attr: _attr,
            str: _str.into(),
        }
    }
}

impl ToString for ColoredString {
    fn to_string(&self) -> String {
        if self.r != -1 {
            let rgb_str = format!(
                "\x1b[38;2;{};{};{}",
                self.r.to_string(),
                self.g.to_string(),
                self.b.to_string()
            );

            return format!("{rgb_str}m{}{ANSI_RESET}", self.str);
        }

        let style = &self.attr;

        if style.bold {
            let rgb_str = format!("\x1b[1m");
            return format!("{rgb_str}{}{ANSI_RESET}", self.str);
        }
        if style.italic {
            let rgb_str = format!("\x1b[3m");
            return format!("{rgb_str}{}{ANSI_RESET}", self.str);
        }
        if style.underlined {
            let rgb_str = format!("\x1b[4m");
            return format!("{rgb_str}{}{ANSI_RESET}", self.str);
        }
        if style.strike {
            let rgb_str = format!("\x1b[9m");
            return format!("{rgb_str}{}{ANSI_RESET}", self.str);
        }

        if style.bg {
            let rgb_str = format!("\x1b[48;2;{};{};{}",
                self.attr.bg_r.to_string(),
                self.attr.bg_g.to_string(),
                self.attr.bg_b.to_string(),);

            return format!("{rgb_str}m{}{ANSI_RESET}", self.str);
        }

        String::new()
    }
}

/// Colore encoder:
/// Encodes a string by a specifed format
/// E.g: <blue>Hi I am blue<green>Hi I am green
/// For that use the `encode` function
pub struct ColorEncoder {}

impl ColorEncoder {
    /// Encodes a string by a specifed format
    /// E.g: <blue>Hi I am blue<green>Hi I am green
    pub fn encode(string: &str) -> String {
        let mut str: String = string.into();

        str = str.replace("<black>", &"".black());
        str = str.replace("<red>", &"".red());
        str = str.replace("<blue>", &"".blue());
        str = str.replace("<green>", &"".green());
        str = str.replace("<yellow>", &"".yellow());
        str = str.replace("<magenta>", &"".magenta());
        str = str.replace("<cyan>", &"".cyan());
        str = str.replace("<white>", &"".white());
        str = str.replace("<gray>", &"".gray());
        str = str.replace("<bold>", &"".bold());
        str = str.replace("<italic>", &"".italic());
        str = str.replace("<underline>", &"".underline());
        str = str.replace("<strike>", &"".strike());
        str = str.replace(ANSI_RESET, "");

        let chars = str.chars();
        let mut l_c: char = '\0';
        let mut clr_b = false;
        let mut clr_str: String = String::new();

        for c in chars {
            if c == '>' && clr_b {
                clr_b = false;
                break;
            }

            if clr_b {
                clr_str += &String::from(c);
            }

            if c == '&' && l_c == '<' {
                clr_b = true;
            }

            l_c = c;
        }
        let mut _r: i16 = -1; let mut _g: i16 = -1; let mut _b: i16 = -1;

        let str_len = clr_str.chars().count();

        if str_len == 6 {
            let r = i16::from_str_radix(&clr_str[0..2], 16).ok();
            let g = i16::from_str_radix(&clr_str[2..4], 16).ok();
            let b = i16::from_str_radix(&clr_str[4..6], 16).ok();

            if !r.is_none() { _r = r.unwrap(); } else {
                println!("{} {}", "Error: ".red(), "red color channel in encoded color string is null"); }
            if !r.is_none() { _g = g.unwrap(); } else {
                println!("{} {}", "Error: ".red(), "green color channel in encoded color string is null"); }
            if !r.is_none() { _b = b.unwrap(); } else {
                println!("{} {}", "Error: ".red(), "blue color channel in encoded color string is null"); }

            if _r != -1 && _g != -1 && _b != -1 {
                str = str.replace(
                    &format!("<&{}>", clr_str),
                        &"".color(_r, _g, _b).replace(ANSI_RESET, ""));
            }
        }

        str + ANSI_RESET
    }
} 