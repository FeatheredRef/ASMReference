> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# JMP

The JMP instruction implements an unconditional transfer of control by updating the RIP register to the specified target address.

The table below covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| imm | RIP |
| reg | RIP |
| mN | RIP |

DO NOT support LOCK

In 64-bit mode, the target address must be a canonical address. If the target address is non-canonical, a general-protection exception (#GP) SHALL be generated. The instruction is available in both 64-bit mode and compatibility mode.

When using an immediate operand, the offset is treated as a signed i32 relative to the current RIP. If the target is an absolute address provided via a register or memory operand, the RIP is updated directly to that value. To avoid unintended behavior, ensure that target addresses are properly aligned with the intended instruction boundaries to prevent the processor from executing data as code.