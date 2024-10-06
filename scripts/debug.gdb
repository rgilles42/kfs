target remote localhost:1234
set disassembly-flavor intel

layout split

break panic_inner
commands 
	break *0xc017ee0f 
	c
end
c
