=========
Changelog
=========

Version 0.6.x
===========
- REFACTOR: remove hardcoded strings in to codebase in favor of templates loaded by ```include_str!() `` macro
- Damn I ❤️ Rust ^^^
- 

Version 0.5.45
===========
- FIX: nasty bug fixed 
    This was related to optional args passed to the buildcpp that worked fine in testing the component 
    but did not work after I published it.  This is difficult to unit test due to the nature of these plugins.
    I removed clap for now until I understand the problem better.


Version 0.5.41-44 
===========
- Yanked from crates.io (buggy buildcpp)

Version 0.5.x
===========
- First update in a while
- Updated depedancies (handlebars, etc)
- Add support for release and debug builds
    - buildcpp [--release | {--debug} ]  
- Add cleancpp subcommand


Version 0.4.x
===========
- Stable release 
- Improvements to:
    - project creation (newcpp)
    - buildcpp 
- 

Version 0.3.x
===========

- Basic scaffolding and CMake
- Create new C++ project only
- Added documentation


Version 0.2.x
===========

- First release to crates.io (almost usable)
- 

Version 0.1
===========

- Early Proof of concept 
