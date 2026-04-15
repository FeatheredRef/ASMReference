> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# KUNPCKDQ

Unpacks the quadword from the source operand to the destination operand by duplicating the high 64 bits of the source into both the low and high quadwords of the destination.

The following table covers the supported source and destination operands:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires the SSE4.1 instruction set extension to be supported by the processor.

The destination register must not be the same as the source register if the source is a memory operand, although using the same XMM register for both source and destination is permitted. Failure to ensure the target register is properly initialized or aligned when using memory operands may result in general protection faults.