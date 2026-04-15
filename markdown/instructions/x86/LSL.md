> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# LSL

Loads the segment limit of a segment register into the ZF flag. If the specified offset is greater than the segment limit, ZF is cleared (0); otherwise, ZF is set (1).

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | ZF flag |
| imm | ZF flag |
| mN | #I |

DO NOT support LOCK

LSL is only available in compatibility mode when executing a 16-bit or 32-bit operand size. It is NOT supported in 64-bit mode.

The offset used for the check SHALL be treated as an unsigned integer. Ensure that the operand size matches the expected segment limit scale to avoid incorrect ZF results.