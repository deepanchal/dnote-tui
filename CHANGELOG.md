# Changelog

## [0.2.0](https://github.com/deepanchal/dnote-tui/compare/dnote-tui-v0.1.1...dnote-tui-v0.2.0) (2023-09-03)


### Features

* **dnote_lib:** add summary field on `DnotePage`. update FromStr impl & tests ([08c0757](https://github.com/deepanchal/dnote-tui/commit/08c0757133b49dfd04c9925b257caa0bf9bf9b86))
* **ui:** use new summary field from `DnotePage` to display truncated page content besides page id in pages section ([3683ecf](https://github.com/deepanchal/dnote-tui/commit/3683ecf596060f6d4eac000fb8fd8726d12b7fad))


### Documentation

* **README:** update documentation url in README.md ([34b9292](https://github.com/deepanchal/dnote-tui/commit/34b9292d48c6db758cfda03f4ffbc3635269f54a))

## [0.1.1](https://github.com/deepanchal/dnote-tui/compare/dnote-tui-v0.1.0...dnote-tui-v0.1.1) (2023-09-03)


### Bug Fixes

* **deps:** update rust crate tui to 0.23.0 ([0f988e4](https://github.com/deepanchal/dnote-tui/commit/0f988e4ab4a8815b2521d6bdf6a00e80bde2b3e9))


### Continuous Integration

* **release:** add publish to crates.io job in release.yml to auto-publish on release please pr merge ([c915ff5](https://github.com/deepanchal/dnote-tui/commit/c915ff549ad600868674a34d810514678bf2b518))

## 0.1.0 (2023-08-20)


### Features

* **app:** add page_info state to app, add get_page_content method ([a8b08a9](https://github.com/deepanchal/dnote-tui/commit/a8b08a92d85d745ed281e7e96beefb69998d177f))
* **app:** add stateful pages list, add getters for books & pages ([a99f23f](https://github.com/deepanchal/dnote-tui/commit/a99f23fd71af0194cbca6e7a7a764cc5977b6fb1))
* **app:** add StatefulList, add books list to App, load books from dnote in constructor ([1df5378](https://github.com/deepanchal/dnote-tui/commit/1df53787209f36d0b887bbca5525461c0208e3d1))
* **app:** add TuiChunk enum, add selected_chunk property to app, add select_next_chunk & select_prev_chunk methods ([38ee7e8](https://github.com/deepanchal/dnote-tui/commit/38ee7e884edb50f7549fd371070def331efaf057))
* **app:** remove test counter & related methods from app ([5e83ed0](https://github.com/deepanchal/dnote-tui/commit/5e83ed095b4e6ac43458f8536493260c134449b6))
* **bin:** add dnote_client_test binary w/ test usage from main ([3d4b380](https://github.com/deepanchal/dnote-tui/commit/3d4b380b4d26c6b3a81a95c133e2fb424e8a8c32))
* **cargo:** set default-run to dnote-tui ([47da791](https://github.com/deepanchal/dnote-tui/commit/47da79149b016172df496f7605e7d77056294ba6))
* **deps:** add tui using ratatui & crossterm deps, update lockfile ([d0a6622](https://github.com/deepanchal/dnote-tui/commit/d0a6622681ae9df6e82441213f76a40e0a502d88))
* **dnote_lib:** comment print statements in DnoteClient methods ([e425914](https://github.com/deepanchal/dnote-tui/commit/e4259146e6166adbb5b27c5fb6e22dde1e05519b))
* **handler:** add keybindings for navigating through books list ([40305e5](https://github.com/deepanchal/dnote-tui/commit/40305e59a7f07de644bcead799f307046910085b))
* **handler:** handle navigation using selected_chunk & helper methods ([1b7e5f8](https://github.com/deepanchal/dnote-tui/commit/1b7e5f8f525a36e983a94e13d3cb35992040f953))
* init with binary project ([6cbeb5f](https://github.com/deepanchal/dnote-tui/commit/6cbeb5f324af57fdc0a6d5eced6e743627df8e79))
* **lib:** add DnoteClient, DnoteClientError, add funcs for DnoteClient ([c737810](https://github.com/deepanchal/dnote-tui/commit/c7378102641d7b0c71e70c94c18b37f7c9da0f03))
* **lib:** add DnotePageInfo struct ([00b85d7](https://github.com/deepanchal/dnote-tui/commit/00b85d7a77afdd75d44fbf75ff3db90976646190))
* **lib:** add lib.rs ([56e7c63](https://github.com/deepanchal/dnote-tui/commit/56e7c63da8e0d45a1324115cf3fe4833356beb55))
* **lib:** implement FromStr for DnotePage, remove uuid & content fields from struct ([554f602](https://github.com/deepanchal/dnote-tui/commit/554f60259e4ca159c768aa7d379f38268ecd1450))
* **lib:** implement FromStr for DnotePageInfo ([1823c2c](https://github.com/deepanchal/dnote-tui/commit/1823c2cac3f47fd76219216e7a602ea9af76b209))
* **lib:** implement FromStr trait for DnoteBook to parse string into struct ([68c4af8](https://github.com/deepanchal/dnote-tui/commit/68c4af8671f4f40d2fb4b2ef55dc915e27373284))
* **lib:** implement view_page_info func for DnoteClient ([0cdd126](https://github.com/deepanchal/dnote-tui/commit/0cdd126ca180cb97e851c371bd72061ab1667071))
* **lib:** implement view_pages func for DnoteClient, format file ([5554887](https://github.com/deepanchal/dnote-tui/commit/55548874f22747aebf6be1aad3274c05ed9a6d26))
* **lib:** improve DnoteClientError enum, use ? shorthand for view_books ([f06b3e8](https://github.com/deepanchal/dnote-tui/commit/f06b3e82a6638ad0322b4ac34039011a5284a1ff))
* **main:** test out DnoteClient in main fn ([25fdb48](https://github.com/deepanchal/dnote-tui/commit/25fdb48e25782496abbdea5b15021dd3d663006e))
* **setup:** use rust-tui-template to setup files & mods ([3ebf5c4](https://github.com/deepanchal/dnote-tui/commit/3ebf5c4b4d34b31f54111b9a66e62a9194d421f7))
* **src:** add dnote_lib mod w/ DnoteBook & DnotePage public structs ([7ca9652](https://github.com/deepanchal/dnote-tui/commit/7ca965234a101721b2442fb39040a493f85766b7))
* **ui:** adjust areas for chunks ([7808385](https://github.com/deepanchal/dnote-tui/commit/78083853e2f4f2339a88abd7ce16bdf5c14d276e))
* **ui:** render books list in books_block chunk from stateful list of books in app's instance ([85cd6de](https://github.com/deepanchal/dnote-tui/commit/85cd6de5bdb6345338cc9fe4190d56cad2c1bc4a))
* **ui:** render page content in page_content chunk using page_content state ([b7ab6e5](https://github.com/deepanchal/dnote-tui/commit/b7ab6e5b8ec7c2513ce6c3e1894f790c9f654535))
* **ui:** setup layout for books, pages, & content ([9e1d769](https://github.com/deepanchal/dnote-tui/commit/9e1d7690047ac0fbcf11ef1ddcef8dd202231e1b))
* **ui:** use get_books, render pages list for currently selected book ([1635055](https://github.com/deepanchal/dnote-tui/commit/1635055239837311634ae869a87a479deda6f8c8))


### Bug Fixes

* **app:** don't allow next selection when on pages section ([54ed8ff](https://github.com/deepanchal/dnote-tui/commit/54ed8ff7e0b2ff9e2321ac6f94ce751e70aa2fce))
* **deps:** update rust crate crossterm to 0.27.0 ([7e25972](https://github.com/deepanchal/dnote-tui/commit/7e25972d918e200d0ea306020e6b7d16d290ad67))
* **deps:** update rust crate tui to 0.22.0 ([a85cd3b](https://github.com/deepanchal/dnote-tui/commit/a85cd3b9141cb752407410b21f0850590f667ff8))
* **handler:** deselect page before selecting books chunk ([e615765](https://github.com/deepanchal/dnote-tui/commit/e6157653c3c99aab4342a9d83dcf55c95c68159d))
* **handler:** select page on right key handler and then select chunk ([0741da7](https://github.com/deepanchal/dnote-tui/commit/0741da78987d201cfcc4a8650cb08c271a677972))
* **ui:** remove redundant clone call in ui.rs ([e520072](https://github.com/deepanchal/dnote-tui/commit/e520072da3b14300f34bdd6403b06509a5e89cdd))


### Documentation

* **README:** add README.md from https://github.com/othneildrew/Best-README-Template template ([d04b911](https://github.com/deepanchal/dnote-tui/commit/d04b911dbd7361a7d218ae90554c32a06d297973))
* **README:** fix license link ([5ddba61](https://github.com/deepanchal/dnote-tui/commit/5ddba6144ff4f577ae4feb974bc77102087eb3d4))
* **README:** fix license text in README.md ([ef4ce5c](https://github.com/deepanchal/dnote-tui/commit/ef4ce5c5b73e392f16faead9e41b2244c768e633))
* **README:** make improvements to README.md ([1cb8f36](https://github.com/deepanchal/dnote-tui/commit/1cb8f36672619696fdc99e9e6276a33eafe0e76b))
* **README:** remove table of contents from README.md ([c86cabd](https://github.com/deepanchal/dnote-tui/commit/c86cabd31dd5ca3ee78ba29117c3a65eb24d4bb6))


### Code Refactoring

* **dnote_lib:** change fn names in DnoteClient ([5d7c1e6](https://github.com/deepanchal/dnote-tui/commit/5d7c1e698c5caff3ae27fb79485572c1906b2699))
* **dnote_lib:** use ? for handling errors instead of nested match statements ([d7796ca](https://github.com/deepanchal/dnote-tui/commit/d7796caf59b51c175949311dea9ea244c198f166))
* fix git blame ignore revs filename ([24a5cf0](https://github.com/deepanchal/dnote-tui/commit/24a5cf0bd29c0315cd9d6b8998dbd8ff8ecce710))
* **lib:** use vector collect to improve parsing cmd output to DnoteBook vector ([68f9bed](https://github.com/deepanchal/dnote-tui/commit/68f9bed16a4f6b3878a6abf7c224a32cc0c0e63a))
* rename `TuiChunk` to `TuiSection` ([7e03f92](https://github.com/deepanchal/dnote-tui/commit/7e03f925514e2e83f3060d886be36b00fe9ead3a))
* replace vars + methods with chunk text to section as it makes more sense ([c824b9b](https://github.com/deepanchal/dnote-tui/commit/c824b9b4d501f7116073b871189a72b9f78aba91))
* **ui:** store chunks in vars & use them to render widget ([71cf60f](https://github.com/deepanchal/dnote-tui/commit/71cf60fd7f6a6c262410ad811dfca389a5c2dc85))


### Tests

* **lib:** add test for parsing FromStr implementation of DnotePageInfo ([e9860c6](https://github.com/deepanchal/dnote-tui/commit/e9860c60c8942be7ff6f9e61a23b6d6cef807067))
* **lib:** add tests for parsing implementations for DnoteBook & DnotePage ([c578273](https://github.com/deepanchal/dnote-tui/commit/c578273667e141feaa7ee485cb2eceb48da51085))


### Continuous Integration

* **main:** add main workflow for build, lint & test ([708c61b](https://github.com/deepanchal/dnote-tui/commit/708c61bfd3f8748bc4d21f08be3da649293b1e86))
* **release:** add release please action in release.yml ([aae525f](https://github.com/deepanchal/dnote-tui/commit/aae525f393b0566fb9c56633fde2fbe83c4d9b69))
* **release:** allow workflow_dispatch to trigger release workflow ([9055233](https://github.com/deepanchal/dnote-tui/commit/90552339eef2ec5a9fdcbb796afc6a515e04f63e))
