> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# INTO

Checks the Overflow Flag (OF). If OF is 1, the instruction generates an interrupt by calling the interrupt handler associated with the vector specified by the immediate operand. If OF is 0, the instruction does nothing.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm8 | #I |

DO NOT support LOCK

The `INTO` instruction is NOT supported in 64-bit mode. It is only available when the processor is operating in compatibility mode or in a 32-bit protected mode/real mode environment. Attempting to execute this instruction in 64-bit mode SHALL result in an invalid opcode exception.

The immediate operand MUST be a byte (imm8). If the Overflow Flag is set, the processor pushes the current flags, CS, and EIP/RIP onto the stack before branching to the interrupt service routine. Ensure that the interrupt vector specified by the immediate operand is correctly initialized in the Interrupt Descriptor Table (IDT) to avoid a double fault.