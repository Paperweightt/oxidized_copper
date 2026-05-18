set shell := ["powershell.exe", "-c"]

install:
    cargo install --path crates/cli
    $cfgPath = "$Env:USERPROFILE\.cargo\bin"
    $userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
    if ($userPath -notlike ("*" + $cfgPath + "*")) {\
    [Environment]::SetEnvironmentVariable("PATH", ("$userPath;$cfgPath"), "User");\
    Write-Host "Added Cargo bin to User PATH." -ForegroundColor Cyan;\
    } else { \
    Write-Host "Cargo bin is already in your PATH." -ForegroundColor Yellow; \
    }

test-all:
  cargo test -- --nocapture 

test TEST:
  cargo test -p {{TEST}} -- --nocapture 
