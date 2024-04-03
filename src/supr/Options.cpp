#include <supr/Options.hpp>

#include <gubg/cli/Range.hpp>
#include <gubg/mss.hpp>

#include <sstream>

namespace supr {

    bool Options::parse(int argc, const char **argv)
    {
        MSS_BEGIN(bool);

        gubg::cli::Range range{argc, argv};
        MSS(range.pop(exe_name));

        for (std::string arg; range.pop(arg);)
        {
            auto is = [&](const char *sh, const char *lh) { return arg == sh || arg == lh; };
            if (false) {}
            else if (is("-h", "--help")) { verb = Verb::PrintHelp; }
            else if (is("-C", "--folder")) { MSS(range.pop(folder.emplace())); }
            else if (is("i", "info")) { verb = Verb::Info; }
        }

        MSS_END();
    }

    std::string Options::help() const
    {
        std::ostringstream oss;
        oss << "Help for 'supr' (" << version_.to_str("v") << ")" << std::endl;
        oss << "-h       --help         Print this help" << std::endl;
        oss << "-C dir   --folder dir   Use folder as base" << std::endl;
        oss << "Developed by Geert Fannes." << std::endl;
        return oss.str();
    }

    std::ostream &operator<<(std::ostream &os, Verb verb)
    {
        switch (verb)
        {
            case Verb::PrintHelp: os << "PrintHelp"; break;
            case Verb::Info: os << "Info"; break;
        }
        os << "[Unknown](verb:" << (int)verb << ")";
        return os;
    }

} // namespace supr
