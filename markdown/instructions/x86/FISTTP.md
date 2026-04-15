> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# FISTTP

Stores the floating-point value from ST(0) as a truncated 32-bit signed integer to the specified destination, then pops the floating-point stack.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| ST(0) | m32 |
| ST(0) | r32 |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It requires the use of the x87 FPU register set.

The destination MUST be aligned to the appropriate size boundary to avoid performance penalties or alignment faults depending on the processor configuration. If the result of the truncation exceeds the range of a 32-bit signed integer, a #O exception SHALL be generated. Any precision loss during truncation SHALL trigger a #P exception.