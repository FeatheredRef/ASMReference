> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPDPWSSD

Multiplies signed word-size elements from a source register and a source memory or register location, adds the results to the destination register elements, and stores the result as signed doubleword-size elements.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| m16 | reg |

DO NOT support LOCK

This instruction SHALL be used only in 64-bit mode. It is not supported in compatibility mode.

The destination register MUST be larger than the source registers to accommodate the widening from word (16-bit) to doubleword (32-bit) results. Failure to ensure the destination register is correctly initialized or sized for the vector length may result in undefined behavior or General Protection faults if accessing out-of-bounds memory.