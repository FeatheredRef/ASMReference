> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SENDUIPI

Sends an Unaddressed User-Level Interrupt (UIPI) to the local processor.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | #I |
| reg | #I |
| mN | #I |

DO NOT support LOCK

The instruction is only supported on processors that implement User-Level Interrupts. It MUST be executed in CPL 3; execution at a higher privilege level SHALL result in an undefined operation or a general-protection exception (#GP), depending on the processor implementation.

The instruction does not take any operands. Attempting to specify a source or destination operand SHALL result in an invalid opcode exception. The target of the interrupt is always the local processor (self-IPI).