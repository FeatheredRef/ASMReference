> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PMAXUD

Compares two unsigned 32-bit integers and stores the maximum value in the destination operand.

The following table covers the supported source and destination operands:

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m4 | reg |
| reg | m4 |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode. It requires the SSE4.1 instruction set extension to be supported by the hardware.

To avoid undefined behavior or `#I` exceptions, ensure that the operands are correctly aligned to 4-byte boundaries when using memory operands (m4), as unaligned access may incur performance penalties or trigger alignment checks depending on the architectural state.