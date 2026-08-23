CC = gcc

CFLAGS = -Wall -Wextra -std=c11 -Iinclude

TARGET = chaos

SRC = \
	src/main.c \
	src/lexer.c \
	src/parser.c \
	src/ast.c \
	src/runtime.c \
	src/runtime_state.c

all:
	$(CC) $(CFLAGS) $(SRC) -o $(TARGET)

clean:
	rm -f $(TARGET)

run:
	./$(TARGET) examples/register.chaos