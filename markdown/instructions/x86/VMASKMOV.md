> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VMASKMOV

Moves a doubleword value from a memory location to a specific element of a SIMD register, or from a specific element of a SIMD register to a memory location, based on an immediate index.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| m4 | xmm |
| xmm | m4 |

DO NOT support LOCK

This instruction is only available in 64-bit mode or compatibility mode. It requires the processor to support the SSE2 instruction set.

The immediate operand MUST be a constant within the range of the target register's element indices (e.g., 0-3 for xmm). Providing an immediate value that exceeds the register's capacity SHALL result in an invalid operand encoding. Because this instruction accesses memory as a dword, any memory access that crosses a page boundary MAY trigger a general-protection exception (#GP) or a page fault.