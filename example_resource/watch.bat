@REM TODO: use proper crate for creating "npm like scripts" 
@REM but for now it doesn't matter

@REM !! requires cargo-watch to be installed

cargo watch --exec build --why --watch ../altvx --watch ../resource_main_macro --watch ../resource_impl --watch .
