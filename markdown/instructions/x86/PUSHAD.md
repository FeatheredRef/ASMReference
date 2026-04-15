> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PUSHAD

Pushes the current values of the general-purpose registers EAX, ECX, EDX, EBX, ESP, EBP, ESI, and EDI onto the stack.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | m4 |

DO NOT support LOCK

This instruction is ONLY available in 32-bit mode or in compatibility mode when the processor is operating in 64-bit mode. It is NOT supported in 64-bit mode.

The instruction modifies the ESP register by decrementing it by 32 bytes (8 dwords) to accommodate the pushed registers. Failure to account for this stack shift when restoring registers via POPAD will result in stack misalignment. Since this instruction operates on 32-bit registers, it SHALL NOT be used to preserve 64-bit state in long mode.