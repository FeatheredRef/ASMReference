> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CMPXCHG16B

The instruction compares the combined value in `rdx:rax` with the value at a memory location. If they are equal, the combined value in `rcx:rbx` is stored at the memory location. If they are not equal, the value from the memory location is loaded into `rdx:rax`. The zero flag (ZF) is set if the comparison is equal, otherwise it is cleared.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | m16 |
| #I | imm |

Support LOCK

This instruction is ONLY available in 64-bit mode. It is NOT supported in compatibility mode.

The memory operand MUST be aligned on a 16-byte boundary to ensure atomicity when the LOCK prefix is used. If the memory operand spans two cache lines, the operation may result in a performance penalty or a general protection fault depending on the processor implementation.

The instruction implicitly uses `rax` and `rdx` as the comparator and `rbx` and `rcx` as the source for the exchange. Users SHALL not attempt to specify other registers as these are hardcoded in the ISA. Since the instruction modifies `rax` and `rdx` on failure, these registers MUST be preserved if their original values are required after the operation.