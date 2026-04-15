> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# VPMOVSQB

Moves a quadword from the source operand to the destination operand.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| xmm | xmm |
| m8 | xmm |
| #I | imm |

DO NOT support LOCK

This instruction is available only in 64-bit mode. It is not supported in compatibility mode.

The instruction requires the destination to be an XMM register. When the source is a memory operand (m8), it is treated as a quadword access to the address specified. Failure to align memory operands according to the architectural requirements may result in performance degradation or alignment check exceptions depending on the CR0.AMD setting.