> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VFPCLASSSD

Compares a scalar double-precision floating-point value in an XMM register with a specified immediate value and sets the flags in the MXCSR register according to the class of the value.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| f64 | reg |
| imm | reg |

DO NOT support LOCK

This instruction is only available in compatibility mode. It operates on the legacy SSE floating-point state and requires the processor to be executing in a mode that supports the XMM register set.

The immediate operand MUST be a valid class identifier as defined by the SSE specification; providing an unsupported immediate value SHALL result in an #I exception. The instruction does not modify the source register but modifies the MXCSR register flags based on the comparison result.