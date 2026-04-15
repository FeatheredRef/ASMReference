> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# XLATB

The instruction replaces the byte in the AL register with a byte from a lookup table in memory. The table starting address is specified by the BX register, and the index is the current value of the AL register.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| m1 | r8 |

DO NOT support LOCK

XLATB is only available in 16-bit mode or 32-bit compatibility mode. It is NOT supported in 64-bit mode.

The instruction implicitly uses the AL and BX registers; any other register specifications are invalid. The memory access is performed as a byte read from the address calculated as `BX + AL`. Failure to ensure the table is aligned or within valid memory bounds will trigger a general-protection exception or page fault.