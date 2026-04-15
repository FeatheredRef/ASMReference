> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KXORW

Performs a bitwise logical XOR operation on the lower 16 bits of the source and destination operands. The result is stored in the destination operand.

The following table specifies the supported source and destination operands:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| imm | reg |

DO NOT support LOCK

This instruction is only available in 64-bit mode. It is NOT supported in compatibility mode or 32-bit mode.

The destination register MUST be an xmm register (the lower 16 bits of the first element). Using general-purpose registers as operands will result in an invalid operation. Since this instruction operates on the lower 16 bits of a 32-bit lane in an xmm register, the upper bits of that lane are preserved. Failure to account for the existing values in the upper bits of the target lane may lead to unexpected results when performing subsequent wider operations.