#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
  let scale = 2;
  let rect1 = Rectangle {
      width: dbg!(30 * scale),
      height: 50,
  };
  
  /**
   * println!
   * - takes a reference
   * - prints to the standard output console stream (stdout).
   *
   * dbg!
   * - takes ownership
   * - prints to the standard error console stream (stderr).
   * - prints the file and line number too.
   */
  dbg!(&rect1); 
  /**
   * [src/main.rs:10:16] 30 * scale = 60
   * [src/main.rs:14:5] &rect1 = Rectangle {
   *     width: 60,
   *     height: 50,
   * }
   */
}
