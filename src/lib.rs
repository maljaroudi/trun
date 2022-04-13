pub mod funcs;
use std::alloc::System;

#[global_allocator]
static A: System = System;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
