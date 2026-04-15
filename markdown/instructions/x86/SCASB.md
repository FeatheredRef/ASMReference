> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SCASB

Compares the value in the `al` register with the byte at the memory location pointed to by `rdi`. After the comparison, the `rdi` register is incremented or decremented by 1, depending on the Direction Flag (DF).

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg (`al`) | m1 ([rdi]) |

DO NOT support LOCK

The instruction is available in 64-bit mode, 32-bit mode, and 16-bit mode. It operates specifically on 8-bit operands regardless of the current processor mode.

The behavior of the pointer `rdi` depends on the state of the DF flag: if DF=0, `rdi` is incremented; if DF=1, `rdi` is decremented. Users MUST ensure the DF flag is set correctly via `CLD` or `STD` to avoid unintended memory access patterns. When used with the `REP` prefix, the instruction will continue until `al` equals the value at `[rdi]` or `rcx` reaches 0. Failure to initialize `rcx` correctly when using `REP` MAY result in a memory access violation (General Protection Fault) as the pointer increments through the address space.