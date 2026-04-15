> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ENCODEKEY256

The instruction generates a key used for memory encryption by performing a series of transformations on a source value and an internal system-specific seed, storing the resulting 256-bit encrypted key into the destination.

The table below specifies the supported sources and destinations.

| source | destination(s) |
| :--- | :--- |
| reg | m32 |
| imm | m32 |
| #I | #I |

DO NOT support LOCK

This instruction is only available when the processor is operating in Long Mode. It is NOT supported in compatibility mode.

The instruction requires the destination memory region to be 32-byte aligned. If the destination address is not aligned to a 32-byte boundary, the processor SHALL generate a general-protection exception (#GP). Failure to ensure alignment will result in immediate application termination in protected environments.