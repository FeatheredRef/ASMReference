> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PCMPGTD

Compares two packed signed 32-bit integers in each element of the source operands. For each element, the instruction sets the corresponding dword in the destination to all ones (mask) if the first operand is greater than the second; otherwise, it sets the destination dword to zero.

The table below covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction is available in both 32-bit and 64-bit modes. It requires the SSE4.1 instruction set extension to be supported by the processor.

The destination register must be an xmm register; memory is not supported as a destination. Using an unsupported operand size or an invalid register type SHALL result in an invalid opcode exception. Use of this instruction on a processor lacking SSE4.1 support SHALL trigger an invalid opcode exception.