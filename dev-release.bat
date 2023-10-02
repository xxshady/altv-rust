@REM temp workaround https://github.com/google/autocxx/issues/1327
set PATH=%PATH:C:\Program Files\LLVM\bin;=%
echo %PATH%

cargo release 15.0.0-dev.%1 --execute
