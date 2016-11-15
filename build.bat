setlocal EnableDelayedExpansion 

@for /r %%f in (*.go) do @(
    @echo Processing %%~f
    @set filename=%%~nf
    @REM Folienbeispiele sind nicht immer kompilierbar
    @if NOT "!filename:~0,5!"=="folie" @go fmt "%%~f" || goto :gobuildfailed
    @if NOT "!filename:~0,5!"=="folie" @go build "%%~f" || goto :gobuildfailed
    @py -3 -mpygments.__init__ -f img -O font_size=48 -o "%%~nf-go.png" "%%~f" || goto :gobuildfailed
)
goto buildrust

:gobuildfailed
@echo !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
@echo Build failure during GO Build!!!!!
@echo !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
goto end

:buildrust
@for /r %%f in (*.rs) do @(
    @echo Processing %%~f
    @set filename=%%~nf
    @REM Folienbeispiele sind nicht immer kompilierbar
    @if NOT "!filename:~0,5!"=="folie" @rustc "%%~f" || goto :rustbuildfailed
    @py -3 -mpygments.__init__ -f img -O font_size=48 -o "%%~nf-rs.png" "%%f" || goto :rustbuildfailed
)
goto end

:rustbuildfailed

@echo !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
@echo Build failure during Rust Build!!!!!
@echo !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

:end
