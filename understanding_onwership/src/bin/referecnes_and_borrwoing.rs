fn main() {
    fn main() {
        let s1 = String::from("hello");

        let (s2, len) = calculate_length(s1);

        println!("The length of '{s2}' is {len}.");
    }

    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len(); // len() returns the length of a String

        (s, length)
    }

    /**
     * References and Borrowing
     */

    // First, notice that all the tuple code in the variable declaration and the function return value is gone.
    // Second, we take &String rather than String. These "ampersands" represent "references".
    // they allow you to refer to some value without taking ownership of it.

    // The &s1 syntax lets us create a reference that refers to the value of s1. &s -> s1 -> "hello"
    // we won’t need to return the values in order to give back ownership, because we never had ownership.
    fn main() {
        let s1 = String::from("hello");

        let len = calculate_length(&s1);

        println!("The length of '{s1}' is {len}.");
    }

    fn calculate_length(s: &String) -> usize {
        // s is a reference to a String
        s.len()

        s.push_str(", world"); // Error: cannot borrow `*s` as mutable, as it is behind a `&` reference
    } // Here, s goes out of scope. But because it does not have ownership of what it refers to, it is not dropped.

    /**
     * Mutable References
     */

    // Then we create a mutable reference with &mut s.
    // This makes it very clear that the called function will mutate the value it borrows.

    fn main() {
      let mut s = String::from("hello");
      
      change(&mut s);
    }
    
    fn change(some_string: &mut String) {
      some_string.push_str(", world");
      (*some_string).push_str(", world"); // Also works
    }

    // Restriction: you can only have one mutable reference to a particular piece of data in a particular scope.
    // new Rustaceans struggles. But Rust can prevent data races at compile time
    
    let mut s = String::from("hello");
    
    let r1 = &mut s;
    let r2 = &mut s;
    
    println!("{}, {}", r1, r2); // Error: cannot borrow `s` as mutable more than once at a time.
    
    // We also cannot have a mutable reference while we have an immutable one to the same value.
    // Users of an immutable reference don’t expect the value to suddenly change out from under them!
    
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM
    
    println!("{}, {}, and {}", r1, r2, r3);
    
    // reference’s scope : where it is introduced ~ the last time that reference is used.

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}"); // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");

    /**
     * Dangling References is Not Allowed
     */

    fn main() {
      let reference_to_nothing = dangle();
    }
  
    fn dangle() -> &String { // dangle returns a reference to a String

      let s = String::from("hello"); // s is a new String
  
      &s // we return a reference to the String, s
    } // Here, s goes out of scope, and is dropped. Its memory goes away.
      // Danger!


    // Move the ownership of the String to the calling function and return the String directly.

    fn no_dangle() -> String {
      let s = String::from("hello");
  
      s
    }
  }
