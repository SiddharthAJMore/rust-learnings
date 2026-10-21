### Phase 1: Core Mental Model & Borrow Checker (Weeks 1–3)


**1.Week 1: Fundamentals & Setup:**Target: Install Cargo & Read Chapters 1–3 of The Rust Book.

- **Topics:** Cargo CLI toolchain, immutable vs. mutable variables (`let` vs `let mut`), primitive types, functions, and control flow.
- **Bridge from Go/Java:** Think of `cargo` as `go build` + `npm` + `maven` all in one.

- **🎬 Watch:** [Let's Get Rusty - Rust Programming Language Playlist (Ch 1–3)](https://www.google.com/search?q=https://www.youtube.com/playlist%3Flist%3DPLqbS7AVVErFiWDOavrPh7AMiS5yct9C7B) (Clear, bite-sized walkthroughs of the basics).
- **Hands-on Exercises:** Solve _Variables_, _Functions_, and _Primitive Types_ in **Rustlings** or **practice.rs**.
============================       DONE      ============================

**2.Week 2: Ownership, Borrowing, & Lifetimes: Target: Read Chapter 4 & 10.3 of The Rust Book.

- **Topics:** Heap vs. Stack allocation, Move semantics, Borrowing (`&T` vs `&mut T`), and explicit Lifetimes (`'a`).
- **Bridge from Java/Go:** In Java, everything is a reference tracked by the JVM. In Rust, _only one variable owns data at a time_.
    
- **🎬 Watch:** [No Boilerplate - Rust for Beginners](https://www.youtube.com/watch?v=br3qAlDuVCE) (High-energy breakdown of _why_ ownership matters) and [Let's Get Rusty - Understanding Ownership in Rust](https://www.youtube.com/watch?v=VFIOSWyqXBo).
    
      
    
- **Hands-on Exercises:** Complete _Ownership_ and _Borrowing_ on **Rustlings**.
    
      
    

**3.Week 3: Structs, Enums, & Pattern Matching:**Target: Read Chapters 5, 6 & 18 of The Rust Book.

- **Topics:** Structs (tuple/named), Algebraic Enums (enums holding data), `match` expressions, `if let`.
    
      
    
- **Bridge from Go:** Enums with data replace Go's empty interface `interface{}` pattern with type-safe pattern matching.
    
      
    
- **🎬 Watch:** [Let's Get Rusty - Enums and Pattern Matching in Rust](https://www.youtube.com/watch?v=0tWl-9G-8pQ).
    
      
    
- **Mini Project:** Build a command-line **In-Memory Key-Value Store** supporting `Get`, `Set`, and `Delete` operations using enums for commands.
    
      
    

### Phase 2: Idiomatic Rust & Code Organization (Weeks 4–5)

**Goal:** Learn how Rust organizes code, handles errors, and enforces abstraction without traditional OOP inheritance.

  

**1.Week 4: Error Handling & Modules:**Target: Read Chapters 7 & 9 of The Rust Book.

- **Topics:** Modules (`mod`), crates, `Result<T, E>` and `Option<T>`, the `?` error propagation operator, custom error types using `thiserror` crate.
    
      
    
- **Bridge from Go/Java:** No `try-catch` exceptions like Java, and cleaner error handling than Go's `if err != nil` boilerplate using `?`.
    
      
    
- **🎬 Watch:** [Let's Get Rusty - Managing Growing Projects with Packages, Crates, and Modules](https://www.youtube.com/watch?v=7z2LIGfJ-E8) & _Error Handling in Rust_ video.
    
      
    
- **Hands-on Exercises:** Complete _Error Handling_ and _Modules_ tracks on **Exercism.org**.
    
**2.Week 5: Traits, Generics, & Collections:**Target: Read Chapters 8 & 10 of The Rust Book.

- **Topics:** `Vec<T>`, `HashMap<K, V>`, Generics, Traits (similar to Java interfaces / Go interfaces), Trait Bounds, `derive` macros (`#[derive(Debug, Clone)]`).
    
- **🎬 Watch:** [Let's Get Rusty - Generic Types, Traits, and Lifetimes](https://www.youtube.com/watch?v=zT2g3Uv2K50) to solidify how Rust handles object polymorphism via traits instead of classes.
    
- **Mini Project:** Build a **CSV / JSON Parser CLI** using `clap` (for argument parsing) and `serde` (for serialization/deserialization).
    
### Phase 3: Systems & Asynchronous Programming (Weeks 6–8)

**Goal:** Master concurrency, multithreading, and async web programming.

**1.Week 6: Smart Pointers, Closures, & Iterators:**Target: Read Chapters 13 & 15 of The Rust Book.

- **Topics:** `Box<T>`, `Rc<T>`, `Arc<T>`, `Mutex<T>`, `RefCell<T>`. Iterators, functional methods (`map`, `filter`, `fold`), closures.
    
- **Bridge from Java:** Closures and iterators feel very similar to Java Streams or Python list comprehensions.
    
- **🎬 Watch:** [Let's Get Rusty - Smart Pointers Playlist](https://www.youtube.com/watch?v=W0S9aocg_SU) (`Box`, `Rc`, `Arc`, `Mutex` explained visually).
    
**2.Week 7: Concurrency & Async Rust:**Target: Read Chapters 16 & Tokio Docs.

- **Topics:** Threads (`std::thread`), Message passing channels (`mpsc`), `Send` and `Sync` traits. Async programming with **Tokio runtime** (`async`/`await`).
    
- **Bridge from Go:** Go channels map directly to Rust's `mpsc` channels, but Rust checks data race safety at compile time!
    
- **🎬 Watch:** [No Boilerplate - Fearless Concurrency](https://www.youtube.com/watch?v=48S4yEjOMJA) and [Let's Get Rusty - Fearless Concurrency in Rust](https://www.youtube.com/watch?v=O15-AecC68o).
    
**3.Week 8: Capstone Portfolio Project:**Target: Build a complete production-grade application.

- **Project Option A (Backend):** Build an asynchronous **REST API** using **Axum** + **SQLx** (PostgreSQL) + **Tokio** with JWT Authentication.  
    
- **Project Option B (CLI / DevOps Tool):** Build a high-performance **Concurrent Log Parser / System Monitor** with multi-threading and live terminal outputs.
    
- **🎬 Watch (Advanced Masterclass):** [Jon Gjengset's Crust of Rust Series](https://www.google.com/search?q=https://www.youtube.com/%40JonGjengset) (Pick a video like _Async/Await_, _Lifetimes_, or _Smart Pointers_ for deep systems-level comprehension).
    
### Summary of YouTube Channels Featured

- **[Let's Get Rusty](https://www.google.com/search?q=https://www.youtube.com/%40LetsGetRusty):** Best chapter-by-chapter visual companion to _The Rust Book_ (Weeks 1–6).
       
- **[No Boilerplate](https://www.youtube.com/@NoBoilerplate):** Best for motivational, high-level architecture insights and conceptual clarity (Weeks 2 & 7).
    
- **[Jon Gjengset](https://www.google.com/search?q=https://www.youtube.com/%40JonGjengset):** Best for advanced deep dives once you finish the core path (Week 8 & beyond).