> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# RDSEED

RDSEED reads a high-quality hardware random number from the hardware random number generator and stores it in the destination register.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| #I | r64 |

DO NOT support LOCK

RDSEED is only available in 64-bit mode or compatibility mode. It is not supported in 32-bit mode.

The instruction MAY fail to generate a random number if the hardware random number generator has not yet collected enough entropy. When this occurs, the destination register is not updated, and the carry flag (CF) is set to 1. The software SHALL check the value of CF after execution; if CF=1, the software MUST retry the instruction. Failure to verify CF may result in using stale data from the destination register.