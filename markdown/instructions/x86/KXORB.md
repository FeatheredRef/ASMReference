> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KXORB

Performs a bitwise logical XOR operation between a source operand and a destination operand, storing the result in the destination.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| reg | m1 |
| imm | reg |
| imm | m1 |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It is NOT supported in compatibility mode.

When using a memory destination, the operation is performed on a 1-byte alignment. Failure to align memory accesses may result in performance degradation or alignment check exceptions if Alignment Check (AC) flag is set in CR0.