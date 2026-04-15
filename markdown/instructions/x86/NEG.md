> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# NEG

Negates the value of the destination operand by calculating the two's complement (subtracts the operand from zero).

The following table covers the supported source and destination operands:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| mN | reg |
| #I | mN |
| #I | imm |

DO NOT support LOCK

The instruction is available in 16-bit, 32-bit, and 64-bit operand sizes. It operates on `rN` or `mN` where N is 8, 16, 32, or 64.

The `CF` flag is set to 1 if the operand is not zero; if the operand is zero, `CF` is cleared to 0. The `SF` flag is set if the result is negative. The `OF` flag is set if the operand is the minimum representable signed integer for the given size (e.g., `0x80` for 8-bit), as the result cannot be represented in two's complement.

When using `NEG` on the minimum signed value (e.g., `-128` for `i8`), the result is the same as the input value, and the `OF` flag WILL be set. This is a critical edge case for arithmetic logic.