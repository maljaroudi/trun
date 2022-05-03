pub mod funcs;
use std::alloc::System;

#[global_allocator]
static A: System = System;
#[cfg(test)]
mod tests {

    use super::funcs::interpret;
    #[test]
    fn it_works() {
        let mut file = r#"
         [prompt]
          module = "Prompt"
          name = "Say Hello"
          command = "echo hello"
          strict = true
          message = "Are you Sure you want to print hello?"
          answer = ["Yes","No"]
            "#
        .as_bytes();
        assert_eq!(interpret(&mut file), Ok(()));
    }
}
