> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SCAS

Scans a value in the `al`, `ax`, `eax`, or `rax` register against a value in the memory location pointed to by `rdi`. The instruction compares the two values and updates the CPU flags. After the comparison, the `rdi` register is incremented or decremented by the size of the operand, depending on the Direction Flag (DF).

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg (`al`/`ax`/`eax`/`rax`) | m1/m2/m4/m8 (via `rdi`) |

DO NOT support LOCK

The instruction is available in 32-bit and 64-bit modes. In 64-bit mode, if a 32-bit operand size is used, `rdi` is still updated based on the 32-bit size, but the upper 32 bits of the register are preserved.

The `rdi` register MUST be correctly initialized to the starting address of the memory array before execution. If the Direction Flag (DF) is set to 1, `rdi` will be decremented; if DF is 0, `rdi` will be incremented. Failure to clear or set DF according to the intended traversal direction SHALL result in an out-of-bounds memory access.

When used with the `REP` prefix, the instruction continues until the Zero Flag (ZF) is cleared or the `rcx` register reaches zero. If `rcx` is initially zero, the instruction SHALL NOT execute the comparison and the loop will terminate immediately.