#include <stdio.h>
#include <stdlib.h>
#include <string.h>

enum TokenType {
  ALPHANUMERIC,
  BACKSLASH_FOLLOWED_BY_ALPHANUMERICS,
  CARET,
  UNDERSCORE,
  ORDINARY_OPERATOR,
  LEFT_PAREN,
  RIGHT_PAREN,
  LEFT_BRACE,
  RIGHT_BRACE,
  END,
};

struct Token {
  enum TokenType kind;
  const char *string_representation;
};

struct Token get_token(const char **ptr_to_str) {
  const char *str = *ptr_to_str;
  struct Token t;
  t.string_representation = 0;

  if (*str == 0) { /* '\0' is 0 in C */
    t.kind = END;
    return t;
  }

  if (*str == ' ' || *str == '\t' || *str == '\n' || *str == '\v' ||
      *str == '\f' || *str == '\r') {
    ++*ptr_to_str;
    return get_token(ptr_to_str);
  }

  if (*str == '+') {
    t.kind = ORDINARY_OPERATOR;
    t.string_representation = "+";
    ++*ptr_to_str;
    return t;
  } else if (*str == '*') {
    t.kind = ORDINARY_OPERATOR;
    t.string_representation = "*";
    ++*ptr_to_str;
    return t;
  } else if (*str == '(') {
    t.kind = LEFT_PAREN;
    t.string_representation = "(";
    ++*ptr_to_str;
    return t;
  } else if (*str == ')') {
    t.kind = RIGHT_PAREN;
    t.string_representation = ")";
    ++*ptr_to_str;
    return t;
  } else if (*str == ',') {
    t.kind = ORDINARY_OPERATOR;
    t.string_representation = ",";
    ++*ptr_to_str;
    return t;
  } else if (*str == '^') {
    t.kind = CARET;
    t.string_representation = "^";
    ++*ptr_to_str;
    return t;
  } else if (*str == '{') {
    t.kind = LEFT_BRACE;
    t.string_representation = "{";
    ++*ptr_to_str;
    return t;
  } else if (*str == '}') {
    t.kind = RIGHT_BRACE;
    t.string_representation = "}";
    ++*ptr_to_str;
    return t;
  } else if (*str == '<') {
    t.kind = ORDINARY_OPERATOR;
    t.string_representation = "<";
    ++*ptr_to_str;
    return t;
  } else if (*str == '>') {
    t.kind = ORDINARY_OPERATOR;
    t.string_representation = ">";
    ++*ptr_to_str;
    return t;
  } else if (*str == '=') {
    t.kind = ORDINARY_OPERATOR;
    t.string_representation = "=";
    ++*ptr_to_str;
    return t;
  } else if (*str == '_') {
    t.kind = UNDERSCORE;
    t.string_representation = "_";
    ++*ptr_to_str;
    return t;
  }

  if (strchr("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
             *str) != NULL) {
    t.kind = ALPHANUMERIC;
    char *rep = calloc(3, sizeof(char));
    rep[0] = *str;
    rep[1] = ' ';
    t.string_representation = rep;
    ++*ptr_to_str;
    return t;
  }

  if (*str == '\\') {
    // If none of these, that's a problem
    if (strchr("_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ",
               str[1]) == NULL) {
      fprintf(stderr,
              "Found unexpected character after a backslash: '%c' (%d)\n",
              str[1], (int)(str[1]));
      exit(EXIT_FAILURE);
    }

    t.kind = BACKSLASH_FOLLOWED_BY_ALPHANUMERICS;
    int i = 2;

    for (;; ++i) {
      if (strchr("_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
                 "0123456789",
                 str[i]) == NULL) {
        break;
      }
    }
    /*
        identifier: str[1] ~ str[i-1]
    */
    char *new_str = malloc(i + 1);

    for (int j = 0; j < i; j++) {
      new_str[j] = str[j];
    }
    new_str[i] = '\0';
    t.string_representation = new_str;
    *ptr_to_str = str + i;
    return t;
  }

  fprintf(stderr, "Found unexpected character: '%c' (%d)\n", *str, (int)*str);
  exit(EXIT_FAILURE);
}

int main(int argc, char **argv) {
  if (argc != 2) {
    fprintf(stderr, "Incorrect number of arguments\n");
    return 1;
  }
  const char *str = argv[1];
  const char **ptr_to_input = &str;

  int i = 0;
  while (1) {
    struct Token t = get_token(ptr_to_input);

    i++;
    if (t.kind == END) {
      break;
    }
    fprintf(stderr, "%s\n", t.string_representation);
  }

  struct Token *tokens = calloc(i, sizeof(struct Token));

  for (int j = 0;;) {
    struct Token t = get_token(ptr_to_input);
    tokens[j] = t;
    j++;

    if (t.kind == END) {
      break;
    }
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
  printf("      %d\n", 42);
  printf("    });\n");
  printf("  >\n");
  printf(">\n");

  return 0;
}
