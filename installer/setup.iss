#define MyAppName "copper.rs"
#define MyAppVersion "0.1.0"
#define MyAppPublisher "paperweightt"
#define MyAppExeName "copper.exe"

[Setup]
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
DefaultDirName={autopf}\copper
DefaultGroupName={#MyAppName}
OutputDir=dist
OutputBaseFilename=copper-installer
Compression=lzma
SolidCompression=yes

[Files]
Source: "../target/release/cu.exe"; DestDir: "{app}"; Flags: ignoreversion

[Tasks]
Name: "addtopath"; Description: "Add to PATH"; GroupDescription: "Additional tasks:";

[Code]
procedure EnvAddPath(Path: string);
var
    Paths: string;
begin
    if not RegQueryStringValue(HKEY_CURRENT_USER,
        'Environment',
        'Path',
        Paths)
    then
        Paths := '';

    if Pos(';' + Uppercase(Path) + ';', ';' + Uppercase(Paths) + ';') = 0 then
    begin
        Paths := Paths + ';' + Path;

        RegWriteStringValue(HKEY_CURRENT_USER,
            'Environment',
            'Path',
            Paths);
    end;
end;

procedure CurStepChanged(CurStep: TSetupStep);
begin
    if CurStep = ssPostInstall then
    begin
        if WizardIsTaskSelected('addtopath') then
        begin
            EnvAddPath(ExpandConstant('{app}'));
        end;
    end;
end;
