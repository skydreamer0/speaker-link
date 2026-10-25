# Third-party sources and provenance

At the specification baseline this repository contains original project documents and a standard MIT license. No third-party implementation, firmware, binary, logo or packet capture is bundled.

| Reference | Use so far | Adoption rule |
| --- | --- | --- |
| [aaronsb/bosectl](https://github.com/aaronsb/bosectl) | Protocol/project research; MIT repository | Verify exact files/version and retain copyright and license if reused |
| [bose-connect Rust docs](https://docs.rs/bose-connect/latest/bose_connect/) | API research | Audit exact crate source, license and transitive dependencies before use |
| [BoseConnect-Linux_based-connect](https://github.com/bosefirmware/BoseConnect-Linux_based-connect) | Historical GPL project reference | No copied, translated or linked code in this MIT-only distribution without a separate licensing decision |
| Microsoft Windows API documentation | API design reference | Review selected windows crate version and license before dependency adoption |
| egui/eframe | Proposed native UI | Review chosen release and transitive licenses before adoption |

For every adopted dependency or source fragment, record version/commit, upstream URL, SPDX expression, files used, modifications and notice obligations. A permissive top-level license does not settle every bundled file or transitive dependency. This table is a research inventory, not a completed dependency audit.
