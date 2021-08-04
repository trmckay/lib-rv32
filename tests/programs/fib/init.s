.section .text.init

start:
    la   sp, __sp-32
    li   a0, 5

    call fib

end: j end
