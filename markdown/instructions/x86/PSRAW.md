> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PSRAW

The PSRAW instruction performs a packed raw bit-stream write operation, transferring raw data from a source operand to a destination operand without performing any arithmetic or logical transformations.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode or compatibility mode. It requires the SSE3 instruction set extension to be supported by the hardware.

To avoid alignment exceptions, the memory operand m128 SHALL be aligned on a 16-byte boundary. If the operand is not aligned, the processor may trigger a general protection fault (#GP) or performance degradation depending on the alignment check (AC) flag in the EFLAGS register.