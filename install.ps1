#Requires -Version 5.1
$ErrorActionPreference = 'Stop'

# Windows PowerShell 5.1 defaults to TLS 1.0/1.1, which GitHub rejects.
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12

$Repo = 'carlosarraes/mb-cli'
$Binary = 'mb.exe'
$InstallDir = Join-Path $env:LOCALAPPDATA 'Programs\mb'

function Get-Arch {
    switch ($env:PROCESSOR_ARCHITECTURE) {
        'AMD64' { 'x86_64' }
        'ARM64' { 'aarch64' }
        default {
            Write-Error "Unsupported architecture '$env:PROCESSOR_ARCHITECTURE'. Download manually from: https://github.com/$Repo/releases"
        }
    }
}

function Resolve-Version {
    if ($env:MB_VERSION) { return $env:MB_VERSION }

    $headers = @{ 'User-Agent' = 'mb-cli-installer' }
    $release = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest" -Headers $headers
    if (-not $release.tag_name) {
        Write-Error "Could not resolve latest release. Set `$env:MB_VERSION explicitly (e.g. 'v0.1.1')."
    }
    return $release.tag_name
}

function Update-Path {
    param([string]$Dir)

    $userPath = [Environment]::GetEnvironmentVariable('Path', 'User')
    $entries = if ($userPath) { $userPath -split ';' } else { @() }

    if ($entries -notcontains $Dir) {
        $newPath = if ($userPath) { "$userPath;$Dir" } else { $Dir }
        [Environment]::SetEnvironmentVariable('Path', $newPath, 'User')

        # Broadcast WM_SETTINGCHANGE so already-running processes (Explorer,
        # other shells, VS Code) pick up the new PATH without a logoff.
        if (-not ('MbInstall.NativeMethods' -as [type])) {
            Add-Type -Namespace MbInstall -Name NativeMethods -MemberDefinition @'
[System.Runtime.InteropServices.DllImport("user32.dll", SetLastError = true, CharSet = System.Runtime.InteropServices.CharSet.Auto)]
public static extern System.IntPtr SendMessageTimeout(
    System.IntPtr hWnd, uint Msg, System.UIntPtr wParam, string lParam,
    uint fuFlags, uint uTimeout, out System.UIntPtr lpdwResult);
'@
        }
        [UIntPtr]$result = [UIntPtr]::Zero
        [void][MbInstall.NativeMethods]::SendMessageTimeout(
            [IntPtr]0xffff, 0x1A, [UIntPtr]::Zero, 'Environment',
            0x0002, 5000, [ref]$result)

        Write-Host "Added $Dir to user PATH."
    }

    # Update the current session so 'mb' works without reopening the shell.
    if (($env:Path -split ';') -notcontains $Dir) {
        $env:Path = "$env:Path;$Dir"
    }
}

function Main {
    $arch = Get-Arch
    $version = Resolve-Version
    $target = "$arch-pc-windows-msvc"

    Write-Host "Installing mb $version ($target)..."

    $url = "https://github.com/$Repo/releases/download/$version/mb-$target.zip"
    $tmp = New-Item -ItemType Directory -Path (Join-Path $env:TEMP "mb-install-$([guid]::NewGuid())")

    try {
        $zipPath = Join-Path $tmp 'mb.zip'
        Write-Host "Downloading $url..."
        Invoke-WebRequest -Uri $url -OutFile $zipPath -UseBasicParsing

        Expand-Archive -Path $zipPath -DestinationPath $tmp -Force

        New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
        Move-Item -Path (Join-Path $tmp $Binary) -Destination (Join-Path $InstallDir $Binary) -Force

        Write-Host "Installed mb to $InstallDir\$Binary"
        Update-Path -Dir $InstallDir
    }
    finally {
        Remove-Item -Recurse -Force $tmp -ErrorAction SilentlyContinue
    }
}

Main
