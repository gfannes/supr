#ifndef HEADER_supr_App_hpp_ALREAD_INCLUDED
#define HEADER_supr_App_hpp_ALREAD_INCLUDED

#include <supr/Options.hpp>

namespace supr {

    class App
    {
    public:
        App(const Options &options)
            : options_(options) {}

            bool run();

    private:
        void print_help_() const;
        bool collect_info_() const;

        const Options options_;
    };

} // namespace supr

#endif
