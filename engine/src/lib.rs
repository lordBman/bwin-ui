use windows::{
    Win32::{Foundation::*, Graphics::Gdi::*, System::LibraryLoader::GetModuleHandleA, UI::WindowsAndMessaging::*}, core::*
};

struct AppData {
    font: HFONT,
}

static mut APP_DATA: Option<*mut AppData> = None;

fn text(hwnd: HWND) -> LRESULT{
    // Create a larger font
    let font = unsafe {
        CreateFontA(
            20, // Height
            0, 0, 0,
            600,
            0, 0, 0,
            ANSI_CHARSET,
            OUT_DEFAULT_PRECIS,
            CLIP_DEFAULT_PRECIS,
            CLEARTYPE_QUALITY,
            FF_DONTCARE.0.into(),
            s!("Segoe UI"),
        )
    };

    // Store app data
    let app_data = Box::new(AppData { font });
    let app_data_ptr = Box::into_raw(app_data);
    unsafe {
        APP_DATA = Some(app_data_ptr);
        SetWindowLongPtrA(hwnd, GWLP_USERDATA, app_data_ptr as isize);
    };
    
    let text_hwnd = unsafe {
        CreateWindowExA(
            WINDOW_EX_STYLE(0),
            s!("STATIC"),  // Class name for static control
            s!("Hello, Windows API with Rust!"),  // Text
            WS_VISIBLE | WS_CHILD,
            50,  // x position
            50,  // y position
            30, // width
            30,  // height
            Some(hwnd),
            Some(HMENU(1 as _)),  // Control ID
            None,
            None,
        )
    };

    if text_hwnd.is_err() {
        unsafe { MessageBoxA(
            None,
            s!("Failed to create controls"),
            s!("Error"),
            MB_OK | MB_ICONERROR,
        ) };
        return LRESULT(1);
    }else{
        let hwnd = text_hwnd.unwrap();
        unsafe{
            SendMessageA(hwnd, WM_SETFONT, WPARAM(font.0 as usize), LPARAM(1));
        }
        println!("button was created successfull: {:?}", hwnd);
    }
    LRESULT(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn start(title: *const std::os::raw::c_char, width: i32, height: i32) {
    let width_option = if width == -1 { CW_USEDEFAULT } else { width };
    let height_option = if height == -1 { CW_USEDEFAULT } else { height };

    let _ = window( title, width_option, height_option);
    /*if !title.is_null() {
        unsafe {
            let _ = Box::from_raw(title as *mut Result<()>);
        }
    }*/
}

fn window(title: *const std::os::raw::c_char, width: i32, height: i32) -> Result<()> {
    let title_pcstr = PCSTR::from_raw(title as _);

    unsafe {
        let instance = GetModuleHandleA(None)?;
        let window_class = s!("window");

        let wc = WNDCLASSA {
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hInstance: instance.into(),
            lpszClassName: window_class,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            //hbrBackground: HBRUSH(1 as _),
            ..Default::default()
        };

        let atom = RegisterClassA(&wc);
        debug_assert!(atom != 0);

        CreateWindowExA(
            WINDOW_EX_STYLE::default(),
            window_class,
            title_pcstr,
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            width,
            height,
            None,
            None,
            None,
            None,
        )?;

        let mut message = MSG::default();

        while GetMessageA(&mut message, None, 0, 0).into() {
            DispatchMessageA(&message);
        }

        Ok(())
    }
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_DESTROY => {
                println!("WM_DESTROY");
                PostQuitMessage(0);
                LRESULT(0)
            },
            WM_CREATE =>{
                text(window)
            },
            WM_CTLCOLORSTATIC => {
                let hdc = HDC(wparam.0 as _);
                let hwnd_ctl = HWND(lparam.0 as _);
                
                unsafe {
                    // Only make our text label transparent
                    if GetDlgCtrlID(hwnd_ctl) == 1001 {
                        SetBkMode(hdc, TRANSPARENT);
                        SetTextColor(hdc, COLORREF(RGB(0, 0, 139))); // Dark blue
                        
                        // Get app data to use the font
                        if let Some(app_data_ptr) = APP_DATA {
                            let app_data = *app_data_ptr;
                            SelectObject(hdc, HGDIOBJ(app_data.font.0));
                        }
                        
                        return LRESULT(GetStockObject(NULL_BRUSH).0 as isize);
                    }
                }
                LRESULT(0)
            }
            
            WM_CTLCOLORBTN => {
                // Optional: Style button background
                let hdc = HDC(wparam.0 as _);
                SetBkColor(hdc, COLORREF(RGB(240, 240, 240)));
                return LRESULT(GetSysColorBrush(COLOR_BTNFACE).0 as isize);
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}