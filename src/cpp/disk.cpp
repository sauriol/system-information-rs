#include <iostream>
#include <boost/filesystem.hpp>

using namespace boost::filesystem;
using std::cout;
using std::endl;

int main(int argc, char* argv[])
{
    if (argc != 2) {
        return 1;
    }
    space_info si = space(argv[1]);

    uintmax_t in_use = si.capacity- si.free;

    cout << si.capacity << endl;
    cout << si.free     << endl;
    cout << in_use      << endl;
}
