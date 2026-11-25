@echo off
setlocal
if not exist assets\trainer.ico (
    echo assets missing, generating placeholders...
    python tools\make_assets.py || goto :error
)
rustc -O src\main.rs -o REHDTrainer.exe -C strip=symbols || goto :error
echo Built REHDTrainer.exe
exit /b 0

:error
echo Build failed.
exit /b 1
