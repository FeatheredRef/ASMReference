> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# SHLX

Shifts the value of the source operand to the left by the count specified in the second operand. The bits shifted out are discarded, and zero bits are shifted into the low-order positions.

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| mem | reg |

DO NOT support LOCK

SHLX is only available in 64-bit mode. It is NOT supported in compatibility mode or 32-bit mode. 

The shift count is masked to 6 bits for r32 operands (modulo 32) and 6 bits for r64 operands (modulo 64) to prevent undefined behavior. Failure to account for this masking when using a register for the shift count MAY result in unexpected shift distances.