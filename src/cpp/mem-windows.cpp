#include <iostream>
#include <windows.h>

using std::cout;
using std::endl;

int main()
{
    MEMORYSTATUSEX memInfo;
    memInfo.dwLength = sizeof(MEMORYSTATUSEX);
    GlobalMemoryStatusEx(&memInfo);

    DWORDLONG totalMemPhys = memInfo.ullTotalPhys;
    DWORDLONG totalMemAvail = memInfo.ullAvailPhys;
    DWORDLONG totalMemInUse = totalMemPhys - totalMemAvail;

    cout << totalMemPhys << endl;
    cout << totalMemAvail << endl;
    cout << totalMemInUse << endl;
}
