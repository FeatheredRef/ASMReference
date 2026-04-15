> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# PUSHA

Pushes all 32-bit general-purpose registers onto the stack. The registers are pushed in a specific order (EBP, EDI, ESI, ESP, EBP, EBX, EDX, ECX, EAX), decrementing the stack pointer after each push.

The following table covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| Internal Registers | m4 (via ESP) |

DO NOT support LOCK

This instruction is ONLY available in compatibility mode. In 64-bit mode, PUSHA is not supported and will trigger an invalid opcode exception.

The instruction implicitly uses the ESP register to determine the destination memory address. The stack MUST be aligned to a 4-byte boundary to avoid alignment check exceptions if alignment checking is enabled. Because this instruction modifies the stack pointer (ESP) multiple times, any interrupt or exception occurring during execution MUST ensure the stack state is consistent.