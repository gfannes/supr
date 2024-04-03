#include <supr/App.hpp>
#include <supr/Options.hpp>

#include <gubg/mss.hpp>
#include <iostream>

int main(int argc, const char **argv)
{
    MSS_BEGIN(int);

    supr::Options options;
    MSS(options.parse(argc, argv));

    supr::App app{options};
    MSS(app.run(), std::cout << "Something went wrong" << std::endl);

    std::cout << "Everything went OK" << std::endl;

    MSS_END();
    return 0;
}
