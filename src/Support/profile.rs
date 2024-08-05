use super::{Color, Colorize};

/// The user color profile is used to store how ir elements are colored
/// For example instructions are red
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorProfile {
    instr: Color,
    ty: Color,
    var: Color,
    name: Color,
    value: Color,
}

impl ColorProfile {
    /// Creates new color settings which are defaulted to black
    pub fn new() -> Self {
        Self {
            instr: Color::default(),
            ty: Color::default(),
            var: Color::default(),
            name: Color::default(),
            value: Color::default(),
        }
    }

    /// Sets the color of instr class
    pub fn instr(&mut self, clr: Color) -> &mut Self {
        self.instr = clr;
        self
    }

    /// Sets the color of ty class
    pub fn ty(&mut self, clr: Color) -> &mut Self {
        self.ty = clr;
        self
    }

    /// Sets the color of var class
    pub fn var(&mut self, clr: Color) -> &mut Self {
        self.var = clr;
        self
    }

    /// Sets the color of name class
    pub fn name(&mut self, clr: Color) -> &mut Self {
        self.name = clr;
        self
    }

    /// Sets the color of value class
    pub fn value(&mut self, clr: Color) -> &mut Self {
        self.value = clr;
        self
    }

    /// Markups the given string by the color class
    pub fn markup(&self, string: &str, class: ColorClass) -> String {
        match class {
            ColorClass::Instr => string.color(self.instr.r as i16, self.instr.g as i16, self.instr.b as i16),
            ColorClass::Ty => string.color(self.ty.r as i16, self.instr.g as i16, self.instr.b as i16),
            ColorClass::Var => string.color(self.var.r as i16, self.instr.g as i16, self.instr.b as i16),
            ColorClass::Name => string.color(self.name.r as i16, self.instr.g as i16, self.instr.b as i16),
            ColorClass::Value => string.color(self.value.r as i16, self.value.g as i16, self.value.b as i16),
        }
    }
}

/// The color markup class
#[allow(missing_docs)]
pub enum ColorClass {
    Instr,
    Ty,
    Var,
    Name,
    Value,
}

impl Default for ColorProfile {
    fn default() -> Self {
        Self { 
            instr: Color { r: 36, g: 114, b: 200 }, 
            ty: Color { r: 13, g: 188, b: 121 }, 
            var: Color { r: 168, g: 63, b: 168 }, 
            name: Color { r: 17, g: 168, b: 205 },
            value: Color { r: 36, g: 114, b: 200 },
        }
    }
}