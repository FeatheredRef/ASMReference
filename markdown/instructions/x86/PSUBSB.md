> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PSUBSB

Subtracts the unsigned byte value from the source operand from the unsigned byte value in the destination operand for each corresponding element of the source and destination bytes.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It requires the SSE3 instruction set extension to be supported by the processor.

The operation is performed on 16 unsigned bytes (u8) packed into xmm registers. The subtraction is performed modulo $2^8$; results that wrap around do not trigger exceptions or affect CPU flags. Failure to ensure the destination register is properly initialized may result in the processing of garbage data.