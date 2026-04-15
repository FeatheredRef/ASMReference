> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CLAC

Sets the AC flag (Auxiliary Carry flag) in the RFLAGS register to 1.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | RFLAGS (AC bit) |

DO NOT support LOCK

The instruction is available in 16-bit, 32-bit, and 64-bit operand size modes. It does not require a specific compatibility mode to function.

CLAC does not affect any flags other than the AC flag. Since the AC flag is primarily used by BCD (Binary Coded Decimal) arithmetic instructions (such as AAA or DAA), CLAC is typically used to prepare the processor state for such operations or to clear side effects from previous calculations.