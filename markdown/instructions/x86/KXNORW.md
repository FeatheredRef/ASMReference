> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KXNORW

Performs a bitwise logical NOR operation between the source and destination operands, then applies a bitwise NOT to the result (effectively producing an XNOR operation) on the low 16 bits of the operands.

The following table covers the supported source and destinations:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| mem | reg |
| imm | reg |
| reg | mem |
| imm | mem |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode. It is NOT supported in compatibility mode.

When using a memory operand as a destination, the operation is performed on a word-sized region (m2). Ensure that the memory address is properly aligned to avoid performance penalties or exceptions depending on the alignment check settings.