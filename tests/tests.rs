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

dies!(_01, 11, 1320851, 31, 26859182);
dies!(_02, 2, 306, 4, 366);

// XXX 161 is wrong for part 2, since example input should be different,
//     but this macro doesn't account for it,
//     so we handle it within the dies_03 module's inner test suite .
dies!(_03, 161, 188116424, 161, 104245808);
dies!(_04, 18, 2517, 9, 1960);
dies!(_05, 143, 4662, 123, 5900);
dies!(_06, 41, 5080, 6, 1919);
dies!(_07, 3749, 20281182715321, 11387, 159490400628354);
