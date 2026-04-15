> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# OR

Performs a bitwise logical OR operation on the source and destination operands. The result is stored in the destination operand.

The following table describes the supported source and destination operands:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| imm | reg |
| reg | mN |
| imm | mN |

Support LOCK

The instruction is supported in 64-bit mode, 32-bit mode, and 16-bit mode. When operating on memory, the `LOCK` prefix SHALL be used to ensure atomicity across multiple processors.

The `OR` instruction affects the EFLAGS register: CF and OF are always cleared; SF, ZF, and PF are updated based on the result of the operation.

To avoid unintended behavior, the user MUST ensure that the immediate operand size matches the operand size of the destination. In 64-bit mode, if a 32-bit immediate is used with a 64-bit register, the immediate is sign-extended to 64 bits before the operation.