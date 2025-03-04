/*
  ok, so:

  heap and stack exist.

  stack is basically a whole bunch of data of known size stacked on top of each other.
  you can access any part of it, but you can only add and remove stuff on top of it.

  heap is a whole bunch of memory, and you can store data of unknown size on it.
  you will need to store in a var what are the coords on any bit of data in the heap.

  Stack is more efficient.
  Heap is geared towards data of unknown size
*/
/*
fn main() {
  let s = String::from("hello");

  // Now this does something pretty unexpected:
  // d copies the data from s.
  // that makes sense.
  // but now, if you change the data in d or s, the other changes as well.
  // that happens because the String data type actually only has a pointer reffering to the part of the heap inside of it.
  // that means that if I were to change the data in d, I would edit the memory in the heap,
  // which is the same one that s accesses.
  //
  // This is done because copying a lot of data from one place on the heap to the other could be quite expencive.
  let d = s;

  // but that doesn't matter because of:
}
*/

/*
fn main() {
  // now this is weirder.
  // after the creation of s2, s1 is considered 
  // to have moved, and thus goes out of scope

  let s1 = String::from("hello");
  let s2 = s1;

  println!("{}, world!", s1); //  will throw an error
}
*/

fn main(){
  // so if we do need to create a copy of a var's data, we use
  let s1 = String::from("hello");
  let s2 = s1.clone();

  println!("s1 = {}, s2 = {}", s1, s2);
}