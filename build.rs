fn main() {
    // Компиляция UI-файла Slint во время сборки проекта.
    // Это создаст сгенерированный модуль ui.rs в папке target.
    slint_build::compile("src/ui.slint").unwrap();

    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("MyALL_icon.ico");
        res.compile().unwrap();
    }
}
