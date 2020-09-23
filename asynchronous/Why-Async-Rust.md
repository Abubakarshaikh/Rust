# Why Async Rust
* Async (or asynchronous) programming is a technique to run simultaneous operations in our applications.
* Whatever the nature of your application a web server, a database or an operating system, using async programing you can get most out of the underlying hardware
* Asynchronous Rust allows us to run multiple tasks concurrently on the same OS thread.
* In a typical threaded application, if you wanted to download two different webpages at the same time, you would spread the work across two different threads
### Example:
```
fn get_two_sites(){
   //spwand two threads to do work
   let threads_one = threda::spawn(|| download("https://www.foo.com"));
   let threads_two = threda::spawn(|| download("https://www.bar.com"));
   
   //wait for both two thread complete
   thread_one.join().expect("thread one panicked");
   thread_two.join().expect("thread two panicked");
}
```
* The code on above works fine for many applications
* Since threads were designed to run multiple different tasks at once
* However, they also come with some limitations
* There's a lot of overhead in switching between different threads and sharing data between them
* Even a thread which just sits and does nothing uses up valuable system resources
* These are the costs that asynchronous Rust is designed to eliminate
* We can rewrite the function above using Rust's async/.await notation
* Which will allow us to run multiple tasks at once without creating multiple threads
### Example:
```
async fn get_two_sites()
      // create two differents "futures" which, when to run completion will asynchronously the download the webpage
      let future_one = download_async("https://www.foo.com");
      let future_two = download_async("https://www.bar.com");
      
      // run both futures to completion at the same time
      join!(future_one,future_two);
}
```
* Overall, asynchronous applications have the potential to be much faster and use fewer resources than a corresponding threaded implementation
* However, there is a cost
* Threads are natively supported by the OS (operating system), and using them doesn't require any special programming model
* Any function can create a thread 
* A function that uses threads is usually just as easy as calling any normal function
* However, asynchronous functions require special support from the language or libraries
* In Rust, async fn creates an asynchronous function which returns a Future 
* To execute the body of the function, the returned Future must be run to completion
* It's important to remember that traditional threaded applications can be quite effective
* The increased complexity of the asynchronous programming model isn't always worth it
* it's important to consider whether your application would be better served by using a simpler threaded model

# The State of Asynchronous Rust
* Asynchronous Rust ecosystem has undergone a lot of evolution over time
* So it can be hard to know what tools to use, what libraries to invest in, or what documentation to read
* However, the Future trait inside the standard library and the async/await language feature has recently been stabilized.
* The ecosystem as a whole is therefore in the midst of migrating to the newly-stabilized API
* After which point churn will be significantly reduced
* At the moment, however, the ecosystem is still undergoing rapid development and the asynchronous Rust experience is unpolished
* Most libraries use the 0.1 definitions of the futures crate, meaning that to interoperate developers frequently need to reach for the compat functionality from the 0.3 futures crate
* The async/await language feature is still new
* Important extensions like async fn syntax in trait methods are still unimplemented
* Current compiler error messages can be difficult to parse
* In short, Rust is well on its way to having some of the most performant and ergonomic support for asynchronous programming

# Async/.await Primer
* Async/.await is Rust's built-in tool for writing asynchronous functions that look like synchronous code
* Async transforms a block of code into a state machine that implements a trait called Future
* Whereas calling a blocking function in a synchronous method would block the whole thread
* Blocked Futures will yield control of the thread, allowing other Futures to run
* To create an asynchronous function, you can use the ``async fn`` syntax:
```
async fn do_something(...){}
```
* The value returned by async fn is a Future
* Future needs to be run on an executor, so that a task may be done
* “block_on” blocks the current thread until the provided future has run to completion
* Other executors provide more complex behavior, like scheduling multiple futures onto the same thread
### Example:
```
use futures::executor::block_on;
async fn hello_world(){
      println!("Hello, world);
}
fn main(){
   let future = hello_world();
   block_on(future);
}
```
* We can also use .await instead of block_on inside async fn
* .await doesn’t block the whole thread but wait for the specific Future
* Allows the other tasks to run if the future unable to progress or busy
* 
