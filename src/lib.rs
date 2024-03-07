//! # **The C Code Generator for Rust.**
//!
//! C-Emit provides a polished Builder API for generating C Code.
//!
//! ## Example
//!
//! ```rust
//! use c_emit::{Code, CArg};
//!
//! fn main() {
//!     let mut code = Code::new();
//!
//!     code.include("stdio.h");
//!     code.call_func_with_args("printf", vec![CArg::String("Hello, world!".to_string())]);
//!
//!     assert_eq!(code.to_string(), r#"
//! #include<stdio.h>
//! int main() {
//! printf("Hello, world!");
//! return 0;
//! }
//! "#.trim_start().to_string());
//! }
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
/// fn main() {
///     let mut code = Code::new();
///
///     code.exit(1);
///
///     assert_eq!(code.to_string(), r#"
/// int main() {
/// return 1;
/// }
/// "#.trim_start().to_string());
/// }
/// ```
pub struct Code {
    code: String,
    requires: Vec<String>,
    exit: i32
}

/// # The C Argument.
pub enum CArg {
    /// The String argument.
    String(String)
}

impl Code {
    /// # Create a new C Code object.
    ///
    /// ## Example
    /// ```rust
    /// use c_emit::Code;
    ///
    /// fn main() {
    ///     let code = Code::new();
    ///
    ///     assert_eq!(code.to_string(), r#"
    /// int main() {
    /// return 0;
    /// }
    /// "#.trim_start().to_string());
    /// }
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
    /// fn main() {
    ///     let mut code = Code::new();
    ///
    ///     code.exit(1);
    ///
    ///     assert_eq!(code.to_string(), r#"
    /// int main() {
    /// return 1;
    /// }
    /// "#.trim_start().to_string());
    /// }
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
    /// fn main() {
    ///     let mut code = Code::new();
    ///
    ///     code.include("stdio.h");
    ///
    ///     assert_eq!(code.to_string(), r#"
    /// #include<stdio.h>
    /// int main() {
    /// return 0;
    /// }
    /// "#.trim_start().to_string());
    /// }
    /// ```
    pub fn include(&mut self, file: &str) {
        self.requires.push(file.to_string());
    }

    /// # Call a function WITHOUT arguments.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use c_emit::Code;
    ///
    /// fn main() {
    ///     let mut code = Code::new();
    ///
    ///     code.call_func("printf");
    ///
    ///     assert_eq!(code.to_string(), r#"
    /// int main() {
    /// printf();
    /// return 0;
    /// }
    /// "#.trim_start().to_string());
    /// }
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
    /// fn main() {
    ///     let mut code = Code::new();
    ///
    ///     code.call_func_with_args("printf", vec![CArg::String("Hello, world!".to_string())]);
    ///
    ///     assert_eq!(code.to_string(), r#"
    /// int main() {
    /// printf("Hello, world!");
    /// return 0;
    /// }
    /// "#.trim_start().to_string());
    /// }
    /// ```
    pub fn call_func_with_args(&mut self, func: &str, args: Vec<CArg>) {
        self.code.push_str(func);
        self.code.push_str("(");

        for arg in args {
            match arg {
                CArg::String(s) => {
                    let s = s.replace("\r\n","\\r\\n");
                    let s = s.replace('\n',"\\n");
                    let s = s.replace('\t', "\\t");
                    let s = s.replace('"', "\\\"");

                    self.code.push('"');
                    self.code.push_str(s.as_str());
                    self.code.push('"');
                }
            }
            self.code.push(',');
        }

        if self.code.ends_with(',') {
            self.code = self.code.strip_suffix(',').unwrap().to_string();
        }

        self.code.push_str(");\n")
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut require_string = String::new();

        for require in &self.requires {
            require_string.push_str("#include<");
            require_string.extend(require.chars());
            require_string.push_str(">\n");
        }

        writeln!(f, "{}int main() {{\n{}return {};\n}}", require_string, self.code, self.exit)
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

        assert_eq!(code.to_string(), "#include<stdio.h>\nint main() {\nreturn 0;\n}\n");
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

        code.call_func_with_args("printf", vec![CArg::String("Hello World! \"How are you?\"\n \r\n \t".to_string()), CArg::String("Hi".to_string())]);


        assert_eq!(code.to_string(), "int main() {\nprintf(\"Hello World! \\\"How are you?\\\"\\n \\r\\n \\t\",\"Hi\");\nreturn 0;\n}\n");
    }
}
