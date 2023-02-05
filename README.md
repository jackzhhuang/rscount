# rscount command
Count the valid lines in Rust codes. 

## The rule of counting

It will skip the blank line and comments. For example:

```rust
// this line will be ignored since it is a comment.
/*
	this comment is also ignored since it is a block of a comment.
*/

/*
 another block of comment.
*/	// this is a comment line too.

/*

*/ fn main() { // but this is a valid line
  println!("hello world!"); // a valid line again
}
```

the total of valid Rust code lines above is 3.



## Usages

### Count the lines of a Rust source file

It is able to count the valid line in a Rust source file.

Type the command like this:

```rust
rscount file="./test_dir/test.rs"
```

It will only count the lines of the file we input. This example show that rscount will count the valid lines of ./test_dir/test.rs and report to the user.



### Count the lines of Rust source files in a specific path

It is able to search a path that contains many sub-directories having number of Rust source files and count  valid lines of all Rust source files. 

Type the command like this:

```rust
rscount path="./test_dir/"
```

It will find the Rust source files recursively in test_dir and sum up the valid lines and report it to the user.



### Count the lines using multiple thread

If there are lots of Rust code files in the path, you can add *thread* in the parameters like this:

```rust
rscount path="./test_dir/" thread
```

It will count the lines using multiple threads and the number of the threads will be estimated by *std::thread::available_parallelism*.



**Possibly, there are some other interesting and simple things we can do in the future**

