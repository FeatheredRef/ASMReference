> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CMPPS

Compares the packed single-precision floating-point values in the first operand with the packed single-precision floating-point values in the second operand. The comparison is performed according to the specified predicate, and the results are stored in the destination operand as masks.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m128 | xmm |

DO NOT support LOCK

This instruction is available in 64-bit mode and compatibility mode. It requires the SSE instruction set to be supported by the processor.

The behavior of the comparison depends on the immediate byte (predicate) provided. If the predicate is invalid, the instruction will trigger an invalid operation exception (#I). The instruction does not modify the EFLAGS register. All operations are performed on packed values; therefore, the source and destination MUST be 128-bit XMM registers or aligned memory regions to avoid performance penalties or faults.