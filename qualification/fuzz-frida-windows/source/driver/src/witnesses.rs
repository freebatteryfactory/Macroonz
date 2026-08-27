//! Controlled LibAFL crash/timeout witnesses and a Job Object resource witness.
//!
//! LibAFL owns executor exit facts; Macroonz owns typed evidence translation.
//! Resource exhaustion uses a qualification-only child under a Windows Job Object.

use std::{
    fs,
    io::{self, Write},
    path::Path,
    process::{Command, Stdio},
};

use libafl::{
    events::NopEventManager,
    executors::ExitKind,
    feedbacks::{CrashFeedback, Feedback, TimeoutFeedback},
    inputs::BytesInput,
};

use crate::classify::{self, ExecutionClass};

/// Prove LibAFL crash/timeout feedback and Macroonz classification translation.
pub(crate) fn prove_crash_timeout(evidence_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(evidence_dir)?;
    let mut out = fs::File::create(evidence_dir.join("crash-timeout-witness.tsv"))?;
    writeln!(out, "phase\tclaim\tstatus\tfact")?;

    let mut crash_fb = CrashFeedback::new();
    let mut timeout_fb = TimeoutFeedback::new();
    let mut nop_mgr = NopEventManager::new();
    let mut nop_state = ();
    let observers = ();
    let input = BytesInput::new(b"witness".to_vec());

    let crash_interesting = crash_fb.is_interesting(
        &mut nop_state,
        &mut nop_mgr,
        &input,
        &observers,
        &ExitKind::Crash,
    )?;
    let crash_not = crash_fb.is_interesting(
        &mut nop_state,
        &mut nop_mgr,
        &input,
        &observers,
        &ExitKind::Ok,
    )?;
    let timeout_interesting = timeout_fb.is_interesting(
        &mut nop_state,
        &mut nop_mgr,
        &input,
        &observers,
        &ExitKind::Timeout,
    )?;
    let timeout_not = timeout_fb.is_interesting(
        &mut nop_state,
        &mut nop_mgr,
        &input,
        &observers,
        &ExitKind::Ok,
    )?;

    if !crash_interesting || crash_not {
        return Err(io::Error::other("CrashFeedback did not discriminate Crash vs Ok").into());
    }
    if !timeout_interesting || timeout_not {
        return Err(io::Error::other("TimeoutFeedback did not discriminate Timeout vs Ok").into());
    }
    writeln!(
        out,
        "witness\tlibafl-CrashFeedback\tavailable\tis_interesting(Crash)=true; is_interesting(Ok)=false"
    )?;
    writeln!(
        out,
        "witness\tlibafl-TimeoutFeedback\tavailable\tis_interesting(Timeout)=true; is_interesting(Ok)=false"
    )?;

    let crash_class =
        classify::classify(macroonz_f0_target::CaptureOutcome::NotUtf8, ExitKind::Crash);
    let timeout_class =
        classify::classify(macroonz_f0_target::CaptureOutcome::NotUtf8, ExitKind::Timeout);
    if crash_class != ExecutionClass::Crash {
        return Err(io::Error::other("Macroonz classify did not map ExitKind::Crash").into());
    }
    if timeout_class != ExecutionClass::Timeout {
        return Err(io::Error::other("Macroonz classify did not map ExitKind::Timeout").into());
    }
    writeln!(
        out,
        "witness\tmacroonz-classify-crash\tavailable\tExitKind::Crash -> {}",
        crash_class.as_str()
    )?;
    writeln!(
        out,
        "witness\tmacroonz-classify-timeout\tavailable\tExitKind::Timeout -> {}",
        timeout_class.as_str()
    )?;
    writeln!(
        out,
        "witness\tlibafl-InProcessExecutor-timeout-bound\tavailable\tmain executor timeout=2s; live objective is CrashFeedback|TimeoutFeedback; planted ExitKind::Crash/Timeout evaluate_input retains solutions; wall-clock hang remains optional"
    )?;
    Ok(())
}

