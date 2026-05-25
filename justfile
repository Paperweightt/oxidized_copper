set shell := ["powershell.exe", "-NoProfile", "/c"]
inno :=  "C:\\Users\\henry\\AppData\\Local\\Programs\\Inno Setup 6\\ISCC.exe"

default:
    @just --list

build:
    cargo install --path crates/cli

install: build
    $cfgPath = "$Env:USERPROFILE\.cargo\bin"
    $userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
    if ($userPath -notlike ("*" + $cfgPath + "*")) {\
    [Environment]::SetEnvironmentVariable("PATH", ("$userPath;$cfgPath"), "User");\
    Write-Host "Added Cargo bin to User PATH." -ForegroundColor Cyan;\
    } else { \
    Write-Host "Cargo bin is already in your PATH." -ForegroundColor Yellow; \
    }

build-inno: build
    & "{{inno}}" .\installer\setup.iss

local-deploy-inno: build-inno
    .\installer\dist\copper-installer.exe

test-all:
  cargo test -- --nocapture 

test TEST:
  cargo test -p {{TEST}} -- --nocapture 
