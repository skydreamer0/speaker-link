# Speaker Link

開發中的本地 Rust 工具，目標是在 Windows 查看 Bose 音響的電量與連線。

Speaker Link 規劃由同一套核心提供 Rust 套件、可供腳本呼叫的 CLI，以及原生 Windows App。對象包括一般 Windows 使用者、開發者，以及願意協助驗證相容性的裝置持有人。

**目前沒有可安裝版本、已發布 crate、可用 CLI，或經驗證的音響控制支援。** 請依照目的選擇下方入口。

[English](README.md) · [快速開始](docs/QUICKSTART.md) · [相容性](docs/COMPATIBILITY.md) · [疑難排解](docs/TROUBLESHOOTING.md)

## 從這裡開始

| 我想要… | 閱讀位置 | 目前能做什麼 |
| --- | --- | --- |
| 安裝 Windows App 或 CLI | [安裝狀態](docs/QUICKSTART.md#install-the-app-or-cli) | 尚未提供安裝程式或 `cargo install` 指令 |
| 閱讀或檢查 Rust 基礎程式 | [原始碼與測試教學](docs/QUICKSTART.md#check-the-foundation-from-source) | 使用固定開發版本執行不需裝置的測試 |
| 確認音響能否使用 | [相容性與證據](docs/COMPATIBILITY.md) | 查看逐項功能狀態，目前沒有已驗證的硬體組合 |
| 參與開發或回報問題 | [貢獻規範](CONTRIBUTING.md)與[疑難排解](docs/TROUBLESHOOTING.md) | 文件、程式審查與去識別回報 |

## 公開進度

核對日期：2026-10-25。規格提案、測試通過與正式發布是不同階段。

- [PR #7](https://github.com/skydreamer0/speaker-link/pull/7)已於 2026-10-25 合併至 `main`，合併 commit 為 [`709bffe2`](https://github.com/skydreamer0/speaker-link/commit/709bffe2718fbd07fb1544d83cf3e7d774630a99)。儲存庫現在包含未發布的 Rust 契約與合成測試。CLI 與 App 仍只是 library 佔位套件，探索介面會直接回傳 `Unsupported`，不會連絡裝置。
- [`2782c396`](https://github.com/skydreamer0/speaker-link/tree/2782c396f1542f3421565e36ff4d8212c14eacaf)是歷史上的純文件基準。原始碼教學固定使用已納入 `main` 的 foundation 版本 [`382294f4`](https://github.com/skydreamer0/speaker-link/tree/382294f407e5b6ef4dd8c3512e0c4f1b2c480b9d)。
- [該固定版本的 CI](https://github.com/skydreamer0/speaker-link/actions/runs/40165335679)已通過 Ubuntu 與 Windows 檢查。這是 `382294f4` 的證據，不是合併 commit 或後續文件修改的 CI 結果；範圍是程式與合成契約，不含 Windows 藍牙存取或音響功能。
- 真正的 Windows 探索與實機驗收由 [issue #2](https://github.com/skydreamer0/speaker-link/issues/2)追蹤。[相容性紀錄](docs/COMPATIBILITY.md)說明各類證據能支持哪些結論。

原始碼教學刻意固定在上述 foundation commit，不涵蓋尚未審查的本地修改。foundation 合併不代表已發布可用產品。

## 規劃中的功能

| 功能 | 預期結果 | 宣稱支援前需要的證據 |
| --- | --- | --- |
| 電量 | 電量、資料來源、觀察時間與新鮮度 | 與獨立的裝置讀值比較 |
| 音響連線清單 | 分別呈現已配對、已連線與正在播放的狀態 | 確認來自音響端，不能用電腦的配對清單代替 |
| 連線控制 | 對明確選定的已知裝置連線或中斷連線 | 經測試的能力設定，以及操作後重新讀取確認 |
| 私人診斷 | 預覽並在本機儲存允許欄位的報告 | 去識別與預覽、匯出內容一致性檢查 |

第一個研究對象是 SoundLink Flex 第一代，型號 435910。它是起始樣本，不代表產品只能用在某一台電腦、某款藍牙介面或特定韌體。每種型號、韌體與主機組合都需要各自的驗證。標準藍牙探索成功，不代表已能控制音響端的功能。

缺少資料時必須明確顯示未知。未知韌體在評估前維持唯讀，不支援的操作不能送出指令。首版不包含內建 EQ、刷機、任意封包、刪除配對歷史、修改實體按鍵，或完整取代 Bose App。

## 設計與文件

先完成 Rust 核心與 Windows 介面，再做 CLI，最後做 App。三者共用能力判斷、狀態、指令與錯誤，不各自實作控制邏輯。

- [正式規格](docs/SPEC.md)：功能驗收、隱私、錯誤與介面提案。
- [架構](docs/ARCHITECTURE.md)：套件責任與傳輸邊界。
- [開發順序](docs/DEVELOPMENT.md)：各階段工作與完成條件。
- [測試計畫](docs/TEST_PLAN.md)：合成資料、實機與發布驗證。

規格中的 CLI 範例仍是提案，不能直接執行。Ubuntu CI 通過也不代表已有 Linux 藍牙後端。

## 求助與參與

改動行為前請先讀[貢獻規範](CONTRIBUTING.md)。問題、文件缺漏，以及去識別後的錯誤或相容性回報，請使用 [GitHub Issues](https://github.com/skydreamer0/speaker-link/issues)，附上測試 commit，並區分通過、失敗與未測試的項目。

請勿公開 MAC、序號、私人裝置名稱、本地路徑、原始擷取或完整 log。[疑難排解](docs/TROUBLESHOOTING.md#report-a-problem-without-sharing-private-data)提供最小回報格式；敏感問題請依 [SECURITY](SECURITY.md) 的方式處理。

專案採本地運作設計，沒有規劃帳號、雲端服務或自動上傳診斷。是否分享報告由你決定。

## 授權與獨立性

原創程式與文件採 [MIT 授權](LICENSE)。第三方材料請見[來源與授權政策](THIRD_PARTY_NOTICES.md)及[散布注意事項](docs/COMPLIANCE.md)。

本專案是獨立社群專案，與 Bose Corporation、Bluetooth SIG 沒有隸屬或背書關係。產品與公司名稱用來辨識相容性研究對象，商標仍屬各自權利人；本專案不宣稱認證、等同官方 App，或能解鎖內建 EQ。
