#include <iostream>
#include <windows.h>

using std::cout;
using std::endl;

int main()
{
    MEMORYSTATUSEX memInfo;
    memInfo.dwLength = sizeOf(MEMORYSTATUSEX);
    GlobalMemoryStatusEx(&memInfo);

    DWORDLONG totalMemPhys = memInfo.ullTotalPhys;
    DWORDLONG totalmMemAvail = memInfo.ullAvailPhys;
    DWORDLONG totalMemInUse = totalMemPhys - totalMemAvail;

    cout << totalMemPhys << endl;
    cout << totalMemAvail << endl;
    cout << totalMemInUse << endl;
}
