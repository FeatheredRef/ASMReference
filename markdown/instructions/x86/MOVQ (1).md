> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# MOVQ (1)

Copies a quad word (64 bits) from the source operand to the destination operand.

The following table covers what the source and destinations can be:

| source | destination(s) |
| :--- | :--- |
| r64 | r64 |
| m8 | r64 |
| imm | r64 |
| r64 | m8 |

DO NOT support LOCK

The instruction SHALL be used in 64-bit operand size mode. If the processor is operating in compatibility mode, the behavior is restricted to 32-bit operations unless specifically targeting XMM registers (MOVQ for SSE), which is a distinct encoding.

When moving an immediate to a register, the immediate SHALL be sign-extended if it is smaller than 64 bits. Memory accesses SHALL be aligned to the architectural requirements of the processor to avoid performance penalties or alignment check exceptions. Writing to a memory location (m8) SHALL overwrite the existing qword at that address.