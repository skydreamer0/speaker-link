# Speaker Link

開發中的本地 Rust 工具，目標是在 Windows 查看 Bose 音響的電量與連線。

[![main 的 Foundation 契約：查看合成檢查結果](https://github.com/skydreamer0/speaker-link/actions/workflows/ci.yml/badge.svg?branch=main&event=push)](https://github.com/skydreamer0/speaker-link/actions/workflows/ci.yml?query=branch%3Amain+event%3Apush)
[![閱讀原創程式與文件的 MIT 授權](https://img.shields.io/badge/license-MIT-blue)](LICENSE)
[![開發階段：只有 foundation，查看里程碑與驗收門檻](https://img.shields.io/badge/stage-foundation%20only-orange)](docs/DEVELOPMENT.md)
[![Windows 研究目標：查看目前相容性限制](https://img.shields.io/badge/research%20target-Windows-lightgrey)](docs/COMPATIBILITY.md)

**目前沒有可安裝版本、已發布 crate、可用 CLI，或經驗證的音響控制支援。**

[English](README.md) · [快速開始](docs/QUICKSTART.md) · [相容性](docs/COMPATIBILITY.md) · [疑難排解](docs/TROUBLESHOOTING.md)

## 從這裡開始

- **安裝 App 或 CLI：** 目前尚未提供，請看[可用狀態](docs/QUICKSTART.md#install-the-app-or-cli)。
- **閱讀原始碼或參與開發：** 依照[固定版本教學](docs/QUICKSTART.md#check-the-foundation-from-source)檢查 foundation。
- **確認自己的音響：** 查看[逐功能相容性紀錄](docs/COMPATIBILITY.md)，不能只憑品牌或型號名稱推論支援。
- **回報問題：** 先讀[疑難排解](docs/TROUBLESHOOTING.md)與[貢獻規範](CONTRIBUTING.md)。

## 目前能做什麼

foundation 已合併至 `main`，包含 Rust 契約與不需要裝置的合成測試，還不是可用的音響控制核心。

| 項目 | 當前狀態 |
| --- | --- |
| Rust foundation | 未發布的資料模型、錯誤、指令輸入與診斷預覽契約 |
| Windows 實機探索 | 公開 foundation 尚未實作；[issue #2](https://github.com/skydreamer0/speaker-link/issues/2) 的驗收門檻仍開放 |
| 電量、音響端裝置清單、連線控制 | 沒有經審查的實機支援 profile |
| 診斷匯出、CLI、Windows App | 尚未提供完整可用實作 |

[Foundation CI 徽章](https://github.com/skydreamer0/speaker-link/actions/workflows/ci.yml?query=branch%3Amain+event%3Apush)追蹤 `main` 的 push 執行，只檢查程式與合成契約，不驗證 Windows 藍牙存取或音響功能。Ubuntu CI 也不代表 Linux 藍牙支援。[固定 commit 的證據與歷史](docs/COMPATIBILITY.md#source-and-platform-status)另行保留，不由動態徽章取代。

## 快速開始：檢查 foundation

目前可以動手做的第一步是檢查開發原始碼，不是安裝產品。

依[原始碼與測試教學](docs/QUICKSTART.md#check-the-foundation-from-source)準備 Git 與 Rust、取得固定 foundation commit，再執行檢查。這條路徑不需要音響，也不會探索藍牙；通過不代表硬體相容。

## 技術與套件

採用 [Rust 2021 workspace](Cargo.toml)，包含四個未發布套件。[Cargo.lock](Cargo.lock) 目前沒有第三方 crate，尚未選定最低 Rust 或 Windows 版本。

| 套件 | 目前內容 |
| --- | --- |
| [speakerlink-core](crates/speakerlink-core/) | 模型、指令輸入、錯誤與診斷契約；沒有協定 codec 或硬體控制 |
| [speakerlink-windows](crates/speakerlink-windows/) | 探索契約與 `Unsupported` 佔位實作；沒有真實 Windows adapter |
| [speakerlink-cli](crates/speakerlink-cli/) | library 佔位套件，沒有可執行命令 |
| [speakerlink-app](crates/speakerlink-app/) | library 佔位套件，沒有 GUI |

Windows API bindings 與 eframe/egui 屬於[架構候選](docs/ARCHITECTURE.md)，不是已整合的依賴。未來套件、CLI 與 App 將共用能力判斷與控制實作。

## 相容性與限制

第一個研究樣本是 SoundLink Flex 第一代，型號 435910。目前沒有已驗證的硬體 profile，產品也不限定某台主機、某個藍牙介面廠牌或特定韌體。型號、韌體、Windows build、adapter/driver 與各項功能都需要分別留下[證據](docs/COMPATIBILITY.md)。

缺少資料時維持未知，不能用電腦的配對清單代替音響自己的清單，標準服務探索成功也不代表支援私有控制。未知韌體在評估前保持唯讀，不支援的操作不能送出指令。

首版不包含內建 EQ、刷機、任意封包、刪除配對歷史、修改實體按鍵，或完整取代 Bose App。

## 開發順序

- **現在：** foundation 契約與合成 CI 已進 `main`；Windows 與實機驗收仍未完成。
- **下一步：** 真實 Windows 探索與經審查的硬體證據。
- **之後：** 核心四項功能，再依序完成 CLI、App 與發布驗收。

詳見[開發門檻](docs/DEVELOPMENT.md)、[規格](docs/SPEC.md)、[架構](docs/ARCHITECTURE.md)與[測試計畫](docs/TEST_PLAN.md)。規格內的 CLI 範例仍是提案。

## 求助與參與

改動行為前請先讀[貢獻規範](CONTRIBUTING.md)。問題與去識別回報請使用 [GitHub Issues](https://github.com/skydreamer0/speaker-link/issues)，附上測試 commit，並區分通過、失敗與未測試。

請勿公開 MAC、序號、私人裝置名稱、本地路徑、原始擷取或完整 log。請依[最小回報清單](docs/TROUBLESHOOTING.md#report-a-problem-without-sharing-private-data)整理資訊；敏感問題依 [SECURITY](SECURITY.md) 處理。

產品採本地運作設計，沒有規劃帳號、雲端服務或自動上傳診斷，是否分享報告由你決定。README 徽章圖片由 GitHub 與 Shields.io 提供；[徽章說明](THIRD_PARTY_NOTICES.md#readme-badges)另行解釋這種網頁行為。

## 授權與獨立性

原創程式與文件採 [MIT 授權](LICENSE)。第三方材料請見[來源與授權政策](THIRD_PARTY_NOTICES.md)及[散布注意事項](docs/COMPLIANCE.md)。

本專案是獨立社群專案，與 Bose Corporation 或 Bluetooth SIG 沒有隸屬、贊助或背書關係。產品與公司名稱用來辨識相容性研究對象，商標仍屬各自權利人；本專案不宣稱認證、等同官方 App，或能解鎖內建 EQ。
