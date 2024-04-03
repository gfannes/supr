#ifndef HEADER_supr_Options_hpp_ALREAD_INCLUDED
#define HEADER_supr_Options_hpp_ALREAD_INCLUDED

#include <gubg/Version.hpp>

#include <optional>
#include <string>

namespace supr {

    enum class Verb
    {
        PrintHelp,
        Info,
    };
    std::ostream &operator<<(std::ostream &os, Verb verb);

    class Options
    {
    public:
        std::string exe_name;

        std::optional<Verb> verb;
        std::optional<std::string> folder;

        bool parse(int argc, const char **argv);

        std::string help() const;

    private:
        gubg::Version version_{.major = 0, .minor = 0, .patch = 1};
    };

} // namespace supr

#endif
