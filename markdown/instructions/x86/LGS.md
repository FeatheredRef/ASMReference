> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# LGS

Loads the Global Descriptor Table (GDT) register (GDTR) from a memory location.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m8 | GDTR |

DO NOT support LOCK

LGS SHALL only be executed at CPL=0. If executed at CPL > 0, it SHALL generate a general-protection exception (#GP(0)). In 64-bit mode, the instruction loads a 8-byte value from memory into the GDTR, consisting of a 16-bit limit and a 64-bit base address. In compatibility mode, it loads a 6-byte value.

The source memory operand MUST be a byte pointer to the descriptor; the processor will automatically read the subsequent bytes required for the GDT register based on the current operating mode. Ensure that the memory region is aligned and accessible to avoid page faults or alignment checks.