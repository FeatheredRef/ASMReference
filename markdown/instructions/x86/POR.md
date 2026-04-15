> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# POR

Performs a bitwise OR operation between the source operand and the destination operand, storing the result in the destination operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| imm | reg |
| reg | mN |
| imm | mN |

Support LOCK

The instruction is supported in 64-bit mode and compatibility mode. In 64-bit mode, the operand size may be 8, 16, 32, or 64 bits.

The destination operand MUST NOT be the same as the source operand when utilizing a memory reference to avoid potential aliasing issues, although the ISA allows it. When the destination is a memory location, the operation is performed on the memory content and the result is written back to that same location.