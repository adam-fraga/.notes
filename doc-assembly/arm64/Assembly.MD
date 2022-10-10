#ARM64

## REGISTER

Registers are area in memory which are very close to the CPU, they can be access quickly and reading to quickly.
Register are compose by multiple Zeros each zero is an hexadecimal value (4 or 8 bits depending on arch).
In ARM64 every instruction is encoded in 32bites and code must be align by 4 bytes boundary.

**Register size depend of the architecture**

- x64 -> 64 bits / register
- x32 -> 32 bits / register

"Word" refere to the total size can be store on a register.
"Half Word" half size can be store in a register.
"Byte" Single piece of data. (8bits)

**Type of registers**

`X0-X30` 31 General purpose register.

`W0-30` Lower 32 bites of General purpose register.

`R7` System call interoperate with the OS each value stored is mapping to an action like 1 = "interrupt program"

`SP` Stack pointer, related to stack memory (on RAM) slower to access and write but more space than register.
tell us the address of the next available piece of memory on the stack.

`LR` (=X30) Link register store the location that a function should return back to.

`PC` Program counter, keep track to the next instruction to execute. It contain the address of the next instruction to
execute.

`FP` (=X29) Frame pointer, point to the stack base during a function call, to recover stack from the calling function.

`CPSR` Store information about our program, can represent some behaviors which doesnt exist in binary with flag.

`XZR` Zero register always set to zero.

`X18` RESERVED BY APPLE ON MACOS DO NOTE USE IT!!!

**Level of privileges**

`USR` User mode leas privileges standart of program execution.
`SVC` Supervisor mode kernel lvl of privileges.

---
