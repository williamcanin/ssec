#[cfg(target_os = "windows")]
pub(crate) mod windows {
  use std::ffi::OsStr;
  use windows::{
    Win32::{
      Foundation::{CloseHandle, HANDLE, HWND},
      System::Threading::{
        CREATE_NEW_CONSOLE, CREATE_NO_WINDOW, CreateProcessW, PROCESS_INFORMATION, STARTUPINFOW,
        STARTUPINFOW_FLAGS, WaitForSingleObject,
      },
      UI::{
        Shell::{SEE_MASK_NOCLOSEPROCESS, SHELLEXECUTEINFOW, ShellExecuteExW},
        WindowsAndMessaging::{SW_HIDE, SW_SHOW},
      },
    },
    core::{PCWSTR, PWSTR},
  };
  use crate::utils::capitalize;

  fn to_pcw_str(s: &str) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt;
    OsStr::new(s).encode_wide().chain(Some(0)).collect()
  }

  // Implementação alternativa para elevação quando usando CreateProcess
  fn elevate_and_run(
    command: &str,
    wait: bool,
    show_window: bool,
  ) -> Result<(), Box<dyn std::error::Error>> {
    let operation = to_pcw_str("runas");
    let file = to_pcw_str("cmd.exe");
    let params_str = format!(r#"/C "{}""#, command);
    let params = to_pcw_str(&params_str);

    let mut sei = SHELLEXECUTEINFOW {
      cbSize: size_of::<SHELLEXECUTEINFOW>() as u32,
      fMask: SEE_MASK_NOCLOSEPROCESS,
      hwnd: HWND(std::ptr::null_mut()),
      lpVerb: PCWSTR(operation.as_ptr()),
      lpFile: PCWSTR(file.as_ptr()),
      lpParameters: PCWSTR(params.as_ptr()),
      lpDirectory: PCWSTR::null(),
      nShow: if show_window { SW_SHOW.0 } else { SW_HIDE.0 },
      ..Default::default()
    };

    unsafe {
      if !ShellExecuteExW(&mut sei).is_ok() {
        return Err(
          format!(
            "Failed to elevate privileges (err: {})",
            std::io::Error::last_os_error()
          )
            .into(),
        );
      }

      if sei.hProcess == HANDLE(std::ptr::null_mut()) {
        return Err("UAC was canceled by user or process not created.".into());
      }

      if wait {
        WaitForSingleObject(sei.hProcess, u32::MAX);
        CloseHandle(sei.hProcess)?;
      } else {
        CloseHandle(sei.hProcess)?; // Fecha o handle mesmo sem esperar
      }
    }

    Ok(())
  }

  pub(crate) fn service(
    names: &Vec<String>,
    action: &str,
    as_admin: bool,
    wait: bool,
    show_window: bool,
  ) -> Result<(), Box<dyn std::error::Error>> {
    let mut pi = PROCESS_INFORMATION::default();
    let mut si = STARTUPINFOW::default();
    si.cb = size_of::<STARTUPINFOW>() as u32;

    // Configura visibilidade da janela
    if !show_window {
      si.dwFlags |= STARTUPINFOW_FLAGS(0x00000001); // STARTF_USESHOWWINDOW
      si.wShowWindow = SW_HIDE.0 as u16;
    }

    let joined = names.join(",");
    let command_line = format!(
      "powershell -Command \"$names='{}' -split ','; foreach ($n in $names) {{ {}-Service -Name $n }}\"",
      joined, capitalize(action)
    );
    let mut command_wide: Vec<u16> = command_line.encode_utf16().chain(Some(0)).collect();

    unsafe {
      let flags = if show_window {
        CREATE_NEW_CONSOLE
      } else {
        CREATE_NO_WINDOW
      };

      // Se precisar de elevação, vamos usar uma técnica alternativa
      if as_admin {
        return elevate_and_run(&command_line, wait, show_window);
      }

      let success = CreateProcessW(
        PWSTR::null(),
        Option::from(PWSTR(command_wide.as_mut_ptr())),
        None,
        None,
        false,
        flags,
        None,
        None,
        &si,
        &mut pi,
      );

      match success {
        Ok(_) => {
          if wait {
            WaitForSingleObject(pi.hProcess, u32::MAX);
            CloseHandle(pi.hProcess)?;
            CloseHandle(pi.hThread)?;
          }
          Ok(())
        }
        Err(e) => Err(format!("Error run command {} (erro: {})", command_line, e).into()),
      }
    }
  }
}
