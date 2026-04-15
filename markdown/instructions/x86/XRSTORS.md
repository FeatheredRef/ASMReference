> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# XRSTORS

Restores the extended processor state from a memory region into the processor registers. This instruction reads a state component bitmap from the memory region to determine which state components are present and should be restored.

The table below covers the supported source and destinations.

| source | destination(s) |
| :--- | :--- |
| mN | Internal State |

DO NOT support LOCK

This instruction is only available in 64-bit mode or compatibility mode. It is not supported in 32-bit protected mode.

The memory region mN SHALL be 16-byte aligned; otherwise, a general-protection exception (#GP) is generated. The state component bitmap MUST be valid and correspond to the supported features of the processor; invalid bits in the bitmap may cause the instruction to fail or result in undefined behavior.

The memory region pointed to by the operand SHALL be readable. If the memory region is not properly aligned or if the requested state components exceed the allocated buffer size, the processor SHALL trigger a #GP. Using a non-aligned memory address is a common cause of application crashes when implementing context switching.