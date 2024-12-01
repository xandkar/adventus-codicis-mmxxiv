macro_rules! path {
    ($dies:expr, $type:expr) => {
        std::path::Path::new(concat!(
            "tests/input/dies",
            stringify!($dies),
            "/",
            $type,
            ".txt"
        ))
    };
}

macro_rules! path_to_example {
    ($dies:ident) => {
        path!($dies, "example")
    };
}

macro_rules! path_to_input {
    ($dies:ident) => {
        path!($dies, "input")
    };
}

macro_rules! dies {
  ($n:ident, $p1_ex:expr, $p1_in:expr, $p2_ex:expr, $p2_in:expr) => {
       paste::paste! {
           #[cfg(test)]
           mod [<dies $n>] {
               mod part_1 {
                   #[test]
                   fn example() {
                       let data = adventus_codicis_mmxxiv::[<dies $n>]::Data::load(path_to_example!($n)).unwrap();
                       assert_eq!($p1_ex, data.solve1().unwrap());
                   }

                   #[test]
                   fn input() {
                       let data = adventus_codicis_mmxxiv::[<dies $n>]::Data::load(path_to_input!($n)).unwrap();
                       assert_eq!($p1_in, data.solve1().unwrap());
                   }
               }
               mod part_2 {
                    #[test]
                    fn example() {
                        let data = adventus_codicis_mmxxiv::[<dies $n>]::Data::load(path_to_example!($n)).unwrap();
                        assert_eq!($p2_ex, data.solve2().unwrap());
                    }

                    #[test]
                    fn input() {
                        let data = adventus_codicis_mmxxiv::[<dies $n>]::Data::load(path_to_input!($n)).unwrap();
                        assert_eq!($p2_in, data.solve2().unwrap());
                    }
               }
           }
       }
  };
}
