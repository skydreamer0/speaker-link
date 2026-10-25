# Distribution and research cautions

Document version 0.2.0. These are project policies and open review items, not a legal determination for every jurisdiction.

## Original work and dependencies

Use MIT for original contributions. Retain required notices for adopted permissive code. Keep GPL/AGPL copied, translated or linked implementations out of the proposed MIT-only distribution unless maintainers make and document a separate licensing decision. An AI rewrite is not proof of independent authorship or permission. Quarantine uncertain material and review its provenance before inclusion.

Before the first binary release, complete the [actual-artifact notices checklist](../THIRD_PARTY_NOTICES.md#before-distributing-binaries), including bundled standard-library components, runtimes, and other redistributed material. A clean Cargo dependency list alone does not complete that review.

No official application assets, firmware dumps, signing keys or protected source belong here. Independently observed behavior and protocol descriptions require their own provenance record; they do not automatically settle contract or anti-circumvention questions. Review the terms applying to the actual software/device and jurisdiction before expanding reverse-engineering work. Bose website terms and device software terms have different scopes.

Before expanding research to decompilation, firmware extraction, or circumvention of access controls, record the applicable software terms, jurisdiction, lawful source, proposed method, and case-specific review outcome. This record is not blanket permission to proceed; unresolved questions need appropriate review before that expansion.

## Names and claims

Use Speaker Link branding. Use Bose and model names only to describe compatibility targets, with a visible independence disclaimer. Do not copy logos, app icons, trade dress or certification marks. Do not say "official", "certified", "unlocks EQ" or "full compatibility" without the corresponding rights and evidence.

Use of the Windows Bluetooth stack does not by itself settle Bluetooth SIG qualification or trademark obligations. Evaluate the actual distributed product and any hardware/commercial plan against the SIG rules before making certification claims or expanding distribution. No logo permission or qualification is claimed by this project.

## Privacy and control

Follow the allowlist in SPEC. Do not collect audio content, contact lists, pairing secrets or other people's devices. Keep local control explicit. Sharing diagnostics is voluntary. Sanitized output can still reveal model, software version and timing, so preview and consent remain necessary.

Do not advertise safe arbitrary writes or universal compatibility. Unsupported operations must send nothing. Firmware flashing, pairing deletion and internal EQ require a separate proposal and risk review; none is authorized by this scope.

## Primary references

- [MIT license](https://opensource.org/license/mit)
- [GNU GPL FAQ](https://www.gnu.org/licenses/gpl-faq.en.html)
- [Bose trademarks](https://www.bose.com/legal/trademarks)
- [Bose terms of use](https://www.bose.com/legal/terms-of-use)
- [Bose end user license agreement](https://www.bose.com/legal/end-user-license-agreement)
- [Bluetooth SIG qualification](https://www.bluetooth.com/develop-with-bluetooth/qualify/)
- [Taiwan Copyright Act](https://law.moj.gov.tw/LawClass/LawAll.aspx?PCode=J0070017), including Article 80-2; assess the applicable rules and exceptions for the specific research.
- [U.S. 17 U.S.C. § 1201](https://www.govinfo.gov/content/pkg/USCODE-2024-title17/html/USCODE-2024-title17-chap12-sec1201.htm), including subsection (f); not a universal reverse-engineering authorization.
- [GitHub private vulnerability reporting](https://docs.github.com/en/code-security/how-tos/report-and-fix-vulnerabilities/report-privately)
