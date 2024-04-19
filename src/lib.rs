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
//! code.call_func_with_args("printf", vec![CArg::String("Hello, world!".to_string())]);
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
    /// code.call_func_with_args("printf", vec![CArg::String("Hello, world!".to_string())]);
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
            }
            self.code.push(',');
        }

        if self.code.ends_with(',') {
            self.code = self.code.strip_suffix(',').unwrap().to_string();
        }

        self.code.push_str(");\n")
    }

    /// # Make a new string variable.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use c_emit::{Code, CArg};
    ///
    /// let mut code = Code::new();
    ///
    /// code.new_var_string("a", Some("hello"), None);
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
    pub fn new_var_string<S: AsRef<str>>(
        &mut self,
        name: S,
        initval: Option<S>,
        size: Option<u32>,
    ) {
        self.code.push_str("char ");
        self.code.push_str(name.as_ref());

        if initval.is_none() {
            self.code.push('[');
            self.code
                .push_str(&size.expect("Expected size if uninitialized.").to_string());
            self.code.push_str("];");
        } else {
            self.code.push_str("[]=\"");
            if let Some(val) = initval {
                self.code.push_str(val.as_ref());
            }
            self.code.push_str("\";\n");
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
    fn test_exit() {
        let mut code = Code::new();

        code.exit(1);

        assert_eq!(code.to_string(), "int main() {\nreturn 1;\n}\n");
    }
    #[test]
    fn test_include() {
        let mut code = Code::new();

        code.include("stdio.h");

        assert_eq!(
            code.to_string(),
            "#include<stdio.h>\nint main() {\nreturn 0;\n}\n"
        );
    }
    #[test]
    fn test_func() {
        let mut code = Code::new();

        code.call_func("printf");

        assert_eq!(code.to_string(), "int main() {\nprintf();\nreturn 0;\n}\n");
    }
    #[test]
    fn test_func_with_args() {
        let mut code = Code::new();

        code.call_func_with_args(
            "printf",
            vec![
                CArg::String("Hello World! \"How are you?\"\n \r\n \t"),
                CArg::String("Hi"),
            ],
        );

        assert_eq!(code.to_string(), "int main() {\nprintf(\"Hello World! \\\"How are you?\\\"\\n \\r\\n \\t\",\"Hi\");\nreturn 0;\n}\n");
    }
    #[test]
    fn test_multiple_requires() {
        let mut code = Code::new();

        code.include("stdio.h");
        code.include("stdio.h");

        assert_eq!(
            code.to_string(),
            "#include<stdio.h>\nint main() {\nreturn 0;\n}\n"
        );
    }

    #[test]
    fn test_variable_string() {
        let mut code = Code::new();

        code.new_var_string("a", Some("Hi"), None);

        assert_eq!(
            code.to_string(),
            "int main() {\nchar a[]=\"Hi\";\nreturn 0;\n}\n"
        );
    }

    #[test]
    fn test_variable_string_arg() {
        let mut code = Code::new();

        code.new_var_string("a", Some("Hi"), None);
        code.call_func_with_args("printf", vec![CArg::Ident("a")]);

        assert_eq!(
            code.to_string(),
            "int main() {\nchar a[]=\"Hi\";\nprintf(a);\nreturn 0;\n}\n"
        );
    }
}
