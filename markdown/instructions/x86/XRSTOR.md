> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# XRSTOR

Restores the state of the floating-point and SIMD registers from a memory-resident structure to the processor. The instruction loads the state of the registers based on the state-component bitmap provided in the structure header.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m512 | reg |

DO NOT support LOCK

The instruction is available in 64-bit mode and compatibility mode. The memory operand MUST be 16-byte aligned; otherwise, a general-protection exception (#GP) is generated.

To avoid unexpected behavior or crashes, ensure that the `XRSTOR` operation is performed with the appropriate privilege level. If the state being restored contains components that require higher privileges than the current CPL, the processor SHALL trigger a general-protection exception (#GP). Additionally, software MUST verify that the memory region provided is valid and contains a correctly formatted `XSAVE` area to prevent the corruption of the SIMD register state.