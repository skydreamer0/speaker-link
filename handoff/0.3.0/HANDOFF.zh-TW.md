# Speaker Link #2｜本地生命週期驗收補充

文件版本：0.3.0（2026-10-25）。狀態：本輪有限實測完成；Issue #2 整票與 Issue #3 gate 仍待確認。

核心程式仍固定在本地 commit `5293fe849f857517f9d23e990595c40c48519d14`，未修改或發布。原交接 0.2.0 與原 host 保持凍結。本包提供完整核心工作樹、新 host-v2、兩份擇一使用的整合 patch、測試紀錄與本報告，供線上開發接續。PR7 已確認的遠端版本仍為 `382294f407e5b6ef4dd8c3512e0c4f1b2c480b9d`；本地修正尚未整合到該遠端版本。

## 實測結果

目標為使用者的 Bose SoundLink Flex；韌體 3.0.1 是使用者回報，未由 API 讀取。Wrapper 選取唯一的既有裝置，host 再核對精確名稱、已配對與目前存取 Allowed。地址與名稱透過私人 stdin 傳遞，不存入診斷。

| 探針 | 實際結果 | 可成立的結論 |
| --- | --- | --- |
| Pending 後 drop | 首 poll Pending；真正 Future owner 已銷毀；host guard 未超時 | public Future 的 Pending-drop 實機路徑已涵蓋 |
| drop 後重查 | Services，數量 1 | 後續唯讀查詢可用 |
| 1ms 短期限 | 首 poll Ready，ErrorCode::Timeout；未施加 Pending 後繼續輪詢的刺激 | 確實觀察到公開 Timeout 結果；Pending→Timeout 路徑未涵蓋 |
| 短期限後重查 | Services，數量 1 | 後續唯讀查詢可用 |

只執行一次 suite，最多 4 次 Uncached 服務查詢，沒有重試追求涵蓋率。host 與 wrapper exit 0；退出碼僅代表觀察程序正常結束，不能取代各欄位驗收。此次恢復查詢只記錄服務數量，不蒐集 UUID；前次 0.2.0 的 `0x111E` 結果不當成本輪重新讀到 UUID 的證據。

Windows 原生 HRESULT 與內部 deadline 都可能正規化成 Timeout；公開結果不能區分來源。Native Cancel 呼叫結果、取消完成確認、Close 結果及 OS 資源完全釋放皆不可觀測。重查成功、程序結束或 drop 返回都不能補足這些證據。實機拒絕與不可用沒有自然出現，保持 NOT_RUN；未改權限、radio、配對或連線來製造錯誤。

## 編譯、安全審查與範圍

新 host 的 9 個合成測試通過；offline locked test、Clippy all-targets -D warnings、fmt、check all-targets、build 全部 exit 0，原始輸出及逐命令退出碼納入補充包。原核心先前的 32 個測試未在本輪重跑，來源雜湊與 clean 工作樹已重新核對。

非作者審查者已自行載入 pstack 流程、核對凍結程式與執行檔雜湊、檢查 API/COM/owner/drop/遮罩/查詢上限，於硬體執行前給予同版安全 PASS。審查者屬同一模型供應商，並非跨供應商驗證。交付包另外核對內容、manifest、硬體紀錄與本報告一致性，結果見 REVIEW.zh-TW.md。

沒有喇叭設定指令、Bose frames、socket、GATT、配對、明確 connect、存取權要求或權限變更。Uncached 探索可能造成 Windows 自行管理的藍牙連線活動；未量測前後底層連線狀態，不能聲稱連線狀態完全不變。55 秒硬限制只適用 wrapper 所建立的子程序，不涵蓋前置 PnP 列舉；終止該子程序也不證明合作式清理成功。本次未觸發 host/process guard。

## 線上接續程序

1. 沿用原 Issue #2 與 PR7。依基底擇一套用 `fix-from-ce9dfb6.patch` 或 `discovery-from-pr7.patch`，不要兩份都套用；對照完整 `source/speaker-link` 與核心版本。原 0.2.0 包仍保留。
2. 在公開整合後的新 commit 重跑 CI，不能把本地 5293fe8 的測試當成新遠端版本已通過。
3. 新 host 位於 `work/discovery-host-v2`，Cargo path dependency 指向包內 `source/speaker-link`。Windows 安裝 Rust/MSVC/SDK 後，先 `cargo test --locked --manifest-path work/discovery-host-v2/Cargo.toml`、`cargo build --locked --manifest-path work/discovery-host-v2/Cargo.toml`。只有相同目標與授權範圍才執行其 `run-flex.ps1`；這是本地驗收工具，尚非正式 CLI 或 Windows App。
4. 接續設計可觀測的取消與清理驗收，以及可控且明確授權的拒絕／不可用測試。不得把目前成功重查當成底層資源清理已驗證。Pending→Timeout 仍缺實機證據，避免靠無上限重試取得想要的結果。
5. 正式通用裝置選擇、診斷 preview/export 一致性及使用者選擇匯出路徑仍待實作。私有通道、電量與裝置資訊、連線管理、設定與內建 EQ 尚未證實。README／教學 docs-only 排隊事項與原線上權責保留。

## 交付與追蹤

完整補充包為 `speaker-link-lifecycle-20261025.zip`。包內 manifest 記錄實際工作樹檔案 bytes 的 SHA256（保留既有換行），不以 Git blob hash 代替；不含 `.git`、target、執行檔、私人地址、端點或完整設備診斷。`SHA256SUMS.txt` 提供外層交付檔案校驗。

工作項目：Speaker Link #2／WB-0037，本地本輪完成、整票待確認；程式版本為上述本地 commit，文件 0.3.0。[Workboard](https://workboard-george.georgehsiao.chatgpt.site/)；[PR7](https://github.com/skydreamer0/speaker-link/pull/7)。下一步是線上整合修正及補齊尚缺 gate，未推送、合併或發布。
