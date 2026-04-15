> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KORD

The `KORD` instruction performs a signed integer division of a 64-bit dividend by a 64-bit divisor, storing the quotient in the destination operand.

The following table covers the supported source and destination operands:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| memory | reg |
| reg | memory |

DO NOT support LOCK

This instruction is ONLY available in 64-bit mode. It is NOT supported in compatibility mode.

The destination register MUST NOT be the same as the source register used for the divisor to avoid undefined behavior during the operation. If the divisor is 0, the instruction SHALL trigger a #Z exception. If the quotient exceeds the capacity of r64, the instruction SHALL trigger a #O exception.