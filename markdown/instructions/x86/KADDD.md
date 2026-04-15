> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KADDD

Adds two double-precision floating-point values and stores the result in a destination operand.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| r64 | r64 |
| m8 | r64 |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode. It requires the AVX instruction set extension to be enabled.

The destination register MUST be an XMM register. Using this instruction on registers not supported by the AVX extension will result in an invalid opcode exception. Ensure the MXCSR register is correctly configured to handle rounding and exception flags to avoid unexpected #P or #O results.