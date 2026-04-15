> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ENQCMDS

Enqueues a Cuckoo Monitor Start (CMS) descriptor into the specified queue.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | #I |
| reg | #I |
| mN | #I |
| #I | r64 |
| #I | m8 |

DO NOT support LOCK

This instruction is only available when the processor is operating in 64-bit mode. It SHALL NOT be used in compatibility mode.

The destination operand MUST be a valid memory location for a queue that supports the Cuckoo Monitor Start operation. Attempting to execute this instruction on an unsupported memory region MAY result in a general-protection exception (#GP). The queue must be properly initialized according to the architectural specifications for Cuckoo Monitors to avoid undefined behavior.