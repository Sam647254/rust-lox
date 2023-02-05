pub struct Scanner<'a> {
   current: usize,
   start: usize,
   line: usize,
   source: &'a str,
}

impl Scanner<'_> {
   pub fn new(source: &str) -> Scanner {
      Scanner {
         current: 0,
         start: 0,
         line: 1,
         source
      }
   }
}