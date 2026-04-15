> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# DAA

Adjusts the AL register to be in packed BCD format after an addition operation. It checks the Auxiliary Carry Flag (AF) and the lower nibble of AL; if AF is set or the lower nibble is greater than 9, it adds 6 to the lower nibble. It then checks if the upper nibble of AL is greater than 9 or if a carry occurred from the lower nibble to the upper nibble; if so, it adds 6 to the upper nibble and sets the Carry Flag (CF).

The table after the description covers what the source and destinations can be.

| source | destination(s) |
| :--- | :--- |
| reg (AL) | reg (AL) |
| imm | #I |
| mN | #I |

DO NOT support LOCK

The DAA instruction is only available in 32-bit operation or when the processor is operating in compatibility mode. It is NOT supported in 64-bit operation mode.

To avoid incorrect BCD results, DAA MUST be executed immediately following an addition instruction (such as ADD or ADC) that operated on the AL register. Because the instruction relies on the state of the AF and CF flags, any intervening instruction that modifies these flags will result in incorrect decimal adjustment.