# intro2rust

#### Objective : 
To learn the Rust programming language by making basic programs and document important points/learnings.

---

Why Rust?


- Faster and Reliable software. __It just works!__
- Brings high-level ergonomics and low-level control together. __Feels like Python, executes like C++!__
    - It gives you low-level control like Memory usage, but removes all the hassles associated with that. 
    - Rust does not handle the memory usage for you, but handles the boilerplate code for you.
- 

Summary : Rust is a low-level statically-typed multi-paradigm programming language that’s focused on safety and performance.

Why is it better than other languages? 
It has three main benefits:
- Better memory safety due to the compiler;
    - Rust’s ownership system analyses the program’s memory management at compile-time, making sure that bugs due to poor memory management can’t happen and that garbage collection is unnecessary.

- Easier concurrency due to the data ownership model that prevents data races
    - Due to the borrow checker, Rust can prevent data races at compile-time.
    Data races occur when two threads access the same memory at the same time, and they can lead to some nasty, unpredictable behavior. Thankfully, preventing undefined behavior is all what Rust is about.

- Zero-cost abstractions.
    - Zero-cost abstractions make sure that there is virtually no runtime overhead for the abstractions that you use. In simpler words: there is no speed difference between low-level code and one written with abstractions. __Take Notes! JavaScript and GoLang!__

