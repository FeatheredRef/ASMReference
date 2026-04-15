> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FNSTCW

Stores the current x87 floating-point control word into the specified destination.

The following table describes the supported source and destination.

| source | destination(s) |
| :--- | :--- |
| Internal x87 Control Word | m16 |
| Internal x87 Control Word | reg16 |

DO NOT support LOCK

This instruction is available in 64-bit mode, but it operates on the x87 FPU state. In x86-64, the x87 state is preserved across context switches, but the instruction is primarily used for compatibility with 32-bit x87 code.

The destination must be a 16-bit register or memory location. Attempting to store the control word into a destination larger than word size may result in undefined behavior or incorrect data alignment if not handled by the assembler. Since this instruction modifies memory or a general-purpose register, it does not trigger any floating-point exceptions (#I, #Z, #D, #O, #U, #P).