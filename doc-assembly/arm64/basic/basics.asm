  .global _main ;Specify the entrypoint of the program here the label _main
  .align 4      ;Assembly directive that specify everything follows is align to 4bytes boundary

  _main:
      MOV   X0,#1              ; arg[0] = 1 = STDOUT
      ADR   X1, helloworld    ; arg[1] = string to print
      MOV   X2,#16             ; arg[2] = Length of our string
      MOV   X16,#4             ; Unix write system call
      SVC   #0x80              ; Call kernel to output the string

      MOV   X0,#0              ; use 0 return code
      MOV   X16,#1             ; Unix exit system call
      SVC   #0xFFF             ; Call kernel to stop program

  helloworld:     .ascii "Hello M1-World!\n"
