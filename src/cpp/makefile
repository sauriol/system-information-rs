CC	= g++
CFLAGS	= -I/usr/lib
LDFLAGS	= -lboost_filesystem -lboost_system

all: disk

windows: disk mem-windows

disk: disk.cpp
	$(CC) -o $@ $^ $(CFLAGS) $(LDFLAGS)

mem-windows: mem-windows.cpp
	$(CC) -o $@ $^
