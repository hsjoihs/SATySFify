c:
	clang-format main.c -i
	gcc main.c -o a.out
	./a.out '\int e^x x^3dx'

all:
	clang-format main.c -i
	gcc main.c -o a.out
	./a.out 42 > test/test001.saty
	satysfi test/test001.saty
