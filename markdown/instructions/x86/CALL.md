> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# CALL

Pushes the current return address (the address of the instruction following the CALL) onto the stack and transfers execution to the target address.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | #I |
| reg | #I |
| m64 | #I |
| #I | imm |
| #I | reg |
| #I | m64 |

DO NOT support LOCK

In 64-bit mode, the instruction SHALL operate on 64-bit operands. In compatibility mode, the instruction SHALL operate on 32-bit operands. The target address MUST be a valid executable memory location.

When using a memory operand for the target address, the processor SHALL read the address from the specified m64. If the stack pointer (RSP) is not properly aligned before a CALL, certain ABI-specific requirements for function calls may be violated, potentially leading to faults during the execution of the called procedure. The instruction automatically decrements RSP by 8 bytes (in 64-bit mode) before storing the return address.