- literal
- A reference can NEVER outlive the owner: Your lifetime 'a enforces this rule at compile time.
- usize
- Ownership = who holds the key
- Borrowing = temporary access card
- Lifetime = expiration date on the card
- If something can move, it cannot contain references to itself
- Library code -> generics -> compile time polymorphism
- Application code -> trait objects -> run time polymorphism
- Low-level code → std::io::Error
- Mid-level code → parsing / validation
- High-level code → app-specific meaning
- If it compiles, it cannot data-race.
- Ordering in atomic operation
    - SeqCst → safest, slowest
    - Relaxed → fastest, but tricky
    - Use SeqCst unless you fully understand memory ordering
- Mutex → “One at a time”
- RwLock → “Many readers or one writer”
- Atomic → “CPU guarantees this operation is indivisible”
- Rust is safe because it forbids certain patterns.
- Data races are UB, not “wrong output”.
- unsafe: Compiler, step aside — I take responsibility
- unsafe: I know more than the compiler — and I accept the cost.
    - mutex, refCell, atomic, arc build using unsafecell and unsafe
- Rust doesn’t prevent bugs, It prevents classes of bugs.
- Relaxed Ordering:
    - Atomicity, No torn writes, No ordering guarantees
    - Reorder loads/stores
    - Delay visibility to other threads
    - Use case:
        - Statistics
        - Counters
        - Metrics
        - Non-synchronizing state
- Release / Acquire Ordering = Visibility Contract
    - Think of Release and Acquire as a handshake between threads.
    - Everything I did before this line (Release) must be visible to anyone who acquires this atomic.
        ```rust
            // It is a publish point.
            DATA.store(42, Ordering::Relaxed);
            READY.store(true, Ordering::Release);
        ```
    - After I see this value, I promise to see everything the writer released.
        ```rust
            // It is a consume point.
            while !READY.load(Ordering::Acquire) {}
            println!("DATA = {}", DATA.load(Ordering::Relaxed));
        ```
        ```
        Writer thread                   Reader thread
        --------------                 --------------
        DATA = 42         \
        READY.store(Release)  ----->   READY.load(Acquire)
                                       DATA.load()  ✅ must see 42
        ```
