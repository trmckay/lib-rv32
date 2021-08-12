.section .text.init

start:
    la   sp, __sp-32

    li   a0, 4
    li   a1, 5

    call mul

    li t0, 0x0F00
    sw a0, 0(t0)

end: j end
