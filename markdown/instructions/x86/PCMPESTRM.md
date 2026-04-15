> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PCMPESTRM

This instruction compares two operand strings (source and destination) based on the mode specified in the immediate operand. It performs a character-by-character comparison of the strings until a termination condition is met (such as finding a character not in a specified set or reaching the end of a string), returning the index of the first non-matching character and updating the destination register.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m128 | xmm |
| xmm | xmm |

DO NOT support LOCK

This instruction is available only in 64-bit mode or compatibility mode. It requires the processor to support the SSE4.2 instruction set.

The immediate operand MUST be a valid mode identifier (0-15); using an unsupported immediate value will result in an invalid opcode exception. Since the instruction operates on 128-bit XMM registers, the input strings MUST be properly aligned to avoid performance penalties, although alignment is not strictly required for correctness. The instruction's behavior is heavily dependent on the `ECX` register, which MUST contain the maximum number of characters to be processed to prevent reading beyond the intended buffer boundaries. Failure to correctly set `ECX` may lead to reading memory beyond the allocated buffer, potentially causing a general protection fault.