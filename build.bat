@for /r %%f in (*.go) do (
    @echo Processing %%~f
    @go fmt "%%~f" || goto :gobuildfailed
    @go build "%%~f" || goto :gobuildfailed
    @py -3 -mpygments.cmdline -f img -o "%%~nf-go.png" "%%~f" || goto :gobuildfailed
)
goto buildrust

:gobuildfailed
@echo !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
@echo Build failure during GO Build!!!!!
@echo !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
goto end

:buildrust
@for /r %%f in (*.rs) do (
    @echo Processing %%~f
    @rustc "%%~f" || gogo :rustbuildfailed
    @py -3 -mpygments.cmdline  -f img -o "%%~nf-rs.png" "%%f" || goto :rustbuildfailed
)
goto end

:rustbuildfailed

@echo !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
@echo Build failure during Rust Build!!!!!
@echo !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

:end
