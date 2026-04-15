> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ENDBR32

Marks a valid indirect branch target for 32-bit code in compatibility mode. If the processor is in Control-flow Enforcement Technology (CET) Indirect Branch Tracking (IBT) enabled state and the instruction following an indirect branch is not ENDBR32, a Control Protection (#CP) exception is generated.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

This instruction is only available when the processor is operating in compatibility mode. It is used specifically to mark valid targets for indirect jumps or calls targeting 32-bit code.

To avoid a #CP exception, every legitimate indirect branch target in 32-bit code MUST start with an ENDBR32 instruction when CET IBT is active. If the instruction is executed directly (via a sequential flow), it behaves as a NOP.