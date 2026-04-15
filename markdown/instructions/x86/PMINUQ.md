> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PMINUQ

Computes the minimum of two unsigned 64-bit integers and stores the result in the destination operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| m8 | #I |
| m16 | #I |
| m32 | #I |
| m64 | m64 |
| reg | reg |
| reg | m64 |
| imm | #I |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It is NOT supported in compatibility mode.

The destination operand SHALL NOT be the same as the source operand for memory-to-memory operations, as the instruction requires a register or memory source and a register or memory destination according to the XMM/SSE specification; however, standard `PMINUQ` operates on XMM registers. If utilizing the `PMINUQ` mnemonic for general-purpose registers (via extensions), ensure the operands are u64.

The instruction operates on 128-bit XMM registers, processing two u64 elements in parallel (SIMD). Failure to align memory operands to 16-byte boundaries MAY result in performance degradation or general protection faults depending on the alignment check flags.