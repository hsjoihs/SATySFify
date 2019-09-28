#include <stdio.h>
#include <stdlib.h>

int main(int argc, char **argv)
{
    if (argc != 2)
    {
        fprintf(stderr, "引数の個数が正しくありません\n");
        return 1;
    }

    printf("@require: stdjabook\n");
    printf("@require: code\n");
    printf("@require: itemize\n");
    printf("@require: tabular\n");
    printf("@require: math\n");
    printf("\n");
    printf("document (|\n");
    printf("  title = {};\n");
    printf("  author = {};\n");
    printf("  show-title = false;\n");
    printf("  show-toc = false;\n");
    printf("|) '<\n");
    printf("  +section{}<\n");
    printf("    +math(${\n");
    printf("      %d\n", atoi(argv[1]));
    printf("    });\n");
    printf("  >\n");
    printf(">\n");

    return 0;
}
