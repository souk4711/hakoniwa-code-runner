#[cfg(test)]
mod executor_test {
    use hakoniwa_code_runner::{App, AppConfig, ExecutorFile};
    use lazy_static::lazy_static;
    use rust_embed::RustEmbed;
    use std::str;

    #[derive(RustEmbed)]
    #[folder = "./.devcontainer"]
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

    fn test_run_hello_world_example(lang: &str, files: &[ExecutorFile]) {
        let executor = APP.get_executor(lang).unwrap();
        let result = executor.run(files);
        match result.exit_code {
            Some(0) => {
                assert_eq!(result.stdout, "Hello, World!\n");
            }
            v => {
                eprintln!("  CODE: {:?}: {}", v, result.reason);
                eprint!("STDOUT:\n{}", result.stdout);
                eprint!("STDERR:\n{}", result.stderr);
                assert_eq!(v, Some(0));
            }
        }
    }

    #[test]
    fn test_run_hello_world_example_lang_c() {
        test_run_hello_world_example(
            "c",
            &[ExecutorFile::new(
                "main.c",
                r#"
#include <stdio.h>

int main() {
    printf("Hello, World!\n");
    return 0;
}
            "#,
            )],
        );
    }

    #[test]
    fn test_run_hello_world_example_lang_cpp() {
        test_run_hello_world_example(
            "cpp",
            &[ExecutorFile::new(
                "main.cpp",
                r#"
#include <iostream>

int main() {
    std::cout << "Hello, World!\n";
    return 0;
}
            "#,
            )],
        );
    }

    #[test]
    fn test_run_hello_world_example_lang_d() {
        test_run_hello_world_example(
            "d",
            &[ExecutorFile::new(
                "main.d",
                r#"
import std.stdio;

void main() {
    writeln("Hello, World!");
}
            "#,
            )],
        );
    }

    #[test]
    fn test_run_hello_world_example_lang_erlang() {
        test_run_hello_world_example(
            "erlang",
            &[ExecutorFile::new(
                "main.erl",
                r#"
-module(main).
-export([start/0]).

start() ->
    io:fwrite("Hello, World!\n").
            "#,
            )],
        );
    }

    #[test]
    fn test_run_hello_world_example_lang_go() {
        test_run_hello_world_example(
            "go",
            &[ExecutorFile::new(
                "main.go",
                r#"
package main

import "fmt"

func main() {
    fmt.Println("Hello, World!")
}
            "#,
            )],
        );
    }

    #[test]
    fn test_run_hello_world_example_lang_haskell() {
        test_run_hello_world_example(
            "haskell",
            &[ExecutorFile::new(
                "main.hs",
                r#"
main :: IO ()
main = putStrLn "Hello, World!"
            "#,
            )],
        );
    }

    #[test]
    fn test_run_hello_world_example_lang_java() {
        test_run_hello_world_example(
            "java",
            &[ExecutorFile::new(
                "main.java",
                r#"
class Main {
    public static void main(String[] args) {
        System.out.println("Hello, World!");
    }
}
            "#,
            )],
        );
    }

    #[test]
    fn test_run_hello_world_example_lang_javascript() {
        test_run_hello_world_example(
            "javascript",
            &[ExecutorFile::new(
                "main.js",
                r#"
console.log("Hello, World!");
            "#,
            )],
        );
    }

    #[test]
    fn test_run_hello_world_example_lang_python() {
        test_run_hello_world_example(
            "python",
            &[ExecutorFile::new(
                "main.py",
                r#"
print('Hello, World!')
            "#,
            )],
        );
    }

    #[test]
    fn test_run_hello_world_example_lang_ruby() {
        test_run_hello_world_example(
            "ruby",
            &[ExecutorFile::new(
                "main.rb",
                r#"
puts 'Hello, World!'
            "#,
            )],
        );
    }

    #[test]
    fn test_run_hello_world_example_lang_rust() {
        test_run_hello_world_example(
            "rust",
            &[ExecutorFile::new(
                "main.rs",
                r#"
fn main() {
    println!("Hello, World!");
}
            "#,
            )],
        );
    }

    #[test]
    fn test_run_hello_world_example_lang_scala() {
        test_run_hello_world_example(
            "scala",
            &[ExecutorFile::new(
                "main.scala",
                r#"
object Main {
    def main(args: Array[String]) = {
        println("Hello, World!")
    }
}
            "#,
            )],
        );
    }

    #[test]
    fn test_run_hello_world_example_lang_typescript() {
        test_run_hello_world_example(
            "typescript",
            &[ExecutorFile::new(
                "main.ts",
                r#"
let message: string = 'Hello, World!';
console.log(message);
            "#,
            )],
        );
    }
}
