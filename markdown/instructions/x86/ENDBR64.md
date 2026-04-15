> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ENDBR64

Acts as a no-operation (NOP) instruction when Indirect Branch Tracking (IBT) is disabled. When IBT is enabled, it serves as a valid target for indirect branches; if an indirect branch transfers control to an instruction that is not ENDBR64, a control-protection exception (#CP) is generated.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | #I |

DO NOT support LOCK

The instruction SHALL only be executed in 64-bit mode. It is NOT supported in compatibility mode.

To avoid #CP exceptions, ENDBR64 MUST be placed at the exact destination of every indirect call or jump. Failure to place this instruction at the target of an indirect branch while IBT is active WILL result in an immediate exception. Since the instruction is treated as a NOP when IBT is disabled, it does not affect program logic on hardware or operating systems where IBT is not supported or enabled.