//! # **The C Code Generator for Rust.**
//!
//! C-Emit provides a polished Builder API for generating C Code.
//!
//! ## Example
//!
//! ```rust
//! use c_emit::{Code, CArg};
//!
//! let mut code = Code::new();
//!
//! code.include("stdio.h");
//! code.call_func_with_args("printf", vec![CArg::String("Hello, world!")]);
//! assert_eq!(code.to_string(), r#"
//! #include<stdio.h>
//! int main() {
//! printf("Hello, world!");
//! return 0;
//! }
//! "#.trim_start().to_string());
//! ```

#![deny(missing_docs)]

use std::fmt::{Display, Formatter};

/// # The Code Struct.
///
/// ## Example
///
/// ```rust
/// use c_emit::Code;
///
/// let mut code = Code::new();
///
/// code.exit(1);
///
/// assert_eq!(code.to_string(), r#"
/// int main() {
/// return 1;
/// }
/// "#.trim_start().to_string());
/// ```
pub struct Code<'a> {
    code: String,
    requires: Vec<&'a str>,
    exit: i32,
}

/// # The C Argument.
pub enum CArg<'a> {
    /// The String argument.
    String(&'a str),

    /// The identifier argument.
    Ident(&'a str),

    /// The i32 argument.
    Int32(i32),

    /// The i64 argument.
    Int64(i64),

    /// The float argument.
    Float(f32),

    /// The 'double' argument.
    Double(f64),

    /// The boolean argument.
    Bool(bool),

    /// The character argument.
    Char(char),
}

/// # The C Argument.
pub enum VarInit<'a> {
    /// Initialize a string.
    String(&'a str),

    /// Initialize an i32.
    Int32(i32),

    /// Initialize an i64.
    Int64(i64),

    /// Initialize a float.
    Float(f32),

    /// Initialize a 'double'.
    Double(f64),

    /// Initialize a boolean.
    Bool(bool),

    /// Initialize a character.
    Char(char),

    /// **(FOR STRINGS ONLY!)** Set the variable to uninitialized with a specific size.
    SizeString(usize),
}

impl Default for Code<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl Code<'_> {
    /// # Create a new C Code object.
    ///
    /// ## Example
    /// ```rust
    /// use c_emit::Code;
    ///
    /// let code = Code::new();
    ///
    /// assert_eq!(code.to_string(), r#"
    /// int main() {
    /// return 0;
    /// }
    /// "#.trim_start().to_string());
    /// ```
    pub fn new() -> Self {
        Self {
            code: String::new(),
            requires: vec![],
            exit: 0,
        }
    }

    /// # Add the exit code to the main function.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use c_emit::Code;
    ///
    /// let mut code = Code::new();
    ///
    /// code.exit(1);
    ///
    /// assert_eq!(code.to_string(), r#"
    /// int main() {
    /// return 1;
    /// }
    /// "#.trim_start().to_string());
    /// ```
    pub fn exit(&mut self, code: i32) {
        self.exit = code;
    }

    /// # #include < any file into the C Code. >
    ///
    /// ## Example
    ///
    /// ```rust
    /// use c_emit::Code;
    ///
    /// let mut code = Code::new();
    ///
    /// code.include("stdio.h");
    ///
    /// assert_eq!(code.to_string(), r#"
    /// #include<stdio.h>
    /// int main() {
    /// return 0;
    /// }
    /// "#.trim_start().to_string());
    /// ```
    pub fn include(&mut self, file: &'static str) {
        if self.requires.contains(&file) {
            return;
        }
        self.requires.push(file);
    }

    /// # Call a function WITHOUT arguments.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use c_emit::Code;
    ///
    /// let mut code = Code::new();
    ///
    /// code.call_func("printf");
    ///
    /// assert_eq!(code.to_string(), r#"
    /// int main() {
    /// printf();
    /// return 0;
    /// }
    /// "#.trim_start().to_string());
    /// ```
    pub fn call_func(&mut self, func: &str) {
        self.code.push_str(func);
        self.code.push_str("();\n")
    }

    /// # Call a function WITH arguments.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use c_emit::{Code, CArg};
    ///
    /// let mut code = Code::new();
    ///
    /// code.call_func_with_args("printf", vec![CArg::String("Hello, world!")]);
    ///
    /// assert_eq!(code.to_string(), r#"
    /// int main() {
    /// printf("Hello, world!");
    /// return 0;
    /// }
    /// "#.trim_start().to_string());
    /// ```
    pub fn call_func_with_args(&mut self, func: &str, args: Vec<CArg>) {
        self.code.push_str(func);
        self.code.push('(');

        for arg in args {
            match arg {
                CArg::String(s) => {
                    let s = s.replace("\r\n", "\\r\\n");
                    let s = s.replace('\n', "\\n");
                    let s = s.replace('\t', "\\t");
                    let s = s.replace('"', "\\\"");

                    self.code.push('"');
                    self.code.push_str(s.as_str());
                    self.code.push('"');
                }
                CArg::Ident(id) => {
                    self.code.push_str(id);
                }
                CArg::Int32(n) => {
                    self.code.push_str(&n.to_string());
                }
                CArg::Int64(n) => {
                    self.code.push_str(&n.to_string());
                }
                CArg::Float(n) => {
                    self.code.push_str(&n.to_string());
                }
                CArg::Double(n) => {
                    self.code.push_str(&n.to_string());
                }
                CArg::Bool(b) => {
                    self.code.push_str(&b.to_string());
                }
                CArg::Char(c) => {
                    self.code.push(c);
                }
            }
            self.code.push(',');
        }

        if self.code.ends_with(',') {
            self.code = self.code.strip_suffix(',').unwrap().to_string();
        }

        self.code.push_str(");\n")
    }

    /// # Make a new variable.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use c_emit::{Code, CArg};
    ///
    /// let mut code = Code::new();
    ///
    /// code.new_var("a", VarInit::Initval("hello"));
    ///
    /// assert_eq!(code.to_string(), r#"
    /// int main() {
    /// char a[]="hello";
    /// return 0;
    /// }
    /// "#.trim_start().to_string());
    ///
    /// ```
    /// ## NOTE:
    /// Set the `initval` argument to `None` to make the variable uninitialized.
    pub fn new_var<S: AsRef<str>>(&mut self, name: S, value: VarInit) {
        let name = name.as_ref();

        match value {
            VarInit::String(s) => {
                self.code.push_str("char ");
                self.code.push_str(name);

                self.code.push_str("[]=\"");
                self.code.push_str(s);
                self.code.push_str("\";");
                self.code.push('\n');
            }
            VarInit::Bool(b) => {
                self.requires.push("stdbool.h");

                self.code.push_str("bool ");
                self.code.push_str(name);

                self.code.push('=');
                self.code.push_str(&b.to_string());
                self.code.push_str(";\n");
            }
            VarInit::Char(c) => {
                self.code.push_str("char ");
                self.code.push_str(name);

                self.code.push_str("='");
                self.code.push(c);
                self.code.push_str("';\n");
            }
            VarInit::Double(f) => {
                self.code.push_str("double ");
                self.code.push_str(name);

                self.code.push_str("=");
                self.code.push_str(&f.to_string());
                self.code.push_str(";\n");
            }
            VarInit::Float(f) => {
                self.code.push_str("float ");
                self.code.push_str(name);

                self.code.push_str("=");
                self.code.push_str(&f.to_string());
                self.code.push_str(";\n");
            }
            VarInit::Int32(i) => {
                self.code.push_str("int ");
                self.code.push_str(name);

                self.code.push_str("=");
                self.code.push_str(&i.to_string());
                self.code.push_str(";\n");
            }
            VarInit::Int64(i) => {
                self.code.push_str("int ");
                self.code.push_str(name);

                self.code.push_str("=");
                self.code.push_str(&i.to_string());
                self.code.push_str(";\n");
            }
            VarInit::SizeString(size) => {
                self.code.push_str("char ");
                self.code.push_str(name);

                self.code.push('[');
                self.code.push_str(&size.to_string());
                self.code.push_str("];\n");
            }
        }
    }
}

