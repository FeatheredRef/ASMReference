> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PSIGNB

Sign-extends a byte operand from a source to a destination, filling the destination with the sign bit of the source byte.

The following table covers the supported source and destination operands:

| source | destination(s) |
| :--- | :--- |
| m1 | #I |
| reg | reg |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode as part of the SSE4.1 instruction set. It requires the destination register to be an XMM register.

The destination register is overwritten; therefore, the original value of the destination register must be preserved in another register if it is required for subsequent operations. Failure to ensure the destination is an XMM register will result in an invalid opcode exception.