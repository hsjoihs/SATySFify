c:
	clang-format main.c -i
	gcc main.c -o a.out -Wall
	./a.out '\int e^x x^3dx'

all:
	clang-format main.c -i
	gcc main.c -o a.out
	./a.out 42 > test/test001.saty
	satysfi test/test001.saty
	./a.out '\int e^x x^3dx' > test/test002.saty
	satysfi test/test002.saty
