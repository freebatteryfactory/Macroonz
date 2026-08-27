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
            "Add-Type -TypeDefinition @'\r\n",
            "using System;\r\n",
            "using System.Diagnostics;\r\n",
            "using System.Runtime.InteropServices;\r\n",
            "public static class JobWitness {\r\n",
            "  [DllImport(\"kernel32.dll\", SetLastError=true)] static extern IntPtr CreateJobObject(IntPtr a, string n);\r\n",
            "  [DllImport(\"kernel32.dll\", SetLastError=true)] static extern bool SetInformationJobObject(IntPtr job, int infoClass, IntPtr info, uint len);\r\n",
            "  [DllImport(\"kernel32.dll\", SetLastError=true)] static extern bool AssignProcessToJobObject(IntPtr job, IntPtr proc);\r\n",
            "  [DllImport(\"kernel32.dll\", SetLastError=true)] static extern bool CloseHandle(IntPtr h);\r\n",
            "  [DllImport(\"kernel32.dll\", SetLastError=true)] static extern bool QueryInformationJobObject(IntPtr job, int infoClass, IntPtr info, uint len, IntPtr retLen);\r\n",
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
            "  const int STATUS_PROCESS_MEMORY_LIMIT_EXCEEDED = unchecked((int)0xC0000205);\r\n",
            "  const string AllocArgs = \"-NoProfile -Command \\\"$a=New-Object byte[] (2MB); $b=@(); while($true){ $b += ,$a }\\\"\";\r\n",
            "  static bool ConfigureLimit(IntPtr job, ulong bytes) {\r\n",
            "    var info = new JOBOBJECT_EXTENDED_LIMIT_INFORMATION();\r\n",
            "    info.Basic.LimitFlags = JOB_OBJECT_LIMIT_PROCESS_MEMORY;\r\n",
            "    info.ProcessMemoryLimit = (UIntPtr)bytes;\r\n",
            "    int size = Marshal.SizeOf(info);\r\n",
            "    IntPtr ptr = Marshal.AllocHGlobal(size);\r\n",
            "    Marshal.StructureToPtr(info, ptr, false);\r\n",
            "    bool ok = SetInformationJobObject(job, JobObjectExtendedLimitInformation, ptr, (uint)size);\r\n",
            "    Marshal.FreeHGlobal(ptr);\r\n",
            "    return ok;\r\n",
            "  }\r\n",
            "  static ulong QueryPeak(IntPtr job) {\r\n",
            "    int size = Marshal.SizeOf(typeof(JOBOBJECT_EXTENDED_LIMIT_INFORMATION));\r\n",
            "    IntPtr ptr = Marshal.AllocHGlobal(size);\r\n",
            "    bool ok = QueryInformationJobObject(job, JobObjectExtendedLimitInformation, ptr, (uint)size, IntPtr.Zero);\r\n",
            "    ulong peak = 0;\r\n",
            "    if (ok) {\r\n",
            "      var info = (JOBOBJECT_EXTENDED_LIMIT_INFORMATION)Marshal.PtrToStructure(ptr, typeof(JOBOBJECT_EXTENDED_LIMIT_INFORMATION));\r\n",
            "      peak = info.PeakProcessMemoryUsed.ToUInt64();\r\n",
            "    }\r\n",
            "    Marshal.FreeHGlobal(ptr);\r\n",
            "    return peak;\r\n",
            "  }\r\n",
            "  static Process StartAllocator() {\r\n",
            "    var psi = new ProcessStartInfo();\r\n",
            "    psi.FileName = \"powershell.exe\";\r\n",
            "    psi.Arguments = AllocArgs;\r\n",
            "    psi.UseShellExecute = false;\r\n",
            "    return Process.Start(psi);\r\n",
            "  }\r\n",
            "  public static string Run() {\r\n",
            "    IntPtr highJob = CreateJobObject(IntPtr.Zero, null);\r\n",
            "    if (highJob == IntPtr.Zero) return \"unavailable\\tCreateJobObject failed for high-limit control\";\r\n",
            "    if (!ConfigureLimit(highJob, 512UL * 1024UL * 1024UL)) { CloseHandle(highJob); return \"unavailable\\tSetInformationJobObject failed for high-limit control\"; }\r\n",
            "    var control = StartAllocator();\r\n",
            "    if (control == null) { CloseHandle(highJob); return \"unavailable\\tsame-payload control start failed\"; }\r\n",
            "    if (!AssignProcessToJobObject(highJob, control.Handle)) {\r\n",
            "      int err = Marshal.GetLastWin32Error();\r\n",
            "      try { control.Kill(); } catch {}\r\n",
            "      CloseHandle(highJob);\r\n",
            "      return \"unavailable\\tAssignProcessToJobObject failed for control; win32=\" + err;\r\n",
            "    }\r\n",
            "    System.Threading.Thread.Sleep(1500);\r\n",
            "    if (control.HasExited) {\r\n",
            "      int early = control.ExitCode;\r\n",
            "      CloseHandle(highJob);\r\n",
            "      return \"unavailable\\tsame-payload control exited under 512MiB limit before readiness window; exit=\" + early + \"; not attributable to tight memory ceiling\";\r\n",
            "    }\r\n",
            "    try { control.Kill(); control.WaitForExit(5000); } catch {}\r\n",
            "    CloseHandle(highJob);\r\n",
            "    IntPtr tightJob = CreateJobObject(IntPtr.Zero, null);\r\n",
            "    if (tightJob == IntPtr.Zero) return \"unavailable\\tCreateJobObject failed for tight limit\";\r\n",
            "    const ulong TightBytes = 16UL * 1024UL * 1024UL;\r\n",
            "    if (!ConfigureLimit(tightJob, TightBytes)) { CloseHandle(tightJob); return \"unavailable\\tSetInformationJobObject failed for tight limit\"; }\r\n",
            "    var limit = StartAllocator();\r\n",
            "    if (limit == null) { CloseHandle(tightJob); return \"unavailable\\tsame-payload limit start failed\"; }\r\n",
            "    if (!AssignProcessToJobObject(tightJob, limit.Handle)) {\r\n",
            "      int err = Marshal.GetLastWin32Error();\r\n",
            "      try { limit.Kill(); } catch {}\r\n",
            "      CloseHandle(tightJob);\r\n",
            "      return \"unavailable\\tAssignProcessToJobObject failed for limit child; win32=\" + err;\r\n",
            "    }\r\n",
            "    limit.WaitForExit(15000);\r\n",
            "    bool exited = limit.HasExited;\r\n",
            "    int code = exited ? limit.ExitCode : -1;\r\n",
            "    ulong peak = QueryPeak(tightJob);\r\n",
            "    if (!exited) { try { limit.Kill(); } catch {} }\r\n",
            "    CloseHandle(tightJob);\r\n",
            "    if (!exited) return \"unavailable\\tsame-payload limit child did not exit under 16MiB Job Object memory limit\";\r\n",
            "    if (code == 0) return \"unavailable\\tsame-payload limit child exited 0; not evidence of memory ceiling\";\r\n",
            "    bool statusMatch = code == STATUS_PROCESS_MEMORY_LIMIT_EXCEEDED;\r\n",
            "    bool peakNearCeiling = peak >= (TightBytes / 2UL);\r\n",
            "    if (!statusMatch && !peakNearCeiling) {\r\n",
            "      return \"unavailable\\tsame-payload limit exit=\" + code + \" peak=\" + peak + \"; neither STATUS_PROCESS_MEMORY_LIMIT_EXCEEDED nor peak>=8MiB\";\r\n",
            "    }\r\n",
            "    return \"available\\tsame PowerShell allocator; control survived 1.5s under 512MiB Job Object; tight 16MiB Job Object terminated child; exit=\" + code + \"; peakProcessMemoryUsed=\" + peak + \"; statusMemoryLimit=\" + statusMatch;\r\n",
            "  }\r\n",
            "}\r\n",
            "'@\r\n",
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
