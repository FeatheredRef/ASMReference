> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# BTR

Copies the value of a specified bit from a destination operand to the Carry Flag (CF) and then clears (sets to 0) that bit in the destination operand.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| reg | mN |
| imm | reg |
| imm | mN |

DO NOT support LOCK

The instruction is available in 64-bit mode and compatibility mode. The operand size is determined by the current address size or a prefix; it supports 8-bit, 16-bit, 32-bit, and 64-bit operations.

The bit index specified by the source operand MUST be less than the bit width of the destination operand. If the bit index is greater than or equal to the size of the destination (e.g., specifying bit 32 for a 32-bit register), the behavior is undefined or may result in no operation depending on the processor implementation; however, in standard x86-64 architectural behavior, it typically does not wrap and may not affect the destination. Users SHOULD ensure the bit index is within the valid range of the operand size to avoid unpredictable results.