# Quickfall
Quickfall is a programming language focusing  on allowing everyone to build fast, reliable and safe software.

## Why Quickfall
Quickfall aims to give the performance of extremely fast languages like C while giving the safety of Rust and also being reliable.

What Quickfall focuses on:
- **Speed**: Quickfall focuses on runtime speed which means that every single compile time safety check can be simply disabled either globally or for a single element.
- **Strict syntax**: Quickfall enforces a strict syntax to avoid mistakes or error prone code
- **Safety**: By default, Quickfall variables are guaranteed to contain valid data and not cause any UB. Quickfall also allows for unsafe operations by using *unsafe* marked variables which can themselves be safely converted to normal variables
- **Thread safety**: Quickfall gives tools to enforce thread safety mostly on compile time such as the *thread markers* to identify and restrict variables to only certain threads
