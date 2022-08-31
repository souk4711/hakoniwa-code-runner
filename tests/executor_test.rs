#[cfg(test)]
mod executor_test {
    use hakoniwa_code_runner::{App, AppConfig, Executor, ExecutorFile};
    use lazy_static::lazy_static;
    use rust_embed::RustEmbed;
    use std::str;

    #[derive(RustEmbed)]
    #[folder = "./src/embed"]
    pub struct Embed;

    lazy_static! {
        static ref APP: App = {
            let config_data = str::from_utf8(&Embed::get("app.toml").unwrap().data)
                .unwrap()
                .to_string();
            let config = AppConfig::from_str(&config_data).unwrap();
            let mut app = App::new(config);
            app.initialize().unwrap();
            app
        };
    }

    fn executor(lang: &str) -> &Executor {
        APP.get_executor(lang).unwrap()
    }

    #[test]
    fn test_run_lang_c() {
        let result = executor("c").run(&[ExecutorFile::new(
            "main.c",
            r#"
#include <stdio.h>
    int main() {
    printf("Hello, World!\n");
    return 0;
}
            "#,
        )]);
        assert_eq!(result.status, "OK");
        assert_eq!(result.exit_code, Some(0));
        assert_eq!(result.stdout, "Hello, World!\n");
    }

    #[test]
    fn test_run_lang_cpp() {
        let result = executor("cpp").run(&[ExecutorFile::new(
            "main.cpp",
            r#"
#include <iostream>

int main() {
    std::cout << "Hello, World!\n";
    return 0;
}
            "#,
        )]);
        assert_eq!(result.status, "OK");
        assert_eq!(result.exit_code, Some(0));
        assert_eq!(result.stdout, "Hello, World!\n");
    }

    #[test]
    fn test_run_lang_go() {
        let result = executor("go").run(&[ExecutorFile::new(
            "main.go",
            r#"
package main

import "fmt"

func main() {
  fmt.Println("Hello, World!")
}
            "#,
        )]);
        assert_eq!(result.status, "OK");
        assert_eq!(result.exit_code, Some(0));
        assert_eq!(result.stdout, "Hello, World!\n");
    }

    #[test]
    fn test_run_lang_java() {
        let result = executor("java").run(&[ExecutorFile::new(
            "main.java",
            r#"
class Main {
  public static void main(String[] args) {
    System.out.println("Hello, World!");
  }
}
            "#,
        )]);
        assert_eq!(result.status, "OK");
        assert_eq!(result.exit_code, Some(0));
        assert_eq!(result.stdout, "Hello, World!\n");
    }

    #[test]
    fn test_run_lang_javascript() {
        let result = executor("javascript").run(&[ExecutorFile::new(
            "main.js",
            r#"
console.log("Hello, World!");
            "#,
        )]);
        assert_eq!(result.status, "OK");
        assert_eq!(result.exit_code, Some(0));
        assert_eq!(result.stdout, "Hello, World!\n");
    }
}
