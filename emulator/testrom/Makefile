jtest.gb: jtestrom.asm
	rgbasm $^ -o jtestrom.o
	rgblink -d -o $@ jtestrom.o
	rgbfix -p 0 -r 0 -v $@
	rm jtestrom.o
	
dtest.gb: dtestrom.asm
	rgbasm $^ -o dtestrom.o
	rgblink -d -o $@ dtestrom.o
	rgbfix -p 0 -r 0 -v $@
	rm dtestrom.o
