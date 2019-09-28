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

struct Token get_token(const char *const initial, size_t *ptr_offset) {
  struct Token t;
  t.string_representation = 0;

  if (initial[*ptr_offset] == 0) { /* '\0' is 0 in C */
    t.kind = END;
    return t;
  }

  if (initial[*ptr_offset] == ' ' || initial[*ptr_offset] == '\t' ||
      initial[*ptr_offset] == '\n' || initial[*ptr_offset] == '\v' ||
      initial[*ptr_offset] == '\f' || initial[*ptr_offset] == '\r') {
    ++*ptr_offset;
    return get_token(initial, ptr_offset);
  }

  if (initial[*ptr_offset] == '+') {
    t.kind = ORDINARY_OPERATOR;
    t.string_representation = "+";
    ++*ptr_offset;
    return t;
  } else if (initial[*ptr_offset] == '*') {
    t.kind = ORDINARY_OPERATOR;
    t.string_representation = "*";
    ++*ptr_offset;
    return t;
  } else if (initial[*ptr_offset] == '(') {
    t.kind = LEFT_PAREN;
    t.string_representation = "(";
    ++*ptr_offset;
    return t;
  } else if (initial[*ptr_offset] == ')') {
    t.kind = RIGHT_PAREN;
    t.string_representation = ")";
    ++*ptr_offset;
    return t;
  } else if (initial[*ptr_offset] == ',') {
    t.kind = ORDINARY_OPERATOR;
    t.string_representation = ",";
    ++*ptr_offset;
    return t;
  } else if (initial[*ptr_offset] == '^') {
    t.kind = CARET;
    t.string_representation = "^";
    ++*ptr_offset;
    return t;
  } else if (initial[*ptr_offset] == '{') {
    t.kind = LEFT_BRACE;
    t.string_representation = "{";
    ++*ptr_offset;
    return t;
  } else if (initial[*ptr_offset] == '}') {
    t.kind = RIGHT_BRACE;
    t.string_representation = "}";
    ++*ptr_offset;
    return t;
  } else if (initial[*ptr_offset] == '<') {
    t.kind = ORDINARY_OPERATOR;
    t.string_representation = "<";
    ++*ptr_offset;
    return t;
  } else if (initial[*ptr_offset] == '>') {
    t.kind = ORDINARY_OPERATOR;
    t.string_representation = ">";
    ++*ptr_offset;
    return t;
  } else if (initial[*ptr_offset] == '=') {
    t.kind = ORDINARY_OPERATOR;
    t.string_representation = "=";
    ++*ptr_offset;
    return t;
  } else if (initial[*ptr_offset] == '_') {
    t.kind = UNDERSCORE;
    t.string_representation = "_";
    ++*ptr_offset;
    return t;
  }

  if (strchr("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
             initial[*ptr_offset]) != NULL) {
    t.kind = ALPHANUMERIC;
    char *rep = calloc(3, sizeof(char));
    rep[0] = initial[*ptr_offset];
    rep[1] = ' ';
    t.string_representation = rep;
    ++*ptr_offset;
    return t;
  }

  if (initial[*ptr_offset] == '\\') {
    // If none of these, that's a problem
    if (strchr("_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ",
               initial[1 + *ptr_offset]) == NULL) {
      fprintf(stderr,
              "Found unexpected character after a backslash: '%c' (%d)\n",
              initial[1 + *ptr_offset], (int)(initial[1 + *ptr_offset]));
      exit(EXIT_FAILURE);
    }

    t.kind = BACKSLASH_FOLLOWED_BY_ALPHANUMERICS;
    int i = 2;

    for (;; ++i) {
      if (strchr("_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
                 "0123456789",
                 (initial + *ptr_offset)[i]) == NULL) {
        break;
      }
    }
    /*
        identifier: (initial + *ptr_offset)[1] ~ (initial + *ptr_offset)[i-1]
    */
    char *new_str = malloc(i + 1);

    for (int j = 0; j < i; j++) {
      new_str[j] = initial[*ptr_offset + j];
    }
    new_str[i] = '\0';
    t.string_representation = new_str;
    *ptr_offset += i;
    return t;
  }

  fprintf(stderr, "Found unexpected character: '%c' (%d)\n",
          initial[*ptr_offset], (int)initial[*ptr_offset]);
  exit(EXIT_FAILURE);
}
