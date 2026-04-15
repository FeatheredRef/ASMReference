> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPERMT2D

Permutes 32-bit elements from two 256-bit vector sources based on indices provided in a third 256-bit vector source. For each 32-bit element in the destination, the instruction selects an element from either the first or second source vector based on the index provided in the control vector.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm/ymm reg | ymm reg |
| xmm/ymm reg | ymm reg |
| xmm/ymm reg | ymm reg |

DO NOT support LOCK

The instruction SHALL only be executed in 64-bit mode or compatibility mode. It REQUIRES the AVX2 instruction set extension.

Indices in the control vector that are greater than 15 SHALL be ignored; the corresponding destination element is set to zero. Indices that specify an element from the second source vector are indicated by the most significant bit of the index byte. Ensure that the destination register is not used as a source if the operation depends on the previous value of that register, as the instruction overwrites the destination.