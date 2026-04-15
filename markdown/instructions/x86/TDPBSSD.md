> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# TDPBSSD

Subtracts a scalar double-precision floating-point value from a double-precision floating-point tensor and stores the result in a destination tensor.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| m8 | m8 |
| reg | m8 |
| m8 | reg |
| reg | reg |

DO NOT support LOCK

This instruction is ONLY available when the processor is operating in 64-bit mode. It is NOT supported in compatibility mode.

The operation MUST follow the IEEE 754 standard for double-precision floating-point subtraction. If the destination memory operand is not aligned to the required boundary, a general-protection exception MAY occur depending on the alignment check (AC) flag in the EFLAGS register. Ensure that the memory operands are correctly aligned to avoid performance degradation or exceptions.