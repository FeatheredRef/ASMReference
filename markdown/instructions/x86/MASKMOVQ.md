> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MASKMOVQ

Moves a qword from a source to a destination based on a mask. If the mask bit is 1, the corresponding byte is moved; if the mask bit is 0, the corresponding byte in the destination remains unchanged.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m8 | xmm |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode. It is NOT supported in compatibility mode.

The destination register MUST be an XMM register; memory destinations are NOT supported. If the source is a memory operand, the instruction performs a masked load. Because the mask determines which bytes are accessed, this instruction can be used to avoid memory faults (such as segmentation faults or page faults) for bytes where the mask bit is 0. However, the memory operand MUST be aligned to the size of the data being accessed if the mask does not cover the full range, otherwise, performance penalties or faults may occur depending on the alignment check settings.