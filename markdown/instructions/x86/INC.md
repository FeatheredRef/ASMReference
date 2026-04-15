> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# INC

Increments the destination operand by 1.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| #I | reg |
| #I | mN |

Support LOCK

The instruction is available in 64-bit mode, 32-bit mode, and 16-bit mode.

The `INC` instruction does not affect the Carry Flag (CF). This behavior differs from `ADD rN, 1`, and can lead to logic errors if the instruction is used in a loop counter where a carry-out is expected to trigger a condition.

When using `INC` with a memory operand in a multi-processor environment, the `LOCK` prefix SHALL be used to ensure atomicity, as the standard read-modify-write cycle is not inherently atomic.