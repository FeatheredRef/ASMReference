> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# LLDT

Loads the Local Descriptor Table register (LDTR) with the 16-bit selector specified by the source operand.

The following table specifies the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg | LDTR |
| m16 | LDTR |
| imm | #I |

DO NOT support LOCK

LLDT is available in legacy mode and compatibility mode. It is NOT supported in 64-bit mode.

The selector must be a valid index into the Global Descriptor Table (GDT) and must point to a Local Descriptor Table (LDT) descriptor. If the selector is null or the descriptor is not a valid LDT descriptor, a General Protection Exception (#GP) SHALL be generated. The instruction updates the hidden portion of the LDTR with the base address and limit from the referenced descriptor.