//! PowerShell shell integration.

use crate::shell::EnvContext;
use std::path::{Path, PathBuf};

pub fn profile_path() -> Option<PathBuf> {
    dirs::home_dir().map(|h| h.join("Documents").join("PowerShell").join("profile.ps1"))
}

/// Generates the PowerShell initialisation script.
///
/// Wraps `Set-Location` for directory-change detection. The `_gvm_hook`
/// checks `$env:GVM_SHELL_VERSION` first so that session-scoped activations
/// from `gvm shell <version>` are not overridden when the user changes
/// directories.
pub fn env_script(ctx: &EnvContext<'_>) -> String {
    let gvm_dir = ctx.gvm_dir.display().to_string();

    let path_stmt = ctx.active_bin.map_or_else(String::new, |bin| {
        format!(
            "$env:PATH = \"{bin};\" + \
             ((($env:PATH -split \";\") | Where-Object {{ $_ -ne \"\" -and $_ -notlike \"$env:GVM_DIR\\versions\\*\\bin\" }}) -join \";\")\n",
            bin = bin.display(),
        )
    });

    let goroot_stmt = ctx.active_root.map_or_else(String::new, |root| {
        format!("$env:GOROOT = \"{}\"\n", root.display())
    });

    format!(
        r#"$env:GVM_DIR = "{gvm_dir}"
{goroot_stmt}{path_stmt}
if (-not (Get-Command _gvm_hook -ErrorAction SilentlyContinue)) {{
    function _gvm_hook {{
        if ($env:GVM_SHELL_VERSION) {{ return }}
        $p = & gvm path 2>$null
        if ($p -and $p -ne "") {{
            $goroot = Split-Path $p -Parent
            $env:GOROOT = $goroot
            $env:PATH = "$p;" + (
                (($env:PATH -split ";") |
                Where-Object {{ $_ -ne "" -and $_ -notlike "$env:GVM_DIR\versions\*\bin" }}) -join ";"
            )
        }}
    }}
    function Set-Location {{
        Microsoft.PowerShell.Management\Set-Location @args
        _gvm_hook
    }}
    Set-Alias -Name cd -Value Set-Location -Force -Option AllScope
}}"#,
        gvm_dir = gvm_dir,
        goroot_stmt = goroot_stmt,
        path_stmt = path_stmt,
    )
}

pub fn wrapper_function() -> &'static str {
    r#"function gvm {
    $gvmBin = (Get-Command gvm -CommandType Application -ErrorAction SilentlyContinue | Select-Object -First 1).Source
    if (-not $gvmBin) { Write-Error 'gvm binary not found in PATH'; return }
    if ($args.Count -gt 0 -and $args[0] -eq 'shell') {
        $script = & $gvmBin @args
        $script:_gvmExit = $LASTEXITCODE
        if ($script:_gvmExit -eq 0 -and $script) {
            $script | Out-String | Invoke-Expression
        }
    } else {
        & $gvmBin @args
        $script:_gvmExit = $LASTEXITCODE
        if ($args.Count -gt 0 -and $args[0] -in @('use', 'default', 'local')) {
            & $gvmBin env --shell powershell 2>$null | Out-String | Invoke-Expression
        }
    }
    $global:LASTEXITCODE = $script:_gvmExit
}"#
}

pub fn shell_version_script(tag: &str, bin: &Path, root: &Path) -> String {
    format!(
        r#"$env:GVM_SHELL_VERSION = "{tag}"
$env:GOROOT = "{root}"
$env:PATH = "{bin};" + ((($env:PATH -split ";") | Where-Object {{ $_ -ne "" -and $_ -notlike "$env:GVM_DIR\versions\*\bin" }}) -join ";")
"#,
        tag = tag,
        root = root.display(),
        bin = bin.display(),
    )
}

pub fn shell_unset_script() -> &'static str {
    "Remove-Item env:GVM_SHELL_VERSION -ErrorAction SilentlyContinue\n_gvm_hook 2>$null\n"
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shell::EnvContext;
    use std::path::Path;

    #[test]
    fn hook_checks_gvm_shell_version() {
        let ctx = EnvContext {
            gvm_dir: Path::new(r"C:\Users\user\.gvm"),
            active_bin: None,
            active_root: None,
        };
        let script = env_script(&ctx);
        assert!(script.contains("GVM_SHELL_VERSION"));
    }

    #[test]
    fn wrapper_handles_shell_subcommand() {
        let w = wrapper_function();
        assert!(w.contains("'shell'") || w.contains("\"shell\"") || w.contains("-eq 'shell'"));
        assert!(w.contains("Invoke-Expression"));
    }

    #[test]
    fn shell_version_script_uses_ps_syntax() {
        let s = shell_version_script(
            "go1.22.4",
            Path::new(r"C:\Users\user\.gvm\versions\go1.22.4\bin"),
            Path::new(r"C:\Users\user\.gvm\versions\go1.22.4"),
        );
        assert!(s.contains("$env:GVM_SHELL_VERSION"));
        assert!(s.contains("go1.22.4"));
        assert!(s.contains("$env:GOROOT"));
        assert!(s.contains("$env:PATH"));
    }

    #[test]
    fn shell_unset_script_removes_env_var() {
        let s = shell_unset_script();
        assert!(s.contains("GVM_SHELL_VERSION"));
        assert!(s.contains("_gvm_hook"));
    }
}
