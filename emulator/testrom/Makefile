jtest.gb: jtestrom.asm
	rgbasm $^ -o jtestrom.o
	rgblink -d -o $@ jtestrom.o
	rgbfix -p 0 -r 0 -v $@
	rm jtestrom.o

blank.gb: blank.asm
	rgbasm $^ -o blank.o
	rgblink -d -o $@ blank.o
	rgbfix -p 0 -r 0 -v $@
	rm blank.o
