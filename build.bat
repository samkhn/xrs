@ECHO OFF

where /q cindex
IF ERRORLEVEL==0 (
   cindex .
)

SET "PROJECTDIR=%cd:~3%"
SET "BUILDDIR=C:\build\%PROJECTDIR%"

IF NOT EXIST %BUILDDIR% (
	MD %BUILDDIR%
)

cargo check --target-dir=%BUILDDIR%
cargo build --target-dir=%BUILDDIR%
