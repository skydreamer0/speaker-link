# Speaker Link

在 Windows 桌面查看音響電量與連線，讓腳本和桌面 App 用同一套控制核心。

**目前進入 P0 契約準備，尚無可執行版本、可用藍牙介面或已驗證的 SoundLink Flex 支援。** [基礎 workspace](docs/FOUNDATION.md) 只有未發行的型別、探索介面與合成測試，不代表實機功能已完成。

[English](README.md) · [正式規格](docs/SPEC.md) · [開發順序](docs/DEVELOPMENT.md) · [相容性紀錄](docs/COMPATIBILITY.md)

首版只做四項功能：電量、音響端配對及連線清單、已驗證的連線與中斷連線、可預覽且去識別的本地診斷匯出。找不到資料時會明確顯示未知，不會以零電量或電腦自己的裝置清單代替。

先用 Rust 做套件和 Windows 藍牙介面，再做 CLI，最後做原生 Windows App。三者共用指令、錯誤和能力判斷，讓一般使用者與開發者得到一致結果。

第一個研究對象是 Bose SoundLink Flex 第一代 435910。硬體尚未驗證。內建 EQ、刷機、任意封包及實體按鍵修改不在首版範圍。沒有帳號、雲端服務或自動上傳報告。

目前沒有安裝指令或 crates.io 發行。想參與可先閱讀 [貢獻規範](CONTRIBUTING.md)，從 [Issues](https://github.com/skydreamer0/speaker-link/issues) 提交已去識別的相容性資訊。請勿公開 MAC、序號、原始藍牙擷取或私人裝置名稱。

原創程式與文件採 [MIT](LICENSE)。本專案與 Bose、Bluetooth SIG 沒有隸屬或背書關係；商標屬各自權利人。第三方原始碼另依其授權處理，不能把 GPL 程式翻寫成 Rust 就視為 MIT。
