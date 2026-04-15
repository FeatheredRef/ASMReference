> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FSTSW

Stores the current floating-point status word into the specified destination operand.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| ST(0) | word |

DO NOT support LOCK

This instruction is available in 64-bit mode, but it specifically accesses the x87 FPU status word. In x86-64, the x87 FPU is maintained for backward compatibility.

The destination operand MUST be a 16-bit register or memory location. Attempting to use a larger destination size will not affect the remaining bits of the destination register/memory beyond the first 2 bytes. The status word contains flags regarding floating-point exceptions (#I, #Z, #D, #O, #U, #P) and the current rounding control.