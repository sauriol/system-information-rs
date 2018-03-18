CC		= g++
CFLAGS	= -I/usr/lib
LDFLAGS	= -lboost_filesystem -lboost_system

all: disk

disk: disk.cpp
	$(CC) -o $@ $^ $(CFLAGS) $(LDFLAGS)
