use std::fmt::{Display, Formatter};

pub struct Code {
    code: String,
    requires: Vec<String>,
    exit: i32
}

pub enum CArg {
    String(String)
}

impl Code {
    pub fn new() -> Self {
        Self {
            code: String::new(),
            requires: vec![],
            exit: 0,
        }
    }
    pub fn exit(&mut self, code: i32) {
        self.exit = code;
    }
    pub fn include(&mut self, file: &str) {
        self.requires.push(file.to_string());
    }
    pub fn call_func(&mut self, func: &str) {
        self.code.push_str(func);
        self.code.push_str("();\n")
    }
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

        code.call_func_with_args("printf", vec![CArg::String("Hello World! \"How are you?\"\n \r\n \t".to_string())]);


        assert_eq!(code.to_string(), "int main() {\nprintf(\"Hello World! \\\"How are you?\\\"\\n \\r\\n \\t\");\nreturn 0;\n}\n");
    }
}
