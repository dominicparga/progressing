#[test]
fn loops() {
    #[allow(dead_code)]
    mod example {
        include!("../../examples/loops.rs");

        pub fn test() {
            main();
        }
    }

    example::test();
}

#[test]
fn simple() {
    #[allow(dead_code)]
    mod example {
        include!("../../examples/simple.rs");

        pub fn test() {
            main();
        }
    }

    example::test();
}
