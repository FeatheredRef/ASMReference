> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# XGETBV

Retrieves the contents of the extended control register specified by the immediate value (ECX) and stores the result in the destination register pair (EDX:EAX).

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | r32, r32 |

DO NOT support LOCK

The instruction is only available in 64-bit mode and compatibility mode. It is NOT supported in 32-bit mode. If executed in 32-bit mode, it will trigger an invalid opcode exception.

The immediate value provided in ECX MUST be a valid index for an extended control register (e.g., XCR0). If the requested register is not supported by the processor or is disabled in the BIOS/firmware, the instruction SHALL trigger a general-protection exception (#GP(0)). Use the `CPUID` instruction to verify support for the specific extended state before execution to avoid #GP.