# Rust Out-of-Bounds Vector Access

This repository demonstrates a common error in Rust: accessing an element in a vector using an index that is out of bounds.  This will result in a runtime panic.

The `bug.rs` file contains the buggy code. The `bugSolution.rs` file provides a corrected version that prevents the panic.

## Bug Description

The buggy code attempts to access the third element of a vector containing only two elements.  This leads to a panic due to an index out of bounds error.

## Solution

The solution involves checking the bounds of the vector before attempting to access an element.