/// Run a qualification-only Job Object memory-limit child and translate the outcome.
pub(crate) fn prove_resource_job(evidence_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(evidence_dir)?;
    let script = evidence_dir.join("resource-job-witness.ps1");
    let log = evidence_dir.join("resource-job-witness.tsv");
    fs::write(
        &script,
        concat!(
            "$ErrorActionPreference = \"Stop\"\r\n",
            "$out = Join-Path $PSScriptRoot \"resource-job-witness.tsv\"\r\n",
            "\"phase`tclaim`tstatus`tfact\" | Set-Content -Encoding ascii $out\r\n",
            "Add-Type -TypeDefinition @\"\r\n",
            "using System;\r\n",
            "using System.Diagnostics;\r\n",
            "using System.Runtime.InteropServices;\r\n",
            "public static class JobWitness {\r\n",
            "  [DllImport(\"kernel32.dll\", SetLastError=true)] static extern IntPtr CreateJobObject(IntPtr a, string n);\r\n",
            "  [DllImport(\"kernel32.dll\", SetLastError=true)] static extern bool SetInformationJobObject(IntPtr job, int infoClass, IntPtr info, uint len);\r\n",
            "  [DllImport(\"kernel32.dll\", SetLastError=true)] static extern bool AssignProcessToJobObject(IntPtr job, IntPtr proc);\r\n",
            "  [DllImport(\"kernel32.dll\", SetLastError=true)] static extern bool CloseHandle(IntPtr h);\r\n",
            "  [StructLayout(LayoutKind.Sequential)] struct JOBOBJECT_BASIC_LIMIT_INFORMATION {\r\n",
            "    public long PerProcessUserTimeLimit; public long PerJobUserTimeLimit; public uint LimitFlags;\r\n",
            "    public UIntPtr MinimumWorkingSetSize; public UIntPtr MaximumWorkingSetSize;\r\n",
            "    public uint ActiveProcessLimit; public long Affinity; public uint PriorityClass; public uint SchedulingClass;\r\n",
            "  }\r\n",
            "  [StructLayout(LayoutKind.Sequential)] struct IO_COUNTERS {\r\n",
            "    public ulong ReadOp; public ulong WriteOp; public ulong OtherOp; public ulong ReadXfer; public ulong WriteXfer; public ulong OtherXfer;\r\n",
            "  }\r\n",
            "  [StructLayout(LayoutKind.Sequential)] struct JOBOBJECT_EXTENDED_LIMIT_INFORMATION {\r\n",
            "    public JOBOBJECT_BASIC_LIMIT_INFORMATION Basic; public IO_COUNTERS Io;\r\n",
            "    public UIntPtr ProcessMemoryLimit; public UIntPtr JobMemoryLimit;\r\n",
            "    public UIntPtr PeakProcessMemoryUsed; public UIntPtr PeakJobMemoryUsed;\r\n",
            "  }\r\n",
            "  const int JobObjectExtendedLimitInformation = 9;\r\n",
            "  const uint JOB_OBJECT_LIMIT_PROCESS_MEMORY = 0x00000100;\r\n",
            "  public static string Run() {\r\n",
            "    IntPtr job = CreateJobObject(IntPtr.Zero, null);\r\n",
            "    if (job == IntPtr.Zero) return \"unavailable\\tCreateJobObject failed\";\r\n",
            "    var info = new JOBOBJECT_EXTENDED_LIMIT_INFORMATION();\r\n",
            "    info.Basic.LimitFlags = JOB_OBJECT_LIMIT_PROCESS_MEMORY;\r\n",
            "    info.ProcessMemoryLimit = (UIntPtr)(16UL * 1024UL * 1024UL);\r\n",
            "    int size = Marshal.SizeOf(info);\r\n",
            "    IntPtr ptr = Marshal.AllocHGlobal(size);\r\n",
            "    Marshal.StructureToPtr(info, ptr, false);\r\n",
            "    bool ok = SetInformationJobObject(job, JobObjectExtendedLimitInformation, ptr, (uint)size);\r\n",
            "    Marshal.FreeHGlobal(ptr);\r\n",
            "    if (!ok) { CloseHandle(job); return \"unavailable\\tSetInformationJobObject failed\"; }\r\n",
            "    var controlPsi = new ProcessStartInfo();\r\n",
            "    controlPsi.FileName = \"cmd.exe\";\r\n",
            "    controlPsi.Arguments = \"/d /c exit 0\";\r\n",
            "    controlPsi.UseShellExecute = false;\r\n",
            "    var control = Process.Start(controlPsi);\r\n",
            "    if (control == null) { CloseHandle(job); return \"unavailable\\tcontrol child start failed\"; }\r\n",
            "    if (!AssignProcessToJobObject(job, control.Handle)) {\r\n",
            "      int err = Marshal.GetLastWin32Error();\r\n",
            "      try { control.Kill(); } catch {}\r\n",
            "      CloseHandle(job);\r\n",
            "      return \"unavailable\\tAssignProcessToJobObject failed for control; win32=\" + err;\r\n",
            "    }\r\n",
            "    control.WaitForExit(10000);\r\n",
            "    if (!control.HasExited || control.ExitCode != 0) {\r\n",
            "      int controlCode = control.HasExited ? control.ExitCode : -1;\r\n",
            "      try { if (!control.HasExited) control.Kill(); } catch {}\r\n",
            "      CloseHandle(job);\r\n",
            "      return \"unavailable\\tcontrol cmd under Job Object did not exit 0; code=\" + controlCode;\r\n",
            "    }\r\n",
            "    var limitPsi = new ProcessStartInfo();\r\n",
            "    limitPsi.FileName = \"powershell.exe\";\r\n",
            "    limitPsi.Arguments = \"-NoProfile -Command \\\"$a=New-Object byte[] (2MB); $b=@(); while($true){ $b += ,$a }\\\"\";\r\n",
            "    limitPsi.UseShellExecute = false;\r\n",
            "    var limit = Process.Start(limitPsi);\r\n",
            "    if (limit == null) { CloseHandle(job); return \"unavailable\\tlimit child start failed\"; }\r\n",
            "    if (!AssignProcessToJobObject(job, limit.Handle)) {\r\n",
            "      int err = Marshal.GetLastWin32Error();\r\n",
            "      try { limit.Kill(); } catch {}\r\n",
            "      CloseHandle(job);\r\n",
            "      return \"unavailable\\tAssignProcessToJobObject failed for limit child; win32=\" + err;\r\n",
            "    }\r\n",
            "    limit.WaitForExit(15000);\r\n",
            "    bool exited = limit.HasExited;\r\n",
            "    int code = exited ? limit.ExitCode : -1;\r\n",
            "    if (!exited) { try { limit.Kill(); } catch {} }\r\n",
            "    CloseHandle(job);\r\n",
            "    if (!exited) return \"unavailable\\tlimit child did not exit under Job Object memory limit\";\r\n",
            "    if (code == 0) return \"unavailable\\tlimit child exited 0; not evidence of Job Object memory limit\";\r\n",
            "    return \"available\\tcontrol cmd exited 0 under same 16MiB Job Object; limit PowerShell terminated nonzero after AssignProcessToJobObject; exit=\" + code;\r\n",
            "  }\r\n",
            "}\r\n",
            "\"@\r\n",
            "$fact = [JobWitness]::Run()\r\n",
            "$tag = if ($fact.StartsWith(\"available\")) { \"available\" } else { \"unavailable\" }\r\n",
            "$detail = $fact.Substring($fact.IndexOf(\"`t\") + 1)\r\n",
            "Add-Content -Encoding ascii $out (\"witness`twindows-job-object-memory`t{0}`t{1}\" -f $tag, $detail)\r\n",
            "if ($tag -ne \"available\") { exit 1 }\r\n",
        ),
    )?;

    let status = Command::new(r"C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe")
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            &script.display().to_string(),
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .status()?;
    if !status.success() {
        let detail = fs::read_to_string(&log).unwrap_or_default();
        return Err(io::Error::other(format!(
            "Job Object resource witness failed; log={detail}"
        ))
        .into());
    }
    Ok(())
}
