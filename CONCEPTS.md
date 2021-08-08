
## Type System and Memory Safety
- The unique type system of Rust guarantees the absence of uninitialized variables, dangling pointers, race conditions, etc.

## Absence of Automated Garbage Collection
- Most Rust objects have statistically known lifetime and rest are managed by Smart Pointers

## Absence of Null Pointers
- Data Structures have to be acyclic in Rust

## Memory Safety through Exclusive Entities
- Rust ensures that no memory objects are both mutable and aliased at the same time.
