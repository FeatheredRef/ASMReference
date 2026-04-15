> This content has not been validated; it was AI-generated following the [x86-64 ISA Documentation generation](</articles/x86-64-isa-doc-gen.md>) process to maintain high quality. However, as no human has verified this data, errors may exist. If the accuracy of this data is critical, please consult the [Intel SDM](<https://cdrdv2.intel.com/v1/dl/getContent/671200>). The model used to generate this is [gemma4:31b](<https://ollama.com/library/gemma4>)

# ADC

Adds the source operand and the destination operand, plus the value of the Carry Flag (CF), and stores the result in the destination operand.

The following table covers the supported source and destination operands.

| source | destination(s) |
| :--- | :--- |
| reg | reg |
| imm | reg |
| reg | mN |
| imm | mN |
| mN | reg |
| mN | mN |

DO NOT support LOCK

The instruction is available in 16-bit, 32-bit, and 64-bit operand sizes. When operating in 64-bit mode, the instruction supports r64, r32, r16, and r8 registers.

The instruction updates the following flags based on the result: CF, ZF, SF, PF, and AF (only for operand size $\le$ 32 bits). The Overflow Flag (OF) is updated based on the result of the addition of the source and destination, ignoring the carry-in from CF.

To avoid unexpected behavior during multi-precision arithmetic, the programmer MUST ensure that the Carry Flag is correctly initialized (e.g., using `CLC` or `STC`) before the first `ADC` or `ADD` operation in a sequence. Failure to do so SHALL result in an incorrect sum due to an indeterminate carry-in.