use std::fmt::{Display, Formatter};

pub struct Code {
    code: String,
    requires: Vec<String>,
}

impl Code {
    pub fn new() -> Self {
        Self {
            code: String::new(),
            requires: vec![],
        }
    }
    pub fn include(&mut self, file: &str) {
        self.requires.push(file.to_string());
    }
    pub fn call_func(&mut self, func: &str) {
        self.code.push_str(func);
        self.code.push_str("();\n")
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

        writeln!(f, "{}int main() {{\n{}}}", require_string, self.code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let code = Code::new();

        assert_eq!(code.to_string(), "int main() {\n}\n");
    }
    #[test]
    fn test_include() {
        let mut code = Code::new();

        code.include("stdio.h");

        assert_eq!(code.to_string(), "#include<stdio.h>\nint main() {\n}\n");
    }
    #[test]
    fn test_func() {
        let mut code = Code::new();

        code.call_func("printf");

        assert_eq!(code.to_string(), "int main() {\nprintf();\n}\n");
    }
}