impl Display for Code<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut require_string = String::new();

        for require in &self.requires {
            require_string.push_str("#include<");
            require_string.push_str(require);
            require_string.push_str(">\n");
        }

        writeln!(
            f,
            "{}int main() {{\n{}return {};\n}}",
            require_string, self.code, self.exit
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let code = Code::new();

        assert_eq!(code.to_string(), "int main() {\nreturn 0;\n}\n");
    }

    #[test]
    fn test_exit_zero() {
        let mut code = Code::new();

        code.exit(0);

        assert_eq!(code.to_string(), "int main() {\nreturn 0;\n}\n");
    }

    #[test]
    fn test_exit_non_zero() {
        let mut code = Code::new();

        code.exit(1);

        assert_eq!(code.to_string(), "int main() {\nreturn 1;\n}\n");
    }

    #[test]
    fn test_multiple_exits() {
        let mut code = Code::new();

        code.exit(0);
        code.exit(1);

        assert_eq!(code.to_string(), "int main() {\nreturn 1;\n}\n");
    }

    #[test]
    fn test_include_valid() {
        let mut code = Code::new();

        code.include("stdio.h");

        assert!(code.to_string().contains("#include<stdio.h>"));
    }

    #[test]
    fn test_func_no_args() {
        let mut code = Code::new();

        code.call_func("printf");

        assert!(code.to_string().contains("printf();"));
    }

    #[test]
    fn test_func_with_args() {
        let mut code = Code::new();

        code.call_func_with_args("printf", vec![CArg::String("Hello")]);

        assert!(code.to_string().contains("printf(\"Hello\");"));
    }

    #[test]
    fn test_variable_string() {
        let mut code = Code::new();

        code.new_var("msg", VarInit::String("Hello"));

        assert!(code.to_string().contains("char msg[]=\"Hello\";"));
    }
}
