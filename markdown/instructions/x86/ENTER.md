> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ENTER

Pushes the current base pointer (RBP) onto the stack, sets the new base pointer to the current stack pointer (RSP), and decrements the stack pointer by the specified size to allocate space for local variables. If the immediate value is non-zero, it also pushes the immediate value onto the stack (forming a nested frame) before setting the base pointer.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | r64, m64 |
| #I | #I |

DO NOT support LOCK

The ENTER instruction is available in 64-bit mode but operates primarily to support the legacy prologue of 32-bit functions. In x86-64, it is rarely used as compilers typically generate a sequence of `PUSH RBP` and `SUB RSP, imm` for better performance.

The instruction's behavior depends on the size of the immediate operand. If the immediate is 0, the instruction effectively performs a `PUSH RBP` followed by `MOV RBP, RSP`. If the immediate is non-zero, it allocates the specified number of bytes for the local frame. Users SHALL ensure the stack is properly aligned to 16 bytes before calling functions to avoid performance penalties or crashes in SIMD operations, as ENTER does not automatically handle alignment.