

# Rust-Backed Linked List with C++ Wrapper

This project implements a **linked list** in Rust for memory safety and performance while providing a **C++ wrapper** for seamless integration with existing C++ codebases. The Rust implementation manages all the memory and data structures internally, while the C++ interface provides a clean and intuitive way to interact with the linked list.

---

## Features

- **Rust Implementation**: A linked list implemented in Rust, leveraging its memory safety and concurrency guarantees.
- **C++ Wrapper**: A wrapper interface for C++ that hides Rust's internals and uses only IDs and integer values for operations.
- **Thread-Safe Design**: Multiple linked lists can be managed concurrently with unique IDs.
- **Supported Operations**:
  - Add a node at the beginning or end.
  - Remove a node from the beginning or end.
  - Print the linked list.

---

## Prerequisites

### Rust
- Install Rust: [Rust Installation Guide](https://www.rust-lang.org/tools/install)

### C++ Compiler
- A C++ compiler that supports C++11 or later.
- GCC, Clang, or MSVC.

---

## Build Instructions

### 1. Clone the Repository


---

### 2. Build the Rust Library

1. Navigate to the Rust code directory:
   

2. Add the following to `Cargo.toml` (if not already included):
   ```toml
   [dependencies]
   lazy_static = "1.4"
   ```

3. Build the Rust library:
   ```bash
   cargo build --release
   ```

4. Locate the compiled shared library:
   - On **Linux**: `target/release/liblinked_list.so`
   - On **macOS**: `target/release/liblinked_list.dylib`
   - On **Windows**: `target/release/linked_list.dll`

---

### 3. Build the C++ Wrapper

1. Navigate to the C++ code directory:
   

2. Compile the C++ code and link it with the Rust library:
   ```bash
   g++ main.cpp -o main -Ltarget/release -llinked_list -ldl
   ```

   - Replace `../rust/target/release` with the path to the Rust shared library if it's in a different location.

---

### 4. Run the Program

Run the compiled executable:
```bash
./main
```

---

## Example Usage

### C++ Interface

Here’s an example of how you can use the linked list through the C++ wrapper:

```cpp
#include <iostream>
#include "LinkedListWrapper.h"

int main() {
    LinkedListWrapper list(1); // Initialize a linked list with ID = 1

    list.addAtEnd(10);
    list.addAtEnd(20);
    list.addAtBeginning(5);

    std::cout << "Initial List: ";
    list.printList();

    list.removeFromEnd();
    std::cout << "After Removing Last Element: ";
    list.printList();

    list.removeFromBeginning();
    std::cout << "After Removing First Element: ";
    list.printList();

    return 0;
}
```

### Output

```text
Initial List: 5 -> 10 -> 20 -> None
After Removing Last Element: 5 -> 10 -> None
After Removing First Element: 10 -> None
```

---


## How It Works

1. **Rust Implementation**:
   - The linked list is implemented in Rust for safe memory management.
   - All linked list instances are managed in a global, thread-safe registry using `lazy_static` and `Mutex`.

2. **C++ Wrapper**:
   - Provides a simple API for creating and manipulating linked lists using unique integer IDs and integer values.
   - Hides all pointers and unsafe code from the C++ side.

3. **Thread-Safe Design**:
   - The `LINKED_LISTS` global registry ensures safe concurrent access to multiple linked list instances.

---

## Advantages

- **Safety**: Rust ensures no memory leaks or undefined behavior in the linked list operations.
- **Performance**: Rust’s zero-cost abstractions provide excellent runtime performance.
- **Integration**: The C++ wrapper simplifies interaction with Rust, allowing easy integration into existing C++ projects.

---

## Future Enhancements

- Add support for additional operations:
  - Searching for a value.
  - Reversing the linked list.
  - Finding the length of the list.
- Improve error handling in Rust to provide meaningful feedback to the C++ wrapper.
- Add automated testing for both Rust and C++.

---

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

