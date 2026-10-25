# 獨立驗收紀錄

文件版號 0.3.0。工程核心 commit 為 `5293fe849f857517f9d23e990595c40c48519d14`。

本席未設計或實作 host-v2，已自行實讀 pstack 入口、完整流程、Codex adapter、Feature、no-comments 與 Comment Sicko。審查與實作者使用同一模型供應商，本次不構成跨供應商驗證。本席未執行硬體程序，也未更動程式、喇叭設定、配對、連線或存取權。

硬體執行前的同版安全 gate 為 PASS。已檢查 API allowlist、COM 生命週期、真正 Future owner 的銷毀順序、最多四次查詢、停止條件、私人 stdin、輸出遮罩與只終止自身子程序的 wrapper。未發現必須修正的安全問題。九個合成測試與五項離線檢查的原始紀錄及退出碼均已核讀。Wrapper 語法解析零錯誤。Comment review 無建議刪除、無已刪除註解、無 MUST KILL 與無 checker suppression。

已獨立核對凍結來源、wrapper 與執行檔。實測執行檔 SHA256 為 `79d67d1b118edfae4be4f09c8d4c15367ea21a1d813ec1da6f9125b4749d98bd`。核心工作樹乾淨，舊交付 manifest 的八項來源與原 host artifact 雜湊全部未變。

| 驗收項目 | 本輪結果 |
| --- | --- |
| 實際 Pending 後銷毀 Future owner | 已涵蓋。首 poll Pending，ActualOwnerDropped，host guard 未超時。 |
| drop 後正常重查 | 成功，回傳一項服務。 |
| 公開 Timeout 結果 | 已觀測。1ms request 首 poll 即 Ready ErrorCode::Timeout。 |
| 實機 Pending 後 Timeout 路徑 | 未涵蓋。首 poll Ready，stimulus 為 NotApplied，pending coverage 為 false。 |
| 短期限後正常重查 | 成功，回傳一項服務。 |
| 原生 Cancel 結果、取消確認與 Close 結果 | 不可觀測。 |
| OS 資源完全釋放 | 無驗證證據。drop 返回、重查成功與程序結束不能證明此項。 |
| Timeout 原因 | 不可觀測。native HRESULT 與內部 deadline 可能映射至相同公開碼。 |
| 真實拒絕與不可用 | NOT_RUN，未製造錯誤條件。 |
| host 與程序 guard | 本次未觸發。未量測前後底層連線狀態。 |

已直接對照原始實機紀錄、遮罩紀錄、RESULT.json 與 HANDOFF.zh-TW.md，各項公開結果與限制一致。只執行一次有限 suite，沒有為追求涵蓋而重試。exit 0 與 HarnessCompleted 只代表觀察程序正常結束。韌體 3.0.1 標為使用者回報，沒有誤稱 API 驗證。原核心三十二項測試沒有誤稱本輪重跑。

上述文件判讀為 PASS。Issue #2 整票與 Issue #3 gate 仍待確認。本紀錄不把部分生命週期證據提升為完整產品、內建 EQ、私有控制通道或完整資源清理驗收。ZIP、manifest 與封裝 source bytes 待封裝後另核，未以此文件先行宣稱封裝通過。
