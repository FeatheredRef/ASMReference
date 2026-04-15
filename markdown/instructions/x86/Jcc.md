> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# Jcc

Conditional jump. If the specified condition is met, the instruction transfers control to the target address by updating the RIP register.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| EFLAGS | imm |

DO NOT support LOCK

The instruction is available in 64-bit mode, 32-bit mode, and 16-bit mode. In 64-bit mode, the jump target is relative to the current RIP and must be within a signed 32-bit displacement range (-2 GB to +2 GB).

To avoid unexpected behavior, ensure the condition is based on the most recent flags-affecting instruction, as subsequent instructions may modify EFLAGS and change the outcome of the conditional jump. If a jump target is calculated to be outside the reachable range of a 32-bit signed immediate, a near jump will fail; in such cases, an unconditional jump or a register-indirect jump SHALL be used.