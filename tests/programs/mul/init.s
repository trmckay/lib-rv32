.section .text.init

la   sp, __sp-32

li   a0, 4
li   a0, 5

call mul

end: j end