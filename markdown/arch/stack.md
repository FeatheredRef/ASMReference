# Stack

> Memory region used to manage execution context and short-lived data.

Stack is a per-thread OS-allocated memory region. Unlike heap, it commonly grows downwards (toward lower addresses), used to manage function execution context (saved registers, return addresses), and short-lived data. Its size is by default 8MiB in Linux, generally 1MiB or larger in other OS.

## Stack speed

Stack is generally fast. While it has same memory access speed as heap since they are both in RAM, it is much more efficiently cached in [L-Cache](</arch/l-cache>), and more likely on L1 or L2. Paired with cache locality, the chance of [MESI](</arch/mesi>) flagging stack's memory regions as invalid is very small, since stack is per-thread, and *mostly* used by a single one.
