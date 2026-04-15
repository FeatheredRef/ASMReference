> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ARPL

Adjusts the Requested Privilege Level (RPL) of a selector. The instruction checks if the RPL of the source selector is numerically greater than or equal to the RPL specified in the immediate operand; if so, it sets the RPL of the destination selector to the value of the immediate operand. Otherwise, it sets the destination selector to all ones.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| reg | m2 |
| m2 | reg |
| m2 | m2 |

DO NOT support LOCK

ARPL is only available in compatibility mode when executing in a 64-bit environment. In 64-bit mode, this instruction is not supported and will trigger an invalid opcode exception.

To avoid unexpected behavior, ensure the immediate operand is a u2 value between 0 and 3. If the source selector is not a valid segment selector or the RPL check fails, the destination will be loaded with 0xFFFF, which is an invalid selector for any segment.