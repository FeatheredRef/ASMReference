> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# LTR

Loads the selector from the operand into the LT (Local Descriptor Table) register.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | LT Register |
| m16 | LT Register |
| imm | #I |

DO NOT support LOCK

This instruction is ONLY available in compatibility mode. In 64-bit mode, LTR is not supported and will trigger an invalid opcode exception.

The operand MUST be a valid 16-bit selector. If the selector is null (0), the LT register is cleared. If the selector is not null, the processor SHALL verify that the index is within the limits of the GDT and that the descriptor is a valid LDT descriptor. Failure to meet these conditions SHALL trigger a General Protection Fault (#GP).